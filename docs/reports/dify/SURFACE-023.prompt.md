You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-023
- **Kind**: endpoint
- **Identifier**: Observability/Tracing Integrations (Langfuse, LangSmith, MLflow, etc.)
- **Description**: Observability integrations sending trace data to external services. Risk of credential leakage in trace payloads, PII exposure in logged prompts/responses, and SSRF via user-configured tracing endpoints.
- **Locations**: api/core/ops/langfuse_trace/, api/core/ops/langsmith_trace/, api/core/ops/mlflow_trace/, api/core/ops/opik_trace/

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

### api/core/ops/langfuse_trace/__init__.py
```py

```

### api/core/ops/langfuse_trace/langfuse_trace.py
```py
import logging
import os
import uuid
from datetime import UTC, datetime, timedelta

from graphon.enums import BuiltinNodeTypes
from langfuse import Langfuse
from langfuse.api import (
    CreateGenerationBody,
    CreateSpanBody,
    IngestionEvent_GenerationCreate,
    IngestionEvent_SpanCreate,
    IngestionEvent_TraceCreate,
    TraceBody,
)
from langfuse.api.commons.types.usage import Usage
from sqlalchemy.orm import sessionmaker

from core.ops.base_trace_instance import BaseTraceInstance
from core.ops.entities.config_entity import LangfuseConfig
from core.ops.entities.trace_entity import (
    BaseTraceInfo,
    DatasetRetrievalTraceInfo,
    GenerateNameTraceInfo,
    MessageTraceInfo,
    ModerationTraceInfo,
    SuggestedQuestionTraceInfo,
    ToolTraceInfo,
    TraceTaskName,
    WorkflowTraceInfo,
)
from core.ops.langfuse_trace.entities.langfuse_trace_entity import (
    GenerationUsage,
    LangfuseGeneration,
    LangfuseSpan,
    LangfuseTrace,
    LevelEnum,
    UnitEnum,
)
from core.ops.utils import filter_none_values
from core.repositories import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models import EndUser, WorkflowNodeExecutionTriggeredFrom
from models.enums import MessageStatus

logger = logging.getLogger(__name__)


class LangFuseDataTrace(BaseTraceInstance):
    def __init__(
        self,
        langfuse_config: LangfuseConfig,
    ):
        super().__init__(langfuse_config)
        self.langfuse_client = Langfuse(
            public_key=langfuse_config.public_key,
            secret_key=langfuse_config.secret_key,
            host=langfuse_config.host,
        )
        self.file_base_url = os.getenv("FILES_URL", "http://127.0.0.1:5001")

    def trace(self, trace_info: BaseTraceInfo):
        if isinstance(trace_info, WorkflowTraceInfo):
            self.workflow_trace(trace_info)
        if isinstance(trace_info, MessageTraceInfo):
            self.message_trace(trace_info)
        if isinstance(trace_info, ModerationTraceInfo):
            self.moderation_trace(trace_info)
        if isinstance(trace_info, SuggestedQuestionTraceInfo):
            self.suggested_question_trace(trace_info)
        if isinstance(trace_info, DatasetRetrievalTraceInfo):
            self.dataset_retrieval_trace(trace_info)
        if isinstance(trace_info, ToolTraceInfo):
            self.tool_trace(trace_info)
        if isinstance(trace_info, GenerateNameTraceInfo):
            self.generate_name_trace(trace_info)

    def workflow_trace(self, trace_info: WorkflowTraceInfo):
        trace_id = trace_info.trace_id or trace_info.workflow_run_id
        user_id = trace_info.metadata.get("user_id")
        metadata = trace_info.metadata
        metadata["workflow_app_log_id"] = trace_info.workflow_app_log_id

        if trace_info.message_id:
            trace_id = trace_info.trace_id or trace_info.message_id
            name = TraceTaskName.MESSAGE_TRACE
            trace_data = LangfuseTrace(
                id=trace_id,
                user_id=user_id,
                name=name,
                input=dict(trace_info.workflow_run_inputs),
                output=dict(trace_info.workflow_run_outputs),
                metadata=metadata,
                session_id=trace_info.conversation_id,
                tags=["message", "workflow"],
                version=trace_info.workflow_run_version,
            )
            self.add_trace(langfuse_trace_data=trace_data)
            workflow_span_data = LangfuseSpan(
                id=trace_info.workflow_run_id,
                name=TraceTaskName.WORKFLOW_TRACE,
                input=dict(trace_info.workflow_run_inputs),
                output=dict(trace_info.workflow_run_outputs),
                trace_id=trace_id,
                start_time=trace_info.start_time,
                end_time=trace_info.end_time,
                metadata=metadata,
                level=LevelEnum.DEFAULT if trace_info.error == "" else LevelEnum.ERROR,
                status_message=trace_info.error or "",
            )
            self.add_span(langfuse_span_data=workflow_span_data)
        else:
            trace_data = LangfuseTrace(
                id=trace_id,
                user_id=user_id,
                name=TraceTaskName.WORKFLOW_TRACE,
                input=dict(trace_info.workflow_run_inputs),
                output=dict(trace_info.workflow_run_outputs),
                metadata=metadata,
                session_id=trace_info.conversation_id,
                tags=["workflow"],
                version=trace_info.workflow_run_version,
            )
            self.add_trace(langfuse_trace_data=trace_data)

        # through workflow_run_id get all_nodes_execution using repository
        session_factory = sessionmaker(bind=db.engine)
        # Find the app's creator account
        app_id = trace_info.metadata.get("app_id")
        if not app_id:
            raise ValueError("No app_id found in trace_info metadata")

        service_account = self.get_service_account_with_tenant(app_id)

        workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
            session_factory=session_factory,
            user=service_account,
            app_id=app_id,
            triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
        )

        # Get all executions for this workflow run
        workflow_node_executions = workflow_node_execution_repository.get_by_workflow_execution(
            workflow_execution_id=trace_info.workflow_run_id
        )

        for node_execution in workflow_node_executions:
            node_execution_id = node_execution.id
            tenant_id = trace_info.tenant_id  # Use from trace_info instead
            app_id = trace_info.metadata.get("app_id")  # Use from trace_info instead
            node_name = node_execution.title
            node_type = node_execution.node_type
            status = node_execution.status
            if node_type == BuiltinNodeTypes.LLM:
                inputs = node_execution.process_data.get("prompts", {}) if node_execution.process_data else {}
            else:
                inputs = node_execution.inputs or {}
            outputs = node_execution.outputs or {}
            created_at = node_execution.created_at or datetime.now()
            elapsed_time = node_execution.elapsed_time
            finished_at = created_at + timedelta(seconds=elapsed_time)

            execution_metadata = node_execution.metadata or {}
            metadata = {str(k): v for k, v in execution_metadata.items()}
            metadata.update(
                {
                    "workflow_run_id": trace_info.workflow_run_id,
                    "node_execution_id": node_execution_id,
                    "tenant_id": tenant_id,
                    "app_id": app_id,
                    "node_name": node_name,
                    "node_type": node_type,
                    "status": status,
                }
            )
            process_data = node_execution.process_data or {}
            model_provider = process_data.get("model_provider", None)
            model_name = process_data.get("model_name", None)
            if model_provider is not None and model_name is not None:
                metadata.update(
                    {
                        "model_provider": model_provider,
                        "model_name": model_name,
                    }
                )

            # add generation span
            if process_data and process_data.get("model_mode") == "chat":
                total_token = metadata.get("total_tokens", 0)
                prompt_tokens = 0
                completion_tokens = 0
                try:
                    usage_data = process_data.get("usage", {}) if "usage" in process_data else outputs.get("usage", {})
                    prompt_tokens = usage_data.get("prompt_tokens", 0)
                    completion_tokens = usage_data.get("completion_tokens", 0)
                except Exception:
                    logger.error("Failed to extract usage", exc_info=True)

                # add generation
                generation_usage = GenerationUsage(
                    input=prompt_tokens,
                    output=completion_tokens,
                    total=total_token,
                    unit=UnitEnum.TOKENS,
                )

                node_generation_data = LangfuseGeneration(
                    id=node_execution_id,
                    name=node_name,
                    trace_id=trace_id,
                    model=process_data.get("model_name"),
                    start_time=created_at,
                    end_time=finished_at,
                    input=inputs,
                    output=outputs,
                    metadata=metadata,
                    level=(LevelEnum.DEFAULT if status == "succeeded" else LevelEnum.ERROR),
                    status_message=trace_info.error or "",
                    parent_observation_id=trace_info.workflow_run_id if trace_info.message_id else None,
                    usage=generation_usage,
                )

                self.add_generation(langfuse_generation_data=node_generation_data)

            # add normal span
            else:
                span_data = LangfuseSpan(
                    id=node_execution_id,
                    name=node_name,
                    input=inputs,
                    output=outputs,
                    trace_id=trace_id,
                    start_time=created_at,
                    end_time=finished_at,
                    metadata=metadata,
                    level=(LevelEnum.DEFAULT if status == "succeeded" else LevelEnum.ERROR),
                    status_message=trace_info.error or "",
                    parent_observation_id=trace_info.workflow_run_id if trace_info.message_id else None,
                )

                self.add_span(langfuse_span_data=span_data)

    def message_trace(self, trace_info: MessageTraceInfo, **kwargs):
        # get message file data
        file_list = trace_info.file_list
        metadata = trace_info.metadata
        message_data = trace_info.message_data
        if message_data is None:
            return
        message_id = message_data.id

        user_id = message_data.from_account_id
        if message_data.from_end_user_id:
            end_user_data: EndUser | None = db.session.get(EndUser, message_data.from_end_user_id)
            if end_user_data is not None:
                user_id = end_user_data.session_id
                metadata["user_id"] = user_id

        trace_id = trace_info.trace_id or message_id

        trace_data = LangfuseTrace(
            id=trace_id,
            user_id=user_id,
            name=TraceTaskName.MESSAGE_TRACE,
            input={
                "message": trace_info.inputs,
                "files": file_list,
                "message_tokens": trace_info.message_tokens,
                "answer_tokens": trace_info.answer_tokens,
                "total_tokens": trace_info.total_tokens,
                "error": trace_info.error,
                "provider_response_latency": message_data.provider_response_latency,
                "created_at": trace_info.start_time,
            },
            output=trace_info.outputs,
            metadata=metadata,
            session_id=message_data.conversation_id,
            tags=["message", str(trace_info.conversation_mode)],
            version=None,
            release=None,
            public=None,
        )
        self.add_trace(langfuse_trace_data=trace_data)

        # add generation
        generation_usage = GenerationUsage(
            input=trace_info.message_tokens,
            output=trace_info.answer_tokens,
            total=trace_info.total_tokens,
            unit=UnitEnum.TOKENS,
            totalCost=message_data.total_price,
        )

        langfuse_generation_data = LangfuseGeneration(
            name="llm",
            trace_id=trace_id,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            model=message_data.model_id,
            input=trace_info.inputs,
            output=message_data.answer,
            metadata=metadata,
            level=(LevelEnum.DEFAULT if message_data.status != MessageStatus.ERROR else LevelEnum.ERROR),
            status_message=message_data.error or "",
            usage=generation_usage,
        )

        self.add_generation(langfuse_generation_data)

    def moderation_trace(self, trace_info: ModerationTraceInfo):
        if trace_info.message_data is None:
            return
        span_data = LangfuseSpan(
            name=TraceTaskName.MODERATION_TRACE,
            input=trace_info.inputs,
            output={
                "action": trace_info.action,
                "flagged": trace_info.flagged,
                "preset_response": trace_info.preset_response,
                "inputs": trace_info.inputs,
            },
            trace_id=trace_info.trace_id or trace_info.message_id,
            start_time=trace_info.start_time or trace_info.message_data.created_at,
            end_time=trace_info.end_time or trace_info.message_data.created_at,
            metadata=trace_info.metadata,
        )

        self.add_span(langfuse_span_data=span_data)

    def suggested_question_trace(self, trace_info: SuggestedQuestionTraceInfo):
        message_data = trace_info.message_data
        if message_data is None:
            return
        generation_usage = GenerationUsage(
            total=len(str(trace_info.suggested_question)),
            input=len(trace_info.inputs) if trace_info.inputs else 0,
            output=len(trace_info.suggested_question),
            unit=UnitEnum.CHARACTERS,
        )

        generation_data = LangfuseGeneration(
            name=TraceTaskName.SUGGESTED_QUESTION_TRACE,
            input=trace_info.inputs,
            output=str(trace_info.suggested_question),
            trace_id=trace_info.trace_id or trace_info.message_id,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            metadata=trace_info.metadata,
            level=(LevelEnum.DEFAULT if message_data.status != MessageStatus.ERROR else LevelEnum.ERROR),
            status_message=message_data.error or "",
            usage=generation_usage,
        )

        self.add_generation(langfuse_generation_data=generation_data)

    def dataset_retrieval_trace(self, trace_info: DatasetRetrievalTraceInfo):
        if trace_info.message_data is None:
            return
        dataset_retrieval_span_data = LangfuseSpan(
            name=TraceTaskName.DATASET_RETRIEVAL_TRACE,
            input=trace_info.inputs,
            output={"documents": trace_info.documents},
            trace_id=trace_info.trace_id or trace_info.message_id,
            start_time=trace_info.start_time or trace_info.message_data.created_at,
            end_time=trace_info.end_time or trace_info.message_data.updated_at,
            metadata=trace_info.metadata,
        )

        self.add_span(langfuse_span_data=dataset_retrieval_span_data)

    def tool_trace(self, trace_info: ToolTraceInfo):
        tool_span_data = LangfuseSpan(
            name=trace_info.tool_name,
            input=trace_info.tool_inputs,
            output=trace_info.tool_outputs,
            trace_id=trace_info.trace_id or trace_info.message_id,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            metadata=trace_info.metadata,
            level=(LevelEnum.DEFAULT if trace_info.error == "" or trace_info.error is None else LevelEnum.ERROR),
            status_message=trace_info.error,
        )

        self.add_span(langfuse_span_data=tool_span_data)

    def generate_name_trace(self, trace_info: GenerateNameTraceInfo):
        name_generation_trace_data = LangfuseTrace(
            name=TraceTaskName.GENERATE_NAME_TRACE,
            input=trace_info.inputs,
            output=trace_info.outputs,
            user_id=trace_info.tenant_id,
            metadata=trace_info.metadata,
            session_id=trace_info.conversation_id,
        )

        self.add_trace(langfuse_trace_data=name_generation_trace_data)

        name_generation_span_data = LangfuseSpan(
            name=TraceTaskName.GENERATE_NAME_TRACE,
            input=trace_info.inputs,
            output=trace_info.outputs,
            trace_id=trace_info.conversation_id,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            metadata=trace_info.metadata,
        )
        self.add_span(langfuse_span_data=name_generation_span_data)

    def _make_event_id(self) -> str:
        return str(uuid.uuid4())

    def _now_iso(self) -> str:
        return datetime.now(UTC).isoformat()

    def add_trace(self, langfuse_trace_data: LangfuseTrace | None = None):
        data = filter_none_values(langfuse_trace_data.model_dump()) if langfuse_trace_data else {}
        try:
            body = TraceBody(
                id=data.get("id"),
                name=data.get("name"),
                user_id=data.get("user_id"),
                input=data.get("input"),
                output=data.get("output"),
                metadata=data.get("metadata"),
                session_id=data.get("session_id"),
                version=data.get("version"),
                release=data.get("release"),
                tags=data.get("tags"),
                public=data.get("public"),
            )
            event = IngestionEvent_TraceCreate(
                body=body,
                id=self._make_event_id(),
                timestamp=self._now_iso(),
            )
            self.langfuse_client.api.ingestion.batch(batch=[event])
            logger.debug("LangFuse Trace created successfully")
        except Exception as e:
            raise ValueError(f"LangFuse Failed to create trace: {str(e)}")

    def add_span(self, langfuse_span_data: LangfuseSpan | None = None):
        data = filter_none_values(langfuse_span_data.model_dump()) if langfuse_span_data else {}
        try:
            body = CreateSpanBody(
                id=data.get("id"),
                trace_id=data.get("trace_id"),
                name=data.get("name"),
                start_time=data.get("start_time"),
                end_time=data.get("end_time"),
                input=data.get("input"),
                output=data.get("output"),
                metadata=data.get("metadata"),
                level=data.get("level"),
                status_message=data.get("status_message"),
                parent_observation_id=data.get("parent_observation_id"),
                version=data.get("version"),
            )
            event = IngestionEvent_SpanCreate(
                body=body,
                id=self._make_event_id(),
                timestamp=self._now_iso(),
            )
            self.langfuse_client.api.ingestion.batch(batch=[event])
            logger.debug("LangFuse Span created successfully")
        except Exception as e:
            raise ValueError(f"LangFuse Failed to create span: {str(e)}")

    def update_span(self, span, langfuse_span_data: LangfuseSpan | None = None):
        format_span_data = filter_none_values(langfuse_span_data.model_dump()) if langfuse_span_data else {}

        span.end(**format_span_data)

    def add_generation(self, langfuse_generation_data: LangfuseGeneration | None = None):
        data = filter_none_values(langfuse_generation_data.model_dump()) if langfuse_generation_data else {}
        try:
            usage_data = data.pop("usage", None)
            usage = None
            if usage_data:
                usage = Usage(
                    input=usage_data.get("input", 0) or 0,
                    output=usage_data.get("output", 0) or 0,
                    total=usage_data.get("total", 0) or 0,
                    unit=usage_data.get("unit"),
                    input_cost=usage_data.get("inputCost"),
                    output_cost=usage_data.get("outputCost"),
                    total_cost=usage_data.get("totalCost"),
                )

            body = CreateGenerationBody(
                id=data.get("id"),
                trace_id=data.get("trace_id"),
                name=data.get("name"),
                start_time=data.get("start_time"),
                end_time=data.get("end_time"),
                model=data.get("model"),
                model_parameters=data.get("model_parameters"),
                input=data.get("input"),
                output=data.get("output"),
                usage=usage,
                metadata=data.get("metadata"),
                level=data.get("level"),
                status_message=data.get("status_message"),
                parent_observation_id=data.get("parent_observation_id"),
                version=data.get("version"),
                completion_start_time=data.get("completion_start_time"),
            )
            event = IngestionEvent_GenerationCreate(
                body=body,
                id=self._make_event_id(),
                timestamp=self._now_iso(),
            )
            self.langfuse_client.api.ingestion.batch(batch=[event])
            logger.debug("LangFuse Generation created successfully")
        except Exception as e:
            raise ValueError(f"LangFuse Failed to create generation: {str(e)}")

    def update_generation(self, generation, langfuse_generation_data: LangfuseGeneration | None = None):
        format_generation_data = (
            filter_none_values(langfuse_generation_data.model_dump()) if langfuse_generation_data else {}
        )

        generation.end(**format_generation_data)

    def api_check(self):
        try:
            return self.langfuse_client.auth_check()
        except Exception as e:
            logger.debug("LangFuse API check failed: %s", str(e))
            raise ValueError(f"LangFuse API check failed: {str(e)}")

    def get_project_key(self):
        try:
            projects = self.langfuse_client.api.projects.get()
            return projects.data[0].id
        except Exception as e:
            logger.debug("LangFuse get project key failed: %s", str(e))
            raise ValueError(f"LangFuse get project key failed: {str(e)}")

```

