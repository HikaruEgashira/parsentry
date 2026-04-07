You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-008
- **Kind**: public_api
- **Identifier**: Plugin System (install, execute, backwards invocation)
- **Description**: Plugin system allowing third-party code installation and execution with backwards invocation into host app. Plugin supply chain attacks, privilege escalation via backwards invocation, and HTTP parsing vulnerabilities
- **Locations**: api/core/plugin/impl/plugin.py, api/core/plugin/impl/tool.py, api/core/plugin/impl/endpoint.py, api/core/plugin/backwards_invocation/app.py, api/core/plugin/backwards_invocation/tool.py, api/core/plugin/backwards_invocation/encrypt.py, api/core/plugin/utils/http_parser.py

## Source Code

### api/core/plugin/impl/plugin.py
```py
from collections.abc import Sequence

from requests import HTTPError

from core.plugin.entities.bundle import PluginBundleDependency
from core.plugin.entities.plugin import (
    MissingPluginDependency,
    PluginDeclaration,
    PluginEntity,
    PluginInstallation,
    PluginInstallationSource,
)
from core.plugin.entities.plugin_daemon import (
    PluginDecodeResponse,
    PluginInstallTask,
    PluginInstallTaskStartResponse,
    PluginListResponse,
    PluginReadmeResponse,
)
from core.plugin.impl.base import BasePluginClient
from models.provider_ids import GenericProviderID


class PluginInstaller(BasePluginClient):
    def fetch_plugin_readme(self, tenant_id: str, plugin_unique_identifier: str, language: str) -> str:
        """
        Fetch plugin readme
        """
        try:
            response = self._request_with_plugin_daemon_response(
                "GET",
                f"plugin/{tenant_id}/management/fetch/readme",
                PluginReadmeResponse,
                params={
                    "tenant_id": tenant_id,
                    "plugin_unique_identifier": plugin_unique_identifier,
                    "language": language,
                },
            )
            return response.content
        except HTTPError as e:
            message = e.args[0]
            if "404" in message:
                return ""
            raise e

    def fetch_plugin_by_identifier(
        self,
        tenant_id: str,
        identifier: str,
    ) -> bool:
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/fetch/identifier",
            bool,
            params={"plugin_unique_identifier": identifier},
        )

    def list_plugins(self, tenant_id: str) -> list[PluginEntity]:
        result = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/list",
            PluginListResponse,
            params={"page": 1, "page_size": 256, "response_type": "paged"},
        )
        return result.list

    def list_plugins_with_total(self, tenant_id: str, page: int, page_size: int) -> PluginListResponse:
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/list",
            PluginListResponse,
            params={"page": page, "page_size": page_size, "response_type": "paged"},
        )

    def upload_pkg(
        self,
        tenant_id: str,
        pkg: bytes,
        verify_signature: bool = False,
    ) -> PluginDecodeResponse:
        """
        Upload a plugin package and return the plugin unique identifier.
        """
        body = {
            "dify_pkg": ("dify_pkg", pkg, "application/octet-stream"),
        }

        data = {
            "verify_signature": "true" if verify_signature else "false",
        }

        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upload/package",
            PluginDecodeResponse,
            files=body,
            data=data,
        )

    def upload_bundle(
        self,
        tenant_id: str,
        bundle: bytes,
        verify_signature: bool = False,
    ) -> Sequence[PluginBundleDependency]:
        """
        Upload a plugin bundle and return the dependencies.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upload/bundle",
            list[PluginBundleDependency],
            files={"dify_bundle": ("dify_bundle", bundle, "application/octet-stream")},
            data={"verify_signature": "true" if verify_signature else "false"},
        )

    def install_from_identifiers(
        self,
        tenant_id: str,
        identifiers: Sequence[str],
        source: PluginInstallationSource,
        metas: list[dict],
    ) -> PluginInstallTaskStartResponse:
        """
        Install a plugin from an identifier.
        """
        # exception will be raised if the request failed
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/identifiers",
            PluginInstallTaskStartResponse,
            data={
                "plugin_unique_identifiers": identifiers,
                "source": source,
                "metas": metas,
            },
            headers={"Content-Type": "application/json"},
        )

    def fetch_plugin_installation_tasks(self, tenant_id: str, page: int, page_size: int) -> Sequence[PluginInstallTask]:
        """
        Fetch plugin installation tasks.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/install/tasks",
            list[PluginInstallTask],
            params={"page": page, "page_size": page_size},
        )

    def fetch_plugin_installation_task(self, tenant_id: str, task_id: str) -> PluginInstallTask:
        """
        Fetch a plugin installation task.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}",
            PluginInstallTask,
        )

    def delete_plugin_installation_task(self, tenant_id: str, task_id: str) -> bool:
        """
        Delete a plugin installation task.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}/delete",
            bool,
        )

    def delete_all_plugin_installation_task_items(self, tenant_id: str) -> bool:
        """
        Delete all plugin installation task items.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/delete_all",
            bool,
        )

    def delete_plugin_installation_task_item(self, tenant_id: str, task_id: str, identifier: str) -> bool:
        """
        Delete a plugin installation task item.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}/delete/{identifier}",
            bool,
        )

    def fetch_plugin_manifest(self, tenant_id: str, plugin_unique_identifier: str) -> PluginDeclaration:
        """
        Fetch a plugin manifest.
        """

        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/fetch/manifest",
            PluginDeclaration,
            params={"plugin_unique_identifier": plugin_unique_identifier},
        )

    def decode_plugin_from_identifier(self, tenant_id: str, plugin_unique_identifier: str) -> PluginDecodeResponse:
        """
        Decode a plugin from an identifier.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/decode/from_identifier",
            PluginDecodeResponse,
            params={"plugin_unique_identifier": plugin_unique_identifier},
        )

    def fetch_plugin_installation_by_ids(
        self, tenant_id: str, plugin_ids: Sequence[str]
    ) -> Sequence[PluginInstallation]:
        """
        Fetch plugin installations by ids.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/installation/fetch/batch",
            list[PluginInstallation],
            data={"plugin_ids": plugin_ids},
            headers={"Content-Type": "application/json"},
        )

    def fetch_missing_dependencies(
        self, tenant_id: str, plugin_unique_identifiers: list[str]
    ) -> list[MissingPluginDependency]:
        """
        Fetch missing dependencies
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/installation/missing",
            list[MissingPluginDependency],
            data={"plugin_unique_identifiers": plugin_unique_identifiers},
            headers={"Content-Type": "application/json"},
        )

    def uninstall(self, tenant_id: str, plugin_installation_id: str) -> bool:
        """
        Uninstall a plugin.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/uninstall",
            bool,
            data={
                "plugin_installation_id": plugin_installation_id,
            },
            headers={"Content-Type": "application/json"},
        )

    def upgrade_plugin(
        self,
        tenant_id: str,
        original_plugin_unique_identifier: str,
        new_plugin_unique_identifier: str,
        source: PluginInstallationSource,
        meta: dict,
    ) -> PluginInstallTaskStartResponse:
        """
        Upgrade a plugin.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upgrade",
            PluginInstallTaskStartResponse,
            data={
                "original_plugin_unique_identifier": original_plugin_unique_identifier,
                "new_plugin_unique_identifier": new_plugin_unique_identifier,
                "source": source,
                "meta": meta,
            },
            headers={"Content-Type": "application/json"},
        )

    def check_tools_existence(self, tenant_id: str, provider_ids: Sequence[GenericProviderID]) -> Sequence[bool]:
        """
        Check if the tools exist
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/tools/check_existence",
            list[bool],
            data={
                "provider_ids": [
                    {
                        "plugin_id": provider_id.plugin_id,
                        "provider_name": provider_id.provider_name,
                    }
                    for provider_id in provider_ids
                ]
            },
            headers={"Content-Type": "application/json"},
        )

```

