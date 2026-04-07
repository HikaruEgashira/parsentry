You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-002
- **Kind**: endpoint
- **Identifier**: POST /api/login, POST /api/email-code-login, POST /api/forgot-password/resets
- **Description**: Public web app authentication with password reset flow. Password reset token validation and email-based login require review for token predictability and rate limiting
- **Locations**: api/controllers/web/login.py, api/controllers/web/forgot_password.py

## Source Code

### api/controllers/web/login.py
```py
from flask import make_response, request
from flask_restx import Resource
from jwt import InvalidTokenError
from pydantic import BaseModel, Field, field_validator

import services
from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    InvalidEmailError,
)
from controllers.console.error import AccountBannedError
from controllers.console.wraps import (
    decrypt_code_field,
    decrypt_password_field,
    only_edition_enterprise,
    setup_required,
)
from controllers.web import web_ns
from controllers.web.wraps import decode_jwt_token
from libs.helper import EmailStr
from libs.passport import PassportService
from libs.password import valid_password
from libs.token import (
    clear_webapp_access_token_from_cookie,
    extract_webapp_access_token,
)
from services.account_service import AccountService
from services.app_service import AppService
from services.webapp_auth_service import WebAppAuthService


class LoginPayload(BaseModel):
    email: EmailStr
    password: str

    @field_validator("password")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


class EmailCodeLoginSendPayload(BaseModel):
    email: EmailStr
    language: str | None = None


class EmailCodeLoginVerifyPayload(BaseModel):
    email: EmailStr
    code: str
    token: str = Field(min_length=1)


register_schema_models(web_ns, LoginPayload, EmailCodeLoginSendPayload, EmailCodeLoginVerifyPayload)


@web_ns.route("/login")
class LoginApi(Resource):
    """Resource for web app email/password login."""

    @web_ns.expect(web_ns.models[LoginPayload.__name__])
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("web_app_login")
    @web_ns.doc(description="Authenticate user for web application access")
    @web_ns.doc(
        responses={
            200: "Authentication successful",
            400: "Bad request - invalid email or password format",
            401: "Authentication failed - email or password mismatch",
            403: "Account banned or login disabled",
            404: "Account not found",
        }
    )
    @decrypt_password_field
    def post(self):
        """Authenticate user and login."""
        payload = LoginPayload.model_validate(web_ns.payload or {})

        try:
            account = WebAppAuthService.authenticate(payload.email, payload.password)
        except services.errors.account.AccountLoginError:
            raise AccountBannedError()
        except services.errors.account.AccountPasswordError:
            raise AuthenticationFailedError()
        except services.errors.account.AccountNotFoundError:
            raise AuthenticationFailedError()

        token = WebAppAuthService.login(account=account)
        response = make_response({"result": "success", "data": {"access_token": token}})
        # set_access_token_to_cookie(request, response, token, samesite="None", httponly=False)
        return response


# this api helps frontend to check whether user is authenticated
# TODO: remove in the future. frontend should redirect to login page by catching 401 status
@web_ns.route("/login/status")
class LoginStatusApi(Resource):
    @setup_required
    @web_ns.doc("web_app_login_status")
    @web_ns.doc(description="Check login status")
    @web_ns.doc(
        responses={
            200: "Login status",
            401: "Login status",
        }
    )
    def get(self):
        app_code = request.args.get("app_code")
        user_id = request.args.get("user_id")
        token = extract_webapp_access_token(request)
        if not app_code:
            return {
                "logged_in": bool(token),
                "app_logged_in": False,
            }
        app_id = AppService.get_app_id_by_code(app_code)
        is_public = not dify_config.ENTERPRISE_ENABLED or not WebAppAuthService.is_app_require_permission_check(
            app_id=app_id
        )
        user_logged_in = False

        if is_public:
            user_logged_in = True
        else:
            try:
                PassportService().verify(token=token)
                user_logged_in = True
            except Exception:
                user_logged_in = False

        try:
            _ = decode_jwt_token(app_code=app_code, user_id=user_id)
            app_logged_in = True
        except Exception:
            app_logged_in = False

        return {
            "logged_in": user_logged_in,
            "app_logged_in": app_logged_in,
        }


@web_ns.route("/logout")
class LogoutApi(Resource):
    @setup_required
    @web_ns.doc("web_app_logout")
    @web_ns.doc(description="Logout user from web application")
    @web_ns.doc(
        responses={
            200: "Logout successful",
        }
    )
    def post(self):
        response = make_response({"result": "success"})
        # enterprise SSO sets same site to None in https deployment
        # so we need to logout by calling api
        clear_webapp_access_token_from_cookie(response, samesite="None")
        return response


@web_ns.route("/email-code-login")
class EmailCodeLoginSendEmailApi(Resource):
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("send_email_code_login")
    @web_ns.doc(description="Send email verification code for login")
    @web_ns.expect(web_ns.models[EmailCodeLoginSendPayload.__name__])
    @web_ns.doc(
        responses={
            200: "Email code sent successfully",
            400: "Bad request - invalid email format",
            404: "Account not found",
        }
    )
    def post(self):
        payload = EmailCodeLoginSendPayload.model_validate(web_ns.payload or {})

        if payload.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        account = WebAppAuthService.get_user_through_email(payload.email)
        if account is None:
            raise AuthenticationFailedError()
        else:
            token = WebAppAuthService.send_email_code_login_email(account=account, language=language)
        return {"result": "success", "data": token}


@web_ns.route("/email-code-login/validity")
class EmailCodeLoginApi(Resource):
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("verify_email_code_login")
    @web_ns.doc(description="Verify email code and complete login")
    @web_ns.expect(web_ns.models[EmailCodeLoginVerifyPayload.__name__])
    @web_ns.doc(
        responses={
            200: "Email code verified and login successful",
            400: "Bad request - invalid code or token",
            401: "Invalid token or expired code",
            404: "Account not found",
        }
    )
    @decrypt_code_field
    def post(self):
        payload = EmailCodeLoginVerifyPayload.model_validate(web_ns.payload or {})

        user_email = payload.email.lower()

        token_data = WebAppAuthService.get_email_code_login_data(payload.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        if not isinstance(token_email, str):
            raise InvalidEmailError()
        normalized_token_email = token_email.lower()
        if normalized_token_email != user_email:
            raise InvalidEmailError()

        if token_data["code"] != payload.code:
            raise EmailCodeError()

        WebAppAuthService.revoke_email_code_login_token(payload.token)
        account = WebAppAuthService.get_user_through_email(token_email)
        if not account:
            raise AuthenticationFailedError()

        token = WebAppAuthService.login(account=account)
        AccountService.reset_login_error_rate_limit(user_email)
        response = make_response({"result": "success", "data": {"access_token": token}})
        # set_access_token_to_cookie(request, response, token, samesite="None", httponly=False)
        return response

```