### api/core/ops/langfuse_trace/entities/__init__.py
```py

```

### api/core/ops/langfuse_trace/entities/langfuse_trace_entity.py
```py
from collections.abc import Mapping
from datetime import datetime
from enum import StrEnum
from typing import Any, Union

from pydantic import BaseModel, ConfigDict, Field, field_validator
from pydantic_core.core_schema import ValidationInfo

from core.ops.utils import replace_text_with_content


def validate_input_output(v, field_name):
    """
    Validate input output
    :param v:
    :param field_name:
    :return:
    """
    if v == {} or v is None:
        return v
    if isinstance(v, str):
        return [
            {
                "role": "assistant" if field_name == "output" else "user",
                "content": v,
            }
        ]
    elif isinstance(v, list):
        if len(v) > 0 and isinstance(v[0], dict):
            v = replace_text_with_content(data=v)
            return v
        else:
            return [
                {
                    "role": "assistant" if field_name == "output" else "user",
                    "content": str(v),
                }
            ]

    return v


class LevelEnum(StrEnum):
    DEBUG = "DEBUG"
    WARNING = "WARNING"
    ERROR = "ERROR"
    DEFAULT = "DEFAULT"


class LangfuseTrace(BaseModel):
    """
    Langfuse trace model
    """

    id: str | None = Field(
        default=None,
        description="The id of the trace can be set, defaults to a random id. Used to link traces to external systems "
        "or when creating a distributed trace. Traces are upserted on id.",
    )
    name: str | None = Field(
        default=None,
        description="Identifier of the trace. Useful for sorting/filtering in the UI.",
    )
    input: Union[str, dict[str, Any], list, None] | None = Field(
        default=None, description="The input of the trace. Can be any JSON object."
    )
    output: Union[str, dict[str, Any], list, None] | None = Field(
        default=None, description="The output of the trace. Can be any JSON object."
    )
    metadata: dict[str, Any] | None = Field(
        default=None,
        description="Additional metadata of the trace. Can be any JSON object. Metadata is merged when being updated "
        "via the API.",
    )
    user_id: str | None = Field(
        default=None,
        description="The id of the user that triggered the execution. Used to provide user-level analytics.",
    )
    session_id: str | None = Field(
        default=None,
        description="Used to group multiple traces into a session in Langfuse. Use your own session/thread identifier.",
    )
    version: str | None = Field(
        default=None,
        description="The version of the trace type. Used to understand how changes to the trace type affect metrics. "
        "Useful in debugging.",
    )
    release: str | None = Field(
        default=None,
        description="The release identifier of the current deployment. Used to understand how changes of different "
        "deployments affect metrics. Useful in debugging.",
    )
    tags: list[str] | None = Field(
        default=None,
        description="Tags are used to categorize or label traces. Traces can be filtered by tags in the UI and GET "
        "API. Tags can also be changed in the UI. Tags are merged and never deleted via the API.",
    )
    public: bool | None = Field(
        default=None,
        description="You can make a trace public to share it via a public link. This allows others to view the trace "
        "without needing to log in or be members of your Langfuse project.",
    )

    @field_validator("input", "output")
    @classmethod
    def ensure_dict(cls, v, info: ValidationInfo):
        field_name = info.field_name
        return validate_input_output(v, field_name)


class LangfuseSpan(BaseModel):
    """
    Langfuse span model
    """

    id: str | None = Field(
        default=None,
        description="The id of the span can be set, otherwise a random id is generated. Spans are upserted on id.",
    )
    session_id: str | None = Field(
        default=None,
        description="Used to group multiple spans into a session in Langfuse. Use your own session/thread identifier.",
    )
    trace_id: str | None = Field(
        default=None,
        description="The id of the trace the span belongs to. Used to link spans to traces.",
    )
    user_id: str | None = Field(
        default=None,
        description="The id of the user that triggered the execution. Used to provide user-level analytics.",
    )
    start_time: datetime | None = Field(
        default_factory=datetime.now,
        description="The time at which the span started, defaults to the current time.",
    )
    end_time: datetime | None = Field(
        default=None,
        description="The time at which the span ended. Automatically set by span.end().",
    )
    name: str | None = Field(
        default=None,
        description="Identifier of the span. Useful for sorting/filtering in the UI.",
    )
    metadata: dict[str, Any] | None = Field(
        default=None,
        description="Additional metadata of the span. Can be any JSON object. Metadata is merged when being updated "
        "via the API.",
    )
    level: LevelEnum | None = Field(
        default=None,
        description="The level of the span. Can be DEBUG, DEFAULT, WARNING or ERROR. Used for sorting/filtering of "
        "traces with elevated error levels and for highlighting in the UI.",
    )
    status_message: str | None = Field(
        default=None,
        description="The status message of the span. Additional field for context of the event. E.g. the error "
        "message of an error event.",
    )
    input: Union[str, Mapping[str, Any], list, None] | None = Field(
        default=None, description="The input of the span. Can be any JSON object."
    )
    output: Union[str, Mapping[str, Any], list, None] | None = Field(
        default=None, description="The output of the span. Can be any JSON object."
    )
    version: str | None = Field(
        default=None,
        description="The version of the span type. Used to understand how changes to the span type affect metrics. "
        "Useful in debugging.",
    )
    parent_observation_id: str | None = Field(
        default=None,
        description="The id of the observation the span belongs to. Used to link spans to observations.",
    )

    @field_validator("input", "output")
    @classmethod
    def ensure_dict(cls, v, info: ValidationInfo):
        field_name = info.field_name
        return validate_input_output(v, field_name)


class UnitEnum(StrEnum):
    CHARACTERS = "CHARACTERS"
    TOKENS = "TOKENS"
    SECONDS = "SECONDS"
    MILLISECONDS = "MILLISECONDS"
    IMAGES = "IMAGES"


class GenerationUsage(BaseModel):
    promptTokens: int | None = None
    completionTokens: int | None = None
    total: int | None = None
    input: int | None = None
    output: int | None = None
    unit: UnitEnum | None = None
    inputCost: float | None = None
    outputCost: float | None = None
    totalCost: float | None = None

    @field_validator("input", "output")
    @classmethod
    def ensure_dict(cls, v, info: ValidationInfo):
        field_name = info.field_name
        return validate_input_output(v, field_name)


class LangfuseGeneration(BaseModel):
    id: str | None = Field(
        default=None,
        description="The id of the generation can be set, defaults to random id.",
    )
    trace_id: str | None = Field(
        default=None,
        description="The id of the trace the generation belongs to. Used to link generations to traces.",
    )
    parent_observation_id: str | None = Field(
        default=None,
        description="The id of the observation the generation belongs to. Used to link generations to observations.",
    )
    name: str | None = Field(
        default=None,
        description="Identifier of the generation. Useful for sorting/filtering in the UI.",
    )
    start_time: datetime | None = Field(
        default_factory=datetime.now,
        description="The time at which the generation started, defaults to the current time.",
    )
    completion_start_time: datetime | None = Field(
        default=None,
        description="The time at which the completion started (streaming). Set it to get latency analytics broken "
        "down into time until completion started and completion duration.",
    )
    end_time: datetime | None = Field(
        default=None,
        description="The time at which the generation ended. Automatically set by generation.end().",
    )
    model: str | None = Field(default=None, description="The name of the model used for the generation.")
    model_parameters: dict[str, Any] | None = Field(
        default=None,
        description="The parameters of the model used for the generation; can be any key-value pairs.",
    )
    input: Any | None = Field(
        default=None,
        description="The prompt used for the generation. Can be any string or JSON object.",
    )
    output: Any | None = Field(
        default=None,
        description="The completion generated by the model. Can be any string or JSON object.",
    )
    usage: GenerationUsage | None = Field(
        default=None,
        description="The usage object supports the OpenAi structure with tokens and a more generic version with "
        "detailed costs and units.",
    )
    metadata: dict[str, Any] | None = Field(
        default=None,
        description="Additional metadata of the generation. Can be any JSON object. Metadata is merged when being "
        "updated via the API.",
    )
    level: LevelEnum | None = Field(
        default=None,
        description="The level of the generation. Can be DEBUG, DEFAULT, WARNING or ERROR. Used for sorting/filtering "
        "of traces with elevated error levels and for highlighting in the UI.",
    )
    status_message: str | None = Field(
        default=None,
        description="The status message of the generation. Additional field for context of the event. E.g. the error "
        "message of an error event.",
    )
    version: str | None = Field(
        default=None,
        description="The version of the generation type. Used to understand how changes to the span type affect "
        "metrics. Useful in debugging.",
    )

    model_config = ConfigDict(protected_namespaces=())

    @field_validator("input", "output")
    @classmethod
    def ensure_dict(cls, v, info: ValidationInfo):
        field_name = info.field_name
        return validate_input_output(v, field_name)

```