### api/core/plugin/impl/tool.py
```py
from collections.abc import Generator
from typing import Any

from pydantic import BaseModel

from configs import dify_config

# from core.plugin.entities.plugin import GenericProviderID, ToolProviderID
from core.plugin.entities.plugin_daemon import CredentialType, PluginBasicBooleanResponse, PluginToolProviderEntity
from core.plugin.impl.base import BasePluginClient
from core.plugin.utils.chunk_merger import merge_blob_chunks
from core.schemas.resolver import resolve_dify_schema_refs
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolParameter
from models.provider_ids import GenericProviderID, ToolProviderID


class PluginToolManager(BasePluginClient):
    def fetch_tool_providers(self, tenant_id: str) -> list[PluginToolProviderEntity]:
        """
        Fetch tool providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]):
            for provider in json_response.get("data", []):
                declaration = provider.get("declaration", {}) or {}
                provider_name = declaration.get("identity", {}).get("name")
                for tool in declaration.get("tools", []):
                    tool["identity"]["provider"] = provider_name
                    # resolve refs
                    if tool.get("output_schema"):
                        tool["output_schema"] = resolve_dify_schema_refs(tool["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/tools",
            list[PluginToolProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )

        for provider in response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each tool to plugin_id/provider_name
            for tool in provider.declaration.tools:
                tool.identity.provider = provider.declaration.identity.name

        return response

    def fetch_tool_provider(self, tenant_id: str, provider: str) -> PluginToolProviderEntity:
        """
        Fetch tool provider for the given tenant and plugin.
        """
        tool_provider_id = ToolProviderID(provider)

        def transformer(json_response: dict[str, Any]):
            data = json_response.get("data")
            if data:
                for tool in data.get("declaration", {}).get("tools", []):
                    tool["identity"]["provider"] = tool_provider_id.provider_name
                    # resolve refs
                    if tool.get("output_schema"):
                        tool["output_schema"] = resolve_dify_schema_refs(tool["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/tool",
            PluginToolProviderEntity,
            params={"provider": tool_provider_id.provider_name, "plugin_id": tool_provider_id.plugin_id},
            transformer=transformer,
        )

        response.declaration.identity.name = f"{response.plugin_id}/{response.declaration.identity.name}"

        # override the provider name for each tool to plugin_id/provider_name
        for tool in response.declaration.tools:
            tool.identity.provider = response.declaration.identity.name

        return response

    def invoke(
        self,
        tenant_id: str,
        user_id: str,
        tool_provider: str,
        tool_name: str,
        credentials: dict[str, Any],
        credential_type: CredentialType,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Invoke the tool with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        tool_provider_id = GenericProviderID(tool_provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/invoke",
            ToolInvokeMessage,
            data={
                "user_id": user_id,
                "conversation_id": conversation_id,
                "app_id": app_id,
                "message_id": message_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "tool": tool_name,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "tool_parameters": tool_parameters,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        return merge_blob_chunks(response, max_file_size=dify_config.PLUGIN_MAX_FILE_SIZE)

    def validate_provider_credentials(
        self, tenant_id: str, user_id: str, provider: str, credentials: dict[str, Any]
    ) -> bool:
        """
        validate the credentials of the provider
        """
        tool_provider_id = GenericProviderID(provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/validate_credentials",
            PluginBasicBooleanResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        return False

    def validate_datasource_credentials(
        self, tenant_id: str, user_id: str, provider: str, credentials: dict[str, Any]
    ) -> bool:
        """
        validate the credentials of the datasource
        """
        tool_provider_id = GenericProviderID(provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/validate_credentials",
            PluginBasicBooleanResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        return False

    def get_runtime_parameters(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        credentials: dict[str, Any],
        tool: str,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> list[ToolParameter]:
        """
        get the runtime parameters of the tool
        """
        tool_provider_id = GenericProviderID(provider)

        class RuntimeParametersResponse(BaseModel):
            parameters: list[ToolParameter]

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/get_runtime_parameters",
            RuntimeParametersResponse,
            data={
                "user_id": user_id,
                "conversation_id": conversation_id,
                "app_id": app_id,
                "message_id": message_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "tool": tool,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.parameters

        return []

```

