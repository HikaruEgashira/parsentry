You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-010
- **Kind**: endpoint
- **Identifier**: Workflow Engine (Node Execution, Variable Interpolation)
- **Description**: Visual workflow engine executing user-defined node graphs with variable interpolation. Risk of template injection via variable substitution, workflow logic bypass, and resource exhaustion from infinite loops.
- **Locations**: api/core/workflow/nodes/, api/core/app/workflow/, api/controllers/console/app/, web/app/(shareLayout)/workflow/

## Repository Context

## Directory Structure
```
AGENTS.md
AUTHORS
CLAUDE.md
CONTRIBUTING.md
LICENSE
Makefile
README.md
api/ 
  AGENTS.md
  Dockerfile
  README.md
  app.py
  app_factory.py
  celery_entrypoint.py
  celery_healthcheck.py
  cnt_base.sh
  commands/ 
  configs/ 
    deploy/ 
    enterprise/ 
    extra/ 
    feature/ 
      hosted_service/ 
    middleware/ 
      cache/ 
      storage/ 
      vdb/ 
    observability/ 
      otel/ 
    packaging/ 
    remote_settings_sources/ 
      apollo/ 
      nacos/ 
  constants/ 
  context/ 
  contexts/ 
  controllers/ 
    common/ 
    console/ 
      app/ 
      auth/ 
      billing/ 
      datasets/ 
      explore/ 
      tag/ 
      workspace/ 
    files/ 
    inner_api/ 
      app/ 
      plugin/ 
      workspace/ 
    mcp/ 
    service_api/ 
      app/ 
      dataset/ 
      end_user/ 
      workspace/ 
    trigger/ 
    web/ 
  core/ 
    agent/ 
      output_parser/ 
      prompt/ 
      strategy/ 
    app/ 
      app_config/ 
      apps/ 
      entities/ 
      features/ 
      file_access/ 
      layers/ 
      llm/ 
      task_pipeline/ 
      workflow/ 
    base/ 
      tts/ 
    callback_handler/ 
    datasource/ 
      __base/ 
      entities/ 
      local_file/ 
      online_document/ 
      online_drive/ 
      utils/ 
      website_crawl/ 
    db/ 
    entities/ 
    errors/ 
    extension/ 
    external_data_tool/ 
      api/ 
    helper/ 
      code_executor/ 
    llm_generator/ 
      output_parser/ 
    logging/ 
    mcp/ 
      auth/ 
      client/ 
      server/ 
      session/ 
    memory/ 
    moderation/ 
      api/ 
      keywords/ 
      openai_moderation/ 
    ops/ 
      aliyun_trace/ 
      arize_phoenix_trace/ 
      entities/ 
      langfuse_trace/ 
      langsmith_trace/ 
      mlflow_trace/ 
      opik_trace/ 
      tencent_trace/ 
      weave_trace/ 
    plugin/ 
      backwards_invocation/ 
      endpoint/ 
      entities/ 
      impl/ 
      utils/ 
    prompt/ 
      entities/ 
      prompt_templates/ 
      utils/ 
    rag/ 
      cleaner/ 
      data_post_processor/ 
      datasource/ 
      docstore/ 
      embedding/ 
      entities/ 
      extractor/ 
      index_processor/ 
      models/ 
      pipeline/ 
      rerank/ 
      retrieval/ 
      splitter/ 
      summary_index/ 
    repositories/ 
    schemas/ 
      builtin/ 
    telemetry/ 
    tools/ 
      __base/ 
      builtin_tool/ 
      custom_tool/ 
      entities/ 
      mcp_tool/ 
      plugin_tool/ 
      utils/ 
      workflow_as_tool/ 
    trigger/ 
      debug/ 
      entities/ 
      utils/ 
    workflow/ 
      nodes/ 
  dify_app.py
  docker/ 
  enterprise/ 
    telemetry/ 
      entities/ 
  enums/ 
  events/ 
    event_handlers/ 
  extensions/ 
    logstore/ 
      repositories/ 
    otel/ 
      decorators/ 
      parser/ 
      semconv/ 
    storage/ 
      clickzetta_volume/ 
  factories/ 
    file_factory/ 
  fields/ 
  gunicorn.conf.py
  libs/ 
    broadcast_channel/ 
      redis/ 
  migrations/ 
    versions/ 
  models/ 
    utils/ 
  pyproject.toml
  pyrefly-local-excludes.txt
  pyrightconfig.json
  pytest.ini
  repositories/ 
    entities/ 
  schedule/ 
  services/ 
    auth/ 
      firecrawl/ 
      jina/ 
      watercrawl/ 
    document_indexing_proxy/ 
    enterprise/ 
    entities/ 
      external_knowledge_entities/ 
      knowledge_entities/ 
    errors/ 
    plugin/ 
    rag_pipeline/ 
      entity/ 
      pipeline_template/ 
      transform/ 
    recommend_app/ 
      buildin/ 
      database/ 
      remote/ 
    retention/ 
      conversation/ 
      workflow_run/ 
    tools/ 
    trigger/ 
    workflow/ 
  tasks/ 
    annotation/ 
    app_generate/ 
    rag_pipeline/ 
    workflow_cfs_scheduler/ 
  templates/ 
    without-brand/ 
  tests/ 
    fixtures/ 
      workflow/ 
    integration_tests/ 
      controllers/ 
      core/ 
      factories/ 
      libs/ 
      model_runtime/ 
      plugin/ 
      services/ 
      storage/ 
      tasks/ 
      tools/ 
      utils/ 
      vdb/ 
      workflow/ 
    test_containers_integration_tests/ 
      controllers/ 
      core/ 
      factories/ 
      helpers/ 
      libs/ 
      models/ 
      repositories/ 
      services/ 
      tasks/ 
      trigger/ 
      workflow/ 
    unit_tests/ 
      commands/ 
      configs/ 
      controllers/ 
      core/ 
      enterprise/ 
      events/ 
      extensions/ 
      factories/ 
      fields/ 
      libs/ 
      models/ 
      oss/ 
      repositories/ 
      services/ 
      tasks/ 
      tools/ 
      utils/ 
  uv.lock
codecov.yml
dev/ 
  basedpyright-check
  pyrefly-check-local
  pytest/ 
  reformat
  setup
  start-api
  start-beat
  start-docker-compose
  start-web
  start-worker
  sync-uv
  ty-check
  update-uv
docker/ 
  README.md
  certbot/ 
  couchbase-server/ 
  dify-env-sync.py
  dify-env-sync.sh
  docker-compose-template.yaml
  docker-compose.middleware.yaml
  docker-compose.png
  docker-compose.yaml
  elasticsearch/ 
  generate_docker_compose
  iris/ 
  middleware.env.example
  nginx/ 
    conf.d/ 
    ssl/ 
  pgvector/ 
  ssrf_proxy/ 
  startupscripts/ 
  tidb/ 
    config/ 
  volumes/ 
    myscale/ 
      config/ 
    oceanbase/ 
      init.d/ 
    opensearch/ 
    sandbox/ 
      conf/ 
docs/ 
  ar-SA/ 
  bn-BD/ 
  de-DE/ 
  es-ES/ 
  eu-ai-act-compliance.md
  fr-FR/ 
  hi-IN/ 
  it-IT/ 
  ja-JP/ 
  ko-KR/ 
  pt-BR/ 
  sl-SI/ 
  suggested-questions-configuration.md
  tlh/ 
  tr-TR/ 
  vi-VN/ 
  weaviate/ 
    WEAVIATE_MIGRATION_GUIDE/ 
  zh-CN/ 
  zh-TW/ 
e2e/ 
  AGENTS.md
  README.md
  cucumber.config.ts
  features/ 
    apps/ 
    smoke/ 
    step-definitions/ 
      apps/ 
      common/ 
      smoke/ 
    support/ 
  fixtures/ 
  package.json
  scripts/ 
  support/ 
  test-env.ts
  tsconfig.json
  vite.config.ts
images/ 
  GitHub_README_if.png
  describe.png
  models.png
package.json
packages/ 
  iconify-collections/ 
    assets/ 
      public/ 
      vender/ 
    custom-public/ 
    custom-vender/ 
    scripts/ 
pnpm-lock.yaml
pnpm-workspace.yaml
scripts/ 
  stress-test/ 
    common/ 
    setup/ 
      dsl/ 
sdks/ 
  README.md
  nodejs-client/ 
    scripts/ 
    src/ 
      client/ 
      errors/ 
      http/ 
      internal/ 
      types/ 
    tests/ 
  php-client/ 
vite.config.ts
web/ 
  AGENTS.md
  CLAUDE.md
  Dockerfile
  Dockerfile.dockerignore
  README.md
  __mocks__/ 
    @tanstack/ 
  __tests__/ 
    apps/ 
    billing/ 
    datasets/ 
    develop/ 
    explore/ 
    goto-anything/ 
    plugins/ 
    rag-pipeline/ 
    share/ 
    tools/ 
  app/ 
    (commonLayout)/ 
      app/ 
      apps/ 
      datasets/ 
      education-apply/ 
      explore/ 
      plugins/ 
      tools/ 
    (humanInputLayout)/ 
      form/ 
    (shareLayout)/ 
      chat/ 
      chatbot/ 
      completion/ 
      components/ 
      webapp-reset-password/ 
      webapp-signin/ 
      workflow/ 
    account/ 
      (commonLayout)/ 
      oauth/ 
    activate/ 
    components/ 
      app/ 
      app-sidebar/ 
      apps/ 
      base/ 
      billing/ 
      custom/ 
      datasets/ 
      develop/ 
      devtools/ 
      explore/ 
      goto-anything/ 
      header/ 
      plugins/ 
      provider/ 
      rag-pipeline/ 
      share/ 
      signin/ 
      tools/ 
      workflow/ 
      workflow-app/ 
    education-apply/ 
    forgot-password/ 
    init/ 
    install/ 
    oauth-callback/ 
    reset-password/ 
      check-code/ 
      set-password/ 
    signin/ 
      assets/ 
      check-code/ 
      components/ 
      invite-settings/ 
      utils/ 
    signup/ 
      check-code/ 
      components/ 
      set-password/ 
    styles/ 
  assets/ 
  bin/ 
  config/ 
  constants/ 
  context/ 
    hooks/ 
  contract/ 
    console/ 
  docker/ 
  docs/ 
  env.ts
  eslint-suppressions.json
  eslint.config.mjs
  eslint.constants.mjs
  global.d.ts
  hooks/ 
  i18n/ 
    ar-TN/ 
    de-DE/ 
    en-US/ 
    es-ES/ 
    fa-IR/ 
    fr-FR/ 
    hi-IN/ 
    id-ID/ 
    it-IT/ 
    ja-JP/ 
    ko-KR/ 
    nl-NL/ 
    pl-PL/ 
    pt-BR/ 
    ro-RO/ 
    ru-RU/ 
    sl-SI/ 
    th-TH/ 
    tr-TR/ 
    uk-UA/ 
    vi-VN/ 
    zh-Hans/ 
    zh-Hant/ 
  i18n-config/ 
  instrumentation-client.ts
  knip.config.ts
  models/ 
  next/ 
  next.config.ts
  package.json
  plugins/ 
    dev-proxy/ 
    eslint/ 
      rules/ 
    vite/ 
  postcss.config.js
  proxy.ts
  public/ 
    education/ 
    in-site-message/ 
    logo/ 
    screenshots/ 
      dark/ 
      light/ 
    vs/ 
      base/ 
      basic-languages/ 
      editor/ 
      language/ 
  scripts/ 
  service/ 
    knowledge/ 
  tailwind-common-config.ts
  tailwind.config.ts
  test/ 
  themes/ 
  tsconfig.json
  tsslint.config.ts
  types/ 
  typography.js
  utils/ 
  vite.config.ts
  vitest.setup.ts

```

## Languages
- TypeScript: 5508 files
- Python: 2785 files
- JavaScript: 122 files
- Yaml: 95 files
- Bash: 20 files
- Php: 1 files

## Dependencies
### package.json
```
{
  "name": "dify",
  "private": true,
  "scripts": {
    "prepare": "vp config"
  },
  "devDependencies": {
    "vite-plus": "catalog:"
  },
  "engines": {
    "node": "^22.22.1"
  },
  "packageManager": "pnpm@10.33.0"
}

```

## Entry Points
- sdks/nodejs-client/src/index.ts
- web/next/index.ts
- web/types/app.ts
- web/app/components/tools/utils/index.ts
- web/app/components/plugins/plugin-detail-panel/tool-selector/components/index.ts
- web/app/components/plugins/plugin-detail-panel/tool-selector/hooks/index.ts
- web/app/components/plugins/plugin-detail-panel/detail-header/components/index.ts
- web/app/components/plugins/plugin-detail-panel/detail-header/hooks/index.ts
- web/app/components/goto-anything/components/index.ts
- web/app/components/goto-anything/hooks/index.ts
- web/app/components/goto-anything/actions/commands/index.ts
- web/app/components/goto-anything/actions/index.ts
- web/app/components/workflow-app/hooks/index.ts
- web/app/components/datasets/documents/create-from-pipeline/data-source/store/index.ts
- web/app/components/datasets/documents/create-from-pipeline/steps/index.ts
- web/app/components/datasets/documents/create-from-pipeline/hooks/index.ts
- web/app/components/datasets/documents/components/document-list/components/index.ts
- web/app/components/datasets/documents/components/document-list/hooks/index.ts
- web/app/components/datasets/documents/detail/completed/components/index.ts
- web/app/components/datasets/documents/detail/completed/hooks/index.ts
- web/app/components/datasets/documents/detail/embedding/components/index.ts
- web/app/components/datasets/documents/detail/embedding/hooks/index.ts
- web/app/components/datasets/create/step-one/components/index.ts
- web/app/components/datasets/create/step-one/hooks/index.ts
- web/app/components/datasets/create/step-two/components/index.ts
- web/app/components/datasets/create/step-two/hooks/index.ts
- web/app/components/rag-pipeline/utils/index.ts
- web/app/components/rag-pipeline/hooks/index.ts
- web/app/components/rag-pipeline/store/index.ts
- web/app/components/workflow/hooks-store/index.ts
- web/app/components/workflow/note-node/note-editor/theme/index.ts
- web/app/components/workflow/utils/index.ts
- web/app/components/workflow/hooks/use-workflow-run-event/index.ts
- web/app/components/workflow/hooks/index.ts
- web/app/components/workflow/run/utils/format-log/parallel/index.ts
- web/app/components/workflow/run/utils/format-log/retry/index.ts
- web/app/components/workflow/run/utils/format-log/human-input/index.ts
- web/app/components/workflow/run/utils/format-log/agent/index.ts
- web/app/components/workflow/run/utils/format-log/index.ts
- web/app/components/workflow/run/utils/format-log/iteration/index.ts
- web/app/components/workflow/run/utils/format-log/loop/index.ts
- web/app/components/workflow/store/workflow/index.ts
- web/app/components/workflow/store/index.ts
- web/app/components/header/account-setting/model-provider-page/model-auth/hooks/index.ts
- web/app/components/header/account-setting/data-source-page-new/hooks/index.ts
- web/app/components/base/form/utils/secret-input/index.ts
- web/app/components/base/form/hooks/index.ts
- web/app/components/base/radio/context/index.ts
- web/app/components/base/amplitude/index.ts
- web/app/components/base/markdown-blocks/index.ts
- web/app/components/base/icons/src/public/tracing/index.ts
- web/app/components/base/icons/src/public/llm/index.ts
- web/app/components/base/icons/src/public/education/index.ts
- web/app/components/base/icons/src/public/other/index.ts
- web/app/components/base/icons/src/public/common/index.ts
- web/app/components/base/icons/src/public/knowledge/dataset-card/index.ts
- web/app/components/base/icons/src/public/knowledge/index.ts
- web/app/components/base/icons/src/public/knowledge/online-drive/index.ts
- web/app/components/base/icons/src/public/avatar/index.ts
- web/app/components/base/icons/src/public/files/index.ts
- web/app/components/base/icons/src/public/thought/index.ts
- web/app/components/base/icons/src/public/billing/index.ts
- web/app/components/base/icons/src/vender/pipeline/index.ts
- web/app/components/base/icons/src/vender/features/index.ts
- web/app/components/base/icons/src/vender/other/index.ts
- web/app/components/base/icons/src/vender/plugin/index.ts
- web/app/components/base/icons/src/vender/solid/mediaAndDevices/index.ts
- web/app/components/base/icons/src/vender/solid/security/index.ts
- web/app/components/base/icons/src/vender/solid/general/index.ts
- web/app/components/base/icons/src/vender/solid/development/index.ts
- web/app/components/base/icons/src/vender/solid/education/index.ts
- web/app/components/base/icons/src/vender/solid/shapes/index.ts
- web/app/components/base/icons/src/vender/solid/users/index.ts
- web/app/components/base/icons/src/vender/solid/files/index.ts
- web/app/components/base/icons/src/vender/solid/arrows/index.ts
- web/app/components/base/icons/src/vender/solid/communication/index.ts
- web/app/components/base/icons/src/vender/solid/editor/index.ts
- web/app/components/base/icons/src/vender/solid/FinanceAndECommerce/index.ts
- web/app/components/base/icons/src/vender/solid/alertsAndFeedback/index.ts
- web/app/components/base/icons/src/vender/system/index.ts
- web/app/components/base/icons/src/vender/knowledge/index.ts
- web/app/components/base/icons/src/vender/line/mediaAndDevices/index.ts
- web/app/components/base/icons/src/vender/line/images/index.ts
- web/app/components/base/icons/src/vender/line/general/index.ts
- web/app/components/base/icons/src/vender/line/development/index.ts
- web/app/components/base/icons/src/vender/line/layout/index.ts
- web/app/components/base/icons/src/vender/line/education/index.ts
- web/app/components/base/icons/src/vender/line/others/index.ts
- web/app/components/base/icons/src/vender/line/time/index.ts
- web/app/components/base/icons/src/vender/line/files/index.ts
- web/app/components/base/icons/src/vender/line/arrows/index.ts
- web/app/components/base/icons/src/vender/line/communication/index.ts
- web/app/components/base/icons/src/vender/line/editor/index.ts
- web/app/components/base/icons/src/vender/line/financeAndECommerce/index.ts
- web/app/components/base/icons/src/vender/line/alertsAndFeedback/index.ts
- web/app/components/base/icons/src/vender/workflow/index.ts
- web/app/components/base/file-uploader/index.ts
- web/app/components/billing/utils/index.ts
- web/config/index.ts
- web/plugins/eslint/index.js
- web/plugins/dev-proxy/server.ts
- web/utils/index.ts
- web/models/app.ts
- web/i18n-config/index.ts
- web/i18n-config/server.ts
- packages/iconify-collections/custom-vender/index.js
- packages/iconify-collections/custom-public/index.js
- api/core/plugin/backwards_invocation/app.py
- api/app.py
- api/controllers/web/app.py
- api/controllers/service_api/app/app.py
- api/controllers/console/app/app.py
- api/services/errors/app.py

Total source files: 8531


## Source Code

### api/core/workflow/nodes/datasource/protocols.py
```py
from collections.abc import Generator
from typing import Any, Protocol

from graphon.file import File
from graphon.node_events import StreamChunkEvent, StreamCompletedEvent

from .entities import DatasourceParameter, OnlineDriveDownloadFileParam


class DatasourceManagerProtocol(Protocol):
    @classmethod
    def get_icon_url(cls, provider_id: str, tenant_id: str, datasource_name: str, datasource_type: str) -> str: ...

    @classmethod
    def stream_node_events(
        cls,
        *,
        node_id: str,
        user_id: str,
        datasource_name: str,
        datasource_type: str,
        provider_id: str,
        tenant_id: str,
        provider: str,
        plugin_id: str,
        credential_id: str,
        parameters_for_log: dict[str, Any],
        datasource_info: dict[str, Any],
        variable_pool: Any,
        datasource_param: DatasourceParameter | None = None,
        online_drive_request: OnlineDriveDownloadFileParam | None = None,
    ) -> Generator[StreamChunkEvent | StreamCompletedEvent, None, None]: ...

    @classmethod
    def get_upload_file_by_id(cls, file_id: str, tenant_id: str) -> File: ...

```

### api/core/workflow/nodes/datasource/__init__.py
```py
"""Datasource workflow node package."""

```

### api/core/workflow/nodes/datasource/exc.py
```py
class DatasourceNodeError(ValueError):
    """Base exception for datasource node errors."""

    pass


class DatasourceParameterError(DatasourceNodeError):
    """Exception raised for errors in datasource parameters."""

    pass


class DatasourceFileError(DatasourceNodeError):
    """Exception raised for errors related to datasource files."""

    pass

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

### api/core/workflow/nodes/datasource/entities.py
```py
from typing import Any, Literal, Union

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import BuiltinNodeTypes, NodeType
from pydantic import BaseModel, field_validator
from pydantic_core.core_schema import ValidationInfo


class DatasourceEntity(BaseModel):
    plugin_id: str
    provider_name: str  # redundancy
    provider_type: str
    datasource_name: str | None = "local_file"
    datasource_configurations: dict[str, Any] | None = None
    plugin_unique_identifier: str | None = None  # redundancy


class DatasourceNodeData(BaseNodeData, DatasourceEntity):
    type: NodeType = BuiltinNodeTypes.DATASOURCE

    class DatasourceInput(BaseModel):
        # TODO: check this type
        value: Union[Any, list[str]]
        type: Literal["mixed", "variable", "constant"] | None = None

        @field_validator("type", mode="before")
        @classmethod
        def check_type(cls, value, validation_info: ValidationInfo):
            typ = value
            value = validation_info.data.get("value")
            if typ == "mixed" and not isinstance(value, str):
                raise ValueError("value must be a string")
            elif typ == "variable":
                if not isinstance(value, list):
                    raise ValueError("value must be a list")
                for val in value:
                    if not isinstance(val, str):
                        raise ValueError("value must be a list of strings")
            elif typ == "constant" and not isinstance(value, str | int | float | bool):
                raise ValueError("value must be a string, int, float, or bool")
            return typ

    datasource_parameters: dict[str, DatasourceInput] | None = None


class DatasourceParameter(BaseModel):
    workspace_id: str
    page_id: str
    type: str


class OnlineDriveDownloadFileParam(BaseModel):
    id: str
    bucket: str

```

### api/core/workflow/nodes/trigger_plugin/__init__.py
```py
from .trigger_event_node import TriggerEventNode

__all__ = ["TriggerEventNode"]

```

### api/core/workflow/nodes/trigger_plugin/trigger_event_node.py
```py
from collections.abc import Mapping
from typing import Any

from graphon.enums import NodeExecutionType, WorkflowNodeExecutionMetadataKey, WorkflowNodeExecutionStatus
from graphon.node_events import NodeRunResult
from graphon.nodes.base.node import Node

from core.trigger.constants import TRIGGER_PLUGIN_NODE_TYPE
from core.workflow.variable_prefixes import SYSTEM_VARIABLE_NODE_ID

from .entities import TriggerEventNodeData


class TriggerEventNode(Node[TriggerEventNodeData]):
    node_type = TRIGGER_PLUGIN_NODE_TYPE
    execution_type = NodeExecutionType.ROOT

    @classmethod
    def get_default_config(cls, filters: Mapping[str, object] | None = None) -> Mapping[str, object]:
        return {
            "type": "plugin",
            "config": {
                "title": "",
                "plugin_id": "",
                "provider_id": "",
                "event_name": "",
                "subscription_id": "",
                "plugin_unique_identifier": "",
                "event_parameters": {},
            },
        }

    @classmethod
    def version(cls) -> str:
        return "1"

    def populate_start_event(self, event) -> None:
        event.provider_id = self.node_data.provider_id

    def _run(self) -> NodeRunResult:
        """
        Run the plugin trigger node.

        This node invokes the trigger to convert request data into events
        and makes them available to downstream nodes.
        """

        # Get trigger data passed when workflow was triggered
        metadata: dict[WorkflowNodeExecutionMetadataKey, Any] = {
            WorkflowNodeExecutionMetadataKey.TRIGGER_INFO: {
                "provider_id": self.node_data.provider_id,
                "event_name": self.node_data.event_name,
                "plugin_unique_identifier": self.node_data.plugin_unique_identifier,
            },
        }
        node_inputs = dict(self.graph_runtime_state.variable_pool.get_by_prefix(self.id))
        system_inputs = self.graph_runtime_state.variable_pool.get_by_prefix(SYSTEM_VARIABLE_NODE_ID)

        for variable_name, value in system_inputs.items():
            node_inputs[f"{SYSTEM_VARIABLE_NODE_ID}.{variable_name}"] = value
        outputs = dict(node_inputs)
        return NodeRunResult(
            status=WorkflowNodeExecutionStatus.SUCCEEDED,
            inputs=node_inputs,
            outputs=outputs,
            metadata=metadata,
        )

```

### api/core/workflow/nodes/trigger_plugin/exc.py
```py
class TriggerEventNodeError(ValueError):
    """Base exception for plugin trigger node errors."""

    pass


class TriggerEventParameterError(TriggerEventNodeError):
    """Exception raised for errors in plugin trigger parameters."""

    pass

```

### api/core/workflow/nodes/trigger_plugin/entities.py
```py
from collections.abc import Mapping
from typing import Any, Literal, Union

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import NodeType
from pydantic import BaseModel, Field, ValidationInfo, field_validator

from core.trigger.constants import TRIGGER_PLUGIN_NODE_TYPE
from core.trigger.entities.entities import EventParameter

from .exc import TriggerEventParameterError


class TriggerEventNodeData(BaseNodeData):
    """Plugin trigger node data"""

    type: NodeType = TRIGGER_PLUGIN_NODE_TYPE

    class TriggerEventInput(BaseModel):
        value: Union[Any, list[str]]
        type: Literal["mixed", "variable", "constant"]

        @field_validator("type", mode="before")
        @classmethod
        def check_type(cls, value, validation_info: ValidationInfo):
            type = value
            value = validation_info.data.get("value")

            if value is None:
                return type

            if type == "mixed" and not isinstance(value, str):
                raise ValueError("value must be a string")

            if type == "variable":
                if not isinstance(value, list):
                    raise ValueError("value must be a list")
                for val in value:
                    if not isinstance(val, str):
                        raise ValueError("value must be a list of strings")

            if type == "constant" and not isinstance(value, str | int | float | bool | dict | list):
                raise ValueError("value must be a string, int, float, bool or dict")
            return type

    plugin_id: str = Field(..., description="Plugin ID")
    provider_id: str = Field(..., description="Provider ID")
    event_name: str = Field(..., description="Event name")
    subscription_id: str = Field(..., description="Subscription ID")
    plugin_unique_identifier: str = Field(..., description="Plugin unique identifier")
    event_parameters: Mapping[str, TriggerEventInput] = Field(default_factory=dict, description="Trigger parameters")

    def resolve_parameters(
        self,
        *,
        parameter_schemas: Mapping[str, EventParameter],
    ) -> Mapping[str, Any]:
        """
        Generate parameters based on the given plugin trigger parameters.

        Args:
            parameter_schemas (Mapping[str, EventParameter]): The mapping of parameter schemas.

        Returns:
            Mapping[str, Any]: A dictionary containing the generated parameters.

        """
        result: dict[str, Any] = {}
        for parameter_name in self.event_parameters:
            parameter: EventParameter | None = parameter_schemas.get(parameter_name)
            if not parameter:
                result[parameter_name] = None
                continue
            event_input = self.event_parameters[parameter_name]

            # trigger node only supports constant input
            if event_input.type != "constant":
                raise TriggerEventParameterError(f"Unknown plugin trigger input type '{event_input.type}'")
            result[parameter_name] = event_input.value
        return result

```

### api/core/workflow/nodes/trigger_schedule/trigger_schedule_node.py
```py
from collections.abc import Mapping

from graphon.enums import NodeExecutionType, WorkflowNodeExecutionStatus
from graphon.node_events import NodeRunResult
from graphon.nodes.base.node import Node

from core.trigger.constants import TRIGGER_SCHEDULE_NODE_TYPE
from core.workflow.variable_prefixes import SYSTEM_VARIABLE_NODE_ID

from .entities import TriggerScheduleNodeData


class TriggerScheduleNode(Node[TriggerScheduleNodeData]):
    node_type = TRIGGER_SCHEDULE_NODE_TYPE
    execution_type = NodeExecutionType.ROOT

    @classmethod
    def version(cls) -> str:
        return "1"

    @classmethod
    def get_default_config(cls, filters: Mapping[str, object] | None = None) -> Mapping[str, object]:
        return {
            "type": TRIGGER_SCHEDULE_NODE_TYPE,
            "config": {
                "mode": "visual",
                "frequency": "daily",
                "visual_config": {"time": "12:00 AM", "on_minute": 0, "weekdays": ["sun"], "monthly_days": [1]},
                "timezone": "UTC",
            },
        }

    def _run(self) -> NodeRunResult:
        node_inputs = dict(self.graph_runtime_state.variable_pool.get_by_prefix(self.id))
        system_inputs = self.graph_runtime_state.variable_pool.get_by_prefix(SYSTEM_VARIABLE_NODE_ID)

        for variable_name, value in system_inputs.items():
            node_inputs[f"{SYSTEM_VARIABLE_NODE_ID}.{variable_name}"] = value
        outputs = dict(node_inputs)
        return NodeRunResult(
            status=WorkflowNodeExecutionStatus.SUCCEEDED,
            inputs=node_inputs,
            outputs=outputs,
        )

```

### api/core/workflow/nodes/trigger_schedule/__init__.py
```py
from .trigger_schedule_node import TriggerScheduleNode

__all__ = ["TriggerScheduleNode"]

```

### api/core/workflow/nodes/trigger_schedule/exc.py
```py
from graphon.entities.exc import BaseNodeError


class ScheduleNodeError(BaseNodeError):
    """Base schedule node error."""

    pass


class ScheduleNotFoundError(ScheduleNodeError):
    """Schedule not found error."""

    pass


class ScheduleConfigError(ScheduleNodeError):
    """Schedule configuration error."""

    pass


class ScheduleExecutionError(ScheduleNodeError):
    """Schedule execution error."""

    pass


class TenantOwnerNotFoundError(ScheduleExecutionError):
    """Tenant owner not found error for schedule execution."""

    pass

```

### api/core/workflow/nodes/trigger_schedule/entities.py
```py
from typing import Literal, Union

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import NodeType
from pydantic import BaseModel, Field

from core.trigger.constants import TRIGGER_SCHEDULE_NODE_TYPE


class TriggerScheduleNodeData(BaseNodeData):
    """
    Trigger Schedule Node Data
    """

    type: NodeType = TRIGGER_SCHEDULE_NODE_TYPE
    mode: str = Field(default="visual", description="Schedule mode: visual or cron")
    frequency: str | None = Field(default=None, description="Frequency for visual mode: hourly, daily, weekly, monthly")
    cron_expression: str | None = Field(default=None, description="Cron expression for cron mode")
    visual_config: dict | None = Field(default=None, description="Visual configuration details")
    timezone: str = Field(default="UTC", description="Timezone for schedule execution")


class ScheduleConfig(BaseModel):
    node_id: str
    cron_expression: str
    timezone: str = "UTC"


class SchedulePlanUpdate(BaseModel):
    node_id: str | None = None
    cron_expression: str | None = None
    timezone: str | None = None


class VisualConfig(BaseModel):
    """Visual configuration for schedule trigger"""

    # For hourly frequency
    on_minute: int | None = Field(default=0, ge=0, le=59, description="Minute of the hour (0-59)")

    # For daily, weekly, monthly frequencies
    time: str | None = Field(default="12:00 AM", description="Time in 12-hour format (e.g., '2:30 PM')")

    # For weekly frequency
    weekdays: list[Literal["sun", "mon", "tue", "wed", "thu", "fri", "sat"]] | None = Field(
        default=None, description="List of weekdays to run on"
    )

    # For monthly frequency
    monthly_days: list[Union[int, Literal["last"]]] | None = Field(
        default=None, description="Days of month to run on (1-31 or 'last')"
    )

```

### api/core/workflow/nodes/__init__.py
```py
"""Workflow node implementations that remain under the legacy core.workflow namespace."""

```

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

### api/core/workflow/nodes/agent/message_transformer.py
```py
from __future__ import annotations

from collections.abc import Generator, Mapping
from typing import Any, cast

from graphon.enums import BuiltinNodeTypes, NodeType, WorkflowNodeExecutionMetadataKey, WorkflowNodeExecutionStatus
from graphon.file import File, FileTransferMethod, get_file_type_by_mime_type
from graphon.model_runtime.entities.llm_entities import LLMUsage, LLMUsageMetadata
from graphon.model_runtime.utils.encoders import jsonable_encoder
from graphon.node_events import (
    AgentLogEvent,
    NodeEventBase,
    NodeRunResult,
    StreamChunkEvent,
    StreamCompletedEvent,
)
from graphon.variables.segments import ArrayFileSegment
from sqlalchemy import select
from sqlalchemy.orm import Session

from core.app.file_access import DatabaseFileAccessController
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.utils.message_transformer import ToolFileMessageTransformer
from extensions.ext_database import db
from factories import file_factory
from models import ToolFile
from services.tools.builtin_tools_manage_service import BuiltinToolManageService

from .exceptions import AgentNodeError, AgentVariableTypeError, ToolFileNotFoundError

_file_access_controller = DatabaseFileAccessController()


