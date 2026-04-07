You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-010
- **Kind**: public_api
- **Identifier**: Workflow Execution Engine (node execution, variable interpolation)
- **Description**: Workflow engine executing directed acyclic graphs of nodes with user-controlled variable interpolation. Variable injection, node execution order manipulation, and data leakage between workflow contexts
- **Locations**: api/core/workflow/nodes/agent/agent_node.py, api/core/workflow/nodes/knowledge_retrieval/knowledge_retrieval_node.py, api/core/workflow/nodes/datasource/datasource_node.py, api/core/workflow/nodes/trigger_webhook/node.py

## Source Code

### api/core/workflow/nodes/agent/agent_node.py
```py
from __future__ import annotations

from collections.abc import Generator, Mapping, Sequence
from typing import TYPE_CHECKING, Any

from graphon.entities.graph_config import NodeConfigDict
from graphon.enums import BuiltinNodeTypes, WorkflowNodeExecutionStatus
from graphon.node_events import NodeEventBase, NodeRunResult, StreamCompletedEvent
from graphon.nodes.base.node import Node
from graphon.nodes.base.variable_template_parser import VariableTemplateParser

from core.app.entities.app_invoke_entities import DIFY_RUN_CONTEXT_KEY, DifyRunContext
from core.workflow.system_variables import SystemVariableKey, get_system_text

from .entities import AgentNodeData
from .exceptions import (
    AgentInvocationError,
    AgentMessageTransformError,
)
from .message_transformer import AgentMessageTransformer
from .runtime_support import AgentRuntimeSupport
from .strategy_protocols import AgentStrategyPresentationProvider, AgentStrategyResolver

if TYPE_CHECKING:
    from graphon.entities import GraphInitParams
    from graphon.runtime import GraphRuntimeState


class AgentNode(Node[AgentNodeData]):
    node_type = BuiltinNodeTypes.AGENT

    _strategy_resolver: AgentStrategyResolver
    _presentation_provider: AgentStrategyPresentationProvider
    _runtime_support: AgentRuntimeSupport
    _message_transformer: AgentMessageTransformer

    def __init__(
        self,
        id: str,
        config: NodeConfigDict,
        graph_init_params: GraphInitParams,
        graph_runtime_state: GraphRuntimeState,
        *,
        strategy_resolver: AgentStrategyResolver,
        presentation_provider: AgentStrategyPresentationProvider,
        runtime_support: AgentRuntimeSupport,
        message_transformer: AgentMessageTransformer,
    ) -> None:
        super().__init__(
            id=id,
            config=config,
            graph_init_params=graph_init_params,
            graph_runtime_state=graph_runtime_state,
        )
        self._strategy_resolver = strategy_resolver
        self._presentation_provider = presentation_provider
        self._runtime_support = runtime_support
        self._message_transformer = message_transformer

    @classmethod
    def version(cls) -> str:
        return "1"

    def populate_start_event(self, event) -> None:
        dify_ctx = DifyRunContext.model_validate(self.require_run_context_value(DIFY_RUN_CONTEXT_KEY))
        event.extras["agent_strategy"] = {
            "name": self.node_data.agent_strategy_name,
            "icon": self._presentation_provider.get_icon(
                tenant_id=dify_ctx.tenant_id,
                agent_strategy_provider_name=self.node_data.agent_strategy_provider_name,
            ),
        }

    def _run(self) -> Generator[NodeEventBase, None, None]:
        from core.plugin.impl.exc import PluginDaemonClientSideError

        dify_ctx = DifyRunContext.model_validate(self.require_run_context_value(DIFY_RUN_CONTEXT_KEY))

        try:
            strategy = self._strategy_resolver.resolve(
                tenant_id=dify_ctx.tenant_id,
                agent_strategy_provider_name=self.node_data.agent_strategy_provider_name,
                agent_strategy_name=self.node_data.agent_strategy_name,
            )
        except Exception as e:
            yield StreamCompletedEvent(
                node_run_result=NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs={},
                    error=f"Failed to get agent strategy: {str(e)}",
                ),
            )
            return

        agent_parameters = strategy.get_parameters()

        parameters = self._runtime_support.build_parameters(
            agent_parameters=agent_parameters,
            variable_pool=self.graph_runtime_state.variable_pool,
            node_data=self.node_data,
            strategy=strategy,
            tenant_id=dify_ctx.tenant_id,
            user_id=dify_ctx.user_id,
            app_id=dify_ctx.app_id,
            invoke_from=dify_ctx.invoke_from,
        )
        parameters_for_log = self._runtime_support.build_parameters(
            agent_parameters=agent_parameters,
            variable_pool=self.graph_runtime_state.variable_pool,
            node_data=self.node_data,
            strategy=strategy,
            tenant_id=dify_ctx.tenant_id,
            user_id=dify_ctx.user_id,
            app_id=dify_ctx.app_id,
            invoke_from=dify_ctx.invoke_from,
            for_log=True,
        )
        credentials = self._runtime_support.build_credentials(parameters=parameters)

        conversation_id = get_system_text(self.graph_runtime_state.variable_pool, SystemVariableKey.CONVERSATION_ID)

        try:
            message_stream = strategy.invoke(
                params=parameters,
                user_id=dify_ctx.user_id,
                app_id=dify_ctx.app_id,
                conversation_id=conversation_id,
                credentials=credentials,
            )
        except Exception as e:
            error = AgentInvocationError(f"Failed to invoke agent: {str(e)}", original_error=e)
            yield StreamCompletedEvent(
                node_run_result=NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs=parameters_for_log,
                    error=str(error),
                )
            )
            return

        try:
            yield from self._message_transformer.transform(
                messages=message_stream,
                tool_info={
                    "icon": self._presentation_provider.get_icon(
                        tenant_id=dify_ctx.tenant_id,
                        agent_strategy_provider_name=self.node_data.agent_strategy_provider_name,
                    ),
                    "agent_strategy": self.node_data.agent_strategy_name,
                },
                parameters_for_log=parameters_for_log,
                user_id=dify_ctx.user_id,
                tenant_id=dify_ctx.tenant_id,
                conversation_id=conversation_id,
                node_type=self.node_type,
                node_id=self._node_id,
                node_execution_id=self.id,
            )
        except PluginDaemonClientSideError as e:
            transform_error = AgentMessageTransformError(
                f"Failed to transform agent message: {str(e)}", original_error=e
            )
            yield StreamCompletedEvent(
                node_run_result=NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs=parameters_for_log,
                    error=str(transform_error),
                )
            )

    @classmethod
    def _extract_variable_selector_to_variable_mapping(
        cls,
        *,
        graph_config: Mapping[str, Any],
        node_id: str,
        node_data: AgentNodeData,
    ) -> Mapping[str, Sequence[str]]:
        _ = graph_config  # Explicitly mark as unused
        result: dict[str, Any] = {}
        typed_node_data = node_data
        for parameter_name in typed_node_data.agent_parameters:
            input = typed_node_data.agent_parameters[parameter_name]
            match input.type:
                case "mixed" | "constant":
                    selectors = VariableTemplateParser(str(input.value)).extract_variable_selectors()
                    for selector in selectors:
                        result[selector.variable] = selector.value_selector
                case "variable":
                    result[parameter_name] = input.value

        result = {node_id + "." + key: value for key, value in result.items()}

        return result

```

