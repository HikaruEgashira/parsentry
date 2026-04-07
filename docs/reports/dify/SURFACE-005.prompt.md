You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-005
- **Kind**: endpoint
- **Identifier**: POST /v1/completion-messages, POST /v1/chat-messages, POST /v1/workflows/run
- **Description**: Service API endpoints for LLM completion, chat, and workflow execution. Accept user-controlled inputs that flow into prompt templates and tool invocations - prompt injection and variable interpolation risks
- **Locations**: api/controllers/service_api/app/completion.py, api/controllers/service_api/app/workflow.py

## Source Code

### api/controllers/service_api/app/completion.py
```py
import logging
from typing import Any, Literal
from uuid import UUID

from flask import request
from flask_restx import Resource
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, field_validator
from werkzeug.exceptions import BadRequest, InternalServerError, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
    AppUnavailableError,
    CompletionRequestError,
    ConversationCompletedError,
    NotChatAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from core.helper.trace_id_helper import get_external_trace_id
from libs import helper
from libs.helper import UUIDStrOrEmpty
from models.model import App, AppMode, EndUser
from services.app_generate_service import AppGenerateService
from services.app_task_service import AppTaskService
from services.errors.app import IsDraftWorkflowError, WorkflowIdFormatError, WorkflowNotFoundError
from services.errors.llm import InvokeRateLimitError

logger = logging.getLogger(__name__)


class CompletionRequestPayload(BaseModel):
    inputs: dict[str, Any]
    query: str = Field(default="")
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None
    retriever_from: str = Field(default="dev")


class ChatRequestPayload(BaseModel):
    inputs: dict[str, Any]
    query: str
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None
    conversation_id: UUIDStrOrEmpty | None = Field(default=None, description="Conversation UUID")
    retriever_from: str = Field(default="dev")
    auto_generate_name: bool = Field(default=True, description="Auto generate conversation name")
    workflow_id: str | None = Field(default=None, description="Workflow ID for advanced chat")

    @field_validator("conversation_id", mode="before")
    @classmethod
    def normalize_conversation_id(cls, value: str | UUID | None) -> str | None:
        """Allow missing or blank conversation IDs; enforce UUID format when provided."""
        if isinstance(value, str):
            value = value.strip()

        if not value:
            return None

        try:
            return helper.uuid_value(value)
        except ValueError as exc:
            raise ValueError("conversation_id must be a valid UUID") from exc


register_schema_models(service_api_ns, CompletionRequestPayload, ChatRequestPayload)


@service_api_ns.route("/completion-messages")
class CompletionApi(Resource):
    @service_api_ns.expect(service_api_ns.models[CompletionRequestPayload.__name__])
    @service_api_ns.doc("create_completion")
    @service_api_ns.doc(description="Create a completion for the given prompt")
    @service_api_ns.doc(
        responses={
            200: "Completion created successfully",
            400: "Bad request - invalid parameters",
            401: "Unauthorized - invalid API token",
            404: "Conversation not found",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Create a completion for the given prompt.

        This endpoint generates a completion based on the provided inputs and query.
        Supports both blocking and streaming response modes.
        """
        if app_model.mode != AppMode.COMPLETION:
            raise AppUnavailableError()

        payload = CompletionRequestPayload.model_validate(service_api_ns.payload or {})
        external_trace_id = get_external_trace_id(request)
        args = payload.model_dump(exclude_none=True)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id

        streaming = payload.response_mode == "streaming"

        args["auto_generate_name"] = False

        try:
            response = AppGenerateService.generate(
                app_model=app_model,
                user=end_user,
                args=args,
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=streaming,
            )

            return helper.compact_generate_response(response)
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except services.errors.conversation.ConversationCompletedError:
            raise ConversationCompletedError()
        except services.errors.app_model_config.AppModelConfigBrokenError:
            logger.exception("App model config broken.")
            raise AppUnavailableError()
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise e
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/completion-messages/<string:task_id>/stop")
class CompletionStopApi(Resource):
    @service_api_ns.doc("stop_completion")
    @service_api_ns.doc(description="Stop a running completion task")
    @service_api_ns.doc(params={"task_id": "The ID of the task to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running completion task."""
        if app_model.mode != AppMode.COMPLETION:
            raise AppUnavailableError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.SERVICE_API,
            user_id=end_user.id,
            app_mode=AppMode.value_of(app_model.mode),
        )

        return {"result": "success"}, 200


@service_api_ns.route("/chat-messages")
class ChatApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ChatRequestPayload.__name__])
    @service_api_ns.doc("create_chat_message")
    @service_api_ns.doc(description="Send a message in a chat conversation")
    @service_api_ns.doc(
        responses={
            200: "Message sent successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Conversation or workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Send a message in a chat conversation.

        This endpoint handles chat messages for chat, agent chat, and advanced chat applications.
        Supports conversation management and both blocking and streaming response modes.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        payload = ChatRequestPayload.model_validate(service_api_ns.payload or {})

        external_trace_id = get_external_trace_id(request)
        args = payload.model_dump(exclude_none=True)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id

        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
        except WorkflowNotFoundError as ex:
            raise NotFound(str(ex))
        except IsDraftWorkflowError as ex:
            raise BadRequest(str(ex))
        except WorkflowIdFormatError as ex:
            raise BadRequest(str(ex))
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except services.errors.conversation.ConversationCompletedError:
            raise ConversationCompletedError()
        except services.errors.app_model_config.AppModelConfigBrokenError:
            logger.exception("App model config broken.")
            raise AppUnavailableError()
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise e
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/chat-messages/<string:task_id>/stop")
class ChatStopApi(Resource):
    @service_api_ns.doc("stop_chat_message")
    @service_api_ns.doc(description="Stop a running chat message generation")
    @service_api_ns.doc(params={"task_id": "The ID of the task to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running chat message generation."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.SERVICE_API,
            user_id=end_user.id,
            app_mode=app_mode,
        )

        return {"result": "success"}, 200

```