### api/core/plugin/impl/endpoint.py
```py
from core.plugin.entities.endpoint import EndpointEntityWithInstance
from core.plugin.impl.base import BasePluginClient
from core.plugin.impl.exc import PluginDaemonInternalServerError


class PluginEndpointClient(BasePluginClient):
    def create_endpoint(
        self, tenant_id: str, user_id: str, plugin_unique_identifier: str, name: str, settings: dict
    ) -> bool:
        """
        Create an endpoint for the given plugin.

        Errors will be raised if any error occurs.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/setup",
            bool,
            headers={
                "Content-Type": "application/json",
            },
            data={
                "user_id": user_id,
                "plugin_unique_identifier": plugin_unique_identifier,
                "settings": settings,
                "name": name,
            },
        )

    def list_endpoints(self, tenant_id: str, user_id: str, page: int, page_size: int):
        """
        List all endpoints for the given tenant and user.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/endpoint/list",
            list[EndpointEntityWithInstance],
            params={"page": page, "page_size": page_size},
        )

    def list_endpoints_for_single_plugin(self, tenant_id: str, user_id: str, plugin_id: str, page: int, page_size: int):
        """
        List all endpoints for the given tenant, user and plugin.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/endpoint/list/plugin",
            list[EndpointEntityWithInstance],
            params={"plugin_id": plugin_id, "page": page, "page_size": page_size},
        )

    def update_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str, name: str, settings: dict):
        """
        Update the settings of the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/update",
            bool,
            data={
                "user_id": user_id,
                "endpoint_id": endpoint_id,
                "name": name,
                "settings": settings,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

    def delete_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Delete the given endpoint.

        This operation is idempotent: if the endpoint is already deleted (record not found),
        it will return True instead of raising an error.
        """
        try:
            return self._request_with_plugin_daemon_response(
                "POST",
                f"plugin/{tenant_id}/endpoint/remove",
                bool,
                data={
                    "endpoint_id": endpoint_id,
                },
                headers={
                    "Content-Type": "application/json",
                },
            )
        except PluginDaemonInternalServerError as e:
            # Make delete idempotent: if record is not found, consider it a success
            if "record not found" in str(e.description).lower():
                return True
            raise

    def enable_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Enable the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/enable",
            bool,
            data={
                "endpoint_id": endpoint_id,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

    def disable_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Disable the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/disable",
            bool,
            data={
                "endpoint_id": endpoint_id,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

```

