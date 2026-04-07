You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-009
- **Kind**: endpoint
- **Identifier**: POST /mcp/server/<server_code>/mcp (JSON-RPC 2.0)
- **Description**: Model Context Protocol server accepting JSON-RPC requests without explicit authentication. MCP tool invocation can trigger arbitrary tool execution and data access
- **Locations**: api/controllers/mcp/mcp.py, api/core/mcp/server/streamable_http.py, api/core/mcp/client/streamable_client.py, api/core/mcp/auth/auth_flow.py, api/core/mcp/session/base_session.py

## Source Code

### api/controllers/mcp/mcp.py
```py
from typing import Any, Union

from flask import Response
from flask_restx import Resource
from graphon.variables.input_entities import VariableEntity
from pydantic import BaseModel, Field, ValidationError
from sqlalchemy import select
from sqlalchemy.orm import Session, sessionmaker

from controllers.common.schema import register_schema_model
from controllers.mcp import mcp_ns
from core.mcp import types as mcp_types
from core.mcp.server.streamable_http import handle_mcp_request
from extensions.ext_database import db
from libs import helper
from models.enums import AppMCPServerStatus
from models.model import App, AppMCPServer, AppMode, EndUser


class MCPRequestError(Exception):
    """Custom exception for MCP request processing errors"""

    def __init__(self, error_code: int, message: str):
        self.error_code = error_code
        self.message = message
        super().__init__(message)


class MCPRequestPayload(BaseModel):
    jsonrpc: str = Field(description="JSON-RPC version (should be '2.0')")
    method: str = Field(description="The method to invoke")
    params: dict[str, Any] | None = Field(default=None, description="Parameters for the method")
    id: int | str | None = Field(default=None, description="Request ID for tracking responses")


register_schema_model(mcp_ns, MCPRequestPayload)


@mcp_ns.route("/server/<string:server_code>/mcp")
class MCPAppApi(Resource):
    @mcp_ns.expect(mcp_ns.models[MCPRequestPayload.__name__])
    @mcp_ns.doc("handle_mcp_request")
    @mcp_ns.doc(description="Handle Model Context Protocol (MCP) requests for a specific server")
    @mcp_ns.doc(params={"server_code": "Unique identifier for the MCP server"})
    @mcp_ns.doc(
        responses={
            200: "MCP response successfully processed",
            400: "Invalid MCP request or parameters",
            404: "Server or app not found",
        }
    )
    def post(self, server_code: str):
        """Handle MCP requests for a specific server.

        Processes JSON-RPC formatted requests according to the Model Context Protocol specification.
        Validates the server status and associated app before processing the request.

        Args:
            server_code: Unique identifier for the MCP server

        Returns:
            dict: JSON-RPC response from the MCP handler

        Raises:
            ValidationError: Invalid request format or parameters
        """
        args = MCPRequestPayload.model_validate(mcp_ns.payload or {})
        request_id: Union[int, str] | None = args.id
        mcp_request = self._parse_mcp_request(args.model_dump(exclude_none=True))

        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            # Get MCP server and app
            mcp_server, app = self._get_mcp_server_and_app(server_code, session)
            self._validate_server_status(mcp_server)

            # Get user input form
            user_input_form = self._get_user_input_form(app)

            # Handle notification vs request differently
            return self._process_mcp_message(mcp_request, request_id, app, mcp_server, user_input_form, session)

    def _get_mcp_server_and_app(self, server_code: str, session: Session) -> tuple[AppMCPServer, App]:
        """Get and validate MCP server and app in one query session"""
        mcp_server = session.scalar(select(AppMCPServer).where(AppMCPServer.server_code == server_code).limit(1))
        if not mcp_server:
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "Server Not Found")

        app = session.scalar(select(App).where(App.id == mcp_server.app_id).limit(1))
        if not app:
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "App Not Found")

        return mcp_server, app

    def _validate_server_status(self, mcp_server: AppMCPServer):
        """Validate MCP server status"""
        if mcp_server.status != AppMCPServerStatus.ACTIVE:
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "Server is not active")

    def _process_mcp_message(
        self,
        mcp_request: mcp_types.ClientRequest | mcp_types.ClientNotification,
        request_id: Union[int, str] | None,
        app: App,
        mcp_server: AppMCPServer,
        user_input_form: list[VariableEntity],
        session: Session,
    ) -> Response:
        """Process MCP message (notification or request)"""
        if isinstance(mcp_request, mcp_types.ClientNotification):
            return self._handle_notification(mcp_request)
        else:
            return self._handle_request(mcp_request, request_id, app, mcp_server, user_input_form, session)

    def _handle_notification(self, mcp_request: mcp_types.ClientNotification) -> Response:
        """Handle MCP notification"""
        # For notifications, only support init notification
        if mcp_request.root.method != "notifications/initialized":
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "Invalid notification method")
        # Return HTTP 202 Accepted for notifications (no response body)
        return Response("", status=202, content_type="application/json")

    def _handle_request(
        self,
        mcp_request: mcp_types.ClientRequest,
        request_id: Union[int, str] | None,
        app: App,
        mcp_server: AppMCPServer,
        user_input_form: list[VariableEntity],
        session: Session,
    ) -> Response:
        """Handle MCP request"""
        if request_id is None:
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "Request ID is required")

        result = self._handle_mcp_request(app, mcp_server, mcp_request, user_input_form, session, request_id)
        if result is None:
            # This shouldn't happen for requests, but handle gracefully
            raise MCPRequestError(mcp_types.INTERNAL_ERROR, "No response generated for request")

        return helper.compact_generate_response(result.model_dump(by_alias=True, mode="json", exclude_none=True))

    def _get_user_input_form(self, app: App) -> list[VariableEntity]:
        """Get and convert user input form"""
        # Get raw user input form based on app mode
        if app.mode in {AppMode.ADVANCED_CHAT, AppMode.WORKFLOW}:
            if not app.workflow:
                raise MCPRequestError(mcp_types.INVALID_REQUEST, "App is unavailable")
            raw_user_input_form = app.workflow.user_input_form(to_old_structure=True)
        else:
            if not app.app_model_config:
                raise MCPRequestError(mcp_types.INVALID_REQUEST, "App is unavailable")
            features_dict = app.app_model_config.to_dict()
            raw_user_input_form = features_dict.get("user_input_form", [])

        # Convert to VariableEntity objects
        try:
            return self._convert_user_input_form(raw_user_input_form)
        except ValidationError as e:
            raise MCPRequestError(mcp_types.INVALID_PARAMS, f"Invalid user_input_form: {str(e)}")

    def _convert_user_input_form(self, raw_form: list[dict]) -> list[VariableEntity]:
        """Convert raw user input form to VariableEntity objects"""
        return [self._create_variable_entity(item) for item in raw_form]

    def _create_variable_entity(self, item: dict) -> VariableEntity:
        """Create a single VariableEntity from raw form item"""
        variable_type = item.get("type", "") or list(item.keys())[0]
        variable = item[variable_type]

        return VariableEntity(
            type=variable_type,
            variable=variable.get("variable"),
            description=variable.get("description") or "",
            label=variable.get("label"),
            required=variable.get("required", False),
            max_length=variable.get("max_length"),
            options=variable.get("options") or [],
            json_schema=variable.get("json_schema"),
        )

    def _parse_mcp_request(self, args: dict) -> mcp_types.ClientRequest | mcp_types.ClientNotification:
        """Parse and validate MCP request"""
        try:
            return mcp_types.ClientRequest.model_validate(args)
        except ValidationError:
            try:
                return mcp_types.ClientNotification.model_validate(args)
            except ValidationError as e:
                raise MCPRequestError(mcp_types.INVALID_PARAMS, f"Invalid MCP request: {str(e)}")

    def _retrieve_end_user(self, tenant_id: str, mcp_server_id: str) -> EndUser | None:
        """Get end user - manages its own database session"""
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            return session.scalar(
                select(EndUser)
                .where(EndUser.tenant_id == tenant_id)
                .where(EndUser.session_id == mcp_server_id)
                .where(EndUser.type == "mcp")
                .limit(1)
            )

    def _create_end_user(
        self, client_name: str, tenant_id: str, app_id: str, mcp_server_id: str, session: Session
    ) -> EndUser:
        """Create end user in existing session"""
        end_user = EndUser(
            tenant_id=tenant_id,
            app_id=app_id,
            type="mcp",
            name=client_name,
            session_id=mcp_server_id,
        )
        session.add(end_user)
        session.flush()  # Use flush instead of commit to keep transaction open
        session.refresh(end_user)
        return end_user

    def _handle_mcp_request(
        self,
        app: App,
        mcp_server: AppMCPServer,
        mcp_request: mcp_types.ClientRequest,
        user_input_form: list[VariableEntity],
        session: Session,
        request_id: Union[int, str],
    ) -> mcp_types.JSONRPCResponse | mcp_types.JSONRPCError | None:
        """Handle MCP request and return response"""
        end_user = self._retrieve_end_user(mcp_server.tenant_id, mcp_server.id)

        if not end_user and isinstance(mcp_request.root, mcp_types.InitializeRequest):
            client_info = mcp_request.root.params.clientInfo
            client_name = f"{client_info.name}@{client_info.version}"
            with sessionmaker(db.engine, expire_on_commit=False).begin() as create_session:
                end_user = self._create_end_user(client_name, app.tenant_id, app.id, mcp_server.id, create_session)

        return handle_mcp_request(app, mcp_request, user_input_form, mcp_server, end_user, request_id)

```