class AgentMessageTransformer:
    def transform(
        self,
        *,
        messages: Generator[ToolInvokeMessage, None, None],
        tool_info: Mapping[str, Any],
        parameters_for_log: dict[str, Any],
        user_id: str,
        tenant_id: str,
        conversation_id: str | None,
        node_type: NodeType,
        node_id: str,
        node_execution_id: str,
    ) -> Generator[NodeEventBase, None, None]:
        from core.plugin.impl.plugin import PluginInstaller

        message_stream = ToolFileMessageTransformer.transform_tool_invoke_messages(
            messages=messages,
            user_id=user_id,
            tenant_id=tenant_id,
            conversation_id=conversation_id,
        )

        text = ""
        files: list[File] = []
        json_list: list[dict | list] = []

        agent_logs: list[AgentLogEvent] = []
        agent_execution_metadata: Mapping[WorkflowNodeExecutionMetadataKey, Any] = {}
        llm_usage = LLMUsage.empty_usage()
        variables: dict[str, Any] = {}

        for message in message_stream:
            if message.type in {
                ToolInvokeMessage.MessageType.IMAGE_LINK,
                ToolInvokeMessage.MessageType.BINARY_LINK,
                ToolInvokeMessage.MessageType.IMAGE,
            }:
                assert isinstance(message.message, ToolInvokeMessage.TextMessage)

                url = message.message.text
                if message.meta:
                    transfer_method = message.meta.get("transfer_method", FileTransferMethod.TOOL_FILE)
                    tool_file_id = message.meta.get("tool_file_id")
                else:
                    transfer_method = FileTransferMethod.TOOL_FILE
                    tool_file_id = None
                if not isinstance(tool_file_id, str) or not tool_file_id:
                    raise ToolFileNotFoundError("missing tool_file_id metadata")

                with Session(db.engine) as session:
                    stmt = select(ToolFile).where(ToolFile.id == tool_file_id)
                    tool_file = session.scalar(stmt)
                    if tool_file is None:
                        raise ToolFileNotFoundError(tool_file_id)

                mapping = {
                    "tool_file_id": tool_file_id,
                    "type": get_file_type_by_mime_type(tool_file.mimetype),
                    "transfer_method": transfer_method,
                    "url": url,
                }
                file = file_factory.build_from_mapping(
                    mapping=mapping,
                    tenant_id=tenant_id,
                    access_controller=_file_access_controller,
                )
                files.append(file)
            elif message.type == ToolInvokeMessage.MessageType.BLOB:
                assert isinstance(message.message, ToolInvokeMessage.TextMessage)
                assert message.meta

                tool_file_id = message.meta.get("tool_file_id")
                if not isinstance(tool_file_id, str) or not tool_file_id:
                    raise ToolFileNotFoundError("missing tool_file_id metadata")
                with Session(db.engine) as session:
                    stmt = select(ToolFile).where(ToolFile.id == tool_file_id)
                    tool_file = session.scalar(stmt)
                    if tool_file is None:
                        raise ToolFileNotFoundError(tool_file_id)

                mapping = {
                    "tool_file_id": tool_file_id,
                    "transfer_method": FileTransferMethod.TOOL_FILE,
                }
                files.append(
                    file_factory.build_from_mapping(
                        mapping=mapping,
                        tenant_id=tenant_id,
                        access_controller=_file_access_controller,
                    )
                )
            elif message.type == ToolInvokeMessage.MessageType.TEXT:
                assert isinstance(message.message, ToolInvokeMessage.TextMessage)
                text += message.message.text
                yield StreamChunkEvent(
                    selector=[node_id, "text"],
                    chunk=message.message.text,
                    is_final=False,
                )
            elif message.type == ToolInvokeMessage.MessageType.JSON:
                assert isinstance(message.message, ToolInvokeMessage.JsonMessage)
                if node_type == BuiltinNodeTypes.AGENT:
                    if isinstance(message.message.json_object, dict):
                        msg_metadata: dict[str, Any] = message.message.json_object.pop("execution_metadata", {})
                        llm_usage = LLMUsage.from_metadata(cast(LLMUsageMetadata, msg_metadata))
                        agent_execution_metadata = {
                            WorkflowNodeExecutionMetadataKey(key): value
                            for key, value in msg_metadata.items()
                            if key in WorkflowNodeExecutionMetadataKey.__members__.values()
                        }
                    else:
                        llm_usage = LLMUsage.empty_usage()
                        agent_execution_metadata = {}
                if message.message.json_object:
                    json_list.append(message.message.json_object)
            elif message.type == ToolInvokeMessage.MessageType.LINK:
                assert isinstance(message.message, ToolInvokeMessage.TextMessage)
                stream_text = f"Link: {message.message.text}\n"
                text += stream_text
                yield StreamChunkEvent(
                    selector=[node_id, "text"],
                    chunk=stream_text,
                    is_final=False,
                )
            elif message.type == ToolInvokeMessage.MessageType.VARIABLE:
                assert isinstance(message.message, ToolInvokeMessage.VariableMessage)
                variable_name = message.message.variable_name
                variable_value = message.message.variable_value
                if message.message.stream:
                    if not isinstance(variable_value, str):
                        raise AgentVariableTypeError(
                            "When 'stream' is True, 'variable_value' must be a string.",
                            variable_name=variable_name,
                            expected_type="str",
                            actual_type=type(variable_value).__name__,
                        )
                    if variable_name not in variables:
                        variables[variable_name] = ""
                    variables[variable_name] += variable_value

                    yield StreamChunkEvent(
                        selector=[node_id, variable_name],
                        chunk=variable_value,
                        is_final=False,
                    )
                else:
                    variables[variable_name] = variable_value
            elif message.type == ToolInvokeMessage.MessageType.FILE:
                assert message.meta is not None
                assert isinstance(message.meta, dict)
                if "file" not in message.meta:
                    raise AgentNodeError("File message is missing 'file' key in meta")

                if not isinstance(message.meta["file"], File):
                    raise AgentNodeError(f"Expected File object but got {type(message.meta['file']).__name__}")
                files.append(message.meta["file"])
            elif message.type == ToolInvokeMessage.MessageType.LOG:
                assert isinstance(message.message, ToolInvokeMessage.LogMessage)
                if message.message.metadata:
                    icon = tool_info.get("icon", "")
                    dict_metadata = dict(message.message.metadata)
                    if dict_metadata.get("provider"):
                        manager = PluginInstaller()
                        plugins = manager.list_plugins(tenant_id)
                        try:
                            current_plugin = next(
                                plugin
                                for plugin in plugins
                                if f"{plugin.plugin_id}/{plugin.name}" == dict_metadata["provider"]
                            )
                            icon = current_plugin.declaration.icon
                        except StopIteration:
                            pass
                        icon_dark = None
                        try:
                            builtin_tool = next(
                                provider
                                for provider in BuiltinToolManageService.list_builtin_tools(
                                    user_id,
                                    tenant_id,
                                )
                                if provider.name == dict_metadata["provider"]
                            )
                            icon = builtin_tool.icon
                            icon_dark = builtin_tool.icon_dark
                        except StopIteration:
                            pass

                        dict_metadata["icon"] = icon
                        dict_metadata["icon_dark"] = icon_dark
                        message.message.metadata = dict_metadata
                agent_log = AgentLogEvent(
                    message_id=message.message.id,
                    node_execution_id=node_execution_id,
                    parent_id=message.message.parent_id,
                    error=message.message.error,
                    status=message.message.status.value,
                    data=message.message.data,
                    label=message.message.label,
                    metadata=message.message.metadata,
                    node_id=node_id,
                )

                for log in agent_logs:
                    if log.message_id == agent_log.message_id:
                        log.data = agent_log.data
                        log.status = agent_log.status
                        log.error = agent_log.error
                        log.label = agent_log.label
                        log.metadata = agent_log.metadata
                        break
                else:
                    agent_logs.append(agent_log)

                yield agent_log

        json_output: list[dict[str, Any] | list[Any]] = []
        if agent_logs:
            for log in agent_logs:
                json_output.append(
                    {
                        "id": log.message_id,
                        "parent_id": log.parent_id,
                        "error": log.error,
                        "status": log.status,
                        "data": log.data,
                        "label": log.label,
                        "metadata": log.metadata,
                        "node_id": log.node_id,
                    }
                )
        if json_list:
            json_output.extend(json_list)
        else:
            json_output.append({"data": []})

        yield StreamChunkEvent(
            selector=[node_id, "text"],
            chunk="",
            is_final=True,
        )

        for var_name in variables:
            yield StreamChunkEvent(
                selector=[node_id, var_name],
                chunk="",
                is_final=True,
            )

        yield StreamCompletedEvent(
            node_run_result=NodeRunResult(
                status=WorkflowNodeExecutionStatus.SUCCEEDED,
                outputs={
                    "text": text,
                    "usage": jsonable_encoder(llm_usage),
                    "files": ArrayFileSegment(value=files),
                    "json": json_output,
                    **variables,
                },
                metadata={
                    **agent_execution_metadata,
                    WorkflowNodeExecutionMetadataKey.TOOL_INFO: tool_info,
                    WorkflowNodeExecutionMetadataKey.AGENT_LOG: agent_logs,
                },
                inputs=parameters_for_log,
                llm_usage=llm_usage,
            )
        )

```

### api/core/workflow/nodes/agent/__init__.py
```py
from .agent_node import AgentNode
from .entities import AgentNodeData

__all__ = ["AgentNode", "AgentNodeData"]

```

### api/core/workflow/nodes/agent/strategy_protocols.py
```py
from __future__ import annotations

from collections.abc import Generator, Sequence
from typing import Any, Protocol

from core.agent.plugin_entities import AgentStrategyParameter
from core.plugin.entities.request import InvokeCredentials
from core.tools.entities.tool_entities import ToolInvokeMessage


class ResolvedAgentStrategy(Protocol):
    meta_version: str | None

    def get_parameters(self) -> Sequence[AgentStrategyParameter]: ...

    def invoke(
        self,
        *,
        params: dict[str, Any],
        user_id: str,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
        credentials: InvokeCredentials | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]: ...


class AgentStrategyResolver(Protocol):
    def resolve(
        self,
        *,
        tenant_id: str,
        agent_strategy_provider_name: str,
        agent_strategy_name: str,
    ) -> ResolvedAgentStrategy: ...


class AgentStrategyPresentationProvider(Protocol):
    def get_icon(self, *, tenant_id: str, agent_strategy_provider_name: str) -> str | None: ...

```

### api/core/workflow/nodes/agent/exceptions.py
```py
class AgentNodeError(Exception):
    """Base exception for all agent node errors."""

    def __init__(self, message: str):
        self.message = message
        super().__init__(self.message)


class AgentStrategyError(AgentNodeError):
    """Exception raised when there's an error with the agent strategy."""

    def __init__(self, message: str, strategy_name: str | None = None, provider_name: str | None = None):
        self.strategy_name = strategy_name
        self.provider_name = provider_name
        super().__init__(message)


class AgentStrategyNotFoundError(AgentStrategyError):
    """Exception raised when the specified agent strategy is not found."""

    def __init__(self, strategy_name: str, provider_name: str | None = None):
        super().__init__(
            f"Agent strategy '{strategy_name}' not found"
            + (f" for provider '{provider_name}'" if provider_name else ""),
            strategy_name,
            provider_name,
        )


class AgentInvocationError(AgentNodeError):
    """Exception raised when there's an error invoking the agent."""

    def __init__(self, message: str, original_error: Exception | None = None):
        self.original_error = original_error
        super().__init__(message)


class AgentParameterError(AgentNodeError):
    """Exception raised when there's an error with agent parameters."""

    def __init__(self, message: str, parameter_name: str | None = None):
        self.parameter_name = parameter_name
        super().__init__(message)


class AgentVariableError(AgentNodeError):
    """Exception raised when there's an error with variables in the agent node."""

    def __init__(self, message: str, variable_name: str | None = None):
        self.variable_name = variable_name
        super().__init__(message)


class AgentVariableNotFoundError(AgentVariableError):
    """Exception raised when a variable is not found in the variable pool."""

    def __init__(self, variable_name: str):
        super().__init__(f"Variable '{variable_name}' does not exist", variable_name)


class AgentInputTypeError(AgentNodeError):
    """Exception raised when an unknown agent input type is encountered."""

    def __init__(self, input_type: str):
        super().__init__(f"Unknown agent input type '{input_type}'")


class ToolFileError(AgentNodeError):
    """Exception raised when there's an error with a tool file."""

    def __init__(self, message: str, file_id: str | None = None):
        self.file_id = file_id
        super().__init__(message)


class ToolFileNotFoundError(ToolFileError):
    """Exception raised when a tool file is not found."""

    def __init__(self, file_id: str):
        super().__init__(f"Tool file '{file_id}' does not exist", file_id)


class AgentMessageTransformError(AgentNodeError):
    """Exception raised when there's an error transforming agent messages."""

    def __init__(self, message: str, original_error: Exception | None = None):
        self.original_error = original_error
        super().__init__(message)


class AgentModelError(AgentNodeError):
    """Exception raised when there's an error with the model used by the agent."""

    def __init__(self, message: str, model_name: str | None = None, provider: str | None = None):
        self.model_name = model_name
        self.provider = provider
        super().__init__(message)


class AgentMemoryError(AgentNodeError):
    """Exception raised when there's an error with the agent's memory."""

    def __init__(self, message: str, conversation_id: str | None = None):
        self.conversation_id = conversation_id
        super().__init__(message)


class AgentVariableTypeError(AgentNodeError):
    """Exception raised when a variable has an unexpected type."""

    def __init__(
        self,
        message: str,
        variable_name: str | None = None,
        expected_type: str | None = None,
        actual_type: str | None = None,
    ):
        self.variable_name = variable_name
        self.expected_type = expected_type
        self.actual_type = actual_type
        super().__init__(message)

```

### api/core/workflow/nodes/agent/plugin_strategy_adapter.py
```py
from __future__ import annotations

from factories.agent_factory import get_plugin_agent_strategy

from .strategy_protocols import AgentStrategyPresentationProvider, AgentStrategyResolver, ResolvedAgentStrategy


class PluginAgentStrategyResolver(AgentStrategyResolver):
    def resolve(
        self,
        *,
        tenant_id: str,
        agent_strategy_provider_name: str,
        agent_strategy_name: str,
    ) -> ResolvedAgentStrategy:
        return get_plugin_agent_strategy(
            tenant_id=tenant_id,
            agent_strategy_provider_name=agent_strategy_provider_name,
            agent_strategy_name=agent_strategy_name,
        )


class PluginAgentStrategyPresentationProvider(AgentStrategyPresentationProvider):
    def get_icon(self, *, tenant_id: str, agent_strategy_provider_name: str) -> str | None:
        from core.plugin.impl.plugin import PluginInstaller

        manager = PluginInstaller()
        try:
            plugins = manager.list_plugins(tenant_id)
        except Exception:
            return None

        try:
            current_plugin = next(
                plugin for plugin in plugins if f"{plugin.plugin_id}/{plugin.name}" == agent_strategy_provider_name
            )
        except StopIteration:
            return None

        return current_plugin.declaration.icon

```

### api/core/workflow/nodes/agent/entities.py
```py
from enum import IntEnum, StrEnum, auto
from typing import Any, Literal, Union

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import BuiltinNodeTypes, NodeType
from pydantic import BaseModel

from core.prompt.entities.advanced_prompt_entities import MemoryConfig
from core.tools.entities.tool_entities import ToolSelector


class AgentNodeData(BaseNodeData):
    type: NodeType = BuiltinNodeTypes.AGENT
    agent_strategy_provider_name: str
    agent_strategy_name: str
    agent_strategy_label: str
    memory: MemoryConfig | None = None
    # The version of the tool parameter.
    # If this value is None, it indicates this is a previous version
    # and requires using the legacy parameter parsing rules.
    tool_node_version: str | None = None

    class AgentInput(BaseModel):
        value: Union[list[str], list[ToolSelector], Any]
        type: Literal["mixed", "variable", "constant"]

    agent_parameters: dict[str, AgentInput]


class ParamsAutoGenerated(IntEnum):
    CLOSE = 0
    OPEN = 1


class AgentOldVersionModelFeatures(StrEnum):
    """
    Enum class for old SDK version llm feature.
    """

    TOOL_CALL = "tool-call"
    MULTI_TOOL_CALL = "multi-tool-call"
    AGENT_THOUGHT = "agent-thought"
    VISION = auto()
    STREAM_TOOL_CALL = "stream-tool-call"
    DOCUMENT = auto()
    VIDEO = auto()
    AUDIO = auto()

```

### api/core/workflow/nodes/agent/runtime_support.py
```py
from __future__ import annotations

import json
from collections.abc import Sequence
from typing import Any, cast

from graphon.model_runtime.entities.model_entities import AIModelEntity, ModelType
from graphon.runtime import VariablePool
from packaging.version import Version
from pydantic import ValidationError
from sqlalchemy import select
from sqlalchemy.orm import Session

from core.agent.entities import AgentToolEntity
from core.agent.plugin_entities import AgentStrategyParameter
from core.memory.token_buffer_memory import TokenBufferMemory
from core.model_manager import ModelInstance
from core.plugin.entities.request import InvokeCredentials
from core.plugin.impl.model_runtime_factory import create_plugin_model_assembly
from core.tools.entities.tool_entities import ToolIdentity, ToolParameter, ToolProviderType
from core.tools.tool_manager import ToolManager
from core.workflow.system_variables import SystemVariableKey, get_system_text
from extensions.ext_database import db
from models.model import Conversation

from .entities import AgentNodeData, AgentOldVersionModelFeatures, ParamsAutoGenerated
from .exceptions import AgentInputTypeError, AgentVariableNotFoundError
from .strategy_protocols import ResolvedAgentStrategy


class AgentRuntimeSupport:
    def build_parameters(
        self,
        *,
        agent_parameters: Sequence[AgentStrategyParameter],
        variable_pool: VariablePool,
        node_data: AgentNodeData,
        strategy: ResolvedAgentStrategy,
        tenant_id: str,
        user_id: str,
        app_id: str,
        invoke_from: Any,
        for_log: bool = False,
    ) -> dict[str, Any]:
        agent_parameters_dictionary = {parameter.name: parameter for parameter in agent_parameters}

        result: dict[str, Any] = {}
        for parameter_name in node_data.agent_parameters:
            parameter = agent_parameters_dictionary.get(parameter_name)
            if not parameter:
                result[parameter_name] = None
                continue

            agent_input = node_data.agent_parameters[parameter_name]
            match agent_input.type:
                case "variable":
                    variable = variable_pool.get(agent_input.value)  # type: ignore[arg-type]
                    if variable is None:
                        raise AgentVariableNotFoundError(str(agent_input.value))
                    parameter_value = variable.value
                case "mixed" | "constant":
                    try:
                        if not isinstance(agent_input.value, str):
                            parameter_value = json.dumps(agent_input.value, ensure_ascii=False)
                        else:
                            parameter_value = str(agent_input.value)
                    except TypeError:
                        parameter_value = str(agent_input.value)

                    segment_group = variable_pool.convert_template(parameter_value)
                    parameter_value = segment_group.log if for_log else segment_group.text
                    try:
                        if not isinstance(agent_input.value, str):
                            parameter_value = json.loads(parameter_value)
                    except json.JSONDecodeError:
                        parameter_value = parameter_value
                case _:
                    raise AgentInputTypeError(agent_input.type)

            value = parameter_value
            if parameter.type == "array[tools]":
                value = cast(list[dict[str, Any]], value)
                value = [tool for tool in value if tool.get("enabled", False)]
                value = self._filter_mcp_type_tool(strategy, value)
                for tool in value:
                    if "schemas" in tool:
                        tool.pop("schemas")
                    parameters = tool.get("parameters", {})
                    if all(isinstance(v, dict) for _, v in parameters.items()):
                        params = {}
                        for key, param in parameters.items():
                            if param.get("auto", ParamsAutoGenerated.OPEN) in (
                                ParamsAutoGenerated.CLOSE,
                                0,
                            ):
                                value_param = param.get("value", {})
                                if value_param and value_param.get("type", "") == "variable":
                                    variable_selector = value_param.get("value")
                                    if not variable_selector:
                                        raise ValueError("Variable selector is missing for a variable-type parameter.")

                                    variable = variable_pool.get(variable_selector)
                                    if variable is None:
                                        raise AgentVariableNotFoundError(str(variable_selector))

                                    params[key] = variable.value
                                else:
                                    params[key] = value_param.get("value", "") if value_param is not None else None
                            else:
                                params[key] = None
                        parameters = params
                    tool["settings"] = {k: v.get("value", None) for k, v in tool.get("settings", {}).items()}
                    tool["parameters"] = parameters

            if not for_log:
                if parameter.type == "array[tools]":
                    value = cast(list[dict[str, Any]], value)
                    tool_value = []
                    for tool in value:
                        provider_type = ToolProviderType(tool.get("type", ToolProviderType.BUILT_IN))
                        setting_params = tool.get("settings", {})
                        parameters = tool.get("parameters", {})
                        manual_input_params = [key for key, value in parameters.items() if value is not None]

                        parameters = {**parameters, **setting_params}
                        entity = AgentToolEntity(
                            provider_id=tool.get("provider_name", ""),
                            provider_type=provider_type,
                            tool_name=tool.get("tool_name", ""),
                            tool_parameters=parameters,
                            plugin_unique_identifier=tool.get("plugin_unique_identifier", None),
                            credential_id=tool.get("credential_id", None),
                        )

                        extra = tool.get("extra", {})

                        runtime_variable_pool: VariablePool | None = None
                        if node_data.version != "1" or node_data.tool_node_version is not None:
                            runtime_variable_pool = variable_pool
                        tool_runtime = ToolManager.get_agent_tool_runtime(
                            tenant_id,
                            app_id,
                            entity,
                            user_id,
                            invoke_from,
                            runtime_variable_pool,
                        )
                        if tool_runtime.entity.description:
                            tool_runtime.entity.description.llm = (
                                extra.get("description", "") or tool_runtime.entity.description.llm
                            )
                        for tool_runtime_params in tool_runtime.entity.parameters:
                            tool_runtime_params.form = (
                                ToolParameter.ToolParameterForm.FORM
                                if tool_runtime_params.name in manual_input_params
                                else tool_runtime_params.form
                            )
                        manual_input_value = {}
                        if tool_runtime.entity.parameters:
                            manual_input_value = {
                                key: value for key, value in parameters.items() if key in manual_input_params
                            }
                        runtime_parameters = {
                            **tool_runtime.runtime.runtime_parameters,
                            **manual_input_value,
                        }
                        tool_value.append(
                            {
                                **tool_runtime.entity.model_dump(mode="json"),
                                "runtime_parameters": runtime_parameters,
                                "credential_id": tool.get("credential_id", None),
                                "provider_type": provider_type.value,
                            }
                        )
                    value = tool_value
                if parameter.type == AgentStrategyParameter.AgentStrategyParameterType.MODEL_SELECTOR:
                    value = cast(dict[str, Any], value)
                    model_instance, model_schema = self.fetch_model(
                        tenant_id=tenant_id,
                        user_id=user_id,
                        value=value,
                    )
                    history_prompt_messages = []
                    if node_data.memory:
                        memory = self.fetch_memory(
                            variable_pool=variable_pool,
                            app_id=app_id,
                            model_instance=model_instance,
                        )
                        if memory:
                            prompt_messages = memory.get_history_prompt_messages(
                                message_limit=node_data.memory.window.size or None
                            )
                            history_prompt_messages = [
                                prompt_message.model_dump(mode="json") for prompt_message in prompt_messages
                            ]
                    value["history_prompt_messages"] = history_prompt_messages
                    if model_schema:
                        model_schema = self._remove_unsupported_model_features_for_old_version(model_schema)
                        value["entity"] = model_schema.model_dump(mode="json")
                    else:
                        value["entity"] = None
            result[parameter_name] = value

        return result

    def build_credentials(self, *, parameters: dict[str, Any]) -> InvokeCredentials:
        credentials = InvokeCredentials()
        credentials.tool_credentials = {}
        for tool in parameters.get("tools", []):
            if not tool.get("credential_id"):
                continue
            try:
                identity = ToolIdentity.model_validate(tool.get("identity", {}))
            except ValidationError:
                continue
            credentials.tool_credentials[identity.provider] = tool.get("credential_id", None)
        return credentials

    def fetch_memory(
        self,
        *,
        variable_pool: VariablePool,
        app_id: str,
        model_instance: ModelInstance,
    ) -> TokenBufferMemory | None:
        conversation_id = get_system_text(variable_pool, SystemVariableKey.CONVERSATION_ID)
        if conversation_id is None:
            return None

        with Session(db.engine, expire_on_commit=False) as session:
            stmt = select(Conversation).where(Conversation.app_id == app_id, Conversation.id == conversation_id)
            conversation = session.scalar(stmt)
            if not conversation:
                return None

        return TokenBufferMemory(conversation=conversation, model_instance=model_instance)

    def fetch_model(
        self,
        *,
        tenant_id: str,
        user_id: str,
        value: dict[str, Any],
    ) -> tuple[ModelInstance, AIModelEntity | None]:
        assembly = create_plugin_model_assembly(tenant_id=tenant_id, user_id=user_id)
        provider_model_bundle = assembly.provider_manager.get_provider_model_bundle(
            tenant_id=tenant_id,
            provider=value.get("provider", ""),
            model_type=ModelType.LLM,
        )
        model_name = value.get("model", "")
        model_credentials = provider_model_bundle.configuration.get_current_credentials(
            model_type=ModelType.LLM,
            model=model_name,
        )
        provider_name = provider_model_bundle.configuration.provider.provider
        model_type_instance = provider_model_bundle.model_type_instance
        model_instance = assembly.model_manager.get_model_instance(
            tenant_id=tenant_id,
            provider=provider_name,
            model_type=ModelType(value.get("model_type", "")),
            model=model_name,
        )
        model_schema = model_type_instance.get_model_schema(model_name, model_credentials)
        return model_instance, model_schema

    @staticmethod
    def _remove_unsupported_model_features_for_old_version(model_schema: AIModelEntity) -> AIModelEntity:
        if model_schema.features:
            for feature in model_schema.features[:]:
                try:
                    AgentOldVersionModelFeatures(feature.value)
                except ValueError:
                    model_schema.features.remove(feature)
        return model_schema

    @staticmethod
    def _filter_mcp_type_tool(
        strategy: ResolvedAgentStrategy,
        tools: list[dict[str, Any]],
    ) -> list[dict[str, Any]]:
        meta_version = strategy.meta_version
        if meta_version and Version(meta_version) > Version("0.0.1"):
            return tools
        return [tool for tool in tools if tool.get("type") != ToolProviderType.MCP]

```

### api/core/workflow/nodes/knowledge_retrieval/__init__.py
```py
"""Knowledge retrieval workflow node package."""

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

### api/core/workflow/nodes/knowledge_retrieval/exc.py
```py
class KnowledgeRetrievalNodeError(ValueError):
    """Base class for KnowledgeRetrievalNode errors."""


class ModelNotExistError(KnowledgeRetrievalNodeError):
    """Raised when the model does not exist."""


class ModelCredentialsNotInitializedError(KnowledgeRetrievalNodeError):
    """Raised when the model credentials are not initialized."""


class ModelNotSupportedError(KnowledgeRetrievalNodeError):
    """Raised when the model is not supported."""


class ModelQuotaExceededError(KnowledgeRetrievalNodeError):
    """Raised when the model provider quota is exceeded."""


class InvalidModelTypeError(KnowledgeRetrievalNodeError):
    """Raised when the model is not a Large Language Model."""


class RateLimitExceededError(KnowledgeRetrievalNodeError):
    """Raised when the rate limit is exceeded."""

```

### api/core/workflow/nodes/knowledge_retrieval/retrieval.py
```py
from typing import Any, Literal, Protocol

from graphon.model_runtime.entities import LLMUsage
from graphon.nodes.llm.entities import ModelConfig
from pydantic import BaseModel, Field

from core.rag.data_post_processor.data_post_processor import RerankingModelDict, WeightsDict

from .entities import MetadataFilteringCondition


class SourceChildChunk(BaseModel):
    id: str = Field(default="", description="Child chunk ID")
    content: str = Field(default="", description="Child chunk content")
    position: int = Field(default=0, description="Child chunk position")
    score: float = Field(default=0.0, description="Child chunk relevance score")


class SourceMetadata(BaseModel):
    source: str = Field(
        default="knowledge",
        serialization_alias="_source",
        description="Data source identifier",
    )
    dataset_id: str = Field(description="Dataset unique identifier")
    dataset_name: str = Field(description="Dataset display name")
    document_id: str = Field(description="Document unique identifier")
    document_name: str = Field(description="Document display name")
    data_source_type: str = Field(description="Type of data source")
    segment_id: str | None = Field(default=None, description="Segment unique identifier")
    retriever_from: str = Field(default="workflow", description="Retriever source context")
    score: float = Field(default=0.0, description="Retrieval relevance score")
    child_chunks: list[SourceChildChunk] = Field(default_factory=list, description="List of child chunks")
    segment_hit_count: int | None = Field(default=0, description="Number of times segment was retrieved")
    segment_word_count: int | None = Field(default=0, description="Word count of the segment")
    segment_position: int | None = Field(default=0, description="Position of segment in document")
    segment_index_node_hash: str | None = Field(default=None, description="Hash of index node for the segment")
    doc_metadata: dict[str, Any] | None = Field(default=None, description="Additional document metadata")
    position: int | None = Field(default=0, description="Position of the document in the dataset")

    class Config:
        populate_by_name = True


class Source(BaseModel):
    metadata: SourceMetadata = Field(description="Source metadata information")
    title: str = Field(description="Document title")
    files: list[Any] | None = Field(default=None, description="Associated file references")
    content: str | None = Field(description="Segment content text")
    summary: str | None = Field(default=None, description="Content summary if available")


class KnowledgeRetrievalRequest(BaseModel):
    tenant_id: str = Field(description="Tenant unique identifier")
    user_id: str = Field(description="User unique identifier")
    app_id: str = Field(description="Application unique identifier")
    user_from: str = Field(description="User identity source for audit logging (e.g., 'account', 'end-user')")
    dataset_ids: list[str] = Field(description="List of dataset IDs to retrieve from")
    query: str | None = Field(default=None, description="Query text for knowledge retrieval")
    retrieval_mode: str = Field(description="Retrieval strategy: 'single' or 'multiple'")
    model_provider: str | None = Field(default=None, description="Model provider name (e.g., 'openai', 'anthropic')")
    completion_params: dict[str, Any] | None = Field(
        default=None, description="Model completion parameters (e.g., temperature, max_tokens)"
    )
    model_mode: str | None = Field(default=None, description="Model mode (e.g., 'chat', 'completion')")
    model_name: str | None = Field(default=None, description="Model name (e.g., 'gpt-4', 'claude-3-opus')")
    metadata_model_config: ModelConfig | None = Field(
        default=None, description="Model config for metadata-based filtering"
    )
    metadata_filtering_conditions: MetadataFilteringCondition | None = Field(
        default=None, description="Conditions for filtering by metadata"
    )
    metadata_filtering_mode: Literal["disabled", "automatic", "manual"] = Field(
        default="disabled", description="Metadata filtering mode: 'disabled', 'automatic', or 'manual'"
    )
    top_k: int = Field(default=0, description="Number of top results to return")
    score_threshold: float = Field(default=0.0, description="Minimum relevance score threshold")
    reranking_mode: str = Field(default="reranking_model", description="Reranking strategy")
    reranking_model: RerankingModelDict | None = Field(default=None, description="Reranking model configuration")
    weights: WeightsDict | None = Field(default=None, description="Weights for weighted score reranking")
    reranking_enable: bool = Field(default=True, description="Whether reranking is enabled")
    attachment_ids: list[str] | None = Field(default=None, description="List of attachment file IDs for retrieval")


class RAGRetrievalProtocol(Protocol):
    @property
    def llm_usage(self) -> LLMUsage: ...

    def knowledge_retrieval(self, request: KnowledgeRetrievalRequest) -> list[Source]: ...

```

### api/core/workflow/nodes/knowledge_retrieval/entities.py
```py
from collections.abc import Sequence
from typing import Literal

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import BuiltinNodeTypes, NodeType
from graphon.nodes.llm.entities import ModelConfig, VisionConfig
from pydantic import BaseModel, Field


class RerankingModelConfig(BaseModel):
    """
    Reranking Model Config.
    """

    provider: str
    model: str


class VectorSetting(BaseModel):
    """
    Vector Setting.
    """

    vector_weight: float
    embedding_provider_name: str
    embedding_model_name: str


class KeywordSetting(BaseModel):
    """
    Keyword Setting.
    """

    keyword_weight: float


class WeightedScoreConfig(BaseModel):
    """
    Weighted score Config.
    """

    vector_setting: VectorSetting
    keyword_setting: KeywordSetting


class MultipleRetrievalConfig(BaseModel):
    """
    Multiple Retrieval Config.
    """

    top_k: int
    score_threshold: float | None = None
    reranking_mode: str = "reranking_model"
    reranking_enable: bool = True
    reranking_model: RerankingModelConfig | None = None
    weights: WeightedScoreConfig | None = None


class SingleRetrievalConfig(BaseModel):
    """
    Single Retrieval Config.
    """

    model: ModelConfig


SupportedComparisonOperator = Literal[
    # for string or array
    "contains",
    "not contains",
    "start with",
    "end with",
    "is",
    "is not",
    "empty",
    "not empty",
    "in",
    "not in",
    # for number
    "=",
    "≠",
    ">",
    "<",
    "≥",
    "≤",
    # for time
    "before",
    "after",
]


class Condition(BaseModel):
    """
    Condition detail
    """

    name: str
    comparison_operator: SupportedComparisonOperator
    value: str | Sequence[str] | None | int | float = None


class MetadataFilteringCondition(BaseModel):
    """
    Metadata Filtering Condition.
    """

    logical_operator: Literal["and", "or"] | None = "and"
    conditions: list[Condition] | None = Field(default=None, deprecated=True)


class KnowledgeRetrievalNodeData(BaseNodeData):
    """
    Knowledge retrieval Node Data.
    """

    type: NodeType = BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL
    query_variable_selector: list[str] | None | str = None
    query_attachment_selector: list[str] | None | str = None
    dataset_ids: list[str]
    retrieval_mode: Literal["single", "multiple"]
    multiple_retrieval_config: MultipleRetrievalConfig | None = None
    single_retrieval_config: SingleRetrievalConfig | None = None
    metadata_filtering_mode: Literal["disabled", "automatic", "manual"] | None = "disabled"
    metadata_model_config: ModelConfig | None = None
    metadata_filtering_conditions: MetadataFilteringCondition | None = None
    vision: VisionConfig = Field(default_factory=VisionConfig)

    @property
    def structured_output_enabled(self) -> bool:
        # NOTE(QuantumGhost): Temporary workaround for issue #20725
        # (https://github.com/langgenius/dify/issues/20725).
        #
        # The proper fix would be to make `KnowledgeRetrievalNode` inherit
        # from `BaseNode` instead of `LLMNode`.
        return False

```

### api/core/workflow/nodes/knowledge_retrieval/template_prompts.py
```py
METADATA_FILTER_SYSTEM_PROMPT = """
    ### Job Description',
    You are a text metadata extract engine that extract text's metadata based on user input and set the metadata value
    ### Task
    Your task is to ONLY extract the metadatas that exist in the input text from the provided metadata list and Use the following operators ["contains", "not contains", "start with", "end with", "is", "is not", "empty", "not empty", "=", "≠", ">", "<", "≥", "≤", "before", "after"] to express logical relationships, then return result in JSON format with the key "metadata_fields" and value "metadata_field_value" and comparison operator "comparison_operator".
    ### Format
    The input text is in the variable input_text. Metadata are specified as a list in the variable metadata_fields.
    ### Constraint
    DO NOT include anything other than the JSON array in your response.
"""  # noqa: E501

METADATA_FILTER_USER_PROMPT_1 = """
    { "input_text": "I want to know which company’s email address test@example.com is?",
    "metadata_fields": ["filename", "email", "phone", "address"]
    }
"""

METADATA_FILTER_ASSISTANT_PROMPT_1 = """
```json
    {"metadata_map": [
        {"metadata_field_name": "email", "metadata_field_value": "test@example.com", "comparison_operator": "="}
    ]
    }
```
"""

METADATA_FILTER_USER_PROMPT_2 = """
    {"input_text": "What are the movies with a score of more than 9 in 2024?",
    "metadata_fields": ["name", "year", "rating", "country"]}
"""

METADATA_FILTER_ASSISTANT_PROMPT_2 = """
```json
    {"metadata_map": [
        {"metadata_field_name": "year", "metadata_field_value": "2024", "comparison_operator": "="},
        {"metadata_field_name": "rating", "metadata_field_value": "9", "comparison_operator": ">"},
    ]}
```
"""

METADATA_FILTER_USER_PROMPT_3 = """
    '{{"input_text": "{input_text}",',
    '"metadata_fields": {metadata_fields}}}'
"""

METADATA_FILTER_COMPLETION_PROMPT = """
### Job Description
You are a text metadata extract engine that extract text's metadata based on user input and set the metadata value
### Task
# Your task is to ONLY extract the metadatas that exist in the input text from the provided metadata list and Use the following operators ["=", "!=", ">", "<", ">=", "<="] to express logical relationships, then return result in JSON format with the key "metadata_fields" and value "metadata_field_value" and comparison operator "comparison_operator".
### Format
The input text is in the variable input_text. Metadata are specified as a list in the variable metadata_fields.
### Constraint
DO NOT include anything other than the JSON array in your response.
### Example
Here is the chat example between human and assistant, inside <example></example> XML tags.
<example>
User:{{"input_text": ["I want to know which company’s email address test@example.com is?"], "metadata_fields": ["filename", "email", "phone", "address"]}}
Assistant:{{"metadata_map": [{{"metadata_field_name": "email", "metadata_field_value": "test@example.com", "comparison_operator": "="}}]}}
User:{{"input_text": "What are the movies with a score of more than 9 in 2024?", "metadata_fields": ["name", "year", "rating", "country"]}}
Assistant:{{"metadata_map": [{{"metadata_field_name": "year", "metadata_field_value": "2024", "comparison_operator": "="}, {{"metadata_field_name": "rating", "metadata_field_value": "9", "comparison_operator": ">"}}]}}
</example>
### User Input
{{"input_text" : "{input_text}", "metadata_fields" : {metadata_fields}}}
### Assistant Output
"""  # noqa: E501

```

### api/core/workflow/nodes/trigger_webhook/__init__.py
```py
from .node import TriggerWebhookNode

__all__ = ["TriggerWebhookNode"]

```

### api/core/workflow/nodes/trigger_webhook/exc.py
```py
from graphon.entities.exc import BaseNodeError


class WebhookNodeError(BaseNodeError):
    """Base webhook node error."""

    pass


class WebhookTimeoutError(WebhookNodeError):
    """Webhook timeout error."""

    pass


class WebhookNotFoundError(WebhookNodeError):
    """Webhook not found error."""

    pass


class WebhookConfigError(WebhookNodeError):
    """Webhook configuration error."""

    pass

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

### api/core/workflow/nodes/trigger_webhook/entities.py
```py
from collections.abc import Sequence
from enum import StrEnum

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import NodeType
from graphon.variables.types import SegmentType
from pydantic import BaseModel, Field, field_validator

from core.trigger.constants import TRIGGER_WEBHOOK_NODE_TYPE

_WEBHOOK_HEADER_ALLOWED_TYPES: frozenset[SegmentType] = frozenset((SegmentType.STRING,))

_WEBHOOK_QUERY_PARAMETER_ALLOWED_TYPES: frozenset[SegmentType] = frozenset(
    (
        SegmentType.STRING,
        SegmentType.NUMBER,
        SegmentType.BOOLEAN,
    )
)

_WEBHOOK_PARAMETER_ALLOWED_TYPES = _WEBHOOK_HEADER_ALLOWED_TYPES | _WEBHOOK_QUERY_PARAMETER_ALLOWED_TYPES

_WEBHOOK_BODY_ALLOWED_TYPES: frozenset[SegmentType] = frozenset(
    (
        SegmentType.STRING,
        SegmentType.NUMBER,
        SegmentType.BOOLEAN,
        SegmentType.OBJECT,
        SegmentType.ARRAY_STRING,
        SegmentType.ARRAY_NUMBER,
        SegmentType.ARRAY_BOOLEAN,
        SegmentType.ARRAY_OBJECT,
        SegmentType.FILE,
    )
)


class Method(StrEnum):
    GET = "get"
    POST = "post"
    HEAD = "head"
    PATCH = "patch"
    PUT = "put"
    DELETE = "delete"


class ContentType(StrEnum):
    JSON = "application/json"
    FORM_DATA = "multipart/form-data"
    FORM_URLENCODED = "application/x-www-form-urlencoded"
    TEXT = "text/plain"
    BINARY = "application/octet-stream"


class WebhookParameter(BaseModel):
    """Parameter definition for headers or query params."""

    name: str
    type: SegmentType = SegmentType.STRING
    required: bool = False

    @field_validator("type", mode="after")
    @classmethod
    def validate_type(cls, v: SegmentType) -> SegmentType:
        if v not in _WEBHOOK_PARAMETER_ALLOWED_TYPES:
            raise ValueError(f"Unsupported webhook parameter type: {v}")
        return v


class WebhookBodyParameter(BaseModel):
    """Body parameter with type information."""

    name: str
    type: SegmentType = SegmentType.STRING
    required: bool = False

    @field_validator("type", mode="after")
    @classmethod
    def validate_type(cls, v: SegmentType) -> SegmentType:
        if v not in _WEBHOOK_BODY_ALLOWED_TYPES:
            raise ValueError(f"Unsupported webhook body parameter type: {v}")
        return v