### api/core/ops/langsmith_trace/__init__.py
```py

```

### api/core/ops/langsmith_trace/langsmith_trace.py
```py
import logging
import os
import uuid
from datetime import datetime, timedelta
from typing import cast

from graphon.enums import BuiltinNodeTypes, WorkflowNodeExecutionMetadataKey
from langsmith import Client
from langsmith.schemas import RunBase
from sqlalchemy.orm import sessionmaker

from core.ops.base_trace_instance import BaseTraceInstance
from core.ops.entities.config_entity import LangSmithConfig
from core.ops.entities.trace_entity import (
    BaseTraceInfo,
    DatasetRetrievalTraceInfo,
    GenerateNameTraceInfo,
    MessageTraceInfo,
    ModerationTraceInfo,
    SuggestedQuestionTraceInfo,
    ToolTraceInfo,
    TraceTaskName,
    WorkflowTraceInfo,
)
from core.ops.langsmith_trace.entities.langsmith_trace_entity import (
    LangSmithRunModel,
    LangSmithRunType,
    LangSmithRunUpdateModel,
)
from core.ops.utils import filter_none_values, generate_dotted_order
from core.repositories import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models import EndUser, MessageFile, WorkflowNodeExecutionTriggeredFrom

logger = logging.getLogger(__name__)


class LangSmithDataTrace(BaseTraceInstance):
    def __init__(
        self,
        langsmith_config: LangSmithConfig,
    ):
        super().__init__(langsmith_config)
        self.langsmith_key = langsmith_config.api_key
        self.project_name = langsmith_config.project
        self.project_id = None
        self.langsmith_client = Client(api_key=langsmith_config.api_key, api_url=langsmith_config.endpoint)
        self.file_base_url = os.getenv("FILES_URL", "http://127.0.0.1:5001")

    def trace(self, trace_info: BaseTraceInfo):
        if isinstance(trace_info, WorkflowTraceInfo):
            self.workflow_trace(trace_info)
        if isinstance(trace_info, MessageTraceInfo):
            self.message_trace(trace_info)
        if isinstance(trace_info, ModerationTraceInfo):
            self.moderation_trace(trace_info)
        if isinstance(trace_info, SuggestedQuestionTraceInfo):
            self.suggested_question_trace(trace_info)
        if isinstance(trace_info, DatasetRetrievalTraceInfo):
            self.dataset_retrieval_trace(trace_info)
        if isinstance(trace_info, ToolTraceInfo):
            self.tool_trace(trace_info)
        if isinstance(trace_info, GenerateNameTraceInfo):
            self.generate_name_trace(trace_info)

    def workflow_trace(self, trace_info: WorkflowTraceInfo):
        trace_id = trace_info.trace_id or trace_info.message_id or trace_info.workflow_run_id
        if trace_info.start_time is None:
            trace_info.start_time = datetime.now()
        message_dotted_order = (
            generate_dotted_order(trace_info.message_id, trace_info.start_time) if trace_info.message_id else None
        )
        workflow_dotted_order = generate_dotted_order(
            trace_info.workflow_run_id,
            trace_info.workflow_data.created_at,
            message_dotted_order,
        )
        metadata = trace_info.metadata
        metadata["workflow_app_log_id"] = trace_info.workflow_app_log_id

        if trace_info.message_id:
            message_run = LangSmithRunModel(
                id=trace_info.message_id,
                name=TraceTaskName.MESSAGE_TRACE,
                inputs=dict(trace_info.workflow_run_inputs),
                outputs=dict(trace_info.workflow_run_outputs),
                run_type=LangSmithRunType.chain,
                start_time=trace_info.start_time,
                end_time=trace_info.end_time,
                extra={
                    "metadata": metadata,
                },
                tags=["message", "workflow"],
                error=trace_info.error,
                trace_id=trace_id,
                dotted_order=message_dotted_order,
                file_list=[],
                serialized=None,
                parent_run_id=None,
                events=[],
                session_id=None,
                session_name=None,
                reference_example_id=None,
                input_attachments={},
                output_attachments={},
            )
            self.add_run(message_run)

        langsmith_run = LangSmithRunModel(
            file_list=trace_info.file_list,
            total_tokens=trace_info.total_tokens,
            id=trace_info.workflow_run_id,
            name=TraceTaskName.WORKFLOW_TRACE,
            inputs=dict(trace_info.workflow_run_inputs),
            run_type=LangSmithRunType.tool,
            start_time=trace_info.workflow_data.created_at,
            end_time=trace_info.workflow_data.finished_at,
            outputs=dict(trace_info.workflow_run_outputs),
            extra={
                "metadata": metadata,
            },
            error=trace_info.error,
            tags=["workflow"],
            parent_run_id=trace_info.message_id or None,
            trace_id=trace_id,
            dotted_order=workflow_dotted_order,
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
        )

        self.add_run(langsmith_run)

        # through workflow_run_id get all_nodes_execution using repository
        session_factory = sessionmaker(bind=db.engine)
        # Find the app's creator account
        app_id = trace_info.metadata.get("app_id")
        if not app_id:
            raise ValueError("No app_id found in trace_info metadata")

        service_account = self.get_service_account_with_tenant(app_id)

        workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
            session_factory=session_factory,
            user=service_account,
            app_id=app_id,
            triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
        )

        # Get all executions for this workflow run
        workflow_node_executions = workflow_node_execution_repository.get_by_workflow_execution(
            workflow_execution_id=trace_info.workflow_run_id
        )

        for node_execution in workflow_node_executions:
            node_execution_id = node_execution.id
            tenant_id = trace_info.tenant_id  # Use from trace_info instead
            app_id = trace_info.metadata.get("app_id")  # Use from trace_info instead
            node_name = node_execution.title
            node_type = node_execution.node_type
            status = node_execution.status
            if node_type == BuiltinNodeTypes.LLM:
                inputs = node_execution.process_data.get("prompts", {}) if node_execution.process_data else {}
            else:
                inputs = node_execution.inputs or {}
            outputs = node_execution.outputs or {}
            created_at = node_execution.created_at or datetime.now()
            elapsed_time = node_execution.elapsed_time
            finished_at = created_at + timedelta(seconds=elapsed_time)

            execution_metadata = node_execution.metadata or {}
            node_total_tokens = execution_metadata.get(WorkflowNodeExecutionMetadataKey.TOTAL_TOKENS) or 0
            metadata = {str(key): value for key, value in execution_metadata.items()}
            metadata.update(
                {
                    "workflow_run_id": trace_info.workflow_run_id,
                    "node_execution_id": node_execution_id,
                    "tenant_id": tenant_id,
                    "app_id": app_id,
                    "app_name": node_name,
                    "node_type": node_type,
                    "status": status,
                }
            )

            process_data = node_execution.process_data or {}

            if process_data and process_data.get("model_mode") == "chat":
                run_type = LangSmithRunType.llm
                metadata.update(
                    {
                        "ls_provider": process_data.get("model_provider", ""),
                        "ls_model_name": process_data.get("model_name", ""),
                    }
                )
            elif node_type == BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL:
                run_type = LangSmithRunType.retriever
            else:
                run_type = LangSmithRunType.tool

            prompt_tokens = 0
            completion_tokens = 0
            try:
                usage_data = process_data.get("usage", {}) if "usage" in process_data else outputs.get("usage", {})
                prompt_tokens = usage_data.get("prompt_tokens", 0)
                completion_tokens = usage_data.get("completion_tokens", 0)
            except Exception:
                logger.error("Failed to extract usage", exc_info=True)

            node_dotted_order = generate_dotted_order(node_execution_id, created_at, workflow_dotted_order)
            langsmith_run = LangSmithRunModel(
                total_tokens=node_total_tokens,
                input_tokens=prompt_tokens,
                output_tokens=completion_tokens,
                name=node_type,
                inputs=inputs,
                run_type=run_type,
                start_time=created_at,
                end_time=finished_at,
                outputs=outputs,
                file_list=trace_info.file_list,
                extra={
                    "metadata": metadata,
                },
                parent_run_id=trace_info.workflow_run_id,
                tags=["node_execution"],
                id=node_execution_id,
                trace_id=trace_id,
                dotted_order=node_dotted_order,
                error="",
                serialized=None,
                events=[],
                session_id=None,
                session_name=None,
                reference_example_id=None,
                input_attachments={},
                output_attachments={},
            )

            self.add_run(langsmith_run)

    def message_trace(self, trace_info: MessageTraceInfo):
        # get message file data
        file_list = cast(list[str], trace_info.file_list) or []
        message_file_data: MessageFile | None = trace_info.message_file_data
        file_url = f"{self.file_base_url}/{message_file_data.url}" if message_file_data else ""
        file_list.append(file_url)
        metadata = trace_info.metadata
        message_data = trace_info.message_data
        if message_data is None:
            return
        message_id = message_data.id

        user_id = message_data.from_account_id
        metadata["user_id"] = user_id

        if message_data.from_end_user_id:
            end_user_data: EndUser | None = db.session.get(EndUser, message_data.from_end_user_id)
            if end_user_data is not None:
                end_user_id = end_user_data.session_id
                metadata["end_user_id"] = end_user_id

        message_run = LangSmithRunModel(
            input_tokens=trace_info.message_tokens,
            output_tokens=trace_info.answer_tokens,
            total_tokens=trace_info.total_tokens,
            id=message_id,
            name=TraceTaskName.MESSAGE_TRACE,
            inputs=trace_info.inputs,
            run_type=LangSmithRunType.chain,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            outputs=message_data.answer,
            extra={"metadata": metadata},
            tags=["message", str(trace_info.conversation_mode)],
            error=trace_info.error,
            file_list=file_list,
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            parent_run_id=None,
        )
        self.add_run(message_run)

        # create llm run parented to message run
        llm_run = LangSmithRunModel(
            input_tokens=trace_info.message_tokens,
            output_tokens=trace_info.answer_tokens,
            total_tokens=trace_info.total_tokens,
            name="llm",
            inputs=trace_info.inputs,
            run_type=LangSmithRunType.llm,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            outputs=message_data.answer,
            extra={"metadata": metadata},
            parent_run_id=message_id,
            tags=["llm", str(trace_info.conversation_mode)],
            error=trace_info.error,
            file_list=file_list,
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            id=str(uuid.uuid4()),
        )
        self.add_run(llm_run)

    def moderation_trace(self, trace_info: ModerationTraceInfo):
        if trace_info.message_data is None:
            return
        langsmith_run = LangSmithRunModel(
            name=TraceTaskName.MODERATION_TRACE,
            inputs=trace_info.inputs,
            outputs={
                "action": trace_info.action,
                "flagged": trace_info.flagged,
                "preset_response": trace_info.preset_response,
                "inputs": trace_info.inputs,
            },
            run_type=LangSmithRunType.tool,
            extra={"metadata": trace_info.metadata},
            tags=["moderation"],
            parent_run_id=trace_info.message_id,
            start_time=trace_info.start_time or trace_info.message_data.created_at,
            end_time=trace_info.end_time or trace_info.message_data.updated_at,
            id=str(uuid.uuid4()),
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            error="",
            file_list=[],
        )

        self.add_run(langsmith_run)

    def suggested_question_trace(self, trace_info: SuggestedQuestionTraceInfo):
        message_data = trace_info.message_data
        if message_data is None:
            return
        suggested_question_run = LangSmithRunModel(
            name=TraceTaskName.SUGGESTED_QUESTION_TRACE,
            inputs=trace_info.inputs,
            outputs=trace_info.suggested_question,
            run_type=LangSmithRunType.tool,
            extra={"metadata": trace_info.metadata},
            tags=["suggested_question"],
            parent_run_id=trace_info.message_id,
            start_time=trace_info.start_time or message_data.created_at,
            end_time=trace_info.end_time or message_data.updated_at,
            id=str(uuid.uuid4()),
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            error="",
            file_list=[],
        )

        self.add_run(suggested_question_run)

    def dataset_retrieval_trace(self, trace_info: DatasetRetrievalTraceInfo):
        if trace_info.message_data is None:
            return
        dataset_retrieval_run = LangSmithRunModel(
            name=TraceTaskName.DATASET_RETRIEVAL_TRACE,
            inputs=trace_info.inputs,
            outputs={"documents": trace_info.documents},
            run_type=LangSmithRunType.retriever,
            extra={"metadata": trace_info.metadata},
            tags=["dataset_retrieval"],
            parent_run_id=trace_info.message_id,
            start_time=trace_info.start_time or trace_info.message_data.created_at,
            end_time=trace_info.end_time or trace_info.message_data.updated_at,
            id=str(uuid.uuid4()),
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            error="",
            file_list=[],
        )

        self.add_run(dataset_retrieval_run)

    def tool_trace(self, trace_info: ToolTraceInfo):
        tool_run = LangSmithRunModel(
            name=trace_info.tool_name,
            inputs=trace_info.tool_inputs,
            outputs=trace_info.tool_outputs,
            run_type=LangSmithRunType.tool,
            extra={
                "metadata": trace_info.metadata,
            },
            tags=["tool", trace_info.tool_name],
            parent_run_id=trace_info.message_id,
            start_time=trace_info.start_time,
            end_time=trace_info.end_time,
            file_list=[cast(str, trace_info.file_url)],
            id=str(uuid.uuid4()),
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            error=trace_info.error or "",
        )

        self.add_run(tool_run)

    def generate_name_trace(self, trace_info: GenerateNameTraceInfo):
        name_run = LangSmithRunModel(
            name=TraceTaskName.GENERATE_NAME_TRACE,
            inputs=trace_info.inputs,
            outputs=trace_info.outputs,
            run_type=LangSmithRunType.tool,
            extra={"metadata": trace_info.metadata},
            tags=["generate_name"],
            start_time=trace_info.start_time or datetime.now(),
            end_time=trace_info.end_time or datetime.now(),
            id=str(uuid.uuid4()),
            serialized=None,
            events=[],
            session_id=None,
            session_name=None,
            reference_example_id=None,
            input_attachments={},
            output_attachments={},
            trace_id=trace_info.trace_id,
            dotted_order=None,
            error="",
            file_list=[],
            parent_run_id=None,
        )

        self.add_run(name_run)

    def add_run(self, run_data: LangSmithRunModel):
        data = run_data.model_dump()
        if self.project_id:
            data["session_id"] = self.project_id
        elif self.project_name:
            data["session_name"] = self.project_name

        data = filter_none_values(data)
        try:
            self.langsmith_client.create_run(**data)
            logger.debug("LangSmith Run created successfully.")
        except Exception as e:
            raise ValueError(f"LangSmith Failed to create run: {str(e)}")

    def update_run(self, update_run_data: LangSmithRunUpdateModel):
        data = update_run_data.model_dump()
        data = filter_none_values(data)
        try:
            self.langsmith_client.update_run(**data)
            logger.debug("LangSmith Run updated successfully.")
        except Exception as e:
            raise ValueError(f"LangSmith Failed to update run: {str(e)}")

    def api_check(self):
        try:
            random_project_name = f"test_project_{datetime.now().strftime('%Y%m%d%H%M%S')}"
            self.langsmith_client.create_project(project_name=random_project_name)
            self.langsmith_client.delete_project(project_name=random_project_name)
            return True
        except Exception as e:
            logger.debug("LangSmith API check failed: %s", str(e))
            raise ValueError(f"LangSmith API check failed: {str(e)}")

    def get_project_url(self):
        try:
            run_data = RunBase(
                id=uuid.uuid4(),
                name="tool",
                inputs={"input": "test"},
                outputs={"output": "test"},
                run_type=LangSmithRunType.tool,
                start_time=datetime.now(),
            )

            project_url = self.langsmith_client.get_run_url(
                run=run_data, project_id=self.project_id, project_name=self.project_name
            )
            return project_url.split("/r/")[0]
        except Exception as e:
            logger.debug("LangSmith get run url failed: %s", str(e))
            raise ValueError(f"LangSmith get run url failed: {str(e)}")

```