### api/core/workflow/nodes/knowledge_retrieval/knowledge_retrieval_node.py
```py
"""Knowledge retrieval workflow node implementation.

This node now lives under ``core.workflow.nodes`` and is discovered directly by
the workflow node registry.
"""

import logging
from collections.abc import Mapping, Sequence
from typing import TYPE_CHECKING, Any, Literal

from graphon.entities import GraphInitParams
from graphon.entities.graph_config import NodeConfigDict
from graphon.enums import (
    BuiltinNodeTypes,
    WorkflowNodeExecutionMetadataKey,
    WorkflowNodeExecutionStatus,
)
from graphon.model_runtime.entities.llm_entities import LLMUsage
from graphon.model_runtime.utils.encoders import jsonable_encoder
from graphon.node_events import NodeRunResult
from graphon.nodes.base import LLMUsageTrackingMixin
from graphon.nodes.base.node import Node
from graphon.variables import (
    ArrayFileSegment,
    FileSegment,
    StringSegment,
)
from graphon.variables.segments import ArrayObjectSegment

from core.app.app_config.entities import DatasetRetrieveConfigEntity
from core.app.entities.app_invoke_entities import DIFY_RUN_CONTEXT_KEY, DifyRunContext
from core.rag.data_post_processor.data_post_processor import RerankingModelDict, WeightsDict
from core.rag.retrieval.dataset_retrieval import DatasetRetrieval
from core.workflow.file_reference import parse_file_reference

from .entities import (
    Condition,
    KnowledgeRetrievalNodeData,
    MetadataFilteringCondition,
)
from .exc import (
    KnowledgeRetrievalNodeError,
    RateLimitExceededError,
)
from .retrieval import KnowledgeRetrievalRequest, Source

if TYPE_CHECKING:
    from graphon.file import File
    from graphon.runtime import GraphRuntimeState

logger = logging.getLogger(__name__)


class KnowledgeRetrievalNode(LLMUsageTrackingMixin, Node[KnowledgeRetrievalNodeData]):
    node_type = BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL

    # Instance attributes specific to LLMNode.
    # Output variable for file
    _file_outputs: list["File"]

    def __init__(
        self,
        id: str,
        config: NodeConfigDict,
        graph_init_params: "GraphInitParams",
        graph_runtime_state: "GraphRuntimeState",
    ):
        super().__init__(
            id=id,
            config=config,
            graph_init_params=graph_init_params,
            graph_runtime_state=graph_runtime_state,
        )
        # LLM file outputs, used for MultiModal outputs.
        self._file_outputs = []
        self._rag_retrieval = DatasetRetrieval()

    @classmethod
    def version(cls):
        return "1"

    def _run(self) -> NodeRunResult:
        usage = LLMUsage.empty_usage()
        if not self._node_data.query_variable_selector and not self._node_data.query_attachment_selector:
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.SUCCEEDED,
                inputs={},
                process_data={},
                outputs={},
                metadata={},
                llm_usage=usage,
            )
        variables: dict[str, Any] = {}
        # extract variables
        if self._node_data.query_variable_selector:
            variable = self.graph_runtime_state.variable_pool.get(self._node_data.query_variable_selector)
            if not isinstance(variable, StringSegment):
                return NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs={},
                    error="Query variable is not string type.",
                )
            query = variable.value
            variables["query"] = query

        if self._node_data.query_attachment_selector:
            variable = self.graph_runtime_state.variable_pool.get(self._node_data.query_attachment_selector)
            if not isinstance(variable, ArrayFileSegment) and not isinstance(variable, FileSegment):
                return NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs={},
                    error="Attachments variable is not array file or file type.",
                )
            if isinstance(variable, ArrayFileSegment):
                variables["attachments"] = variable.value
            else:
                variables["attachments"] = [variable.value]

        try:
            results, usage = self._fetch_dataset_retriever(node_data=self._node_data, variables=variables)
            outputs = {"result": ArrayObjectSegment(value=[item.model_dump(by_alias=True) for item in results])}
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.SUCCEEDED,
                inputs=variables,
                process_data={"usage": jsonable_encoder(usage)},
                outputs=outputs,  # type: ignore
                metadata={
                    WorkflowNodeExecutionMetadataKey.TOTAL_TOKENS: usage.total_tokens,
                    WorkflowNodeExecutionMetadataKey.TOTAL_PRICE: usage.total_price,
                    WorkflowNodeExecutionMetadataKey.CURRENCY: usage.currency,
                },
                llm_usage=usage,
            )
        except RateLimitExceededError as e:
            logger.warning(e, exc_info=True)
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED,
                inputs=variables,
                error=str(e),
                error_type=type(e).__name__,
                llm_usage=usage,
            )
        except KnowledgeRetrievalNodeError as e:
            logger.warning("Error when running knowledge retrieval node", exc_info=True)
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED,
                inputs=variables,
                error=str(e),
                error_type=type(e).__name__,
                llm_usage=usage,
            )
        # Temporary handle all exceptions from DatasetRetrieval class here.
        except Exception as e:
            logger.warning(e, exc_info=True)
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED,
                inputs=variables,
                error=str(e),
                error_type=type(e).__name__,
                llm_usage=usage,
            )

    def _fetch_dataset_retriever(
        self, node_data: KnowledgeRetrievalNodeData, variables: dict[str, Any]
    ) -> tuple[list[Source], LLMUsage]:
        dify_ctx = DifyRunContext.model_validate(self.require_run_context_value(DIFY_RUN_CONTEXT_KEY))
        dataset_ids = node_data.dataset_ids
        query = variables.get("query")
        attachments = variables.get("attachments")
        retrieval_resource_list = []

        metadata_filtering_mode: Literal["disabled", "automatic", "manual"] = "disabled"
        if node_data.metadata_filtering_mode is not None:
            metadata_filtering_mode = node_data.metadata_filtering_mode

        resolved_metadata_conditions = (
            self._resolve_metadata_filtering_conditions(node_data.metadata_filtering_conditions)
            if node_data.metadata_filtering_conditions
            else None
        )

        if str(node_data.retrieval_mode) == DatasetRetrieveConfigEntity.RetrieveStrategy.SINGLE and query:
            # fetch model config
            if node_data.single_retrieval_config is None:
                raise ValueError("single_retrieval_config is required for single retrieval mode")
            model = node_data.single_retrieval_config.model
            retrieval_resource_list = self._rag_retrieval.knowledge_retrieval(
                request=KnowledgeRetrievalRequest(
                    tenant_id=dify_ctx.tenant_id,
                    user_id=dify_ctx.user_id,
                    app_id=dify_ctx.app_id,
                    user_from=dify_ctx.user_from.value,
                    dataset_ids=dataset_ids,
                    retrieval_mode=DatasetRetrieveConfigEntity.RetrieveStrategy.SINGLE.value,
                    completion_params=model.completion_params,
                    model_provider=model.provider,
                    model_mode=model.mode,
                    model_name=model.name,
                    metadata_model_config=node_data.metadata_model_config,
                    metadata_filtering_conditions=resolved_metadata_conditions,
                    metadata_filtering_mode=metadata_filtering_mode,
                    query=query,
                )
            )
        elif str(node_data.retrieval_mode) == DatasetRetrieveConfigEntity.RetrieveStrategy.MULTIPLE:
            if node_data.multiple_retrieval_config is None:
                raise ValueError("multiple_retrieval_config is required")
            reranking_model: RerankingModelDict | None = None
            weights: WeightsDict | None = None
            match node_data.multiple_retrieval_config.reranking_mode:
                case "reranking_model":
                    if node_data.multiple_retrieval_config.reranking_model:
                        reranking_model = {
                            "reranking_provider_name": node_data.multiple_retrieval_config.reranking_model.provider,
                            "reranking_model_name": node_data.multiple_retrieval_config.reranking_model.model,
                        }
                    else:
                        reranking_model = None
                    weights = None
                case "weighted_score":
                    if node_data.multiple_retrieval_config.weights is None:
                        raise ValueError("weights is required")
                    reranking_model = None
                    vector_setting = node_data.multiple_retrieval_config.weights.vector_setting
                    weights = {
                        "vector_setting": {
                            "vector_weight": vector_setting.vector_weight,
                            "embedding_provider_name": vector_setting.embedding_provider_name,
                            "embedding_model_name": vector_setting.embedding_model_name,
                        },
                        "keyword_setting": {
                            "keyword_weight": node_data.multiple_retrieval_config.weights.keyword_setting.keyword_weight
                        },
                    }
                case _:
                    # Handle any other reranking_mode values
                    reranking_model = None
                    weights = None

            retrieval_resource_list = self._rag_retrieval.knowledge_retrieval(
                request=KnowledgeRetrievalRequest(
                    app_id=dify_ctx.app_id,
                    tenant_id=dify_ctx.tenant_id,
                    user_id=dify_ctx.user_id,
                    user_from=dify_ctx.user_from.value,
                    dataset_ids=dataset_ids,
                    query=query,
                    retrieval_mode=DatasetRetrieveConfigEntity.RetrieveStrategy.MULTIPLE.value,
                    top_k=node_data.multiple_retrieval_config.top_k,
                    score_threshold=node_data.multiple_retrieval_config.score_threshold
                    if node_data.multiple_retrieval_config.score_threshold is not None
                    else 0.0,
                    reranking_mode=node_data.multiple_retrieval_config.reranking_mode,
                    reranking_model=reranking_model,
                    weights=weights,
                    reranking_enable=node_data.multiple_retrieval_config.reranking_enable,
                    metadata_model_config=node_data.metadata_model_config,
                    metadata_filtering_conditions=resolved_metadata_conditions,
                    metadata_filtering_mode=metadata_filtering_mode,
                    attachment_ids=[
                        parsed_reference.record_id
                        for attachment in attachments
                        if (parsed_reference := parse_file_reference(attachment.reference)) is not None
                    ]
                    if attachments
                    else None,
                )
            )

        usage = self._rag_retrieval.llm_usage
        return retrieval_resource_list, usage

    def _resolve_metadata_filtering_conditions(
        self, conditions: MetadataFilteringCondition
    ) -> MetadataFilteringCondition:
        if conditions.conditions is None:
            return MetadataFilteringCondition(
                logical_operator=conditions.logical_operator,
                conditions=None,
            )

        variable_pool = self.graph_runtime_state.variable_pool
        resolved_conditions: list[Condition] = []
        for cond in conditions.conditions or []:
            value = cond.value
            if isinstance(value, str):
                segment_group = variable_pool.convert_template(value)
                if len(segment_group.value) == 1:
                    resolved_value = segment_group.value[0].to_object()
                else:
                    resolved_value = segment_group.text
            elif isinstance(value, Sequence) and all(isinstance(v, str) for v in value):
                resolved_values = []
                for v in value:  # type: ignore
                    segment_group = variable_pool.convert_template(v)
                    if len(segment_group.value) == 1:
                        resolved_values.append(segment_group.value[0].to_object())
                    else:
                        resolved_values.append(segment_group.text)
                resolved_value = resolved_values
            else:
                resolved_value = value
            resolved_conditions.append(
                Condition(
                    name=cond.name,
                    comparison_operator=cond.comparison_operator,
                    value=resolved_value,
                )
            )
        return MetadataFilteringCondition(
            logical_operator=conditions.logical_operator or "and",
            conditions=resolved_conditions,
        )

    @classmethod
    def _extract_variable_selector_to_variable_mapping(
        cls,
        *,
        graph_config: Mapping[str, Any],
        node_id: str,
        node_data: KnowledgeRetrievalNodeData,
    ) -> Mapping[str, Sequence[str]]:
        # graph_config is not used in this node type
        variable_mapping = {}
        if node_data.query_variable_selector:
            variable_mapping[node_id + ".query"] = node_data.query_variable_selector
        if node_data.query_attachment_selector:
            variable_mapping[node_id + ".queryAttachment"] = node_data.query_attachment_selector
        return variable_mapping

```