### api/core/plugin/backwards_invocation/app.py
```py
import uuid
from collections.abc import Generator, Mapping
from typing import Any, Union, cast

from sqlalchemy import select
from sqlalchemy.orm import Session

from core.app.app_config.common.parameters_mapping import get_parameters_from_feature_dict
from core.app.apps.advanced_chat.app_generator import AdvancedChatAppGenerator
from core.app.apps.agent_chat.app_generator import AgentChatAppGenerator
from core.app.apps.chat.app_generator import ChatAppGenerator
from core.app.apps.completion.app_generator import CompletionAppGenerator
from core.app.apps.workflow.app_generator import WorkflowAppGenerator
from core.app.entities.app_invoke_entities import InvokeFrom
from core.app.layers.pause_state_persist_layer import PauseStateLayerConfig
from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from extensions.ext_database import db
from models import Account
from models.model import App, AppMode, EndUser
from services.end_user_service import EndUserService


class PluginAppBackwardsInvocation(BaseBackwardsInvocation):
    @classmethod
    def fetch_app_info(cls, app_id: str, tenant_id: str) -> Mapping:
        """
        Fetch app info
        """
        app = cls._get_app(app_id, tenant_id)

        """Retrieve app parameters."""
        if app.mode in {AppMode.ADVANCED_CHAT, AppMode.WORKFLOW}:
            workflow = app.workflow
            if workflow is None:
                raise ValueError("unexpected app type")

            features_dict: dict[str, Any] = workflow.features_dict
            user_input_form = workflow.user_input_form(to_old_structure=True)
        else:
            app_model_config = app.app_model_config
            if app_model_config is None:
                raise ValueError("unexpected app type")

            features_dict = cast(dict[str, Any], app_model_config.to_dict())

            user_input_form = features_dict.get("user_input_form", [])

        return {
            "data": get_parameters_from_feature_dict(features_dict=features_dict, user_input_form=user_input_form),
        }

    @classmethod
    def invoke_app(
        cls,
        app_id: str,
        user_id: str,
        tenant_id: str,
        conversation_id: str | None,
        query: str | None,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke app
        """
        app = cls._get_app(app_id, tenant_id)
        if not user_id:
            user = EndUserService.get_or_create_end_user(app)
        else:
            user = cls._get_user(user_id)

        conversation_id = conversation_id or ""

        if app.mode in {AppMode.ADVANCED_CHAT, AppMode.AGENT_CHAT, AppMode.CHAT}:
            if not query:
                raise ValueError("missing query")

            return cls.invoke_chat_app(app, user, conversation_id, query, stream, inputs, files)
        elif app.mode == AppMode.WORKFLOW:
            return cls.invoke_workflow_app(app, user, stream, inputs, files)
        elif app.mode == AppMode.COMPLETION:
            return cls.invoke_completion_app(app, user, stream, inputs, files)

        raise ValueError("unexpected app type")

    @classmethod
    def invoke_chat_app(
        cls,
        app: App,
        user: Account | EndUser,
        conversation_id: str,
        query: str,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke chat app
        """
        if app.mode == AppMode.ADVANCED_CHAT:
            workflow = app.workflow
            if not workflow:
                raise ValueError("unexpected app type")

            pause_config = PauseStateLayerConfig(
                session_factory=db.engine,
                state_owner_user_id=workflow.created_by,
            )

            return AdvancedChatAppGenerator().generate(
                app_model=app,
                workflow=workflow,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                workflow_run_id=str(uuid.uuid4()),
                streaming=stream,
                pause_state_config=pause_config,
            )
        elif app.mode == AppMode.AGENT_CHAT:
            return AgentChatAppGenerator().generate(
                app_model=app,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=stream,
            )
        elif app.mode == AppMode.CHAT:
            return ChatAppGenerator().generate(
                app_model=app,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=stream,
            )
        else:
            raise ValueError("unexpected app type")

    @classmethod
    def invoke_workflow_app(
        cls,
        app: App,
        user: EndUser | Account,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke workflow app
        """
        workflow = app.workflow
        if not workflow:
            raise ValueError("unexpected app type")

        pause_config = PauseStateLayerConfig(
            session_factory=db.engine,
            state_owner_user_id=workflow.created_by,
        )

        return WorkflowAppGenerator().generate(
            app_model=app,
            workflow=workflow,
            user=user,
            args={"inputs": inputs, "files": files},
            invoke_from=InvokeFrom.SERVICE_API,
            streaming=stream,
            call_depth=1,
            pause_state_config=pause_config,
        )

    @classmethod
    def invoke_completion_app(
        cls,
        app: App,
        user: EndUser | Account,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke completion app
        """
        return CompletionAppGenerator().generate(
            app_model=app,
            user=user,
            args={"inputs": inputs, "files": files},
            invoke_from=InvokeFrom.SERVICE_API,
            streaming=stream,
        )

    @classmethod
    def _get_user(cls, user_id: str) -> Union[EndUser, Account]:
        """
        get the user by user id
        """
        with Session(db.engine, expire_on_commit=False) as session:
            stmt = select(EndUser).where(EndUser.id == user_id)
            user = session.scalar(stmt)
            if not user:
                stmt = select(Account).where(Account.id == user_id)
                user = session.scalar(stmt)

        if not user:
            raise ValueError("user not found")

        return user

    @classmethod
    def _get_app(cls, app_id: str, tenant_id: str) -> App:
        """
        get app
        """
        try:
            app = db.session.scalar(select(App).where(App.id == app_id, App.tenant_id == tenant_id).limit(1))
        except Exception:
            raise ValueError("app not found")

        if not app:
            raise ValueError("app not found")

        return app

```