class WebhookData(BaseNodeData):
    """
    Webhook Node Data.
    """

    class SyncMode(StrEnum):
        SYNC = "async"  # only support

    type: NodeType = TRIGGER_WEBHOOK_NODE_TYPE
    method: Method = Method.GET
    content_type: ContentType = Field(default=ContentType.JSON)
    headers: Sequence[WebhookParameter] = Field(default_factory=list)
    params: Sequence[WebhookParameter] = Field(default_factory=list)  # query parameters
    body: Sequence[WebhookBodyParameter] = Field(default_factory=list)

    @field_validator("method", mode="before")
    @classmethod
    def normalize_method(cls, v) -> str:
        """Normalize HTTP method to lowercase to support both uppercase and lowercase input."""
        if isinstance(v, str):
            return v.lower()
        return v

    @field_validator("headers", mode="after")
    @classmethod
    def validate_header_types(cls, v: Sequence[WebhookParameter]) -> Sequence[WebhookParameter]:
        for param in v:
            if param.type not in _WEBHOOK_HEADER_ALLOWED_TYPES:
                raise ValueError(f"Unsupported webhook header parameter type: {param.type}")
        return v

    @field_validator("params", mode="after")
    @classmethod
    def validate_query_parameter_types(cls, v: Sequence[WebhookParameter]) -> Sequence[WebhookParameter]:
        for param in v:
            if param.type not in _WEBHOOK_QUERY_PARAMETER_ALLOWED_TYPES:
                raise ValueError(f"Unsupported webhook query parameter type: {param.type}")
        return v

    status_code: int = 200  # Expected status code for response
    response_body: str = ""  # Template for response body

    # Webhook specific fields (not from client data, set internally)
    webhook_id: str | None = None  # Set when webhook trigger is created
    timeout: int = 30  # Timeout in seconds to wait for webhook response

```

### api/core/workflow/nodes/knowledge_index/protocols.py
```py
from collections.abc import Mapping
from typing import Any, Protocol

from pydantic import BaseModel, Field


class PreviewItem(BaseModel):
    content: str | None = Field(default=None)
    child_chunks: list[str] | None = Field(default=None)
    summary: str | None = Field(default=None)


class QaPreview(BaseModel):
    answer: str | None = Field(default=None)
    question: str | None = Field(default=None)


class Preview(BaseModel):
    chunk_structure: str
    parent_mode: str | None = Field(default=None)
    preview: list[PreviewItem] = Field(default_factory=list)
    qa_preview: list[QaPreview] = Field(default_factory=list)
    total_segments: int


class IndexProcessorProtocol(Protocol):
    def format_preview(self, chunk_structure: str, chunks: Any) -> Preview: ...

    def index_and_clean(
        self,
        dataset_id: str,
        document_id: str,
        original_document_id: str,
        chunks: Mapping[str, Any],
        batch: Any,
        summary_index_setting: dict | None = None,
    ) -> dict[str, Any]: ...

    def get_preview_output(
        self, chunks: Any, dataset_id: str, document_id: str, chunk_structure: str, summary_index_setting: dict | None
    ) -> Preview: ...


class SummaryIndexServiceProtocol(Protocol):
    def generate_and_vectorize_summary(
        self, dataset_id: str, document_id: str, is_preview: bool, summary_index_setting: dict | None = None
    ) -> None: ...

```

### api/core/workflow/nodes/knowledge_index/__init__.py
```py
"""Knowledge index workflow node package."""

KNOWLEDGE_INDEX_NODE_TYPE = "knowledge-index"

__all__ = ["KNOWLEDGE_INDEX_NODE_TYPE"]

```

### api/core/workflow/nodes/knowledge_index/knowledge_index_node.py
```py
import logging
from collections.abc import Mapping
from typing import TYPE_CHECKING, Any

from graphon.entities.graph_config import NodeConfigDict
from graphon.enums import NodeExecutionType, WorkflowNodeExecutionStatus
from graphon.node_events import NodeRunResult
from graphon.nodes.base.node import Node
from graphon.nodes.base.template import Template

from core.rag.index_processor.index_processor import IndexProcessor
from core.rag.index_processor.index_processor_base import SummaryIndexSettingDict
from core.rag.summary_index.summary_index import SummaryIndex
from core.workflow.nodes.knowledge_index import KNOWLEDGE_INDEX_NODE_TYPE
from core.workflow.system_variables import SystemVariableKey, get_system_segment, get_system_text

from .entities import KnowledgeIndexNodeData
from .exc import (
    KnowledgeIndexNodeError,
)

if TYPE_CHECKING:
    from graphon.entities import GraphInitParams
    from graphon.runtime import GraphRuntimeState

logger = logging.getLogger(__name__)
_INVOKE_FROM_DEBUGGER = "debugger"


class KnowledgeIndexNode(Node[KnowledgeIndexNodeData]):
    node_type = KNOWLEDGE_INDEX_NODE_TYPE
    execution_type = NodeExecutionType.RESPONSE

    def __init__(
        self,
        id: str,
        config: NodeConfigDict,
        graph_init_params: "GraphInitParams",
        graph_runtime_state: "GraphRuntimeState",
    ) -> None:
        super().__init__(id, config, graph_init_params, graph_runtime_state)
        self.index_processor = IndexProcessor()
        self.summary_index_service = SummaryIndex()

    def _run(self) -> NodeRunResult:  # type: ignore
        node_data = self.node_data
        variable_pool = self.graph_runtime_state.variable_pool

        # get dataset id as string
        dataset_id_segment = get_system_segment(variable_pool, SystemVariableKey.DATASET_ID)
        if not dataset_id_segment:
            raise KnowledgeIndexNodeError("Dataset ID is required.")
        dataset_id: str = dataset_id_segment.value

        # get document id as string (may be empty when not provided)
        document_id_segment = get_system_segment(variable_pool, SystemVariableKey.DOCUMENT_ID)
        document_id: str = document_id_segment.value if document_id_segment else ""

        # extract variables
        variable = variable_pool.get(node_data.index_chunk_variable_selector)
        if not variable:
            raise KnowledgeIndexNodeError("Index chunk variable is required.")
        invoke_from_value = get_system_text(variable_pool, SystemVariableKey.INVOKE_FROM)
        is_preview = invoke_from_value == _INVOKE_FROM_DEBUGGER

        chunks = variable.value
        variables = {"chunks": chunks}
        if not chunks:
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED, inputs=variables, error="Chunks is required."
            )

        try:
            summary_index_setting = node_data.summary_index_setting
            if is_preview:
                # Preview mode: generate summaries for chunks directly without saving to database
                # Format preview and generate summaries on-the-fly
                # Get indexing_technique and summary_index_setting from node_data (workflow graph config)
                # or fallback to dataset if not available in node_data

                outputs = self.index_processor.get_preview_output(
                    chunks, dataset_id, document_id, node_data.chunk_structure, summary_index_setting
                )
                return NodeRunResult(
                    status=WorkflowNodeExecutionStatus.SUCCEEDED,
                    inputs=variables,
                    outputs=outputs.model_dump(exclude_none=True),
                )

            original_document_id_segment = get_system_segment(variable_pool, SystemVariableKey.ORIGINAL_DOCUMENT_ID)
            batch = get_system_segment(variable_pool, SystemVariableKey.BATCH)
            if not batch:
                raise KnowledgeIndexNodeError("Batch is required.")

            results = self._invoke_knowledge_index(
                dataset_id=dataset_id,
                document_id=document_id,
                original_document_id=original_document_id_segment.value if original_document_id_segment else "",
                is_preview=is_preview,
                batch=batch.value,
                chunks=chunks,
                summary_index_setting=summary_index_setting,
            )
            return NodeRunResult(status=WorkflowNodeExecutionStatus.SUCCEEDED, inputs=variables, outputs=results)

        except KnowledgeIndexNodeError as e:
            logger.warning("Error when running knowledge index node", exc_info=True)
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED,
                inputs=variables,
                error=str(e),
                error_type=type(e).__name__,
            )
        except Exception as e:
            logger.error(e, exc_info=True)
            return NodeRunResult(
                status=WorkflowNodeExecutionStatus.FAILED,
                inputs=variables,
                error=str(e),
                error_type=type(e).__name__,
            )

    def _invoke_knowledge_index(
        self,
        dataset_id: str,
        document_id: str,
        original_document_id: str,
        is_preview: bool,
        batch: Any,
        chunks: Mapping[str, Any],
        summary_index_setting: SummaryIndexSettingDict | None = None,
    ):
        if not document_id:
            raise KnowledgeIndexNodeError("document_id is required.")
        rst = self.index_processor.index_and_clean(
            dataset_id, document_id, original_document_id, chunks, batch, summary_index_setting
        )
        self.summary_index_service.generate_and_vectorize_summary(
            dataset_id, document_id, is_preview, summary_index_setting
        )
        return rst

    @classmethod
    def version(cls) -> str:
        return "1"

    def get_streaming_template(self) -> Template:
        """
        Get the template for streaming.

        Returns:
            Template instance for this knowledge index node
        """
        return Template(segments=[])

```

### api/core/workflow/nodes/knowledge_index/exc.py
```py
class KnowledgeIndexNodeError(ValueError):
    """Base class for KnowledgeIndexNode errors."""


class ModelNotExistError(KnowledgeIndexNodeError):
    """Raised when the model does not exist."""


class ModelCredentialsNotInitializedError(KnowledgeIndexNodeError):
    """Raised when the model credentials are not initialized."""


class ModelNotSupportedError(KnowledgeIndexNodeError):
    """Raised when the model is not supported."""


class ModelQuotaExceededError(KnowledgeIndexNodeError):
    """Raised when the model provider quota is exceeded."""


class InvalidModelTypeError(KnowledgeIndexNodeError):
    """Raised when the model is not a Large Language Model."""

```

### api/core/workflow/nodes/knowledge_index/entities.py
```py
from typing import Literal, Union

from graphon.entities.base_node_data import BaseNodeData
from graphon.enums import NodeType
from pydantic import BaseModel

from core.rag.index_processor.index_processor_base import SummaryIndexSettingDict
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.workflow.nodes.knowledge_index import KNOWLEDGE_INDEX_NODE_TYPE


class RerankingModelConfig(BaseModel):
    """
    Reranking Model Config.
    """

    reranking_provider_name: str
    reranking_model_name: str


class VectorSetting(BaseModel):
    """
    Vector Setting.
    """

    vector_weight: float
    embedding_provider_name: str
    embedding_model_name: str


class KeywordSetting(BaseModel):
    """
    Keyword Setting.
    """

    keyword_weight: float


class WeightedScoreConfig(BaseModel):
    """
    Weighted score Config.
    """

    vector_setting: VectorSetting
    keyword_setting: KeywordSetting


class EmbeddingSetting(BaseModel):
    """
    Embedding Setting.
    """

    embedding_provider_name: str
    embedding_model_name: str


class EconomySetting(BaseModel):
    """
    Economy Setting.
    """

    keyword_number: int


class RetrievalSetting(BaseModel):
    """
    Retrieval Setting.
    """

    search_method: RetrievalMethod
    top_k: int
    score_threshold: float | None = 0.5
    score_threshold_enabled: bool = False
    reranking_mode: str = "reranking_model"
    reranking_enable: bool = True
    reranking_model: RerankingModelConfig | None = None
    weights: WeightedScoreConfig | None = None


class IndexMethod(BaseModel):
    """
    Knowledge Index Setting.
    """

    indexing_technique: Literal["high_quality", "economy"]
    embedding_setting: EmbeddingSetting
    economy_setting: EconomySetting


class FileInfo(BaseModel):
    """
    File Info.
    """

    file_id: str


class OnlineDocumentIcon(BaseModel):
    """
    Document Icon.
    """

    icon_url: str
    icon_type: str
    icon_emoji: str


class OnlineDocumentInfo(BaseModel):
    """
    Online document info.
    """

    provider: str
    workspace_id: str | None = None
    page_id: str
    page_type: str
    icon: OnlineDocumentIcon | None = None


class WebsiteInfo(BaseModel):
    """
    website import info.
    """

    provider: str
    url: str


class GeneralStructureChunk(BaseModel):
    """
    General Structure Chunk.
    """

    general_chunks: list[str]
    data_source_info: Union[FileInfo, OnlineDocumentInfo, WebsiteInfo]


class ParentChildChunk(BaseModel):
    """
    Parent Child Chunk.
    """

    parent_content: str
    child_contents: list[str]


class ParentChildStructureChunk(BaseModel):
    """
    Parent Child Structure Chunk.
    """

    parent_child_chunks: list[ParentChildChunk]
    data_source_info: Union[FileInfo, OnlineDocumentInfo, WebsiteInfo]


class KnowledgeIndexNodeData(BaseNodeData):
    """
    Knowledge index Node Data.
    """

    type: NodeType = KNOWLEDGE_INDEX_NODE_TYPE
    chunk_structure: str
    index_chunk_variable_selector: list[str]
    indexing_technique: str | None = None
    summary_index_setting: SummaryIndexSettingDict | None = None

```

### api/core/app/workflow/layers/persistence.py
```py
"""Workflow persistence layer for GraphEngine.

This layer mirrors the former ``WorkflowCycleManager`` responsibilities by
listening to ``GraphEngineEvent`` instances directly and persisting workflow
and node execution state via the injected repositories.

The design keeps domain persistence concerns inside the engine thread, while
allowing presentation layers to remain read-only observers of repository
state.
"""

from collections.abc import Mapping
from dataclasses import dataclass
from datetime import datetime
from typing import Any, Union

from graphon.entities import WorkflowExecution, WorkflowNodeExecution
from graphon.enums import (
    WorkflowExecutionStatus,
    WorkflowNodeExecutionMetadataKey,
    WorkflowNodeExecutionStatus,
    WorkflowType,
)
from graphon.graph_engine.layers import GraphEngineLayer
from graphon.graph_events import (
    GraphEngineEvent,
    GraphRunAbortedEvent,
    GraphRunFailedEvent,
    GraphRunPartialSucceededEvent,
    GraphRunPausedEvent,
    GraphRunStartedEvent,
    GraphRunSucceededEvent,
    NodeRunExceptionEvent,
    NodeRunFailedEvent,
    NodeRunPauseRequestedEvent,
    NodeRunRetryEvent,
    NodeRunStartedEvent,
    NodeRunSucceededEvent,
)
from graphon.node_events import NodeRunResult

from core.app.entities.app_invoke_entities import AdvancedChatAppGenerateEntity, WorkflowAppGenerateEntity
from core.ops.entities.trace_entity import TraceTaskName
from core.ops.ops_trace_manager import TraceQueueManager, TraceTask
from core.repositories.factory import WorkflowExecutionRepository, WorkflowNodeExecutionRepository
from core.workflow.system_variables import SystemVariableKey
from core.workflow.variable_prefixes import SYSTEM_VARIABLE_NODE_ID
from core.workflow.workflow_run_outputs import project_node_outputs_for_workflow_run
from libs.datetime_utils import naive_utc_now


@dataclass(slots=True)
class PersistenceWorkflowInfo:
    """Static workflow metadata required for persistence."""

    workflow_id: str
    workflow_type: WorkflowType
    version: str
    graph_data: Mapping[str, Any]


@dataclass(slots=True)
class _NodeRuntimeSnapshot:
    """Lightweight cache to keep node metadata across event phases."""

    node_id: str
    title: str
    predecessor_node_id: str | None
    iteration_id: str | None
    loop_id: str | None
    created_at: datetime


class WorkflowPersistenceLayer(GraphEngineLayer):
    """GraphEngine layer that persists workflow and node execution state."""

    def __init__(
        self,
        *,
        application_generate_entity: Union[AdvancedChatAppGenerateEntity, WorkflowAppGenerateEntity],
        workflow_info: PersistenceWorkflowInfo,
        workflow_execution_repository: WorkflowExecutionRepository,
        workflow_node_execution_repository: WorkflowNodeExecutionRepository,
        trace_manager: TraceQueueManager | None = None,
    ) -> None:
        super().__init__()
        self._application_generate_entity = application_generate_entity
        self._workflow_info = workflow_info
        self._workflow_execution_repository = workflow_execution_repository
        self._workflow_node_execution_repository = workflow_node_execution_repository
        self._trace_manager = trace_manager

        self._workflow_execution: WorkflowExecution | None = None
        self._node_execution_cache: dict[str, WorkflowNodeExecution] = {}
        self._node_snapshots: dict[str, _NodeRuntimeSnapshot] = {}
        self._node_sequence: int = 0

    # ------------------------------------------------------------------
    # GraphEngineLayer lifecycle
    # ------------------------------------------------------------------
    def on_graph_start(self) -> None:
        self._workflow_execution = None
        self._node_execution_cache.clear()
        self._node_snapshots.clear()
        self._node_sequence = 0

    def on_event(self, event: GraphEngineEvent) -> None:
        if isinstance(event, GraphRunStartedEvent):
            self._handle_graph_run_started()
            return

        if isinstance(event, GraphRunSucceededEvent):
            self._handle_graph_run_succeeded(event)
            return

        if isinstance(event, GraphRunPartialSucceededEvent):
            self._handle_graph_run_partial_succeeded(event)
            return

        if isinstance(event, GraphRunFailedEvent):
            self._handle_graph_run_failed(event)
            return

        if isinstance(event, GraphRunAbortedEvent):
            self._handle_graph_run_aborted(event)
            return

        if isinstance(event, GraphRunPausedEvent):
            self._handle_graph_run_paused(event)
            return

        if isinstance(event, NodeRunRetryEvent):
            self._handle_node_retry(event)
            return

        if isinstance(event, NodeRunStartedEvent):
            self._handle_node_started(event)
            return

        if isinstance(event, NodeRunSucceededEvent):
            self._handle_node_succeeded(event)
            return

        if isinstance(event, NodeRunFailedEvent):
            self._handle_node_failed(event)
            return

        if isinstance(event, NodeRunExceptionEvent):
            self._handle_node_exception(event)
            return

        if isinstance(event, NodeRunPauseRequestedEvent):
            self._handle_node_pause_requested(event)

    def on_graph_end(self, error: Exception | None) -> None:
        return

    # ------------------------------------------------------------------
    # Graph-level handlers
    # ------------------------------------------------------------------
    def _handle_graph_run_started(self) -> None:
        execution_id = self._get_execution_id()
        workflow_execution = WorkflowExecution.new(
            id_=execution_id,
            workflow_id=self._workflow_info.workflow_id,
            workflow_type=self._workflow_info.workflow_type,
            workflow_version=self._workflow_info.version,
            graph=self._workflow_info.graph_data,
            inputs=self._prepare_workflow_inputs(),
            started_at=naive_utc_now(),
        )

        self._workflow_execution_repository.save(workflow_execution)
        self._workflow_execution = workflow_execution

    def _handle_graph_run_succeeded(self, event: GraphRunSucceededEvent) -> None:
        execution = self._get_workflow_execution()
        execution.outputs = event.outputs
        execution.status = WorkflowExecutionStatus.SUCCEEDED
        self._populate_completion_statistics(execution)

        self._workflow_execution_repository.save(execution)
        self._enqueue_trace_task(execution)

    def _handle_graph_run_partial_succeeded(self, event: GraphRunPartialSucceededEvent) -> None:
        execution = self._get_workflow_execution()
        execution.outputs = event.outputs
        execution.status = WorkflowExecutionStatus.PARTIAL_SUCCEEDED
        execution.exceptions_count = event.exceptions_count
        self._populate_completion_statistics(execution)

        self._workflow_execution_repository.save(execution)
        self._enqueue_trace_task(execution)

    def _handle_graph_run_failed(self, event: GraphRunFailedEvent) -> None:
        execution = self._get_workflow_execution()
        execution.status = WorkflowExecutionStatus.FAILED
        execution.error_message = event.error
        execution.exceptions_count = event.exceptions_count
        self._populate_completion_statistics(execution)

        self._fail_running_node_executions(error_message=event.error)
        self._workflow_execution_repository.save(execution)
        self._enqueue_trace_task(execution)

    def _handle_graph_run_aborted(self, event: GraphRunAbortedEvent) -> None:
        execution = self._get_workflow_execution()
        execution.status = WorkflowExecutionStatus.STOPPED
        execution.error_message = event.reason or "Workflow execution aborted"
        self._populate_completion_statistics(execution)

        self._fail_running_node_executions(error_message=execution.error_message or "")
        self._workflow_execution_repository.save(execution)
        self._enqueue_trace_task(execution)

    def _handle_graph_run_paused(self, event: GraphRunPausedEvent) -> None:
        execution = self._get_workflow_execution()
        execution.status = WorkflowExecutionStatus.PAUSED
        execution.outputs = event.outputs
        self._populate_completion_statistics(execution, update_finished=False)

        self._workflow_execution_repository.save(execution)

    # ------------------------------------------------------------------
    # Node-level handlers
    # ------------------------------------------------------------------
    def _handle_node_started(self, event: NodeRunStartedEvent) -> None:
        execution = self._get_workflow_execution()

        metadata = {
            WorkflowNodeExecutionMetadataKey.ITERATION_ID: event.in_iteration_id,
            WorkflowNodeExecutionMetadataKey.LOOP_ID: event.in_loop_id,
        }

        domain_execution = WorkflowNodeExecution(
            id=event.id,
            node_execution_id=event.id,
            workflow_id=execution.workflow_id,
            workflow_execution_id=execution.id_,
            predecessor_node_id=event.predecessor_node_id,
            index=self._next_node_sequence(),
            node_id=event.node_id,
            node_type=event.node_type,
            title=event.node_title,
            status=WorkflowNodeExecutionStatus.RUNNING,
            metadata=metadata,
            created_at=event.start_at,
        )

        self._node_execution_cache[event.id] = domain_execution
        self._workflow_node_execution_repository.save(domain_execution)

        snapshot = _NodeRuntimeSnapshot(
            node_id=event.node_id,
            title=event.node_title,
            predecessor_node_id=event.predecessor_node_id,
            iteration_id=event.in_iteration_id,
            loop_id=event.in_loop_id,
            created_at=event.start_at,
        )
        self._node_snapshots[event.id] = snapshot

    def _handle_node_retry(self, event: NodeRunRetryEvent) -> None:
        domain_execution = self._get_node_execution(event.id)
        domain_execution.status = WorkflowNodeExecutionStatus.RETRY
        domain_execution.error = event.error
        self._workflow_node_execution_repository.save(domain_execution)
        self._workflow_node_execution_repository.save_execution_data(domain_execution)

    def _handle_node_succeeded(self, event: NodeRunSucceededEvent) -> None:
        domain_execution = self._get_node_execution(event.id)
        self._update_node_execution(
            domain_execution,
            event.node_run_result,
            WorkflowNodeExecutionStatus.SUCCEEDED,
            finished_at=event.finished_at,
        )

    def _handle_node_failed(self, event: NodeRunFailedEvent) -> None:
        domain_execution = self._get_node_execution(event.id)
        self._update_node_execution(
            domain_execution,
            event.node_run_result,
            WorkflowNodeExecutionStatus.FAILED,
            error=event.error,
            finished_at=event.finished_at,
        )

    def _handle_node_exception(self, event: NodeRunExceptionEvent) -> None:
        domain_execution = self._get_node_execution(event.id)
        self._update_node_execution(
            domain_execution,
            event.node_run_result,
            WorkflowNodeExecutionStatus.EXCEPTION,
            error=event.error,
            finished_at=event.finished_at,
        )

    def _handle_node_pause_requested(self, event: NodeRunPauseRequestedEvent) -> None:
        domain_execution = self._get_node_execution(event.id)
        self._update_node_execution(
            domain_execution,
            event.node_run_result,
            WorkflowNodeExecutionStatus.PAUSED,
            error="",
            update_outputs=False,
        )

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------
    def _get_execution_id(self) -> str:
        workflow_execution_id = self._system_variables().get(SystemVariableKey.WORKFLOW_EXECUTION_ID)
        if not workflow_execution_id:
            raise ValueError("workflow_execution_id must be provided in system variables for pause/resume flows")
        return str(workflow_execution_id)

    def _prepare_workflow_inputs(self) -> Mapping[str, Any]:
        inputs = {**self._application_generate_entity.inputs}
        for field_name, value in self._system_variables().items():
            if field_name == SystemVariableKey.CONVERSATION_ID.value:
                # Conversation IDs are tied to the current session; omit them so persisted
                # workflow inputs stay reusable without binding future runs to this conversation.
                continue
            inputs[f"sys.{field_name}"] = value
        # Local import to avoid circular dependency during app bootstrapping.
        from core.workflow.workflow_entry import WorkflowEntry

        handled = WorkflowEntry.handle_special_values(inputs)
        return handled or {}

    def _get_workflow_execution(self) -> WorkflowExecution:
        if self._workflow_execution is None:
            raise ValueError("workflow execution not initialized")
        return self._workflow_execution

    def _get_node_execution(self, node_execution_id: str) -> WorkflowNodeExecution:
        if node_execution_id not in self._node_execution_cache:
            raise ValueError(f"Node execution not found for id={node_execution_id}")
        return self._node_execution_cache[node_execution_id]

    def _next_node_sequence(self) -> int:
        self._node_sequence += 1
        return self._node_sequence

    def _populate_completion_statistics(self, execution: WorkflowExecution, *, update_finished: bool = True) -> None:
        if update_finished:
            execution.finished_at = naive_utc_now()
        runtime_state = self.graph_runtime_state
        execution.total_tokens = runtime_state.total_tokens
        execution.total_steps = runtime_state.node_run_steps
        execution.outputs = execution.outputs or runtime_state.outputs
        execution.exceptions_count = runtime_state.exceptions_count

    def _update_node_execution(
        self,
        domain_execution: WorkflowNodeExecution,
        node_result: NodeRunResult,
        status: WorkflowNodeExecutionStatus,
        *,
        error: str | None = None,
        update_outputs: bool = True,
        finished_at: datetime | None = None,
    ) -> None:
        actual_finished_at = finished_at or naive_utc_now()
        snapshot = self._node_snapshots.get(domain_execution.id)
        start_at = snapshot.created_at if snapshot else domain_execution.created_at
        domain_execution.status = status
        domain_execution.finished_at = actual_finished_at
        domain_execution.elapsed_time = max((actual_finished_at - start_at).total_seconds(), 0.0)

        if error:
            domain_execution.error = error

        if update_outputs:
            projected_outputs = project_node_outputs_for_workflow_run(
                node_type=domain_execution.node_type,
                inputs=node_result.inputs,
                outputs=node_result.outputs,
            )
            domain_execution.update_from_mapping(
                inputs=node_result.inputs,
                process_data=node_result.process_data,
                outputs=projected_outputs,
                metadata=node_result.metadata,
            )

        self._workflow_node_execution_repository.save(domain_execution)
        self._workflow_node_execution_repository.save_execution_data(domain_execution)

    def _fail_running_node_executions(self, *, error_message: str) -> None:
        now = naive_utc_now()
        for execution in self._node_execution_cache.values():
            if execution.status == WorkflowNodeExecutionStatus.RUNNING:
                execution.status = WorkflowNodeExecutionStatus.FAILED
                execution.error = error_message
                execution.finished_at = now
                execution.elapsed_time = max((now - execution.created_at).total_seconds(), 0.0)
                self._workflow_node_execution_repository.save(execution)

    def _enqueue_trace_task(self, execution: WorkflowExecution) -> None:
        if not self._trace_manager:
            return

        conversation_id = self._system_variables().get(SystemVariableKey.CONVERSATION_ID.value)
        external_trace_id = None
        if isinstance(self._application_generate_entity, (WorkflowAppGenerateEntity, AdvancedChatAppGenerateEntity)):
            external_trace_id = self._application_generate_entity.extras.get("external_trace_id")

        trace_task = TraceTask(
            TraceTaskName.WORKFLOW_TRACE,
            workflow_execution=execution,
            conversation_id=conversation_id,
            user_id=self._trace_manager.user_id,
            external_trace_id=external_trace_id,
        )
        self._trace_manager.add_trace_task(trace_task)

    def _system_variables(self) -> Mapping[str, Any]:
        runtime_state = self.graph_runtime_state
        return runtime_state.variable_pool.get_by_prefix(SYSTEM_VARIABLE_NODE_ID)

```

### api/core/app/workflow/layers/llm_quota.py
```py
"""
LLM quota deduction layer for GraphEngine.

This layer centralizes model-quota deduction outside node implementations.
"""

import logging
from typing import TYPE_CHECKING, cast, final, override

from graphon.enums import BuiltinNodeTypes
from graphon.graph_engine.entities.commands import AbortCommand, CommandType
from graphon.graph_engine.layers import GraphEngineLayer
from graphon.graph_events import GraphEngineEvent, GraphNodeEventBase, NodeRunSucceededEvent
from graphon.nodes.base.node import Node

from core.app.entities.app_invoke_entities import DIFY_RUN_CONTEXT_KEY, DifyRunContext
from core.app.llm import deduct_llm_quota, ensure_llm_quota_available
from core.errors.error import QuotaExceededError
from core.model_manager import ModelInstance

if TYPE_CHECKING:
    from graphon.nodes.llm.node import LLMNode
    from graphon.nodes.parameter_extractor.parameter_extractor_node import ParameterExtractorNode
    from graphon.nodes.question_classifier.question_classifier_node import QuestionClassifierNode

logger = logging.getLogger(__name__)


@final
class LLMQuotaLayer(GraphEngineLayer):
    """Graph layer that applies LLM quota deduction after node execution."""

    def __init__(self) -> None:
        super().__init__()
        self._abort_sent = False

    @override
    def on_graph_start(self) -> None:
        self._abort_sent = False

    @override
    def on_event(self, event: GraphEngineEvent) -> None:
        _ = event

    @override
    def on_graph_end(self, error: Exception | None) -> None:
        _ = error

    @override
    def on_node_run_start(self, node: Node) -> None:
        if self._abort_sent:
            return

        model_instance = self._extract_model_instance(node)
        if model_instance is None:
            return

        try:
            ensure_llm_quota_available(model_instance=model_instance)
        except QuotaExceededError as exc:
            self._set_stop_event(node)
            self._send_abort_command(reason=str(exc))
            logger.warning("LLM quota check failed, node_id=%s, error=%s", node.id, exc)

    @override
    def on_node_run_end(
        self, node: Node, error: Exception | None, result_event: GraphNodeEventBase | None = None
    ) -> None:
        if error is not None or not isinstance(result_event, NodeRunSucceededEvent):
            return

        model_instance = self._extract_model_instance(node)
        if model_instance is None:
            return

        try:
            dify_ctx = DifyRunContext.model_validate(node.require_run_context_value(DIFY_RUN_CONTEXT_KEY))
            deduct_llm_quota(
                tenant_id=dify_ctx.tenant_id,
                model_instance=model_instance,
                usage=result_event.node_run_result.llm_usage,
            )
        except QuotaExceededError as exc:
            self._set_stop_event(node)
            self._send_abort_command(reason=str(exc))
            logger.warning("LLM quota deduction exceeded, node_id=%s, error=%s", node.id, exc)
        except Exception:
            logger.exception("LLM quota deduction failed, node_id=%s", node.id)

    @staticmethod
    def _set_stop_event(node: Node) -> None:
        stop_event = getattr(node.graph_runtime_state, "stop_event", None)
        if stop_event is not None:
            stop_event.set()

    def _send_abort_command(self, *, reason: str) -> None:
        if not self.command_channel or self._abort_sent:
            return

        try:
            self.command_channel.send_command(
                AbortCommand(
                    command_type=CommandType.ABORT,
                    reason=reason,
                )
            )
            self._abort_sent = True
        except Exception:
            logger.exception("Failed to send quota abort command")

    @staticmethod
    def _extract_model_instance(node: Node) -> ModelInstance | None:
        try:
            match node.node_type:
                case BuiltinNodeTypes.LLM:
                    model_instance = cast("LLMNode", node).model_instance
                case BuiltinNodeTypes.PARAMETER_EXTRACTOR:
                    model_instance = cast("ParameterExtractorNode", node).model_instance
                case BuiltinNodeTypes.QUESTION_CLASSIFIER:
                    model_instance = cast("QuestionClassifierNode", node).model_instance
                case _:
                    return None
        except AttributeError:
            logger.warning(
                "LLMQuotaLayer skipped quota deduction because node does not expose a model instance, node_id=%s",
                node.id,
            )
            return None

        if isinstance(model_instance, ModelInstance):
            return model_instance

        raw_model_instance = getattr(model_instance, "_model_instance", None)
        if isinstance(raw_model_instance, ModelInstance):
            return raw_model_instance

        return None

```

### api/core/app/workflow/layers/__init__.py
```py
"""Workflow-level GraphEngine layers that depend on outer infrastructure."""

from .llm_quota import LLMQuotaLayer
from .observability import ObservabilityLayer
from .persistence import PersistenceWorkflowInfo, WorkflowPersistenceLayer

__all__ = [
    "LLMQuotaLayer",
    "ObservabilityLayer",
    "PersistenceWorkflowInfo",
    "WorkflowPersistenceLayer",
]

```

### api/core/app/workflow/layers/observability.py
```py
"""
Observability layer for GraphEngine.

This layer creates OpenTelemetry spans for node execution, enabling distributed
tracing of workflow execution. It establishes OTel context during node execution
so that automatic instrumentation (HTTP requests, DB queries, etc.) automatically
associates with the node span.
"""

import logging
from contextvars import Token
from dataclasses import dataclass
from typing import cast, final, override

from graphon.enums import BuiltinNodeTypes, NodeType
from graphon.graph_engine.layers import GraphEngineLayer
from graphon.graph_events import GraphNodeEventBase
from graphon.nodes.base.node import Node
from opentelemetry import context as context_api
from opentelemetry.trace import Span, SpanKind, Tracer, get_tracer, set_span_in_context

from configs import dify_config
from extensions.otel.parser import (
    DefaultNodeOTelParser,
    LLMNodeOTelParser,
    NodeOTelParser,
    RetrievalNodeOTelParser,
    ToolNodeOTelParser,
)
from extensions.otel.runtime import is_instrument_flag_enabled

logger = logging.getLogger(__name__)


@dataclass(slots=True)
class _NodeSpanContext:
    span: "Span"
    token: Token[context_api.Context]


@final
class ObservabilityLayer(GraphEngineLayer):
    """
    Layer that creates OpenTelemetry spans for node execution.

    This layer:
    - Creates a span when a node starts execution
    - Establishes OTel context so automatic instrumentation associates with the span
    - Sets complete attributes and status when node execution ends
    """

    def __init__(self) -> None:
        super().__init__()
        self._node_contexts: dict[str, _NodeSpanContext] = {}
        self._parsers: dict[NodeType, NodeOTelParser] = {}
        self._default_parser: NodeOTelParser = cast(NodeOTelParser, DefaultNodeOTelParser())
        self._is_disabled: bool = False
        self._tracer: Tracer | None = None
        self._build_parser_registry()
        self._init_tracer()

    def _init_tracer(self) -> None:
        """Initialize OpenTelemetry tracer in constructor."""
        if not (dify_config.ENABLE_OTEL or is_instrument_flag_enabled()):
            self._is_disabled = True
            return

        try:
            self._tracer = get_tracer(__name__)
        except Exception as e:
            logger.warning("Failed to get OpenTelemetry tracer: %s", e)
            self._is_disabled = True

    def _build_parser_registry(self) -> None:
        """Initialize parser registry for node types."""
        self._parsers = {
            BuiltinNodeTypes.TOOL: ToolNodeOTelParser(),
            BuiltinNodeTypes.LLM: LLMNodeOTelParser(),
            BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL: RetrievalNodeOTelParser(),
        }

    def _get_parser(self, node: Node) -> NodeOTelParser:
        return self._parsers.get(node.node_type, self._default_parser)

    @override
    def on_graph_start(self) -> None:
        """Called when graph execution starts."""
        self._node_contexts.clear()

    @override
    def on_node_run_start(self, node: Node) -> None:
        """
        Called when a node starts execution.

        Creates a span and establishes OTel context for automatic instrumentation.
        """
        if self._is_disabled:
            return

        try:
            if not self._tracer:
                return

            execution_id = node.execution_id
            if not execution_id:
                return

            parent_context = context_api.get_current()
            span = self._tracer.start_span(
                f"{node.title}",
                kind=SpanKind.INTERNAL,
                context=parent_context,
            )

            new_context = set_span_in_context(span)
            token = context_api.attach(new_context)

            self._node_contexts[execution_id] = _NodeSpanContext(span=span, token=token)

        except Exception as e:
            logger.warning("Failed to create OpenTelemetry span for node %s: %s", node.id, e)

    @override
    def on_node_run_end(
        self, node: Node, error: Exception | None, result_event: GraphNodeEventBase | None = None
    ) -> None:
        """
        Called when a node finishes execution.

        Sets complete attributes, records exceptions, and ends the span.
        """
        if self._is_disabled:
            return

        try:
            execution_id = node.execution_id
            if not execution_id:
                return
            node_context = self._node_contexts.get(execution_id)
            if not node_context:
                return

            span = node_context.span
            parser = self._get_parser(node)
            try:
                parser.parse(node=node, span=span, error=error, result_event=result_event)
                span.end()
            finally:
                token = node_context.token
                if token is not None:
                    try:
                        context_api.detach(token)
                    except Exception:
                        logger.warning("Failed to detach OpenTelemetry token: %s", token)
                self._node_contexts.pop(execution_id, None)

        except Exception as e:
            logger.warning("Failed to end OpenTelemetry span for node %s: %s", node.id, e)

    @override
    def on_event(self, event) -> None:
        """Not used in this layer."""
        pass

    @override
    def on_graph_end(self, error: Exception | None) -> None:
        """Called when graph execution ends."""
        if self._node_contexts:
            logger.warning(
                "ObservabilityLayer: %d node spans were not properly ended",
                len(self._node_contexts),
            )
            self._node_contexts.clear()