### api/core/ops/langsmith_trace/entities/langsmith_trace_entity.py
```py
from collections.abc import Mapping
from datetime import datetime
from enum import StrEnum
from typing import Any, Union

from pydantic import BaseModel, Field, field_validator
from pydantic_core.core_schema import ValidationInfo

from core.ops.utils import replace_text_with_content


class LangSmithRunType(StrEnum):
    tool = "tool"
    chain = "chain"
    llm = "llm"
    retriever = "retriever"
    embedding = "embedding"
    prompt = "prompt"
    parser = "parser"


class LangSmithTokenUsage(BaseModel):
    input_tokens: int | None = None
    output_tokens: int | None = None
    total_tokens: int | None = None


class LangSmithMultiModel(BaseModel):
    file_list: list[str] | None = Field(None, description="List of files")


class LangSmithRunModel(LangSmithTokenUsage, LangSmithMultiModel):
    name: str | None = Field(..., description="Name of the run")
    inputs: Union[str, Mapping[str, Any], list, None] | None = Field(None, description="Inputs of the run")
    outputs: Union[str, Mapping[str, Any], list, None] | None = Field(None, description="Outputs of the run")
    run_type: LangSmithRunType = Field(..., description="Type of the run")
    start_time: datetime | str | None = Field(None, description="Start time of the run")
    end_time: datetime | str | None = Field(None, description="End time of the run")
    extra: dict[str, Any] | None = Field(None, description="Extra information of the run")
    error: str | None = Field(None, description="Error message of the run")
    serialized: dict[str, Any] | None = Field(None, description="Serialized data of the run")
    parent_run_id: str | None = Field(None, description="Parent run ID")
    events: list[dict[str, Any]] | None = Field(None, description="Events associated with the run")
    tags: list[str] | None = Field(None, description="Tags associated with the run")
    trace_id: str | None = Field(None, description="Trace ID associated with the run")
    dotted_order: str | None = Field(None, description="Dotted order of the run")
    id: str | None = Field(None, description="ID of the run")
    session_id: str | None = Field(None, description="Session ID associated with the run")
    session_name: str | None = Field(None, description="Session name associated with the run")
    reference_example_id: str | None = Field(None, description="Reference example ID associated with the run")
    input_attachments: dict[str, Any] | None = Field(None, description="Input attachments of the run")
    output_attachments: dict[str, Any] | None = Field(None, description="Output attachments of the run")

    @field_validator("inputs", "outputs")
    @classmethod
    def ensure_dict(cls, v, info: ValidationInfo):
        field_name = info.field_name
        values = info.data
        if v == {} or v is None:
            return v
        usage_metadata = {
            "input_tokens": values.get("input_tokens", 0),
            "output_tokens": values.get("output_tokens", 0),
            "total_tokens": values.get("total_tokens", 0),
        }
        file_list = values.get("file_list", [])
        if isinstance(v, str):
            if field_name == "inputs":
                return {
                    "messages": {
                        "role": "user",
                        "content": v,
                        "usage_metadata": usage_metadata,
                        "file_list": file_list,
                    },
                }
            elif field_name == "outputs":
                return {
                    "choices": {
                        "role": "ai",
                        "content": v,
                        "usage_metadata": usage_metadata,
                        "file_list": file_list,
                    },
                }
        elif isinstance(v, list):
            data = {}
            if len(v) > 0 and isinstance(v[0], dict):
                # rename text to content
                v = replace_text_with_content(data=v)
                if field_name == "inputs":
                    data = {
                        "messages": v,
                    }
                elif field_name == "outputs":
                    data = {
                        "choices": {
                            "role": "ai",
                            "content": v,
                            "usage_metadata": usage_metadata,
                            "file_list": file_list,
                        },
                    }
                return data
            else:
                return {
                    "choices": {
                        "role": "ai" if field_name == "outputs" else "user",
                        "content": str(v),
                        "usage_metadata": usage_metadata,
                        "file_list": file_list,
                    },
                }
        if isinstance(v, dict):
            v["usage_metadata"] = usage_metadata
            v["file_list"] = file_list
            return v
        return v

    @classmethod
    @field_validator("start_time", "end_time")
    def format_time(cls, v, info: ValidationInfo):
        if not isinstance(v, datetime):
            raise ValueError(f"{info.field_name} must be a datetime object")
        else:
            return v.strftime("%Y-%m-%dT%H:%M:%S.%fZ")


class LangSmithRunUpdateModel(BaseModel):
    run_id: str = Field(..., description="ID of the run")
    trace_id: str | None = Field(None, description="Trace ID associated with the run")
    dotted_order: str | None = Field(None, description="Dotted order of the run")
    parent_run_id: str | None = Field(None, description="Parent run ID")
    end_time: datetime | str | None = Field(None, description="End time of the run")
    error: str | None = Field(None, description="Error message of the run")
    inputs: dict[str, Any] | None = Field(None, description="Inputs of the run")
    outputs: dict[str, Any] | None = Field(None, description="Outputs of the run")
    events: list[dict[str, Any]] | None = Field(None, description="Events associated with the run")
    tags: list[str] | None = Field(None, description="Tags associated with the run")
    extra: dict[str, Any] | None = Field(None, description="Extra information of the run")
    input_attachments: dict[str, Any] | None = Field(None, description="Input attachments of the run")
    output_attachments: dict[str, Any] | None = Field(None, description="Output attachments of the run")

```