### api/core/mcp/server/streamable_http.py
```py
import json
import logging
from collections.abc import Mapping
from typing import Any, NotRequired, TypedDict, cast

from graphon.variables.input_entities import VariableEntity, VariableEntityType

from configs import dify_config
from core.app.entities.app_invoke_entities import InvokeFrom
from core.app.features.rate_limiting.rate_limit import RateLimitGenerator
from core.mcp import types as mcp_types
from models.model import App, AppMCPServer, AppMode, EndUser
from services.app_generate_service import AppGenerateService

logger = logging.getLogger(__name__)


class ToolParameterSchemaDict(TypedDict):
    type: str
    properties: dict[str, Any]
    required: list[str]


class ToolArgumentsDict(TypedDict):
    query: NotRequired[str]
    inputs: dict[str, Any]


def handle_mcp_request(
    app: App,
    request: mcp_types.ClientRequest,
    user_input_form: list[VariableEntity],
    mcp_server: AppMCPServer,
    end_user: EndUser | None = None,
    request_id: int | str = 1,
) -> mcp_types.JSONRPCResponse | mcp_types.JSONRPCError:
    """
    Handle MCP request and return JSON-RPC response

    Args:
        app: The Dify app instance
        request: The JSON-RPC request message
        user_input_form: List of variable entities for the app
        mcp_server: The MCP server configuration
        end_user: Optional end user
        request_id: The request ID

    Returns:
        JSON-RPC response or error
    """

    request_type = type(request.root)
    request_root = request.root

    def create_success_response(result_data: mcp_types.Result) -> mcp_types.JSONRPCResponse:
        """Create success response with business result data"""
        return mcp_types.JSONRPCResponse(
            jsonrpc="2.0",
            id=request_id,
            result=result_data.model_dump(by_alias=True, mode="json", exclude_none=True),
        )

    def create_error_response(code: int, message: str) -> mcp_types.JSONRPCError:
        """Create error response with error code and message"""
        from core.mcp.types import ErrorData

        error_data = ErrorData(code=code, message=message)
        return mcp_types.JSONRPCError(
            jsonrpc="2.0",
            id=request_id,
            error=error_data,
        )

    try:
        # Dispatch request to appropriate handler based on instance type
        if isinstance(request_root, mcp_types.InitializeRequest):
            return create_success_response(handle_initialize(mcp_server.description))
        elif isinstance(request_root, mcp_types.ListToolsRequest):
            return create_success_response(
                handle_list_tools(
                    app.name, app.mode, user_input_form, mcp_server.description, mcp_server.parameters_dict
                )
            )
        elif isinstance(request_root, mcp_types.CallToolRequest):
            return create_success_response(handle_call_tool(app, request, user_input_form, end_user))
        elif isinstance(request_root, mcp_types.PingRequest):
            return create_success_response(handle_ping())
        else:
            return create_error_response(mcp_types.METHOD_NOT_FOUND, f"Method not found: {request_type.__name__}")

    except ValueError as e:
        logger.exception("Invalid params")
        return create_error_response(mcp_types.INVALID_PARAMS, str(e))
    except Exception as e:
        logger.exception("Internal server error")
        return create_error_response(mcp_types.INTERNAL_ERROR, "Internal server error: " + str(e))


def handle_ping() -> mcp_types.EmptyResult:
    """Handle ping request"""
    return mcp_types.EmptyResult()


def handle_initialize(description: str) -> mcp_types.InitializeResult:
    """Handle initialize request"""
    capabilities = mcp_types.ServerCapabilities(
        tools=mcp_types.ToolsCapability(listChanged=False),
    )

    return mcp_types.InitializeResult(
        protocolVersion=mcp_types.SERVER_LATEST_PROTOCOL_VERSION,
        capabilities=capabilities,
        serverInfo=mcp_types.Implementation(name="Dify", version=dify_config.project.version),
        instructions=description,
    )


def handle_list_tools(
    app_name: str,
    app_mode: str,
    user_input_form: list[VariableEntity],
    description: str,
    parameters_dict: dict[str, str],
) -> mcp_types.ListToolsResult:
    """Handle list tools request"""
    parameter_schema = build_parameter_schema(app_mode, user_input_form, parameters_dict)

    return mcp_types.ListToolsResult(
        tools=[
            mcp_types.Tool(
                name=app_name,
                description=description,
                inputSchema=cast(dict[str, Any], parameter_schema),
            )
        ],
    )


def handle_call_tool(
    app: App,
    request: mcp_types.ClientRequest,
    user_input_form: list[VariableEntity],
    end_user: EndUser | None,
) -> mcp_types.CallToolResult:
    """Handle call tool request"""
    request_obj = cast(mcp_types.CallToolRequest, request.root)
    args = prepare_tool_arguments(app, request_obj.params.arguments or {})

    if not end_user:
        raise ValueError("End user not found")

    response = AppGenerateService.generate(
        app,
        end_user,
        args,
        InvokeFrom.SERVICE_API,
        streaming=app.mode == AppMode.AGENT_CHAT,
    )

    answer = extract_answer_from_response(app, response)
    return mcp_types.CallToolResult(content=[mcp_types.TextContent(text=answer, type="text")])


def build_parameter_schema(
    app_mode: str,
    user_input_form: list[VariableEntity],
    parameters_dict: dict[str, str],
) -> ToolParameterSchemaDict:
    """Build parameter schema for the tool"""
    parameters, required = convert_input_form_to_parameters(user_input_form, parameters_dict)

    if app_mode in {AppMode.COMPLETION, AppMode.WORKFLOW}:
        return {
            "type": "object",
            "properties": parameters,
            "required": required,
        }
    return {
        "type": "object",
        "properties": {
            "query": {"type": "string", "description": "User Input/Question content"},
            **parameters,
        },
        "required": ["query", *required],
    }


def prepare_tool_arguments(app: App, arguments: dict[str, Any]) -> ToolArgumentsDict:
    """Prepare arguments based on app mode"""
    if app.mode == AppMode.WORKFLOW:
        return {"inputs": arguments}
    elif app.mode == AppMode.COMPLETION:
        return {"query": "", "inputs": arguments}
    else:
        # Chat modes - create a copy to avoid modifying original dict
        args_copy = arguments.copy()
        query = args_copy.pop("query", "")
        return {"query": query, "inputs": args_copy}


def extract_answer_from_response(app: App, response: Any) -> str:
    """Extract answer from app generate response"""
    answer = ""

    if isinstance(response, RateLimitGenerator):
        answer = process_streaming_response(response)
    elif isinstance(response, Mapping):
        answer = process_mapping_response(app, response)
    else:
        logger.warning("Unexpected response type: %s", type(response))

    return answer


def process_streaming_response(response: RateLimitGenerator) -> str:
    """Process streaming response for agent chat mode"""
    answer = ""
    for item in response.generator:
        if isinstance(item, str) and item.startswith("data: "):
            try:
                json_str = item[6:].strip()
                parsed_data = json.loads(json_str)
                if parsed_data.get("event") == "agent_thought":
                    answer += parsed_data.get("thought", "")
            except json.JSONDecodeError:
                continue
    return answer


def process_mapping_response(app: App, response: Mapping) -> str:
    """Process mapping response based on app mode"""
    if app.mode in {
        AppMode.ADVANCED_CHAT,
        AppMode.COMPLETION,
        AppMode.CHAT,
        AppMode.AGENT_CHAT,
    }:
        return response.get("answer", "")
    elif app.mode == AppMode.WORKFLOW:
        return json.dumps(response["data"]["outputs"], ensure_ascii=False)
    else:
        raise ValueError("Invalid app mode: " + str(app.mode))


def convert_input_form_to_parameters(
    user_input_form: list[VariableEntity],
    parameters_dict: dict[str, str],
) -> tuple[dict[str, dict[str, Any]], list[str]]:
    """Convert user input form to parameter schema"""
    parameters: dict[str, dict[str, Any]] = {}
    required = []

    for item in user_input_form:
        if item.type in (
            VariableEntityType.FILE,
            VariableEntityType.FILE_LIST,
            VariableEntityType.EXTERNAL_DATA_TOOL,
        ):
            continue
        parameters[item.variable] = {}
        if item.required:
            required.append(item.variable)
        # if the workflow republished, the parameters not changed
        # we should not raise error here
        description = parameters_dict.get(item.variable, "")
        parameters[item.variable]["description"] = description
        if item.type in (VariableEntityType.TEXT_INPUT, VariableEntityType.PARAGRAPH):
            parameters[item.variable]["type"] = "string"
        elif item.type == VariableEntityType.SELECT:
            parameters[item.variable]["type"] = "string"
            parameters[item.variable]["enum"] = item.options
        elif item.type == VariableEntityType.NUMBER:
            parameters[item.variable]["type"] = "number"
        elif item.type == VariableEntityType.CHECKBOX:
            parameters[item.variable]["type"] = "boolean"
        elif item.type == VariableEntityType.JSON_OBJECT:
            parameters[item.variable]["type"] = "object"
            if item.json_schema:
                for key in ("properties", "required", "additionalProperties"):
                    if key in item.json_schema:
                        parameters[item.variable][key] = item.json_schema[key]
    return parameters, required

```

