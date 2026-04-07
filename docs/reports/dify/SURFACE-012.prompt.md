You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-012
- **Kind**: endpoint
- **Identifier**: POST /inner/api/invoke/*, POST /inner/api/upload/file/request
- **Description**: Internal API endpoints for plugin invocation, workspace management, and email sending with no visible authentication. If exposed beyond internal network, allows arbitrary LLM invocation, tool execution, and email sending
- **Locations**: api/controllers/inner_api/plugin/plugin.py, api/controllers/inner_api/workspace/workspace.py, api/controllers/inner_api/mail.py

## Source Code

### api/controllers/inner_api/plugin/plugin.py
```py
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder

from controllers.console.wraps import setup_required
from controllers.inner_api import inner_api_ns
from controllers.inner_api.plugin.wraps import get_user_tenant, plugin_data
from controllers.inner_api.wraps import plugin_inner_api_only
from core.plugin.backwards_invocation.app import PluginAppBackwardsInvocation
from core.plugin.backwards_invocation.base import BaseBackwardsInvocationResponse
from core.plugin.backwards_invocation.encrypt import PluginEncrypter
from core.plugin.backwards_invocation.model import PluginModelBackwardsInvocation
from core.plugin.backwards_invocation.node import PluginNodeBackwardsInvocation
from core.plugin.backwards_invocation.tool import PluginToolBackwardsInvocation
from core.plugin.entities.request import (
    RequestFetchAppInfo,
    RequestInvokeApp,
    RequestInvokeEncrypt,
    RequestInvokeLLM,
    RequestInvokeLLMWithStructuredOutput,
    RequestInvokeModeration,
    RequestInvokeParameterExtractorNode,
    RequestInvokeQuestionClassifierNode,
    RequestInvokeRerank,
    RequestInvokeSpeech2Text,
    RequestInvokeSummary,
    RequestInvokeTextEmbedding,
    RequestInvokeTool,
    RequestInvokeTTS,
    RequestRequestUploadFile,
)
from core.tools.entities.tool_entities import ToolProviderType
from core.tools.signature import get_signed_file_url_for_plugin
from libs.helper import length_prefixed_response
from models import Account, Tenant
from models.model import EndUser


@inner_api_ns.route("/invoke/llm")
class PluginInvokeLLMApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeLLM)
    @inner_api_ns.doc("plugin_invoke_llm")
    @inner_api_ns.doc(description="Invoke LLM models through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "LLM invocation successful (streaming response)",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeLLM):
        def generator():
            response = PluginModelBackwardsInvocation.invoke_llm(user_model.id, tenant_model, payload)
            return PluginModelBackwardsInvocation.convert_to_event_stream(response)

        return length_prefixed_response(0xF, generator())


@inner_api_ns.route("/invoke/llm/structured-output")
class PluginInvokeLLMWithStructuredOutputApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeLLMWithStructuredOutput)
    @inner_api_ns.doc("plugin_invoke_llm_structured")
    @inner_api_ns.doc(description="Invoke LLM models with structured output through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "LLM structured output invocation successful (streaming response)",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeLLMWithStructuredOutput):
        def generator():
            response = PluginModelBackwardsInvocation.invoke_llm_with_structured_output(
                user_model.id, tenant_model, payload
            )
            return PluginModelBackwardsInvocation.convert_to_event_stream(response)

        return length_prefixed_response(0xF, generator())


@inner_api_ns.route("/invoke/text-embedding")
class PluginInvokeTextEmbeddingApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeTextEmbedding)
    @inner_api_ns.doc("plugin_invoke_text_embedding")
    @inner_api_ns.doc(description="Invoke text embedding models through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Text embedding successful",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeTextEmbedding):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginModelBackwardsInvocation.invoke_text_embedding(
                        user_id=user_model.id,
                        tenant=tenant_model,
                        payload=payload,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/rerank")
class PluginInvokeRerankApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeRerank)
    @inner_api_ns.doc("plugin_invoke_rerank")
    @inner_api_ns.doc(description="Invoke rerank models through plugin interface")
    @inner_api_ns.doc(
        responses={200: "Rerank successful", 401: "Unauthorized - invalid API key", 404: "Service not available"}
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeRerank):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginModelBackwardsInvocation.invoke_rerank(
                        user_id=user_model.id,
                        tenant=tenant_model,
                        payload=payload,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/tts")
class PluginInvokeTTSApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeTTS)
    @inner_api_ns.doc("plugin_invoke_tts")
    @inner_api_ns.doc(description="Invoke text-to-speech models through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "TTS invocation successful (streaming response)",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeTTS):
        def generator():
            response = PluginModelBackwardsInvocation.invoke_tts(
                user_id=user_model.id,
                tenant=tenant_model,
                payload=payload,
            )
            return PluginModelBackwardsInvocation.convert_to_event_stream(response)

        return length_prefixed_response(0xF, generator())


@inner_api_ns.route("/invoke/speech2text")
class PluginInvokeSpeech2TextApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeSpeech2Text)
    @inner_api_ns.doc("plugin_invoke_speech2text")
    @inner_api_ns.doc(description="Invoke speech-to-text models through plugin interface")
    @inner_api_ns.doc(
        responses={200: "Speech2Text successful", 401: "Unauthorized - invalid API key", 404: "Service not available"}
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeSpeech2Text):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginModelBackwardsInvocation.invoke_speech2text(
                        user_id=user_model.id,
                        tenant=tenant_model,
                        payload=payload,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/moderation")
class PluginInvokeModerationApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeModeration)
    @inner_api_ns.doc("plugin_invoke_moderation")
    @inner_api_ns.doc(description="Invoke moderation models through plugin interface")
    @inner_api_ns.doc(
        responses={200: "Moderation successful", 401: "Unauthorized - invalid API key", 404: "Service not available"}
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeModeration):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginModelBackwardsInvocation.invoke_moderation(
                        user_id=user_model.id,
                        tenant=tenant_model,
                        payload=payload,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/tool")
class PluginInvokeToolApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeTool)
    @inner_api_ns.doc("plugin_invoke_tool")
    @inner_api_ns.doc(description="Invoke tools through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Tool invocation successful (streaming response)",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeTool):
        def generator():
            return PluginToolBackwardsInvocation.convert_to_event_stream(
                PluginToolBackwardsInvocation.invoke_tool(
                    tenant_id=tenant_model.id,
                    user_id=user_model.id,
                    tool_type=ToolProviderType.value_of(payload.tool_type),
                    provider=payload.provider,
                    tool_name=payload.tool,
                    tool_parameters=payload.tool_parameters,
                    credential_id=payload.credential_id,
                ),
            )

        return length_prefixed_response(0xF, generator())


@inner_api_ns.route("/invoke/parameter-extractor")
class PluginInvokeParameterExtractorNodeApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeParameterExtractorNode)
    @inner_api_ns.doc("plugin_invoke_parameter_extractor")
    @inner_api_ns.doc(description="Invoke parameter extractor node through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Parameter extraction successful",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeParameterExtractorNode):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginNodeBackwardsInvocation.invoke_parameter_extractor(
                        tenant_id=tenant_model.id,
                        user_id=user_model.id,
                        parameters=payload.parameters,
                        model_config=payload.model,
                        instruction=payload.instruction,
                        query=payload.query,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/question-classifier")
class PluginInvokeQuestionClassifierNodeApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeQuestionClassifierNode)
    @inner_api_ns.doc("plugin_invoke_question_classifier")
    @inner_api_ns.doc(description="Invoke question classifier node through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Question classification successful",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeQuestionClassifierNode):
        try:
            return jsonable_encoder(
                BaseBackwardsInvocationResponse(
                    data=PluginNodeBackwardsInvocation.invoke_question_classifier(
                        tenant_id=tenant_model.id,
                        user_id=user_model.id,
                        query=payload.query,
                        model_config=payload.model,
                        classes=payload.classes,
                        instruction=payload.instruction,
                    )
                )
            )
        except Exception as e:
            return jsonable_encoder(BaseBackwardsInvocationResponse(error=str(e)))


@inner_api_ns.route("/invoke/app")
class PluginInvokeAppApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeApp)
    @inner_api_ns.doc("plugin_invoke_app")
    @inner_api_ns.doc(description="Invoke application through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "App invocation successful (streaming response)",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeApp):
        response = PluginAppBackwardsInvocation.invoke_app(
            app_id=payload.app_id,
            user_id=user_model.id,
            tenant_id=tenant_model.id,
            conversation_id=payload.conversation_id,
            query=payload.query,
            stream=payload.response_mode == "streaming",
            inputs=payload.inputs,
            files=payload.files,
        )

        return length_prefixed_response(0xF, PluginAppBackwardsInvocation.convert_to_event_stream(response))


@inner_api_ns.route("/invoke/encrypt")
class PluginInvokeEncryptApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeEncrypt)
    @inner_api_ns.doc("plugin_invoke_encrypt")
    @inner_api_ns.doc(description="Encrypt or decrypt data through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Encryption/decryption successful",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeEncrypt):
        """
        encrypt or decrypt data
        """
        try:
            return BaseBackwardsInvocationResponse(
                data=PluginEncrypter.invoke_encrypt(tenant_model, payload)
            ).model_dump()
        except Exception as e:
            return BaseBackwardsInvocationResponse(error=str(e)).model_dump()


@inner_api_ns.route("/invoke/summary")
class PluginInvokeSummaryApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestInvokeSummary)
    @inner_api_ns.doc("plugin_invoke_summary")
    @inner_api_ns.doc(description="Invoke summary functionality through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Summary generation successful",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestInvokeSummary):
        try:
            return BaseBackwardsInvocationResponse(
                data={
                    "summary": PluginModelBackwardsInvocation.invoke_summary(
                        user_id=user_model.id,
                        tenant=tenant_model,
                        payload=payload,
                    )
                }
            ).model_dump()
        except Exception as e:
            return BaseBackwardsInvocationResponse(error=str(e)).model_dump()


@inner_api_ns.route("/upload/file/request")
class PluginUploadFileRequestApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestRequestUploadFile)
    @inner_api_ns.doc("plugin_upload_file_request")
    @inner_api_ns.doc(description="Request signed URL for file upload through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "Signed URL generated successfully",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestRequestUploadFile):
        # generate signed url
        url = get_signed_file_url_for_plugin(
            filename=payload.filename,
            mimetype=payload.mimetype,
            tenant_id=tenant_model.id,
            user_id=user_model.id,
        )
        return BaseBackwardsInvocationResponse(data={"url": url}).model_dump()


@inner_api_ns.route("/fetch/app/info")
class PluginFetchAppInfoApi(Resource):
    @get_user_tenant
    @setup_required
    @plugin_inner_api_only
    @plugin_data(payload_type=RequestFetchAppInfo)
    @inner_api_ns.doc("plugin_fetch_app_info")
    @inner_api_ns.doc(description="Fetch application information through plugin interface")
    @inner_api_ns.doc(
        responses={
            200: "App information retrieved successfully",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self, user_model: Account | EndUser, tenant_model: Tenant, payload: RequestFetchAppInfo):
        return BaseBackwardsInvocationResponse(
            data=PluginAppBackwardsInvocation.fetch_app_info(payload.app_id, tenant_model.id)
        ).model_dump()

```

### api/controllers/inner_api/workspace/workspace.py
```py
import json

from flask_restx import Resource
from pydantic import BaseModel
from sqlalchemy import select

from controllers.common.schema import register_schema_models
from controllers.console.wraps import setup_required
from controllers.inner_api import inner_api_ns
from controllers.inner_api.wraps import enterprise_inner_api_only
from events.tenant_event import tenant_was_created
from extensions.ext_database import db
from models import Account
from services.account_service import TenantService


class WorkspaceCreatePayload(BaseModel):
    name: str
    owner_email: str


class WorkspaceOwnerlessPayload(BaseModel):
    name: str


register_schema_models(inner_api_ns, WorkspaceCreatePayload, WorkspaceOwnerlessPayload)


@inner_api_ns.route("/enterprise/workspace")
class EnterpriseWorkspace(Resource):
    @setup_required
    @enterprise_inner_api_only
    @inner_api_ns.doc("create_enterprise_workspace")
    @inner_api_ns.doc(description="Create a new enterprise workspace with owner assignment")
    @inner_api_ns.expect(inner_api_ns.models[WorkspaceCreatePayload.__name__])
    @inner_api_ns.doc(
        responses={
            200: "Workspace created successfully",
            401: "Unauthorized - invalid API key",
            404: "Owner account not found or service not available",
        }
    )
    def post(self):
        args = WorkspaceCreatePayload.model_validate(inner_api_ns.payload or {})

        account = db.session.scalar(select(Account).where(Account.email == args.owner_email).limit(1))
        if account is None:
            return {"message": "owner account not found."}, 404

        tenant = TenantService.create_tenant(args.name, is_from_dashboard=True)
        TenantService.create_tenant_member(tenant, account, role="owner")

        tenant_was_created.send(tenant)

        resp = {
            "id": tenant.id,
            "name": tenant.name,
            "plan": tenant.plan,
            "status": tenant.status,
            "created_at": tenant.created_at.isoformat() + "Z" if tenant.created_at else None,
            "updated_at": tenant.updated_at.isoformat() + "Z" if tenant.updated_at else None,
        }

        return {
            "message": "enterprise workspace created.",
            "tenant": resp,
        }


@inner_api_ns.route("/enterprise/workspace/ownerless")
class EnterpriseWorkspaceNoOwnerEmail(Resource):
    @setup_required
    @enterprise_inner_api_only
    @inner_api_ns.doc("create_enterprise_workspace_ownerless")
    @inner_api_ns.doc(description="Create a new enterprise workspace without initial owner assignment")
    @inner_api_ns.expect(inner_api_ns.models[WorkspaceOwnerlessPayload.__name__])
    @inner_api_ns.doc(
        responses={
            200: "Workspace created successfully",
            401: "Unauthorized - invalid API key",
            404: "Service not available",
        }
    )
    def post(self):
        args = WorkspaceOwnerlessPayload.model_validate(inner_api_ns.payload or {})

        tenant = TenantService.create_tenant(args.name, is_from_dashboard=True)

        tenant_was_created.send(tenant)

        resp = {
            "id": tenant.id,
            "name": tenant.name,
            "encrypt_public_key": tenant.encrypt_public_key,
            "plan": tenant.plan,
            "status": tenant.status,
            "custom_config": json.loads(tenant.custom_config) if tenant.custom_config else {},
            "created_at": tenant.created_at.isoformat() + "Z" if tenant.created_at else None,
            "updated_at": tenant.updated_at.isoformat() + "Z" if tenant.updated_at else None,
        }

        return {
            "message": "enterprise workspace created.",
            "tenant": resp,
        }

```

### api/controllers/inner_api/mail.py
```py
from typing import Any

from flask_restx import Resource
from pydantic import BaseModel, Field

from controllers.common.schema import register_schema_model
from controllers.console.wraps import setup_required
from controllers.inner_api import inner_api_ns
from controllers.inner_api.wraps import billing_inner_api_only, enterprise_inner_api_only
from tasks.mail_inner_task import send_inner_email_task


class InnerMailPayload(BaseModel):
    to: list[str] = Field(description="Recipient email addresses", min_length=1)
    subject: str
    body: str
    substitutions: dict[str, Any] | None = None


register_schema_model(inner_api_ns, InnerMailPayload)


class BaseMail(Resource):
    """Shared logic for sending an inner email."""

    @inner_api_ns.doc("send_inner_mail")
    @inner_api_ns.doc(description="Send internal email")
    @inner_api_ns.expect(inner_api_ns.models[InnerMailPayload.__name__])
    def post(self):
        args = InnerMailPayload.model_validate(inner_api_ns.payload or {})
        send_inner_email_task.delay(
            to=args.to,
            subject=args.subject,
            body=args.body,
            substitutions=args.substitutions,  # type: ignore
        )
        return {"message": "success"}, 200


@inner_api_ns.route("/enterprise/mail")
class EnterpriseMail(BaseMail):
    method_decorators = [setup_required, enterprise_inner_api_only]

    @inner_api_ns.doc("send_enterprise_mail")
    @inner_api_ns.doc(description="Send internal email for enterprise features")
    @inner_api_ns.expect(inner_api_ns.models[InnerMailPayload.__name__])
    @inner_api_ns.doc(
        responses={200: "Email sent successfully", 401: "Unauthorized - invalid API key", 404: "Service not available"}
    )
    def post(self):
        """Send internal email for enterprise features.

        This endpoint allows sending internal emails for enterprise-specific
        notifications and communications.

        Returns:
            dict: Success message with status code 200
        """
        return super().post()


@inner_api_ns.route("/billing/mail")
class BillingMail(BaseMail):
    method_decorators = [setup_required, billing_inner_api_only]

    @inner_api_ns.doc("send_billing_mail")
    @inner_api_ns.doc(description="Send internal email for billing notifications")
    @inner_api_ns.expect(inner_api_ns.models[InnerMailPayload.__name__])
    @inner_api_ns.doc(
        responses={200: "Email sent successfully", 401: "Unauthorized - invalid API key", 404: "Service not available"}
    )
    def post(self):
        """Send internal email for billing notifications.

        This endpoint allows sending internal emails for billing-related
        notifications and alerts.

        Returns:
            dict: Success message with status code 200
        """
        return super().post()

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-012.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