```

### api/core/app/workflow/__init__.py
```py
from core.workflow.node_factory import DifyNodeFactory

__all__ = ["DifyNodeFactory"]

```

### api/core/app/workflow/file_runtime.py
```py
from __future__ import annotations

import base64
import hashlib
import hmac
import os
import time
import urllib.parse
from collections.abc import Generator
from typing import TYPE_CHECKING, Literal

from graphon.file import FileTransferMethod
from graphon.file.protocols import HttpResponseProtocol, WorkflowFileRuntimeProtocol
from graphon.file.runtime import set_workflow_file_runtime

from configs import dify_config
from core.app.file_access import DatabaseFileAccessController, FileAccessControllerProtocol
from core.db.session_factory import session_factory
from core.helper.ssrf_proxy import ssrf_proxy
from core.tools.signature import sign_tool_file
from core.workflow.file_reference import parse_file_reference
from extensions.ext_storage import storage

if TYPE_CHECKING:
    from graphon.file import File


class DifyWorkflowFileRuntime(WorkflowFileRuntimeProtocol):
    """Production runtime wiring for ``graphon.file``.

    Opaque file references are resolved back to canonical database records before
    URLs are signed or storage keys are used. When a request-scoped file access
    scope is present, those lookups additionally enforce tenant and end-user
    ownership filters.
    """

    _file_access_controller: FileAccessControllerProtocol

    def __init__(self, *, file_access_controller: FileAccessControllerProtocol) -> None:
        self._file_access_controller = file_access_controller

    @property
    def multimodal_send_format(self) -> str:
        return dify_config.MULTIMODAL_SEND_FORMAT

    def http_get(self, url: str, *, follow_redirects: bool = True) -> HttpResponseProtocol:
        return ssrf_proxy.get(url, follow_redirects=follow_redirects)

    def storage_load(self, path: str, *, stream: bool = False) -> bytes | Generator:
        return storage.load(path, stream=stream)

    def load_file_bytes(self, *, file: File) -> bytes:
        storage_key = self._resolve_storage_key(file=file)
        data = storage.load(storage_key, stream=False)
        if not isinstance(data, bytes):
            raise ValueError(f"file {storage_key} is not a bytes object")
        return data

    def resolve_file_url(self, *, file: File, for_external: bool = True) -> str | None:
        if file.transfer_method == FileTransferMethod.REMOTE_URL:
            return file.remote_url
        parsed_reference = parse_file_reference(file.reference)
        if parsed_reference is None:
            raise ValueError("Missing file reference")
        if file.transfer_method == FileTransferMethod.LOCAL_FILE:
            return self.resolve_upload_file_url(
                upload_file_id=parsed_reference.record_id,
                for_external=for_external,
            )
        if file.transfer_method == FileTransferMethod.DATASOURCE_FILE:
            if file.extension is None:
                raise ValueError("Missing file extension")
            self._assert_upload_file_access(upload_file_id=parsed_reference.record_id)
            return sign_tool_file(
                tool_file_id=parsed_reference.record_id,
                extension=file.extension,
                for_external=for_external,
            )
        if file.transfer_method == FileTransferMethod.TOOL_FILE:
            if file.extension is None:
                raise ValueError("Missing file extension")
            return self.resolve_tool_file_url(
                tool_file_id=parsed_reference.record_id,
                extension=file.extension,
                for_external=for_external,
            )
        return None

    def resolve_upload_file_url(
        self,
        *,
        upload_file_id: str,
        as_attachment: bool = False,
        for_external: bool = True,
    ) -> str:
        self._assert_upload_file_access(upload_file_id=upload_file_id)
        base_url = self._base_url(for_external=for_external)
        url = f"{base_url}/files/{upload_file_id}/file-preview"
        query = self._sign_query(payload=f"file-preview|{upload_file_id}")
        if as_attachment:
            query["as_attachment"] = "true"
        return f"{url}?{urllib.parse.urlencode(query)}"

    def resolve_tool_file_url(self, *, tool_file_id: str, extension: str, for_external: bool = True) -> str:
        self._assert_tool_file_access(tool_file_id=tool_file_id)
        return sign_tool_file(tool_file_id=tool_file_id, extension=extension, for_external=for_external)

    def verify_preview_signature(
        self,
        *,
        preview_kind: Literal["image", "file"],
        file_id: str,
        timestamp: str,
        nonce: str,
        sign: str,
    ) -> bool:
        payload = f"{preview_kind}-preview|{file_id}|{timestamp}|{nonce}"
        recalculated = hmac.new(self._secret_key(), payload.encode(), hashlib.sha256).digest()
        if sign != base64.urlsafe_b64encode(recalculated).decode():
            return False
        return int(time.time()) - int(timestamp) <= dify_config.FILES_ACCESS_TIMEOUT

    @staticmethod
    def _base_url(*, for_external: bool) -> str:
        if for_external:
            return dify_config.FILES_URL
        return dify_config.INTERNAL_FILES_URL or dify_config.FILES_URL

    @staticmethod
    def _secret_key() -> bytes:
        return dify_config.SECRET_KEY.encode() if dify_config.SECRET_KEY else b""

    def _sign_query(self, *, payload: str) -> dict[str, str]:
        timestamp = str(int(time.time()))
        nonce = os.urandom(16).hex()
        sign = hmac.new(self._secret_key(), f"{payload}|{timestamp}|{nonce}".encode(), hashlib.sha256).digest()
        return {
            "timestamp": timestamp,
            "nonce": nonce,
            "sign": base64.urlsafe_b64encode(sign).decode(),
        }

    def _resolve_storage_key(self, *, file: File) -> str:
        parsed_reference = parse_file_reference(file.reference)
        if parsed_reference is None:
            raise ValueError("Missing file reference")

        record_id = parsed_reference.record_id
        with session_factory.create_session() as session:
            if file.transfer_method in {
                FileTransferMethod.LOCAL_FILE,
                FileTransferMethod.REMOTE_URL,
                FileTransferMethod.DATASOURCE_FILE,
            }:
                upload_file = self._file_access_controller.get_upload_file(session=session, file_id=record_id)
                if upload_file is None:
                    raise ValueError(f"Upload file {record_id} not found")
                return upload_file.key

            tool_file = self._file_access_controller.get_tool_file(session=session, file_id=record_id)
            if tool_file is None:
                raise ValueError(f"Tool file {record_id} not found")
            return tool_file.file_key

    def _assert_upload_file_access(self, *, upload_file_id: str) -> None:
        if self._file_access_controller.current_scope() is None:
            return

        with session_factory.create_session() as session:
            upload_file = self._file_access_controller.get_upload_file(session=session, file_id=upload_file_id)
            if upload_file is None:
                raise ValueError(f"Upload file {upload_file_id} not found")

    def _assert_tool_file_access(self, *, tool_file_id: str) -> None:
        if self._file_access_controller.current_scope() is None:
            return

        with session_factory.create_session() as session:
            tool_file = self._file_access_controller.get_tool_file(session=session, file_id=tool_file_id)
            if tool_file is None:
                raise ValueError(f"Tool file {tool_file_id} not found")


def bind_dify_workflow_file_runtime() -> None:
    set_workflow_file_runtime(DifyWorkflowFileRuntime(file_access_controller=DatabaseFileAccessController()))

```

### api/controllers/console/app/conversation_variables.py
```py
from flask import request
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from extensions.ext_database import db
from fields.conversation_variable_fields import (
    conversation_variable_fields,
    paginated_conversation_variable_fields,
)
from libs.login import login_required
from models import ConversationVariable
from models.model import AppMode

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ConversationVariablesQuery(BaseModel):
    conversation_id: str = Field(..., description="Conversation ID to filter variables")


console_ns.schema_model(
    ConversationVariablesQuery.__name__,
    ConversationVariablesQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)

# Register models for flask_restx to avoid dict type issues in Swagger
# Register base model first
conversation_variable_model = console_ns.model("ConversationVariable", conversation_variable_fields)

# For nested models, need to replace nested dict with registered model
paginated_conversation_variable_fields_copy = paginated_conversation_variable_fields.copy()
paginated_conversation_variable_fields_copy["data"] = fields.List(
    fields.Nested(conversation_variable_model), attribute="data"
)
paginated_conversation_variable_model = console_ns.model(
    "PaginatedConversationVariable", paginated_conversation_variable_fields_copy
)


@console_ns.route("/apps/<uuid:app_id>/conversation-variables")
class ConversationVariablesApi(Resource):
    @console_ns.doc("get_conversation_variables")
    @console_ns.doc(description="Get conversation variables for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[ConversationVariablesQuery.__name__])
    @console_ns.response(200, "Conversation variables retrieved successfully", paginated_conversation_variable_model)
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.ADVANCED_CHAT)
    @marshal_with(paginated_conversation_variable_model)
    def get(self, app_model):
        args = ConversationVariablesQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        stmt = (
            select(ConversationVariable)
            .where(ConversationVariable.app_id == app_model.id)
            .order_by(ConversationVariable.created_at)
        )
        stmt = stmt.where(ConversationVariable.conversation_id == args.conversation_id)

        # NOTE: This is a temporary solution to avoid performance issues.
        page = 1
        page_size = 100
        stmt = stmt.limit(page_size).offset((page - 1) * page_size)

        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            rows = session.scalars(stmt).all()

        return {
            "page": page,
            "limit": page_size,
            "total": len(rows),
            "has_more": False,
            "data": [
                {
                    "created_at": row.created_at,
                    "updated_at": row.updated_at,
                    **row.to_variable().model_dump(),
                }
                for row in rows
            ],
        }

```

### api/controllers/console/app/advanced_prompt_template.py
```py
from flask import request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field

from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import login_required
from services.advanced_prompt_template_service import AdvancedPromptTemplateService


class AdvancedPromptTemplateQuery(BaseModel):
    app_mode: str = Field(..., description="Application mode")
    model_mode: str = Field(..., description="Model mode")
    has_context: str = Field(default="true", description="Whether has context")
    model_name: str = Field(..., description="Model name")


console_ns.schema_model(
    AdvancedPromptTemplateQuery.__name__,
    AdvancedPromptTemplateQuery.model_json_schema(ref_template="#/definitions/{model}"),
)


@console_ns.route("/app/prompt-templates")
class AdvancedPromptTemplateList(Resource):
    @console_ns.doc("get_advanced_prompt_templates")
    @console_ns.doc(description="Get advanced prompt templates based on app mode and model configuration")
    @console_ns.expect(console_ns.models[AdvancedPromptTemplateQuery.__name__])
    @console_ns.response(
        200, "Prompt templates retrieved successfully", fields.List(fields.Raw(description="Prompt template data"))
    )
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        args = AdvancedPromptTemplateQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        return AdvancedPromptTemplateService.get_prompt(args.model_dump())

```

### api/controllers/console/app/annotation.py
```py
from typing import Any, Literal

from flask import abort, make_response, request
from flask_restx import Resource
from pydantic import BaseModel, Field, TypeAdapter, field_validator

from controllers.common.errors import NoFileUploadedError, TooManyFilesError
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import (
    account_initialization_required,
    annotation_import_concurrency_limit,
    annotation_import_rate_limit,
    cloud_edition_billing_resource_check,
    edit_permission_required,
    setup_required,
)
from extensions.ext_redis import redis_client
from fields.annotation_fields import (
    Annotation,
    AnnotationExportList,
    AnnotationHitHistory,
    AnnotationHitHistoryList,
    AnnotationList,
)
from libs.helper import uuid_value
from libs.login import login_required
from services.annotation_service import AppAnnotationService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class AnnotationReplyPayload(BaseModel):
    score_threshold: float = Field(..., description="Score threshold for annotation matching")
    embedding_provider_name: str = Field(..., description="Embedding provider name")
    embedding_model_name: str = Field(..., description="Embedding model name")


class AnnotationSettingUpdatePayload(BaseModel):
    score_threshold: float = Field(..., description="Score threshold")


class AnnotationListQuery(BaseModel):
    page: int = Field(default=1, ge=1, description="Page number")
    limit: int = Field(default=20, ge=1, description="Page size")
    keyword: str = Field(default="", description="Search keyword")


class CreateAnnotationPayload(BaseModel):
    message_id: str | None = Field(default=None, description="Message ID")
    question: str | None = Field(default=None, description="Question text")
    answer: str | None = Field(default=None, description="Answer text")
    content: str | None = Field(default=None, description="Content text")
    annotation_reply: dict[str, Any] | None = Field(default=None, description="Annotation reply data")

    @field_validator("message_id")
    @classmethod
    def validate_message_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class UpdateAnnotationPayload(BaseModel):
    question: str | None = None
    answer: str | None = None
    content: str | None = None
    annotation_reply: dict[str, Any] | None = None


class AnnotationReplyStatusQuery(BaseModel):
    action: Literal["enable", "disable"]


class AnnotationFilePayload(BaseModel):
    message_id: str = Field(..., description="Message ID")

    @field_validator("message_id")
    @classmethod
    def validate_message_id(cls, value: str) -> str:
        return uuid_value(value)


def reg(model: type[BaseModel]) -> None:
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(AnnotationReplyPayload)
reg(AnnotationSettingUpdatePayload)
reg(AnnotationListQuery)
reg(CreateAnnotationPayload)
reg(UpdateAnnotationPayload)
reg(AnnotationReplyStatusQuery)
reg(AnnotationFilePayload)
register_schema_models(
    console_ns,
    Annotation,
    AnnotationList,
    AnnotationExportList,
    AnnotationHitHistory,
    AnnotationHitHistoryList,
)


@console_ns.route("/apps/<uuid:app_id>/annotation-reply/<string:action>")
class AnnotationReplyActionApi(Resource):
    @console_ns.doc("annotation_reply_action")
    @console_ns.doc(description="Enable or disable annotation reply for an app")
    @console_ns.doc(params={"app_id": "Application ID", "action": "Action to perform (enable/disable)"})
    @console_ns.expect(console_ns.models[AnnotationReplyPayload.__name__])
    @console_ns.response(200, "Action completed successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @edit_permission_required
    def post(self, app_id, action: Literal["enable", "disable"]):
        app_id = str(app_id)
        args = AnnotationReplyPayload.model_validate(console_ns.payload)
        match action:
            case "enable":
                result = AppAnnotationService.enable_app_annotation(args.model_dump(), app_id)
            case "disable":
                result = AppAnnotationService.disable_app_annotation(app_id)
        return result, 200


@console_ns.route("/apps/<uuid:app_id>/annotation-setting")
class AppAnnotationSettingDetailApi(Resource):
    @console_ns.doc("get_annotation_setting")
    @console_ns.doc(description="Get annotation settings for an app")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Annotation settings retrieved successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, app_id):
        app_id = str(app_id)
        result = AppAnnotationService.get_app_annotation_setting_by_app_id(app_id)
        return result, 200


@console_ns.route("/apps/<uuid:app_id>/annotation-settings/<uuid:annotation_setting_id>")
class AppAnnotationSettingUpdateApi(Resource):
    @console_ns.doc("update_annotation_setting")
    @console_ns.doc(description="Update annotation settings for an app")
    @console_ns.doc(params={"app_id": "Application ID", "annotation_setting_id": "Annotation setting ID"})
    @console_ns.expect(console_ns.models[AnnotationSettingUpdatePayload.__name__])
    @console_ns.response(200, "Settings updated successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, app_id, annotation_setting_id):
        app_id = str(app_id)
        annotation_setting_id = str(annotation_setting_id)

        args = AnnotationSettingUpdatePayload.model_validate(console_ns.payload)

        result = AppAnnotationService.update_app_annotation_setting(app_id, annotation_setting_id, args.model_dump())
        return result, 200


@console_ns.route("/apps/<uuid:app_id>/annotation-reply/<string:action>/status/<uuid:job_id>")
class AnnotationReplyActionStatusApi(Resource):
    @console_ns.doc("get_annotation_reply_action_status")
    @console_ns.doc(description="Get status of annotation reply action job")
    @console_ns.doc(params={"app_id": "Application ID", "job_id": "Job ID", "action": "Action type"})
    @console_ns.response(200, "Job status retrieved successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @edit_permission_required
    def get(self, app_id, job_id, action):
        job_id = str(job_id)
        app_annotation_job_key = f"{action}_app_annotation_job_{str(job_id)}"
        cache_result = redis_client.get(app_annotation_job_key)
        if cache_result is None:
            raise ValueError("The job does not exist.")

        job_status = cache_result.decode()
        error_msg = ""
        if job_status == "error":
            app_annotation_error_key = f"{action}_app_annotation_error_{str(job_id)}"
            error_msg = redis_client.get(app_annotation_error_key).decode()

        return {"job_id": job_id, "job_status": job_status, "error_msg": error_msg}, 200


@console_ns.route("/apps/<uuid:app_id>/annotations")
class AnnotationApi(Resource):
    @console_ns.doc("list_annotations")
    @console_ns.doc(description="Get annotations for an app with pagination")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AnnotationListQuery.__name__])
    @console_ns.response(200, "Annotations retrieved successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, app_id):
        args = AnnotationListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        page = args.page
        limit = args.limit
        keyword = args.keyword

        app_id = str(app_id)
        annotation_list, total = AppAnnotationService.get_annotation_list_by_app_id(app_id, page, limit, keyword)
        annotation_models = TypeAdapter(list[Annotation]).validate_python(annotation_list, from_attributes=True)
        response = AnnotationList(
            data=annotation_models,
            has_more=len(annotation_list) == limit,
            limit=limit,
            total=total,
            page=page,
        )
        return response.model_dump(mode="json"), 200

    @console_ns.doc("create_annotation")
    @console_ns.doc(description="Create a new annotation for an app")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[CreateAnnotationPayload.__name__])
    @console_ns.response(201, "Annotation created successfully", console_ns.models[Annotation.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @edit_permission_required
    def post(self, app_id):
        app_id = str(app_id)
        args = CreateAnnotationPayload.model_validate(console_ns.payload)
        data = args.model_dump(exclude_none=True)
        annotation = AppAnnotationService.up_insert_app_annotation_from_message(data, app_id)
        return Annotation.model_validate(annotation, from_attributes=True).model_dump(mode="json")

    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def delete(self, app_id):
        app_id = str(app_id)

        # Use request.args.getlist to get annotation_ids array directly
        annotation_ids = request.args.getlist("annotation_id")

        # If annotation_ids are provided, handle batch deletion
        if annotation_ids:
            # Check if any annotation_ids contain empty strings or invalid values
            if not all(annotation_id.strip() for annotation_id in annotation_ids if annotation_id):
                return {
                    "code": "bad_request",
                    "message": "annotation_ids are required if the parameter is provided.",
                }, 400

            result = AppAnnotationService.delete_app_annotations_in_batch(app_id, annotation_ids)
            return result, 204
        # If no annotation_ids are provided, handle clearing all annotations
        else:
            AppAnnotationService.clear_all_annotations(app_id)
            return {"result": "success"}, 204


@console_ns.route("/apps/<uuid:app_id>/annotations/export")
class AnnotationExportApi(Resource):
    @console_ns.doc("export_annotations")
    @console_ns.doc(description="Export all annotations for an app with CSV injection protection")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(
        200,
        "Annotations exported successfully",
        console_ns.models[AnnotationExportList.__name__],
    )
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, app_id):
        app_id = str(app_id)
        annotation_list = AppAnnotationService.export_annotation_list_by_app_id(app_id)
        annotation_models = TypeAdapter(list[Annotation]).validate_python(annotation_list, from_attributes=True)
        response_data = AnnotationExportList(data=annotation_models).model_dump(mode="json")

        # Create response with secure headers for CSV export
        response = make_response(response_data, 200)
        response.headers["Content-Type"] = "application/json; charset=utf-8"
        response.headers["X-Content-Type-Options"] = "nosniff"

        return response


@console_ns.route("/apps/<uuid:app_id>/annotations/<uuid:annotation_id>")
class AnnotationUpdateDeleteApi(Resource):
    @console_ns.doc("update_delete_annotation")
    @console_ns.doc(description="Update or delete an annotation")
    @console_ns.doc(params={"app_id": "Application ID", "annotation_id": "Annotation ID"})
    @console_ns.response(200, "Annotation updated successfully", console_ns.models[Annotation.__name__])
    @console_ns.response(204, "Annotation deleted successfully")
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.expect(console_ns.models[UpdateAnnotationPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @edit_permission_required
    def post(self, app_id, annotation_id):
        app_id = str(app_id)
        annotation_id = str(annotation_id)
        args = UpdateAnnotationPayload.model_validate(console_ns.payload)
        annotation = AppAnnotationService.update_app_annotation_directly(
            args.model_dump(exclude_none=True), app_id, annotation_id
        )
        return Annotation.model_validate(annotation, from_attributes=True).model_dump(mode="json")

    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def delete(self, app_id, annotation_id):
        app_id = str(app_id)
        annotation_id = str(annotation_id)
        AppAnnotationService.delete_app_annotation(app_id, annotation_id)
        return {"result": "success"}, 204


@console_ns.route("/apps/<uuid:app_id>/annotations/batch-import")
class AnnotationBatchImportApi(Resource):
    @console_ns.doc("batch_import_annotations")
    @console_ns.doc(description="Batch import annotations from CSV file with rate limiting and security checks")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Batch import started successfully")
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(400, "No file uploaded or too many files")
    @console_ns.response(413, "File too large")
    @console_ns.response(429, "Too many requests or concurrent imports")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @annotation_import_rate_limit
    @annotation_import_concurrency_limit
    @edit_permission_required
    def post(self, app_id):
        from configs import dify_config

        app_id = str(app_id)

        # check file
        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()

        # get file from request
        file = request.files["file"]

        # check file type
        if not file.filename or not file.filename.lower().endswith(".csv"):
            raise ValueError("Invalid file type. Only CSV files are allowed")

        # Check file size before processing
        file.seek(0, 2)  # Seek to end of file
        file_size = file.tell()
        file.seek(0)  # Reset to beginning

        max_size_bytes = dify_config.ANNOTATION_IMPORT_FILE_SIZE_LIMIT * 1024 * 1024
        if file_size > max_size_bytes:
            abort(
                413,
                f"File size exceeds maximum limit of {dify_config.ANNOTATION_IMPORT_FILE_SIZE_LIMIT}MB. "
                f"Please reduce the file size and try again.",
            )

        if file_size == 0:
            raise ValueError("The uploaded file is empty")

        return AppAnnotationService.batch_import_app_annotations(app_id, file)


@console_ns.route("/apps/<uuid:app_id>/annotations/batch-import-status/<uuid:job_id>")
class AnnotationBatchImportStatusApi(Resource):
    @console_ns.doc("get_batch_import_status")
    @console_ns.doc(description="Get status of batch import job")
    @console_ns.doc(params={"app_id": "Application ID", "job_id": "Job ID"})
    @console_ns.response(200, "Job status retrieved successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("annotation")
    @edit_permission_required
    def get(self, app_id, job_id):
        job_id = str(job_id)
        indexing_cache_key = f"app_annotation_batch_import_{str(job_id)}"
        cache_result = redis_client.get(indexing_cache_key)
        if cache_result is None:
            raise ValueError("The job does not exist.")
        job_status = cache_result.decode()
        error_msg = ""
        if job_status == "error":
            indexing_error_msg_key = f"app_annotation_batch_import_error_msg_{str(job_id)}"
            error_msg = redis_client.get(indexing_error_msg_key).decode()

        return {"job_id": job_id, "job_status": job_status, "error_msg": error_msg}, 200


@console_ns.route("/apps/<uuid:app_id>/annotations/<uuid:annotation_id>/hit-histories")
class AnnotationHitHistoryListApi(Resource):
    @console_ns.doc("list_annotation_hit_histories")
    @console_ns.doc(description="Get hit histories for an annotation")
    @console_ns.doc(params={"app_id": "Application ID", "annotation_id": "Annotation ID"})
    @console_ns.expect(
        console_ns.parser()
        .add_argument("page", type=int, location="args", default=1, help="Page number")
        .add_argument("limit", type=int, location="args", default=20, help="Page size")
    )
    @console_ns.response(
        200,
        "Hit histories retrieved successfully",
        console_ns.models[AnnotationHitHistoryList.__name__],
    )
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, app_id, annotation_id):
        page = request.args.get("page", default=1, type=int)
        limit = request.args.get("limit", default=20, type=int)
        app_id = str(app_id)
        annotation_id = str(annotation_id)
        annotation_hit_history_list, total = AppAnnotationService.get_annotation_hit_histories(
            app_id, annotation_id, page, limit
        )
        history_models = TypeAdapter(list[AnnotationHitHistory]).validate_python(
            annotation_hit_history_list, from_attributes=True
        )
        response = AnnotationHitHistoryList(
            data=history_models,
            has_more=len(annotation_hit_history_list) == limit,
            limit=limit,
            total=total,
            page=page,
        )
        return response.model_dump(mode="json")

```

### api/controllers/console/app/error.py
```py
from libs.exception import BaseHTTPException


class AppNotFoundError(BaseHTTPException):
    error_code = "app_not_found"
    description = "App not found."
    code = 404


class ProviderNotInitializeError(BaseHTTPException):
    error_code = "provider_not_initialize"
    description = (
        "No valid model provider credentials found. "
        "Please go to Settings -> Model Provider to complete your provider credentials."
    )
    code = 400


class ProviderQuotaExceededError(BaseHTTPException):
    error_code = "provider_quota_exceeded"
    description = (
        "Your quota for Dify Hosted Model Provider has been exhausted. "
        "Please go to Settings -> Model Provider to complete your own provider credentials."
    )
    code = 400


class ProviderModelCurrentlyNotSupportError(BaseHTTPException):
    error_code = "model_currently_not_support"
    description = "Dify Hosted OpenAI trial currently not support the GPT-4 model."
    code = 400


class ConversationCompletedError(BaseHTTPException):
    error_code = "conversation_completed"
    description = "The conversation has ended. Please start a new conversation."
    code = 400


class AppUnavailableError(BaseHTTPException):
    error_code = "app_unavailable"
    description = "App unavailable, please check your app configurations."
    code = 400


class CompletionRequestError(BaseHTTPException):
    error_code = "completion_request_error"
    description = "Completion request failed."
    code = 400


class AppMoreLikeThisDisabledError(BaseHTTPException):
    error_code = "app_more_like_this_disabled"
    description = "The 'More like this' feature is disabled. Please refresh your page."
    code = 403


class NoAudioUploadedError(BaseHTTPException):
    error_code = "no_audio_uploaded"
    description = "Please upload your audio."
    code = 400


class AudioTooLargeError(BaseHTTPException):
    error_code = "audio_too_large"
    description = "Audio size exceeded. {message}"
    code = 413


class UnsupportedAudioTypeError(BaseHTTPException):
    error_code = "unsupported_audio_type"
    description = "Audio type not allowed."
    code = 415


class ProviderNotSupportSpeechToTextError(BaseHTTPException):
    error_code = "provider_not_support_speech_to_text"
    description = "Provider not support speech to text."
    code = 400


class DraftWorkflowNotExist(BaseHTTPException):
    error_code = "draft_workflow_not_exist"
    description = "Draft workflow need to be initialized."
    code = 404


class DraftWorkflowNotSync(BaseHTTPException):
    error_code = "draft_workflow_not_sync"
    description = "Workflow graph might have been modified, please refresh and resubmit."
    code = 409


class TracingConfigNotExist(BaseHTTPException):
    error_code = "trace_config_not_exist"
    description = "Trace config not exist."
    code = 400


class TracingConfigIsExist(BaseHTTPException):
    error_code = "trace_config_is_exist"
    description = "Trace config is exist."
    code = 400


class TracingConfigCheckError(BaseHTTPException):
    error_code = "trace_config_check_error"
    description = "Invalid Credentials."
    code = 400


class InvokeRateLimitError(BaseHTTPException):
    """Raised when the Invoke returns rate limit error."""

    error_code = "rate_limit_error"
    description = "Rate Limit Error"
    code = 429


class NeedAddIdsError(BaseHTTPException):
    error_code = "need_add_ids"
    description = "Need to add ids."
    code = 400

```

### api/controllers/console/app/model_config.py
```py
import json
from typing import cast

from flask import request
from flask_restx import Resource, fields

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from core.agent.entities import AgentToolEntity
from core.tools.tool_manager import ToolManager
from core.tools.utils.configuration import ToolParameterConfigurationManager
from events.app_event import app_model_config_was_updated
from extensions.ext_database import db
from libs.datetime_utils import naive_utc_now
from libs.login import current_account_with_tenant, login_required
from models.model import AppMode, AppModelConfig
from services.app_model_config_service import AppModelConfigService


@console_ns.route("/apps/<uuid:app_id>/model-config")
class ModelConfigResource(Resource):
    @console_ns.doc("update_app_model_config")
    @console_ns.doc(description="Update application model configuration")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(
        console_ns.model(
            "ModelConfigRequest",
            {
                "provider": fields.String(description="Model provider"),
                "model": fields.String(description="Model name"),
                "configs": fields.Raw(description="Model configuration parameters"),
                "opening_statement": fields.String(description="Opening statement"),
                "suggested_questions": fields.List(fields.String(), description="Suggested questions"),
                "more_like_this": fields.Raw(description="More like this configuration"),
                "speech_to_text": fields.Raw(description="Speech to text configuration"),
                "text_to_speech": fields.Raw(description="Text to speech configuration"),
                "retrieval_model": fields.Raw(description="Retrieval model configuration"),
                "tools": fields.List(fields.Raw(), description="Available tools"),
                "dataset_configs": fields.Raw(description="Dataset configurations"),
                "agent_mode": fields.Raw(description="Agent mode configuration"),
            },
        )
    )
    @console_ns.response(200, "Model configuration updated successfully")
    @console_ns.response(400, "Invalid configuration")
    @console_ns.response(404, "App not found")
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.AGENT_CHAT, AppMode.CHAT, AppMode.COMPLETION])
    def post(self, app_model):
        """Modify app model config"""
        current_user, current_tenant_id = current_account_with_tenant()
        # validate config
        model_configuration = AppModelConfigService.validate_configuration(
            tenant_id=current_tenant_id,
            config=cast(dict, request.json),
            app_mode=AppMode.value_of(app_model.mode),
        )

        new_app_model_config = AppModelConfig(
            app_id=app_model.id,
            created_by=current_user.id,
            updated_by=current_user.id,
        )
        new_app_model_config = new_app_model_config.from_model_config_dict(model_configuration)

        if app_model.mode == AppMode.AGENT_CHAT or app_model.is_agent:
            # get original app model config
            original_app_model_config = db.session.get(AppModelConfig, app_model.app_model_config_id)
            if original_app_model_config is None:
                raise ValueError("Original app model config not found")
            agent_mode = original_app_model_config.agent_mode_dict
            # decrypt agent tool parameters if it's secret-input
            parameter_map = {}
            masked_parameter_map = {}
            tool_map = {}
            for tool in agent_mode.get("tools") or []:
                if not isinstance(tool, dict) or len(tool.keys()) <= 3:
                    continue

                agent_tool_entity = AgentToolEntity.model_validate(tool)
                # get tool
                try:
                    tool_runtime = ToolManager.get_agent_tool_runtime(
                        tenant_id=current_tenant_id,
                        app_id=app_model.id,
                        agent_tool=agent_tool_entity,
                        user_id=current_user.id,
                    )
                    manager = ToolParameterConfigurationManager(
                        tenant_id=current_tenant_id,
                        tool_runtime=tool_runtime,
                        provider_name=agent_tool_entity.provider_id,
                        provider_type=agent_tool_entity.provider_type,
                        identity_id=f"AGENT.{app_model.id}",
                    )
                except Exception:
                    continue

                # get decrypted parameters
                if agent_tool_entity.tool_parameters:
                    parameters = manager.decrypt_tool_parameters(agent_tool_entity.tool_parameters or {})
                    masked_parameter = manager.mask_tool_parameters(parameters or {})
                else:
                    parameters = {}
                    masked_parameter = {}

                key = f"{agent_tool_entity.provider_id}.{agent_tool_entity.provider_type}.{agent_tool_entity.tool_name}"
                masked_parameter_map[key] = masked_parameter
                parameter_map[key] = parameters
                tool_map[key] = tool_runtime

            # encrypt agent tool parameters if it's secret-input
            agent_mode = new_app_model_config.agent_mode_dict
            for tool in agent_mode.get("tools") or []:
                agent_tool_entity = AgentToolEntity.model_validate(tool)

                # get tool
                key = f"{agent_tool_entity.provider_id}.{agent_tool_entity.provider_type}.{agent_tool_entity.tool_name}"
                if key in tool_map:
                    tool_runtime = tool_map[key]
                else:
                    try:
                        tool_runtime = ToolManager.get_agent_tool_runtime(
                            tenant_id=current_tenant_id,
                            app_id=app_model.id,
                            agent_tool=agent_tool_entity,
                            user_id=current_user.id,
                        )
                    except Exception:
                        continue

                manager = ToolParameterConfigurationManager(
                    tenant_id=current_tenant_id,
                    tool_runtime=tool_runtime,
                    provider_name=agent_tool_entity.provider_id,
                    provider_type=agent_tool_entity.provider_type,
                    identity_id=f"AGENT.{app_model.id}",
                )
                manager.delete_tool_parameters_cache()

                # override parameters if it equals to masked parameters
                if agent_tool_entity.tool_parameters:
                    if key not in masked_parameter_map:
                        continue

                    for masked_key, masked_value in masked_parameter_map[key].items():
                        if (
                            masked_key in agent_tool_entity.tool_parameters
                            and agent_tool_entity.tool_parameters[masked_key] == masked_value
                        ):
                            agent_tool_entity.tool_parameters[masked_key] = parameter_map[key].get(masked_key)

                # encrypt parameters
                if agent_tool_entity.tool_parameters:
                    tool["tool_parameters"] = manager.encrypt_tool_parameters(agent_tool_entity.tool_parameters or {})

            # update app model config
            new_app_model_config.agent_mode = json.dumps(agent_mode)

        db.session.add(new_app_model_config)
        db.session.flush()

        app_model.app_model_config_id = new_app_model_config.id
        app_model.updated_by = current_user.id
        app_model.updated_at = naive_utc_now()
        db.session.commit()

        app_model_config_was_updated.send(app_model, app_model_config=new_app_model_config)

        return {"result": "success"}

```

### api/controllers/console/app/completion.py
```py
import logging
from typing import Any, Literal

from flask import request
from flask_restx import Resource
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, field_validator
from werkzeug.exceptions import InternalServerError, NotFound

import services
from controllers.console import console_ns
from controllers.console.app.error import (
    AppUnavailableError,
    CompletionRequestError,
    ConversationCompletedError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from core.helper.trace_id_helper import get_external_trace_id
from libs import helper
from libs.helper import uuid_value
from libs.login import current_user, login_required
from models import Account
from models.model import AppMode
from services.app_generate_service import AppGenerateService
from services.app_task_service import AppTaskService
from services.errors.llm import InvokeRateLimitError

logger = logging.getLogger(__name__)
DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class BaseMessagePayload(BaseModel):
    inputs: dict[str, Any]
    model_config_data: dict[str, Any] = Field(..., alias="model_config")
    files: list[Any] | None = Field(default=None, description="Uploaded files")
    response_mode: Literal["blocking", "streaming"] = Field(default="blocking", description="Response mode")
    retriever_from: str = Field(default="dev", description="Retriever source")


class CompletionMessagePayload(BaseMessagePayload):
    query: str = Field(default="", description="Query text")


class ChatMessagePayload(BaseMessagePayload):
    query: str = Field(..., description="User query")
    conversation_id: str | None = Field(default=None, description="Conversation ID")
    parent_message_id: str | None = Field(default=None, description="Parent message ID")

    @field_validator("conversation_id", "parent_message_id")
    @classmethod
    def validate_uuid(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


console_ns.schema_model(
    CompletionMessagePayload.__name__,
    CompletionMessagePayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)
console_ns.schema_model(
    ChatMessagePayload.__name__, ChatMessagePayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


# define completion message api for user
@console_ns.route("/apps/<uuid:app_id>/completion-messages")
class CompletionMessageApi(Resource):
    @console_ns.doc("create_completion_message")
    @console_ns.doc(description="Generate completion message for debugging")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[CompletionMessagePayload.__name__])
    @console_ns.response(200, "Completion generated successfully")
    @console_ns.response(400, "Invalid request parameters")
    @console_ns.response(404, "App not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    def post(self, app_model):
        args_model = CompletionMessagePayload.model_validate(console_ns.payload)
        args = args_model.model_dump(exclude_none=True, by_alias=True)

        streaming = args_model.response_mode != "blocking"
        args["auto_generate_name"] = False

        try:
            if not isinstance(current_user, Account):
                raise ValueError("current_user must be an Account or EndUser instance")
            response = AppGenerateService.generate(
                app_model=app_model, user=current_user, args=args, invoke_from=InvokeFrom.DEBUGGER, streaming=streaming
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
        except Exception as e:
            logger.exception("internal server error.")
            raise InternalServerError()


@console_ns.route("/apps/<uuid:app_id>/completion-messages/<string:task_id>/stop")
class CompletionMessageStopApi(Resource):
    @console_ns.doc("stop_completion_message")
    @console_ns.doc(description="Stop a running completion message generation")
    @console_ns.doc(params={"app_id": "Application ID", "task_id": "Task ID to stop"})
    @console_ns.response(200, "Task stopped successfully")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    def post(self, app_model, task_id):
        if not isinstance(current_user, Account):
            raise ValueError("current_user must be an Account instance")

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.DEBUGGER,
            user_id=current_user.id,
            app_mode=AppMode.value_of(app_model.mode),
        )

        return {"result": "success"}, 200


@console_ns.route("/apps/<uuid:app_id>/chat-messages")
class ChatMessageApi(Resource):
    @console_ns.doc("create_chat_message")
    @console_ns.doc(description="Generate chat message for debugging")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[ChatMessagePayload.__name__])
    @console_ns.response(200, "Chat message generated successfully")
    @console_ns.response(400, "Invalid request parameters")
    @console_ns.response(404, "App or conversation not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT])
    @edit_permission_required
    def post(self, app_model):
        args_model = ChatMessagePayload.model_validate(console_ns.payload)
        args = args_model.model_dump(exclude_none=True, by_alias=True)

        streaming = args_model.response_mode != "blocking"
        args["auto_generate_name"] = False

        external_trace_id = get_external_trace_id(request)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id

        try:
            if not isinstance(current_user, Account):
                raise ValueError("current_user must be an Account or EndUser instance")
            response = AppGenerateService.generate(
                app_model=app_model, user=current_user, args=args, invoke_from=InvokeFrom.DEBUGGER, streaming=streaming
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
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise e
        except Exception as e:
            logger.exception("internal server error.")
            raise InternalServerError()


@console_ns.route("/apps/<uuid:app_id>/chat-messages/<string:task_id>/stop")
class ChatMessageStopApi(Resource):
    @console_ns.doc("stop_chat_message")
    @console_ns.doc(description="Stop a running chat message generation")
    @console_ns.doc(params={"app_id": "Application ID", "task_id": "Task ID to stop"})
    @console_ns.response(200, "Task stopped successfully")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    def post(self, app_model, task_id):
        if not isinstance(current_user, Account):
            raise ValueError("current_user must be an Account instance")

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.DEBUGGER,
            user_id=current_user.id,
            app_mode=AppMode.value_of(app_model.mode),
        )

        return {"result": "success"}, 200

```

### api/controllers/console/app/workflow_draft_variable.py
```py
import logging
from collections.abc import Callable
from functools import wraps
from typing import Any

from flask import Response, request
from flask_restx import Resource, fields, marshal, marshal_with
from graphon.file import helpers as file_helpers
from graphon.variables.segment_group import SegmentGroup
from graphon.variables.segments import ArrayFileSegment, FileSegment, Segment
from graphon.variables.types import SegmentType
from pydantic import BaseModel, Field
from sqlalchemy.orm import sessionmaker

from controllers.console import console_ns
from controllers.console.app.error import (
    DraftWorkflowNotExist,
)
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from controllers.web.error import InvalidArgumentError, NotFoundError
from core.app.file_access import DatabaseFileAccessController
from core.workflow.variable_prefixes import CONVERSATION_VARIABLE_NODE_ID, SYSTEM_VARIABLE_NODE_ID
from extensions.ext_database import db
from factories.file_factory import build_from_mapping, build_from_mappings
from factories.variable_factory import build_segment_with_type
from libs.login import current_user, login_required
from models import App, AppMode
from models.workflow import WorkflowDraftVariable
from services.workflow_draft_variable_service import WorkflowDraftVariableList, WorkflowDraftVariableService
from services.workflow_service import WorkflowService

logger = logging.getLogger(__name__)
_file_access_controller = DatabaseFileAccessController()
DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class WorkflowDraftVariableListQuery(BaseModel):
    page: int = Field(default=1, ge=1, le=100_000, description="Page number")
    limit: int = Field(default=20, ge=1, le=100, description="Items per page")


class WorkflowDraftVariableUpdatePayload(BaseModel):
    name: str | None = Field(default=None, description="Variable name")
    value: Any | None = Field(default=None, description="Variable value")


console_ns.schema_model(
    WorkflowDraftVariableListQuery.__name__,
    WorkflowDraftVariableListQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)
console_ns.schema_model(
    WorkflowDraftVariableUpdatePayload.__name__,
    WorkflowDraftVariableUpdatePayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)


def _convert_values_to_json_serializable_object(value: Segment):
    if isinstance(value, FileSegment):
        return value.value.model_dump()
    elif isinstance(value, ArrayFileSegment):
        return [i.model_dump() for i in value.value]
    elif isinstance(value, SegmentGroup):
        return [_convert_values_to_json_serializable_object(i) for i in value.value]
    else:
        return value.value


def _serialize_var_value(variable: WorkflowDraftVariable):
    value = variable.get_value()
    # create a copy of the value to avoid affecting the model cache.
    value = value.model_copy(deep=True)
    # Refresh the url signature before returning it to client.
    if isinstance(value, FileSegment):
        file = value.value
        file.remote_url = file.generate_url()
    elif isinstance(value, ArrayFileSegment):
        files = value.value
        for file in files:
            file.remote_url = file.generate_url()
    return _convert_values_to_json_serializable_object(value)


def _serialize_variable_type(workflow_draft_var: WorkflowDraftVariable) -> str:
    value_type = workflow_draft_var.value_type
    return value_type.exposed_type().value


def _serialize_full_content(variable: WorkflowDraftVariable) -> dict | None:
    """Serialize full_content information for large variables."""
    if not variable.is_truncated():
        return None

    variable_file = variable.variable_file
    assert variable_file is not None

    return {
        "size_bytes": variable_file.size,
        "value_type": variable_file.value_type.exposed_type().value,
        "length": variable_file.length,
        "download_url": file_helpers.get_signed_file_url(variable_file.upload_file_id, as_attachment=True),
    }


def _ensure_variable_access(
    variable: WorkflowDraftVariable | None,
    app_id: str,
    variable_id: str,
) -> WorkflowDraftVariable:
    if variable is None:
        raise NotFoundError(description=f"variable not found, id={variable_id}")
    if variable.app_id != app_id or variable.user_id != current_user.id:
        raise NotFoundError(description=f"variable not found, id={variable_id}")
    return variable


_WORKFLOW_DRAFT_VARIABLE_WITHOUT_VALUE_FIELDS = {
    "id": fields.String,
    "type": fields.String(attribute=lambda model: model.get_variable_type()),
    "name": fields.String,
    "description": fields.String,
    "selector": fields.List(fields.String, attribute=lambda model: model.get_selector()),
    "value_type": fields.String(attribute=_serialize_variable_type),
    "edited": fields.Boolean(attribute=lambda model: model.edited),
    "visible": fields.Boolean,
    "is_truncated": fields.Boolean(attribute=lambda model: model.file_id is not None),
}

_WORKFLOW_DRAFT_VARIABLE_FIELDS = {
    **_WORKFLOW_DRAFT_VARIABLE_WITHOUT_VALUE_FIELDS,
    "value": fields.Raw(attribute=_serialize_var_value),
    "full_content": fields.Raw(attribute=_serialize_full_content),
}

_WORKFLOW_DRAFT_ENV_VARIABLE_FIELDS = {
    "id": fields.String,
    "type": fields.String(attribute=lambda _: "env"),
    "name": fields.String,
    "description": fields.String,
    "selector": fields.List(fields.String, attribute=lambda model: model.get_selector()),
    "value_type": fields.String(attribute=_serialize_variable_type),
    "edited": fields.Boolean(attribute=lambda model: model.edited),
    "visible": fields.Boolean,
}

_WORKFLOW_DRAFT_ENV_VARIABLE_LIST_FIELDS = {
    "items": fields.List(fields.Nested(_WORKFLOW_DRAFT_ENV_VARIABLE_FIELDS)),
}


def _get_items(var_list: WorkflowDraftVariableList) -> list[WorkflowDraftVariable]:
    return var_list.variables


_WORKFLOW_DRAFT_VARIABLE_LIST_WITHOUT_VALUE_FIELDS = {
    "items": fields.List(fields.Nested(_WORKFLOW_DRAFT_VARIABLE_WITHOUT_VALUE_FIELDS), attribute=_get_items),
    "total": fields.Raw(),
}

_WORKFLOW_DRAFT_VARIABLE_LIST_FIELDS = {
    "items": fields.List(fields.Nested(_WORKFLOW_DRAFT_VARIABLE_FIELDS), attribute=_get_items),
}

# Register models for flask_restx to avoid dict type issues in Swagger
workflow_draft_variable_without_value_model = console_ns.model(
    "WorkflowDraftVariableWithoutValue", _WORKFLOW_DRAFT_VARIABLE_WITHOUT_VALUE_FIELDS
)

workflow_draft_variable_model = console_ns.model("WorkflowDraftVariable", _WORKFLOW_DRAFT_VARIABLE_FIELDS)

workflow_draft_env_variable_model = console_ns.model("WorkflowDraftEnvVariable", _WORKFLOW_DRAFT_ENV_VARIABLE_FIELDS)

workflow_draft_env_variable_list_fields_copy = _WORKFLOW_DRAFT_ENV_VARIABLE_LIST_FIELDS.copy()
workflow_draft_env_variable_list_fields_copy["items"] = fields.List(fields.Nested(workflow_draft_env_variable_model))
workflow_draft_env_variable_list_model = console_ns.model(
    "WorkflowDraftEnvVariableList", workflow_draft_env_variable_list_fields_copy
)

workflow_draft_variable_list_without_value_fields_copy = _WORKFLOW_DRAFT_VARIABLE_LIST_WITHOUT_VALUE_FIELDS.copy()
workflow_draft_variable_list_without_value_fields_copy["items"] = fields.List(
    fields.Nested(workflow_draft_variable_without_value_model), attribute=_get_items
)
workflow_draft_variable_list_without_value_model = console_ns.model(
    "WorkflowDraftVariableListWithoutValue", workflow_draft_variable_list_without_value_fields_copy
)

workflow_draft_variable_list_fields_copy = _WORKFLOW_DRAFT_VARIABLE_LIST_FIELDS.copy()
workflow_draft_variable_list_fields_copy["items"] = fields.List(
    fields.Nested(workflow_draft_variable_model), attribute=_get_items
)
workflow_draft_variable_list_model = console_ns.model(
    "WorkflowDraftVariableList", workflow_draft_variable_list_fields_copy
)


def _api_prerequisite[**P, R](f: Callable[P, R]) -> Callable[P, R | Response]:
    """Common prerequisites for all draft workflow variable APIs.

    It ensures the following conditions are satisfied:

    - Dify has been property setup.
    - The request user has logged in and initialized.
    - The requested app is a workflow or a chat flow.
    - The request user has the edit permission for the app.
    """

    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT, AppMode.WORKFLOW])
    @wraps(f)
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R | Response:
        return f(*args, **kwargs)

    return wrapper


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/variables")
class WorkflowVariableCollectionApi(Resource):
    @console_ns.expect(console_ns.models[WorkflowDraftVariableListQuery.__name__])
    @console_ns.doc("get_workflow_variables")
    @console_ns.doc(description="Get draft workflow variables")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.doc(params={"page": "Page number (1-100000)", "limit": "Number of items per page (1-100)"})
    @console_ns.response(
        200, "Workflow variables retrieved successfully", workflow_draft_variable_list_without_value_model
    )
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_without_value_model)
    def get(self, app_model: App):
        """
        Get draft workflow
        """
        args = WorkflowDraftVariableListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        # fetch draft workflow by app_model
        workflow_service = WorkflowService()
        workflow_exist = workflow_service.is_workflow_exist(app_model=app_model)
        if not workflow_exist:
            raise DraftWorkflowNotExist()

        # fetch draft workflow by app_model
        with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
            draft_var_srv = WorkflowDraftVariableService(
                session=session,
            )
            workflow_vars = draft_var_srv.list_variables_without_values(
                app_id=app_model.id,
                page=args.page,
                limit=args.limit,
                user_id=current_user.id,
            )

        return workflow_vars

    @console_ns.doc("delete_workflow_variables")
    @console_ns.doc(description="Delete all draft workflow variables")
    @console_ns.response(204, "Workflow variables deleted successfully")
    @_api_prerequisite
    def delete(self, app_model: App):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        draft_var_srv.delete_user_workflow_variables(app_model.id, user_id=current_user.id)
        db.session.commit()
        return Response("", 204)


def validate_node_id(node_id: str) -> None:
    if node_id in [
        CONVERSATION_VARIABLE_NODE_ID,
        SYSTEM_VARIABLE_NODE_ID,
    ]:
        # NOTE(QuantumGhost): While we store the system and conversation variables as node variables
        # with specific `node_id` in database, we still want to make the API separated. By disallowing
        # accessing system and conversation variables in `WorkflowDraftNodeVariableListApi`,
        # we mitigate the risk that user of the API depending on the implementation detail of the API.
        #
        # ref: [Hyrum's Law](https://www.hyrumslaw.com/)

        raise InvalidArgumentError(
            f"invalid node_id, please use correspond api for conversation and system variables, node_id={node_id}",
        )


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/nodes/<string:node_id>/variables")
class NodeVariableCollectionApi(Resource):
    @console_ns.doc("get_node_variables")
    @console_ns.doc(description="Get variables for a specific node")
    @console_ns.doc(params={"app_id": "Application ID", "node_id": "Node ID"})
    @console_ns.response(200, "Node variables retrieved successfully", workflow_draft_variable_list_model)
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_model)
    def get(self, app_model: App, node_id: str):
        validate_node_id(node_id)
        with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
            draft_var_srv = WorkflowDraftVariableService(
                session=session,
            )
            node_vars = draft_var_srv.list_node_variables(app_model.id, node_id, user_id=current_user.id)

        return node_vars

    @console_ns.doc("delete_node_variables")
    @console_ns.doc(description="Delete all variables for a specific node")
    @console_ns.response(204, "Node variables deleted successfully")
    @_api_prerequisite
    def delete(self, app_model: App, node_id: str):
        validate_node_id(node_id)
        srv = WorkflowDraftVariableService(db.session())
        srv.delete_node_variables(app_model.id, node_id, user_id=current_user.id)
        db.session.commit()
        return Response("", 204)


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/variables/<uuid:variable_id>")
class VariableApi(Resource):
    _PATCH_NAME_FIELD = "name"
    _PATCH_VALUE_FIELD = "value"

    @console_ns.doc("get_variable")
    @console_ns.doc(description="Get a specific workflow variable")
    @console_ns.doc(params={"app_id": "Application ID", "variable_id": "Variable ID"})
    @console_ns.response(200, "Variable retrieved successfully", workflow_draft_variable_model)
    @console_ns.response(404, "Variable not found")
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_model)
    def get(self, app_model: App, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        variable = _ensure_variable_access(
            variable=draft_var_srv.get_variable(variable_id=variable_id),
            app_id=app_model.id,
            variable_id=variable_id,
        )
        return variable

    @console_ns.doc("update_variable")
    @console_ns.doc(description="Update a workflow variable")
    @console_ns.expect(console_ns.models[WorkflowDraftVariableUpdatePayload.__name__])
    @console_ns.response(200, "Variable updated successfully", workflow_draft_variable_model)
    @console_ns.response(404, "Variable not found")
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_model)
    def patch(self, app_model: App, variable_id: str):
        # Request payload for file types:
        #
        # Local File:
        #
        #     {
        #         "type": "image",
        #         "transfer_method": "local_file",
        #         "url": "",
        #         "upload_file_id": "daded54f-72c7-4f8e-9d18-9b0abdd9f190"
        #     }
        #
        # Remote File:
        #
        #
        #     {
        #         "type": "image",
        #         "transfer_method": "remote_url",
        #         "url": "http://127.0.0.1:5001/files/1602650a-4fe4-423c-85a2-af76c083e3c4/file-preview?timestamp=1750041099&nonce=...&sign=...=",
        #         "upload_file_id": "1602650a-4fe4-423c-85a2-af76c083e3c4"
        #     }

        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        args_model = WorkflowDraftVariableUpdatePayload.model_validate(console_ns.payload or {})

        variable = _ensure_variable_access(
            variable=draft_var_srv.get_variable(variable_id=variable_id),
            app_id=app_model.id,
            variable_id=variable_id,
        )

        new_name = args_model.name
        raw_value = args_model.value
        if new_name is None and raw_value is None:
            return variable

        new_value = None
        if raw_value is not None:
            if variable.value_type == SegmentType.FILE:
                if not isinstance(raw_value, dict):
                    raise InvalidArgumentError(description=f"expected dict for file, got {type(raw_value)}")
                raw_value = build_from_mapping(
                    mapping=raw_value,
                    tenant_id=app_model.tenant_id,
                    access_controller=_file_access_controller,
                )
            elif variable.value_type == SegmentType.ARRAY_FILE:
                if not isinstance(raw_value, list):
                    raise InvalidArgumentError(description=f"expected list for files, got {type(raw_value)}")
                if len(raw_value) > 0 and not isinstance(raw_value[0], dict):
                    raise InvalidArgumentError(description=f"expected dict for files[0], got {type(raw_value)}")
                raw_value = build_from_mappings(
                    mappings=raw_value,
                    tenant_id=app_model.tenant_id,
                    access_controller=_file_access_controller,
                )
            new_value = build_segment_with_type(variable.value_type, raw_value)
        draft_var_srv.update_variable(variable, name=new_name, value=new_value)
        db.session.commit()
        return variable

    @console_ns.doc("delete_variable")
    @console_ns.doc(description="Delete a workflow variable")
    @console_ns.response(204, "Variable deleted successfully")
    @console_ns.response(404, "Variable not found")
    @_api_prerequisite
    def delete(self, app_model: App, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        variable = _ensure_variable_access(
            variable=draft_var_srv.get_variable(variable_id=variable_id),
            app_id=app_model.id,
            variable_id=variable_id,
        )
        draft_var_srv.delete_variable(variable)
        db.session.commit()
        return Response("", 204)


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/variables/<uuid:variable_id>/reset")
class VariableResetApi(Resource):
    @console_ns.doc("reset_variable")
    @console_ns.doc(description="Reset a workflow variable to its default value")
    @console_ns.doc(params={"app_id": "Application ID", "variable_id": "Variable ID"})
    @console_ns.response(200, "Variable reset successfully", workflow_draft_variable_model)
    @console_ns.response(204, "Variable reset (no content)")
    @console_ns.response(404, "Variable not found")
    @_api_prerequisite
    def put(self, app_model: App, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )

        workflow_srv = WorkflowService()
        draft_workflow = workflow_srv.get_draft_workflow(app_model)
        if draft_workflow is None:
            raise NotFoundError(
                f"Draft workflow not found, app_id={app_model.id}",
            )
        variable = _ensure_variable_access(
            variable=draft_var_srv.get_variable(variable_id=variable_id),
            app_id=app_model.id,
            variable_id=variable_id,
        )

        resetted = draft_var_srv.reset_variable(draft_workflow, variable)
        db.session.commit()
        if resetted is None:
            return Response("", 204)
        else:
            return marshal(resetted, workflow_draft_variable_model)


def _get_variable_list(app_model: App, node_id) -> WorkflowDraftVariableList:
    with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
        draft_var_srv = WorkflowDraftVariableService(
            session=session,
        )
        if node_id == CONVERSATION_VARIABLE_NODE_ID:
            draft_vars = draft_var_srv.list_conversation_variables(app_model.id, user_id=current_user.id)
        elif node_id == SYSTEM_VARIABLE_NODE_ID:
            draft_vars = draft_var_srv.list_system_variables(app_model.id, user_id=current_user.id)
        else:
            draft_vars = draft_var_srv.list_node_variables(
                app_id=app_model.id,
                node_id=node_id,
                user_id=current_user.id,
            )
    return draft_vars


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/conversation-variables")
class ConversationVariableCollectionApi(Resource):
    @console_ns.doc("get_conversation_variables")
    @console_ns.doc(description="Get conversation variables for workflow")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Conversation variables retrieved successfully", workflow_draft_variable_list_model)
    @console_ns.response(404, "Draft workflow not found")
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_model)
    def get(self, app_model: App):
        # NOTE(QuantumGhost): Prefill conversation variables into the draft variables table
        # so their IDs can be returned to the caller.
        workflow_srv = WorkflowService()
        draft_workflow = workflow_srv.get_draft_workflow(app_model)
        if draft_workflow is None:
            raise NotFoundError(description=f"draft workflow not found, id={app_model.id}")
        draft_var_srv = WorkflowDraftVariableService(db.session())
        draft_var_srv.prefill_conversation_variable_default_values(draft_workflow, user_id=current_user.id)
        db.session.commit()
        return _get_variable_list(app_model, CONVERSATION_VARIABLE_NODE_ID)


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/system-variables")
class SystemVariableCollectionApi(Resource):
    @console_ns.doc("get_system_variables")
    @console_ns.doc(description="Get system variables for workflow")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "System variables retrieved successfully", workflow_draft_variable_list_model)
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_model)
    def get(self, app_model: App):
        return _get_variable_list(app_model, SYSTEM_VARIABLE_NODE_ID)