### api/core/mcp/client/streamable_client.py
```py
"""
StreamableHTTP Client Transport Module

This module implements the StreamableHTTP transport for MCP clients,
providing support for HTTP POST requests with optional SSE streaming responses
and session management.
"""

import logging
import queue
import threading
from collections.abc import Callable, Generator
from concurrent.futures import ThreadPoolExecutor
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import timedelta
from typing import Any, cast

import httpx
from httpx_sse import EventSource, ServerSentEvent

from core.mcp.types import (
    ClientMessageMetadata,
    ErrorData,
    JSONRPCError,
    JSONRPCMessage,
    JSONRPCNotification,
    JSONRPCRequest,
    JSONRPCResponse,
    RequestId,
    SessionMessage,
)
from core.mcp.utils import create_ssrf_proxy_mcp_http_client, ssrf_proxy_sse_connect

logger = logging.getLogger(__name__)


SessionMessageOrError = SessionMessage | Exception | None
# Queue types with clearer names for their roles
ServerToClientQueue = queue.Queue[SessionMessageOrError]  # Server to client messages
ClientToServerQueue = queue.Queue[SessionMessage | None]  # Client to server messages
GetSessionIdCallback = Callable[[], str | None]

MCP_SESSION_ID = "mcp-session-id"
LAST_EVENT_ID = "last-event-id"
CONTENT_TYPE = "content-type"
ACCEPT = "Accept"


JSON = "application/json"
SSE = "text/event-stream"

DEFAULT_QUEUE_READ_TIMEOUT = 3


class StreamableHTTPError(Exception):
    """Base exception for StreamableHTTP transport errors."""


class ResumptionError(StreamableHTTPError):
    """Raised when resumption request is invalid."""


@dataclass
class RequestContext:
    """Context for a request operation."""

    client: httpx.Client
    headers: dict[str, str]
    session_id: str | None
    session_message: SessionMessage
    metadata: ClientMessageMetadata | None
    server_to_client_queue: ServerToClientQueue  # Renamed for clarity
    sse_read_timeout: float


class StreamableHTTPTransport:
    """StreamableHTTP client transport implementation."""

    def __init__(
        self,
        url: str,
        headers: dict[str, Any] | None = None,
        timeout: float | timedelta = 30,
        sse_read_timeout: float | timedelta = 60 * 5,
    ):
        """Initialize the StreamableHTTP transport.

        Args:
            url: The endpoint URL.
            headers: Optional headers to include in requests.
            timeout: HTTP timeout for regular operations.
            sse_read_timeout: Timeout for SSE read operations.
        """
        self.url = url
        self.headers = headers or {}
        self.timeout = timeout.total_seconds() if isinstance(timeout, timedelta) else timeout
        self.sse_read_timeout = (
            sse_read_timeout.total_seconds() if isinstance(sse_read_timeout, timedelta) else sse_read_timeout
        )
        self.session_id: str | None = None
        self.request_headers = {
            ACCEPT: f"{JSON}, {SSE}",
            CONTENT_TYPE: JSON,
            **self.headers,
        }
        self.stop_event = threading.Event()
        self._active_responses: list[httpx.Response] = []
        self._lock = threading.Lock()

    def _update_headers_with_session(self, base_headers: dict[str, str]) -> dict[str, str]:
        """Update headers with session ID if available."""
        headers = base_headers.copy()
        if self.session_id:
            headers[MCP_SESSION_ID] = self.session_id
        return headers

    def _register_response(self, response: httpx.Response):
        """Register a response for cleanup on shutdown."""
        with self._lock:
            self._active_responses.append(response)

    def _unregister_response(self, response: httpx.Response):
        """Unregister a response after it's closed."""
        with self._lock:
            try:
                self._active_responses.remove(response)
            except ValueError as e:
                logger.debug("Ignoring error during response unregister: %s", e)

    def close_active_responses(self):
        """Close all active SSE connections to unblock threads."""
        with self._lock:
            responses_to_close = list(self._active_responses)
            self._active_responses.clear()
            for response in responses_to_close:
                try:
                    response.close()
                except RuntimeError as e:
                    logger.debug("Ignoring error during active response close: %s", e)

    def _is_initialization_request(self, message: JSONRPCMessage) -> bool:
        """Check if the message is an initialization request."""
        return isinstance(message.root, JSONRPCRequest) and message.root.method == "initialize"

    def _is_initialized_notification(self, message: JSONRPCMessage) -> bool:
        """Check if the message is an initialized notification."""
        return isinstance(message.root, JSONRPCNotification) and message.root.method == "notifications/initialized"

    def _maybe_extract_session_id_from_response(
        self,
        response: httpx.Response,
    ):
        """Extract and store session ID from response headers."""
        new_session_id = response.headers.get(MCP_SESSION_ID)
        if new_session_id:
            self.session_id = new_session_id
            logger.info("Received session ID: %s", self.session_id)

    def _handle_sse_event(
        self,
        sse: ServerSentEvent,
        server_to_client_queue: ServerToClientQueue,
        original_request_id: RequestId | None = None,
        resumption_callback: Callable[[str], None] | None = None,
    ) -> bool:
        """Handle an SSE event, returning True if the response is complete."""
        if sse.event == "message":
            # ping event send by server will be recognized  as a message event with empty data by httpx-sse's SSEDecoder
            if not sse.data.strip():
                return False

            try:
                message = JSONRPCMessage.model_validate_json(sse.data)
                logger.debug("SSE message: %s", message)

                # If this is a response and we have original_request_id, replace it
                if original_request_id is not None and isinstance(message.root, JSONRPCResponse | JSONRPCError):
                    message.root.id = original_request_id

                session_message = SessionMessage(message)
                # Put message in queue that goes to client
                server_to_client_queue.put(session_message)

                # Call resumption token callback if we have an ID
                if sse.id and resumption_callback:
                    resumption_callback(sse.id)

                # If this is a response or error return True indicating completion
                # Otherwise, return False to continue listening
                return isinstance(message.root, JSONRPCResponse | JSONRPCError)

            except Exception as exc:
                # Put exception in queue that goes to client
                server_to_client_queue.put(exc)
                return False
        elif sse.event == "ping":
            logger.debug("Received ping event")
            return False
        else:
            logger.warning("Unknown SSE event: %s", sse.event)
            return False

    def handle_get_stream(
        self,
        client: httpx.Client,
        server_to_client_queue: ServerToClientQueue,
    ):
        """Handle GET stream for server-initiated messages."""
        try:
            if not self.session_id:
                return

            headers = self._update_headers_with_session(self.request_headers)

            with ssrf_proxy_sse_connect(
                self.url,
                headers=headers,
                timeout=httpx.Timeout(self.timeout, read=self.sse_read_timeout),
                client=client,
                method="GET",
            ) as event_source:
                event_source.response.raise_for_status()
                logger.debug("GET SSE connection established")

                # Register response for cleanup
                self._register_response(event_source.response)

                try:
                    for sse in event_source.iter_sse():
                        if self.stop_event.is_set():
                            logger.debug("GET stream received stop signal")
                            break
                        self._handle_sse_event(sse, server_to_client_queue)
                finally:
                    self._unregister_response(event_source.response)

        except Exception as exc:
            if not self.stop_event.is_set():
                logger.debug("GET stream error (non-fatal): %s", exc)

    def _handle_resumption_request(self, ctx: RequestContext):
        """Handle a resumption request using GET with SSE."""
        headers = self._update_headers_with_session(ctx.headers)
        if ctx.metadata and ctx.metadata.resumption_token:
            headers[LAST_EVENT_ID] = ctx.metadata.resumption_token
        else:
            raise ResumptionError("Resumption request requires a resumption token")

        # Extract original request ID to map responses
        original_request_id = None
        if isinstance(ctx.session_message.message.root, JSONRPCRequest):
            original_request_id = ctx.session_message.message.root.id

        with ssrf_proxy_sse_connect(
            self.url,
            headers=headers,
            timeout=httpx.Timeout(self.timeout, read=self.sse_read_timeout),
            client=ctx.client,
            method="GET",
        ) as event_source:
            event_source.response.raise_for_status()
            logger.debug("Resumption GET SSE connection established")

            # Register response for cleanup
            self._register_response(event_source.response)

            try:
                for sse in event_source.iter_sse():
                    if self.stop_event.is_set():
                        logger.debug("Resumption stream received stop signal")
                        break
                    is_complete = self._handle_sse_event(
                        sse,
                        ctx.server_to_client_queue,
                        original_request_id,
                        ctx.metadata.on_resumption_token_update if ctx.metadata else None,
                    )
                    if is_complete:
                        break
            finally:
                self._unregister_response(event_source.response)

    def _handle_post_request(self, ctx: RequestContext):
        """Handle a POST request with response processing."""
        headers = self._update_headers_with_session(ctx.headers)
        message = ctx.session_message.message
        is_initialization = self._is_initialization_request(message)

        with ctx.client.stream(
            "POST",
            self.url,
            json=message.model_dump(by_alias=True, mode="json", exclude_none=True),
            headers=headers,
        ) as response:
            if response.status_code == 202:
                logger.debug("Received 202 Accepted")
                return

            if response.status_code == 204:
                logger.debug("Received 204 No Content")
                return

            if response.status_code == 404:
                if isinstance(message.root, JSONRPCRequest):
                    self._send_session_terminated_error(
                        ctx.server_to_client_queue,
                        message.root.id,
                    )
                return

            response.raise_for_status()
            if is_initialization:
                self._maybe_extract_session_id_from_response(response)

            # Per https://modelcontextprotocol.io/specification/2025-06-18/basic#notifications:
            # The server MUST NOT send a response to notifications.
            if isinstance(message.root, JSONRPCRequest):
                content_type = cast(str, response.headers.get(CONTENT_TYPE, "").lower())

                if content_type.startswith(JSON):
                    self._handle_json_response(response, ctx.server_to_client_queue)
                elif content_type.startswith(SSE):
                    self._handle_sse_response(response, ctx)
                else:
                    self._handle_unexpected_content_type(
                        content_type,
                        ctx.server_to_client_queue,
                    )

    def _handle_json_response(
        self,
        response: httpx.Response,
        server_to_client_queue: ServerToClientQueue,
    ):
        """Handle JSON response from the server."""
        try:
            content = response.read()
            message = JSONRPCMessage.model_validate_json(content)
            session_message = SessionMessage(message)
            server_to_client_queue.put(session_message)
        except Exception as exc:
            server_to_client_queue.put(exc)

    def _handle_sse_response(self, response: httpx.Response, ctx: RequestContext):
        """Handle SSE response from the server."""
        try:
            # Register response for cleanup
            self._register_response(response)

            event_source = EventSource(response)
            try:
                for sse in event_source.iter_sse():
                    if self.stop_event.is_set():
                        logger.debug("SSE response stream received stop signal")
                        break
                    is_complete = self._handle_sse_event(
                        sse,
                        ctx.server_to_client_queue,
                        resumption_callback=(ctx.metadata.on_resumption_token_update if ctx.metadata else None),
                    )
                    if is_complete:
                        break
            finally:
                self._unregister_response(response)
        except Exception as e:
            if not self.stop_event.is_set():
                ctx.server_to_client_queue.put(e)

    def _handle_unexpected_content_type(
        self,
        content_type: str,
        server_to_client_queue: ServerToClientQueue,
    ):
        """Handle unexpected content type in response."""
        error_msg = f"Unexpected content type: {content_type}"
        logger.error(error_msg)
        server_to_client_queue.put(ValueError(error_msg))

    def _send_session_terminated_error(
        self,
        server_to_client_queue: ServerToClientQueue,
        request_id: RequestId,
    ):
        """Send a session terminated error response."""
        jsonrpc_error = JSONRPCError(
            jsonrpc="2.0",
            id=request_id,
            error=ErrorData(code=32600, message="Session terminated by server"),
        )
        session_message = SessionMessage(JSONRPCMessage(jsonrpc_error))
        server_to_client_queue.put(session_message)

    def post_writer(
        self,
        client: httpx.Client,
        client_to_server_queue: ClientToServerQueue,
        server_to_client_queue: ServerToClientQueue,
        start_get_stream: Callable[[], None],
    ):
        """Handle writing requests to the server.

        This method processes messages from the client_to_server_queue and sends them to the server.
        Responses are written to the server_to_client_queue.
        """
        while True:
            try:
                # Check if we should stop
                if self.stop_event.is_set():
                    logger.debug("Post writer received stop signal")
                    break

                # Read message from client queue with timeout to check stop_event periodically
                session_message = client_to_server_queue.get(timeout=DEFAULT_QUEUE_READ_TIMEOUT)
                if session_message is None:
                    break

                message = session_message.message
                metadata = (
                    session_message.metadata if isinstance(session_message.metadata, ClientMessageMetadata) else None
                )

                # Check if this is a resumption request
                is_resumption = bool(metadata and metadata.resumption_token)

                logger.debug("Sending client message: %s", message)

                # Handle initialized notification
                if self._is_initialized_notification(message):
                    start_get_stream()

                ctx = RequestContext(
                    client=client,
                    headers=self.request_headers,
                    session_id=self.session_id,
                    session_message=session_message,
                    metadata=metadata,
                    server_to_client_queue=server_to_client_queue,  # Queue to write responses to client
                    sse_read_timeout=self.sse_read_timeout,
                )

                if is_resumption:
                    self._handle_resumption_request(ctx)
                else:
                    self._handle_post_request(ctx)
            except queue.Empty:
                continue
            except Exception as exc:
                if not self.stop_event.is_set():
                    server_to_client_queue.put(exc)

    def terminate_session(self, client: httpx.Client):
        """Terminate the session by sending a DELETE request."""
        if not self.session_id:
            return

        try:
            headers = self._update_headers_with_session(self.request_headers)
            response = client.delete(self.url, headers=headers)

            if response.status_code == 405:
                logger.debug("Server does not allow session termination")
            elif response.status_code != 200:
                logger.warning("Session termination failed: %s", response.status_code)
        except Exception as exc:
            logger.warning("Session termination failed: %s", exc)

    def get_session_id(self) -> str | None:
        """Get the current session ID."""
        return self.session_id


@contextmanager
def streamablehttp_client(
    url: str,
    headers: dict[str, Any] | None = None,
    timeout: float | timedelta = 30,
    sse_read_timeout: float | timedelta = 60 * 5,
    terminate_on_close: bool = True,
) -> Generator[
    tuple[
        ServerToClientQueue,  # Queue for receiving messages FROM server
        ClientToServerQueue,  # Queue for sending messages TO server
        GetSessionIdCallback,
    ],
    None,
    None,
]:
    """
    Client transport for StreamableHTTP.

    `sse_read_timeout` determines how long (in seconds) the client will wait for a new
    event before disconnecting. All other HTTP operations are controlled by `timeout`.

    Yields:
        Tuple containing:
            - server_to_client_queue: Queue for reading messages FROM the server
            - client_to_server_queue: Queue for sending messages TO the server
            - get_session_id_callback: Function to retrieve the current session ID
    """
    transport = StreamableHTTPTransport(url, headers, timeout, sse_read_timeout)

    # Create queues with clear directional meaning
    server_to_client_queue: ServerToClientQueue = queue.Queue()  # For messages FROM server TO client
    client_to_server_queue: ClientToServerQueue = queue.Queue()  # For messages FROM client TO server

    executor = ThreadPoolExecutor(max_workers=2)
    try:
        with create_ssrf_proxy_mcp_http_client(
            headers=transport.request_headers,
            timeout=httpx.Timeout(transport.timeout, read=transport.sse_read_timeout),
        ) as client:
            # Define callbacks that need access to thread pool
            def start_get_stream():
                """Start a worker thread to handle server-initiated messages."""
                executor.submit(transport.handle_get_stream, client, server_to_client_queue)

            # Start the post_writer worker thread
            executor.submit(
                transport.post_writer,
                client,
                client_to_server_queue,  # Queue for messages FROM client TO server
                server_to_client_queue,  # Queue for messages FROM server TO client
                start_get_stream,
            )

            try:
                yield (
                    server_to_client_queue,  # Queue for receiving messages FROM server
                    client_to_server_queue,  # Queue for sending messages TO server
                    transport.get_session_id,
                )
            finally:
                # Set stop event to signal all threads to stop
                transport.stop_event.set()

                # Close all active SSE connections to unblock threads
                transport.close_active_responses()

                if transport.session_id and terminate_on_close:
                    transport.terminate_session(client)

                # Signal threads to stop
                client_to_server_queue.put(None)
    finally:
        # Clear any remaining items and add None sentinel to unblock any waiting threads
        try:
            while not client_to_server_queue.empty():
                client_to_server_queue.get_nowait()
        except queue.Empty:
            pass

        client_to_server_queue.put(None)
        server_to_client_queue.put(None)

        # Shutdown executor without waiting to prevent hanging
        executor.shutdown(wait=False)

```