### api/core/ops/langsmith_trace/entities/__init__.py
```py

```

### api/core/ops/mlflow_trace/__init__.py
```py

```

### api/core/ops/mlflow_trace/mlflow_trace.py
```py
import logging
import os
from datetime import datetime, timedelta
from typing import Any, cast

import mlflow
from graphon.enums import BuiltinNodeTypes
from mlflow.entities import Document, Span, SpanEvent, SpanStatusCode, SpanType
from mlflow.tracing.constant import SpanAttributeKey, TokenUsageKey, TraceMetadataKey
from mlflow.tracing.fluent import start_span_no_context, update_current_trace
from mlflow.tracing.provider import detach_span_from_context, set_span_in_context
from sqlalchemy import select

from core.ops.base_trace_instance import BaseTraceInstance
from core.ops.entities.config_entity import DatabricksConfig, MLflowConfig
from core.ops.entities.trace_entity import (
    BaseTraceInfo,
    DatasetRetrievalTraceInfo,
    GenerateNameTraceInfo,
    MessageTraceInfo,
    ModerationTraceInfo,
    SuggestedQuestionTraceInfo,
    ToolTraceInfo,
    TraceTaskName,
    WorkflowTraceInfo,
)
from core.ops.utils import JSON_DICT_ADAPTER
from extensions.ext_database import db
from models import EndUser
from models.workflow import WorkflowNodeExecutionModel

logger = logging.getLogger(__name__)


def datetime_to_nanoseconds(dt: datetime | None) -> int | None:
    """Convert datetime to nanosecond timestamp for MLflow API"""
    if dt is None:
        return None
    return int(dt.timestamp() * 1_000_000_000)


class MLflowDataTrace(BaseTraceInstance):
    def __init__(self, config: MLflowConfig | DatabricksConfig):
        super().__init__(config)
        if isinstance(config, DatabricksConfig):
            self._setup_databricks(config)
        else:
            self._setup_mlflow(config)

        # Enable async logging to minimize performance overhead
        os.environ["MLFLOW_ENABLE_ASYNC_TRACE_LOGGING"] = "true"

    def _setup_databricks(self, config: DatabricksConfig):
        """Setup connection to Databricks-managed MLflow instances"""
        os.environ["DATABRICKS_HOST"] = config.host

        if config.client_id and config.client_secret:
            # OAuth: https://docs.databricks.com/aws/en/dev-tools/auth/oauth-m2m?language=Environment
            os.environ["DATABRICKS_CLIENT_ID"] = config.client_id
            os.environ["DATABRICKS_CLIENT_SECRET"] = config.client_secret
        elif config.personal_access_token:
            # PAT: https://docs.databricks.com/aws/en/dev-tools/auth/pat
            os.environ["DATABRICKS_TOKEN"] = config.personal_access_token
        else:
            raise ValueError(
                "Either Databricks token (PAT) or client id and secret (OAuth) must be provided"
                "See https://docs.databricks.com/aws/en/dev-tools/auth/#what-authorization-option-should-i-choose "
                "for more information about the authorization options."
            )
        mlflow.set_tracking_uri("databricks")
        mlflow.set_experiment(experiment_id=config.experiment_id)

        # Remove trailing slash from host
        config.host = config.host.rstrip("/")
        self._project_url = f"{config.host}/ml/experiments/{config.experiment_id}/traces"

    def _setup_mlflow(self, config: MLflowConfig):
        """Setup connection to MLflow instances"""
        mlflow.set_tracking_uri(config.tracking_uri)
        mlflow.set_experiment(experiment_id=config.experiment_id)

        # Simple auth if provided
        if config.username and config.password:
            os.environ["MLFLOW_TRACKING_USERNAME"] = config.username
            os.environ["MLFLOW_TRACKING_PASSWORD"] = config.password

        self._project_url = f"{config.tracking_uri}/#/experiments/{config.experiment_id}/traces"

    def trace(self, trace_info: BaseTraceInfo):
        """Simple dispatch to trace methods"""
        try:
            if isinstance(trace_info, WorkflowTraceInfo):
                self.workflow_trace(trace_info)
            elif isinstance(trace_info, MessageTraceInfo):
                self.message_trace(trace_info)
            elif isinstance(trace_info, ToolTraceInfo):
                self.tool_trace(trace_info)
            elif isinstance(trace_info, ModerationTraceInfo):
                self.moderation_trace(trace_info)
            elif isinstance(trace_info, DatasetRetrievalTraceInfo):
                self.dataset_retrieval_trace(trace_info)
            elif isinstance(trace_info, SuggestedQuestionTraceInfo):
                self.suggested_question_trace(trace_info)
            elif isinstance(trace_info, GenerateNameTraceInfo):
                self.generate_name_trace(trace_info)
        except Exception:
            logger.exception("[MLflow] Trace error")
            raise

    def workflow_trace(self, trace_info: WorkflowTraceInfo):
        """Create workflow span as root, with node spans as children"""
        # fields with sys.xyz is added by Dify, they are duplicate to trace_info.metadata
        raw_inputs = trace_info.workflow_run_inputs or {}
        workflow_inputs = {k: v for k, v in raw_inputs.items() if not k.startswith("sys.")}

        # Special inputs propagated by system
        if trace_info.query:
            workflow_inputs["query"] = trace_info.query

        workflow_span = start_span_no_context(
            name=TraceTaskName.WORKFLOW_TRACE.value,
            span_type=SpanType.CHAIN,
            inputs=workflow_inputs,
            attributes=trace_info.metadata,
            start_time_ns=datetime_to_nanoseconds(trace_info.start_time),
        )

        # Set reserved fields in trace-level metadata
        trace_metadata = {}
        if user_id := trace_info.metadata.get("user_id"):
            trace_metadata[TraceMetadataKey.TRACE_USER] = user_id
        if session_id := trace_info.conversation_id:
            trace_metadata[TraceMetadataKey.TRACE_SESSION] = session_id
        self._set_trace_metadata(workflow_span, trace_metadata)

        try:
            # Create child spans for workflow nodes
            for node in self._get_workflow_nodes(trace_info.workflow_run_id):
                inputs = None
                attributes = {
                    "node_id": node.id,
                    "node_type": node.node_type,
                    "status": node.status,
                    "tenant_id": node.tenant_id,
                    "app_id": node.app_id,
                    "app_name": node.title,
                }

                if node.node_type in (BuiltinNodeTypes.LLM, BuiltinNodeTypes.QUESTION_CLASSIFIER):
                    inputs, llm_attributes = self._parse_llm_inputs_and_attributes(node)
                    attributes.update(llm_attributes)
                elif node.node_type == BuiltinNodeTypes.HTTP_REQUEST:
                    inputs = node.process_data  # contains request URL

                if not inputs:
                    inputs = JSON_DICT_ADAPTER.validate_json(node.inputs) if node.inputs else {}

                node_span = start_span_no_context(
                    name=node.title,
                    span_type=self._get_node_span_type(node.node_type),
                    parent_span=workflow_span,
                    inputs=inputs,
                    attributes=attributes,
                    start_time_ns=datetime_to_nanoseconds(node.created_at),
                )

                # Handle node errors
                if node.status != "succeeded":
                    node_span.set_status(SpanStatusCode.ERROR)
                    node_span.add_event(
                        SpanEvent(  # type: ignore[abstract]
                            name="exception",
                            attributes={
                                "exception.message": f"Node failed with status: {node.status}",
                                "exception.type": "Error",
                                "exception.stacktrace": f"Node failed with status: {node.status}",
                            },
                        )
                    )

                # End node span
                finished_at = node.created_at + timedelta(seconds=node.elapsed_time)
                outputs = JSON_DICT_ADAPTER.validate_json(node.outputs) if node.outputs else {}
                if node.node_type == BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL:
                    outputs = self._parse_knowledge_retrieval_outputs(outputs)
                elif node.node_type == BuiltinNodeTypes.LLM:
                    outputs = outputs.get("text", outputs)
                node_span.end(
                    outputs=outputs,
                    end_time_ns=datetime_to_nanoseconds(finished_at),
                )

            # Handle workflow-level errors
            if trace_info.error:
                workflow_span.set_status(SpanStatusCode.ERROR)
                workflow_span.add_event(
                    SpanEvent(  # type: ignore[abstract]
                        name="exception",
                        attributes={
                            "exception.message": trace_info.error,
                            "exception.type": "Error",
                            "exception.stacktrace": trace_info.error,
                        },
                    )
                )

        finally:
            workflow_span.end(
                outputs=trace_info.workflow_run_outputs,
                end_time_ns=datetime_to_nanoseconds(trace_info.end_time),
            )

    def _parse_llm_inputs_and_attributes(self, node: WorkflowNodeExecutionModel) -> tuple[Any, dict]:
        """Parse LLM inputs and attributes from LLM workflow node"""
        if node.process_data is None:
            return {}, {}

        try:
            data = JSON_DICT_ADAPTER.validate_json(node.process_data)
        except (ValueError, TypeError):
            return {}, {}

        inputs = self._parse_prompts(data.get("prompts"))
        attributes = {
            "model_name": data.get("model_name"),
            "model_provider": data.get("model_provider"),
            "finish_reason": data.get("finish_reason"),
        }

        if hasattr(SpanAttributeKey, "MESSAGE_FORMAT"):
            attributes[SpanAttributeKey.MESSAGE_FORMAT] = "dify"

        if usage := data.get("usage"):
            # Set reserved token usage attributes
            attributes[SpanAttributeKey.CHAT_USAGE] = {
                TokenUsageKey.INPUT_TOKENS: usage.get("prompt_tokens", 0),
                TokenUsageKey.OUTPUT_TOKENS: usage.get("completion_tokens", 0),
                TokenUsageKey.TOTAL_TOKENS: usage.get("total_tokens", 0),
            }
            # Store raw usage data as well as it includes more data like price
            attributes["usage"] = usage

        return inputs, attributes

    def _parse_knowledge_retrieval_outputs(self, outputs: dict):
        """Parse KR outputs and attributes from KR workflow node"""
        retrieved = outputs.get("result", [])

        if not retrieved or not isinstance(retrieved, list):
            return outputs

        documents = []
        for item in retrieved:
            documents.append(Document(page_content=item.get("content", ""), metadata=item.get("metadata", {})))
        return documents

    def message_trace(self, trace_info: MessageTraceInfo):
        """Create span for CHATBOT message processing"""
        if not trace_info.message_data:
            return

        file_list = cast(list[str], trace_info.file_list) or []
        if message_file_data := trace_info.message_file_data:
            base_url = os.getenv("FILES_URL", "http://127.0.0.1:5001")
            file_list.append(f"{base_url}/{message_file_data.url}")

        span = start_span_no_context(
            name=TraceTaskName.MESSAGE_TRACE.value,
            span_type=SpanType.LLM,
            inputs=self._parse_prompts(trace_info.inputs),  # type: ignore[arg-type]
            attributes={
                "message_id": trace_info.message_id,  # type: ignore[dict-item]
                "model_provider": trace_info.message_data.model_provider,
                "model_id": trace_info.message_data.model_id,
                "conversation_mode": trace_info.conversation_mode,
                "file_list": file_list,  # type: ignore[dict-item]
                "total_price": trace_info.message_data.total_price,
                **trace_info.metadata,
            },
            start_time_ns=datetime_to_nanoseconds(trace_info.start_time),
        )

        if hasattr(SpanAttributeKey, "MESSAGE_FORMAT"):
            span.set_attribute(SpanAttributeKey.MESSAGE_FORMAT, "dify")

        # Set token usage
        span.set_attribute(
            SpanAttributeKey.CHAT_USAGE,
            {
                TokenUsageKey.INPUT_TOKENS: trace_info.message_tokens or 0,
                TokenUsageKey.OUTPUT_TOKENS: trace_info.answer_tokens or 0,
                TokenUsageKey.TOTAL_TOKENS: trace_info.total_tokens or 0,
            },
        )

        # Set reserved fields in trace-level metadata
        trace_metadata = {}
        if user_id := self._get_message_user_id(trace_info.metadata):
            trace_metadata[TraceMetadataKey.TRACE_USER] = user_id
        if session_id := trace_info.metadata.get("conversation_id"):
            trace_metadata[TraceMetadataKey.TRACE_SESSION] = session_id
        self._set_trace_metadata(span, trace_metadata)

        if trace_info.error:
            span.set_status(SpanStatusCode.ERROR)
            span.add_event(
                SpanEvent(  # type: ignore[abstract]
                    name="error",
                    attributes={
                        "exception.message": trace_info.error,
                        "exception.type": "Error",
                        "exception.stacktrace": trace_info.error,
                    },
                )
            )

        span.end(
            outputs=trace_info.message_data.answer,
            end_time_ns=datetime_to_nanoseconds(trace_info.end_time),
        )

    def _get_message_user_id(self, metadata: dict) -> str | None:
        if (end_user_id := metadata.get("from_end_user_id")) and (
            end_user_data := db.session.get(EndUser, end_user_id)
        ):
            return end_user_data.session_id

        return metadata.get("from_account_id")  # type: ignore[return-value]

    def tool_trace(self, trace_info: ToolTraceInfo):
        span = start_span_no_context(
            name=trace_info.tool_name,
            span_type=SpanType.TOOL,
            inputs=trace_info.tool_inputs,  # type: ignore[arg-type]
            attributes={
                "message_id": trace_info.message_id,  # type: ignore[dict-item]
                "metadata": trace_info.metadata,  # type: ignore[dict-item]
                "tool_config": trace_info.tool_config,  # type: ignore[dict-item]
                "tool_parameters": trace_info.tool_parameters,  # type: ignore[dict-item]
            },
            start_time_ns=datetime_to_nanoseconds(trace_info.start_time),
        )

        # Handle tool errors
        if trace_info.error:
            span.set_status(SpanStatusCode.ERROR)
            span.add_event(
                SpanEvent(  # type: ignore[abstract]
                    name="error",
                    attributes={
                        "exception.message": trace_info.error,
                        "exception.type": "Error",
                        "exception.stacktrace": trace_info.error,
                    },
                )
            )

        span.end(
            outputs=trace_info.tool_outputs,
            end_time_ns=datetime_to_nanoseconds(trace_info.end_time),
        )

    def moderation_trace(self, trace_info: ModerationTraceInfo):
        if trace_info.message_data is None:
            return

        start_time = trace_info.start_time or trace_info.message_data.created_at
        span = start_span_no_context(
            name=TraceTaskName.MODERATION_TRACE.value,
            span_type=SpanType.TOOL,
            inputs=trace_info.inputs or {},
            attributes={
                "message_id": trace_info.message_id,  # type: ignore[dict-item]
                "metadata": trace_info.metadata,  # type: ignore[dict-item]
            },
            start_time_ns=datetime_to_nanoseconds(start_time),
        )

        span.end(
            outputs={
                "action": trace_info.action,
                "flagged": trace_info.flagged,
                "preset_response": trace_info.preset_response,
            },
            end_time_ns=datetime_to_nanoseconds(trace_info.end_time),
        )

    def dataset_retrieval_trace(self, trace_info: DatasetRetrievalTraceInfo):
        if trace_info.message_data is None:
            return

        span = start_span_no_context(
            name=TraceTaskName.DATASET_RETRIEVAL_TRACE.value,
            span_type=SpanType.RETRIEVER,
            inputs=trace_info.inputs,
            attributes={
                "message_id": trace_info.message_id,  # type: ignore[dict-item]
                "metadata": trace_info.metadata,  # type: ignore[dict-item]
            },
            start_time_ns=datetime_to_nanoseconds(trace_info.start_time),
        )
        span.end(outputs={"documents": trace_info.documents}, end_time_ns=datetime_to_nanoseconds(trace_info.end_time))

    def suggested_question_trace(self, trace_info: SuggestedQuestionTraceInfo):
        if trace_info.message_data is None:
            return

        start_time = trace_info.start_time or trace_info.message_data.created_at
        end_time = trace_info.end_time or trace_info.message_data.updated_at

        span = start_span_no_context(
            name=TraceTaskName.SUGGESTED_QUESTION_TRACE.value,
            span_type=SpanType.TOOL,
            inputs=trace_info.inputs,
            attributes={
                "message_id": trace_info.message_id,  # type: ignore[dict-item]
                "model_provider": trace_info.model_provider,  # type: ignore[dict-item]
                "model_id": trace_info.model_id,  # type: ignore[dict-item]
                "total_tokens": trace_info.total_tokens or 0,  # type: ignore[dict-item]
            },
            start_time_ns=datetime_to_nanoseconds(start_time),
        )

        if trace_info.error:
            span.set_status(SpanStatusCode.ERROR)
            span.add_event(
                SpanEvent(  # type: ignore[abstract]
                    name="error",
                    attributes={
                        "exception.message": trace_info.error,
                        "exception.type": "Error",
                        "exception.stacktrace": trace_info.error,
                    },
                )
            )

        span.end(outputs=trace_info.suggested_question, end_time_ns=datetime_to_nanoseconds(end_time))

    def generate_name_trace(self, trace_info: GenerateNameTraceInfo):
        span = start_span_no_context(
            name=TraceTaskName.GENERATE_NAME_TRACE.value,
            span_type=SpanType.CHAIN,
            inputs=trace_info.inputs,
            attributes={"message_id": trace_info.message_id},  # type: ignore[dict-item]
            start_time_ns=datetime_to_nanoseconds(trace_info.start_time),
        )
        span.end(outputs=trace_info.outputs, end_time_ns=datetime_to_nanoseconds(trace_info.end_time))

    def _get_workflow_nodes(self, workflow_run_id: str):
        """Helper method to get workflow nodes"""
        workflow_nodes = db.session.scalars(
            select(WorkflowNodeExecutionModel)
            .where(WorkflowNodeExecutionModel.workflow_run_id == workflow_run_id)
            .order_by(WorkflowNodeExecutionModel.created_at)
        ).all()
        return workflow_nodes

    def _get_node_span_type(self, node_type: str) -> str:
        """Map Dify node types to MLflow span types"""
        node_type_mapping = {
            BuiltinNodeTypes.LLM: SpanType.LLM,
            BuiltinNodeTypes.QUESTION_CLASSIFIER: SpanType.LLM,
            BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL: SpanType.RETRIEVER,
            BuiltinNodeTypes.TOOL: SpanType.TOOL,
            BuiltinNodeTypes.CODE: SpanType.TOOL,
            BuiltinNodeTypes.HTTP_REQUEST: SpanType.TOOL,
            BuiltinNodeTypes.AGENT: SpanType.AGENT,
        }
        return node_type_mapping.get(node_type, "CHAIN")  # type: ignore[arg-type,call-overload]

    def _set_trace_metadata(self, span: Span, metadata: dict):
        token = None
        try:
            # NB: Set span in context such that we can use update_current_trace() API
            token = set_span_in_context(span)
            update_current_trace(metadata=metadata)
        finally:
            if token:
                detach_span_from_context(token)

    def _parse_prompts(self, prompts):
        """Postprocess prompts format to be standard chat messages"""
        if isinstance(prompts, str):
            return prompts
        elif isinstance(prompts, dict):
            return self._parse_single_message(prompts)
        elif isinstance(prompts, list):
            messages = [self._parse_single_message(item) for item in prompts]
            messages = self._resolve_tool_call_ids(messages)
            return messages
        return prompts  # Fallback to original format

    def _parse_single_message(self, item: dict):
        """Postprocess single message format to be standard chat message"""
        role = item.get("role", "user")
        msg = {"role": role, "content": item.get("text", "")}

        if (
            (tool_calls := item.get("tool_calls"))
            # Tool message does not contain tool calls normally
            and role != "tool"
        ):
            msg["tool_calls"] = tool_calls

        if files := item.get("files"):
            msg["files"] = files

        return msg

    def _resolve_tool_call_ids(self, messages: list[dict]):
        """
        The tool call message from Dify does not contain tool call ids, which is not
        ideal for debugging. This method resolves the tool call ids by matching the
        tool call name and parameters with the tool instruction messages.
        """
        tool_call_ids = []
        for msg in messages:
            if tool_calls := msg.get("tool_calls"):
                tool_call_ids = [t["id"] for t in tool_calls]
            if msg["role"] == "tool":
                # Get the tool call id in the order of the tool call messages
                # assuming Dify runs tools sequentially
                if tool_call_ids:
                    msg["tool_call_id"] = tool_call_ids.pop(0)
        return messages

    def api_check(self):
        """Simple connection test"""
        try:
            mlflow.search_experiments(max_results=1)
            return True
        except Exception as e:
            raise ValueError(f"MLflow connection failed: {str(e)}")

    def get_project_url(self):
        return self._project_url

```