### api/core/plugin/backwards_invocation/tool.py
```py
from collections.abc import Generator
from typing import Any

from core.callback_handler.workflow_tool_callback_handler import DifyWorkflowCallbackHandler
from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolProviderType
from core.tools.tool_engine import ToolEngine
from core.tools.tool_manager import ToolManager
from core.tools.utils.message_transformer import ToolFileMessageTransformer


class PluginToolBackwardsInvocation(BaseBackwardsInvocation):
    """
    Backwards invocation for plugin tools.
    """

    @classmethod
    def invoke_tool(
        cls,
        tenant_id: str,
        user_id: str,
        tool_type: ToolProviderType,
        provider: str,
        tool_name: str,
        tool_parameters: dict[str, Any],
        credential_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke tool
        """
        # get tool runtime
        try:
            tool_runtime = ToolManager.get_tool_runtime_from_plugin(
                tool_type,
                tenant_id,
                provider,
                tool_name,
                tool_parameters,
                user_id=user_id,
                credential_id=credential_id,
            )
            response = ToolEngine.generic_invoke(
                tool_runtime, tool_parameters, user_id, DifyWorkflowCallbackHandler(), workflow_call_depth=1
            )

            response = ToolFileMessageTransformer.transform_tool_invoke_messages(
                response, user_id=user_id, tenant_id=tenant_id
            )

            return response
        except Exception as e:
            raise e

```