### api/core/mcp/auth/auth_flow.py
```py
import base64
import hashlib
import json
import os
import secrets
import urllib.parse
from urllib.parse import urljoin, urlparse

import httpx
from httpx import RequestError
from pydantic import ValidationError

from core.entities.mcp_provider import MCPProviderEntity, MCPSupportGrantType
from core.helper import ssrf_proxy
from core.mcp.entities import AuthAction, AuthActionType, AuthResult, OAuthCallbackState
from core.mcp.error import MCPRefreshTokenError
from core.mcp.types import (
    LATEST_PROTOCOL_VERSION,
    OAuthClientInformation,
    OAuthClientInformationFull,
    OAuthClientMetadata,
    OAuthMetadata,
    OAuthTokens,
    ProtectedResourceMetadata,
)
from extensions.ext_redis import redis_client

OAUTH_STATE_EXPIRY_SECONDS = 5 * 60  # 5 minutes expiry
OAUTH_STATE_REDIS_KEY_PREFIX = "oauth_state:"


def generate_pkce_challenge() -> tuple[str, str]:
    """Generate PKCE challenge and verifier."""
    code_verifier = base64.urlsafe_b64encode(os.urandom(40)).decode("utf-8")
    code_verifier = code_verifier.replace("=", "").replace("+", "-").replace("/", "_")

    code_challenge_hash = hashlib.sha256(code_verifier.encode("utf-8")).digest()
    code_challenge = base64.urlsafe_b64encode(code_challenge_hash).decode("utf-8")
    code_challenge = code_challenge.replace("=", "").replace("+", "-").replace("/", "_")

    return code_verifier, code_challenge


def build_protected_resource_metadata_discovery_urls(
    www_auth_resource_metadata_url: str | None, server_url: str
) -> list[str]:
    """
    Build a list of URLs to try for Protected Resource Metadata discovery.

    Per RFC 9728 Section 5.1, supports fallback when discovery fails at one URL.
    Priority order:
    1. URL from WWW-Authenticate header (if provided)
    2. Well-known URI with path: https://example.com/.well-known/oauth-protected-resource/public/mcp
    3. Well-known URI at root: https://example.com/.well-known/oauth-protected-resource
    """
    urls = []

    parsed_server_url = urlparse(server_url)
    base_url = f"{parsed_server_url.scheme}://{parsed_server_url.netloc}"
    path = parsed_server_url.path.rstrip("/")

    # First priority: URL from WWW-Authenticate header
    if www_auth_resource_metadata_url:
        parsed_metadata_url = urlparse(www_auth_resource_metadata_url)
        normalized_metadata_url = None
        if parsed_metadata_url.scheme and parsed_metadata_url.netloc:
            normalized_metadata_url = www_auth_resource_metadata_url
        elif not parsed_metadata_url.scheme and parsed_metadata_url.netloc:
            normalized_metadata_url = f"{parsed_server_url.scheme}:{www_auth_resource_metadata_url}"
        elif (
            not parsed_metadata_url.scheme
            and not parsed_metadata_url.netloc
            and parsed_metadata_url.path.startswith("/")
        ):
            first_segment = parsed_metadata_url.path.lstrip("/").split("/", 1)[0]
            if first_segment == ".well-known" or "." not in first_segment:
                normalized_metadata_url = urljoin(base_url, parsed_metadata_url.path)

        if normalized_metadata_url:
            urls.append(normalized_metadata_url)

    # Fallback: construct from server URL
    # Priority 2: With path insertion (e.g., /.well-known/oauth-protected-resource/public/mcp)
    if path:
        path_url = f"{base_url}/.well-known/oauth-protected-resource{path}"
        if path_url not in urls:
            urls.append(path_url)

    # Priority 3: At root (e.g., /.well-known/oauth-protected-resource)
    root_url = f"{base_url}/.well-known/oauth-protected-resource"
    if root_url not in urls:
        urls.append(root_url)

    return urls


def build_oauth_authorization_server_metadata_discovery_urls(auth_server_url: str | None, server_url: str) -> list[str]:
    """
    Build a list of URLs to try for OAuth Authorization Server Metadata discovery.

    Supports both OAuth 2.0 (RFC 8414) and OpenID Connect discovery.

    Per RFC 8414 section 3.1 and section 5, try all possible endpoints:
    - OAuth 2.0 with path insertion: https://example.com/.well-known/oauth-authorization-server/tenant1
    - OpenID Connect with path insertion: https://example.com/.well-known/openid-configuration/tenant1
    - OpenID Connect path appending: https://example.com/tenant1/.well-known/openid-configuration
    - OAuth 2.0 at root: https://example.com/.well-known/oauth-authorization-server
    - OpenID Connect at root: https://example.com/.well-known/openid-configuration
    """
    urls = []
    base_url = auth_server_url or server_url

    parsed = urlparse(base_url)
    base = f"{parsed.scheme}://{parsed.netloc}"
    path = parsed.path.rstrip("/")
    # OAuth 2.0 Authorization Server Metadata at root (MCP-03-26)
    urls.append(f"{base}/.well-known/oauth-authorization-server")

    # OpenID Connect Discovery at root
    urls.append(f"{base}/.well-known/openid-configuration")

    if path:
        # OpenID Connect Discovery with path insertion
        urls.append(f"{base}/.well-known/openid-configuration{path}")

        # OpenID Connect Discovery path appending
        urls.append(f"{base}{path}/.well-known/openid-configuration")

        # OAuth 2.0 Authorization Server Metadata with path insertion
        urls.append(f"{base}/.well-known/oauth-authorization-server{path}")

    return urls


def discover_protected_resource_metadata(
    prm_url: str | None, server_url: str, protocol_version: str | None = None
) -> ProtectedResourceMetadata | None:
    """Discover OAuth 2.0 Protected Resource Metadata (RFC 9470)."""
    urls = build_protected_resource_metadata_discovery_urls(prm_url, server_url)
    headers = {"MCP-Protocol-Version": protocol_version or LATEST_PROTOCOL_VERSION, "User-Agent": "Dify"}

    for url in urls:
        try:
            response = ssrf_proxy.get(url, headers=headers)
            if response.status_code == 200:
                return ProtectedResourceMetadata.model_validate(response.json())
            elif response.status_code == 404:
                continue  # Try next URL
        except (RequestError, ValidationError):
            continue  # Try next URL

    return None


def discover_oauth_authorization_server_metadata(
    auth_server_url: str | None, server_url: str, protocol_version: str | None = None
) -> OAuthMetadata | None:
    """Discover OAuth 2.0 Authorization Server Metadata (RFC 8414)."""
    urls = build_oauth_authorization_server_metadata_discovery_urls(auth_server_url, server_url)
    headers = {"MCP-Protocol-Version": protocol_version or LATEST_PROTOCOL_VERSION, "User-Agent": "Dify"}

    for url in urls:
        try:
            response = ssrf_proxy.get(url, headers=headers)
            if response.status_code == 200:
                return OAuthMetadata.model_validate(response.json())
            elif response.status_code == 404:
                continue  # Try next URL
        except (RequestError, ValidationError):
            continue  # Try next URL

    return None


def get_effective_scope(
    scope_from_www_auth: str | None,
    prm: ProtectedResourceMetadata | None,
    asm: OAuthMetadata | None,
    client_scope: str | None,
) -> str | None:
    """
    Determine effective scope using priority-based selection strategy.

    Priority order:
    1. WWW-Authenticate header scope (server explicit requirement)
    2. Protected Resource Metadata scopes
    3. OAuth Authorization Server Metadata scopes
    4. Client configured scope
    """
    if scope_from_www_auth:
        return scope_from_www_auth

    if prm and prm.scopes_supported:
        return " ".join(prm.scopes_supported)

    if asm and asm.scopes_supported:
        return " ".join(asm.scopes_supported)

    return client_scope


def _create_secure_redis_state(state_data: OAuthCallbackState) -> str:
    """Create a secure state parameter by storing state data in Redis and returning a random state key."""
    # Generate a secure random state key
    state_key = secrets.token_urlsafe(32)

    # Store the state data in Redis with expiration
    redis_key = f"{OAUTH_STATE_REDIS_KEY_PREFIX}{state_key}"
    redis_client.setex(redis_key, OAUTH_STATE_EXPIRY_SECONDS, state_data.model_dump_json())

    return state_key


def _retrieve_redis_state(state_key: str) -> OAuthCallbackState:
    """Retrieve and decode OAuth state data from Redis using the state key, then delete it."""
    redis_key = f"{OAUTH_STATE_REDIS_KEY_PREFIX}{state_key}"

    # Get state data from Redis
    state_data = redis_client.get(redis_key)

    if not state_data:
        raise ValueError("State parameter has expired or does not exist")

    # Delete the state data from Redis immediately after retrieval to prevent reuse
    redis_client.delete(redis_key)

    try:
        # Parse and validate the state data
        oauth_state = OAuthCallbackState.model_validate_json(state_data)

        return oauth_state
    except ValidationError as e:
        raise ValueError(f"Invalid state parameter: {str(e)}")


def handle_callback(state_key: str, authorization_code: str) -> tuple[OAuthCallbackState, OAuthTokens]:
    """
    Handle the callback from the OAuth provider.

    Returns:
        A tuple of (callback_state, tokens) that can be used by the caller to save data.
    """
    # Retrieve state data from Redis (state is automatically deleted after retrieval)
    full_state_data = _retrieve_redis_state(state_key)

    tokens = exchange_authorization(
        full_state_data.server_url,
        full_state_data.metadata,
        full_state_data.client_information,
        authorization_code,
        full_state_data.code_verifier,
        full_state_data.redirect_uri,
    )

    return full_state_data, tokens


def check_support_resource_discovery(server_url: str) -> tuple[bool, str]:
    """Check if the server supports OAuth 2.0 Resource Discovery."""
    b_scheme, b_netloc, _, _, b_query, b_fragment = urlparse(server_url, "", True)
    url_for_resource_discovery = f"{b_scheme}://{b_netloc}/.well-known/oauth-protected-resource"
    if b_query:
        url_for_resource_discovery += f"?{b_query}"
    if b_fragment:
        url_for_resource_discovery += f"#{b_fragment}"
    try:
        headers = {"MCP-Protocol-Version": LATEST_PROTOCOL_VERSION, "User-Agent": "Dify"}
        response = ssrf_proxy.get(url_for_resource_discovery, headers=headers)
        if 200 <= response.status_code < 300:
            body = response.json()
            # Support both singular and plural forms
            if body.get("authorization_servers"):
                return True, body["authorization_servers"][0]
            elif body.get("authorization_server_url"):
                return True, body["authorization_server_url"][0]
            else:
                return False, ""
        return False, ""
    except RequestError:
        # Not support resource discovery, fall back to well-known OAuth metadata
        return False, ""


def discover_oauth_metadata(
    server_url: str,
    resource_metadata_url: str | None = None,
    scope_hint: str | None = None,
    protocol_version: str | None = None,
) -> tuple[OAuthMetadata | None, ProtectedResourceMetadata | None, str | None]:
    """
    Discover OAuth metadata using RFC 8414/9470 standards.

    Args:
        server_url: The MCP server URL
        resource_metadata_url: Protected Resource Metadata URL from WWW-Authenticate header
        scope_hint: Scope hint from WWW-Authenticate header
        protocol_version: MCP protocol version

    Returns:
        (oauth_metadata, protected_resource_metadata, scope_hint)
    """
    # Discover Protected Resource Metadata
    prm = discover_protected_resource_metadata(resource_metadata_url, server_url, protocol_version)

    # Get authorization server URL from PRM or use server URL
    auth_server_url = None
    if prm and prm.authorization_servers:
        auth_server_url = prm.authorization_servers[0]

    # Discover OAuth Authorization Server Metadata
    asm = discover_oauth_authorization_server_metadata(auth_server_url, server_url, protocol_version)

    return asm, prm, scope_hint


def start_authorization(
    server_url: str,
    metadata: OAuthMetadata | None,
    client_information: OAuthClientInformation,
    redirect_url: str,
    provider_id: str,
    tenant_id: str,
    scope: str | None = None,
) -> tuple[str, str]:
    """Begins the authorization flow with secure Redis state storage."""
    response_type = "code"
    code_challenge_method = "S256"

    if metadata:
        authorization_url = metadata.authorization_endpoint
        if response_type not in metadata.response_types_supported:
            raise ValueError(f"Incompatible auth server: does not support response type {response_type}")
    else:
        authorization_url = urljoin(server_url, "/authorize")

    code_verifier, code_challenge = generate_pkce_challenge()

    # Prepare state data with all necessary information
    state_data = OAuthCallbackState(
        provider_id=provider_id,
        tenant_id=tenant_id,
        server_url=server_url,
        metadata=metadata,
        client_information=client_information,
        code_verifier=code_verifier,
        redirect_uri=redirect_url,
    )

    # Store state data in Redis and generate secure state key
    state_key = _create_secure_redis_state(state_data)

    params = {
        "response_type": response_type,
        "client_id": client_information.client_id,
        "code_challenge": code_challenge,
        "code_challenge_method": code_challenge_method,
        "redirect_uri": redirect_url,
        "state": state_key,
    }

    # Add scope if provided
    if scope:
        params["scope"] = scope

    authorization_url = f"{authorization_url}?{urllib.parse.urlencode(params)}"
    return authorization_url, code_verifier


def _parse_token_response(response: httpx.Response) -> OAuthTokens:
    """
    Parse OAuth token response supporting both JSON and form-urlencoded formats.

    Per RFC 6749 Section 5.1, the standard format is JSON.
    However, some legacy OAuth providers (e.g., early GitHub OAuth Apps) return
    application/x-www-form-urlencoded format for backwards compatibility.

    Args:
        response: The HTTP response from token endpoint

    Returns:
        Parsed OAuth tokens

    Raises:
        ValueError: If response cannot be parsed
    """
    content_type = response.headers.get("content-type", "").lower()

    if "application/json" in content_type:
        # Standard OAuth 2.0 JSON response (RFC 6749)
        return OAuthTokens.model_validate(response.json())
    elif "application/x-www-form-urlencoded" in content_type:
        # Legacy form-urlencoded response (non-standard but used by some providers)
        token_data = dict(urllib.parse.parse_qsl(response.text))
        return OAuthTokens.model_validate(token_data)
    else:
        # No content-type or unknown - try JSON first, fallback to form-urlencoded
        try:
            return OAuthTokens.model_validate(response.json())
        except (ValidationError, json.JSONDecodeError):
            token_data = dict(urllib.parse.parse_qsl(response.text))
            return OAuthTokens.model_validate(token_data)


def exchange_authorization(
    server_url: str,
    metadata: OAuthMetadata | None,
    client_information: OAuthClientInformation,
    authorization_code: str,
    code_verifier: str,
    redirect_uri: str,
) -> OAuthTokens:
    """Exchanges an authorization code for an access token."""
    grant_type = MCPSupportGrantType.AUTHORIZATION_CODE.value

    if metadata:
        token_url = metadata.token_endpoint
        if metadata.grant_types_supported and grant_type not in metadata.grant_types_supported:
            raise ValueError(f"Incompatible auth server: does not support grant type {grant_type}")
    else:
        token_url = urljoin(server_url, "/token")

    params = {
        "grant_type": grant_type,
        "client_id": client_information.client_id,
        "code": authorization_code,
        "code_verifier": code_verifier,
        "redirect_uri": redirect_uri,
    }

    if client_information.client_secret:
        params["client_secret"] = client_information.client_secret

    response = ssrf_proxy.post(token_url, data=params)
    if not response.is_success:
        raise ValueError(f"Token exchange failed: HTTP {response.status_code}")
    return _parse_token_response(response)


def refresh_authorization(
    server_url: str,
    metadata: OAuthMetadata | None,
    client_information: OAuthClientInformation,
    refresh_token: str,
) -> OAuthTokens:
    """Exchange a refresh token for an updated access token."""
    grant_type = MCPSupportGrantType.REFRESH_TOKEN.value

    if metadata:
        token_url = metadata.token_endpoint
        if metadata.grant_types_supported and grant_type not in metadata.grant_types_supported:
            raise ValueError(f"Incompatible auth server: does not support grant type {grant_type}")
    else:
        token_url = urljoin(server_url, "/token")

    params = {
        "grant_type": grant_type,
        "client_id": client_information.client_id,
        "refresh_token": refresh_token,
    }

    if client_information.client_secret:
        params["client_secret"] = client_information.client_secret
    try:
        response = ssrf_proxy.post(token_url, data=params)
    except ssrf_proxy.MaxRetriesExceededError as e:
        raise MCPRefreshTokenError(e) from e
    if not response.is_success:
        raise MCPRefreshTokenError(response.text)
    return _parse_token_response(response)


def client_credentials_flow(
    server_url: str,
    metadata: OAuthMetadata | None,
    client_information: OAuthClientInformation,
    scope: str | None = None,
) -> OAuthTokens:
    """Execute Client Credentials Flow to get access token."""
    grant_type = MCPSupportGrantType.CLIENT_CREDENTIALS.value

    if metadata:
        token_url = metadata.token_endpoint
        if metadata.grant_types_supported and grant_type not in metadata.grant_types_supported:
            raise ValueError(f"Incompatible auth server: does not support grant type {grant_type}")
    else:
        token_url = urljoin(server_url, "/token")

    # Support both Basic Auth and body parameters for client authentication
    headers = {"Content-Type": "application/x-www-form-urlencoded"}
    data = {"grant_type": grant_type}

    if scope:
        data["scope"] = scope

    # If client_secret is provided, use Basic Auth (preferred method)
    if client_information.client_secret:
        credentials = f"{client_information.client_id}:{client_information.client_secret}"
        encoded_credentials = base64.b64encode(credentials.encode()).decode()
        headers["Authorization"] = f"Basic {encoded_credentials}"
    else:
        # Fall back to including credentials in the body
        data["client_id"] = client_information.client_id
        if client_information.client_secret:
            data["client_secret"] = client_information.client_secret

    response = ssrf_proxy.post(token_url, headers=headers, data=data)
    if not response.is_success:
        raise ValueError(
            f"Client credentials token request failed: HTTP {response.status_code}, Response: {response.text}"
        )

    return _parse_token_response(response)


def register_client(
    server_url: str,
    metadata: OAuthMetadata | None,
    client_metadata: OAuthClientMetadata,
) -> OAuthClientInformationFull:
    """Performs OAuth 2.0 Dynamic Client Registration."""
    if metadata:
        if not metadata.registration_endpoint:
            raise ValueError("Incompatible auth server: does not support dynamic client registration")
        registration_url = metadata.registration_endpoint
    else:
        registration_url = urljoin(server_url, "/register")

    response = ssrf_proxy.post(
        registration_url,
        json=client_metadata.model_dump(),
        headers={"Content-Type": "application/json"},
    )
    if not response.is_success:
        response.raise_for_status()
    return OAuthClientInformationFull.model_validate(response.json())


def auth(
    provider: MCPProviderEntity,
    authorization_code: str | None = None,
    state_param: str | None = None,
    resource_metadata_url: str | None = None,
    scope_hint: str | None = None,
) -> AuthResult:
    """
    Orchestrates the full auth flow with a server using secure Redis state storage.

    This function performs only network operations and returns actions that need
    to be performed by the caller (such as saving data to database).

    Args:
        provider: The MCP provider entity
        authorization_code: Optional authorization code from OAuth callback
        state_param: Optional state parameter from OAuth callback
        resource_metadata_url: Optional Protected Resource Metadata URL from WWW-Authenticate
        scope_hint: Optional scope hint from WWW-Authenticate header

    Returns:
        AuthResult containing actions to be performed and response data
    """
    actions: list[AuthAction] = []
    server_url = provider.decrypt_server_url()

    # Discover OAuth metadata using RFC 8414/9470 standards
    server_metadata, prm, scope_from_www_auth = discover_oauth_metadata(
        server_url, resource_metadata_url, scope_hint, LATEST_PROTOCOL_VERSION
    )

    client_metadata = provider.client_metadata
    provider_id = provider.id
    tenant_id = provider.tenant_id
    client_information = provider.retrieve_client_information()
    redirect_url = provider.redirect_url
    credentials = provider.decrypt_credentials()

    # Determine grant type based on server metadata
    if not server_metadata:
        raise ValueError("Failed to discover OAuth metadata from server")

    supported_grant_types = server_metadata.grant_types_supported or []

    # Convert to lowercase for comparison
    supported_grant_types_lower = [gt.lower() for gt in supported_grant_types]

    # Determine which grant type to use
    effective_grant_type = None
    if MCPSupportGrantType.AUTHORIZATION_CODE.value in supported_grant_types_lower:
        effective_grant_type = MCPSupportGrantType.AUTHORIZATION_CODE.value
    else:
        effective_grant_type = MCPSupportGrantType.CLIENT_CREDENTIALS.value

    # Determine effective scope using priority-based strategy
    effective_scope = get_effective_scope(scope_from_www_auth, prm, server_metadata, credentials.get("scope"))

    if not client_information:
        if authorization_code is not None:
            raise ValueError("Existing OAuth client information is required when exchanging an authorization code")

        # For client credentials flow, we don't need to register client dynamically
        if effective_grant_type == MCPSupportGrantType.CLIENT_CREDENTIALS.value:
            # Client should provide client_id and client_secret directly
            raise ValueError("Client credentials flow requires client_id and client_secret to be provided")

        try:
            full_information = register_client(server_url, server_metadata, client_metadata)
        except RequestError as e:
            raise ValueError(f"Could not register OAuth client: {e}")

        # Return action to save client information
        actions.append(
            AuthAction(
                action_type=AuthActionType.SAVE_CLIENT_INFO,
                data={"client_information": full_information.model_dump()},
                provider_id=provider_id,
                tenant_id=tenant_id,
            )
        )

        client_information = full_information

    # Handle client credentials flow
    if effective_grant_type == MCPSupportGrantType.CLIENT_CREDENTIALS.value:
        # Direct token request without user interaction
        try:
            tokens = client_credentials_flow(
                server_url,
                server_metadata,
                client_information,
                effective_scope,
            )

            # Return action to save tokens and grant type
            token_data = tokens.model_dump()
            token_data["grant_type"] = MCPSupportGrantType.CLIENT_CREDENTIALS.value

            actions.append(
                AuthAction(
                    action_type=AuthActionType.SAVE_TOKENS,
                    data=token_data,
                    provider_id=provider_id,
                    tenant_id=tenant_id,
                )
            )

            return AuthResult(actions=actions, response={"result": "success"})
        except (RequestError, ValueError, KeyError) as e:
            # RequestError: HTTP request failed
            # ValueError: Invalid response data
            # KeyError: Missing required fields in response
            raise ValueError(f"Client credentials flow failed: {e}")

    # Exchange authorization code for tokens (Authorization Code flow)
    if authorization_code is not None:
        if not state_param:
            raise ValueError("State parameter is required when exchanging authorization code")

        try:
            # Retrieve state data from Redis using state key
            full_state_data = _retrieve_redis_state(state_param)

            code_verifier = full_state_data.code_verifier
            redirect_uri = full_state_data.redirect_uri

            if not code_verifier or not redirect_uri:
                raise ValueError("Missing code_verifier or redirect_uri in state data")

        except (json.JSONDecodeError, ValueError) as e:
            raise ValueError(f"Invalid state parameter: {e}")

        tokens = exchange_authorization(
            server_url,
            server_metadata,
            client_information,
            authorization_code,
            code_verifier,
            redirect_uri,
        )

        # Return action to save tokens
        actions.append(
            AuthAction(
                action_type=AuthActionType.SAVE_TOKENS,
                data=tokens.model_dump(),
                provider_id=provider_id,
                tenant_id=tenant_id,
            )
        )

        return AuthResult(actions=actions, response={"result": "success"})

    provider_tokens = provider.retrieve_tokens()

    # Handle token refresh or new authorization
    if provider_tokens and provider_tokens.refresh_token:
        try:
            new_tokens = refresh_authorization(
                server_url, server_metadata, client_information, provider_tokens.refresh_token
            )

            # Return action to save new tokens
            actions.append(
                AuthAction(
                    action_type=AuthActionType.SAVE_TOKENS,
                    data=new_tokens.model_dump(),
                    provider_id=provider_id,
                    tenant_id=tenant_id,
                )
            )

            return AuthResult(actions=actions, response={"result": "success"})
        except (RequestError, ValueError, KeyError) as e:
            # RequestError: HTTP request failed
            # ValueError: Invalid response data
            # KeyError: Missing required fields in response
            raise ValueError(f"Could not refresh OAuth tokens: {e}")

    # Start new authorization flow (only for authorization code flow)
    authorization_url, code_verifier = start_authorization(
        server_url,
        server_metadata,
        client_information,
        redirect_url,
        provider_id,
        tenant_id,
        effective_scope,
    )

    # Return action to save code verifier
    actions.append(
        AuthAction(
            action_type=AuthActionType.SAVE_CODE_VERIFIER,
            data={"code_verifier": code_verifier},
            provider_id=provider_id,
            tenant_id=tenant_id,
        )
    )

    return AuthResult(actions=actions, response={"authorization_url": authorization_url})

```