@console_ns.route("/apps/<uuid:app_id>/workflows/draft/environment-variables")
class EnvironmentVariableCollectionApi(Resource):
    @console_ns.doc("get_environment_variables")
    @console_ns.doc(description="Get environment variables for workflow")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Environment variables retrieved successfully")
    @console_ns.response(404, "Draft workflow not found")
    @_api_prerequisite
    def get(self, app_model: App):
        """
        Get draft workflow
        """
        # fetch draft workflow by app_model
        workflow_service = WorkflowService()
        workflow = workflow_service.get_draft_workflow(app_model=app_model)
        if workflow is None:
            raise DraftWorkflowNotExist()

        env_vars = workflow.environment_variables
        env_vars_list = []
        for v in env_vars:
            env_vars_list.append(
                {
                    "id": v.id,
                    "type": "env",
                    "name": v.name,
                    "description": v.description,
                    "selector": v.selector,
                    "value_type": v.value_type.exposed_type().value,
                    "value": v.value,
                    # Do not track edited for env vars.
                    "edited": False,
                    "visible": True,
                    "editable": True,
                }
            )

        return {"items": env_vars_list}

```

### api/controllers/console/app/workflow_run.py
```py
from datetime import UTC, datetime, timedelta
from typing import Literal, TypedDict, cast

from flask import request
from flask_restx import Resource, fields, marshal_with
from graphon.entities.pause_reason import HumanInputRequired
from graphon.enums import WorkflowExecutionStatus
from pydantic import BaseModel, Field, field_validator
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from controllers.web.error import NotFoundError
from core.workflow.human_input_forms import load_form_tokens_by_form_id as _load_form_tokens_by_form_id
from extensions.ext_database import db
from fields.end_user_fields import simple_end_user_fields
from fields.member_fields import simple_account_fields
from fields.workflow_run_fields import (
    advanced_chat_workflow_run_for_list_fields,
    advanced_chat_workflow_run_pagination_fields,
    workflow_run_count_fields,
    workflow_run_detail_fields,
    workflow_run_for_list_fields,
    workflow_run_node_execution_fields,
    workflow_run_node_execution_list_fields,
    workflow_run_pagination_fields,
)
from libs.archive_storage import ArchiveStorageNotConfiguredError, get_archive_storage
from libs.custom_inputs import time_duration
from libs.helper import uuid_value
from libs.login import current_user, login_required
from models import Account, App, AppMode, EndUser, WorkflowArchiveLog, WorkflowRunTriggeredFrom
from models.workflow import WorkflowRun
from repositories.factory import DifyAPIRepositoryFactory
from services.retention.workflow_run.constants import ARCHIVE_BUNDLE_NAME
from services.workflow_run_service import WorkflowRunService


def _build_backstage_input_url(form_token: str | None) -> str | None:
    if not form_token:
        return None
    base_url = dify_config.APP_WEB_URL
    if not base_url:
        return None
    return f"{base_url.rstrip('/')}/form/{form_token}"


# Workflow run status choices for filtering
WORKFLOW_RUN_STATUS_CHOICES = ["running", "succeeded", "failed", "stopped", "partial-succeeded"]
EXPORT_SIGNED_URL_EXPIRE_SECONDS = 3600

# Register models for flask_restx to avoid dict type issues in Swagger
# Register in dependency order: base models first, then dependent models

# Base models
simple_account_model = console_ns.model("SimpleAccount", simple_account_fields)

simple_end_user_model = console_ns.model("SimpleEndUser", simple_end_user_fields)

# Models that depend on simple_account_fields
workflow_run_for_list_fields_copy = workflow_run_for_list_fields.copy()
workflow_run_for_list_fields_copy["created_by_account"] = fields.Nested(
    simple_account_model, attribute="created_by_account", allow_null=True
)
workflow_run_for_list_model = console_ns.model("WorkflowRunForList", workflow_run_for_list_fields_copy)

advanced_chat_workflow_run_for_list_fields_copy = advanced_chat_workflow_run_for_list_fields.copy()
advanced_chat_workflow_run_for_list_fields_copy["created_by_account"] = fields.Nested(
    simple_account_model, attribute="created_by_account", allow_null=True
)
advanced_chat_workflow_run_for_list_model = console_ns.model(
    "AdvancedChatWorkflowRunForList", advanced_chat_workflow_run_for_list_fields_copy
)

workflow_run_detail_fields_copy = workflow_run_detail_fields.copy()
workflow_run_detail_fields_copy["created_by_account"] = fields.Nested(
    simple_account_model, attribute="created_by_account", allow_null=True
)
workflow_run_detail_fields_copy["created_by_end_user"] = fields.Nested(
    simple_end_user_model, attribute="created_by_end_user", allow_null=True
)
workflow_run_detail_model = console_ns.model("WorkflowRunDetail", workflow_run_detail_fields_copy)

workflow_run_node_execution_fields_copy = workflow_run_node_execution_fields.copy()
workflow_run_node_execution_fields_copy["created_by_account"] = fields.Nested(
    simple_account_model, attribute="created_by_account", allow_null=True
)
workflow_run_node_execution_fields_copy["created_by_end_user"] = fields.Nested(
    simple_end_user_model, attribute="created_by_end_user", allow_null=True
)
workflow_run_node_execution_model = console_ns.model(
    "WorkflowRunNodeExecution", workflow_run_node_execution_fields_copy
)

# Simple models without nested dependencies
workflow_run_count_model = console_ns.model("WorkflowRunCount", workflow_run_count_fields)

# Pagination models that depend on list models
advanced_chat_workflow_run_pagination_fields_copy = advanced_chat_workflow_run_pagination_fields.copy()
advanced_chat_workflow_run_pagination_fields_copy["data"] = fields.List(
    fields.Nested(advanced_chat_workflow_run_for_list_model), attribute="data"
)
advanced_chat_workflow_run_pagination_model = console_ns.model(
    "AdvancedChatWorkflowRunPagination", advanced_chat_workflow_run_pagination_fields_copy
)

workflow_run_pagination_fields_copy = workflow_run_pagination_fields.copy()
workflow_run_pagination_fields_copy["data"] = fields.List(fields.Nested(workflow_run_for_list_model), attribute="data")
workflow_run_pagination_model = console_ns.model("WorkflowRunPagination", workflow_run_pagination_fields_copy)

workflow_run_node_execution_list_fields_copy = workflow_run_node_execution_list_fields.copy()
workflow_run_node_execution_list_fields_copy["data"] = fields.List(fields.Nested(workflow_run_node_execution_model))
workflow_run_node_execution_list_model = console_ns.model(
    "WorkflowRunNodeExecutionList", workflow_run_node_execution_list_fields_copy
)

workflow_run_export_fields = console_ns.model(
    "WorkflowRunExport",
    {
        "status": fields.String(description="Export status: success/failed"),
        "presigned_url": fields.String(description="Pre-signed URL for download", required=False),
        "presigned_url_expires_at": fields.String(description="Pre-signed URL expiration time", required=False),
    },
)

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class WorkflowRunListQuery(BaseModel):
    last_id: str | None = Field(default=None, description="Last run ID for pagination")
    limit: int = Field(default=20, ge=1, le=100, description="Number of items per page (1-100)")
    status: Literal["running", "succeeded", "failed", "stopped", "partial-succeeded"] | None = Field(
        default=None, description="Workflow run status filter"
    )
    triggered_from: Literal["debugging", "app-run"] | None = Field(
        default=None, description="Filter by trigger source: debugging or app-run"
    )

    @field_validator("last_id")
    @classmethod
    def validate_last_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class WorkflowRunCountQuery(BaseModel):
    status: Literal["running", "succeeded", "failed", "stopped", "partial-succeeded"] | None = Field(
        default=None, description="Workflow run status filter"
    )
    time_range: str | None = Field(default=None, description="Time range filter (e.g., 7d, 4h, 30m, 30s)")
    triggered_from: Literal["debugging", "app-run"] | None = Field(
        default=None, description="Filter by trigger source: debugging or app-run"
    )

    @field_validator("time_range")
    @classmethod
    def validate_time_range(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return time_duration(value)


console_ns.schema_model(
    WorkflowRunListQuery.__name__, WorkflowRunListQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)
console_ns.schema_model(
    WorkflowRunCountQuery.__name__,
    WorkflowRunCountQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)


class HumanInputPauseTypeResponse(TypedDict):
    type: Literal["human_input"]
    form_id: str
    backstage_input_url: str | None


class PausedNodeResponse(TypedDict):
    node_id: str
    node_title: str
    pause_type: HumanInputPauseTypeResponse


class WorkflowPauseDetailsResponse(TypedDict):
    paused_at: str | None
    paused_nodes: list[PausedNodeResponse]


@console_ns.route("/apps/<uuid:app_id>/advanced-chat/workflow-runs")
class AdvancedChatAppWorkflowRunListApi(Resource):
    @console_ns.doc("get_advanced_chat_workflow_runs")
    @console_ns.doc(description="Get advanced chat workflow run list")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.doc(params={"last_id": "Last run ID for pagination", "limit": "Number of items per page (1-100)"})
    @console_ns.doc(
        params={"status": "Filter by status (optional): running, succeeded, failed, stopped, partial-succeeded"}
    )
    @console_ns.doc(
        params={"triggered_from": "Filter by trigger source (optional): debugging or app-run. Default: debugging"}
    )
    @console_ns.expect(console_ns.models[WorkflowRunListQuery.__name__])
    @console_ns.response(200, "Workflow runs retrieved successfully", advanced_chat_workflow_run_pagination_model)
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT])
    @marshal_with(advanced_chat_workflow_run_pagination_model)
    def get(self, app_model: App):
        """
        Get advanced chat app workflow run list
        """
        args_model = WorkflowRunListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        args = args_model.model_dump(exclude_none=True)

        # Default to DEBUGGING if not specified
        triggered_from = (
            WorkflowRunTriggeredFrom(args_model.triggered_from)
            if args_model.triggered_from
            else WorkflowRunTriggeredFrom.DEBUGGING
        )

        workflow_run_service = WorkflowRunService()
        result = workflow_run_service.get_paginate_advanced_chat_workflow_runs(
            app_model=app_model, args=args, triggered_from=triggered_from
        )

        return result


@console_ns.route("/apps/<uuid:app_id>/workflow-runs/<uuid:run_id>/export")
class WorkflowRunExportApi(Resource):
    @console_ns.doc("get_workflow_run_export_url")
    @console_ns.doc(description="Generate a download URL for an archived workflow run.")
    @console_ns.doc(params={"app_id": "Application ID", "run_id": "Workflow run ID"})
    @console_ns.response(200, "Export URL generated", workflow_run_export_fields)
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model()
    def get(self, app_model: App, run_id: str):
        tenant_id = str(app_model.tenant_id)
        app_id = str(app_model.id)
        run_id_str = str(run_id)

        run_created_at = db.session.scalar(
            select(WorkflowArchiveLog.run_created_at)
            .where(
                WorkflowArchiveLog.tenant_id == tenant_id,
                WorkflowArchiveLog.app_id == app_id,
                WorkflowArchiveLog.workflow_run_id == run_id_str,
            )
            .limit(1)
        )
        if not run_created_at:
            return {"code": "archive_log_not_found", "message": "workflow run archive not found"}, 404

        prefix = (
            f"{tenant_id}/app_id={app_id}/year={run_created_at.strftime('%Y')}/"
            f"month={run_created_at.strftime('%m')}/workflow_run_id={run_id_str}"
        )
        archive_key = f"{prefix}/{ARCHIVE_BUNDLE_NAME}"

        try:
            archive_storage = get_archive_storage()
        except ArchiveStorageNotConfiguredError as e:
            return {"code": "archive_storage_not_configured", "message": str(e)}, 500

        presigned_url = archive_storage.generate_presigned_url(
            archive_key,
            expires_in=EXPORT_SIGNED_URL_EXPIRE_SECONDS,
        )
        expires_at = datetime.now(UTC) + timedelta(seconds=EXPORT_SIGNED_URL_EXPIRE_SECONDS)
        return {
            "status": "success",
            "presigned_url": presigned_url,
            "presigned_url_expires_at": expires_at.isoformat(),
        }, 200


@console_ns.route("/apps/<uuid:app_id>/advanced-chat/workflow-runs/count")
class AdvancedChatAppWorkflowRunCountApi(Resource):
    @console_ns.doc("get_advanced_chat_workflow_runs_count")
    @console_ns.doc(description="Get advanced chat workflow runs count statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.doc(
        params={"status": "Filter by status (optional): running, succeeded, failed, stopped, partial-succeeded"}
    )
    @console_ns.doc(
        params={
            "time_range": (
                "Filter by time range (optional): e.g., 7d (7 days), 4h (4 hours), "
                "30m (30 minutes), 30s (30 seconds). Filters by created_at field."
            )
        }
    )
    @console_ns.doc(
        params={"triggered_from": "Filter by trigger source (optional): debugging or app-run. Default: debugging"}
    )
    @console_ns.response(200, "Workflow runs count retrieved successfully", workflow_run_count_model)
    @console_ns.expect(console_ns.models[WorkflowRunCountQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT])
    @marshal_with(workflow_run_count_model)
    def get(self, app_model: App):
        """
        Get advanced chat workflow runs count statistics
        """
        args_model = WorkflowRunCountQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        args = args_model.model_dump(exclude_none=True)

        # Default to DEBUGGING if not specified
        triggered_from = (
            WorkflowRunTriggeredFrom(args_model.triggered_from)
            if args_model.triggered_from
            else WorkflowRunTriggeredFrom.DEBUGGING
        )

        workflow_run_service = WorkflowRunService()
        result = workflow_run_service.get_workflow_runs_count(
            app_model=app_model,
            status=args.get("status"),
            time_range=args.get("time_range"),
            triggered_from=triggered_from,
        )

        return result


@console_ns.route("/apps/<uuid:app_id>/workflow-runs")
class WorkflowRunListApi(Resource):
    @console_ns.doc("get_workflow_runs")
    @console_ns.doc(description="Get workflow run list")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.doc(params={"last_id": "Last run ID for pagination", "limit": "Number of items per page (1-100)"})
    @console_ns.doc(
        params={"status": "Filter by status (optional): running, succeeded, failed, stopped, partial-succeeded"}
    )
    @console_ns.doc(
        params={"triggered_from": "Filter by trigger source (optional): debugging or app-run. Default: debugging"}
    )
    @console_ns.response(200, "Workflow runs retrieved successfully", workflow_run_pagination_model)
    @console_ns.expect(console_ns.models[WorkflowRunListQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT, AppMode.WORKFLOW])
    @marshal_with(workflow_run_pagination_model)
    def get(self, app_model: App):
        """
        Get workflow run list
        """
        args_model = WorkflowRunListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        args = args_model.model_dump(exclude_none=True)

        # Default to DEBUGGING for workflow if not specified (backward compatibility)
        triggered_from = (
            WorkflowRunTriggeredFrom(args_model.triggered_from)
            if args_model.triggered_from
            else WorkflowRunTriggeredFrom.DEBUGGING
        )

        workflow_run_service = WorkflowRunService()
        result = workflow_run_service.get_paginate_workflow_runs(
            app_model=app_model, args=args, triggered_from=triggered_from
        )

        return result


@console_ns.route("/apps/<uuid:app_id>/workflow-runs/count")
class WorkflowRunCountApi(Resource):
    @console_ns.doc("get_workflow_runs_count")
    @console_ns.doc(description="Get workflow runs count statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.doc(
        params={"status": "Filter by status (optional): running, succeeded, failed, stopped, partial-succeeded"}
    )
    @console_ns.doc(
        params={
            "time_range": (
                "Filter by time range (optional): e.g., 7d (7 days), 4h (4 hours), "
                "30m (30 minutes), 30s (30 seconds). Filters by created_at field."
            )
        }
    )
    @console_ns.doc(
        params={"triggered_from": "Filter by trigger source (optional): debugging or app-run. Default: debugging"}
    )
    @console_ns.response(200, "Workflow runs count retrieved successfully", workflow_run_count_model)
    @console_ns.expect(console_ns.models[WorkflowRunCountQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT, AppMode.WORKFLOW])
    @marshal_with(workflow_run_count_model)
    def get(self, app_model: App):
        """
        Get workflow runs count statistics
        """
        args_model = WorkflowRunCountQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        args = args_model.model_dump(exclude_none=True)

        # Default to DEBUGGING for workflow if not specified (backward compatibility)
        triggered_from = (
            WorkflowRunTriggeredFrom(args_model.triggered_from)
            if args_model.triggered_from
            else WorkflowRunTriggeredFrom.DEBUGGING
        )

        workflow_run_service = WorkflowRunService()
        result = workflow_run_service.get_workflow_runs_count(
            app_model=app_model,
            status=args.get("status"),
            time_range=args.get("time_range"),
            triggered_from=triggered_from,
        )

        return result


