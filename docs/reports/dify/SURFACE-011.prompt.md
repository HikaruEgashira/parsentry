You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-011
- **Kind**: endpoint
- **Identifier**: Inner API Endpoints (/inner/api/)
- **Description**: Internal API endpoints for inter-service communication. If authentication is weaker than external APIs, risk of unauthorized access from compromised internal services or SSRF chains.
- **Locations**: api/controllers/inner_api/app/, api/controllers/inner_api/plugin/, api/controllers/inner_api/workspace/

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

### api/controllers/inner_api/app/dsl.py
```py
"""Inner API endpoints for app DSL import/export.

Called by the enterprise admin-api service. Import requires ``creator_email``
to attribute the created app; workspace/membership validation is done by the
Go admin-api caller.
"""

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from controllers.common.schema import register_schema_model
from controllers.console.wraps import setup_required
from controllers.inner_api import inner_api_ns
from controllers.inner_api.wraps import enterprise_inner_api_only
from extensions.ext_database import db
from models import Account, App
from models.account import AccountStatus
from services.app_dsl_service import AppDslService, ImportMode, ImportStatus


class InnerAppDSLImportPayload(BaseModel):
    yaml_content: str = Field(description="YAML DSL content")
    creator_email: str = Field(description="Email of the workspace member who will own the imported app")
    name: str | None = Field(default=None, description="Override app name from DSL")
    description: str | None = Field(default=None, description="Override app description from DSL")


register_schema_model(inner_api_ns, InnerAppDSLImportPayload)


@inner_api_ns.route("/enterprise/workspaces/<string:workspace_id>/dsl/import")
class EnterpriseAppDSLImport(Resource):
    @setup_required
    @enterprise_inner_api_only
    @inner_api_ns.doc("enterprise_app_dsl_import")
    @inner_api_ns.expect(inner_api_ns.models[InnerAppDSLImportPayload.__name__])
    @inner_api_ns.doc(
        responses={
            200: "Import completed",
            202: "Import pending (DSL version mismatch requires confirmation)",
            400: "Import failed (business error)",
            404: "Creator account not found or inactive",
        }
    )
    def post(self, workspace_id: str):
        """Import a DSL into a workspace on behalf of a specified creator."""
        args = InnerAppDSLImportPayload.model_validate(inner_api_ns.payload or {})

        account = _get_active_account(args.creator_email)
        if account is None:
            return {"message": f"account '{args.creator_email}' not found or inactive"}, 404

        account.set_tenant_id(workspace_id)

        with Session(db.engine) as session:
            dsl_service = AppDslService(session)
            result = dsl_service.import_app(
                account=account,
                import_mode=ImportMode.YAML_CONTENT,
                yaml_content=args.yaml_content,
                name=args.name,
                description=args.description,
            )
            session.commit()

        if result.status == ImportStatus.FAILED:
            return result.model_dump(mode="json"), 400
        if result.status == ImportStatus.PENDING:
            return result.model_dump(mode="json"), 202
        return result.model_dump(mode="json"), 200


@inner_api_ns.route("/enterprise/apps/<string:app_id>/dsl")
class EnterpriseAppDSLExport(Resource):
    @setup_required
    @enterprise_inner_api_only
    @inner_api_ns.doc(
        "enterprise_app_dsl_export",
        responses={
            200: "Export successful",
            404: "App not found",
        },
    )
    def get(self, app_id: str):
        """Export an app's DSL as YAML."""
        include_secret = request.args.get("include_secret", "false").lower() == "true"

        app_model = db.session.get(App, app_id)
        if not app_model:
            return {"message": "app not found"}, 404

        data = AppDslService.export_dsl(
            app_model=app_model,
            include_secret=include_secret,
        )

        return {"data": data}, 200


def _get_active_account(email: str) -> Account | None:
    """Look up an active account by email.

    Workspace membership is already validated by the Go admin-api caller.
    """
    account = db.session.scalar(select(Account).where(Account.email == email).limit(1))
    if account is None or account.status != AccountStatus.ACTIVE:
        return None
    return account

```

### api/controllers/inner_api/app/__init__.py
```py


```

### api/controllers/inner_api/plugin/__init__.py
```py

```

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

### api/controllers/inner_api/plugin/wraps.py
```py
from collections.abc import Callable
from functools import wraps

from flask import current_app, request
from flask_login import user_logged_in
from pydantic import BaseModel
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

from extensions.ext_database import db
from libs.login import current_user
from models.account import Tenant
from models.model import DefaultEndUserSessionID, EndUser


class TenantUserPayload(BaseModel):
    tenant_id: str
    user_id: str


def get_user(tenant_id: str, user_id: str | None) -> EndUser:
    """
    Get current user

    NOTE: user_id is not trusted, it could be maliciously set to any value.
    As a result, it could only be considered as an end user id.
    """
    if not user_id:
        user_id = DefaultEndUserSessionID.DEFAULT_SESSION_ID
    is_anonymous = user_id == DefaultEndUserSessionID.DEFAULT_SESSION_ID
    try:
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            user_model = None

            if is_anonymous:
                user_model = session.scalar(
                    select(EndUser)
                    .where(
                        EndUser.session_id == user_id,
                        EndUser.tenant_id == tenant_id,
                    )
                    .limit(1)
                )
            else:
                user_model = session.get(EndUser, user_id)

            if not user_model:
                user_model = EndUser(
                    tenant_id=tenant_id,
                    type="service_api",
                    is_anonymous=is_anonymous,
                    session_id=user_id,
                )
                session.add(user_model)
                session.flush()
                session.refresh(user_model)

    except Exception:
        raise ValueError("user not found")

    return user_model


def get_user_tenant[**P, R](view_func: Callable[P, R]) -> Callable[P, R]:
    @wraps(view_func)
    def decorated_view(*args: P.args, **kwargs: P.kwargs) -> R:
        payload = TenantUserPayload.model_validate(request.get_json(silent=True) or {})

        user_id = payload.user_id
        tenant_id = payload.tenant_id

        if not tenant_id:
            raise ValueError("tenant_id is required")

        if not user_id:
            user_id = DefaultEndUserSessionID.DEFAULT_SESSION_ID

        tenant_model = db.session.get(Tenant, tenant_id)

        if not tenant_model:
            raise ValueError("tenant not found")

        kwargs["tenant_model"] = tenant_model

        user = get_user(tenant_id, user_id)
        kwargs["user_model"] = user

        current_app.login_manager._update_request_context_with_user(user)  # type: ignore
        user_logged_in.send(current_app._get_current_object(), user=current_user)  # type: ignore

        return view_func(*args, **kwargs)

    return decorated_view


def plugin_data[**P, R](
    view: Callable[P, R] | None = None,
    *,
    payload_type: type[BaseModel],
) -> Callable[P, R] | Callable[[Callable[P, R]], Callable[P, R]]:
    def decorator(view_func: Callable[P, R]) -> Callable[P, R]:
        @wraps(view_func)
        def decorated_view(*args: P.args, **kwargs: P.kwargs) -> R:
            try:
                data = request.get_json()
            except Exception:
                raise ValueError("invalid json")

            try:
                payload = payload_type.model_validate(data)
            except Exception as e:
                raise ValueError(f"invalid payload: {str(e)}")

            kwargs["payload"] = payload
            return view_func(*args, **kwargs)

        return decorated_view

    if view is None:
        return decorator
    else:
        return decorator(view)

```

### api/controllers/inner_api/workspace/__init__.py
```py

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-011.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
