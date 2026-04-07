You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-001
- **Kind**: endpoint
- **Identifier**: POST /console/api/login, POST /console/api/email-code-login, POST /console/api/oauth/login/<provider>
- **Description**: Console authentication endpoints handling login, OAuth, and registration. Vulnerable to brute force, credential stuffing, and OAuth redirect manipulation
- **Locations**: api/controllers/console/auth/login.py, api/controllers/console/auth/oauth.py, api/controllers/console/auth/email_register.py

## Source Code

### api/controllers/console/auth/login.py
```py
import flask_login
from flask import make_response, request
from flask_restx import Resource
from pydantic import BaseModel, Field

import services
from configs import dify_config
from constants.languages import get_valid_language
from controllers.console import console_ns
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    EmailPasswordLoginLimitError,
    InvalidEmailError,
    InvalidTokenError,
)
from controllers.console.error import (
    AccountBannedError,
    AccountInFreezeError,
    AccountNotFound,
    EmailSendIpLimitError,
    NotAllowedCreateWorkspace,
    WorkspacesLimitExceeded,
)
from controllers.console.wraps import (
    decrypt_code_field,
    decrypt_password_field,
    email_password_login_enabled,
    setup_required,
)
from events.tenant_event import tenant_was_created
from libs.helper import EmailStr, extract_remote_ip
from libs.login import current_account_with_tenant
from libs.token import (
    clear_access_token_from_cookie,
    clear_csrf_token_from_cookie,
    clear_refresh_token_from_cookie,
    extract_refresh_token,
    set_access_token_to_cookie,
    set_csrf_token_to_cookie,
    set_refresh_token_to_cookie,
)
from services.account_service import AccountService, InvitationDetailDict, RegisterService, TenantService
from services.billing_service import BillingService
from services.errors.account import AccountRegisterError
from services.errors.workspace import WorkSpaceNotAllowedCreateError, WorkspacesLimitExceededError
from services.feature_service import FeatureService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class LoginPayload(BaseModel):
    email: EmailStr = Field(..., description="Email address")
    password: str = Field(..., description="Password")
    remember_me: bool = Field(default=False, description="Remember me flag")
    invite_token: str | None = Field(default=None, description="Invitation token")


class EmailPayload(BaseModel):
    email: EmailStr = Field(...)
    language: str | None = Field(default=None)


class EmailCodeLoginPayload(BaseModel):
    email: EmailStr = Field(...)
    code: str = Field(...)
    token: str = Field(...)
    language: str | None = Field(default=None)


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(LoginPayload)
reg(EmailPayload)
reg(EmailCodeLoginPayload)


@console_ns.route("/login")
class LoginApi(Resource):
    """Resource for user login."""

    @setup_required
    @email_password_login_enabled
    @console_ns.expect(console_ns.models[LoginPayload.__name__])
    @decrypt_password_field
    def post(self):
        """Authenticate user and login."""
        args = LoginPayload.model_validate(console_ns.payload)
        request_email = args.email
        normalized_email = request_email.lower()

        if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
            raise AccountInFreezeError()

        is_login_error_rate_limit = AccountService.is_login_error_rate_limit(normalized_email)
        if is_login_error_rate_limit:
            raise EmailPasswordLoginLimitError()

        invite_token = args.invite_token
        invitation_data: InvitationDetailDict | None = None
        if invite_token:
            invitation_data = RegisterService.get_invitation_with_case_fallback(None, request_email, invite_token)
            if invitation_data is None:
                invite_token = None

        try:
            if invitation_data:
                data = invitation_data.get("data", {})
                invitee_email = data.get("email") if data else None
                invitee_email_normalized = invitee_email.lower() if isinstance(invitee_email, str) else invitee_email
                if invitee_email_normalized != normalized_email:
                    raise InvalidEmailError()
            account = _authenticate_account_with_case_fallback(
                request_email, normalized_email, args.password, invite_token
            )
        except services.errors.account.AccountLoginError:
            raise AccountBannedError()
        except services.errors.account.AccountPasswordError as exc:
            AccountService.add_login_error_rate_limit(normalized_email)
            raise AuthenticationFailedError() from exc
        # SELF_HOSTED only have one workspace
        tenants = TenantService.get_join_tenants(account)
        if len(tenants) == 0:
            system_features = FeatureService.get_system_features()

            if system_features.is_allow_create_workspace and not system_features.license.workspaces.is_available():
                raise WorkspacesLimitExceeded()
            else:
                return {
                    "result": "fail",
                    "data": "workspace not found, please contact system admin to invite you to join in a workspace",
                }

        token_pair = AccountService.login(account=account, ip_address=extract_remote_ip(request))
        AccountService.reset_login_error_rate_limit(normalized_email)

        # Create response with cookies instead of returning tokens in body
        response = make_response({"result": "success"})

        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)

        return response


@console_ns.route("/logout")
class LogoutApi(Resource):
    @setup_required
    def post(self):
        current_user, _ = current_account_with_tenant()
        account = current_user
        if isinstance(account, flask_login.AnonymousUserMixin):
            response = make_response({"result": "success"})
        else:
            AccountService.logout(account=account)
            flask_login.logout_user()
            response = make_response({"result": "success"})

        # Clear cookies on logout
        clear_access_token_from_cookie(response)
        clear_refresh_token_from_cookie(response)
        clear_csrf_token_from_cookie(response)

        return response


@console_ns.route("/reset-password")
class ResetPasswordSendEmailApi(Resource):
    @setup_required
    @email_password_login_enabled
    @console_ns.expect(console_ns.models[EmailPayload.__name__])
    def post(self):
        args = EmailPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"
        try:
            account = _get_account_with_case_fallback(args.email)
        except AccountRegisterError:
            raise AccountInFreezeError()

        token = AccountService.send_reset_password_email(
            email=normalized_email,
            account=account,
            language=language,
            is_allow_register=FeatureService.get_system_features().is_allow_register,
        )

        return {"result": "success", "data": token}


@console_ns.route("/email-code-login")
class EmailCodeLoginSendEmailApi(Resource):
    @setup_required
    @console_ns.expect(console_ns.models[EmailPayload.__name__])
    def post(self):
        args = EmailPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"
        try:
            account = _get_account_with_case_fallback(args.email)
        except AccountRegisterError:
            raise AccountInFreezeError()

        if account is None:
            if FeatureService.get_system_features().is_allow_register:
                token = AccountService.send_email_code_login_email(email=normalized_email, language=language)
            else:
                raise AccountNotFound()
        else:
            token = AccountService.send_email_code_login_email(account=account, language=language)

        return {"result": "success", "data": token}


@console_ns.route("/email-code-login/validity")
class EmailCodeLoginApi(Resource):
    @setup_required
    @console_ns.expect(console_ns.models[EmailCodeLoginPayload.__name__])
    @decrypt_code_field
    def post(self):
        args = EmailCodeLoginPayload.model_validate(console_ns.payload)

        original_email = args.email
        user_email = original_email.lower()
        language = args.language

        token_data = AccountService.get_email_code_login_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        normalized_token_email = token_email.lower() if isinstance(token_email, str) else token_email
        if normalized_token_email != user_email:
            raise InvalidEmailError()

        if token_data["code"] != args.code:
            raise EmailCodeError()

        AccountService.revoke_email_code_login_token(args.token)
        try:
            account = _get_account_with_case_fallback(original_email)
        except AccountRegisterError:
            raise AccountInFreezeError()
        if account:
            tenants = TenantService.get_join_tenants(account)
            if not tenants:
                workspaces = FeatureService.get_system_features().license.workspaces
                if not workspaces.is_available():
                    raise WorkspacesLimitExceeded()
                if not FeatureService.get_system_features().is_allow_create_workspace:
                    raise NotAllowedCreateWorkspace()
                else:
                    new_tenant = TenantService.create_tenant(f"{account.name}'s Workspace")
                    TenantService.create_tenant_member(new_tenant, account, role="owner")
                    account.current_tenant = new_tenant
                    tenant_was_created.send(new_tenant)

        if account is None:
            try:
                account = AccountService.create_account_and_tenant(
                    email=user_email,
                    name=user_email,
                    interface_language=get_valid_language(language),
                )
            except WorkSpaceNotAllowedCreateError:
                raise NotAllowedCreateWorkspace()
            except AccountRegisterError:
                raise AccountInFreezeError()
            except WorkspacesLimitExceededError:
                raise WorkspacesLimitExceeded()
        token_pair = AccountService.login(account, ip_address=extract_remote_ip(request))
        AccountService.reset_login_error_rate_limit(user_email)

        # Create response with cookies instead of returning tokens in body
        response = make_response({"result": "success"})

        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)
        # Set HTTP-only secure cookies for tokens
        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        return response


@console_ns.route("/refresh-token")
class RefreshTokenApi(Resource):
    def post(self):
        # Get refresh token from cookie instead of request body
        refresh_token = extract_refresh_token(request)

        if not refresh_token:
            return {"result": "fail", "message": "No refresh token provided"}, 401

        try:
            new_token_pair = AccountService.refresh_token(refresh_token)

            # Create response with new cookies
            response = make_response({"result": "success"})

            # Update cookies with new tokens
            set_csrf_token_to_cookie(request, response, new_token_pair.csrf_token)
            set_access_token_to_cookie(request, response, new_token_pair.access_token)
            set_refresh_token_to_cookie(request, response, new_token_pair.refresh_token)
            return response
        except Exception as e:
            return {"result": "fail", "message": str(e)}, 401


def _get_account_with_case_fallback(email: str):
    account = AccountService.get_user_through_email(email)
    if account or email == email.lower():
        return account

    return AccountService.get_user_through_email(email.lower())


def _authenticate_account_with_case_fallback(
    original_email: str, normalized_email: str, password: str, invite_token: str | None
):
    try:
        return AccountService.authenticate(original_email, password, invite_token)
    except services.errors.account.AccountPasswordError:
        if original_email == normalized_email:
            raise
        return AccountService.authenticate(normalized_email, password, invite_token)

```