### api/core/workflow/nodes/datasource/datasource_node.py
```py
from collections.abc import Generator, Mapping, Sequence
from typing import TYPE_CHECKING, Any

from graphon.entities.graph_config import NodeConfigDict
from graphon.enums import (
    BuiltinNodeTypes,
    NodeExecutionType,
    WorkflowNodeExecutionMetadataKey,
    WorkflowNodeExecutionStatus,
)
from graphon.node_events import NodeRunResult, StreamCompletedEvent
from graphon.nodes.base.node import Node
from graphon.nodes.base.variable_template_parser import VariableTemplateParser

from core.app.entities.app_invoke_entities import DIFY_RUN_CONTEXT_KEY, DifyRunContext
from core.datasource.datasource_manager import DatasourceManager
from core.datasource.entities.datasource_entities import DatasourceProviderType
from core.plugin.impl.exc import PluginDaemonClientSideError
from core.workflow.file_reference import resolve_file_record_id
from core.workflow.system_variables import SystemVariableKey, get_system_segment

from .entities import DatasourceNodeData, DatasourceParameter, OnlineDriveDownloadFileParam
from .exc import DatasourceNodeError

if TYPE_CHECKING:
    from graphon.entities import GraphInitParams
    from graphon.runtime import GraphRuntimeState


class DatasourceNode(Node[DatasourceNodeData]):
    """
    Datasource Node
    """

    node_type = BuiltinNodeTypes.DATASOURCE
    execution_type = NodeExecutionType.ROOT

    def __init__(
        self,
        id: str,
        config: NodeConfigDict,
        graph_init_params: "GraphInitParams",
        graph_runtime_state: "GraphRuntimeState",
    ):
        super().__init__(
            id=id,
            config=config,
            graph_init_params=graph_init_params,
            graph_runtime_state=graph_runtime_state,
        )
        self.datasource_manager = DatasourceManager

    def populate_start_event(self, event) -> None:
        event.provider_id = f"{self.node_data.plugin_id}/{self.node_data.provider_name}"
        event.provider_type = self.node_data.provider_type

    def _run(self) -> Generator:
        """
        Run the datasource node
        """
        dify_ctx = DifyRunContext.model_validate(self.require_run_context_value(DIFY_RUN_CONTEXT_KEY))
        node_data = self.node_data
        variable_pool = self.graph_runtime_state.variable_pool
        datasource_type_segment = get_system_segment(variable_pool, SystemVariableKey.DATASOURCE_TYPE)
        if not datasource_type_segment:
            raise DatasourceNodeError("Datasource type is not set")
        datasource_type = str(datasource_type_segment.value) if datasource_type_segment.value else None
        datasource_info_segment = get_system_segment(variable_pool, SystemVariableKey.DATASOURCE_INFO)
        if not datasource_info_segment:
            raise DatasourceNodeError("Datasource info is not set")
        datasource_info_value = datasource_info_segment.value
        if not isinstance(datasource_info_value, dict):
            raise DatasourceNodeError("Invalid datasource info format")
        datasource_info: dict[str, Any] = datasource_info_value

        if datasource_type is None:
            raise DatasourceNodeError("Datasource type is not set")

        datasource_type = DatasourceProviderType.value_of(datasource_type)
        provider_id = f"{node_data.plugin_id}/{node_data.provider_name}"

        datasource_info["icon"] = self.datasource_manager.get_icon_url(
            provider_id=provider_id,
            datasource_name=node_data.datasource_name or "",
            tenant_id=dify_ctx.tenant_id,
            datasource_type=datasource_type.value,
        )

        parameters_for_log = datasource_info

        try:
            match datasource_type:
                case DatasourceProviderType.ONLINE_DOCUMENT | DatasourceProviderType.ONLINE_DRIVE:
                    # Build typed request objects
                    datasource_parameters = None
                    if datasource_type == DatasourceProviderType.ONLINE_DOCUMENT:
                        datasource_parameters = DatasourceParameter(
                            workspace_id=datasource_info.get("workspace_id", ""),
                            page_id=datasource_info.get("page", {}).get("page_id", ""),
                            type=datasource_info.get("page", {}).get("type", ""),
                        )

                    online_drive_request = None
                    if datasource_type == DatasourceProviderType.ONLINE_DRIVE:
                        online_drive_request = OnlineDriveDownloadFileParam(
                            id=datasource_info.get("id", ""),
                            bucket=datasource_info.get("bucket", ""),
                        )

                    credential_id = datasource_info.get("credential_id", "")

                    yield from self.datasource_manager.stream_node_events(
                        node_id=self._node_id,
                        user_id=dify_ctx.user_id,
                        datasource_name=node_data.datasource_name or "",
                        datasource_type=datasource_type.value,
                        provider_id=provider_id,
                        tenant_id=dify_ctx.tenant_id,
                        provider=node_data.provider_name,
                        plugin_id=node_data.plugin_id,
                        credential_id=credential_id,
                        parameters_for_log=parameters_for_log,
                        datasource_info=datasource_info,
                        variable_pool=variable_pool,
                        datasource_param=datasource_parameters,
                        online_drive_request=online_drive_request,
                    )
                case DatasourceProviderType.WEBSITE_CRAWL:
                    yield StreamCompletedEvent(
                        node_run_result=NodeRunResult(
                            status=WorkflowNodeExecutionStatus.SUCCEEDED,
                            inputs=parameters_for_log,
                            metadata={WorkflowNodeExecutionMetadataKey.DATASOURCE_INFO: datasource_info},
                            outputs={
                                **datasource_info,
                                "datasource_type": datasource_type,
                            },
                        )
                    )
                case DatasourceProviderType.LOCAL_FILE:
                    file_id = resolve_file_record_id(
                        datasource_info.get("reference") or datasource_info.get("related_id")
                    )
                    if not file_id:
                        raise DatasourceNodeError("File is not exist")

                    file_info = self.datasource_manager.get_upload_file_by_id(
                        file_id=file_id, tenant_id=dify_ctx.tenant_id
                    )
                    variable_pool.add([self._node_id, "file"], file_info)
                    # variable_pool.add([self.node_id, "file"], file_info.to_dict())
                    yield StreamCompletedEvent(
                        node_run_result=NodeRunResult(
                            status=WorkflowNodeExecutionStatus.SUCCEEDED,
                            inputs=parameters_for_log,
                            metadata={WorkflowNodeExecutionMetadataKey.DATASOURCE_INFO: datasource_info},
                            outputs={
                                "file": file_info,
                                "datasource_type": datasource_type,
                            },
                        )
                    )
                case _:
                    raise DatasourceNodeError(f"Unsupported datasource provider: {datasource_type}")
        except PluginDaemonClientSideError as e:
            yield StreamCompletedEvent(
                node_run_result=NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs=parameters_for_log,
                    metadata={WorkflowNodeExecutionMetadataKey.DATASOURCE_INFO: datasource_info},
                    error=f"Failed to transform datasource message: {str(e)}",
                    error_type=type(e).__name__,
                )
            )
        except DatasourceNodeError as e:
            yield StreamCompletedEvent(
                node_run_result=NodeRunResult(
                    status=WorkflowNodeExecutionStatus.FAILED,
                    inputs=parameters_for_log,
                    metadata={WorkflowNodeExecutionMetadataKey.DATASOURCE_INFO: datasource_info},
                    error=f"Failed to invoke datasource: {str(e)}",
                    error_type=type(e).__name__,
                )
            )

    @classmethod
    def _extract_variable_selector_to_variable_mapping(
        cls,
        *,
        graph_config: Mapping[str, Any],
        node_id: str,
        node_data: DatasourceNodeData,
    ) -> Mapping[str, Sequence[str]]:
        """
        Extract variable selector to variable mapping
        :param graph_config: graph config
        :param node_id: node id
        :param node_data: node data
        :return:
        """
        result = {}
        if node_data.datasource_parameters:
            for parameter_name in node_data.datasource_parameters:
                input = node_data.datasource_parameters[parameter_name]
                match input.type:
                    case "mixed":
                        assert isinstance(input.value, str)
                        selectors = VariableTemplateParser(input.value).extract_variable_selectors()
                        for selector in selectors:
                            result[selector.variable] = selector.value_selector
                    case "variable":
                        result[parameter_name] = input.value
                    case "constant":
                        pass
                    case None:
                        pass

            result = {node_id + "." + key: value for key, value in result.items()}

        return result

    @classmethod
    def version(cls) -> str:
        return "1"

```