@console_ns.route("/apps/<uuid:app_id>/workflow-runs/<uuid:run_id>")
class WorkflowRunDetailApi(Resource):
    @console_ns.doc("get_workflow_run_detail")
    @console_ns.doc(description="Get workflow run detail")
    @console_ns.doc(params={"app_id": "Application ID", "run_id": "Workflow run ID"})
    @console_ns.response(200, "Workflow run detail retrieved successfully", workflow_run_detail_model)
    @console_ns.response(404, "Workflow run not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT, AppMode.WORKFLOW])
    @marshal_with(workflow_run_detail_model)
    def get(self, app_model: App, run_id):
        """
        Get workflow run detail
        """
        run_id = str(run_id)

        workflow_run_service = WorkflowRunService()
        workflow_run = workflow_run_service.get_workflow_run(app_model=app_model, run_id=run_id)

        return workflow_run


@console_ns.route("/apps/<uuid:app_id>/workflow-runs/<uuid:run_id>/node-executions")
class WorkflowRunNodeExecutionListApi(Resource):
    @console_ns.doc("get_workflow_run_node_executions")
    @console_ns.doc(description="Get workflow run node execution list")
    @console_ns.doc(params={"app_id": "Application ID", "run_id": "Workflow run ID"})
    @console_ns.response(200, "Node executions retrieved successfully", workflow_run_node_execution_list_model)
    @console_ns.response(404, "Workflow run not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.ADVANCED_CHAT, AppMode.WORKFLOW])
    @marshal_with(workflow_run_node_execution_list_model)
    def get(self, app_model: App, run_id):
        """
        Get workflow run node execution list
        """
        run_id = str(run_id)

        workflow_run_service = WorkflowRunService()
        user = cast("Account | EndUser", current_user)
        node_executions = workflow_run_service.get_workflow_run_node_executions(
            app_model=app_model,
            run_id=run_id,
            user=user,
        )

        return {"data": node_executions}


@console_ns.route("/workflow/<string:workflow_run_id>/pause-details")
class ConsoleWorkflowPauseDetailsApi(Resource):
    """Console API for getting workflow pause details."""

    @setup_required
    @login_required
    @account_initialization_required
    def get(self, workflow_run_id: str):
        """
        Get workflow pause details.

        GET /console/api/workflow/<workflow_run_id>/pause-details

        Returns information about why and where the workflow is paused.
        """

        # Query WorkflowRun to determine if workflow is suspended
        session_maker = sessionmaker(bind=db.engine)
        workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker=session_maker)

        workflow_run = db.session.get(WorkflowRun, workflow_run_id)
        if not workflow_run:
            raise NotFoundError("Workflow run not found")

        if workflow_run.tenant_id != current_user.current_tenant_id:
            raise NotFoundError("Workflow run not found")

        # Check if workflow is suspended
        is_paused = workflow_run.status == WorkflowExecutionStatus.PAUSED
        if not is_paused:
            empty_response: WorkflowPauseDetailsResponse = {
                "paused_at": None,
                "paused_nodes": [],
            }
            return empty_response, 200

        pause_entity = workflow_run_repo.get_workflow_pause(workflow_run_id)
        pause_reasons = pause_entity.get_pause_reasons() if pause_entity else []
        form_tokens_by_form_id = _load_form_tokens_by_form_id(
            [reason.form_id for reason in pause_reasons if isinstance(reason, HumanInputRequired)]
        )

        # Build response
        paused_at = pause_entity.paused_at if pause_entity else None
        paused_nodes: list[PausedNodeResponse] = []
        response: WorkflowPauseDetailsResponse = {
            "paused_at": paused_at.isoformat() + "Z" if paused_at else None,
            "paused_nodes": paused_nodes,
        }

        for reason in pause_reasons:
            if isinstance(reason, HumanInputRequired):
                paused_nodes.append(
                    {
                        "node_id": reason.node_id,
                        "node_title": reason.node_title,
                        "pause_type": {
                            "type": "human_input",
                            "form_id": reason.form_id,
                            "backstage_input_url": _build_backstage_input_url(
                                form_tokens_by_form_id.get(reason.form_id)
                            ),
                        },
                    }
                )
            else:
                raise AssertionError("unimplemented.")

        return response, 200

```

### api/controllers/console/app/conversation.py
```py
from typing import Literal

import sqlalchemy as sa
from flask import abort, request
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel, Field, field_validator
from sqlalchemy import func, or_
from sqlalchemy.orm import selectinload
from werkzeug.exceptions import NotFound

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from fields.raws import FilesContainedField
from libs.datetime_utils import naive_utc_now, parse_time_range
from libs.helper import TimestampField
from libs.login import current_account_with_tenant, login_required
from models import Conversation, EndUser, Message, MessageAnnotation
from models.model import AppMode
from services.conversation_service import ConversationService
from services.errors.conversation import ConversationNotExistsError

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class BaseConversationQuery(BaseModel):
    keyword: str | None = Field(default=None, description="Search keyword")
    start: str | None = Field(default=None, description="Start date (YYYY-MM-DD HH:MM)")
    end: str | None = Field(default=None, description="End date (YYYY-MM-DD HH:MM)")
    annotation_status: Literal["annotated", "not_annotated", "all"] = Field(
        default="all", description="Annotation status filter"
    )
    page: int = Field(default=1, ge=1, le=99999, description="Page number")
    limit: int = Field(default=20, ge=1, le=100, description="Page size (1-100)")

    @field_validator("start", "end", mode="before")
    @classmethod
    def blank_to_none(cls, value: str | None) -> str | None:
        if value == "":
            return None
        return value


class CompletionConversationQuery(BaseConversationQuery):
    pass


class ChatConversationQuery(BaseConversationQuery):
    sort_by: Literal["created_at", "-created_at", "updated_at", "-updated_at"] = Field(
        default="-updated_at", description="Sort field and direction"
    )


console_ns.schema_model(
    CompletionConversationQuery.__name__,
    CompletionConversationQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)
console_ns.schema_model(
    ChatConversationQuery.__name__,
    ChatConversationQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)

# Register models for flask_restx to avoid dict type issues in Swagger
# Register in dependency order: base models first, then dependent models

# Base models
simple_account_model = console_ns.model(
    "SimpleAccount",
    {
        "id": fields.String,
        "name": fields.String,
        "email": fields.String,
    },
)

feedback_stat_model = console_ns.model(
    "FeedbackStat",
    {
        "like": fields.Integer,
        "dislike": fields.Integer,
    },
)

status_count_model = console_ns.model(
    "StatusCount",
    {
        "success": fields.Integer,
        "failed": fields.Integer,
        "partial_success": fields.Integer,
        "paused": fields.Integer,
    },
)

message_file_model = console_ns.model(
    "MessageFile",
    {
        "id": fields.String,
        "filename": fields.String,
        "type": fields.String,
        "url": fields.String,
        "mime_type": fields.String,
        "size": fields.Integer,
        "transfer_method": fields.String,
        "belongs_to": fields.String(default="user"),
        "upload_file_id": fields.String(default=None),
    },
)

agent_thought_model = console_ns.model(
    "AgentThought",
    {
        "id": fields.String,
        "chain_id": fields.String,
        "message_id": fields.String,
        "position": fields.Integer,
        "thought": fields.String,
        "tool": fields.String,
        "tool_labels": fields.Raw,
        "tool_input": fields.String,
        "created_at": TimestampField,
        "observation": fields.String,
        "files": fields.List(fields.String),
    },
)

simple_model_config_model = console_ns.model(
    "SimpleModelConfig",
    {
        "model": fields.Raw(attribute="model_dict"),
        "pre_prompt": fields.String,
    },
)

model_config_model = console_ns.model(
    "ModelConfig",
    {
        "opening_statement": fields.String,
        "suggested_questions": fields.Raw,
        "model": fields.Raw,
        "user_input_form": fields.Raw,
        "pre_prompt": fields.String,
        "agent_mode": fields.Raw,
    },
)

# Models that depend on simple_account_model
feedback_model = console_ns.model(
    "Feedback",
    {
        "rating": fields.String,
        "content": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account": fields.Nested(simple_account_model, allow_null=True),
    },
)

annotation_model = console_ns.model(
    "Annotation",
    {
        "id": fields.String,
        "question": fields.String,
        "content": fields.String,
        "account": fields.Nested(simple_account_model, allow_null=True),
        "created_at": TimestampField,
    },
)

annotation_hit_history_model = console_ns.model(
    "AnnotationHitHistory",
    {
        "annotation_id": fields.String(attribute="id"),
        "annotation_create_account": fields.Nested(simple_account_model, allow_null=True),
        "created_at": TimestampField,
    },
)


class MessageTextField(fields.Raw):
    def format(self, value):
        return value[0]["text"] if value else ""


# Simple message detail model
simple_message_detail_model = console_ns.model(
    "SimpleMessageDetail",
    {
        "inputs": FilesContainedField,
        "query": fields.String,
        "message": MessageTextField,
        "answer": fields.String,
    },
)

# Message detail model that depends on multiple models
message_detail_model = console_ns.model(
    "MessageDetail",
    {
        "id": fields.String,
        "conversation_id": fields.String,
        "inputs": FilesContainedField,
        "query": fields.String,
        "message": fields.Raw,
        "message_tokens": fields.Integer,
        "answer": fields.String(attribute="re_sign_file_url_answer"),
        "answer_tokens": fields.Integer,
        "provider_response_latency": fields.Float,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account_id": fields.String,
        "feedbacks": fields.List(fields.Nested(feedback_model)),
        "workflow_run_id": fields.String,
        "annotation": fields.Nested(annotation_model, allow_null=True),
        "annotation_hit_history": fields.Nested(annotation_hit_history_model, allow_null=True),
        "created_at": TimestampField,
        "agent_thoughts": fields.List(fields.Nested(agent_thought_model)),
        "message_files": fields.List(fields.Nested(message_file_model)),
        "metadata": fields.Raw(attribute="message_metadata_dict"),
        "status": fields.String,
        "error": fields.String,
        "parent_message_id": fields.String,
    },
)

# Conversation models
conversation_fields_model = console_ns.model(
    "Conversation",
    {
        "id": fields.String,
        "status": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_end_user_session_id": fields.String(),
        "from_account_id": fields.String,
        "from_account_name": fields.String,
        "read_at": TimestampField,
        "created_at": TimestampField,
        "updated_at": TimestampField,
        "annotation": fields.Nested(annotation_model, allow_null=True),
        "model_config": fields.Nested(simple_model_config_model),
        "user_feedback_stats": fields.Nested(feedback_stat_model),
        "admin_feedback_stats": fields.Nested(feedback_stat_model),
        "message": fields.Nested(simple_message_detail_model, attribute="first_message"),
    },
)

conversation_pagination_model = console_ns.model(
    "ConversationPagination",
    {
        "page": fields.Integer,
        "limit": fields.Integer(attribute="per_page"),
        "total": fields.Integer,
        "has_more": fields.Boolean(attribute="has_next"),
        "data": fields.List(fields.Nested(conversation_fields_model), attribute="items"),
    },
)

conversation_message_detail_model = console_ns.model(
    "ConversationMessageDetail",
    {
        "id": fields.String,
        "status": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account_id": fields.String,
        "created_at": TimestampField,
        "model_config": fields.Nested(model_config_model),
        "message": fields.Nested(message_detail_model, attribute="first_message"),
    },
)

conversation_with_summary_model = console_ns.model(
    "ConversationWithSummary",
    {
        "id": fields.String,
        "status": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_end_user_session_id": fields.String,
        "from_account_id": fields.String,
        "from_account_name": fields.String,
        "name": fields.String,
        "summary": fields.String(attribute="summary_or_query"),
        "read_at": TimestampField,
        "created_at": TimestampField,
        "updated_at": TimestampField,
        "annotated": fields.Boolean,
        "model_config": fields.Nested(simple_model_config_model),
        "message_count": fields.Integer,
        "user_feedback_stats": fields.Nested(feedback_stat_model),
        "admin_feedback_stats": fields.Nested(feedback_stat_model),
        "status_count": fields.Nested(status_count_model),
    },
)

conversation_with_summary_pagination_model = console_ns.model(
    "ConversationWithSummaryPagination",
    {
        "page": fields.Integer,
        "limit": fields.Integer(attribute="per_page"),
        "total": fields.Integer,
        "has_more": fields.Boolean(attribute="has_next"),
        "data": fields.List(fields.Nested(conversation_with_summary_model), attribute="items"),
    },
)

conversation_detail_model = console_ns.model(
    "ConversationDetail",
    {
        "id": fields.String,
        "status": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account_id": fields.String,
        "created_at": TimestampField,
        "updated_at": TimestampField,
        "annotated": fields.Boolean,
        "introduction": fields.String,
        "model_config": fields.Nested(model_config_model),
        "message_count": fields.Integer,
        "user_feedback_stats": fields.Nested(feedback_stat_model),
        "admin_feedback_stats": fields.Nested(feedback_stat_model),
    },
)


@console_ns.route("/apps/<uuid:app_id>/completion-conversations")
class CompletionConversationApi(Resource):
    @console_ns.doc("list_completion_conversations")
    @console_ns.doc(description="Get completion conversations with pagination and filtering")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[CompletionConversationQuery.__name__])
    @console_ns.response(200, "Success", conversation_pagination_model)
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    @marshal_with(conversation_pagination_model)
    @edit_permission_required
    def get(self, app_model):
        current_user, _ = current_account_with_tenant()
        args = CompletionConversationQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        query = sa.select(Conversation).where(
            Conversation.app_id == app_model.id, Conversation.mode == "completion", Conversation.is_deleted.is_(False)
        )

        if args.keyword:
            from libs.helper import escape_like_pattern

            escaped_keyword = escape_like_pattern(args.keyword)
            query = query.join(Message, Message.conversation_id == Conversation.id).where(
                or_(
                    Message.query.ilike(f"%{escaped_keyword}%", escape="\\"),
                    Message.answer.ilike(f"%{escaped_keyword}%", escape="\\"),
                )
            )

        account = current_user
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            query = query.where(Conversation.created_at >= start_datetime_utc)

        if end_datetime_utc:
            end_datetime_utc = end_datetime_utc.replace(second=59)
            query = query.where(Conversation.created_at < end_datetime_utc)

        # FIXME, the type ignore in this file
        if args.annotation_status == "annotated":
            query = (
                query.options(selectinload(Conversation.message_annotations))  # type: ignore[arg-type]
                .join(  # type: ignore
                    MessageAnnotation, MessageAnnotation.conversation_id == Conversation.id
                )
                .distinct()
            )
        elif args.annotation_status == "not_annotated":
            query = (
                query.outerjoin(MessageAnnotation, MessageAnnotation.conversation_id == Conversation.id)
                .group_by(Conversation.id)
                .having(func.count(MessageAnnotation.id) == 0)
            )

        query = query.order_by(Conversation.created_at.desc())

        conversations = db.paginate(query, page=args.page, per_page=args.limit, error_out=False)

        return conversations


@console_ns.route("/apps/<uuid:app_id>/completion-conversations/<uuid:conversation_id>")
class CompletionConversationDetailApi(Resource):
    @console_ns.doc("get_completion_conversation")
    @console_ns.doc(description="Get completion conversation details with messages")
    @console_ns.doc(params={"app_id": "Application ID", "conversation_id": "Conversation ID"})
    @console_ns.response(200, "Success", conversation_message_detail_model)
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Conversation not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    @marshal_with(conversation_message_detail_model)
    @edit_permission_required
    def get(self, app_model, conversation_id):
        conversation_id = str(conversation_id)

        return _get_conversation(app_model, conversation_id)

    @console_ns.doc("delete_completion_conversation")
    @console_ns.doc(description="Delete a completion conversation")
    @console_ns.doc(params={"app_id": "Application ID", "conversation_id": "Conversation ID"})
    @console_ns.response(204, "Conversation deleted successfully")
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Conversation not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    @edit_permission_required
    def delete(self, app_model, conversation_id):
        current_user, _ = current_account_with_tenant()
        conversation_id = str(conversation_id)

        try:
            ConversationService.delete(app_model, conversation_id, current_user)
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")

        return {"result": "success"}, 204


@console_ns.route("/apps/<uuid:app_id>/chat-conversations")
class ChatConversationApi(Resource):
    @console_ns.doc("list_chat_conversations")
    @console_ns.doc(description="Get chat conversations with pagination, filtering and summary")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[ChatConversationQuery.__name__])
    @console_ns.response(200, "Success", conversation_with_summary_pagination_model)
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    @marshal_with(conversation_with_summary_pagination_model)
    @edit_permission_required
    def get(self, app_model):
        current_user, _ = current_account_with_tenant()
        args = ChatConversationQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        subquery = (
            sa.select(Conversation.id.label("conversation_id"), EndUser.session_id.label("from_end_user_session_id"))
            .outerjoin(EndUser, Conversation.from_end_user_id == EndUser.id)
            .subquery()
        )

        query = sa.select(Conversation).where(Conversation.app_id == app_model.id, Conversation.is_deleted.is_(False))

        if args.keyword:
            from libs.helper import escape_like_pattern

            escaped_keyword = escape_like_pattern(args.keyword)
            keyword_filter = f"%{escaped_keyword}%"
            query = (
                query.join(
                    Message,
                    Message.conversation_id == Conversation.id,
                )
                .join(subquery, subquery.c.conversation_id == Conversation.id)
                .where(
                    or_(
                        Message.query.ilike(keyword_filter, escape="\\"),
                        Message.answer.ilike(keyword_filter, escape="\\"),
                        Conversation.name.ilike(keyword_filter, escape="\\"),
                        Conversation.introduction.ilike(keyword_filter, escape="\\"),
                        subquery.c.from_end_user_session_id.ilike(keyword_filter, escape="\\"),
                    ),
                )
                .group_by(Conversation.id)
            )

        account = current_user
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            match args.sort_by:
                case "updated_at" | "-updated_at":
                    query = query.where(Conversation.updated_at >= start_datetime_utc)
                case "created_at" | "-created_at" | _:
                    query = query.where(Conversation.created_at >= start_datetime_utc)

        if end_datetime_utc:
            end_datetime_utc = end_datetime_utc.replace(second=59)
            match args.sort_by:
                case "updated_at" | "-updated_at":
                    query = query.where(Conversation.updated_at <= end_datetime_utc)
                case "created_at" | "-created_at" | _:
                    query = query.where(Conversation.created_at <= end_datetime_utc)

        match args.annotation_status:
            case "annotated":
                query = (
                    query.options(selectinload(Conversation.message_annotations))  # type: ignore[arg-type]
                    .join(  # type: ignore
                        MessageAnnotation, MessageAnnotation.conversation_id == Conversation.id
                    )
                    .distinct()
                )
            case "not_annotated":
                query = (
                    query.outerjoin(MessageAnnotation, MessageAnnotation.conversation_id == Conversation.id)
                    .group_by(Conversation.id)
                    .having(func.count(MessageAnnotation.id) == 0)
                )
            case "all":
                pass

        if app_model.mode == AppMode.ADVANCED_CHAT:
            query = query.where(Conversation.invoke_from != InvokeFrom.DEBUGGER)

        match args.sort_by:
            case "created_at":
                query = query.order_by(Conversation.created_at.asc())
            case "-created_at":
                query = query.order_by(Conversation.created_at.desc())
            case "updated_at":
                query = query.order_by(Conversation.updated_at.asc())
            case "-updated_at":
                query = query.order_by(Conversation.updated_at.desc())
            case _:
                query = query.order_by(Conversation.created_at.desc())

        conversations = db.paginate(query, page=args.page, per_page=args.limit, error_out=False)

        return conversations


@console_ns.route("/apps/<uuid:app_id>/chat-conversations/<uuid:conversation_id>")
class ChatConversationDetailApi(Resource):
    @console_ns.doc("get_chat_conversation")
    @console_ns.doc(description="Get chat conversation details")
    @console_ns.doc(params={"app_id": "Application ID", "conversation_id": "Conversation ID"})
    @console_ns.response(200, "Success", conversation_detail_model)
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Conversation not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    @marshal_with(conversation_detail_model)
    @edit_permission_required
    def get(self, app_model, conversation_id):
        conversation_id = str(conversation_id)

        return _get_conversation(app_model, conversation_id)

    @console_ns.doc("delete_chat_conversation")
    @console_ns.doc(description="Delete a chat conversation")
    @console_ns.doc(params={"app_id": "Application ID", "conversation_id": "Conversation ID"})
    @console_ns.response(204, "Conversation deleted successfully")
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Conversation not found")
    @setup_required
    @login_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    @account_initialization_required
    @edit_permission_required
    def delete(self, app_model, conversation_id):
        current_user, _ = current_account_with_tenant()
        conversation_id = str(conversation_id)

        try:
            ConversationService.delete(app_model, conversation_id, current_user)
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")

        return {"result": "success"}, 204


def _get_conversation(app_model, conversation_id):
    current_user, _ = current_account_with_tenant()
    conversation = db.session.scalar(
        sa.select(Conversation).where(Conversation.id == conversation_id, Conversation.app_id == app_model.id).limit(1)
    )

    if not conversation:
        raise NotFound("Conversation Not Exists.")

    db.session.execute(
        sa.update(Conversation)
        .where(Conversation.id == conversation_id, Conversation.read_at.is_(None))
        # Keep updated_at unchanged when only marking a conversation as read.
        .values(
            read_at=naive_utc_now(),
            read_account_id=current_user.id,
            updated_at=Conversation.updated_at,
        )
    )
    db.session.commit()
    db.session.refresh(conversation)

    return conversation

```

### api/controllers/console/app/__init__.py
```py

```

### api/controllers/console/app/workflow_trigger.py
```py
import logging

from flask import request
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import NotFound

from configs import dify_config
from controllers.common.schema import get_or_create_model
from extensions.ext_database import db
from fields.workflow_trigger_fields import trigger_fields, triggers_list_fields, webhook_trigger_fields
from libs.login import current_user, login_required
from models.enums import AppTriggerStatus
from models.model import Account, App, AppMode
from models.trigger import AppTrigger, WorkflowWebhookTrigger

from .. import console_ns
from ..app.wraps import get_app_model
from ..wraps import account_initialization_required, edit_permission_required, setup_required

logger = logging.getLogger(__name__)
DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"

trigger_model = get_or_create_model("WorkflowTrigger", trigger_fields)

triggers_list_fields_copy = triggers_list_fields.copy()
triggers_list_fields_copy["data"] = fields.List(fields.Nested(trigger_model))
triggers_list_model = get_or_create_model("WorkflowTriggerList", triggers_list_fields_copy)

webhook_trigger_model = get_or_create_model("WebhookTrigger", webhook_trigger_fields)


class Parser(BaseModel):
    node_id: str


class ParserEnable(BaseModel):
    trigger_id: str
    enable_trigger: bool


console_ns.schema_model(Parser.__name__, Parser.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))