### api/controllers/console/auth/oauth.py
```py
import logging
import urllib.parse

import httpx
from flask import current_app, redirect, request
from flask_restx import Resource
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Unauthorized

from configs import dify_config
from constants.languages import languages
from events.tenant_event import tenant_was_created
from extensions.ext_database import db
from libs.datetime_utils import naive_utc_now
from libs.helper import extract_remote_ip
from libs.oauth import GitHubOAuth, GoogleOAuth, OAuthUserInfo
from libs.token import (
    set_access_token_to_cookie,
    set_csrf_token_to_cookie,
    set_refresh_token_to_cookie,
)
from models import Account, AccountStatus
from services.account_service import AccountService, RegisterService, TenantService
from services.billing_service import BillingService
from services.errors.account import AccountNotFoundError, AccountRegisterError
from services.errors.workspace import WorkSpaceNotAllowedCreateError, WorkSpaceNotFoundError
from services.feature_service import FeatureService

from .. import console_ns

logger = logging.getLogger(__name__)


def get_oauth_providers():
    with current_app.app_context():
        if not dify_config.GITHUB_CLIENT_ID or not dify_config.GITHUB_CLIENT_SECRET:
            github_oauth = None
        else:
            github_oauth = GitHubOAuth(
                client_id=dify_config.GITHUB_CLIENT_ID,
                client_secret=dify_config.GITHUB_CLIENT_SECRET,
                redirect_uri=dify_config.CONSOLE_API_URL + "/console/api/oauth/authorize/github",
            )
        if not dify_config.GOOGLE_CLIENT_ID or not dify_config.GOOGLE_CLIENT_SECRET:
            google_oauth = None
        else:
            google_oauth = GoogleOAuth(
                client_id=dify_config.GOOGLE_CLIENT_ID,
                client_secret=dify_config.GOOGLE_CLIENT_SECRET,
                redirect_uri=dify_config.CONSOLE_API_URL + "/console/api/oauth/authorize/google",
            )

        OAUTH_PROVIDERS = {"github": github_oauth, "google": google_oauth}
        return OAUTH_PROVIDERS


@console_ns.route("/oauth/login/<provider>")
class OAuthLogin(Resource):
    @console_ns.doc("oauth_login")
    @console_ns.doc(description="Initiate OAuth login process")
    @console_ns.doc(
        params={"provider": "OAuth provider name (github/google)", "invite_token": "Optional invitation token"}
    )
    @console_ns.response(302, "Redirect to OAuth authorization URL")
    @console_ns.response(400, "Invalid provider")
    def get(self, provider: str):
        invite_token = request.args.get("invite_token") or None
        OAUTH_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400

        auth_url = oauth_provider.get_authorization_url(invite_token=invite_token)
        return redirect(auth_url)


@console_ns.route("/oauth/authorize/<provider>")
class OAuthCallback(Resource):
    @console_ns.doc("oauth_callback")
    @console_ns.doc(description="Handle OAuth callback and complete login process")
    @console_ns.doc(
        params={
            "provider": "OAuth provider name (github/google)",
            "code": "Authorization code from OAuth provider",
            "state": "Optional state parameter (used for invite token)",
        }
    )
    @console_ns.response(302, "Redirect to console with access token")
    @console_ns.response(400, "OAuth process failed")
    def get(self, provider: str):
        OAUTH_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400

        code = request.args.get("code")
        state = request.args.get("state")
        invite_token = None
        if state:
            invite_token = state

        if not code:
            return {"error": "Authorization code is required"}, 400

        try:
            token = oauth_provider.get_access_token(code)
            user_info = oauth_provider.get_user_info(token)
        except httpx.RequestError as e:
            error_text = str(e)
            if isinstance(e, httpx.HTTPStatusError):
                error_text = e.response.text
            logger.exception("An error occurred during the OAuth process with %s: %s", provider, error_text)
            return {"error": "OAuth process failed"}, 400
        except ValueError as e:
            logger.warning("OAuth error with %s", provider, exc_info=True)
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message={urllib.parse.quote(str(e))}")

        if invite_token and RegisterService.is_valid_invite_token(invite_token):
            invitation = RegisterService.get_invitation_by_token(token=invite_token)
            if invitation:
                invitation_email = invitation.get("email", None)
                invitation_email_normalized = (
                    invitation_email.lower() if isinstance(invitation_email, str) else invitation_email
                )
                if invitation_email_normalized != user_info.email.lower():
                    return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Invalid invitation token.")

            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin/invite-settings?invite_token={invite_token}")

        try:
            account, oauth_new_user = _generate_account(provider, user_info)
        except AccountNotFoundError:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Account not found.")
        except (WorkSpaceNotFoundError, WorkSpaceNotAllowedCreateError):
            return redirect(
                f"{dify_config.CONSOLE_WEB_URL}/signin"
                "?message=Workspace not found, please contact system admin to invite you to join in a workspace."
            )
        except AccountRegisterError as e:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message={e.description}")

        # Check account status
        if account.status == AccountStatus.BANNED:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Account is banned.")

        if account.status == AccountStatus.PENDING:
            account.status = AccountStatus.ACTIVE
            account.initialized_at = naive_utc_now()
            db.session.commit()

        try:
            TenantService.create_owner_tenant_if_not_exist(account)
        except Unauthorized:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Workspace not found.")
        except WorkSpaceNotAllowedCreateError:
            return redirect(
                f"{dify_config.CONSOLE_WEB_URL}/signin"
                "?message=Workspace not found, please contact system admin to invite you to join in a workspace."
            )

        token_pair = AccountService.login(
            account=account,
            ip_address=extract_remote_ip(request),
        )

        base_url = dify_config.CONSOLE_WEB_URL
        query_char = "&" if "?" in base_url else "?"
        target_url = f"{base_url}{query_char}oauth_new_user={str(oauth_new_user).lower()}"
        response = redirect(target_url)

        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)
        return response


def _get_account_by_openid_or_email(provider: str, user_info: OAuthUserInfo) -> Account | None:
    account: Account | None = Account.get_by_openid(provider, user_info.id)

    if not account:
        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(user_info.email, session=session)

    return account


def _generate_account(provider: str, user_info: OAuthUserInfo) -> tuple[Account, bool]:
    # Get account by openid or email.
    account = _get_account_by_openid_or_email(provider, user_info)
    oauth_new_user = False

    if account:
        tenants = TenantService.get_join_tenants(account)
        if not tenants:
            if not FeatureService.get_system_features().is_allow_create_workspace:
                raise WorkSpaceNotAllowedCreateError()
            else:
                new_tenant = TenantService.create_tenant(f"{account.name}'s Workspace")
                TenantService.create_tenant_member(new_tenant, account, role="owner")
                account.current_tenant = new_tenant
                tenant_was_created.send(new_tenant)

    if not account:
        normalized_email = user_info.email.lower()
        oauth_new_user = True
        if not FeatureService.get_system_features().is_allow_register:
            if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
                raise AccountRegisterError(
                    description=(
                        "This email account has been deleted within the past "
                        "30 days and is temporarily unavailable for new account registration"
                    )
                )
            else:
                raise AccountRegisterError(description=("Invalid email or password"))
        account_name = user_info.name or "Dify"
        account = RegisterService.register(
            email=normalized_email,
            name=account_name,
            password=None,
            open_id=user_info.id,
            provider=provider,
        )

        # Set interface language
        preferred_lang = request.accept_languages.best_match(languages)
        if preferred_lang and preferred_lang in languages:
            interface_language = preferred_lang
        else:
            interface_language = languages[0]
        account.interface_language = interface_language
        db.session.commit()

    # Link account
    AccountService.link_account_integrate(provider, user_info.id, account)

    return account, oauth_new_user

```