### api/controllers/service_api/app/workflow.py
```py
import logging
from typing import Any, Literal

from dateutil.parser import isoparse
from flask import request
from flask_restx import Namespace, Resource, fields
from graphon.enums import WorkflowExecutionStatus
from graphon.graph_engine.manager import GraphEngineManager
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, InternalServerError, NotFound

from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
    CompletionRequestError,
    NotWorkflowAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.apps.base_app_queue_manager import AppQueueManager
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from core.helper.trace_id_helper import get_external_trace_id
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from fields.workflow_app_log_fields import build_workflow_app_log_pagination_model
from libs import helper
from libs.helper import OptionalTimestampField, TimestampField
from models.model import App, AppMode, EndUser
from models.workflow import WorkflowRun
from repositories.factory import DifyAPIRepositoryFactory
from services.app_generate_service import AppGenerateService
from services.errors.app import IsDraftWorkflowError, WorkflowIdFormatError, WorkflowNotFoundError
from services.errors.llm import InvokeRateLimitError
from services.workflow_app_service import WorkflowAppService

logger = logging.getLogger(__name__)


class WorkflowRunPayload(BaseModel):
    inputs: dict[str, Any]
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None


class WorkflowLogQuery(BaseModel):
    keyword: str | None = None
    status: Literal["succeeded", "failed", "stopped"] | None = None
    created_at__before: str | None = None
    created_at__after: str | None = None
    created_by_end_user_session_id: str | None = None
    created_by_account: str | None = None
    page: int = Field(default=1, ge=1, le=99999)
    limit: int = Field(default=20, ge=1, le=100)


register_schema_models(service_api_ns, WorkflowRunPayload, WorkflowLogQuery)


class WorkflowRunStatusField(fields.Raw):
    def output(self, key, obj: WorkflowRun, **kwargs):
        return obj.status.value


class WorkflowRunOutputsField(fields.Raw):
    def output(self, key, obj: WorkflowRun, **kwargs):
        if obj.status == WorkflowExecutionStatus.PAUSED:
            return {}

        outputs = obj.outputs_dict
        return outputs or {}


workflow_run_fields = {
    "id": fields.String,
    "workflow_id": fields.String,
    "status": WorkflowRunStatusField,
    "inputs": fields.Raw,
    "outputs": WorkflowRunOutputsField,
    "error": fields.String,
    "total_steps": fields.Integer,
    "total_tokens": fields.Integer,
    "created_at": TimestampField,
    "finished_at": OptionalTimestampField,
    "elapsed_time": fields.Float,
}


def build_workflow_run_model(api_or_ns: Namespace):
    """Build the workflow run model for the API or Namespace."""
    return api_or_ns.model("WorkflowRun", workflow_run_fields)


@service_api_ns.route("/workflows/run/<string:workflow_run_id>")
class WorkflowRunDetailApi(Resource):
    @service_api_ns.doc("get_workflow_run_detail")
    @service_api_ns.doc(description="Get workflow run details")
    @service_api_ns.doc(params={"workflow_run_id": "Workflow run ID"})
    @service_api_ns.doc(
        responses={
            200: "Workflow run details retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Workflow run not found",
        }
    )
    @validate_app_token
    @service_api_ns.marshal_with(build_workflow_run_model(service_api_ns))
    def get(self, app_model: App, workflow_run_id: str):
        """Get a workflow task running detail.

        Returns detailed information about a specific workflow run.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in [AppMode.WORKFLOW, AppMode.ADVANCED_CHAT]:
            raise NotWorkflowAppError()

        # Use repository to get workflow run
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

        workflow_run = workflow_run_repo.get_workflow_run_by_id(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            run_id=workflow_run_id,
        )
        if not workflow_run:
            raise NotFound("Workflow run not found.")
        return workflow_run


@service_api_ns.route("/workflows/run")
class WorkflowRunApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowRunPayload.__name__])
    @service_api_ns.doc("run_workflow")
    @service_api_ns.doc(description="Execute a workflow")
    @service_api_ns.doc(
        responses={
            200: "Workflow executed successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Execute a workflow.

        Runs a workflow with the provided inputs and returns the results.
        Supports both blocking and streaming response modes.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        payload = WorkflowRunPayload.model_validate(service_api_ns.payload or {})
        args = payload.model_dump(exclude_none=True)
        external_trace_id = get_external_trace_id(request)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id
        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise e
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/workflows/<string:workflow_id>/run")
class WorkflowRunByIdApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowRunPayload.__name__])
    @service_api_ns.doc("run_workflow_by_id")
    @service_api_ns.doc(description="Execute a specific workflow by ID")
    @service_api_ns.doc(params={"workflow_id": "Workflow ID to execute"})
    @service_api_ns.doc(
        responses={
            200: "Workflow executed successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, workflow_id: str):
        """Run specific workflow by ID.

        Executes a specific workflow version identified by its ID.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        payload = WorkflowRunPayload.model_validate(service_api_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        # Add workflow_id to args for AppGenerateService
        args["workflow_id"] = workflow_id

        external_trace_id = get_external_trace_id(request)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id
        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
        except WorkflowNotFoundError as ex:
            raise NotFound(str(ex))
        except IsDraftWorkflowError as ex:
            raise BadRequest(str(ex))
        except WorkflowIdFormatError as ex:
            raise BadRequest(str(ex))
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise e
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/workflows/tasks/<string:task_id>/stop")
class WorkflowTaskStopApi(Resource):
    @service_api_ns.doc("stop_workflow_task")
    @service_api_ns.doc(description="Stop a running workflow task")
    @service_api_ns.doc(params={"task_id": "Task ID to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running workflow task."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        # Stop using both mechanisms for backward compatibility
        # Legacy stop flag mechanism (without user check)
        AppQueueManager.set_stop_flag_no_user_check(task_id)

        # New graph engine command channel mechanism
        GraphEngineManager(redis_client).send_stop_command(task_id)

        return {"result": "success"}


@service_api_ns.route("/workflows/logs")
class WorkflowAppLogApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowLogQuery.__name__])
    @service_api_ns.doc("get_workflow_logs")
    @service_api_ns.doc(description="Get workflow execution logs")
    @service_api_ns.doc(
        responses={
            200: "Logs retrieved successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @validate_app_token
    @service_api_ns.marshal_with(build_workflow_app_log_pagination_model(service_api_ns))
    def get(self, app_model: App):
        """Get workflow app logs.

        Returns paginated workflow execution logs with filtering options.
        """
        args = WorkflowLogQuery.model_validate(request.args.to_dict())

        status = WorkflowExecutionStatus(args.status) if args.status else None
        created_at_before = isoparse(args.created_at__before) if args.created_at__before else None
        created_at_after = isoparse(args.created_at__after) if args.created_at__after else None

        # get paginate workflow app logs
        workflow_app_service = WorkflowAppService()
        with sessionmaker(db.engine).begin() as session:
            workflow_app_log_pagination = workflow_app_service.get_paginate_workflow_app_logs(
                session=session,
                app_model=app_model,
                keyword=args.keyword,
                status=status,
                created_at_before=created_at_before,
                created_at_after=created_at_after,
                page=args.page,
                limit=args.limit,
                created_by_end_user_session_id=args.created_by_end_user_session_id,
                created_by_account=args.created_by_account,
            )

            return workflow_app_log_pagination

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-005.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