console_ns.schema_model(
    ParserEnable.__name__, ParserEnable.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/apps/<uuid:app_id>/workflows/triggers/webhook")
class WebhookTriggerApi(Resource):
    """Webhook Trigger API"""

    @console_ns.expect(console_ns.models[Parser.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.WORKFLOW)
    @marshal_with(webhook_trigger_model)
    def get(self, app_model: App):
        """Get webhook trigger for a node"""
        args = Parser.model_validate(request.args.to_dict(flat=True))  # type: ignore

        node_id = args.node_id

        with sessionmaker(db.engine).begin() as session:
            # Get webhook trigger for this app and node
            webhook_trigger = session.scalar(
                select(WorkflowWebhookTrigger)
                .where(
                    WorkflowWebhookTrigger.app_id == app_model.id,
                    WorkflowWebhookTrigger.node_id == node_id,
                )
                .limit(1)
            )

            if not webhook_trigger:
                raise NotFound("Webhook trigger not found for this node")

            return webhook_trigger


@console_ns.route("/apps/<uuid:app_id>/triggers")
class AppTriggersApi(Resource):
    """App Triggers list API"""

    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.WORKFLOW)
    @marshal_with(triggers_list_model)
    def get(self, app_model: App):
        """Get app triggers list"""
        assert isinstance(current_user, Account)
        assert current_user.current_tenant_id is not None

        with sessionmaker(db.engine).begin() as session:
            # Get all triggers for this app using select API
            triggers = (
                session.execute(
                    select(AppTrigger)
                    .where(
                        AppTrigger.tenant_id == current_user.current_tenant_id,
                        AppTrigger.app_id == app_model.id,
                    )
                    .order_by(AppTrigger.created_at.desc(), AppTrigger.id.desc())
                )
                .scalars()
                .all()
            )

        # Add computed icon field for each trigger
        url_prefix = dify_config.CONSOLE_API_URL + "/console/api/workspaces/current/tool-provider/builtin/"
        for trigger in triggers:
            if trigger.trigger_type == "trigger-plugin":
                trigger.icon = url_prefix + trigger.provider_name + "/icon"  # type: ignore
            else:
                trigger.icon = ""  # type: ignore

        return {"data": triggers}


@console_ns.route("/apps/<uuid:app_id>/trigger-enable")
class AppTriggerEnableApi(Resource):
    @console_ns.expect(console_ns.models[ParserEnable.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_app_model(mode=AppMode.WORKFLOW)
    @marshal_with(trigger_model)
    def post(self, app_model: App):
        """Update app trigger (enable/disable)"""
        args = ParserEnable.model_validate(console_ns.payload)

        assert current_user.current_tenant_id is not None

        trigger_id = args.trigger_id
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            # Find the trigger using select
            trigger = session.execute(
                select(AppTrigger).where(
                    AppTrigger.id == trigger_id,
                    AppTrigger.tenant_id == current_user.current_tenant_id,
                    AppTrigger.app_id == app_model.id,
                )
            ).scalar_one_or_none()

            if not trigger:
                raise NotFound("Trigger not found")

            # Update status based on enable_trigger boolean
            trigger.status = AppTriggerStatus.ENABLED if args.enable_trigger else AppTriggerStatus.DISABLED

        # Add computed icon field
        url_prefix = dify_config.CONSOLE_API_URL + "/console/api/workspaces/current/tool-provider/builtin/"
        if trigger.trigger_type == "trigger-plugin":
            trigger.icon = url_prefix + trigger.provider_name + "/icon"  # type: ignore
        else:
            trigger.icon = ""  # type: ignore

        return trigger

```

### api/controllers/console/app/message.py
```py
import logging
from typing import Literal

from flask import request
from flask_restx import Resource, fields, marshal_with
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, field_validator
from sqlalchemy import exists, func, select
from werkzeug.exceptions import InternalServerError, NotFound

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.app.error import (
    CompletionRequestError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.console.app.wraps import get_app_model
from controllers.console.explore.error import AppSuggestedQuestionsAfterAnswerDisabledError
from controllers.console.wraps import (
    account_initialization_required,
    edit_permission_required,
    setup_required,
)
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from extensions.ext_database import db
from fields.raws import FilesContainedField
from libs.helper import TimestampField, uuid_value
from libs.infinite_scroll_pagination import InfiniteScrollPagination
from libs.login import current_account_with_tenant, login_required
from models.enums import FeedbackFromSource, FeedbackRating
from models.model import AppMode, Conversation, Message, MessageAnnotation, MessageFeedback
from services.errors.conversation import ConversationNotExistsError
from services.errors.message import MessageNotExistsError, SuggestedQuestionsAfterAnswerDisabledError
from services.message_service import MessageService, attach_message_extra_contents

logger = logging.getLogger(__name__)


class ChatMessagesQuery(BaseModel):
    conversation_id: str = Field(..., description="Conversation ID")
    first_id: str | None = Field(default=None, description="First message ID for pagination")
    limit: int = Field(default=20, ge=1, le=100, description="Number of messages to return (1-100)")

    @field_validator("first_id", mode="before")
    @classmethod
    def empty_to_none(cls, value: str | None) -> str | None:
        if value == "":
            return None
        return value

    @field_validator("conversation_id", "first_id")
    @classmethod
    def validate_uuid(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class MessageFeedbackPayload(BaseModel):
    message_id: str = Field(..., description="Message ID")
    rating: Literal["like", "dislike"] | None = Field(default=None, description="Feedback rating")
    content: str | None = Field(default=None, description="Feedback content")

    @field_validator("message_id")
    @classmethod
    def validate_message_id(cls, value: str) -> str:
        return uuid_value(value)


class FeedbackExportQuery(BaseModel):
    from_source: Literal["user", "admin"] | None = Field(default=None, description="Filter by feedback source")
    rating: Literal["like", "dislike"] | None = Field(default=None, description="Filter by rating")
    has_comment: bool | None = Field(default=None, description="Only include feedback with comments")
    start_date: str | None = Field(default=None, description="Start date (YYYY-MM-DD)")
    end_date: str | None = Field(default=None, description="End date (YYYY-MM-DD)")
    format: Literal["csv", "json"] = Field(default="csv", description="Export format")

    @field_validator("has_comment", mode="before")
    @classmethod
    def parse_bool(cls, value: bool | str | None) -> bool | None:
        if isinstance(value, bool) or value is None:
            return value
        lowered = value.lower()
        if lowered in {"true", "1", "yes", "on"}:
            return True
        if lowered in {"false", "0", "no", "off"}:
            return False
        raise ValueError("has_comment must be a boolean value")


class AnnotationCountResponse(BaseModel):
    count: int = Field(description="Number of annotations")


class SuggestedQuestionsResponse(BaseModel):
    data: list[str] = Field(description="Suggested question")


register_schema_models(
    console_ns,
    ChatMessagesQuery,
    MessageFeedbackPayload,
    FeedbackExportQuery,
    AnnotationCountResponse,
    SuggestedQuestionsResponse,
)

# Register models for flask_restx to avoid dict type issues in Swagger
# Register in dependency order: base models first, then dependent models

# Base models
simple_account_model = console_ns.model(
    "SimpleAccount",
    {
        "id": fields.String,
        "name": fields.String,
        "email": fields.String,
    },
)

message_file_model = console_ns.model(
    "MessageFile",
    {
        "id": fields.String,
        "filename": fields.String,
        "type": fields.String,
        "url": fields.String,
        "mime_type": fields.String,
        "size": fields.Integer,
        "transfer_method": fields.String,
        "belongs_to": fields.String(default="user"),
        "upload_file_id": fields.String(default=None),
    },
)

agent_thought_model = console_ns.model(
    "AgentThought",
    {
        "id": fields.String,
        "chain_id": fields.String,
        "message_id": fields.String,
        "position": fields.Integer,
        "thought": fields.String,
        "tool": fields.String,
        "tool_labels": fields.Raw,
        "tool_input": fields.String,
        "created_at": TimestampField,
        "observation": fields.String,
        "files": fields.List(fields.String),
    },
)

# Models that depend on simple_account_model
feedback_model = console_ns.model(
    "Feedback",
    {
        "rating": fields.String,
        "content": fields.String,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account": fields.Nested(simple_account_model, allow_null=True),
    },
)

annotation_model = console_ns.model(
    "Annotation",
    {
        "id": fields.String,
        "question": fields.String,
        "content": fields.String,
        "account": fields.Nested(simple_account_model, allow_null=True),
        "created_at": TimestampField,
    },
)

annotation_hit_history_model = console_ns.model(
    "AnnotationHitHistory",
    {
        "annotation_id": fields.String(attribute="id"),
        "annotation_create_account": fields.Nested(simple_account_model, allow_null=True),
        "created_at": TimestampField,
    },
)

# Message detail model that depends on multiple models
message_detail_model = console_ns.model(
    "MessageDetail",
    {
        "id": fields.String,
        "conversation_id": fields.String,
        "inputs": FilesContainedField,
        "query": fields.String,
        "message": fields.Raw,
        "message_tokens": fields.Integer,
        "answer": fields.String(attribute="re_sign_file_url_answer"),
        "answer_tokens": fields.Integer,
        "provider_response_latency": fields.Float,
        "from_source": fields.String,
        "from_end_user_id": fields.String,
        "from_account_id": fields.String,
        "feedbacks": fields.List(fields.Nested(feedback_model)),
        "workflow_run_id": fields.String,
        "annotation": fields.Nested(annotation_model, allow_null=True),
        "annotation_hit_history": fields.Nested(annotation_hit_history_model, allow_null=True),
        "created_at": TimestampField,
        "agent_thoughts": fields.List(fields.Nested(agent_thought_model)),
        "message_files": fields.List(fields.Nested(message_file_model)),
        "extra_contents": fields.List(fields.Raw),
        "metadata": fields.Raw(attribute="message_metadata_dict"),
        "status": fields.String,
        "error": fields.String,
        "parent_message_id": fields.String,
    },
)

# Message infinite scroll pagination model
message_infinite_scroll_pagination_model = console_ns.model(
    "MessageInfiniteScrollPagination",
    {
        "limit": fields.Integer,
        "has_more": fields.Boolean,
        "data": fields.List(fields.Nested(message_detail_model)),
    },
)


@console_ns.route("/apps/<uuid:app_id>/chat-messages")
class ChatMessageListApi(Resource):
    @console_ns.doc("list_chat_messages")
    @console_ns.doc(description="Get chat messages for a conversation with pagination")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[ChatMessagesQuery.__name__])
    @console_ns.response(200, "Success", message_infinite_scroll_pagination_model)
    @console_ns.response(404, "Conversation not found")
    @login_required
    @account_initialization_required
    @setup_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    @marshal_with(message_infinite_scroll_pagination_model)
    @edit_permission_required
    def get(self, app_model):
        args = ChatMessagesQuery.model_validate(request.args.to_dict())

        conversation = db.session.scalar(
            select(Conversation)
            .where(Conversation.id == args.conversation_id, Conversation.app_id == app_model.id)
            .limit(1)
        )

        if not conversation:
            raise NotFound("Conversation Not Exists.")

        if args.first_id:
            first_message = db.session.scalar(
                select(Message).where(Message.conversation_id == conversation.id, Message.id == args.first_id).limit(1)
            )

            if not first_message:
                raise NotFound("First message not found")

            history_messages = db.session.scalars(
                select(Message)
                .where(
                    Message.conversation_id == conversation.id,
                    Message.created_at < first_message.created_at,
                    Message.id != first_message.id,
                )
                .order_by(Message.created_at.desc())
                .limit(args.limit)
            ).all()
        else:
            history_messages = db.session.scalars(
                select(Message)
                .where(Message.conversation_id == conversation.id)
                .order_by(Message.created_at.desc())
                .limit(args.limit)
            ).all()

        # Initialize has_more based on whether we have a full page
        if len(history_messages) == args.limit:
            current_page_first_message = history_messages[-1]
            # Check if there are more messages before the current page
            has_more = db.session.scalar(
                select(
                    exists().where(
                        Message.conversation_id == conversation.id,
                        Message.created_at < current_page_first_message.created_at,
                        Message.id != current_page_first_message.id,
                    )
                )
            )
        else:
            # If we don't have a full page, there are no more messages
            has_more = False

        history_messages = list(reversed(history_messages))
        attach_message_extra_contents(history_messages)

        return InfiniteScrollPagination(data=history_messages, limit=args.limit, has_more=has_more)


@console_ns.route("/apps/<uuid:app_id>/feedbacks")
class MessageFeedbackApi(Resource):
    @console_ns.doc("create_message_feedback")
    @console_ns.doc(description="Create or update message feedback (like/dislike)")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[MessageFeedbackPayload.__name__])
    @console_ns.response(200, "Feedback updated successfully")
    @console_ns.response(404, "Message not found")
    @console_ns.response(403, "Insufficient permissions")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, app_model):
        current_user, _ = current_account_with_tenant()

        args = MessageFeedbackPayload.model_validate(console_ns.payload)

        message_id = str(args.message_id)

        message = db.session.scalar(
            select(Message).where(Message.id == message_id, Message.app_id == app_model.id).limit(1)
        )

        if not message:
            raise NotFound("Message Not Exists.")

        feedback = message.admin_feedback

        if not args.rating and feedback:
            db.session.delete(feedback)
        elif args.rating and feedback:
            feedback.rating = FeedbackRating(args.rating)
            feedback.content = args.content
        elif not args.rating and not feedback:
            raise ValueError("rating cannot be None when feedback not exists")
        else:
            rating_value = args.rating
            if rating_value is None:
                raise ValueError("rating is required to create feedback")
            feedback = MessageFeedback(
                app_id=app_model.id,
                conversation_id=message.conversation_id,
                message_id=message.id,
                rating=FeedbackRating(rating_value),
                content=args.content,
                from_source=FeedbackFromSource.ADMIN,
                from_account_id=current_user.id,
            )
            db.session.add(feedback)

        db.session.commit()

        return {"result": "success"}


@console_ns.route("/apps/<uuid:app_id>/annotations/count")
class MessageAnnotationCountApi(Resource):
    @console_ns.doc("get_annotation_count")
    @console_ns.doc(description="Get count of message annotations for the app")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(
        200,
        "Annotation count retrieved successfully",
        console_ns.models[AnnotationCountResponse.__name__],
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        count = db.session.scalar(
            select(func.count(MessageAnnotation.id)).where(MessageAnnotation.app_id == app_model.id)
        )

        return {"count": count}


@console_ns.route("/apps/<uuid:app_id>/chat-messages/<uuid:message_id>/suggested-questions")
class MessageSuggestedQuestionApi(Resource):
    @console_ns.doc("get_message_suggested_questions")
    @console_ns.doc(description="Get suggested questions for a message")
    @console_ns.doc(params={"app_id": "Application ID", "message_id": "Message ID"})
    @console_ns.response(
        200,
        "Suggested questions retrieved successfully",
        console_ns.models[SuggestedQuestionsResponse.__name__],
    )
    @console_ns.response(404, "Message or conversation not found")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    def get(self, app_model, message_id):
        current_user, _ = current_account_with_tenant()
        message_id = str(message_id)

        try:
            questions = MessageService.get_suggested_questions_after_answer(
                app_model=app_model, message_id=message_id, user=current_user, invoke_from=InvokeFrom.DEBUGGER
            )
        except MessageNotExistsError:
            raise NotFound("Message not found")
        except ConversationNotExistsError:
            raise NotFound("Conversation not found")
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except SuggestedQuestionsAfterAnswerDisabledError:
            raise AppSuggestedQuestionsAfterAnswerDisabledError()
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()

        return {"data": questions}


@console_ns.route("/apps/<uuid:app_id>/feedbacks/export")
class MessageFeedbackExportApi(Resource):
    @console_ns.doc("export_feedbacks")
    @console_ns.doc(description="Export user feedback data for Google Sheets")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[FeedbackExportQuery.__name__])
    @console_ns.response(200, "Feedback data exported successfully")
    @console_ns.response(400, "Invalid parameters")
    @console_ns.response(500, "Internal server error")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        args = FeedbackExportQuery.model_validate(request.args.to_dict())

        # Import the service function
        from services.feedback_service import FeedbackService

        try:
            export_data = FeedbackService.export_feedbacks(
                app_id=app_model.id,
                from_source=args.from_source,
                rating=args.rating,
                has_comment=args.has_comment,
                start_date=args.start_date,
                end_date=args.end_date,
                format_type=args.format,
            )

            return export_data

        except ValueError as e:
            logger.exception("Parameter validation error in feedback export")
            return {"error": f"Parameter validation error: {str(e)}"}, 400
        except Exception as e:
            logger.exception("Error exporting feedback data")
            raise InternalServerError(str(e))


@console_ns.route("/apps/<uuid:app_id>/messages/<uuid:message_id>")
class MessageApi(Resource):
    @console_ns.doc("get_message")
    @console_ns.doc(description="Get message details by ID")
    @console_ns.doc(params={"app_id": "Application ID", "message_id": "Message ID"})
    @console_ns.response(200, "Message retrieved successfully", message_detail_model)
    @console_ns.response(404, "Message not found")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(message_detail_model)
    def get(self, app_model, message_id: str):
        message_id = str(message_id)

        message = db.session.scalar(
            select(Message).where(Message.id == message_id, Message.app_id == app_model.id).limit(1)
        )

        if not message:
            raise NotFound("Message Not Exists.")

        attach_message_extra_contents([message])
        return message

```

### api/controllers/console/app/ops_trace.py
```py
from typing import Any

from flask import request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field
from werkzeug.exceptions import BadRequest

from controllers.console import console_ns
from controllers.console.app.error import TracingConfigCheckError, TracingConfigIsExist, TracingConfigNotExist
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import login_required
from services.ops_service import OpsService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class TraceProviderQuery(BaseModel):
    tracing_provider: str = Field(..., description="Tracing provider name")


class TraceConfigPayload(BaseModel):
    tracing_provider: str = Field(..., description="Tracing provider name")
    tracing_config: dict[str, Any] = Field(..., description="Tracing configuration data")


console_ns.schema_model(
    TraceProviderQuery.__name__,
    TraceProviderQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)
console_ns.schema_model(
    TraceConfigPayload.__name__, TraceConfigPayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/apps/<uuid:app_id>/trace-config")
class TraceAppConfigApi(Resource):
    """
    Manage trace app configurations
    """

    @console_ns.doc("get_trace_app_config")
    @console_ns.doc(description="Get tracing configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[TraceProviderQuery.__name__])
    @console_ns.response(
        200, "Tracing configuration retrieved successfully", fields.Raw(description="Tracing configuration data")
    )
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_id):
        args = TraceProviderQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            trace_config = OpsService.get_tracing_app_config(app_id=app_id, tracing_provider=args.tracing_provider)
            if not trace_config:
                return {"has_not_configured": True}
            return trace_config
        except Exception as e:
            raise BadRequest(str(e))

    @console_ns.doc("create_trace_app_config")
    @console_ns.doc(description="Create a new tracing configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[TraceConfigPayload.__name__])
    @console_ns.response(
        201, "Tracing configuration created successfully", fields.Raw(description="Created configuration data")
    )
    @console_ns.response(400, "Invalid request parameters or configuration already exists")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, app_id):
        """Create a new trace app configuration"""
        args = TraceConfigPayload.model_validate(console_ns.payload)

        try:
            result = OpsService.create_tracing_app_config(
                app_id=app_id, tracing_provider=args.tracing_provider, tracing_config=args.tracing_config
            )
            if not result:
                raise TracingConfigIsExist()
            if result.get("error"):
                raise TracingConfigCheckError()
            return result
        except Exception as e:
            raise BadRequest(str(e))

    @console_ns.doc("update_trace_app_config")
    @console_ns.doc(description="Update an existing tracing configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[TraceConfigPayload.__name__])
    @console_ns.response(200, "Tracing configuration updated successfully", fields.Raw(description="Success response"))
    @console_ns.response(400, "Invalid request parameters or configuration not found")
    @setup_required
    @login_required
    @account_initialization_required
    def patch(self, app_id):
        """Update an existing trace app configuration"""
        args = TraceConfigPayload.model_validate(console_ns.payload)

        try:
            result = OpsService.update_tracing_app_config(
                app_id=app_id, tracing_provider=args.tracing_provider, tracing_config=args.tracing_config
            )
            if not result:
                raise TracingConfigNotExist()
            return {"result": "success"}
        except Exception as e:
            raise BadRequest(str(e))

    @console_ns.doc("delete_trace_app_config")
    @console_ns.doc(description="Delete an existing tracing configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[TraceProviderQuery.__name__])
    @console_ns.response(204, "Tracing configuration deleted successfully")
    @console_ns.response(400, "Invalid request parameters or configuration not found")
    @setup_required
    @login_required
    @account_initialization_required
    def delete(self, app_id):
        """Delete an existing trace app configuration"""
        args = TraceProviderQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            result = OpsService.delete_tracing_app_config(app_id=app_id, tracing_provider=args.tracing_provider)
            if not result:
                raise TracingConfigNotExist()
            return {"result": "success"}, 204
        except Exception as e:
            raise BadRequest(str(e))

```

### api/controllers/console/app/workflow_statistic.py
```py
from flask import abort, jsonify, request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from extensions.ext_database import db
from libs.datetime_utils import parse_time_range
from libs.login import current_account_with_tenant, login_required
from models.enums import WorkflowRunTriggeredFrom
from models.model import AppMode
from repositories.factory import DifyAPIRepositoryFactory

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class WorkflowStatisticQuery(BaseModel):
    start: str | None = Field(default=None, description="Start date and time (YYYY-MM-DD HH:MM)")
    end: str | None = Field(default=None, description="End date and time (YYYY-MM-DD HH:MM)")

    @field_validator("start", "end", mode="before")
    @classmethod
    def blank_to_none(cls, value: str | None) -> str | None:
        if value == "":
            return None
        return value


console_ns.schema_model(
    WorkflowStatisticQuery.__name__,
    WorkflowStatisticQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)


@console_ns.route("/apps/<uuid:app_id>/workflow/statistics/daily-conversations")
class WorkflowDailyRunsStatistic(Resource):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        self._workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

    @console_ns.doc("get_workflow_daily_runs_statistic")
    @console_ns.doc(description="Get workflow daily runs statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowStatisticQuery.__name__])
    @console_ns.response(200, "Daily runs statistics retrieved successfully")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = WorkflowStatisticQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        assert account.timezone is not None

        try:
            start_date, end_date = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        response_data = self._workflow_run_repo.get_daily_runs_statistics(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            triggered_from=WorkflowRunTriggeredFrom.APP_RUN,
            start_date=start_date,
            end_date=end_date,
            timezone=account.timezone,
        )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/workflow/statistics/daily-terminals")
class WorkflowDailyTerminalsStatistic(Resource):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        self._workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

    @console_ns.doc("get_workflow_daily_terminals_statistic")
    @console_ns.doc(description="Get workflow daily terminals statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowStatisticQuery.__name__])
    @console_ns.response(200, "Daily terminals statistics retrieved successfully")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = WorkflowStatisticQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        assert account.timezone is not None

        try:
            start_date, end_date = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        response_data = self._workflow_run_repo.get_daily_terminals_statistics(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            triggered_from=WorkflowRunTriggeredFrom.APP_RUN,
            start_date=start_date,
            end_date=end_date,
            timezone=account.timezone,
        )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/workflow/statistics/token-costs")
class WorkflowDailyTokenCostStatistic(Resource):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        self._workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

    @console_ns.doc("get_workflow_daily_token_cost_statistic")
    @console_ns.doc(description="Get workflow daily token cost statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowStatisticQuery.__name__])
    @console_ns.response(200, "Daily token cost statistics retrieved successfully")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = WorkflowStatisticQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        assert account.timezone is not None

        try:
            start_date, end_date = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        response_data = self._workflow_run_repo.get_daily_token_cost_statistics(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            triggered_from=WorkflowRunTriggeredFrom.APP_RUN,
            start_date=start_date,
            end_date=end_date,
            timezone=account.timezone,
        )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/workflow/statistics/average-app-interactions")
class WorkflowAverageAppInteractionStatistic(Resource):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        self._workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

    @console_ns.doc("get_workflow_average_app_interaction_statistic")
    @console_ns.doc(description="Get workflow average app interaction statistics")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowStatisticQuery.__name__])
    @console_ns.response(200, "Average app interaction statistics retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.WORKFLOW])
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = WorkflowStatisticQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        assert account.timezone is not None

        try:
            start_date, end_date = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        response_data = self._workflow_run_repo.get_average_app_interaction_statistics(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            triggered_from=WorkflowRunTriggeredFrom.APP_RUN,
            start_date=start_date,
            end_date=end_date,
            timezone=account.timezone,
        )

        return jsonify({"data": response_data})

```

### api/controllers/console/app/generator.py
```py
from collections.abc import Sequence

from flask_restx import Resource
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field

from controllers.console import console_ns
from controllers.console.app.error import (
    CompletionRequestError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.console.wraps import account_initialization_required, setup_required
from core.app.app_config.entities import ModelConfig
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from core.helper.code_executor.code_node_provider import CodeNodeProvider
from core.helper.code_executor.javascript.javascript_code_provider import JavascriptCodeProvider
from core.helper.code_executor.python3.python3_code_provider import Python3CodeProvider
from core.llm_generator.entities import RuleCodeGeneratePayload, RuleGeneratePayload, RuleStructuredOutputPayload
from core.llm_generator.llm_generator import LLMGenerator
from extensions.ext_database import db
from libs.login import current_account_with_tenant, login_required
from models import App
from services.workflow_service import WorkflowService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class InstructionGeneratePayload(BaseModel):
    flow_id: str = Field(..., description="Workflow/Flow ID")
    node_id: str = Field(default="", description="Node ID for workflow context")
    current: str = Field(default="", description="Current instruction text")
    language: str = Field(default="javascript", description="Programming language (javascript/python)")
    instruction: str = Field(..., description="Instruction for generation")
    model_config_data: ModelConfig = Field(..., alias="model_config", description="Model configuration")
    ideal_output: str = Field(default="", description="Expected ideal output")


class InstructionTemplatePayload(BaseModel):
    type: str = Field(..., description="Instruction template type")


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(RuleGeneratePayload)
reg(RuleCodeGeneratePayload)
reg(RuleStructuredOutputPayload)
reg(InstructionGeneratePayload)
reg(InstructionTemplatePayload)
reg(ModelConfig)


@console_ns.route("/rule-generate")
class RuleGenerateApi(Resource):
    @console_ns.doc("generate_rule_config")
    @console_ns.doc(description="Generate rule configuration using LLM")
    @console_ns.expect(console_ns.models[RuleGeneratePayload.__name__])
    @console_ns.response(200, "Rule configuration generated successfully")
    @console_ns.response(400, "Invalid request parameters")
    @console_ns.response(402, "Provider quota exceeded")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = RuleGeneratePayload.model_validate(console_ns.payload)
        _, current_tenant_id = current_account_with_tenant()

        try:
            rules = LLMGenerator.generate_rule_config(tenant_id=current_tenant_id, args=args)
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)

        return rules


@console_ns.route("/rule-code-generate")
class RuleCodeGenerateApi(Resource):
    @console_ns.doc("generate_rule_code")
    @console_ns.doc(description="Generate code rules using LLM")
    @console_ns.expect(console_ns.models[RuleCodeGeneratePayload.__name__])
    @console_ns.response(200, "Code rules generated successfully")
    @console_ns.response(400, "Invalid request parameters")
    @console_ns.response(402, "Provider quota exceeded")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = RuleCodeGeneratePayload.model_validate(console_ns.payload)
        _, current_tenant_id = current_account_with_tenant()

        try:
            code_result = LLMGenerator.generate_code(
                tenant_id=current_tenant_id,
                args=args,
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)

        return code_result


@console_ns.route("/rule-structured-output-generate")
class RuleStructuredOutputGenerateApi(Resource):
    @console_ns.doc("generate_structured_output")
    @console_ns.doc(description="Generate structured output rules using LLM")
    @console_ns.expect(console_ns.models[RuleStructuredOutputPayload.__name__])
    @console_ns.response(200, "Structured output generated successfully")
    @console_ns.response(400, "Invalid request parameters")
    @console_ns.response(402, "Provider quota exceeded")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = RuleStructuredOutputPayload.model_validate(console_ns.payload)
        _, current_tenant_id = current_account_with_tenant()

        try:
            structured_output = LLMGenerator.generate_structured_output(
                tenant_id=current_tenant_id,
                args=args,
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)

        return structured_output


@console_ns.route("/instruction-generate")
class InstructionGenerateApi(Resource):
    @console_ns.doc("generate_instruction")
    @console_ns.doc(description="Generate instruction for workflow nodes or general use")
    @console_ns.expect(console_ns.models[InstructionGeneratePayload.__name__])
    @console_ns.response(200, "Instruction generated successfully")
    @console_ns.response(400, "Invalid request parameters or flow/workflow not found")
    @console_ns.response(402, "Provider quota exceeded")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = InstructionGeneratePayload.model_validate(console_ns.payload)
        _, current_tenant_id = current_account_with_tenant()
        providers: list[type[CodeNodeProvider]] = [Python3CodeProvider, JavascriptCodeProvider]
        code_provider: type[CodeNodeProvider] | None = next(
            (p for p in providers if p.is_accept_language(args.language)), None
        )
        code_template = code_provider.get_default_code() if code_provider else ""
        try:
            # Generate from nothing for a workflow node
            if (args.current in (code_template, "")) and args.node_id != "":
                app = db.session.get(App, args.flow_id)
                if not app:
                    return {"error": f"app {args.flow_id} not found"}, 400
                workflow = WorkflowService().get_draft_workflow(app_model=app)
                if not workflow:
                    return {"error": f"workflow {args.flow_id} not found"}, 400
                nodes: Sequence = workflow.graph_dict["nodes"]
                node = [node for node in nodes if node["id"] == args.node_id]
                if len(node) == 0:
                    return {"error": f"node {args.node_id} not found"}, 400
                node_type = node[0]["data"]["type"]
                match node_type:
                    case "llm":
                        return LLMGenerator.generate_rule_config(
                            current_tenant_id,
                            args=RuleGeneratePayload(
                                instruction=args.instruction,
                                model_config=args.model_config_data,
                                no_variable=True,
                            ),
                        )
                    case "agent":
                        return LLMGenerator.generate_rule_config(
                            current_tenant_id,
                            args=RuleGeneratePayload(
                                instruction=args.instruction,
                                model_config=args.model_config_data,
                                no_variable=True,
                            ),
                        )
                    case "code":
                        return LLMGenerator.generate_code(
                            tenant_id=current_tenant_id,
                            args=RuleCodeGeneratePayload(
                                instruction=args.instruction,
                                model_config=args.model_config_data,
                                code_language=args.language,
                            ),
                        )
                    case _:
                        return {"error": f"invalid node type: {node_type}"}
            if args.node_id == "" and args.current != "":  # For legacy app without a workflow
                return LLMGenerator.instruction_modify_legacy(
                    tenant_id=current_tenant_id,
                    flow_id=args.flow_id,
                    current=args.current,
                    instruction=args.instruction,
                    model_config=args.model_config_data,
                    ideal_output=args.ideal_output,
                )
            if args.node_id != "" and args.current != "":  # For workflow node
                return LLMGenerator.instruction_modify_workflow(
                    tenant_id=current_tenant_id,
                    flow_id=args.flow_id,
                    node_id=args.node_id,
                    current=args.current,
                    instruction=args.instruction,
                    model_config=args.model_config_data,
                    ideal_output=args.ideal_output,
                    workflow_service=WorkflowService(),
                )
            return {"error": "incompatible parameters"}, 400
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)


@console_ns.route("/instruction-generate/template")
class InstructionGenerationTemplateApi(Resource):
    @console_ns.doc("get_instruction_template")
    @console_ns.doc(description="Get instruction generation template")
    @console_ns.expect(console_ns.models[InstructionTemplatePayload.__name__])
    @console_ns.response(200, "Template retrieved successfully")
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = InstructionTemplatePayload.model_validate(console_ns.payload)
        match args.type:
            case "prompt":
                from core.llm_generator.prompts import INSTRUCTION_GENERATE_TEMPLATE_PROMPT

                return {"data": INSTRUCTION_GENERATE_TEMPLATE_PROMPT}
            case "code":
                from core.llm_generator.prompts import INSTRUCTION_GENERATE_TEMPLATE_CODE

                return {"data": INSTRUCTION_GENERATE_TEMPLATE_CODE}
            case _:
                raise ValueError(f"Invalid type: {args.type}")

```

### api/controllers/console/app/site.py
```py
from typing import Literal

from flask_restx import Resource, marshal_with
from pydantic import BaseModel, Field, field_validator
from sqlalchemy import select
from werkzeug.exceptions import NotFound

from constants.languages import supported_language
from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import (
    account_initialization_required,
    edit_permission_required,
    is_admin_or_owner_required,
    setup_required,
)
from extensions.ext_database import db
from fields.app_fields import app_site_fields
from libs.datetime_utils import naive_utc_now
from libs.login import current_account_with_tenant, login_required
from models import Site

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class AppSiteUpdatePayload(BaseModel):
    title: str | None = Field(default=None)
    icon_type: str | None = Field(default=None)
    icon: str | None = Field(default=None)
    icon_background: str | None = Field(default=None)
    description: str | None = Field(default=None)
    default_language: str | None = Field(default=None)
    chat_color_theme: str | None = Field(default=None)
    chat_color_theme_inverted: bool | None = Field(default=None)
    customize_domain: str | None = Field(default=None)
    copyright: str | None = Field(default=None)
    privacy_policy: str | None = Field(default=None)
    custom_disclaimer: str | None = Field(default=None)
    customize_token_strategy: Literal["must", "allow", "not_allow"] | None = Field(default=None)
    prompt_public: bool | None = Field(default=None)
    show_workflow_steps: bool | None = Field(default=None)
    use_icon_as_answer_icon: bool | None = Field(default=None)

    @field_validator("default_language")
    @classmethod
    def validate_language(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return supported_language(value)


console_ns.schema_model(
    AppSiteUpdatePayload.__name__,
    AppSiteUpdatePayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)

# Register model for flask_restx to avoid dict type issues in Swagger
app_site_model = console_ns.model("AppSite", app_site_fields)


@console_ns.route("/apps/<uuid:app_id>/site")
class AppSite(Resource):
    @console_ns.doc("update_app_site")
    @console_ns.doc(description="Update application site configuration")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppSiteUpdatePayload.__name__])
    @console_ns.response(200, "Site configuration updated successfully", app_site_model)
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "App not found")
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    @get_app_model
    @marshal_with(app_site_model)
    def post(self, app_model):
        args = AppSiteUpdatePayload.model_validate(console_ns.payload or {})
        current_user, _ = current_account_with_tenant()
        site = db.session.scalar(select(Site).where(Site.app_id == app_model.id).limit(1))
        if not site:
            raise NotFound

        for attr_name in [
            "title",
            "icon_type",
            "icon",
            "icon_background",
            "description",
            "default_language",
            "chat_color_theme",
            "chat_color_theme_inverted",
            "customize_domain",
            "copyright",
            "privacy_policy",
            "custom_disclaimer",
            "customize_token_strategy",
            "prompt_public",
            "show_workflow_steps",
            "use_icon_as_answer_icon",
        ]:
            value = getattr(args, attr_name)
            if value is not None:
                setattr(site, attr_name, value)

        site.updated_by = current_user.id
        site.updated_at = naive_utc_now()
        db.session.commit()

        return site


@console_ns.route("/apps/<uuid:app_id>/site/access-token-reset")
class AppSiteAccessTokenReset(Resource):
    @console_ns.doc("reset_app_site_access_token")
    @console_ns.doc(description="Reset access token for application site")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Access token reset successfully", app_site_model)
    @console_ns.response(403, "Insufficient permissions (admin/owner required)")
    @console_ns.response(404, "App or site not found")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    @get_app_model
    @marshal_with(app_site_model)
    def post(self, app_model):
        current_user, _ = current_account_with_tenant()
        site = db.session.scalar(select(Site).where(Site.app_id == app_model.id).limit(1))

        if not site:
            raise NotFound

        site.code = Site.generate_code(16)
        site.updated_by = current_user.id
        site.updated_at = naive_utc_now()
        db.session.commit()

        return site

```

### api/controllers/console/app/agent.py
```py
from flask import request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field, field_validator

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from libs.helper import uuid_value
from libs.login import login_required
from models.model import AppMode
from services.agent_service import AgentService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class AgentLogQuery(BaseModel):
    message_id: str = Field(..., description="Message UUID")
    conversation_id: str = Field(..., description="Conversation UUID")

    @field_validator("message_id", "conversation_id")
    @classmethod
    def validate_uuid(cls, value: str) -> str:
        return uuid_value(value)


console_ns.schema_model(
    AgentLogQuery.__name__, AgentLogQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/apps/<uuid:app_id>/agent/logs")
class AgentLogApi(Resource):
    @console_ns.doc("get_agent_logs")
    @console_ns.doc(description="Get agent execution logs for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AgentLogQuery.__name__])
    @console_ns.response(
        200, "Agent logs retrieved successfully", fields.List(fields.Raw(description="Agent log entries"))
    )
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.AGENT_CHAT])
    def get(self, app_model):
        """Get agent logs"""
        args = AgentLogQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        return AgentService.get_agent_logs(app_model, args.conversation_id, args.message_id)

```

### api/controllers/console/app/app.py
```py
import logging
import uuid
from datetime import datetime
from typing import Any, Literal

from flask import request
from flask_restx import Resource
from graphon.enums import WorkflowExecutionStatus
from graphon.file import helpers as file_helpers
from pydantic import AliasChoices, BaseModel, ConfigDict, Field, computed_field, field_validator
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest

from controllers.common.helpers import FileInfo
from controllers.common.schema import register_enum_models, register_schema_models
from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.workspace.models import LoadBalancingPayload
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_resource_check,
    edit_permission_required,
    enterprise_license_required,
    is_admin_or_owner_required,
    setup_required,
)
from core.ops.ops_trace_manager import OpsTraceManager
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.trigger.constants import TRIGGER_NODE_TYPES
from extensions.ext_database import db
from libs.login import current_account_with_tenant, login_required
from models import App, DatasetPermissionEnum, Workflow
from models.model import IconType
from services.app_dsl_service import AppDslService, ImportMode
from services.app_service import AppService
from services.enterprise.enterprise_service import EnterpriseService
from services.entities.knowledge_entities.knowledge_entities import (
    DataSource,
    InfoList,
    NotionIcon,
    NotionInfo,
    NotionPage,
    PreProcessingRule,
    RerankingModel,
    Rule,
    Segmentation,
    WebsiteInfo,
    WeightKeywordSetting,
    WeightModel,
    WeightVectorSetting,
)
from services.feature_service import FeatureService

ALLOW_CREATE_APP_MODES = ["chat", "agent-chat", "advanced-chat", "workflow", "completion"]

register_enum_models(console_ns, IconType)

_logger = logging.getLogger(__name__)


class AppListQuery(BaseModel):
    page: int = Field(default=1, ge=1, le=99999, description="Page number (1-99999)")
    limit: int = Field(default=20, ge=1, le=100, description="Page size (1-100)")
    mode: Literal["completion", "chat", "advanced-chat", "workflow", "agent-chat", "channel", "all"] = Field(
        default="all", description="App mode filter"
    )
    name: str | None = Field(default=None, description="Filter by app name")
    tag_ids: list[str] | None = Field(default=None, description="Comma-separated tag IDs")
    is_created_by_me: bool | None = Field(default=None, description="Filter by creator")

    @field_validator("tag_ids", mode="before")
    @classmethod
    def validate_tag_ids(cls, value: str | list[str] | None) -> list[str] | None:
        if not value:
            return None

        if isinstance(value, str):
            items = [item.strip() for item in value.split(",") if item.strip()]
        elif isinstance(value, list):
            items = [str(item).strip() for item in value if item and str(item).strip()]
        else:
            raise TypeError("Unsupported tag_ids type.")

        if not items:
            return None

        try:
            return [str(uuid.UUID(item)) for item in items]
        except ValueError as exc:
            raise ValueError("Invalid UUID format in tag_ids.") from exc


class CreateAppPayload(BaseModel):
    name: str = Field(..., min_length=1, description="App name")
    description: str | None = Field(default=None, description="App description (max 400 chars)", max_length=400)
    mode: Literal["chat", "agent-chat", "advanced-chat", "workflow", "completion"] = Field(..., description="App mode")
    icon_type: IconType | None = Field(default=None, description="Icon type")
    icon: str | None = Field(default=None, description="Icon")
    icon_background: str | None = Field(default=None, description="Icon background color")


class UpdateAppPayload(BaseModel):
    name: str = Field(..., min_length=1, description="App name")
    description: str | None = Field(default=None, description="App description (max 400 chars)", max_length=400)
    icon_type: IconType | None = Field(default=None, description="Icon type")
    icon: str | None = Field(default=None, description="Icon")
    icon_background: str | None = Field(default=None, description="Icon background color")
    use_icon_as_answer_icon: bool | None = Field(default=None, description="Use icon as answer icon")
    max_active_requests: int | None = Field(default=None, description="Maximum active requests")


class CopyAppPayload(BaseModel):
    name: str | None = Field(default=None, description="Name for the copied app")
    description: str | None = Field(default=None, description="Description for the copied app", max_length=400)
    icon_type: IconType | None = Field(default=None, description="Icon type")
    icon: str | None = Field(default=None, description="Icon")
    icon_background: str | None = Field(default=None, description="Icon background color")


class AppExportQuery(BaseModel):
    include_secret: bool = Field(default=False, description="Include secrets in export")
    workflow_id: str | None = Field(default=None, description="Specific workflow ID to export")


class AppNamePayload(BaseModel):
    name: str = Field(..., min_length=1, description="Name to check")


class AppIconPayload(BaseModel):
    icon: str | None = Field(default=None, description="Icon data")
    icon_background: str | None = Field(default=None, description="Icon background color")


class AppSiteStatusPayload(BaseModel):
    enable_site: bool = Field(..., description="Enable or disable site")


class AppApiStatusPayload(BaseModel):
    enable_api: bool = Field(..., description="Enable or disable API")


class AppTracePayload(BaseModel):
    enabled: bool = Field(..., description="Enable or disable tracing")
    tracing_provider: str | None = Field(default=None, description="Tracing provider")

    @field_validator("tracing_provider")
    @classmethod
    def validate_tracing_provider(cls, value: str | None, info) -> str | None:
        if info.data.get("enabled") and not value:
            raise ValueError("tracing_provider is required when enabled is True")
        return value


type JSONValue = Any


class ResponseModel(BaseModel):
    model_config = ConfigDict(
        from_attributes=True,
        extra="ignore",
        populate_by_name=True,
        serialize_by_alias=True,
        protected_namespaces=(),
    )


def _to_timestamp(value: datetime | int | None) -> int | None:
    if isinstance(value, datetime):
        return int(value.timestamp())
    return value


def _build_icon_url(icon_type: str | IconType | None, icon: str | None) -> str | None:
    if icon is None or icon_type is None:
        return None
    icon_type_value = icon_type.value if isinstance(icon_type, IconType) else str(icon_type)
    if icon_type_value.lower() != IconType.IMAGE:
        return None
    return file_helpers.get_signed_file_url(icon)


class Tag(ResponseModel):
    id: str
    name: str
    type: str


class WorkflowPartial(ResponseModel):
    id: str
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class ModelConfigPartial(ResponseModel):
    model: JSONValue | None = Field(default=None, validation_alias=AliasChoices("model_dict", "model"))
    pre_prompt: str | None = None
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class ModelConfig(ResponseModel):
    opening_statement: str | None = None
    suggested_questions: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("suggested_questions_list", "suggested_questions")
    )
    suggested_questions_after_answer: JSONValue | None = Field(
        default=None,
        validation_alias=AliasChoices("suggested_questions_after_answer_dict", "suggested_questions_after_answer"),
    )
    speech_to_text: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("speech_to_text_dict", "speech_to_text")
    )
    text_to_speech: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("text_to_speech_dict", "text_to_speech")
    )
    retriever_resource: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("retriever_resource_dict", "retriever_resource")
    )
    annotation_reply: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("annotation_reply_dict", "annotation_reply")
    )
    more_like_this: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("more_like_this_dict", "more_like_this")
    )
    sensitive_word_avoidance: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("sensitive_word_avoidance_dict", "sensitive_word_avoidance")
    )
    external_data_tools: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("external_data_tools_list", "external_data_tools")
    )
    model: JSONValue | None = Field(default=None, validation_alias=AliasChoices("model_dict", "model"))
    user_input_form: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("user_input_form_list", "user_input_form")
    )
    dataset_query_variable: str | None = None
    pre_prompt: str | None = None
    agent_mode: JSONValue | None = Field(default=None, validation_alias=AliasChoices("agent_mode_dict", "agent_mode"))
    prompt_type: str | None = None
    chat_prompt_config: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("chat_prompt_config_dict", "chat_prompt_config")
    )
    completion_prompt_config: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("completion_prompt_config_dict", "completion_prompt_config")
    )
    dataset_configs: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("dataset_configs_dict", "dataset_configs")
    )
    file_upload: JSONValue | None = Field(
        default=None, validation_alias=AliasChoices("file_upload_dict", "file_upload")
    )
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class Site(ResponseModel):
    access_token: str | None = Field(default=None, validation_alias="code")
    code: str | None = None
    title: str | None = None
    icon_type: str | IconType | None = None
    icon: str | None = None
    icon_background: str | None = None
    description: str | None = None
    default_language: str | None = None
    chat_color_theme: str | None = None
    chat_color_theme_inverted: bool | None = None
    customize_domain: str | None = None
    copyright: str | None = None
    privacy_policy: str | None = None
    custom_disclaimer: str | None = None
    customize_token_strategy: str | None = None
    prompt_public: bool | None = None
    app_base_url: str | None = None
    show_workflow_steps: bool | None = None
    use_icon_as_answer_icon: bool | None = None
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None

    @computed_field(return_type=str | None)  # type: ignore
    @property
    def icon_url(self) -> str | None:
        return _build_icon_url(self.icon_type, self.icon)

    @field_validator("icon_type", mode="before")
    @classmethod
    def _normalize_icon_type(cls, value: str | IconType | None) -> str | None:
        if isinstance(value, IconType):
            return value.value
        return value

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class DeletedTool(ResponseModel):
    type: str
    tool_name: str
    provider_id: str


class AppPartial(ResponseModel):
    id: str
    name: str
    max_active_requests: int | None = None
    description: str | None = Field(default=None, validation_alias=AliasChoices("desc_or_prompt", "description"))
    mode: str = Field(validation_alias="mode_compatible_with_agent")
    icon_type: str | None = None
    icon: str | None = None
    icon_background: str | None = None
    model_config_: ModelConfigPartial | None = Field(
        default=None,
        validation_alias=AliasChoices("app_model_config", "model_config"),
        alias="model_config",
    )
    workflow: WorkflowPartial | None = None
    use_icon_as_answer_icon: bool | None = None
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None
    tags: list[Tag] = Field(default_factory=list)
    access_mode: str | None = None
    create_user_name: str | None = None
    author_name: str | None = None
    has_draft_trigger: bool | None = None

    @computed_field(return_type=str | None)  # type: ignore
    @property
    def icon_url(self) -> str | None:
        return _build_icon_url(self.icon_type, self.icon)

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class AppDetail(ResponseModel):
    id: str
    name: str
    description: str | None = None
    mode: str = Field(validation_alias="mode_compatible_with_agent")
    icon: str | None = None
    icon_background: str | None = None
    enable_site: bool
    enable_api: bool
    model_config_: ModelConfig | None = Field(
        default=None,
        validation_alias=AliasChoices("app_model_config", "model_config"),
        alias="model_config",
    )
    workflow: WorkflowPartial | None = None
    tracing: JSONValue | None = None
    use_icon_as_answer_icon: bool | None = None
    created_by: str | None = None
    created_at: int | None = None
    updated_by: str | None = None
    updated_at: int | None = None
    access_mode: str | None = None
    tags: list[Tag] = Field(default_factory=list)

    @field_validator("created_at", "updated_at", mode="before")
    @classmethod
    def _normalize_timestamp(cls, value: datetime | int | None) -> int | None:
        return _to_timestamp(value)


class AppDetailWithSite(AppDetail):
    icon_type: str | None = None
    api_base_url: str | None = None
    max_active_requests: int | None = None
    deleted_tools: list[DeletedTool] = Field(default_factory=list)
    site: Site | None = None

    @computed_field(return_type=str | None)  # type: ignore
    @property
    def icon_url(self) -> str | None:
        return _build_icon_url(self.icon_type, self.icon)


class AppPagination(ResponseModel):
    page: int
    limit: int = Field(validation_alias=AliasChoices("per_page", "limit"))
    total: int
    has_more: bool = Field(validation_alias=AliasChoices("has_next", "has_more"))
    data: list[AppPartial] = Field(validation_alias=AliasChoices("items", "data"))


class AppExportResponse(ResponseModel):
    data: str


register_enum_models(console_ns, RetrievalMethod, WorkflowExecutionStatus, DatasetPermissionEnum)

register_schema_models(
    console_ns,
    AppListQuery,
    CreateAppPayload,
    UpdateAppPayload,
    CopyAppPayload,
    AppExportQuery,
    AppNamePayload,
    AppIconPayload,
    AppSiteStatusPayload,
    AppApiStatusPayload,
    AppTracePayload,
    Tag,
    WorkflowPartial,
    ModelConfigPartial,
    ModelConfig,
    Site,
    DeletedTool,
    AppPartial,
    AppDetail,
    AppDetailWithSite,
    AppPagination,
    AppExportResponse,
    Segmentation,
    PreProcessingRule,
    Rule,
    WeightVectorSetting,
    WeightKeywordSetting,
    WeightModel,
    RerankingModel,
    InfoList,
    NotionInfo,
    FileInfo,
    WebsiteInfo,
    NotionPage,
    NotionIcon,
    RerankingModel,
    DataSource,
    LoadBalancingPayload,
)