### api/controllers/web/forgot_password.py
```py
import base64
import secrets

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from controllers.common.schema import register_schema_models
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    EmailPasswordResetLimitError,
    InvalidEmailError,
    InvalidTokenError,
    PasswordMismatchError,
)
from controllers.console.error import EmailSendIpLimitError
from controllers.console.wraps import email_password_login_enabled, only_edition_enterprise, setup_required
from controllers.web import web_ns
from extensions.ext_database import db
from libs.helper import EmailStr, extract_remote_ip
from libs.password import hash_password, valid_password
from models.account import Account
from services.account_service import AccountService


class ForgotPasswordSendPayload(BaseModel):
    email: EmailStr
    language: str | None = None


class ForgotPasswordCheckPayload(BaseModel):
    email: EmailStr
    code: str
    token: str = Field(min_length=1)


class ForgotPasswordResetPayload(BaseModel):
    token: str = Field(min_length=1)
    new_password: str
    password_confirm: str

    @field_validator("new_password", "password_confirm")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


register_schema_models(web_ns, ForgotPasswordSendPayload, ForgotPasswordCheckPayload, ForgotPasswordResetPayload)


@web_ns.route("/forgot-password")
class ForgotPasswordSendEmailApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordSendPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("send_forgot_password_email")
    @web_ns.doc(description="Send password reset email")
    @web_ns.doc(
        responses={
            200: "Password reset email sent successfully",
            400: "Bad request - invalid email format",
            404: "Account not found",
            429: "Too many requests - rate limit exceeded",
        }
    )
    def post(self):
        payload = ForgotPasswordSendPayload.model_validate(web_ns.payload or {})

        request_email = payload.email
        normalized_email = request_email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if payload.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(request_email, session=session)
        token = None
        if account is None:
            raise AuthenticationFailedError()
        else:
            token = AccountService.send_reset_password_email(account=account, email=normalized_email, language=language)

        return {"result": "success", "data": token}


@web_ns.route("/forgot-password/validity")
class ForgotPasswordCheckApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordCheckPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("check_forgot_password_token")
    @web_ns.doc(description="Verify password reset token validity")
    @web_ns.doc(
        responses={200: "Token is valid", 400: "Bad request - invalid token format", 401: "Invalid or expired token"}
    )
    def post(self):
        payload = ForgotPasswordCheckPayload.model_validate(web_ns.payload or {})

        user_email = payload.email.lower()

        is_forgot_password_error_rate_limit = AccountService.is_forgot_password_error_rate_limit(user_email)
        if is_forgot_password_error_rate_limit:
            raise EmailPasswordResetLimitError()

        token_data = AccountService.get_reset_password_data(payload.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        if not isinstance(token_email, str):
            raise InvalidEmailError()
        normalized_token_email = token_email.lower()

        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if payload.code != token_data.get("code"):
            AccountService.add_forgot_password_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_reset_password_token(payload.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_reset_password_token(
            token_email, code=payload.code, additional_data={"phase": "reset"}
        )

        AccountService.reset_forgot_password_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@web_ns.route("/forgot-password/resets")
class ForgotPasswordResetApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordResetPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("reset_password")
    @web_ns.doc(description="Reset user password with verification token")
    @web_ns.doc(
        responses={
            200: "Password reset successfully",
            400: "Bad request - invalid parameters or password mismatch",
            401: "Invalid or expired token",
            404: "Account not found",
        }
    )
    def post(self):
        payload = ForgotPasswordResetPayload.model_validate(web_ns.payload or {})

        # Validate passwords match
        if payload.new_password != payload.password_confirm:
            raise PasswordMismatchError()

        # Validate token and get reset data
        reset_data = AccountService.get_reset_password_data(payload.token)
        if not reset_data:
            raise InvalidTokenError()
        # Must use token in reset phase
        if reset_data.get("phase", "") != "reset":
            raise InvalidTokenError()

        # Revoke token to prevent reuse
        AccountService.revoke_reset_password_token(payload.token)

        # Generate secure salt and hash password
        salt = secrets.token_bytes(16)
        password_hashed = hash_password(payload.new_password, salt)

        email = reset_data.get("email", "")

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(email, session=session)

            if account:
                self._update_existing_account(account, password_hashed, salt)
            else:
                raise AuthenticationFailedError()

        return {"result": "success"}

    def _update_existing_account(self, account: Account, password_hashed, salt):
        # Update existing account credentials
        account.password = base64.b64encode(password_hashed).decode()
        account.password_salt = base64.b64encode(salt).decode()

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-002.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