### api/core/ops/opik_trace/__init__.py
```py

```

### api/core/ops/opik_trace/opik_trace.py
```py
import hashlib
import logging
import os
import uuid
from datetime import datetime, timedelta
from typing import cast

from graphon.enums import BuiltinNodeTypes, WorkflowNodeExecutionMetadataKey
from opik import Opik, Trace
from opik.id_helpers import uuid4_to_uuid7
from sqlalchemy.orm import sessionmaker

from core.ops.base_trace_instance import BaseTraceInstance
from core.ops.entities.config_entity import OpikConfig
from core.ops.entities.trace_entity import (
    BaseTraceInfo,
    DatasetRetrievalTraceInfo,
    GenerateNameTraceInfo,
    MessageTraceInfo,
    ModerationTraceInfo,
    SuggestedQuestionTraceInfo,
    ToolTraceInfo,
    TraceTaskName,
    WorkflowTraceInfo,
)
from core.repositories import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models import EndUser, MessageFile, WorkflowNodeExecutionTriggeredFrom

logger = logging.getLogger(__name__)


def wrap_dict(key_name, data):
    """Make sure that the input data is a dict"""
    if not isinstance(data, dict):
        return {key_name: data}

    return data


def wrap_metadata(metadata, **kwargs):
    """Add common metatada to all Traces and Spans"""
    metadata["created_from"] = "dify"

    metadata.update(kwargs)

    return metadata


def _seed_to_uuid4(seed: str) -> str:
    """Derive a deterministic UUID4-formatted string from an arbitrary seed.

    uuid4_to_uuid7 requires a valid UUID v4 string, but some Dify identifiers
    are not UUIDs (e.g. a workflow_run_id with a "-root" suffix appended to
    distinguish the root span from the trace).  This helper hashes the seed
    with MD5 and patches the version/variant bits so the result satisfies the
    UUID v4 contract.
    """
    raw = hashlib.md5(seed.encode()).digest()
    ba = bytearray(raw)
    ba[6] = (ba[6] & 0x0F) | 0x40  # version 4
    ba[8] = (ba[8] & 0x3F) | 0x80  # variant 1
    return str(uuid.UUID(bytes=bytes(ba)))


def prepare_opik_uuid(user_datetime: datetime | None, user_uuid: str | None):
    """Opik needs UUIDv7 while Dify uses UUIDv4 for identifier of most
    messages and objects. The type-hints of BaseTraceInfo indicates that
    objects start_time and message_id could be null which means we cannot map
    it to a UUIDv7. Given that we have no way to identify that object
    uniquely, generate a new random one UUIDv7 in that case.
    """

    if user_datetime is None:
        user_datetime = datetime.now()

    if user_uuid is None:
        user_uuid = str(uuid.uuid4())

    return uuid4_to_uuid7(user_datetime, user_uuid)


class OpikDataTrace(BaseTraceInstance):
    def __init__(
        self,
        opik_config: OpikConfig,
    ):
        super().__init__(opik_config)
        self.opik_client = Opik(
            project_name=opik_config.project,
            workspace=opik_config.workspace,
            host=opik_config.url,
            api_key=opik_config.api_key,
        )
        self.project = opik_config.project
        self.file_base_url = os.getenv("FILES_URL", "http://127.0.0.1:5001")

    def trace(self, trace_info: BaseTraceInfo):
        if isinstance(trace_info, WorkflowTraceInfo):
            self.workflow_trace(trace_info)
        if isinstance(trace_info, MessageTraceInfo):
            self.message_trace(trace_info)
        if isinstance(trace_info, ModerationTraceInfo):
            self.moderation_trace(trace_info)
        if isinstance(trace_info, SuggestedQuestionTraceInfo):
            self.suggested_question_trace(trace_info)
        if isinstance(trace_info, DatasetRetrievalTraceInfo):
            self.dataset_retrieval_trace(trace_info)
        if isinstance(trace_info, ToolTraceInfo):
            self.tool_trace(trace_info)
        if isinstance(trace_info, GenerateNameTraceInfo):
            self.generate_name_trace(trace_info)

    def workflow_trace(self, trace_info: WorkflowTraceInfo):
        workflow_metadata = wrap_metadata(
            trace_info.metadata, message_id=trace_info.message_id, workflow_app_log_id=trace_info.workflow_app_log_id
        )

        if trace_info.message_id:
            dify_trace_id = trace_info.trace_id or trace_info.message_id
            trace_name = TraceTaskName.MESSAGE_TRACE
            trace_tags = ["message", "workflow"]
            root_span_seed = trace_info.workflow_run_id
        else:
            dify_trace_id = trace_info.trace_id or trace_info.workflow_run_id
            trace_name = TraceTaskName.WORKFLOW_TRACE
            trace_tags = ["workflow"]
            root_span_seed = _seed_to_uuid4(trace_info.workflow_run_id + "-root")

        opik_trace_id = prepare_opik_uuid(trace_info.start_time, dify_trace_id)

        trace_data = {
            "id": opik_trace_id,
            "name": trace_name,
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": workflow_metadata,
            "input": wrap_dict("input", trace_info.workflow_run_inputs),
            "output": wrap_dict("output", trace_info.workflow_run_outputs),
            "thread_id": trace_info.conversation_id,
            "tags": trace_tags,
            "project_name": self.project,
        }
        self.add_trace(trace_data)

        root_span_id = prepare_opik_uuid(trace_info.start_time, root_span_seed)
        span_data = {
            "id": root_span_id,
            "parent_span_id": None,
            "trace_id": opik_trace_id,
            "name": TraceTaskName.WORKFLOW_TRACE,
            "input": wrap_dict("input", trace_info.workflow_run_inputs),
            "output": wrap_dict("output", trace_info.workflow_run_outputs),
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": workflow_metadata,
            "tags": ["workflow"],
            "project_name": self.project,
        }
        self.add_span(span_data)

        # through workflow_run_id get all_nodes_execution using repository
        session_factory = sessionmaker(bind=db.engine)
        # Find the app's creator account
        app_id = trace_info.metadata.get("app_id")
        if not app_id:
            raise ValueError("No app_id found in trace_info metadata")

        service_account = self.get_service_account_with_tenant(app_id)

        workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
            session_factory=session_factory,
            user=service_account,
            app_id=app_id,
            triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
        )

        # Get all executions for this workflow run
        workflow_node_executions = workflow_node_execution_repository.get_by_workflow_execution(
            workflow_execution_id=trace_info.workflow_run_id
        )

        for node_execution in workflow_node_executions:
            node_execution_id = node_execution.id
            tenant_id = trace_info.tenant_id  # Use from trace_info instead
            app_id = trace_info.metadata.get("app_id")  # Use from trace_info instead
            node_name = node_execution.title
            node_type = node_execution.node_type
            status = node_execution.status
            if node_type == BuiltinNodeTypes.LLM:
                inputs = node_execution.process_data.get("prompts", {}) if node_execution.process_data else {}
            else:
                inputs = node_execution.inputs or {}
            outputs = node_execution.outputs or {}
            created_at = node_execution.created_at or datetime.now()
            elapsed_time = node_execution.elapsed_time
            finished_at = created_at + timedelta(seconds=elapsed_time)

            execution_metadata = node_execution.metadata or {}
            metadata = {str(k): v for k, v in execution_metadata.items()}
            metadata.update(
                {
                    "workflow_run_id": trace_info.workflow_run_id,
                    "node_execution_id": node_execution_id,
                    "tenant_id": tenant_id,
                    "app_id": app_id,
                    "app_name": node_name,
                    "node_type": node_type,
                    "status": status,
                }
            )

            process_data = node_execution.process_data or {}

            provider = None
            model = None
            total_tokens = 0
            completion_tokens = 0
            prompt_tokens = 0

            if process_data and process_data.get("model_mode") == "chat":
                run_type = "llm"
                provider = process_data.get("model_provider", None)
                model = process_data.get("model_name", "")
                metadata.update(
                    {
                        "ls_provider": provider,
                        "ls_model_name": model,
                    }
                )

                try:
                    usage_data = process_data.get("usage", {}) if "usage" in process_data else outputs.get("usage", {})
                    total_tokens = usage_data.get("total_tokens", 0)
                    prompt_tokens = usage_data.get("prompt_tokens", 0)
                    completion_tokens = usage_data.get("completion_tokens", 0)
                except Exception:
                    logger.error("Failed to extract usage", exc_info=True)

            else:
                run_type = "tool"

            if not total_tokens:
                total_tokens = execution_metadata.get(WorkflowNodeExecutionMetadataKey.TOTAL_TOKENS) or 0

            span_data = {
                "trace_id": opik_trace_id,
                "id": prepare_opik_uuid(created_at, node_execution_id),
                "parent_span_id": root_span_id,
                "name": node_name,
                "type": run_type,
                "start_time": created_at,
                "end_time": finished_at,
                "metadata": wrap_metadata(metadata),
                "input": wrap_dict("input", inputs),
                "output": wrap_dict("output", outputs),
                "tags": ["node_execution"],
                "project_name": self.project,
                "usage": {
                    "total_tokens": total_tokens,
                    "completion_tokens": completion_tokens,
                    "prompt_tokens": prompt_tokens,
                },
                "model": model,
                "provider": provider,
            }

            self.add_span(span_data)

    def message_trace(self, trace_info: MessageTraceInfo):
        # get message file data
        file_list = cast(list[str], trace_info.file_list) or []
        message_file_data: MessageFile | None = trace_info.message_file_data

        if message_file_data is not None:
            file_url = f"{self.file_base_url}/{message_file_data.url}" if message_file_data else ""
            file_list.append(file_url)

        message_data = trace_info.message_data
        if message_data is None:
            return

        metadata = trace_info.metadata
        dify_trace_id = trace_info.trace_id or trace_info.message_id

        user_id = message_data.from_account_id
        metadata["user_id"] = user_id
        metadata["file_list"] = file_list

        if message_data.from_end_user_id:
            end_user_data: EndUser | None = db.session.get(EndUser, message_data.from_end_user_id)
            if end_user_data is not None:
                end_user_id = end_user_data.session_id
                metadata["end_user_id"] = end_user_id

        trace_data = {
            "id": prepare_opik_uuid(trace_info.start_time, dify_trace_id),
            "name": TraceTaskName.MESSAGE_TRACE,
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": wrap_metadata(metadata),
            "input": trace_info.inputs,
            "output": message_data.answer,
            "thread_id": message_data.conversation_id,
            "tags": ["message", str(trace_info.conversation_mode)],
            "project_name": self.project,
        }
        trace = self.add_trace(trace_data)

        span_data = {
            "trace_id": trace.id,
            "name": "llm",
            "type": "llm",
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": wrap_metadata(metadata),
            "input": {"input": trace_info.inputs},
            "output": {"output": message_data.answer},
            "tags": ["llm", str(trace_info.conversation_mode)],
            "usage": {
                "completion_tokens": trace_info.answer_tokens,
                "prompt_tokens": trace_info.message_tokens,
                "total_tokens": trace_info.total_tokens,
            },
            "project_name": self.project,
        }
        self.add_span(span_data)

    def moderation_trace(self, trace_info: ModerationTraceInfo):
        if trace_info.message_data is None:
            return

        start_time = trace_info.start_time or trace_info.message_data.created_at

        span_data = {
            "trace_id": prepare_opik_uuid(start_time, trace_info.trace_id or trace_info.message_id),
            "name": TraceTaskName.MODERATION_TRACE,
            "type": "tool",
            "start_time": start_time,
            "end_time": trace_info.end_time or trace_info.message_data.updated_at,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": wrap_dict("input", trace_info.inputs),
            "output": {
                "action": trace_info.action,
                "flagged": trace_info.flagged,
                "preset_response": trace_info.preset_response,
                "inputs": trace_info.inputs,
            },
            "tags": ["moderation"],
        }

        self.add_span(span_data)

    def suggested_question_trace(self, trace_info: SuggestedQuestionTraceInfo):
        message_data = trace_info.message_data
        if message_data is None:
            return

        start_time = trace_info.start_time or message_data.created_at

        span_data = {
            "trace_id": prepare_opik_uuid(start_time, trace_info.trace_id or trace_info.message_id),
            "name": TraceTaskName.SUGGESTED_QUESTION_TRACE,
            "type": "tool",
            "start_time": start_time,
            "end_time": trace_info.end_time or message_data.updated_at,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": wrap_dict("input", trace_info.inputs),
            "output": wrap_dict("output", trace_info.suggested_question),
            "tags": ["suggested_question"],
        }

        self.add_span(span_data)

    def dataset_retrieval_trace(self, trace_info: DatasetRetrievalTraceInfo):
        if trace_info.message_data is None:
            return

        start_time = trace_info.start_time or trace_info.message_data.created_at

        span_data = {
            "trace_id": prepare_opik_uuid(start_time, trace_info.trace_id or trace_info.message_id),
            "name": TraceTaskName.DATASET_RETRIEVAL_TRACE,
            "type": "tool",
            "start_time": start_time,
            "end_time": trace_info.end_time or trace_info.message_data.updated_at,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": wrap_dict("input", trace_info.inputs),
            "output": {"documents": trace_info.documents},
            "tags": ["dataset_retrieval"],
        }

        self.add_span(span_data)

    def tool_trace(self, trace_info: ToolTraceInfo):
        span_data = {
            "trace_id": prepare_opik_uuid(trace_info.start_time, trace_info.trace_id or trace_info.message_id),
            "name": trace_info.tool_name,
            "type": "tool",
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": wrap_dict("input", trace_info.tool_inputs),
            "output": wrap_dict("output", trace_info.tool_outputs),
            "tags": ["tool", trace_info.tool_name],
        }

        self.add_span(span_data)

    def generate_name_trace(self, trace_info: GenerateNameTraceInfo):
        trace_data = {
            "id": prepare_opik_uuid(trace_info.start_time, trace_info.trace_id or trace_info.message_id),
            "name": TraceTaskName.GENERATE_NAME_TRACE,
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": trace_info.inputs,
            "output": trace_info.outputs,
            "thread_id": trace_info.conversation_id,
            "tags": ["generate_name"],
            "project_name": self.project,
        }

        trace = self.add_trace(trace_data)

        span_data = {
            "trace_id": trace.id,
            "name": TraceTaskName.GENERATE_NAME_TRACE,
            "start_time": trace_info.start_time,
            "end_time": trace_info.end_time,
            "metadata": wrap_metadata(trace_info.metadata),
            "input": wrap_dict("input", trace_info.inputs),
            "output": wrap_dict("output", trace_info.outputs),
            "tags": ["generate_name"],
        }

        self.add_span(span_data)

    def add_trace(self, opik_trace_data: dict) -> Trace:
        try:
            trace = self.opik_client.trace(**opik_trace_data)
            logger.debug("Opik Trace created successfully")
            return trace
        except Exception as e:
            raise ValueError(f"Opik Failed to create trace: {str(e)}")

    def add_span(self, opik_span_data: dict):
        try:
            self.opik_client.span(**opik_span_data)
            logger.debug("Opik Span created successfully")
        except Exception as e:
            raise ValueError(f"Opik Failed to create span: {str(e)}")

    def api_check(self):
        try:
            self.opik_client.auth_check()
            return True
        except Exception as e:
            logger.info("Opik API check failed: %s", str(e), exc_info=True)
            raise ValueError(f"Opik API check failed: {str(e)}")

    def get_project_url(self):
        try:
            return self.opik_client.get_project_url(project_name=self.project)
        except Exception as e:
            logger.info("Opik get run url failed: %s", str(e), exc_info=True)
            raise ValueError(f"Opik get run url failed: {str(e)}")

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-023.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
