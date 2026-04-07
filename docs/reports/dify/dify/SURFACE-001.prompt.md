You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-001
- **Kind**: endpoint
- **Identifier**: POST /v1/workflows/run
- **Description**: API-key authenticated workflow execution endpoint that triggers the full workflow graph engine including code execution nodes, HTTP request nodes, and tool invocations
- **Locations**: api/controllers/service_api/app/workflow.py, api/core/app/apps/workflow/app_generator.py

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

### api/core/app/apps/workflow/app_generator.py
```py
from __future__ import annotations

import contextvars
import logging
import threading
import uuid
from collections.abc import Generator, Mapping, Sequence
from typing import TYPE_CHECKING, Any, Literal, overload

from flask import Flask, current_app
from graphon.graph_engine.layers import GraphEngineLayer
from graphon.model_runtime.errors.invoke import InvokeAuthorizationError
from graphon.runtime import GraphRuntimeState
from graphon.variable_loader import DUMMY_VARIABLE_LOADER, VariableLoader
from pydantic import ValidationError
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

import contexts
from configs import dify_config
from core.app.app_config.features.file_upload.manager import FileUploadConfigManager
from core.app.apps.base_app_generator import BaseAppGenerator
from core.app.apps.base_app_queue_manager import AppQueueManager, PublishFrom
from core.app.apps.draft_variable_saver import DraftVariableSaverFactory
from core.app.apps.exc import GenerateTaskStoppedError
from core.app.apps.workflow.app_config_manager import WorkflowAppConfigManager
from core.app.apps.workflow.app_queue_manager import WorkflowAppQueueManager
from core.app.apps.workflow.app_runner import WorkflowAppRunner
from core.app.apps.workflow.generate_response_converter import WorkflowAppGenerateResponseConverter
from core.app.apps.workflow.generate_task_pipeline import WorkflowAppGenerateTaskPipeline
from core.app.entities.app_invoke_entities import InvokeFrom, WorkflowAppGenerateEntity
from core.app.entities.task_entities import WorkflowAppBlockingResponse, WorkflowAppStreamResponse
from core.app.layers.pause_state_persist_layer import PauseStateLayerConfig, PauseStatePersistenceLayer
from core.db.session_factory import session_factory
from core.helper.trace_id_helper import extract_external_trace_id_from_args
from core.ops.ops_trace_manager import TraceQueueManager
from core.repositories import DifyCoreRepositoryFactory
from core.repositories.factory import WorkflowExecutionRepository, WorkflowNodeExecutionRepository
from extensions.ext_database import db
from factories import file_factory
from libs.flask_utils import preserve_flask_contexts
from models.account import Account
from models.enums import WorkflowRunTriggeredFrom
from models.model import App, EndUser
from models.workflow import Workflow, WorkflowNodeExecutionTriggeredFrom
from services.workflow_draft_variable_service import DraftVarLoader, WorkflowDraftVariableService

if TYPE_CHECKING:
    from controllers.console.app.workflow import LoopNodeRunPayload

SKIP_PREPARE_USER_INPUTS_KEY = "_skip_prepare_user_inputs"

logger = logging.getLogger(__name__)


class WorkflowAppGenerator(BaseAppGenerator):
    @staticmethod
    def _should_prepare_user_inputs(args: Mapping[str, Any]) -> bool:
        return not bool(args.get(SKIP_PREPARE_USER_INPUTS_KEY))

    @overload
    def generate(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: Literal[True],
        call_depth: int,
        workflow_run_id: str | uuid.UUID | None = None,
        triggered_from: WorkflowRunTriggeredFrom | None = None,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        pause_state_config: PauseStateLayerConfig | None = None,
    ) -> Generator[Mapping[str, Any] | str, None, None]: ...

    @overload
    def generate(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: Literal[False],
        call_depth: int,
        workflow_run_id: str | uuid.UUID | None = None,
        triggered_from: WorkflowRunTriggeredFrom | None = None,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        pause_state_config: PauseStateLayerConfig | None = None,
    ) -> Mapping[str, Any]: ...

    @overload
    def generate(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: bool,
        call_depth: int,
        workflow_run_id: str | uuid.UUID | None = None,
        triggered_from: WorkflowRunTriggeredFrom | None = None,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        pause_state_config: PauseStateLayerConfig | None = None,
    ) -> Mapping[str, Any] | Generator[Mapping[str, Any] | str, None, None]: ...

    def generate(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: bool = True,
        call_depth: int = 0,
        workflow_run_id: str | uuid.UUID | None = None,
        triggered_from: WorkflowRunTriggeredFrom | None = None,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        pause_state_config: PauseStateLayerConfig | None = None,
    ) -> Mapping[str, Any] | Generator[Mapping[str, Any] | str, None, None]:
        with self._bind_file_access_scope(tenant_id=app_model.tenant_id, user=user, invoke_from=invoke_from):
            files: Sequence[Mapping[str, Any]] = args.get("files") or []

            # parse files
            # TODO(QuantumGhost): Move file parsing logic to the API controller layer
            # for better separation of concerns.
            #
            # For implementation reference, see the `_parse_file` function and
            # `DraftWorkflowNodeRunApi` class which handle this properly.
            file_extra_config = FileUploadConfigManager.convert(workflow.features_dict, is_vision=False)
            system_files = file_factory.build_from_mappings(
                mappings=files,
                tenant_id=app_model.tenant_id,
                config=file_extra_config,
                strict_type_validation=True if invoke_from == InvokeFrom.SERVICE_API else False,
                access_controller=self._file_access_controller,
            )

            # convert to app config
            app_config = WorkflowAppConfigManager.get_app_config(
                app_model=app_model,
                workflow=workflow,
            )

            # get tracing instance
            trace_manager = TraceQueueManager(
                app_id=app_model.id,
                user_id=user.id if isinstance(user, Account) else user.session_id,
            )

            inputs: Mapping[str, Any] = args["inputs"]

            extras = {
                **extract_external_trace_id_from_args(args),
            }
            workflow_run_id = str(workflow_run_id or uuid.uuid4())
            # FIXME (Yeuoly): we need to remove the SKIP_PREPARE_USER_INPUTS_KEY from the args
            # trigger shouldn't prepare user inputs
            if self._should_prepare_user_inputs(args):
                inputs = self._prepare_user_inputs(
                    user_inputs=inputs,
                    variables=app_config.variables,
                    tenant_id=app_model.tenant_id,
                    strict_type_validation=True if invoke_from == InvokeFrom.SERVICE_API else False,
                )
            # init application generate entity
            application_generate_entity = WorkflowAppGenerateEntity(
                task_id=str(uuid.uuid4()),
                app_config=app_config,
                file_upload_config=file_extra_config,
                inputs=inputs,
                files=list(system_files),
                user_id=user.id,
                stream=streaming,
                invoke_from=invoke_from,
                call_depth=call_depth,
                trace_manager=trace_manager,
                workflow_execution_id=workflow_run_id,
                extras=extras,
            )

            contexts.plugin_tool_providers.set({})
            contexts.plugin_tool_providers_lock.set(threading.Lock())

            # Create repositories
            #
            # Create session factory
            session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
            # Create workflow execution(aka workflow run) repository
            if triggered_from is not None:
                # Use explicitly provided triggered_from (for async triggers)
                workflow_triggered_from = triggered_from
            elif invoke_from == InvokeFrom.DEBUGGER:
                workflow_triggered_from = WorkflowRunTriggeredFrom.DEBUGGING
            else:
                workflow_triggered_from = WorkflowRunTriggeredFrom.APP_RUN
            workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
                session_factory=session_factory,
                user=user,
                app_id=application_generate_entity.app_config.app_id,
                triggered_from=workflow_triggered_from,
            )
            # Create workflow node execution repository
            workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
                session_factory=session_factory,
                user=user,
                app_id=application_generate_entity.app_config.app_id,
                triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
            )

            return self._generate(
                app_model=app_model,
                workflow=workflow,
                user=user,
                application_generate_entity=application_generate_entity,
                invoke_from=invoke_from,
                workflow_execution_repository=workflow_execution_repository,
                workflow_node_execution_repository=workflow_node_execution_repository,
                streaming=streaming,
                root_node_id=root_node_id,
                graph_engine_layers=graph_engine_layers,
                pause_state_config=pause_state_config,
            )

    def resume(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        application_generate_entity: WorkflowAppGenerateEntity,
        graph_runtime_state: GraphRuntimeState,
        workflow_execution_repository: WorkflowExecutionRepository,
        workflow_node_execution_repository: WorkflowNodeExecutionRepository,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        pause_state_config: PauseStateLayerConfig | None = None,
        variable_loader: VariableLoader = DUMMY_VARIABLE_LOADER,
    ) -> Mapping[str, Any] | Generator[str | Mapping[str, Any], None, None]:
        """
        Resume a paused workflow execution using the persisted runtime state.
        """
        return self._generate(
            app_model=app_model,
            workflow=workflow,
            user=user,
            application_generate_entity=application_generate_entity,
            invoke_from=application_generate_entity.invoke_from,
            workflow_execution_repository=workflow_execution_repository,
            workflow_node_execution_repository=workflow_node_execution_repository,
            streaming=application_generate_entity.stream,
            variable_loader=variable_loader,
            graph_engine_layers=graph_engine_layers,
            graph_runtime_state=graph_runtime_state,
            pause_state_config=pause_state_config,
        )

    def _generate(
        self,
        *,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        application_generate_entity: WorkflowAppGenerateEntity,
        invoke_from: InvokeFrom,
        workflow_execution_repository: WorkflowExecutionRepository,
        workflow_node_execution_repository: WorkflowNodeExecutionRepository,
        streaming: bool = True,
        variable_loader: VariableLoader = DUMMY_VARIABLE_LOADER,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        graph_runtime_state: GraphRuntimeState | None = None,
        pause_state_config: PauseStateLayerConfig | None = None,
    ) -> Mapping[str, Any] | Generator[str | Mapping[str, Any], None, None]:
        """
        Generate App response.

        :param app_model: App
        :param workflow: Workflow
        :param user: account or end user
        :param application_generate_entity: application generate entity
        :param invoke_from: invoke from source
        :param workflow_execution_repository: repository for workflow execution
        :param workflow_node_execution_repository: repository for workflow node execution
        :param streaming: is stream
        """
        with self._bind_file_access_scope(
            tenant_id=application_generate_entity.app_config.tenant_id,
            user=user,
            invoke_from=invoke_from,
        ):
            graph_layers: list[GraphEngineLayer] = list(graph_engine_layers)

            # init queue manager
            queue_manager = WorkflowAppQueueManager(
                task_id=application_generate_entity.task_id,
                user_id=application_generate_entity.user_id,
                invoke_from=application_generate_entity.invoke_from,
                app_mode=app_model.mode,
            )

            if pause_state_config is not None:
                graph_layers.append(
                    PauseStatePersistenceLayer(
                        session_factory=pause_state_config.session_factory,
                        generate_entity=application_generate_entity,
                        state_owner_user_id=pause_state_config.state_owner_user_id,
                    )
                )

            # new thread with request context and contextvars
            context = contextvars.copy_context()

            # release database connection, because the following new thread operations may take a long time
            db.session.close()

            worker_thread = threading.Thread(
                target=self._generate_worker,
                kwargs={
                    "flask_app": current_app._get_current_object(),  # type: ignore
                    "application_generate_entity": application_generate_entity,
                    "queue_manager": queue_manager,
                    "context": context,
                    "variable_loader": variable_loader,
                    "root_node_id": root_node_id,
                    "workflow_execution_repository": workflow_execution_repository,
                    "workflow_node_execution_repository": workflow_node_execution_repository,
                    "graph_engine_layers": tuple(graph_layers),
                    "graph_runtime_state": graph_runtime_state,
                },
            )

            worker_thread.start()

            draft_var_saver_factory = self._get_draft_var_saver_factory(invoke_from, user)

            # return response or stream generator
            response = self._handle_response(
                application_generate_entity=application_generate_entity,
                workflow=workflow,
                queue_manager=queue_manager,
                user=user,
                draft_var_saver_factory=draft_var_saver_factory,
                stream=streaming,
            )

            return WorkflowAppGenerateResponseConverter.convert(response=response, invoke_from=invoke_from)

    def single_iteration_generate(
        self,
        app_model: App,
        workflow: Workflow,
        node_id: str,
        user: Account | EndUser,
        args: Mapping[str, Any],
        streaming: bool = True,
    ) -> Mapping[str, Any] | Generator[str | Mapping[str, Any], None, None]:
        """
        Generate App response.

        :param app_model: App
        :param workflow: Workflow
        :param node_id: the node id
        :param user: account or end user
        :param args: request args
        :param streaming: is streamed
        """
        if not node_id:
            raise ValueError("node_id is required")

        if args.get("inputs") is None:
            raise ValueError("inputs is required")

        # convert to app config
        app_config = WorkflowAppConfigManager.get_app_config(app_model=app_model, workflow=workflow)

        # init application generate entity
        application_generate_entity = WorkflowAppGenerateEntity(
            task_id=str(uuid.uuid4()),
            app_config=app_config,
            inputs={},
            files=[],
            user_id=user.id,
            stream=streaming,
            invoke_from=InvokeFrom.DEBUGGER,
            extras={"auto_generate_conversation_name": False},
            single_iteration_run=WorkflowAppGenerateEntity.SingleIterationRunEntity(
                node_id=node_id, inputs=args["inputs"]
            ),
            workflow_execution_id=str(uuid.uuid4()),
        )
        contexts.plugin_tool_providers.set({})
        contexts.plugin_tool_providers_lock.set(threading.Lock())

        # Create repositories
        #
        # Create session factory
        session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
        # Create workflow execution(aka workflow run) repository
        workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
            session_factory=session_factory,
            user=user,
            app_id=application_generate_entity.app_config.app_id,
            triggered_from=WorkflowRunTriggeredFrom.DEBUGGING,
        )
        # Create workflow node execution repository
        workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
            session_factory=session_factory,
            user=user,
            app_id=application_generate_entity.app_config.app_id,
            triggered_from=WorkflowNodeExecutionTriggeredFrom.SINGLE_STEP,
        )
        draft_var_srv = WorkflowDraftVariableService(db.session())
        draft_var_srv.prefill_conversation_variable_default_values(workflow, user_id=user.id)
        var_loader = DraftVarLoader(
            engine=db.engine,
            app_id=application_generate_entity.app_config.app_id,
            tenant_id=application_generate_entity.app_config.tenant_id,
            user_id=user.id,
        )

        return self._generate(
            app_model=app_model,
            workflow=workflow,
            user=user,
            invoke_from=InvokeFrom.DEBUGGER,
            application_generate_entity=application_generate_entity,
            workflow_execution_repository=workflow_execution_repository,
            workflow_node_execution_repository=workflow_node_execution_repository,
            streaming=streaming,
            variable_loader=var_loader,
            pause_state_config=None,
        )

    def single_loop_generate(
        self,
        app_model: App,
        workflow: Workflow,
        node_id: str,
        user: Account | EndUser,
        args: LoopNodeRunPayload,
        streaming: bool = True,
    ) -> Mapping[str, Any] | Generator[str | Mapping[str, Any], None, None]:
        """
        Generate App response.

        :param app_model: App
        :param workflow: Workflow
        :param node_id: the node id
        :param user: account or end user
        :param args: request args
        :param streaming: is streamed
        """
        if not node_id:
            raise ValueError("node_id is required")

        if args.inputs is None:
            raise ValueError("inputs is required")

        # convert to app config
        app_config = WorkflowAppConfigManager.get_app_config(app_model=app_model, workflow=workflow)

        # init application generate entity
        application_generate_entity = WorkflowAppGenerateEntity(
            task_id=str(uuid.uuid4()),
            app_config=app_config,
            inputs={},
            files=[],
            user_id=user.id,
            stream=streaming,
            invoke_from=InvokeFrom.DEBUGGER,
            extras={"auto_generate_conversation_name": False},
            single_loop_run=WorkflowAppGenerateEntity.SingleLoopRunEntity(node_id=node_id, inputs=args.inputs or {}),
            workflow_execution_id=str(uuid.uuid4()),
        )
        contexts.plugin_tool_providers.set({})
        contexts.plugin_tool_providers_lock.set(threading.Lock())

        # Create repositories
        #
        # Create session factory
        session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
        # Create workflow execution(aka workflow run) repository
        workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
            session_factory=session_factory,
            user=user,
            app_id=application_generate_entity.app_config.app_id,
            triggered_from=WorkflowRunTriggeredFrom.DEBUGGING,
        )
        # Create workflow node execution repository
        workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
            session_factory=session_factory,
            user=user,
            app_id=application_generate_entity.app_config.app_id,
            triggered_from=WorkflowNodeExecutionTriggeredFrom.SINGLE_STEP,
        )
        draft_var_srv = WorkflowDraftVariableService(db.session())
        draft_var_srv.prefill_conversation_variable_default_values(workflow, user_id=user.id)
        var_loader = DraftVarLoader(
            engine=db.engine,
            app_id=application_generate_entity.app_config.app_id,
            tenant_id=application_generate_entity.app_config.tenant_id,
            user_id=user.id,
        )
        return self._generate(
            app_model=app_model,
            workflow=workflow,
            user=user,
            invoke_from=InvokeFrom.DEBUGGER,
            application_generate_entity=application_generate_entity,
            workflow_execution_repository=workflow_execution_repository,
            workflow_node_execution_repository=workflow_node_execution_repository,
            streaming=streaming,
            variable_loader=var_loader,
            pause_state_config=None,
        )

    def _generate_worker(
        self,
        flask_app: Flask,
        application_generate_entity: WorkflowAppGenerateEntity,
        queue_manager: AppQueueManager,
        context: contextvars.Context,
        variable_loader: VariableLoader,
        workflow_execution_repository: WorkflowExecutionRepository,
        workflow_node_execution_repository: WorkflowNodeExecutionRepository,
        root_node_id: str | None = None,
        graph_engine_layers: Sequence[GraphEngineLayer] = (),
        graph_runtime_state: GraphRuntimeState | None = None,
    ) -> None:
        """
        Generate worker in a new thread.
        :param flask_app: Flask app
        :param application_generate_entity: application generate entity
        :param queue_manager: queue manager
        :param workflow_thread_pool_id: workflow thread pool id
        :return:
        """
        with preserve_flask_contexts(flask_app, context_vars=context):
            with session_factory.create_session() as session:
                workflow = session.scalar(
                    select(Workflow).where(
                        Workflow.tenant_id == application_generate_entity.app_config.tenant_id,
                        Workflow.app_id == application_generate_entity.app_config.app_id,
                        Workflow.id == application_generate_entity.app_config.workflow_id,
                    )
                )
                if workflow is None:
                    raise ValueError("Workflow not found")

                # Determine system_user_id based on invocation source
                is_external_api_call = application_generate_entity.invoke_from in {
                    InvokeFrom.WEB_APP,
                    InvokeFrom.SERVICE_API,
                }

                if is_external_api_call:
                    # For external API calls, use end user's session ID
                    end_user = session.scalar(select(EndUser).where(EndUser.id == application_generate_entity.user_id))
                    system_user_id = end_user.session_id if end_user else ""
                else:
                    # For internal calls, use the original user ID
                    system_user_id = application_generate_entity.user_id

            runner = WorkflowAppRunner(
                application_generate_entity=application_generate_entity,
                queue_manager=queue_manager,
                variable_loader=variable_loader,
                workflow=workflow,
                system_user_id=system_user_id,
                workflow_execution_repository=workflow_execution_repository,
                workflow_node_execution_repository=workflow_node_execution_repository,
                root_node_id=root_node_id,
                graph_engine_layers=graph_engine_layers,
                graph_runtime_state=graph_runtime_state,
            )

            try:
                runner.run()
            except GenerateTaskStoppedError as e:
                logger.warning("Task stopped: %s", str(e))
                pass
            except InvokeAuthorizationError:
                queue_manager.publish_error(
                    InvokeAuthorizationError("Incorrect API key provided"), PublishFrom.APPLICATION_MANAGER
                )
            except ValidationError as e:
                logger.exception("Validation Error when generating")
                queue_manager.publish_error(e, PublishFrom.APPLICATION_MANAGER)
            except ValueError as e:
                if dify_config.DEBUG:
                    logger.exception("Error when generating")
                queue_manager.publish_error(e, PublishFrom.APPLICATION_MANAGER)
            except Exception as e:
                logger.exception("Unknown Error when generating")
                queue_manager.publish_error(e, PublishFrom.APPLICATION_MANAGER)

    def _handle_response(
        self,
        application_generate_entity: WorkflowAppGenerateEntity,
        workflow: Workflow,
        queue_manager: AppQueueManager,
        user: Account | EndUser,
        draft_var_saver_factory: DraftVariableSaverFactory,
        stream: bool = False,
    ) -> WorkflowAppBlockingResponse | Generator[WorkflowAppStreamResponse, None, None]:
        """
        Handle response.
        :param application_generate_entity: application generate entity
        :param workflow: workflow
        :param queue_manager: queue manager
        :param user: account or end user
        :param stream: is stream
        :param workflow_node_execution_repository: optional repository for workflow node execution
        :return:
        """
        # init generate task pipeline
        generate_task_pipeline = WorkflowAppGenerateTaskPipeline(
            application_generate_entity=application_generate_entity,
            workflow=workflow,
            queue_manager=queue_manager,
            user=user,
            draft_var_saver_factory=draft_var_saver_factory,
            stream=stream,
        )

        try:
            return generate_task_pipeline.process()
        except ValueError as e:
            if len(e.args) > 0 and e.args[0] == "I/O operation on closed file.":  # ignore this error
                raise GenerateTaskStoppedError()
            else:
                logger.exception(
                    "Fails to process generate task pipeline, task_id: %s", application_generate_entity.task_id
                )
                raise e

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