@console_ns.route("/apps")
class AppListApi(Resource):
    @console_ns.doc("list_apps")
    @console_ns.doc(description="Get list of applications with pagination and filtering")
    @console_ns.expect(console_ns.models[AppListQuery.__name__])
    @console_ns.response(200, "Success", console_ns.models[AppPagination.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self):
        """Get app list"""
        current_user, current_tenant_id = current_account_with_tenant()

        args = AppListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        args_dict = args.model_dump()

        # get app list
        app_service = AppService()
        app_pagination = app_service.get_paginate_apps(current_user.id, current_tenant_id, args_dict)
        if not app_pagination:
            empty = AppPagination(page=args.page, limit=args.limit, total=0, has_more=False, data=[])
            return empty.model_dump(mode="json"), 200

        if FeatureService.get_system_features().webapp_auth.enabled:
            app_ids = [str(app.id) for app in app_pagination.items]
            res = EnterpriseService.WebAppAuth.batch_get_app_access_mode_by_id(app_ids=app_ids)
            if len(res) != len(app_ids):
                raise BadRequest("Invalid app id in webapp auth")

            for app in app_pagination.items:
                if str(app.id) in res:
                    app.access_mode = res[str(app.id)].access_mode

        workflow_capable_app_ids = [
            str(app.id) for app in app_pagination.items if app.mode in {"workflow", "advanced-chat"}
        ]
        draft_trigger_app_ids: set[str] = set()
        if workflow_capable_app_ids:
            draft_workflows = (
                db.session.execute(
                    select(Workflow).where(
                        Workflow.version == Workflow.VERSION_DRAFT,
                        Workflow.app_id.in_(workflow_capable_app_ids),
                        Workflow.tenant_id == current_tenant_id,
                    )
                )
                .scalars()
                .all()
            )
            trigger_node_types = TRIGGER_NODE_TYPES
            for workflow in draft_workflows:
                node_id = None
                try:
                    for node_id, node_data in workflow.walk_nodes():
                        if node_data.get("type") in trigger_node_types:
                            draft_trigger_app_ids.add(str(workflow.app_id))
                            break
                except Exception:
                    _logger.exception("error while walking nodes, workflow_id=%s, node_id=%s", workflow.id, node_id)
                    continue

        for app in app_pagination.items:
            app.has_draft_trigger = str(app.id) in draft_trigger_app_ids

        pagination_model = AppPagination.model_validate(app_pagination, from_attributes=True)
        return pagination_model.model_dump(mode="json"), 200

    @console_ns.doc("create_app")
    @console_ns.doc(description="Create a new application")
    @console_ns.expect(console_ns.models[CreateAppPayload.__name__])
    @console_ns.response(201, "App created successfully", console_ns.models[AppDetail.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("apps")
    @edit_permission_required
    def post(self):
        """Create app"""
        current_user, current_tenant_id = current_account_with_tenant()
        args = CreateAppPayload.model_validate(console_ns.payload)

        app_service = AppService()
        app = app_service.create_app(current_tenant_id, args.model_dump(), current_user)
        app_detail = AppDetail.model_validate(app, from_attributes=True)
        return app_detail.model_dump(mode="json"), 201


@console_ns.route("/apps/<uuid:app_id>")
class AppApi(Resource):
    @console_ns.doc("get_app_detail")
    @console_ns.doc(description="Get application details")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Success", console_ns.models[AppDetailWithSite.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    @get_app_model(mode=None)
    def get(self, app_model):
        """Get app detail"""
        app_service = AppService()

        app_model = app_service.get_app(app_model)

        if FeatureService.get_system_features().webapp_auth.enabled:
            app_setting = EnterpriseService.WebAppAuth.get_app_access_mode_by_id(app_id=str(app_model.id))
            app_model.access_mode = app_setting.access_mode

        response_model = AppDetailWithSite.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")

    @console_ns.doc("update_app")
    @console_ns.doc(description="Update application details")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[UpdateAppPayload.__name__])
    @console_ns.response(200, "App updated successfully", console_ns.models[AppDetailWithSite.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=None)
    @edit_permission_required
    def put(self, app_model):
        """Update app"""
        args = UpdateAppPayload.model_validate(console_ns.payload)

        app_service = AppService()

        args_dict: AppService.ArgsDict = {
            "name": args.name,
            "description": args.description or "",
            "icon_type": args.icon_type,
            "icon": args.icon or "",
            "icon_background": args.icon_background or "",
            "use_icon_as_answer_icon": args.use_icon_as_answer_icon or False,
            "max_active_requests": args.max_active_requests or 0,
        }
        app_model = app_service.update_app(app_model, args_dict)
        response_model = AppDetailWithSite.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")

    @console_ns.doc("delete_app")
    @console_ns.doc(description="Delete application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(204, "App deleted successfully")
    @console_ns.response(403, "Insufficient permissions")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def delete(self, app_model):
        """Delete app"""
        app_service = AppService()
        app_service.delete_app(app_model)

        return {"result": "success"}, 204


@console_ns.route("/apps/<uuid:app_id>/copy")
class AppCopyApi(Resource):
    @console_ns.doc("copy_app")
    @console_ns.doc(description="Create a copy of an existing application")
    @console_ns.doc(params={"app_id": "Application ID to copy"})
    @console_ns.expect(console_ns.models[CopyAppPayload.__name__])
    @console_ns.response(201, "App copied successfully", console_ns.models[AppDetailWithSite.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=None)
    @edit_permission_required
    def post(self, app_model):
        """Copy app"""
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        args = CopyAppPayload.model_validate(console_ns.payload or {})

        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            import_service = AppDslService(session)
            yaml_content = import_service.export_dsl(app_model=app_model, include_secret=True)
            result = import_service.import_app(
                account=current_user,
                import_mode=ImportMode.YAML_CONTENT,
                yaml_content=yaml_content,
                name=args.name,
                description=args.description,
                icon_type=args.icon_type,
                icon=args.icon,
                icon_background=args.icon_background,
            )

            # Inherit web app permission from original app
            if result.app_id and FeatureService.get_system_features().webapp_auth.enabled:
                try:
                    # Get the original app's access mode
                    original_settings = EnterpriseService.WebAppAuth.get_app_access_mode_by_id(app_model.id)
                    access_mode = original_settings.access_mode
                except Exception:
                    # If original app has no settings (old app), default to public to match fallback behavior
                    access_mode = "public"

                # Apply the same access mode to the copied app
                EnterpriseService.WebAppAuth.update_app_access_mode(result.app_id, access_mode)

            stmt = select(App).where(App.id == result.app_id)
            app = session.scalar(stmt)

        response_model = AppDetailWithSite.model_validate(app, from_attributes=True)
        return response_model.model_dump(mode="json"), 201


@console_ns.route("/apps/<uuid:app_id>/export")
class AppExportApi(Resource):
    @console_ns.doc("export_app")
    @console_ns.doc(description="Export application configuration as DSL")
    @console_ns.doc(params={"app_id": "Application ID to export"})
    @console_ns.expect(console_ns.models[AppExportQuery.__name__])
    @console_ns.response(200, "App exported successfully", console_ns.models[AppExportResponse.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, app_model):
        """Export app"""
        args = AppExportQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        payload = AppExportResponse(
            data=AppDslService.export_dsl(
                app_model=app_model,
                include_secret=args.include_secret,
                workflow_id=args.workflow_id,
            )
        )
        return payload.model_dump(mode="json")


@console_ns.route("/apps/<uuid:app_id>/name")
class AppNameApi(Resource):
    @console_ns.doc("check_app_name")
    @console_ns.doc(description="Check if app name is available")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppNamePayload.__name__])
    @console_ns.response(200, "Name availability checked", console_ns.models[AppDetail.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=None)
    @edit_permission_required
    def post(self, app_model):
        args = AppNamePayload.model_validate(console_ns.payload)

        app_service = AppService()
        app_model = app_service.update_app_name(app_model, args.name)
        response_model = AppDetail.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")


@console_ns.route("/apps/<uuid:app_id>/icon")
class AppIconApi(Resource):
    @console_ns.doc("update_app_icon")
    @console_ns.doc(description="Update application icon")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppIconPayload.__name__])
    @console_ns.response(200, "Icon updated successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=None)
    @edit_permission_required
    def post(self, app_model):
        args = AppIconPayload.model_validate(console_ns.payload or {})

        app_service = AppService()
        app_model = app_service.update_app_icon(app_model, args.icon or "", args.icon_background or "")
        response_model = AppDetail.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")


@console_ns.route("/apps/<uuid:app_id>/site-enable")
class AppSiteStatus(Resource):
    @console_ns.doc("update_app_site_status")
    @console_ns.doc(description="Enable or disable app site")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppSiteStatusPayload.__name__])
    @console_ns.response(200, "Site status updated successfully", console_ns.models[AppDetail.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=None)
    @edit_permission_required
    def post(self, app_model):
        args = AppSiteStatusPayload.model_validate(console_ns.payload)

        app_service = AppService()
        app_model = app_service.update_app_site_status(app_model, args.enable_site)
        response_model = AppDetail.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")


@console_ns.route("/apps/<uuid:app_id>/api-enable")
class AppApiStatus(Resource):
    @console_ns.doc("update_app_api_status")
    @console_ns.doc(description="Enable or disable app API")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppApiStatusPayload.__name__])
    @console_ns.response(200, "API status updated successfully", console_ns.models[AppDetail.__name__])
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    @get_app_model(mode=None)
    def post(self, app_model):
        args = AppApiStatusPayload.model_validate(console_ns.payload)

        app_service = AppService()
        app_model = app_service.update_app_api_status(app_model, args.enable_api)
        response_model = AppDetail.model_validate(app_model, from_attributes=True)
        return response_model.model_dump(mode="json")


@console_ns.route("/apps/<uuid:app_id>/trace")
class AppTraceApi(Resource):
    @console_ns.doc("get_app_trace")
    @console_ns.doc(description="Get app tracing configuration")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "Trace configuration retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_id):
        """Get app trace"""
        app_trace_config = OpsTraceManager.get_app_tracing_config(app_id=app_id)

        return app_trace_config

    @console_ns.doc("update_app_trace")
    @console_ns.doc(description="Update app tracing configuration")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[AppTracePayload.__name__])
    @console_ns.response(200, "Trace configuration updated successfully")
    @console_ns.response(403, "Insufficient permissions")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, app_id):
        # add app trace
        args = AppTracePayload.model_validate(console_ns.payload)

        OpsTraceManager.update_app_tracing_config(
            app_id=app_id,
            enabled=args.enabled,
            tracing_provider=args.tracing_provider,
        )

        return {"result": "success"}

```

### api/controllers/console/app/audio.py
```py
import logging

from flask import request
from flask_restx import Resource, fields
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from werkzeug.exceptions import InternalServerError

import services
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.app.error import (
    AppUnavailableError,
    AudioTooLargeError,
    CompletionRequestError,
    NoAudioUploadedError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderNotSupportSpeechToTextError,
    ProviderQuotaExceededError,
    UnsupportedAudioTypeError,
)
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from libs.login import login_required
from models import App, AppMode
from services.audio_service import AudioService
from services.errors.audio import (
    AudioTooLargeServiceError,
    NoAudioUploadedServiceError,
    ProviderNotSupportSpeechToTextServiceError,
    UnsupportedAudioTypeServiceError,
)

logger = logging.getLogger(__name__)


class TextToSpeechPayload(BaseModel):
    message_id: str | None = Field(default=None, description="Message ID")
    text: str = Field(..., description="Text to convert")
    voice: str | None = Field(default=None, description="Voice name")
    streaming: bool | None = Field(default=None, description="Whether to stream audio")


class TextToSpeechVoiceQuery(BaseModel):
    language: str = Field(..., description="Language code")


class AudioTranscriptResponse(BaseModel):
    text: str = Field(description="Transcribed text from audio")


register_schema_models(console_ns, AudioTranscriptResponse, TextToSpeechPayload, TextToSpeechVoiceQuery)


@console_ns.route("/apps/<uuid:app_id>/audio-to-text")
class ChatMessageAudioApi(Resource):
    @console_ns.doc("chat_message_audio_transcript")
    @console_ns.doc(description="Transcript audio to text for chat messages")
    @console_ns.doc(params={"app_id": "App ID"})
    @console_ns.response(
        200,
        "Audio transcription successful",
        console_ns.models[AudioTranscriptResponse.__name__],
    )
    @console_ns.response(400, "Bad request - No audio uploaded or unsupported type")
    @console_ns.response(413, "Audio file too large")
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    def post(self, app_model):
        file = request.files["file"]

        try:
            response = AudioService.transcript_asr(
                app_model=app_model,
                file=file,
                end_user=None,
            )

            return response
        except services.errors.app_model_config.AppModelConfigBrokenError:
            logger.exception("App model config broken.")
            raise AppUnavailableError()
        except NoAudioUploadedServiceError:
            raise NoAudioUploadedError()
        except AudioTooLargeServiceError as e:
            raise AudioTooLargeError(str(e))
        except UnsupportedAudioTypeServiceError:
            raise UnsupportedAudioTypeError()
        except ProviderNotSupportSpeechToTextServiceError:
            raise ProviderNotSupportSpeechToTextError()
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
        except Exception as e:
            logger.exception("Failed to handle post request to ChatMessageAudioApi")
            raise InternalServerError()


@console_ns.route("/apps/<uuid:app_id>/text-to-audio")
class ChatMessageTextApi(Resource):
    @console_ns.doc("chat_message_text_to_speech")
    @console_ns.doc(description="Convert text to speech for chat messages")
    @console_ns.doc(params={"app_id": "App ID"})
    @console_ns.expect(console_ns.models[TextToSpeechPayload.__name__])
    @console_ns.response(200, "Text to speech conversion successful")
    @console_ns.response(400, "Bad request - Invalid parameters")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, app_model: App):
        try:
            payload = TextToSpeechPayload.model_validate(console_ns.payload)

            response = AudioService.transcript_tts(
                app_model=app_model,
                text=payload.text,
                voice=payload.voice,
                message_id=payload.message_id,
                is_draft=True,
            )
            return response
        except services.errors.app_model_config.AppModelConfigBrokenError:
            logger.exception("App model config broken.")
            raise AppUnavailableError()
        except NoAudioUploadedServiceError:
            raise NoAudioUploadedError()
        except AudioTooLargeServiceError as e:
            raise AudioTooLargeError(str(e))
        except UnsupportedAudioTypeServiceError:
            raise UnsupportedAudioTypeError()
        except ProviderNotSupportSpeechToTextServiceError:
            raise ProviderNotSupportSpeechToTextError()
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
        except Exception as e:
            logger.exception("Failed to handle post request to ChatMessageTextApi")
            raise InternalServerError()


@console_ns.route("/apps/<uuid:app_id>/text-to-audio/voices")
class TextModesApi(Resource):
    @console_ns.doc("get_text_to_speech_voices")
    @console_ns.doc(description="Get available TTS voices for a specific language")
    @console_ns.doc(params={"app_id": "App ID"})
    @console_ns.expect(console_ns.models[TextToSpeechVoiceQuery.__name__])
    @console_ns.response(
        200, "TTS voices retrieved successfully", fields.List(fields.Raw(description="Available voices"))
    )
    @console_ns.response(400, "Invalid language parameter")
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        try:
            args = TextToSpeechVoiceQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

            response = AudioService.transcript_tts_voices(
                tenant_id=app_model.tenant_id,
                language=args.language,
            )

            return response
        except services.errors.audio.ProviderNotSupportTextToSpeechLanageServiceError:
            raise AppUnavailableError("Text to audio voices language parameter loss.")
        except NoAudioUploadedServiceError:
            raise NoAudioUploadedError()
        except AudioTooLargeServiceError as e:
            raise AudioTooLargeError(str(e))
        except UnsupportedAudioTypeServiceError:
            raise UnsupportedAudioTypeError()
        except ProviderNotSupportSpeechToTextServiceError:
            raise ProviderNotSupportSpeechToTextError()
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
        except Exception as e:
            logger.exception("Failed to handle get request to TextModesApi")
            raise InternalServerError()

```

### api/controllers/console/app/wraps.py
```py
from collections.abc import Callable
from functools import wraps
from typing import overload

from sqlalchemy import select

from controllers.console.app.error import AppNotFoundError
from extensions.ext_database import db
from libs.login import current_account_with_tenant
from models import App, AppMode


def _load_app_model(app_id: str) -> App | None:
    _, current_tenant_id = current_account_with_tenant()
    app_model = db.session.scalar(
        select(App).where(App.id == app_id, App.tenant_id == current_tenant_id, App.status == "normal").limit(1)
    )
    return app_model


def _load_app_model_with_trial(app_id: str) -> App | None:
    app_model = db.session.scalar(select(App).where(App.id == app_id, App.status == "normal").limit(1))
    return app_model


@overload
def get_app_model[**P, R](
    view: Callable[P, R],
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[P, R]: ...


@overload
def get_app_model[**P, R](
    view: None = None,
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[[Callable[P, R]], Callable[P, R]]: ...


def get_app_model[**P, R](
    view: Callable[P, R] | None = None,
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[P, R] | Callable[[Callable[P, R]], Callable[P, R]]:
    def decorator(view_func: Callable[P, R]) -> Callable[P, R]:
        @wraps(view_func)
        def decorated_view(*args: P.args, **kwargs: P.kwargs) -> R:
            if not kwargs.get("app_id"):
                raise ValueError("missing app_id in path parameters")

            app_id = kwargs.get("app_id")
            app_id = str(app_id)

            del kwargs["app_id"]

            app_model = _load_app_model(app_id)

            if not app_model:
                raise AppNotFoundError()

            app_mode = AppMode.value_of(app_model.mode)

            if mode is not None:
                if isinstance(mode, list):
                    modes = mode
                else:
                    modes = [mode]

                if app_mode not in modes:
                    mode_values = {m.value for m in modes}
                    raise AppNotFoundError(f"App mode is not in the supported list: {mode_values}")

            kwargs["app_model"] = app_model

            return view_func(*args, **kwargs)

        return decorated_view

    if view is None:
        return decorator
    else:
        return decorator(view)


@overload
def get_app_model_with_trial[**P, R](
    view: Callable[P, R],
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[P, R]: ...


@overload
def get_app_model_with_trial[**P, R](
    view: None = None,
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[[Callable[P, R]], Callable[P, R]]: ...


def get_app_model_with_trial[**P, R](
    view: Callable[P, R] | None = None,
    *,
    mode: AppMode | list[AppMode] | None = None,
) -> Callable[P, R] | Callable[[Callable[P, R]], Callable[P, R]]:
    def decorator(view_func: Callable[P, R]) -> Callable[P, R]:
        @wraps(view_func)
        def decorated_view(*args: P.args, **kwargs: P.kwargs) -> R:
            if not kwargs.get("app_id"):
                raise ValueError("missing app_id in path parameters")

            app_id = kwargs.get("app_id")
            app_id = str(app_id)

            del kwargs["app_id"]

            app_model = _load_app_model_with_trial(app_id)

            if not app_model:
                raise AppNotFoundError()

            app_mode = AppMode.value_of(app_model.mode)

            if mode is not None:
                if isinstance(mode, list):
                    modes = mode
                else:
                    modes = [mode]

                if app_mode not in modes:
                    mode_values = {m.value for m in modes}
                    raise AppNotFoundError(f"App mode is not in the supported list: {mode_values}")

            kwargs["app_model"] = app_model

            return view_func(*args, **kwargs)

        return decorated_view

    if view is None:
        return decorator
    else:
        return decorator(view)

```

### api/controllers/console/app/app_import.py
```py
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel, Field
from sqlalchemy.orm import Session, sessionmaker

from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_resource_check,
    edit_permission_required,
    setup_required,
)
from extensions.ext_database import db
from fields.app_fields import (
    app_import_check_dependencies_fields,
    app_import_fields,
    leaked_dependency_fields,
)
from libs.login import current_account_with_tenant, login_required
from models.model import App
from services.app_dsl_service import AppDslService, ImportStatus
from services.enterprise.enterprise_service import EnterpriseService
from services.feature_service import FeatureService

from .. import console_ns

# Register models for flask_restx to avoid dict type issues in Swagger
# Register base model first
leaked_dependency_model = console_ns.model("LeakedDependency", leaked_dependency_fields)

app_import_model = console_ns.model("AppImport", app_import_fields)

# For nested models, need to replace nested dict with registered model
app_import_check_dependencies_fields_copy = app_import_check_dependencies_fields.copy()
app_import_check_dependencies_fields_copy["leaked_dependencies"] = fields.List(fields.Nested(leaked_dependency_model))
app_import_check_dependencies_model = console_ns.model(
    "AppImportCheckDependencies", app_import_check_dependencies_fields_copy
)

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class AppImportPayload(BaseModel):
    mode: str = Field(..., description="Import mode")
    yaml_content: str | None = Field(None)
    yaml_url: str | None = Field(None)
    name: str | None = Field(None)
    description: str | None = Field(None)
    icon_type: str | None = Field(None)
    icon: str | None = Field(None)
    icon_background: str | None = Field(None)
    app_id: str | None = Field(None)


console_ns.schema_model(
    AppImportPayload.__name__, AppImportPayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/apps/imports")
class AppImportApi(Resource):
    @console_ns.expect(console_ns.models[AppImportPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(app_import_model)
    @cloud_edition_billing_resource_check("apps")
    @edit_permission_required
    def post(self):
        # Check user role first
        current_user, _ = current_account_with_tenant()
        args = AppImportPayload.model_validate(console_ns.payload)

        # Create service with session
        with Session(db.engine) as session:
            import_service = AppDslService(session)
            # Import app
            account = current_user
            result = import_service.import_app(
                account=account,
                import_mode=args.mode,
                yaml_content=args.yaml_content,
                yaml_url=args.yaml_url,
                name=args.name,
                description=args.description,
                icon_type=args.icon_type,
                icon=args.icon,
                icon_background=args.icon_background,
                app_id=args.app_id,
            )
        if result.app_id and FeatureService.get_system_features().webapp_auth.enabled:
            # update web app setting as private
            EnterpriseService.WebAppAuth.update_app_access_mode(result.app_id, "private")
        # Return appropriate status code based on result
        status = result.status
        if status == ImportStatus.FAILED:
            return result.model_dump(mode="json"), 400
        elif status == ImportStatus.PENDING:
            return result.model_dump(mode="json"), 202
        return result.model_dump(mode="json"), 200


@console_ns.route("/apps/imports/<string:import_id>/confirm")
class AppImportConfirmApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(app_import_model)
    @edit_permission_required
    def post(self, import_id):
        # Check user role first
        current_user, _ = current_account_with_tenant()

        # Create service with session
        with sessionmaker(db.engine).begin() as session:
            import_service = AppDslService(session)
            # Confirm import
            account = current_user
            result = import_service.confirm_import(import_id=import_id, account=account)

        # Return appropriate status code based on result
        if result.status == ImportStatus.FAILED:
            return result.model_dump(mode="json"), 400
        return result.model_dump(mode="json"), 200


@console_ns.route("/apps/imports/<string:app_id>/check-dependencies")
class AppImportCheckDependenciesApi(Resource):
    @setup_required
    @login_required
    @get_app_model
    @account_initialization_required
    @marshal_with(app_import_check_dependencies_model)
    @edit_permission_required
    def get(self, app_model: App):
        with sessionmaker(db.engine).begin() as session:
            import_service = AppDslService(session)
            result = import_service.check_dependencies(app_model=app_model)

        return result.model_dump(mode="json"), 200

```

### api/controllers/console/app/statistic.py
```py
from decimal import Decimal

import sqlalchemy as sa
from flask import abort, jsonify, request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field, field_validator

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from libs.datetime_utils import parse_time_range
from libs.helper import convert_datetime_to_date
from libs.login import current_account_with_tenant, login_required
from models import AppMode

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class StatisticTimeRangeQuery(BaseModel):
    start: str | None = Field(default=None, description="Start date (YYYY-MM-DD HH:MM)")
    end: str | None = Field(default=None, description="End date (YYYY-MM-DD HH:MM)")

    @field_validator("start", "end", mode="before")
    @classmethod
    def empty_string_to_none(cls, value: str | None) -> str | None:
        if value == "":
            return None
        return value


console_ns.schema_model(
    StatisticTimeRangeQuery.__name__,
    StatisticTimeRangeQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)


@console_ns.route("/apps/<uuid:app_id>/statistics/daily-messages")
class DailyMessageStatistic(Resource):
    @console_ns.doc("get_daily_message_statistics")
    @console_ns.doc(description="Get daily message statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Daily message statistics retrieved successfully",
        fields.List(fields.Raw(description="Daily message count data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    COUNT(*) AS message_count
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append({"date": str(i.date), "message_count": i.message_count})

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/daily-conversations")
class DailyConversationStatistic(Resource):
    @console_ns.doc("get_daily_conversation_statistics")
    @console_ns.doc(description="Get daily conversation statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Daily conversation statistics retrieved successfully",
        fields.List(fields.Raw(description="Daily conversation count data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    COUNT(DISTINCT conversation_id) AS conversation_count
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []
        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append({"date": str(i.date), "conversation_count": i.conversation_count})

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/daily-end-users")
class DailyTerminalsStatistic(Resource):
    @console_ns.doc("get_daily_terminals_statistics")
    @console_ns.doc(description="Get daily terminal/end-user statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Daily terminal statistics retrieved successfully",
        fields.List(fields.Raw(description="Daily terminal count data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    COUNT(DISTINCT messages.from_end_user_id) AS terminal_count
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append({"date": str(i.date), "terminal_count": i.terminal_count})

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/token-costs")
class DailyTokenCostStatistic(Resource):
    @console_ns.doc("get_daily_token_cost_statistics")
    @console_ns.doc(description="Get daily token cost statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Daily token cost statistics retrieved successfully",
        fields.List(fields.Raw(description="Daily token cost data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    (SUM(messages.message_tokens) + SUM(messages.answer_tokens)) AS token_count,
    SUM(total_price) AS total_price
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append(
                    {"date": str(i.date), "token_count": i.token_count, "total_price": i.total_price, "currency": "USD"}
                )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/average-session-interactions")
class AverageSessionInteractionStatistic(Resource):
    @console_ns.doc("get_average_session_interaction_statistics")
    @console_ns.doc(description="Get average session interaction statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Average session interaction statistics retrieved successfully",
        fields.List(fields.Raw(description="Average session interaction data")),
    )
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT])
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("c.created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    AVG(subquery.message_count) AS interactions
FROM
    (
        SELECT
            m.conversation_id,
            COUNT(m.id) AS message_count
        FROM
            conversations c
        JOIN
            messages m
            ON c.id = m.conversation_id
        WHERE
            c.app_id = :app_id
            AND m.invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND c.created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND c.created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += """
        GROUP BY m.conversation_id
    ) subquery
LEFT JOIN
    conversations c
    ON c.id = subquery.conversation_id
GROUP BY
    date
ORDER BY
    date"""

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append(
                    {"date": str(i.date), "interactions": float(i.interactions.quantize(Decimal("0.01")))}
                )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/user-satisfaction-rate")
class UserSatisfactionRateStatistic(Resource):
    @console_ns.doc("get_user_satisfaction_rate_statistics")
    @console_ns.doc(description="Get user satisfaction rate statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "User satisfaction rate statistics retrieved successfully",
        fields.List(fields.Raw(description="User satisfaction rate data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("m.created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    COUNT(m.id) AS message_count,
    COUNT(mf.id) AS feedback_count
FROM
    messages m
LEFT JOIN
    message_feedbacks mf
    ON mf.message_id=m.id AND mf.rating='like'
WHERE
    m.app_id = :app_id
    AND m.invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND m.created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND m.created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append(
                    {
                        "date": str(i.date),
                        "rate": round((i.feedback_count * 1000 / i.message_count) if i.message_count > 0 else 0, 2),
                    }
                )

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/average-response-time")
class AverageResponseTimeStatistic(Resource):
    @console_ns.doc("get_average_response_time_statistics")
    @console_ns.doc(description="Get average response time statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Average response time statistics retrieved successfully",
        fields.List(fields.Raw(description="Average response time data")),
    )
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=AppMode.COMPLETION)
    def get(self, app_model):
        account, _ = current_account_with_tenant()

        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    AVG(provider_response_latency) AS latency
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append({"date": str(i.date), "latency": round(i.latency * 1000, 4)})

        return jsonify({"data": response_data})


@console_ns.route("/apps/<uuid:app_id>/statistics/tokens-per-second")
class TokensPerSecondStatistic(Resource):
    @console_ns.doc("get_tokens_per_second_statistics")
    @console_ns.doc(description="Get tokens per second statistics for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[StatisticTimeRangeQuery.__name__])
    @console_ns.response(
        200,
        "Tokens per second statistics retrieved successfully",
        fields.List(fields.Raw(description="Tokens per second data")),
    )
    @get_app_model
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, app_model):
        account, _ = current_account_with_tenant()
        args = StatisticTimeRangeQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        converted_created_at = convert_datetime_to_date("created_at")
        sql_query = f"""SELECT
    {converted_created_at} AS date,
    CASE
        WHEN SUM(provider_response_latency) = 0 THEN 0
        ELSE (SUM(answer_tokens) / SUM(provider_response_latency))
    END as tokens_per_second
FROM
    messages
WHERE
    app_id = :app_id
    AND invoke_from != :invoke_from"""
        arg_dict = {"tz": account.timezone, "app_id": app_model.id, "invoke_from": InvokeFrom.DEBUGGER}
        assert account.timezone is not None

        try:
            start_datetime_utc, end_datetime_utc = parse_time_range(args.start, args.end, account.timezone)
        except ValueError as e:
            abort(400, description=str(e))

        if start_datetime_utc:
            sql_query += " AND created_at >= :start"
            arg_dict["start"] = start_datetime_utc

        if end_datetime_utc:
            sql_query += " AND created_at < :end"
            arg_dict["end"] = end_datetime_utc

        sql_query += " GROUP BY date ORDER BY date"

        response_data = []

        with db.engine.begin() as conn:
            rs = conn.execute(sa.text(sql_query), arg_dict)
            for i in rs:
                response_data.append({"date": str(i.date), "tps": round(i.tokens_per_second, 4)})

        return jsonify({"data": response_data})

```

### api/controllers/console/app/workflow_app_log.py
```py
from datetime import datetime

from dateutil.parser import isoparse
from flask import request
from flask_restx import Resource, marshal_with
from graphon.enums import WorkflowExecutionStatus
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, setup_required
from extensions.ext_database import db
from fields.workflow_app_log_fields import (
    build_workflow_app_log_pagination_model,
    build_workflow_archived_log_pagination_model,
)
from libs.login import login_required
from models import App
from models.model import AppMode
from services.workflow_app_service import WorkflowAppService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class WorkflowAppLogQuery(BaseModel):
    keyword: str | None = Field(default=None, description="Search keyword for filtering logs")
    status: WorkflowExecutionStatus | None = Field(
        default=None, description="Execution status filter (succeeded, failed, stopped, partial-succeeded)"
    )
    created_at__before: datetime | None = Field(default=None, description="Filter logs created before this timestamp")
    created_at__after: datetime | None = Field(default=None, description="Filter logs created after this timestamp")
    created_by_end_user_session_id: str | None = Field(default=None, description="Filter by end user session ID")
    created_by_account: str | None = Field(default=None, description="Filter by account")
    detail: bool = Field(default=False, description="Whether to return detailed logs")
    page: int = Field(default=1, ge=1, le=99999, description="Page number (1-99999)")
    limit: int = Field(default=20, ge=1, le=100, description="Number of items per page (1-100)")

    @field_validator("created_at__before", "created_at__after", mode="before")
    @classmethod
    def parse_datetime(cls, value: str | None) -> datetime | None:
        if value in (None, ""):
            return None
        return isoparse(value)  # type: ignore

    @field_validator("detail", mode="before")
    @classmethod
    def parse_bool(cls, value: bool | str | None) -> bool:
        if isinstance(value, bool):
            return value
        if value is None:
            return False
        lowered = value.lower()
        if lowered in {"1", "true", "yes", "on"}:
            return True
        if lowered in {"0", "false", "no", "off"}:
            return False
        raise ValueError("Invalid boolean value for detail")


console_ns.schema_model(
    WorkflowAppLogQuery.__name__, WorkflowAppLogQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)

# Register model for flask_restx to avoid dict type issues in Swagger
workflow_app_log_pagination_model = build_workflow_app_log_pagination_model(console_ns)
workflow_archived_log_pagination_model = build_workflow_archived_log_pagination_model(console_ns)


@console_ns.route("/apps/<uuid:app_id>/workflow-app-logs")
class WorkflowAppLogApi(Resource):
    @console_ns.doc("get_workflow_app_logs")
    @console_ns.doc(description="Get workflow application execution logs")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowAppLogQuery.__name__])
    @console_ns.response(200, "Workflow app logs retrieved successfully", workflow_app_log_pagination_model)
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.WORKFLOW])
    @marshal_with(workflow_app_log_pagination_model)
    def get(self, app_model: App):
        """
        Get workflow app logs
        """
        args = WorkflowAppLogQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        # get paginate workflow app logs
        workflow_app_service = WorkflowAppService()
        with sessionmaker(db.engine).begin() as session:
            workflow_app_log_pagination = workflow_app_service.get_paginate_workflow_app_logs(
                session=session,
                app_model=app_model,
                keyword=args.keyword,
                status=args.status,
                created_at_before=args.created_at__before,
                created_at_after=args.created_at__after,
                page=args.page,
                limit=args.limit,
                detail=args.detail,
                created_by_end_user_session_id=args.created_by_end_user_session_id,
                created_by_account=args.created_by_account,
            )

            return workflow_app_log_pagination


@console_ns.route("/apps/<uuid:app_id>/workflow-archived-logs")
class WorkflowArchivedLogApi(Resource):
    @console_ns.doc("get_workflow_archived_logs")
    @console_ns.doc(description="Get workflow archived execution logs")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[WorkflowAppLogQuery.__name__])
    @console_ns.response(200, "Workflow archived logs retrieved successfully", workflow_archived_log_pagination_model)
    @setup_required
    @login_required
    @account_initialization_required
    @get_app_model(mode=[AppMode.WORKFLOW])
    @marshal_with(workflow_archived_log_pagination_model)
    def get(self, app_model: App):
        """
        Get workflow archived logs
        """
        args = WorkflowAppLogQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        workflow_app_service = WorkflowAppService()
        with sessionmaker(db.engine).begin() as session:
            workflow_app_log_pagination = workflow_app_service.get_paginate_workflow_archive_logs(
                session=session,
                app_model=app_model,
                page=args.page,
                limit=args.limit,
            )

            return workflow_app_log_pagination

```

### api/controllers/console/app/mcp_server.py
```py
import json

from flask_restx import Resource, marshal_with
from pydantic import BaseModel, Field
from sqlalchemy import select
from werkzeug.exceptions import NotFound

from controllers.console import console_ns
from controllers.console.app.wraps import get_app_model
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from extensions.ext_database import db
from fields.app_fields import app_server_fields
from libs.login import current_account_with_tenant, login_required
from models.enums import AppMCPServerStatus
from models.model import AppMCPServer

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"

# Register model for flask_restx to avoid dict type issues in Swagger
app_server_model = console_ns.model("AppServer", app_server_fields)


class MCPServerCreatePayload(BaseModel):
    description: str | None = Field(default=None, description="Server description")
    parameters: dict = Field(..., description="Server parameters configuration")


class MCPServerUpdatePayload(BaseModel):
    id: str = Field(..., description="Server ID")
    description: str | None = Field(default=None, description="Server description")
    parameters: dict = Field(..., description="Server parameters configuration")
    status: str | None = Field(default=None, description="Server status")


for model in (MCPServerCreatePayload, MCPServerUpdatePayload):
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/apps/<uuid:app_id>/server")
class AppMCPServerController(Resource):
    @console_ns.doc("get_app_mcp_server")
    @console_ns.doc(description="Get MCP server configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.response(200, "MCP server configuration retrieved successfully", app_server_model)
    @login_required
    @account_initialization_required
    @setup_required
    @get_app_model
    @marshal_with(app_server_model)
    def get(self, app_model):
        server = db.session.scalar(select(AppMCPServer).where(AppMCPServer.app_id == app_model.id).limit(1))
        return server

    @console_ns.doc("create_app_mcp_server")
    @console_ns.doc(description="Create MCP server configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[MCPServerCreatePayload.__name__])
    @console_ns.response(201, "MCP server configuration created successfully", app_server_model)
    @console_ns.response(403, "Insufficient permissions")
    @account_initialization_required
    @get_app_model
    @login_required
    @setup_required
    @marshal_with(app_server_model)
    @edit_permission_required
    def post(self, app_model):
        _, current_tenant_id = current_account_with_tenant()
        payload = MCPServerCreatePayload.model_validate(console_ns.payload or {})

        description = payload.description
        if not description:
            description = app_model.description or ""

        server = AppMCPServer(
            name=app_model.name,
            description=description,
            parameters=json.dumps(payload.parameters, ensure_ascii=False),
            status=AppMCPServerStatus.ACTIVE,
            app_id=app_model.id,
            tenant_id=current_tenant_id,
            server_code=AppMCPServer.generate_server_code(16),
        )
        db.session.add(server)
        db.session.commit()
        return server

    @console_ns.doc("update_app_mcp_server")
    @console_ns.doc(description="Update MCP server configuration for an application")
    @console_ns.doc(params={"app_id": "Application ID"})
    @console_ns.expect(console_ns.models[MCPServerUpdatePayload.__name__])
    @console_ns.response(200, "MCP server configuration updated successfully", app_server_model)
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Server not found")
    @get_app_model
    @login_required
    @setup_required
    @account_initialization_required
    @marshal_with(app_server_model)
    @edit_permission_required
    def put(self, app_model):
        payload = MCPServerUpdatePayload.model_validate(console_ns.payload or {})
        server = db.session.get(AppMCPServer, payload.id)
        if not server:
            raise NotFound()

        description = payload.description
        if description is None or not description:
            server.description = app_model.description or ""
        else:
            server.description = description

        server.name = app_model.name

        server.parameters = json.dumps(payload.parameters, ensure_ascii=False)
        if payload.status:
            try:
                server.status = AppMCPServerStatus(payload.status)
            except ValueError:
                raise ValueError("Invalid status")
        db.session.commit()
        return server


@console_ns.route("/apps/<uuid:server_id>/server/refresh")
class AppMCPServerRefreshController(Resource):
    @console_ns.doc("refresh_app_mcp_server")
    @console_ns.doc(description="Refresh MCP server configuration and regenerate server code")
    @console_ns.doc(params={"server_id": "Server ID"})
    @console_ns.response(200, "MCP server refreshed successfully", app_server_model)
    @console_ns.response(403, "Insufficient permissions")
    @console_ns.response(404, "Server not found")
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(app_server_model)
    @edit_permission_required
    def get(self, server_id):
        _, current_tenant_id = current_account_with_tenant()
        server = db.session.scalar(
            select(AppMCPServer)
            .where(AppMCPServer.id == server_id, AppMCPServer.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not server:
            raise NotFound()
        server.server_code = AppMCPServer.generate_server_code(16)
        db.session.commit()
        return server

```

### web/app/(shareLayout)/workflow/[token]/page.tsx
```tsx
import * as React from 'react'

import Main from '@/app/components/share/text-generation'
import AuthenticatedLayout from '../../components/authenticated-layout'

const Workflow = () => {
  return (
    <AuthenticatedLayout>
      <Main isWorkflow />
    </AuthenticatedLayout>
  )
}

export default React.memo(Workflow)

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-010.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