### api/core/mcp/session/base_session.py
```py
import logging
import queue
from collections.abc import Callable
from concurrent.futures import Future, ThreadPoolExecutor, TimeoutError
from datetime import timedelta
from types import TracebackType
from typing import Any, Self

from httpx import HTTPStatusError
from pydantic import BaseModel

from core.mcp.error import MCPAuthError, MCPConnectionError
from core.mcp.types import (
    CancelledNotification,
    ClientNotification,
    ClientRequest,
    ClientResult,
    ErrorData,
    JSONRPCError,
    JSONRPCMessage,
    JSONRPCNotification,
    JSONRPCRequest,
    JSONRPCResponse,
    MessageMetadata,
    RequestId,
    RequestParams,
    ServerMessageMetadata,
    ServerNotification,
    ServerRequest,
    ServerResult,
    SessionMessage,
)

logger = logging.getLogger(__name__)


DEFAULT_RESPONSE_READ_TIMEOUT = 1.0


class RequestResponder[ReceiveRequestT: ClientRequest | ServerRequest, SendResultT: ClientResult | ServerResult]:
    """Handles responding to MCP requests and manages request lifecycle.

    This class MUST be used as a context manager to ensure proper cleanup and
    cancellation handling:

    Example:
        with request_responder as resp:
            resp.respond(result)

    The context manager ensures:
    1. Proper cancellation scope setup and cleanup
    2. Request completion tracking
    3. Cleanup of in-flight requests
    """

    request: ReceiveRequestT
    _session: "BaseSession[Any, Any, SendResultT, ReceiveRequestT, Any]"
    _on_complete: Callable[["RequestResponder[ReceiveRequestT, SendResultT]"], Any]

    def __init__(
        self,
        request_id: RequestId,
        request_meta: RequestParams.Meta | None,
        request: ReceiveRequestT,
        session: "BaseSession[Any, Any, SendResultT, ReceiveRequestT, Any]",
        on_complete: Callable[["RequestResponder[ReceiveRequestT, SendResultT]"], Any],
    ):
        self.request_id = request_id
        self.request_meta = request_meta
        self.request = request
        self._session = session
        self.completed = False
        self._on_complete = on_complete
        self._entered = False  # Track if we're in a context manager

    def __enter__(self) -> "RequestResponder[ReceiveRequestT, SendResultT]":
        """Enter the context manager, enabling request cancellation tracking."""
        self._entered = True
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ):
        """Exit the context manager, performing cleanup and notifying completion."""
        try:
            if self.completed:
                self._on_complete(self)
        finally:
            self._entered = False

    def respond(self, response: SendResultT | ErrorData):
        """Send a response for this request.

        Must be called within a context manager block.
        Raises:
            RuntimeError: If not used within a context manager
            AssertionError: If request was already responded to
        """
        if not self._entered:
            raise RuntimeError("RequestResponder must be used as a context manager")
        assert not self.completed, "Request already responded to"

        self.completed = True

        self._session.send_response(request_id=self.request_id, response=response)

    def cancel(self):
        """Cancel this request and mark it as completed."""
        if not self._entered:
            raise RuntimeError("RequestResponder must be used as a context manager")

        self.completed = True  # Mark as completed so it's removed from in_flight
        # Send an error response to indicate cancellation
        self._session.send_response(
            request_id=self.request_id,
            response=ErrorData(code=0, message="Request cancelled", data=None),
        )


class BaseSession[
    SendRequestT: ClientRequest | ServerRequest,
    SendNotificationT: ClientNotification | ServerNotification,
    SendResultT: ClientResult | ServerResult,
    ReceiveRequestT: ClientRequest | ServerRequest,
    ReceiveNotificationT: ClientNotification | ServerNotification,
]:
    """
    Implements an MCP "session" on top of read/write streams, including features
    like request/response linking, notifications, and progress.

    This class is a context manager that automatically starts processing
    messages when entered.
    """

    _response_streams: dict[RequestId, queue.Queue[JSONRPCResponse | JSONRPCError | HTTPStatusError]]
    _request_id: int
    _in_flight: dict[RequestId, RequestResponder[ReceiveRequestT, SendResultT]]
    _receive_request_type: type[ReceiveRequestT]
    _receive_notification_type: type[ReceiveNotificationT]

    def __init__(
        self,
        read_stream: queue.Queue,
        write_stream: queue.Queue,
        receive_request_type: type[ReceiveRequestT],
        receive_notification_type: type[ReceiveNotificationT],
        # If none, reading will never time out
        read_timeout_seconds: timedelta | None = None,
    ):
        self._read_stream = read_stream
        self._write_stream = write_stream
        self._response_streams = {}
        self._request_id = 0
        self._receive_request_type = receive_request_type
        self._receive_notification_type = receive_notification_type
        self._session_read_timeout_seconds = read_timeout_seconds
        self._in_flight = {}
        # Initialize executor and future to None for proper cleanup checks
        self._executor: ThreadPoolExecutor | None = None
        self._receiver_future: Future | None = None

    def __enter__(self) -> Self:
        # The thread pool is dedicated to running `_receive_loop`. Setting `max_workers` to 1
        # ensures no unnecessary threads are created.
        self._executor = ThreadPoolExecutor(max_workers=1)
        self._receiver_future = self._executor.submit(self._receive_loop)
        return self

    def check_receiver_status(self):
        """`check_receiver_status` ensures that any exceptions raised during the
        execution of `_receive_loop` are retrieved and propagated."""
        if self._receiver_future and self._receiver_future.done():
            self._receiver_future.result()

    def __exit__(
        self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None
    ):
        self._read_stream.put(None)
        self._write_stream.put(None)

        # Wait for the receiver loop to finish
        if self._receiver_future:
            try:
                self._receiver_future.result(timeout=5.0)  # Wait up to 5 seconds
            except TimeoutError:
                # If the receiver loop is still running after timeout, we'll force shutdown
                # Cancel the future to interrupt the receiver loop
                self._receiver_future.cancel()

        # Shutdown the executor
        if self._executor:
            # Use non-blocking shutdown to prevent hanging
            # The receiver thread should have already exited due to the None message in the queue
            self._executor.shutdown(wait=False)

    def send_request[T: BaseModel](
        self,
        request: SendRequestT,
        result_type: type[T],
        request_read_timeout_seconds: timedelta | None = None,
        metadata: MessageMetadata | None = None,
    ) -> T:
        """
        Sends a request and wait for a response. Raises an McpError if the
        response contains an error. If a request read timeout is provided, it
        will take precedence over the session read timeout.

        Do not use this method to emit notifications! Use send_notification()
        instead.
        """
        self.check_receiver_status()

        request_id = self._request_id
        self._request_id = request_id + 1

        response_queue: queue.Queue[JSONRPCResponse | JSONRPCError | HTTPStatusError] = queue.Queue()
        self._response_streams[request_id] = response_queue

        try:
            jsonrpc_request = JSONRPCRequest(
                jsonrpc="2.0",
                id=request_id,
                **request.model_dump(by_alias=True, mode="json", exclude_none=True),
            )

            self._write_stream.put(SessionMessage(message=JSONRPCMessage(jsonrpc_request), metadata=metadata))
            timeout = DEFAULT_RESPONSE_READ_TIMEOUT
            if request_read_timeout_seconds is not None:
                timeout = float(request_read_timeout_seconds.total_seconds())
            elif self._session_read_timeout_seconds is not None:
                timeout = float(self._session_read_timeout_seconds.total_seconds())
            while True:
                try:
                    response_or_error = response_queue.get(timeout=timeout)
                    break
                except queue.Empty:
                    self.check_receiver_status()
                    continue

            if response_or_error is None:
                raise MCPConnectionError(
                    ErrorData(
                        code=500,
                        message="No response received",
                    )
                )
            elif isinstance(response_or_error, HTTPStatusError):
                # HTTPStatusError from streamable_client with preserved response object
                if response_or_error.response.status_code == 401:
                    raise MCPAuthError(response=response_or_error.response)
                else:
                    raise MCPConnectionError(
                        ErrorData(code=response_or_error.response.status_code, message=str(response_or_error))
                    )
            elif isinstance(response_or_error, JSONRPCError):
                if response_or_error.error.code == 401:
                    raise MCPAuthError(message=response_or_error.error.message)
                else:
                    raise MCPConnectionError(
                        ErrorData(code=response_or_error.error.code, message=response_or_error.error.message)
                    )
            else:
                return result_type.model_validate(response_or_error.result)

        finally:
            self._response_streams.pop(request_id, None)

    def send_notification(
        self,
        notification: SendNotificationT,
        related_request_id: RequestId | None = None,
    ):
        """
        Emits a notification, which is a one-way message that does not expect
        a response.
        """
        self.check_receiver_status()

        # Some transport implementations may need to set the related_request_id
        # to attribute to the notifications to the request that triggered them.
        jsonrpc_notification = JSONRPCNotification(
            jsonrpc="2.0",
            **notification.model_dump(by_alias=True, mode="json", exclude_none=True),
        )
        session_message = SessionMessage(
            message=JSONRPCMessage(jsonrpc_notification),
            metadata=ServerMessageMetadata(related_request_id=related_request_id) if related_request_id else None,
        )
        self._write_stream.put(session_message)

    def send_response(self, request_id: RequestId, response: SendResultT | ErrorData):
        if isinstance(response, ErrorData):
            jsonrpc_error = JSONRPCError(jsonrpc="2.0", id=request_id, error=response)
            session_message = SessionMessage(message=JSONRPCMessage(jsonrpc_error))
            self._write_stream.put(session_message)
        else:
            jsonrpc_response = JSONRPCResponse(
                jsonrpc="2.0",
                id=request_id,
                result=response.model_dump(by_alias=True, mode="json", exclude_none=True),
            )
            session_message = SessionMessage(message=JSONRPCMessage(jsonrpc_response))
            self._write_stream.put(session_message)

    def _receive_loop(self):
        """
        Main message processing loop.
        In a real synchronous implementation, this would likely run in a separate thread.
        """
        while True:
            try:
                # Attempt to receive a message (this would be blocking in a synchronous context)
                message = self._read_stream.get(timeout=DEFAULT_RESPONSE_READ_TIMEOUT)
                if message is None:
                    break
                if isinstance(message, HTTPStatusError):
                    response_queue = self._response_streams.get(self._request_id - 1)
                    if response_queue is not None:
                        # For 401 errors, pass the HTTPStatusError directly to preserve response object
                        if message.response.status_code == 401:
                            response_queue.put(message)
                        else:
                            response_queue.put(
                                JSONRPCError(
                                    jsonrpc="2.0",
                                    id=self._request_id - 1,
                                    error=ErrorData(code=message.response.status_code, message=message.args[0]),
                                )
                            )
                    else:
                        self._handle_incoming(RuntimeError(f"Received response with an unknown request ID: {message}"))
                elif isinstance(message, Exception):
                    self._handle_incoming(message)
                elif isinstance(message.message.root, JSONRPCRequest):
                    validated_request = self._receive_request_type.model_validate(
                        message.message.root.model_dump(by_alias=True, mode="json", exclude_none=True)
                    )

                    responder = RequestResponder[ReceiveRequestT, SendResultT](
                        request_id=message.message.root.id,
                        request_meta=validated_request.root.params.meta if validated_request.root.params else None,
                        request=validated_request,  # type: ignore[arg-type]  # mypy can't narrow constrained TypeVar from model_validate
                        session=self,
                        on_complete=lambda r: self._in_flight.pop(r.request_id, None),
                    )

                    self._in_flight[responder.request_id] = responder
                    self._received_request(responder)

                    if not responder.completed:
                        self._handle_incoming(responder)

                elif isinstance(message.message.root, JSONRPCNotification):
                    try:
                        notification = self._receive_notification_type.model_validate(
                            message.message.root.model_dump(by_alias=True, mode="json", exclude_none=True)
                        )
                        # Handle cancellation notifications
                        if isinstance(notification.root, CancelledNotification):
                            cancelled_id = notification.root.params.requestId
                            if cancelled_id in self._in_flight:
                                self._in_flight[cancelled_id].cancel()
                        else:
                            self._received_notification(notification)  # type: ignore[arg-type]
                            self._handle_incoming(notification)  # type: ignore[arg-type]
                    except Exception as e:
                        # For other validation errors, log and continue
                        logger.warning("Failed to validate notification: %s. Message was: %s", e, message.message.root)
                else:  # Response or error
                    response_queue = self._response_streams.get(message.message.root.id)
                    if response_queue is not None:
                        response_queue.put(message.message.root)
                    else:
                        self._handle_incoming(RuntimeError(f"Server Error: {message}"))
            except queue.Empty:
                continue
            except Exception:
                logger.exception("Error in message processing loop")
                raise

    def _received_request(self, responder: RequestResponder[ReceiveRequestT, SendResultT]):
        """
        Can be overridden by subclasses to handle a request without needing to
        listen on the message stream.

        If the request is responded to within this method, it will not be
        forwarded on to the message stream.
        """

    def _received_notification(self, notification: ReceiveNotificationT):
        """
        Can be overridden by subclasses to handle a notification without needing
        to listen on the message stream.
        """

    def send_progress_notification(self, progress_token: str | int, progress: float, total: float | None = None):
        """
        Sends a progress notification for a request that is currently being
        processed.
        """

    def _handle_incoming(
        self,
        req: RequestResponder[ReceiveRequestT, SendResultT] | ReceiveNotificationT | Exception,
    ):
        """A generic handler for incoming messages. Overwritten by subclasses."""

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-009.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