### api/core/plugin/backwards_invocation/encrypt.py
```py
from core.helper.provider_cache import SingletonProviderCredentialsCache
from core.plugin.entities.request import RequestInvokeEncrypt
from core.tools.utils.encryption import create_provider_encrypter
from models.account import Tenant


class PluginEncrypter:
    @classmethod
    def invoke_encrypt(cls, tenant: Tenant, payload: RequestInvokeEncrypt):
        encrypter, cache = create_provider_encrypter(
            tenant_id=tenant.id,
            config=payload.config,
            cache=SingletonProviderCredentialsCache(
                tenant_id=tenant.id,
                provider_type=payload.namespace,
                provider_identity=payload.identity,
            ),
        )

        if payload.opt == "encrypt":
            return {
                "data": encrypter.encrypt(payload.data),
            }
        elif payload.opt == "decrypt":
            return {
                "data": encrypter.decrypt(payload.data),
            }
        elif payload.opt == "clear":
            cache.delete()
            return {
                "data": {},
            }
        else:
            raise ValueError(f"Invalid opt: {payload.opt}")

```

### api/core/plugin/utils/http_parser.py
```py
from io import BytesIO

from flask import Request, Response
from werkzeug.datastructures import Headers


def serialize_request(request: Request) -> bytes:
    method = request.method
    path = request.full_path.rstrip("?")
    raw = f"{method} {path} HTTP/1.1\r\n".encode()

    for name, value in request.headers.items():
        raw += f"{name}: {value}\r\n".encode()

    raw += b"\r\n"

    body = request.get_data(as_text=False)
    if body:
        raw += body

    return raw


def deserialize_request(raw_data: bytes) -> Request:
    header_end = raw_data.find(b"\r\n\r\n")
    if header_end == -1:
        header_end = raw_data.find(b"\n\n")
        if header_end == -1:
            header_data = raw_data
            body = b""
        else:
            header_data = raw_data[:header_end]
            body = raw_data[header_end + 2 :]
    else:
        header_data = raw_data[:header_end]
        body = raw_data[header_end + 4 :]

    lines = header_data.split(b"\r\n")
    if len(lines) == 1 and b"\n" in lines[0]:
        lines = header_data.split(b"\n")

    if not lines or not lines[0]:
        raise ValueError("Empty HTTP request")

    request_line = lines[0].decode("utf-8", errors="ignore")
    parts = request_line.split(" ", 2)
    if len(parts) < 2:
        raise ValueError(f"Invalid request line: {request_line}")

    method = parts[0]
    full_path = parts[1]
    protocol = parts[2] if len(parts) > 2 else "HTTP/1.1"

    if "?" in full_path:
        path, query_string = full_path.split("?", 1)
    else:
        path = full_path
        query_string = ""

    headers = Headers()
    for line in lines[1:]:
        if not line:
            continue
        line_str = line.decode("utf-8", errors="ignore")
        if ":" not in line_str:
            continue
        name, value = line_str.split(":", 1)
        headers.add(name, value.strip())

    host = headers.get("Host", "localhost")
    if ":" in host:
        server_name, server_port = host.rsplit(":", 1)
    else:
        server_name = host
        server_port = "80"

    environ = {
        "REQUEST_METHOD": method,
        "PATH_INFO": path,
        "QUERY_STRING": query_string,
        "SERVER_NAME": server_name,
        "SERVER_PORT": server_port,
        "SERVER_PROTOCOL": protocol,
        "wsgi.input": BytesIO(body),
        "wsgi.url_scheme": "http",
    }

    if "Content-Type" in headers:
        content_type = headers.get("Content-Type")
        if content_type is not None:
            environ["CONTENT_TYPE"] = content_type

    if "Content-Length" in headers:
        content_length = headers.get("Content-Length")
        if content_length is not None:
            environ["CONTENT_LENGTH"] = content_length
    elif body:
        environ["CONTENT_LENGTH"] = str(len(body))

    for name, value in headers.items():
        if name.upper() in ("CONTENT-TYPE", "CONTENT-LENGTH"):
            continue
        env_name = f"HTTP_{name.upper().replace('-', '_')}"
        environ[env_name] = value

    return Request(environ)


def serialize_response(response: Response) -> bytes:
    raw = f"HTTP/1.1 {response.status}\r\n".encode()

    for name, value in response.headers.items():
        raw += f"{name}: {value}\r\n".encode()

    raw += b"\r\n"

    body = response.get_data(as_text=False)
    if body:
        raw += body

    return raw


def deserialize_response(raw_data: bytes) -> Response:
    header_end = raw_data.find(b"\r\n\r\n")
    if header_end == -1:
        header_end = raw_data.find(b"\n\n")
        if header_end == -1:
            header_data = raw_data
            body = b""
        else:
            header_data = raw_data[:header_end]
            body = raw_data[header_end + 2 :]
    else:
        header_data = raw_data[:header_end]
        body = raw_data[header_end + 4 :]

    lines = header_data.split(b"\r\n")
    if len(lines) == 1 and b"\n" in lines[0]:
        lines = header_data.split(b"\n")

    if not lines or not lines[0]:
        raise ValueError("Empty HTTP response")

    status_line = lines[0].decode("utf-8", errors="ignore")
    parts = status_line.split(" ", 2)
    if len(parts) < 2:
        raise ValueError(f"Invalid status line: {status_line}")

    status_code = int(parts[1])

    response = Response(response=body, status=status_code)

    for line in lines[1:]:
        if not line:
            continue
        line_str = line.decode("utf-8", errors="ignore")
        if ":" not in line_str:
            continue
        name, value = line_str.split(":", 1)
        response.headers[name] = value.strip()

    return response

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-008.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