### api/core/workflow/nodes/trigger_webhook/node.py
```py
import logging
from collections.abc import Mapping
from typing import Any

from graphon.enums import NodeExecutionType, WorkflowNodeExecutionStatus
from graphon.file import FileTransferMethod
from graphon.node_events import NodeRunResult
from graphon.nodes.base.node import Node
from graphon.nodes.protocols import FileReferenceFactoryProtocol
from graphon.variables.types import SegmentType
from graphon.variables.variables import FileVariable

from core.trigger.constants import TRIGGER_WEBHOOK_NODE_TYPE
from core.workflow.file_reference import resolve_file_record_id
from core.workflow.variable_prefixes import SYSTEM_VARIABLE_NODE_ID
from factories.variable_factory import build_segment_with_type

from .entities import ContentType, WebhookData

logger = logging.getLogger(__name__)


class TriggerWebhookNode(Node[WebhookData]):
    node_type = TRIGGER_WEBHOOK_NODE_TYPE
    execution_type = NodeExecutionType.ROOT

    _file_reference_factory: FileReferenceFactoryProtocol

    def post_init(self) -> None:
        from core.workflow.node_runtime import DifyFileReferenceFactory

        self._file_reference_factory = DifyFileReferenceFactory(self.graph_init_params.run_context)

    @classmethod
    def get_default_config(cls, filters: Mapping[str, object] | None = None) -> Mapping[str, object]:
        return {
            "type": "webhook",
            "config": {
                "method": "get",
                "content_type": "application/json",
                "headers": [],
                "params": [],
                "body": [],
                "async_mode": True,
                "status_code": 200,
                "response_body": "",
                "timeout": 30,
            },
        }

    @classmethod
    def version(cls) -> str:
        return "1"

    def _run(self) -> NodeRunResult:
        """
        Run the webhook node.

        Like the start node, this simply takes the webhook data from the variable pool
        and makes it available to downstream nodes. The actual webhook handling
        happens in the trigger controller.
        """
        # Get webhook data from variable pool (injected by Celery task)
        webhook_inputs = dict(self.graph_runtime_state.variable_pool.get_by_prefix(self.id))

        # Extract webhook-specific outputs based on node configuration
        outputs = self._extract_configured_outputs(webhook_inputs)
        system_inputs = self.graph_runtime_state.variable_pool.get_by_prefix(SYSTEM_VARIABLE_NODE_ID)

        for variable_name, value in system_inputs.items():
            outputs[f"{SYSTEM_VARIABLE_NODE_ID}.{variable_name}"] = value
        return NodeRunResult(
            status=WorkflowNodeExecutionStatus.SUCCEEDED,
            inputs=webhook_inputs,
            outputs=outputs,
        )

    def generate_file_var(self, param_name: str, file: dict):
        file_id = resolve_file_record_id(file.get("reference") or file.get("related_id"))
        transfer_method_value = file.get("transfer_method")
        if transfer_method_value:
            transfer_method = FileTransferMethod.value_of(transfer_method_value)
            match transfer_method:
                case FileTransferMethod.LOCAL_FILE | FileTransferMethod.REMOTE_URL:
                    file["upload_file_id"] = file_id
                case FileTransferMethod.TOOL_FILE:
                    file["tool_file_id"] = file_id
                case FileTransferMethod.DATASOURCE_FILE:
                    file["datasource_file_id"] = file_id

            try:
                file_obj = self._file_reference_factory.build_from_mapping(mapping=file)
                file_segment = build_segment_with_type(SegmentType.FILE, file_obj)
                return FileVariable(name=param_name, value=file_segment.value, selector=[self.id, param_name])
            except ValueError:
                logger.error(
                    "Failed to build FileVariable for webhook file parameter %s",
                    param_name,
                    exc_info=True,
                )
        return None

    def _extract_configured_outputs(self, webhook_inputs: dict[str, Any]) -> dict[str, Any]:
        """Extract outputs based on node configuration from webhook inputs."""
        outputs = {}

        # Get the raw webhook data (should be injected by Celery task)
        webhook_data = webhook_inputs.get("webhook_data", {})

        def _to_sanitized(name: str) -> str:
            return name.replace("-", "_")

        def _get_normalized(mapping: dict[str, Any], key: str) -> Any:
            if not isinstance(mapping, dict):
                return None
            if key in mapping:
                return mapping[key]
            alternate = key.replace("-", "_") if "-" in key else key.replace("_", "-")
            if alternate in mapping:
                return mapping[alternate]
            return None

        # Extract configured headers (case-insensitive)
        webhook_headers = webhook_data.get("headers", {})
        webhook_headers_lower = {k.lower(): v for k, v in webhook_headers.items()}

        for header in self.node_data.headers:
            header_name = header.name
            value = _get_normalized(webhook_headers, header_name)
            if value is None:
                value = _get_normalized(webhook_headers_lower, header_name.lower())
            sanitized_name = _to_sanitized(header_name)
            outputs[sanitized_name] = value

        # Extract configured query parameters
        for param in self.node_data.params:
            param_name = param.name
            outputs[param_name] = webhook_data.get("query_params", {}).get(param_name)

        # Extract configured body parameters
        for body_param in self.node_data.body:
            param_name = body_param.name
            param_type = body_param.type

            if self.node_data.content_type == ContentType.TEXT:
                # For text/plain, the entire body is a single string parameter
                outputs[param_name] = str(webhook_data.get("body", {}).get("raw", ""))
                continue
            elif self.node_data.content_type == ContentType.BINARY:
                raw_data: dict = webhook_data.get("body", {}).get("raw", {})
                file_var = self.generate_file_var(param_name, raw_data)
                if file_var:
                    outputs[param_name] = file_var
                else:
                    outputs[param_name] = raw_data
                continue

            if param_type == SegmentType.FILE:
                # Get File object (already processed by webhook controller)
                files = webhook_data.get("files", {})
                if files and isinstance(files, dict):
                    file = files.get(param_name)
                    if file and isinstance(file, dict):
                        file_var = self.generate_file_var(param_name, file)
                        if file_var:
                            outputs[param_name] = file_var
                        else:
                            outputs[param_name] = files
                    else:
                        outputs[param_name] = files
                else:
                    outputs[param_name] = files
            else:
                # Get regular body parameter
                outputs[param_name] = webhook_data.get("body", {}).get(param_name)

        # Include raw webhook data for debugging/advanced use
        outputs["_webhook_raw"] = webhook_data
        return outputs

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-010.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