### api/controllers/console/auth/email_register.py
```py
from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from constants.languages import languages
from controllers.console import console_ns
from controllers.console.auth.error import (
    EmailAlreadyInUseError,
    EmailCodeError,
    EmailRegisterLimitError,
    InvalidEmailError,
    InvalidTokenError,
    PasswordMismatchError,
)
from extensions.ext_database import db
from libs.helper import EmailStr, extract_remote_ip
from libs.password import valid_password
from models import Account
from services.account_service import AccountService
from services.billing_service import BillingService
from services.errors.account import AccountNotFoundError, AccountRegisterError

from ..error import AccountInFreezeError, EmailSendIpLimitError
from ..wraps import email_password_login_enabled, email_register_enabled, setup_required

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class EmailRegisterSendPayload(BaseModel):
    email: EmailStr = Field(..., description="Email address")
    language: str | None = Field(default=None, description="Language code")


class EmailRegisterValidityPayload(BaseModel):
    email: EmailStr = Field(...)
    code: str = Field(...)
    token: str = Field(...)


class EmailRegisterResetPayload(BaseModel):
    token: str = Field(...)
    new_password: str = Field(...)
    password_confirm: str = Field(...)

    @field_validator("new_password", "password_confirm")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


for model in (EmailRegisterSendPayload, EmailRegisterValidityPayload, EmailRegisterResetPayload):
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/email-register/send-email")
class EmailRegisterSendEmailApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterSendPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()
        language = "en-US"
        if args.language in languages:
            language = args.language

        if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
            raise AccountInFreezeError()

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(args.email, session=session)
        token = AccountService.send_email_register_email(email=normalized_email, account=account, language=language)
        return {"result": "success", "data": token}


@console_ns.route("/email-register/validity")
class EmailRegisterCheckApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterValidityPayload.model_validate(console_ns.payload)

        user_email = args.email.lower()

        is_email_register_error_rate_limit = AccountService.is_email_register_error_rate_limit(user_email)
        if is_email_register_error_rate_limit:
            raise EmailRegisterLimitError()

        token_data = AccountService.get_email_register_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        normalized_token_email = token_email.lower() if isinstance(token_email, str) else token_email

        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if args.code != token_data.get("code"):
            AccountService.add_email_register_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_email_register_token(args.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_email_register_token(
            user_email, code=args.code, additional_data={"phase": "register"}
        )

        AccountService.reset_email_register_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@console_ns.route("/email-register")
class EmailRegisterResetApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterResetPayload.model_validate(console_ns.payload)

        # Validate passwords match
        if args.new_password != args.password_confirm:
            raise PasswordMismatchError()

        # Validate token and get register data
        register_data = AccountService.get_email_register_data(args.token)
        if not register_data:
            raise InvalidTokenError()
        # Must use token in reset phase
        if register_data.get("phase", "") != "register":
            raise InvalidTokenError()

        # Revoke token to prevent reuse
        AccountService.revoke_email_register_token(args.token)

        email = register_data.get("email", "")
        normalized_email = email.lower()

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(email, session=session)

            if account:
                raise EmailAlreadyInUseError()
            else:
                account = self._create_new_account(normalized_email, args.password_confirm)
                if not account:
                    raise AccountNotFoundError()
                token_pair = AccountService.login(account=account, ip_address=extract_remote_ip(request))
                AccountService.reset_login_error_rate_limit(normalized_email)

        return {"result": "success", "data": token_pair.model_dump()}

    def _create_new_account(self, email: str, password: str) -> Account | None:
        # Create new account if allowed
        account = None
        try:
            account = AccountService.create_account_and_tenant(
                email=email,
                name=email,
                password=password,
                interface_language=languages[0],
            )
        except AccountRegisterError:
            raise AccountInFreezeError()

        return account

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-001.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
