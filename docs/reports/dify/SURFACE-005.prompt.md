You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-005
- **Kind**: endpoint
- **Identifier**: Console Dataset/Document Management (POST /console/api/datasets, /console/api/datasets/{id}/documents)
- **Description**: Dataset and document management endpoints handling document ingestion from multiple sources (file, URL, online drives). Risk of SSRF via URL-based document import and path traversal in document processing.
- **Locations**: api/controllers/console/datasets/, api/services/rag_pipeline/

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

### api/controllers/console/datasets/rag_pipeline/datasource_auth.py
```py
from typing import Any

from flask import make_response, redirect, request
from flask_restx import Resource
from graphon.model_runtime.errors.validate import CredentialsValidateFailedError
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field
from werkzeug.exceptions import Forbidden, NotFound

from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from core.plugin.impl.oauth import OAuthHandler
from libs.login import current_account_with_tenant, login_required
from models.provider_ids import DatasourceProviderID
from services.datasource_provider_service import DatasourceProviderService
from services.plugin.oauth_service import OAuthProxyService


class DatasourceCredentialPayload(BaseModel):
    name: str | None = Field(default=None, max_length=100)
    credentials: dict[str, Any]


class DatasourceCredentialDeletePayload(BaseModel):
    credential_id: str


class DatasourceCredentialUpdatePayload(BaseModel):
    credential_id: str
    name: str | None = Field(default=None, max_length=100)
    credentials: dict[str, Any] | None = None


class DatasourceCustomClientPayload(BaseModel):
    client_params: dict[str, Any] | None = None
    enable_oauth_custom_client: bool | None = None


class DatasourceDefaultPayload(BaseModel):
    id: str


class DatasourceUpdateNamePayload(BaseModel):
    credential_id: str
    name: str = Field(max_length=100)


register_schema_models(
    console_ns,
    DatasourceCredentialPayload,
    DatasourceCredentialDeletePayload,
    DatasourceCredentialUpdatePayload,
    DatasourceCustomClientPayload,
    DatasourceDefaultPayload,
    DatasourceUpdateNamePayload,
)


@console_ns.route("/oauth/plugin/<path:provider_id>/datasource/get-authorization-url")
class DatasourcePluginOAuthAuthorizationUrl(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def get(self, provider_id: str):
        current_user, current_tenant_id = current_account_with_tenant()

        tenant_id = current_tenant_id

        credential_id = request.args.get("credential_id")
        datasource_provider_id = DatasourceProviderID(provider_id)
        provider_name = datasource_provider_id.provider_name
        plugin_id = datasource_provider_id.plugin_id
        oauth_config = DatasourceProviderService().get_oauth_client(
            tenant_id=tenant_id,
            datasource_provider_id=datasource_provider_id,
        )
        if not oauth_config:
            raise ValueError(f"No OAuth Client Config for {provider_id}")

        context_id = OAuthProxyService.create_proxy_context(
            user_id=current_user.id,
            tenant_id=tenant_id,
            plugin_id=plugin_id,
            provider=provider_name,
            credential_id=credential_id,
        )
        oauth_handler = OAuthHandler()
        redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider_id}/datasource/callback"
        authorization_url_response = oauth_handler.get_authorization_url(
            tenant_id=tenant_id,
            user_id=current_user.id,
            plugin_id=plugin_id,
            provider=provider_name,
            redirect_uri=redirect_uri,
            system_credentials=oauth_config,
        )
        response = make_response(jsonable_encoder(authorization_url_response))
        response.set_cookie(
            "context_id",
            context_id,
            httponly=True,
            samesite="Lax",
            max_age=OAuthProxyService.__MAX_AGE__,
        )
        return response


@console_ns.route("/oauth/plugin/<path:provider_id>/datasource/callback")
class DatasourceOAuthCallback(Resource):
    @setup_required
    def get(self, provider_id: str):
        context_id = request.cookies.get("context_id") or request.args.get("context_id")
        if not context_id:
            raise Forbidden("context_id not found")

        context = OAuthProxyService.use_proxy_context(context_id)
        if context is None:
            raise Forbidden("Invalid context_id")

        user_id: str = context["user_id"]
        tenant_id: str = context["tenant_id"]
        datasource_provider_id = DatasourceProviderID(provider_id)
        plugin_id = datasource_provider_id.plugin_id
        datasource_provider_service = DatasourceProviderService()
        oauth_client_params = datasource_provider_service.get_oauth_client(
            tenant_id=tenant_id,
            datasource_provider_id=datasource_provider_id,
        )
        if not oauth_client_params:
            raise NotFound()
        redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider_id}/datasource/callback"
        oauth_handler = OAuthHandler()
        oauth_response = oauth_handler.get_credentials(
            tenant_id=tenant_id,
            user_id=user_id,
            plugin_id=plugin_id,
            provider=datasource_provider_id.provider_name,
            redirect_uri=redirect_uri,
            system_credentials=oauth_client_params,
            request=request,
        )
        credential_id: str | None = context.get("credential_id")
        if credential_id:
            datasource_provider_service.reauthorize_datasource_oauth_provider(
                tenant_id=tenant_id,
                provider_id=datasource_provider_id,
                avatar_url=oauth_response.metadata.get("avatar_url") or None,
                name=oauth_response.metadata.get("name") or None,
                expire_at=oauth_response.expires_at,
                credentials=dict(oauth_response.credentials),
                credential_id=credential_id,
            )
        else:
            datasource_provider_service.add_datasource_oauth_provider(
                tenant_id=tenant_id,
                provider_id=datasource_provider_id,
                avatar_url=oauth_response.metadata.get("avatar_url") or None,
                name=oauth_response.metadata.get("name") or None,
                expire_at=oauth_response.expires_at,
                credentials=dict(oauth_response.credentials),
            )
        return redirect(f"{dify_config.CONSOLE_WEB_URL}/oauth-callback")


@console_ns.route("/auth/plugin/datasource/<path:provider_id>")
class DatasourceAuth(Resource):
    @console_ns.expect(console_ns.models[DatasourceCredentialPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        payload = DatasourceCredentialPayload.model_validate(console_ns.payload or {})
        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()

        try:
            datasource_provider_service.add_datasource_api_key_provider(
                tenant_id=current_tenant_id,
                provider_id=datasource_provider_id,
                credentials=payload.credentials,
                name=payload.name,
            )
        except CredentialsValidateFailedError as ex:
            raise ValueError(str(ex))
        return {"result": "success"}, 200

    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider_id: str):
        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()
        _, current_tenant_id = current_account_with_tenant()

        datasources = datasource_provider_service.list_datasource_credentials(
            tenant_id=current_tenant_id,
            provider=datasource_provider_id.provider_name,
            plugin_id=datasource_provider_id.plugin_id,
        )
        return {"result": datasources}, 200


@console_ns.route("/auth/plugin/datasource/<path:provider_id>/delete")
class DatasourceAuthDeleteApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceCredentialDeletePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        datasource_provider_id = DatasourceProviderID(provider_id)
        plugin_id = datasource_provider_id.plugin_id
        provider_name = datasource_provider_id.provider_name

        payload = DatasourceCredentialDeletePayload.model_validate(console_ns.payload or {})
        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.remove_datasource_credentials(
            tenant_id=current_tenant_id,
            auth_id=payload.credential_id,
            provider=provider_name,
            plugin_id=plugin_id,
        )
        return {"result": "success"}, 200


@console_ns.route("/auth/plugin/datasource/<path:provider_id>/update")
class DatasourceAuthUpdateApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceCredentialUpdatePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        datasource_provider_id = DatasourceProviderID(provider_id)
        payload = DatasourceCredentialUpdatePayload.model_validate(console_ns.payload or {})

        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.update_datasource_credentials(
            tenant_id=current_tenant_id,
            auth_id=payload.credential_id,
            provider=datasource_provider_id.provider_name,
            plugin_id=datasource_provider_id.plugin_id,
            credentials=payload.credentials or {},
            name=payload.name,
        )
        return {"result": "success"}, 201


@console_ns.route("/auth/plugin/datasource/list")
class DatasourceAuthListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()

        datasource_provider_service = DatasourceProviderService()
        datasources = datasource_provider_service.get_all_datasource_credentials(tenant_id=current_tenant_id)
        return {"result": jsonable_encoder(datasources)}, 200


@console_ns.route("/auth/plugin/datasource/default-list")
class DatasourceHardCodeAuthListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()

        datasource_provider_service = DatasourceProviderService()
        datasources = datasource_provider_service.get_hard_code_datasource_credentials(tenant_id=current_tenant_id)
        return {"result": jsonable_encoder(datasources)}, 200


@console_ns.route("/auth/plugin/datasource/<path:provider_id>/custom-client")
class DatasourceAuthOauthCustomClient(Resource):
    @console_ns.expect(console_ns.models[DatasourceCustomClientPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        payload = DatasourceCustomClientPayload.model_validate(console_ns.payload or {})
        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.setup_oauth_custom_client_params(
            tenant_id=current_tenant_id,
            datasource_provider_id=datasource_provider_id,
            client_params=payload.client_params or {},
            enabled=payload.enable_oauth_custom_client or False,
        )
        return {"result": "success"}, 200

    @setup_required
    @login_required
    @account_initialization_required
    def delete(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.remove_oauth_custom_client_params(
            tenant_id=current_tenant_id,
            datasource_provider_id=datasource_provider_id,
        )
        return {"result": "success"}, 200


@console_ns.route("/auth/plugin/datasource/<path:provider_id>/default")
class DatasourceAuthDefaultApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceDefaultPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        payload = DatasourceDefaultPayload.model_validate(console_ns.payload or {})
        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.set_default_datasource_provider(
            tenant_id=current_tenant_id,
            datasource_provider_id=datasource_provider_id,
            credential_id=payload.id,
        )
        return {"result": "success"}, 200


@console_ns.route("/auth/plugin/datasource/<path:provider_id>/update-name")
class DatasourceUpdateProviderNameApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceUpdateNamePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self, provider_id: str):
        _, current_tenant_id = current_account_with_tenant()

        payload = DatasourceUpdateNamePayload.model_validate(console_ns.payload or {})
        datasource_provider_id = DatasourceProviderID(provider_id)
        datasource_provider_service = DatasourceProviderService()
        datasource_provider_service.update_datasource_provider_name(
            tenant_id=current_tenant_id,
            datasource_provider_id=datasource_provider_id,
            name=payload.name,
            credential_id=payload.credential_id,
        )
        return {"result": "success"}, 200

```

### api/controllers/console/datasets/rag_pipeline/rag_pipeline_datasets.py
```py
from flask_restx import Resource, marshal
from pydantic import BaseModel
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Forbidden

import services
from controllers.common.schema import register_schema_model
from controllers.console import console_ns
from controllers.console.datasets.error import DatasetNameDuplicateError
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_rate_limit_check,
    setup_required,
)
from extensions.ext_database import db
from fields.dataset_fields import dataset_detail_fields
from libs.login import current_account_with_tenant, login_required
from models.dataset import DatasetPermissionEnum
from services.dataset_service import DatasetPermissionService, DatasetService
from services.entities.knowledge_entities.rag_pipeline_entities import IconInfo, RagPipelineDatasetCreateEntity
from services.rag_pipeline.rag_pipeline_dsl_service import RagPipelineDslService


class RagPipelineDatasetImportPayload(BaseModel):
    yaml_content: str


register_schema_model(console_ns, RagPipelineDatasetImportPayload)


@console_ns.route("/rag/pipeline/dataset")
class CreateRagPipelineDatasetApi(Resource):
    @console_ns.expect(console_ns.models[RagPipelineDatasetImportPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def post(self):
        payload = RagPipelineDatasetImportPayload.model_validate(console_ns.payload or {})
        current_user, current_tenant_id = current_account_with_tenant()
        # The role of the current user in the ta table must be admin, owner, or editor, or dataset_operator
        if not current_user.is_dataset_editor:
            raise Forbidden()
        rag_pipeline_dataset_create_entity = RagPipelineDatasetCreateEntity(
            name="",
            description="",
            icon_info=IconInfo(
                icon="📙",
                icon_background="#FFF4ED",
                icon_type="emoji",
            ),
            permission=DatasetPermissionEnum.ONLY_ME,
            partial_member_list=None,
            yaml_content=payload.yaml_content,
        )
        try:
            with sessionmaker(db.engine).begin() as session:
                rag_pipeline_dsl_service = RagPipelineDslService(session)
                import_info = rag_pipeline_dsl_service.create_rag_pipeline_dataset(
                    tenant_id=current_tenant_id,
                    rag_pipeline_dataset_create_entity=rag_pipeline_dataset_create_entity,
                )
            if rag_pipeline_dataset_create_entity.permission == "partial_members":
                DatasetPermissionService.update_partial_member_list(
                    current_tenant_id,
                    import_info["dataset_id"],
                    rag_pipeline_dataset_create_entity.partial_member_list,
                )
        except services.errors.dataset.DatasetNameDuplicateError:
            raise DatasetNameDuplicateError()

        return import_info, 201


@console_ns.route("/rag/pipeline/empty-dataset")
class CreateEmptyRagPipelineDatasetApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def post(self):
        # The role of the current user in the ta table must be admin, owner, or editor, or dataset_operator
        current_user, current_tenant_id = current_account_with_tenant()

        if not current_user.is_dataset_editor:
            raise Forbidden()
        dataset = DatasetService.create_empty_rag_pipeline_dataset(
            tenant_id=current_tenant_id,
            rag_pipeline_dataset_create_entity=RagPipelineDatasetCreateEntity(
                name="",
                description="",
                icon_info=IconInfo(
                    icon="📙",
                    icon_background="#FFF4ED",
                    icon_type="emoji",
                ),
                permission=DatasetPermissionEnum.ONLY_ME,
                partial_member_list=None,
            ),
        )
        return marshal(dataset, dataset_detail_fields), 201

```

### api/controllers/console/datasets/rag_pipeline/datasource_content_preview.py
```py
from flask_restx import (  # type: ignore
    Resource,  # type: ignore
)
from pydantic import BaseModel
from werkzeug.exceptions import Forbidden

from controllers.console import console_ns
from controllers.console.datasets.wraps import get_rag_pipeline
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import current_user, login_required
from models import Account
from models.dataset import Pipeline
from services.rag_pipeline.rag_pipeline import RagPipelineService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class Parser(BaseModel):
    inputs: dict
    datasource_type: str
    credential_id: str | None = None


console_ns.schema_model(Parser.__name__, Parser.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/published/datasource/nodes/<string:node_id>/preview")
class DataSourceContentPreviewApi(Resource):
    @console_ns.expect(console_ns.models[Parser.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run datasource content preview
        """
        if not isinstance(current_user, Account):
            raise Forbidden()

        args = Parser.model_validate(console_ns.payload)

        inputs = args.inputs
        datasource_type = args.datasource_type
        rag_pipeline_service = RagPipelineService()
        preview_content = rag_pipeline_service.run_datasource_node_preview(
            pipeline=pipeline,
            node_id=node_id,
            user_inputs=inputs,
            account=current_user,
            datasource_type=datasource_type,
            is_published=True,
            credential_id=args.credential_id,
        )
        return preview_content, 200

```

### api/controllers/console/datasets/rag_pipeline/rag_pipeline_workflow.py
```py
import json
import logging
from typing import Any, Literal, cast

from flask import abort, request
from flask_restx import Resource, marshal_with  # type: ignore
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field, ValidationError
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, Forbidden, InternalServerError, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.app.error import (
    ConversationCompletedError,
    DraftWorkflowNotExist,
    DraftWorkflowNotSync,
)
from controllers.console.app.workflow import (
    RESTORE_SOURCE_WORKFLOW_MUST_BE_PUBLISHED_MESSAGE,
    workflow_model,
    workflow_pagination_model,
)
from controllers.console.app.workflow_run import (
    workflow_run_detail_model,
    workflow_run_node_execution_list_model,
    workflow_run_node_execution_model,
    workflow_run_pagination_model,
)
from controllers.console.datasets.wraps import get_rag_pipeline
from controllers.console.wraps import (
    account_initialization_required,
    edit_permission_required,
    setup_required,
)
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.apps.base_app_queue_manager import AppQueueManager
from core.app.apps.pipeline.pipeline_generator import PipelineGenerator
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from factories import variable_factory
from libs import helper
from libs.helper import TimestampField, UUIDStrOrEmpty
from libs.login import current_account_with_tenant, current_user, login_required
from models import Account
from models.dataset import Pipeline
from models.model import EndUser
from models.workflow import Workflow
from services.errors.app import IsDraftWorkflowError, WorkflowHashNotEqualError, WorkflowNotFoundError
from services.errors.llm import InvokeRateLimitError
from services.rag_pipeline.pipeline_generate_service import PipelineGenerateService
from services.rag_pipeline.rag_pipeline import RagPipelineService
from services.rag_pipeline.rag_pipeline_manage_service import RagPipelineManageService
from services.rag_pipeline.rag_pipeline_transform_service import RagPipelineTransformService
from services.workflow_service import DraftWorkflowDeletionError, WorkflowInUseError, WorkflowService

logger = logging.getLogger(__name__)


class DraftWorkflowSyncPayload(BaseModel):
    graph: dict[str, Any]
    hash: str | None = None
    environment_variables: list[dict[str, Any]] | None = None
    conversation_variables: list[dict[str, Any]] | None = None
    rag_pipeline_variables: list[dict[str, Any]] | None = None
    features: dict[str, Any] | None = None


class NodeRunPayload(BaseModel):
    inputs: dict[str, Any] | None = None


class NodeRunRequiredPayload(BaseModel):
    inputs: dict[str, Any]


class DatasourceNodeRunPayload(BaseModel):
    inputs: dict[str, Any]
    datasource_type: str
    credential_id: str | None = None


class DraftWorkflowRunPayload(BaseModel):
    inputs: dict[str, Any]
    datasource_type: str
    datasource_info_list: list[dict[str, Any]]
    start_node_id: str


class PublishedWorkflowRunPayload(DraftWorkflowRunPayload):
    is_preview: bool = False
    response_mode: Literal["streaming", "blocking"] = "streaming"
    original_document_id: str | None = None


class DefaultBlockConfigQuery(BaseModel):
    q: str | None = None


class WorkflowListQuery(BaseModel):
    page: int = Field(default=1, ge=1, le=99999)
    limit: int = Field(default=10, ge=1, le=100)
    user_id: str | None = None
    named_only: bool = False


class WorkflowUpdatePayload(BaseModel):
    marked_name: str | None = Field(default=None, max_length=20)
    marked_comment: str | None = Field(default=None, max_length=100)


class NodeIdQuery(BaseModel):
    node_id: str


class WorkflowRunQuery(BaseModel):
    last_id: UUIDStrOrEmpty | None = None
    limit: int = Field(default=20, ge=1, le=100)


class DatasourceVariablesPayload(BaseModel):
    datasource_type: str
    datasource_info: dict[str, Any]
    start_node_id: str
    start_node_title: str


class RagPipelineRecommendedPluginQuery(BaseModel):
    type: str = "all"


register_schema_models(
    console_ns,
    DraftWorkflowSyncPayload,
    NodeRunPayload,
    NodeRunRequiredPayload,
    DatasourceNodeRunPayload,
    DraftWorkflowRunPayload,
    PublishedWorkflowRunPayload,
    DefaultBlockConfigQuery,
    WorkflowListQuery,
    WorkflowUpdatePayload,
    NodeIdQuery,
    WorkflowRunQuery,
    DatasourceVariablesPayload,
    RagPipelineRecommendedPluginQuery,
)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft")
class DraftRagPipelineApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    @marshal_with(workflow_model)
    def get(self, pipeline: Pipeline):
        """
        Get draft rag pipeline's workflow
        """
        # fetch draft workflow by app_model
        rag_pipeline_service = RagPipelineService()
        workflow = rag_pipeline_service.get_draft_workflow(pipeline=pipeline)

        if not workflow:
            raise DraftWorkflowNotExist()

        # return workflow, if not found, return None (initiate graph by frontend)
        return workflow

    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def post(self, pipeline: Pipeline):
        """
        Sync draft workflow
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        content_type = request.headers.get("Content-Type", "")

        if "application/json" in content_type:
            payload_dict = console_ns.payload or {}
            payload = DraftWorkflowSyncPayload.model_validate(payload_dict)
        elif "text/plain" in content_type:
            try:
                payload = DraftWorkflowSyncPayload.model_validate_json(request.data)
            except (ValueError, ValidationError):
                return {"message": "Invalid JSON data"}, 400
        else:
            abort(415)
        rag_pipeline_service = RagPipelineService()

        try:
            environment_variables_list = Workflow.normalize_environment_variable_mappings(
                payload.environment_variables or [],
            )
            environment_variables = [
                variable_factory.build_environment_variable_from_mapping(obj) for obj in environment_variables_list
            ]
            conversation_variables_list = payload.conversation_variables or []
            conversation_variables = [
                variable_factory.build_conversation_variable_from_mapping(obj) for obj in conversation_variables_list
            ]
            workflow = rag_pipeline_service.sync_draft_workflow(
                pipeline=pipeline,
                graph=payload.graph,
                unique_hash=payload.hash,
                account=current_user,
                environment_variables=environment_variables,
                conversation_variables=conversation_variables,
                rag_pipeline_variables=payload.rag_pipeline_variables or [],
            )
        except WorkflowHashNotEqualError:
            raise DraftWorkflowNotSync()

        return {
            "result": "success",
            "hash": workflow.unique_hash,
            "updated_at": TimestampField().format(workflow.updated_at or workflow.created_at),
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/iteration/nodes/<string:node_id>/run")
class RagPipelineDraftRunIterationNodeApi(Resource):
    @console_ns.expect(console_ns.models[NodeRunPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run draft workflow iteration node
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = NodeRunPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        try:
            response = PipelineGenerateService.generate_single_iteration(
                pipeline=pipeline, user=current_user, node_id=node_id, args=args, streaming=True
            )

            return helper.compact_generate_response(response)
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except services.errors.conversation.ConversationCompletedError:
            raise ConversationCompletedError()
        except ValueError as e:
            raise e
        except Exception:
            logging.exception("internal server error.")
            raise InternalServerError()


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/loop/nodes/<string:node_id>/run")
class RagPipelineDraftRunLoopNodeApi(Resource):
    @console_ns.expect(console_ns.models[NodeRunPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run draft workflow loop node
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = NodeRunPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        try:
            response = PipelineGenerateService.generate_single_loop(
                pipeline=pipeline, user=current_user, node_id=node_id, args=args, streaming=True
            )

            return helper.compact_generate_response(response)
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except services.errors.conversation.ConversationCompletedError:
            raise ConversationCompletedError()
        except ValueError as e:
            raise e
        except Exception:
            logging.exception("internal server error.")
            raise InternalServerError()


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/run")
class DraftRagPipelineRunApi(Resource):
    @console_ns.expect(console_ns.models[DraftWorkflowRunPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline):
        """
        Run draft workflow
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = DraftWorkflowRunPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump()

        try:
            response = PipelineGenerateService.generate(
                pipeline=pipeline,
                user=current_user,
                args=args,
                invoke_from=InvokeFrom.DEBUGGER,
                streaming=True,
            )

            return helper.compact_generate_response(response)
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/published/run")
class PublishedRagPipelineRunApi(Resource):
    @console_ns.expect(console_ns.models[PublishedWorkflowRunPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline):
        """
        Run published workflow
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = PublishedWorkflowRunPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)
        streaming = payload.response_mode == "streaming"

        try:
            response = PipelineGenerateService.generate(
                pipeline=pipeline,
                user=current_user,
                args=args,
                invoke_from=InvokeFrom.DEBUGGER if payload.is_preview else InvokeFrom.PUBLISHED_PIPELINE,
                streaming=streaming,
            )

            return helper.compact_generate_response(response)
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)


# class RagPipelinePublishedDatasourceNodeRunStatusApi(Resource):
#     @setup_required
#     @login_required
#     @account_initialization_required
#     @get_rag_pipeline
#     def post(self, pipeline: Pipeline, node_id: str):
#         """
#         Run rag pipeline datasource
#         """
#         # The role of the current user in the ta table must be admin, owner, or editor
#         if not current_user.has_edit_permission:
#             raise Forbidden()
#
#         if not isinstance(current_user, Account):
#             raise Forbidden()
#
#         parser = (reqparse.RequestParser()
#             .add_argument("job_id", type=str, required=True, nullable=False, location="json")
#             .add_argument("datasource_type", type=str, required=True, location="json")
#         )
#         args = parser.parse_args()
#
#         job_id = args.get("job_id")
#         if job_id == None:
#             raise ValueError("missing job_id")
#         datasource_type = args.get("datasource_type")
#         if datasource_type == None:
#             raise ValueError("missing datasource_type")
#
#         rag_pipeline_service = RagPipelineService()
#         result = rag_pipeline_service.run_datasource_workflow_node_status(
#             pipeline=pipeline,
#             node_id=node_id,
#             job_id=job_id,
#             account=current_user,
#             datasource_type=datasource_type,
#             is_published=True
#         )
#
#         return result


# class RagPipelineDraftDatasourceNodeRunStatusApi(Resource):
#     @setup_required
#     @login_required
#     @account_initialization_required
#     @get_rag_pipeline
#     def post(self, pipeline: Pipeline, node_id: str):
#         """
#         Run rag pipeline datasource
#         """
#         # The role of the current user in the ta table must be admin, owner, or editor
#         if not current_user.has_edit_permission:
#             raise Forbidden()
#
#         if not isinstance(current_user, Account):
#             raise Forbidden()
#
#         parser = (reqparse.RequestParser()
#             .add_argument("job_id", type=str, required=True, nullable=False, location="json")
#             .add_argument("datasource_type", type=str, required=True, location="json")
#         )
#         args = parser.parse_args()
#
#         job_id = args.get("job_id")
#         if job_id == None:
#             raise ValueError("missing job_id")
#         datasource_type = args.get("datasource_type")
#         if datasource_type == None:
#             raise ValueError("missing datasource_type")
#
#         rag_pipeline_service = RagPipelineService()
#         result = rag_pipeline_service.run_datasource_workflow_node_status(
#             pipeline=pipeline,
#             node_id=node_id,
#             job_id=job_id,
#             account=current_user,
#             datasource_type=datasource_type,
#             is_published=False
#         )
#
#         return result
#
@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/published/datasource/nodes/<string:node_id>/run")
class RagPipelinePublishedDatasourceNodeRunApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceNodeRunPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run rag pipeline datasource
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = DatasourceNodeRunPayload.model_validate(console_ns.payload or {})

        rag_pipeline_service = RagPipelineService()
        return helper.compact_generate_response(
            PipelineGenerator.convert_to_event_stream(
                rag_pipeline_service.run_datasource_workflow_node(
                    pipeline=pipeline,
                    node_id=node_id,
                    user_inputs=payload.inputs,
                    account=current_user,
                    datasource_type=payload.datasource_type,
                    is_published=False,
                    credential_id=payload.credential_id,
                )
            )
        )


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/datasource/nodes/<string:node_id>/run")
class RagPipelineDraftDatasourceNodeRunApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceNodeRunPayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run rag pipeline datasource
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = DatasourceNodeRunPayload.model_validate(console_ns.payload or {})

        rag_pipeline_service = RagPipelineService()
        return helper.compact_generate_response(
            PipelineGenerator.convert_to_event_stream(
                rag_pipeline_service.run_datasource_workflow_node(
                    pipeline=pipeline,
                    node_id=node_id,
                    user_inputs=payload.inputs,
                    account=current_user,
                    datasource_type=payload.datasource_type,
                    is_published=False,
                    credential_id=payload.credential_id,
                )
            )
        )


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/nodes/<string:node_id>/run")
class RagPipelineDraftNodeRunApi(Resource):
    @console_ns.expect(console_ns.models[NodeRunRequiredPayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    @get_rag_pipeline
    @marshal_with(workflow_run_node_execution_model)
    def post(self, pipeline: Pipeline, node_id: str):
        """
        Run draft workflow node
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        payload = NodeRunRequiredPayload.model_validate(console_ns.payload or {})
        inputs = payload.inputs

        rag_pipeline_service = RagPipelineService()
        workflow_node_execution = rag_pipeline_service.run_draft_workflow_node(
            pipeline=pipeline, node_id=node_id, user_inputs=inputs, account=current_user
        )

        if workflow_node_execution is None:
            raise ValueError("Workflow node execution not found")

        return workflow_node_execution


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflow-runs/tasks/<string:task_id>/stop")
class RagPipelineTaskStopApi(Resource):
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, task_id: str):
        """
        Stop workflow task
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()

        AppQueueManager.set_stop_flag(task_id, InvokeFrom.DEBUGGER, current_user.id)

        return {"result": "success"}


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/publish")
class PublishedRagPipelineApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    @marshal_with(workflow_model)
    def get(self, pipeline: Pipeline):
        """
        Get published pipeline
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        if not pipeline.is_published:
            return None
        # fetch published workflow by pipeline
        rag_pipeline_service = RagPipelineService()
        workflow = rag_pipeline_service.get_published_workflow(pipeline=pipeline)

        # return workflow, if not found, return None
        return workflow

    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline):
        """
        Publish workflow
        """
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, _ = current_account_with_tenant()
        rag_pipeline_service = RagPipelineService()
        workflow = rag_pipeline_service.publish_workflow(
            session=db.session,  # type: ignore[reportArgumentType,arg-type]
            pipeline=pipeline,
            account=current_user,
        )
        pipeline.is_published = True
        pipeline.workflow_id = workflow.id
        db.session.commit()
        workflow_created_at = TimestampField().format(workflow.created_at)

        return {
            "result": "success",
            "created_at": workflow_created_at,
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/default-workflow-block-configs")
class DefaultRagPipelineBlockConfigsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def get(self, pipeline: Pipeline):
        """
        Get default block config
        """
        # Get default block configs
        rag_pipeline_service = RagPipelineService()
        return rag_pipeline_service.get_default_block_configs()


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/default-workflow-block-configs/<string:block_type>")
class DefaultRagPipelineBlockConfigApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def get(self, pipeline: Pipeline, block_type: str):
        """
        Get default block config
        """
        query = DefaultBlockConfigQuery.model_validate(request.args.to_dict())

        filters = None
        if query.q:
            try:
                filters = json.loads(query.q)
            except json.JSONDecodeError:
                raise ValueError("Invalid filters")

        # Get default block configs
        rag_pipeline_service = RagPipelineService()
        return rag_pipeline_service.get_default_block_config(node_type=block_type, filters=filters)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows")
class PublishedAllRagPipelineApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    @marshal_with(workflow_pagination_model)
    def get(self, pipeline: Pipeline):
        """
        Get published workflows
        """
        current_user, _ = current_account_with_tenant()

        query = WorkflowListQuery.model_validate(request.args.to_dict())

        page = query.page
        limit = query.limit
        user_id = query.user_id
        named_only = query.named_only

        if user_id:
            if user_id != current_user.id:
                raise Forbidden()

        rag_pipeline_service = RagPipelineService()
        with sessionmaker(db.engine).begin() as session:
            workflows, has_more = rag_pipeline_service.get_all_published_workflow(
                session=session,
                pipeline=pipeline,
                page=page,
                limit=limit,
                user_id=user_id,
                named_only=named_only,
            )

            return {
                "items": workflows,
                "page": page,
                "limit": limit,
                "has_more": has_more,
            }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/<string:workflow_id>/restore")
class RagPipelineDraftWorkflowRestoreApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def post(self, pipeline: Pipeline, workflow_id: str):
        current_user, _ = current_account_with_tenant()
        rag_pipeline_service = RagPipelineService()

        try:
            workflow = rag_pipeline_service.restore_published_workflow_to_draft(
                pipeline=pipeline,
                workflow_id=workflow_id,
                account=current_user,
            )
        except IsDraftWorkflowError as exc:
            # Use a stable, predefined message to keep the 400 response consistent
            raise BadRequest(RESTORE_SOURCE_WORKFLOW_MUST_BE_PUBLISHED_MESSAGE) from exc
        except WorkflowNotFoundError as exc:
            raise NotFound(str(exc)) from exc

        return {
            "result": "success",
            "hash": workflow.unique_hash,
            "updated_at": TimestampField().format(workflow.updated_at or workflow.created_at),
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/<string:workflow_id>")
class RagPipelineByIdApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    @marshal_with(workflow_model)
    def patch(self, pipeline: Pipeline, workflow_id: str):
        """
        Update workflow attributes
        """
        # Check permission
        current_user, _ = current_account_with_tenant()

        payload = WorkflowUpdatePayload.model_validate(console_ns.payload or {})
        update_data = payload.model_dump(exclude_unset=True)

        if not update_data:
            return {"message": "No valid fields to update"}, 400

        rag_pipeline_service = RagPipelineService()

        # Create a session and manage the transaction
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            workflow = rag_pipeline_service.update_workflow(
                session=session,
                workflow_id=workflow_id,
                tenant_id=pipeline.tenant_id,
                account_id=current_user.id,
                data=update_data,
            )

            if not workflow:
                raise NotFound("Workflow not found")

            return workflow

    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @get_rag_pipeline
    def delete(self, pipeline: Pipeline, workflow_id: str):
        """
        Delete a published workflow version that is not currently active on the pipeline.
        """
        if pipeline.workflow_id == workflow_id:
            abort(400, description=f"Cannot delete workflow that is currently in use by pipeline '{pipeline.id}'")

        workflow_service = WorkflowService()

        with sessionmaker(db.engine).begin() as session:
            try:
                workflow_service.delete_workflow(
                    session=session,
                    workflow_id=workflow_id,
                    tenant_id=pipeline.tenant_id,
                )
            except WorkflowInUseError as e:
                abort(400, description=str(e))
            except DraftWorkflowDeletionError as e:
                abort(400, description=str(e))
            except ValueError as e:
                raise NotFound(str(e))

        return None, 204


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/published/processing/parameters")
class PublishedRagPipelineSecondStepApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def get(self, pipeline: Pipeline):
        """
        Get second step parameters of rag pipeline
        """
        query = NodeIdQuery.model_validate(request.args.to_dict())
        node_id = query.node_id
        rag_pipeline_service = RagPipelineService()
        variables = rag_pipeline_service.get_second_step_parameters(pipeline=pipeline, node_id=node_id, is_draft=False)
        return {
            "variables": variables,
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/published/pre-processing/parameters")
class PublishedRagPipelineFirstStepApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def get(self, pipeline: Pipeline):
        """
        Get first step parameters of rag pipeline
        """
        query = NodeIdQuery.model_validate(request.args.to_dict())
        node_id = query.node_id
        rag_pipeline_service = RagPipelineService()
        variables = rag_pipeline_service.get_first_step_parameters(pipeline=pipeline, node_id=node_id, is_draft=False)
        return {
            "variables": variables,
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/pre-processing/parameters")
class DraftRagPipelineFirstStepApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def get(self, pipeline: Pipeline):
        """
        Get first step parameters of rag pipeline
        """
        query = NodeIdQuery.model_validate(request.args.to_dict())
        node_id = query.node_id
        rag_pipeline_service = RagPipelineService()
        variables = rag_pipeline_service.get_first_step_parameters(pipeline=pipeline, node_id=node_id, is_draft=True)
        return {
            "variables": variables,
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/processing/parameters")
class DraftRagPipelineSecondStepApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    def get(self, pipeline: Pipeline):
        """
        Get second step parameters of rag pipeline
        """
        query = NodeIdQuery.model_validate(request.args.to_dict())
        node_id = query.node_id

        rag_pipeline_service = RagPipelineService()
        variables = rag_pipeline_service.get_second_step_parameters(pipeline=pipeline, node_id=node_id, is_draft=True)
        return {
            "variables": variables,
        }


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflow-runs")
class RagPipelineWorkflowRunListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @marshal_with(workflow_run_pagination_model)
    def get(self, pipeline: Pipeline):
        """
        Get workflow run list
        """
        query = WorkflowRunQuery.model_validate(
            {
                "last_id": request.args.get("last_id"),
                "limit": request.args.get("limit", type=int, default=20),
            }
        )
        args = {
            "last_id": str(query.last_id) if query.last_id else None,
            "limit": query.limit,
        }

        rag_pipeline_service = RagPipelineService()
        result = rag_pipeline_service.get_rag_pipeline_paginate_workflow_runs(pipeline=pipeline, args=args)

        return result


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflow-runs/<uuid:run_id>")
class RagPipelineWorkflowRunDetailApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @marshal_with(workflow_run_detail_model)
    def get(self, pipeline: Pipeline, run_id):
        """
        Get workflow run detail
        """
        run_id = str(run_id)

        rag_pipeline_service = RagPipelineService()
        workflow_run = rag_pipeline_service.get_rag_pipeline_workflow_run(pipeline=pipeline, run_id=run_id)

        return workflow_run


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflow-runs/<uuid:run_id>/node-executions")
class RagPipelineWorkflowRunNodeExecutionListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @marshal_with(workflow_run_node_execution_list_model)
    def get(self, pipeline: Pipeline, run_id: str):
        """
        Get workflow run node execution list
        """
        run_id = str(run_id)

        rag_pipeline_service = RagPipelineService()
        user = cast("Account | EndUser", current_user)
        node_executions = rag_pipeline_service.get_rag_pipeline_workflow_run_node_executions(
            pipeline=pipeline,
            run_id=run_id,
            user=user,
        )

        return {"data": node_executions}


@console_ns.route("/rag/pipelines/datasource-plugins")
class DatasourceListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()
        return jsonable_encoder(RagPipelineManageService.list_rag_pipeline_datasources(current_tenant_id))


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/nodes/<string:node_id>/last-run")
class RagPipelineWorkflowLastRunApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @marshal_with(workflow_run_node_execution_model)
    def get(self, pipeline: Pipeline, node_id: str):
        rag_pipeline_service = RagPipelineService()
        workflow = rag_pipeline_service.get_draft_workflow(pipeline=pipeline)
        if not workflow:
            raise NotFound("Workflow not found")
        node_exec = rag_pipeline_service.get_node_last_run(
            pipeline=pipeline,
            workflow=workflow,
            node_id=node_id,
        )
        if node_exec is None:
            raise NotFound("last run not found")
        return node_exec


@console_ns.route("/rag/pipelines/transform/datasets/<uuid:dataset_id>")
class RagPipelineTransformApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, dataset_id: str):
        current_user, _ = current_account_with_tenant()

        if not (current_user.has_edit_permission or current_user.is_dataset_operator):
            raise Forbidden()

        dataset_id = str(dataset_id)
        rag_pipeline_transform_service = RagPipelineTransformService()
        result = rag_pipeline_transform_service.transform_dataset(dataset_id)
        return result


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/datasource/variables-inspect")
class RagPipelineDatasourceVariableApi(Resource):
    @console_ns.expect(console_ns.models[DatasourceVariablesPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @get_rag_pipeline
    @edit_permission_required
    @marshal_with(workflow_run_node_execution_model)
    def post(self, pipeline: Pipeline):
        """
        Set datasource variables
        """
        current_user, _ = current_account_with_tenant()
        args = DatasourceVariablesPayload.model_validate(console_ns.payload or {}).model_dump()

        rag_pipeline_service = RagPipelineService()
        workflow_node_execution = rag_pipeline_service.set_datasource_variables(
            pipeline=pipeline,
            args=args,
            current_user=current_user,
        )
        return workflow_node_execution


@console_ns.route("/rag/pipelines/recommended-plugins")
class RagPipelineRecommendedPluginApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        query = RagPipelineRecommendedPluginQuery.model_validate(request.args.to_dict())

        rag_pipeline_service = RagPipelineService()
        recommended_plugins = rag_pipeline_service.get_recommended_plugins(query.type)
        return recommended_plugins

```

### api/controllers/console/datasets/rag_pipeline/rag_pipeline_draft_variable.py
```py
import logging
from collections.abc import Callable
from typing import Any, NoReturn

from flask import Response, request
from flask_restx import Resource, marshal, marshal_with
from graphon.variables.types import SegmentType
from pydantic import BaseModel, Field
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Forbidden

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.app.error import (
    DraftWorkflowNotExist,
)
from controllers.console.app.workflow_draft_variable import (
    _WORKFLOW_DRAFT_VARIABLE_FIELDS,  # type: ignore[private-usage]
    workflow_draft_variable_list_model,
    workflow_draft_variable_list_without_value_model,
    workflow_draft_variable_model,
)
from controllers.console.datasets.wraps import get_rag_pipeline
from controllers.console.wraps import account_initialization_required, setup_required
from controllers.web.error import InvalidArgumentError, NotFoundError
from core.app.file_access import DatabaseFileAccessController
from core.workflow.variable_prefixes import CONVERSATION_VARIABLE_NODE_ID, SYSTEM_VARIABLE_NODE_ID
from extensions.ext_database import db
from factories.file_factory import build_from_mapping, build_from_mappings
from factories.variable_factory import build_segment_with_type
from libs.login import current_user, login_required
from models import Account
from models.dataset import Pipeline
from services.rag_pipeline.rag_pipeline import RagPipelineService
from services.workflow_draft_variable_service import WorkflowDraftVariableList, WorkflowDraftVariableService

logger = logging.getLogger(__name__)
_file_access_controller = DatabaseFileAccessController()


def _create_pagination_parser():
    class PaginationQuery(BaseModel):
        page: int = Field(default=1, ge=1, le=100_000)
        limit: int = Field(default=20, ge=1, le=100)

    register_schema_models(console_ns, PaginationQuery)

    return PaginationQuery


class WorkflowDraftVariablePatchPayload(BaseModel):
    name: str | None = None
    value: Any | None = None


register_schema_models(console_ns, WorkflowDraftVariablePatchPayload)


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
    @get_rag_pipeline
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R | Response:
        if not isinstance(current_user, Account) or not current_user.has_edit_permission:
            raise Forbidden()
        return f(*args, **kwargs)

    return wrapper


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/variables")
class RagPipelineVariableCollectionApi(Resource):
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_without_value_model)
    def get(self, pipeline: Pipeline):
        """
        Get draft workflow
        """
        pagination = _create_pagination_parser()
        query = pagination.model_validate(request.args.to_dict())

        # fetch draft workflow by app_model
        rag_pipeline_service = RagPipelineService()
        workflow_exist = rag_pipeline_service.is_workflow_exist(pipeline=pipeline)
        if not workflow_exist:
            raise DraftWorkflowNotExist()

        # fetch draft workflow by app_model
        with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
            draft_var_srv = WorkflowDraftVariableService(
                session=session,
            )
        workflow_vars = draft_var_srv.list_variables_without_values(
            app_id=pipeline.id,
            page=query.page,
            limit=query.limit,
            user_id=current_user.id,
        )

        return workflow_vars

    @_api_prerequisite
    def delete(self, pipeline: Pipeline):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        draft_var_srv.delete_user_workflow_variables(pipeline.id, user_id=current_user.id)
        db.session.commit()
        return Response("", 204)


def validate_node_id(node_id: str) -> NoReturn | None:
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
    return None


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/nodes/<string:node_id>/variables")
class RagPipelineNodeVariableCollectionApi(Resource):
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_model)
    def get(self, pipeline: Pipeline, node_id: str):
        validate_node_id(node_id)
        with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
            draft_var_srv = WorkflowDraftVariableService(
                session=session,
            )
            node_vars = draft_var_srv.list_node_variables(pipeline.id, node_id, user_id=current_user.id)

        return node_vars

    @_api_prerequisite
    def delete(self, pipeline: Pipeline, node_id: str):
        validate_node_id(node_id)
        srv = WorkflowDraftVariableService(db.session())
        srv.delete_node_variables(pipeline.id, node_id, user_id=current_user.id)
        db.session.commit()
        return Response("", 204)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/variables/<uuid:variable_id>")
class RagPipelineVariableApi(Resource):
    _PATCH_NAME_FIELD = "name"
    _PATCH_VALUE_FIELD = "value"

    @_api_prerequisite
    @marshal_with(workflow_draft_variable_model)
    def get(self, pipeline: Pipeline, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        variable = draft_var_srv.get_variable(variable_id=variable_id)
        if variable is None:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        if variable.app_id != pipeline.id:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        return variable

    @_api_prerequisite
    @marshal_with(workflow_draft_variable_model)
    @console_ns.expect(console_ns.models[WorkflowDraftVariablePatchPayload.__name__])
    def patch(self, pipeline: Pipeline, variable_id: str):
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
        payload = WorkflowDraftVariablePatchPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        variable = draft_var_srv.get_variable(variable_id=variable_id)
        if variable is None:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        if variable.app_id != pipeline.id:
            raise NotFoundError(description=f"variable not found, id={variable_id}")

        new_name = args.get(self._PATCH_NAME_FIELD, None)
        raw_value = args.get(self._PATCH_VALUE_FIELD, None)
        if new_name is None and raw_value is None:
            return variable

        new_value = None
        if raw_value is not None:
            if variable.value_type == SegmentType.FILE:
                if not isinstance(raw_value, dict):
                    raise InvalidArgumentError(description=f"expected dict for file, got {type(raw_value)}")
                raw_value = build_from_mapping(
                    mapping=raw_value,
                    tenant_id=pipeline.tenant_id,
                    access_controller=_file_access_controller,
                )
            elif variable.value_type == SegmentType.ARRAY_FILE:
                if not isinstance(raw_value, list):
                    raise InvalidArgumentError(description=f"expected list for files, got {type(raw_value)}")
                if len(raw_value) > 0 and not isinstance(raw_value[0], dict):
                    raise InvalidArgumentError(description=f"expected dict for files[0], got {type(raw_value)}")
                raw_value = build_from_mappings(
                    mappings=raw_value,
                    tenant_id=pipeline.tenant_id,
                    access_controller=_file_access_controller,
                )
            new_value = build_segment_with_type(variable.value_type, raw_value)
        draft_var_srv.update_variable(variable, name=new_name, value=new_value)
        db.session.commit()
        return variable

    @_api_prerequisite
    def delete(self, pipeline: Pipeline, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )
        variable = draft_var_srv.get_variable(variable_id=variable_id)
        if variable is None:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        if variable.app_id != pipeline.id:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        draft_var_srv.delete_variable(variable)
        db.session.commit()
        return Response("", 204)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/variables/<uuid:variable_id>/reset")
class RagPipelineVariableResetApi(Resource):
    @_api_prerequisite
    def put(self, pipeline: Pipeline, variable_id: str):
        draft_var_srv = WorkflowDraftVariableService(
            session=db.session(),
        )

        rag_pipeline_service = RagPipelineService()
        draft_workflow = rag_pipeline_service.get_draft_workflow(pipeline=pipeline)
        if draft_workflow is None:
            raise NotFoundError(
                f"Draft workflow not found, pipeline_id={pipeline.id}",
            )
        variable = draft_var_srv.get_variable(variable_id=variable_id)
        if variable is None:
            raise NotFoundError(description=f"variable not found, id={variable_id}")
        if variable.app_id != pipeline.id:
            raise NotFoundError(description=f"variable not found, id={variable_id}")

        resetted = draft_var_srv.reset_variable(draft_workflow, variable)
        db.session.commit()
        if resetted is None:
            return Response("", 204)
        else:
            return marshal(resetted, _WORKFLOW_DRAFT_VARIABLE_FIELDS)


def _get_variable_list(pipeline: Pipeline, node_id) -> WorkflowDraftVariableList:
    with sessionmaker(bind=db.engine, expire_on_commit=False).begin() as session:
        draft_var_srv = WorkflowDraftVariableService(
            session=session,
        )
        if node_id == CONVERSATION_VARIABLE_NODE_ID:
            draft_vars = draft_var_srv.list_conversation_variables(pipeline.id, user_id=current_user.id)
        elif node_id == SYSTEM_VARIABLE_NODE_ID:
            draft_vars = draft_var_srv.list_system_variables(pipeline.id, user_id=current_user.id)
        else:
            draft_vars = draft_var_srv.list_node_variables(app_id=pipeline.id, node_id=node_id, user_id=current_user.id)
    return draft_vars


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/system-variables")
class RagPipelineSystemVariableCollectionApi(Resource):
    @_api_prerequisite
    @marshal_with(workflow_draft_variable_list_model)
    def get(self, pipeline: Pipeline):
        return _get_variable_list(pipeline, SYSTEM_VARIABLE_NODE_ID)


@console_ns.route("/rag/pipelines/<uuid:pipeline_id>/workflows/draft/environment-variables")
class RagPipelineEnvironmentVariableCollectionApi(Resource):
    @_api_prerequisite
    def get(self, pipeline: Pipeline):
        """
        Get draft workflow
        """
        # fetch draft workflow by app_model
        rag_pipeline_service = RagPipelineService()
        workflow = rag_pipeline_service.get_draft_workflow(pipeline=pipeline)
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
                    "value_type": v.value_type.value,
                    "value": v.value,
                    # Do not track edited for env vars.
                    "edited": False,
                    "visible": True,
                    "editable": True,
                }
            )

        return {"items": env_vars_list}

```

### api/controllers/console/datasets/rag_pipeline/rag_pipeline.py
```py
import logging

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import (
    account_initialization_required,
    enterprise_license_required,
    knowledge_pipeline_publish_enabled,
    setup_required,
)
from extensions.ext_database import db
from libs.login import login_required
from models.dataset import PipelineCustomizedTemplate
from services.entities.knowledge_entities.rag_pipeline_entities import PipelineTemplateInfoEntity
from services.rag_pipeline.rag_pipeline import RagPipelineService

logger = logging.getLogger(__name__)


@console_ns.route("/rag/pipeline/templates")
class PipelineTemplateListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self):
        type = request.args.get("type", default="built-in", type=str)
        language = request.args.get("language", default="en-US", type=str)
        # get pipeline templates
        pipeline_templates = RagPipelineService.get_pipeline_templates(type, language)
        return pipeline_templates, 200


@console_ns.route("/rag/pipeline/templates/<string:template_id>")
class PipelineTemplateDetailApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self, template_id: str):
        type = request.args.get("type", default="built-in", type=str)
        rag_pipeline_service = RagPipelineService()
        pipeline_template = rag_pipeline_service.get_pipeline_template_detail(template_id, type)
        if pipeline_template is None:
            return {"error": "Pipeline template not found from upstream service."}, 404
        return pipeline_template, 200


class Payload(BaseModel):
    name: str = Field(..., min_length=1, max_length=40)
    description: str = Field(default="", max_length=400)
    icon_info: dict[str, object] | None = None


register_schema_models(console_ns, Payload)


@console_ns.route("/rag/pipeline/customized/templates/<string:template_id>")
class CustomizedPipelineTemplateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def patch(self, template_id: str):
        payload = Payload.model_validate(console_ns.payload or {})
        pipeline_template_info = PipelineTemplateInfoEntity.model_validate(payload.model_dump())
        RagPipelineService.update_customized_pipeline_template(template_id, pipeline_template_info)
        return 200

    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def delete(self, template_id: str):
        RagPipelineService.delete_customized_pipeline_template(template_id)
        return 200

    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def post(self, template_id: str):
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            template = session.scalar(
                select(PipelineCustomizedTemplate).where(PipelineCustomizedTemplate.id == template_id).limit(1)
            )
            if not template:
                raise ValueError("Customized pipeline template not found.")

        return {"data": template.yaml_content}, 200


@console_ns.route("/rag/pipelines/<string:pipeline_id>/customized/publish")
class PublishCustomizedPipelineTemplateApi(Resource):
    @console_ns.expect(console_ns.models[Payload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    @knowledge_pipeline_publish_enabled
    def post(self, pipeline_id: str):
        payload = Payload.model_validate(console_ns.payload or {})
        rag_pipeline_service = RagPipelineService()
        rag_pipeline_service.publish_customized_pipeline_template(pipeline_id, payload.model_dump())
        return {"result": "success"}

```

### api/controllers/console/datasets/rag_pipeline/rag_pipeline_import.py
```py
from flask import request
from flask_restx import Resource, fields, marshal_with  # type: ignore
from pydantic import BaseModel, Field
from sqlalchemy.orm import sessionmaker

from controllers.common.schema import get_or_create_model, register_schema_models
from controllers.console import console_ns
from controllers.console.datasets.wraps import get_rag_pipeline
from controllers.console.wraps import (
    account_initialization_required,
    edit_permission_required,
    setup_required,
)
from extensions.ext_database import db
from fields.rag_pipeline_fields import (
    leaked_dependency_fields,
    pipeline_import_check_dependencies_fields,
    pipeline_import_fields,
)
from libs.login import current_account_with_tenant, login_required
from models.dataset import Pipeline
from services.app_dsl_service import ImportStatus
from services.rag_pipeline.rag_pipeline_dsl_service import RagPipelineDslService


class RagPipelineImportPayload(BaseModel):
    mode: str
    yaml_content: str | None = None
    yaml_url: str | None = None
    name: str | None = None
    description: str | None = None
    icon_type: str | None = None
    icon: str | None = None
    icon_background: str | None = None
    pipeline_id: str | None = None


class IncludeSecretQuery(BaseModel):
    include_secret: str = Field(default="false")


register_schema_models(console_ns, RagPipelineImportPayload, IncludeSecretQuery)


pipeline_import_model = get_or_create_model("RagPipelineImport", pipeline_import_fields)

leaked_dependency_model = get_or_create_model("RagPipelineLeakedDependency", leaked_dependency_fields)
pipeline_import_check_dependencies_fields_copy = pipeline_import_check_dependencies_fields.copy()
pipeline_import_check_dependencies_fields_copy["leaked_dependencies"] = fields.List(
    fields.Nested(leaked_dependency_model)
)
pipeline_import_check_dependencies_model = get_or_create_model(
    "RagPipelineImportCheckDependencies", pipeline_import_check_dependencies_fields_copy
)


@console_ns.route("/rag/pipelines/imports")
class RagPipelineImportApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @marshal_with(pipeline_import_model)
    @console_ns.expect(console_ns.models[RagPipelineImportPayload.__name__])
    def post(self):
        # Check user role first
        current_user, _ = current_account_with_tenant()
        payload = RagPipelineImportPayload.model_validate(console_ns.payload or {})

        # Create service with session
        with sessionmaker(db.engine).begin() as session:
            import_service = RagPipelineDslService(session)
            # Import app
            account = current_user
            result = import_service.import_rag_pipeline(
                account=account,
                import_mode=payload.mode,
                yaml_content=payload.yaml_content,
                yaml_url=payload.yaml_url,
                pipeline_id=payload.pipeline_id,
                dataset_name=payload.name,
            )

        # Return appropriate status code based on result
        status = result.status
        if status == ImportStatus.FAILED:
            return result.model_dump(mode="json"), 400
        elif status == ImportStatus.PENDING:
            return result.model_dump(mode="json"), 202
        return result.model_dump(mode="json"), 200


@console_ns.route("/rag/pipelines/imports/<string:import_id>/confirm")
class RagPipelineImportConfirmApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    @marshal_with(pipeline_import_model)
    def post(self, import_id):
        current_user, _ = current_account_with_tenant()

        # Create service with session
        with sessionmaker(db.engine).begin() as session:
            import_service = RagPipelineDslService(session)
            # Confirm import
            account = current_user
            result = import_service.confirm_import(import_id=import_id, account=account)

        # Return appropriate status code based on result
        if result.status == ImportStatus.FAILED:
            return result.model_dump(mode="json"), 400
        return result.model_dump(mode="json"), 200


@console_ns.route("/rag/pipelines/imports/<string:pipeline_id>/check-dependencies")
class RagPipelineImportCheckDependenciesApi(Resource):
    @setup_required
    @login_required
    @get_rag_pipeline
    @account_initialization_required
    @edit_permission_required
    @marshal_with(pipeline_import_check_dependencies_model)
    def get(self, pipeline: Pipeline):
        with sessionmaker(db.engine).begin() as session:
            import_service = RagPipelineDslService(session)
            result = import_service.check_dependencies(pipeline=pipeline)

        return result.model_dump(mode="json"), 200


@console_ns.route("/rag/pipelines/<string:pipeline_id>/exports")
class RagPipelineExportApi(Resource):
    @setup_required
    @login_required
    @get_rag_pipeline
    @account_initialization_required
    @edit_permission_required
    def get(self, pipeline: Pipeline):
        # Add include_secret params
        query = IncludeSecretQuery.model_validate(request.args.to_dict())

        with sessionmaker(db.engine).begin() as session:
            export_service = RagPipelineDslService(session)
            result = export_service.export_rag_pipeline_dsl(
                pipeline=pipeline, include_secret=query.include_secret == "true"
            )

        return {"data": result}, 200

```

### api/controllers/console/datasets/data_source.py
```py
import json
from collections.abc import Generator
from typing import Any, Literal, cast

from flask import request
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import NotFound

from controllers.common.schema import get_or_create_model, register_schema_model
from core.datasource.entities.datasource_entities import DatasourceProviderType, OnlineDocumentPagesMessage
from core.datasource.online_document.online_document_plugin import OnlineDocumentDatasourcePlugin
from core.indexing_runner import IndexingRunner
from core.rag.extractor.entity.datasource_type import DatasourceType
from core.rag.extractor.entity.extract_setting import ExtractSetting, NotionInfo
from core.rag.extractor.notion_extractor import NotionExtractor
from extensions.ext_database import db
from fields.data_source_fields import (
    integrate_fields,
    integrate_icon_fields,
    integrate_list_fields,
    integrate_notion_info_list_fields,
    integrate_page_fields,
    integrate_workspace_fields,
)
from libs.datetime_utils import naive_utc_now
from libs.login import current_account_with_tenant, login_required
from models import DataSourceOauthBinding, Document
from services.dataset_service import DatasetService, DocumentService
from services.datasource_provider_service import DatasourceProviderService
from tasks.document_indexing_sync_task import document_indexing_sync_task

from .. import console_ns
from ..wraps import account_initialization_required, setup_required


class NotionEstimatePayload(BaseModel):
    notion_info_list: list[dict[str, Any]]
    process_rule: dict[str, Any]
    doc_form: str = Field(default="text_model")
    doc_language: str = Field(default="English")


class DataSourceNotionListQuery(BaseModel):
    dataset_id: str | None = Field(default=None, description="Dataset ID")
    credential_id: str = Field(..., description="Credential ID", min_length=1)
    datasource_parameters: dict[str, Any] | None = Field(default=None, description="Datasource parameters JSON string")


class DataSourceNotionPreviewQuery(BaseModel):
    credential_id: str = Field(..., description="Credential ID", min_length=1)


register_schema_model(console_ns, NotionEstimatePayload)


integrate_icon_model = get_or_create_model("DataSourceIntegrateIcon", integrate_icon_fields)

integrate_page_fields_copy = integrate_page_fields.copy()
integrate_page_fields_copy["page_icon"] = fields.Nested(integrate_icon_model, allow_null=True)
integrate_page_model = get_or_create_model("DataSourceIntegratePage", integrate_page_fields_copy)

integrate_workspace_fields_copy = integrate_workspace_fields.copy()
integrate_workspace_fields_copy["pages"] = fields.List(fields.Nested(integrate_page_model))
integrate_workspace_model = get_or_create_model("DataSourceIntegrateWorkspace", integrate_workspace_fields_copy)

integrate_fields_copy = integrate_fields.copy()
integrate_fields_copy["source_info"] = fields.Nested(integrate_workspace_model)
integrate_model = get_or_create_model("DataSourceIntegrate", integrate_fields_copy)

integrate_list_fields_copy = integrate_list_fields.copy()
integrate_list_fields_copy["data"] = fields.List(fields.Nested(integrate_model))
integrate_list_model = get_or_create_model("DataSourceIntegrateList", integrate_list_fields_copy)

notion_page_fields = {
    "page_name": fields.String,
    "page_id": fields.String,
    "page_icon": fields.Nested(integrate_icon_model, allow_null=True),
    "is_bound": fields.Boolean,
    "parent_id": fields.String,
    "type": fields.String,
}
notion_page_model = get_or_create_model("NotionIntegratePage", notion_page_fields)

notion_workspace_fields = {
    "workspace_name": fields.String,
    "workspace_id": fields.String,
    "workspace_icon": fields.String,
    "pages": fields.List(fields.Nested(notion_page_model)),
}
notion_workspace_model = get_or_create_model("NotionIntegrateWorkspace", notion_workspace_fields)

integrate_notion_info_list_fields_copy = integrate_notion_info_list_fields.copy()
integrate_notion_info_list_fields_copy["notion_info"] = fields.List(fields.Nested(notion_workspace_model))
integrate_notion_info_list_model = get_or_create_model(
    "NotionIntegrateInfoList", integrate_notion_info_list_fields_copy
)


@console_ns.route(
    "/data-source/integrates",
    "/data-source/integrates/<uuid:binding_id>/<string:action>",
)
class DataSourceApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(integrate_list_model)
    def get(self):
        _, current_tenant_id = current_account_with_tenant()

        # get workspace data source integrates
        data_source_integrates = db.session.scalars(
            select(DataSourceOauthBinding).where(
                DataSourceOauthBinding.tenant_id == current_tenant_id,
                DataSourceOauthBinding.disabled == False,
            )
        ).all()

        base_url = request.url_root.rstrip("/")
        data_source_oauth_base_path = "/console/api/oauth/data-source"
        providers = ["notion"]

        integrate_data = []
        for provider in providers:
            # existing_integrate = next((ai for ai in data_source_integrates if ai.provider == provider), None)
            existing_integrates = filter(lambda item: item.provider == provider, data_source_integrates)
            if existing_integrates:
                for existing_integrate in list(existing_integrates):
                    integrate_data.append(
                        {
                            "id": existing_integrate.id,
                            "provider": provider,
                            "created_at": existing_integrate.created_at,
                            "is_bound": True,
                            "disabled": existing_integrate.disabled,
                            "source_info": existing_integrate.source_info,
                            "link": f"{base_url}{data_source_oauth_base_path}/{provider}",
                        }
                    )
            else:
                integrate_data.append(
                    {
                        "id": None,
                        "provider": provider,
                        "created_at": None,
                        "source_info": None,
                        "is_bound": False,
                        "disabled": None,
                        "link": f"{base_url}{data_source_oauth_base_path}/{provider}",
                    }
                )
        return {"data": integrate_data}, 200

    @setup_required
    @login_required
    @account_initialization_required
    def patch(self, binding_id, action: Literal["enable", "disable"]):
        _, current_tenant_id = current_account_with_tenant()
        binding_id = str(binding_id)
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            data_source_binding = session.execute(
                select(DataSourceOauthBinding).filter_by(id=binding_id, tenant_id=current_tenant_id)
            ).scalar_one_or_none()
        if data_source_binding is None:
            raise NotFound("Data source binding not found.")
        # enable binding
        match action:
            case "enable":
                if data_source_binding.disabled:
                    data_source_binding.disabled = False
                    data_source_binding.updated_at = naive_utc_now()
                    db.session.add(data_source_binding)
                    db.session.commit()
                else:
                    raise ValueError("Data source is not disabled.")
            # disable binding
            case "disable":
                if not data_source_binding.disabled:
                    data_source_binding.disabled = True
                    data_source_binding.updated_at = naive_utc_now()
                    db.session.add(data_source_binding)
                    db.session.commit()
                else:
                    raise ValueError("Data source is disabled.")
        return {"result": "success"}, 200


@console_ns.route("/notion/pre-import/pages")
class DataSourceNotionListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(integrate_notion_info_list_model)
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()

        query = DataSourceNotionListQuery.model_validate(request.args.to_dict())

        # Get datasource_parameters from query string (optional, for GitHub and other datasources)
        datasource_parameters = query.datasource_parameters or {}

        datasource_provider_service = DatasourceProviderService()
        credential = datasource_provider_service.get_datasource_credentials(
            tenant_id=current_tenant_id,
            credential_id=query.credential_id,
            provider="notion_datasource",
            plugin_id="langgenius/notion_datasource",
        )
        if not credential:
            raise NotFound("Credential not found.")
        exist_page_ids = []
        with sessionmaker(db.engine).begin() as session:
            # import notion in the exist dataset
            if query.dataset_id:
                dataset = DatasetService.get_dataset(query.dataset_id)
                if not dataset:
                    raise NotFound("Dataset not found.")
                if dataset.data_source_type != "notion_import":
                    raise ValueError("Dataset is not notion type.")

                documents = session.scalars(
                    select(Document).filter_by(
                        dataset_id=query.dataset_id,
                        tenant_id=current_tenant_id,
                        data_source_type="notion_import",
                        enabled=True,
                    )
                ).all()
                if documents:
                    for document in documents:
                        data_source_info = json.loads(document.data_source_info)
                        exist_page_ids.append(data_source_info["notion_page_id"])
            # get all authorized pages
            from core.datasource.datasource_manager import DatasourceManager

            datasource_runtime = DatasourceManager.get_datasource_runtime(
                provider_id="langgenius/notion_datasource/notion_datasource",
                datasource_name="notion_datasource",
                tenant_id=current_tenant_id,
                datasource_type=DatasourceProviderType.ONLINE_DOCUMENT,
            )
            datasource_provider_service = DatasourceProviderService()
            if credential:
                datasource_runtime.runtime.credentials = credential
            datasource_runtime = cast(OnlineDocumentDatasourcePlugin, datasource_runtime)
            online_document_result: Generator[OnlineDocumentPagesMessage, None, None] = (
                datasource_runtime.get_online_document_pages(
                    user_id=current_user.id,
                    datasource_parameters=datasource_parameters,
                    provider_type=datasource_runtime.datasource_provider_type(),
                )
            )
            try:
                pages = []
                workspace_info = {}
                for message in online_document_result:
                    result = message.result
                    for info in result:
                        workspace_info = {
                            "workspace_id": info.workspace_id,
                            "workspace_name": info.workspace_name,
                            "workspace_icon": info.workspace_icon,
                        }
                        for page in info.pages:
                            page_info = {
                                "page_id": page.page_id,
                                "page_name": page.page_name,
                                "type": page.type,
                                "parent_id": page.parent_id,
                                "is_bound": page.page_id in exist_page_ids,
                                "page_icon": page.page_icon,
                            }
                            pages.append(page_info)
            except Exception as e:
                raise e
            return {"notion_info": {**workspace_info, "pages": pages}}, 200


@console_ns.route(
    "/notion/pages/<uuid:page_id>/<string:page_type>/preview",
    "/datasets/notion-indexing-estimate",
)
class DataSourceNotionApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, page_id, page_type):
        _, current_tenant_id = current_account_with_tenant()

        query = DataSourceNotionPreviewQuery.model_validate(request.args.to_dict())

        datasource_provider_service = DatasourceProviderService()
        credential = datasource_provider_service.get_datasource_credentials(
            tenant_id=current_tenant_id,
            credential_id=query.credential_id,
            provider="notion_datasource",
            plugin_id="langgenius/notion_datasource",
        )

        page_id = str(page_id)

        extractor = NotionExtractor(
            notion_workspace_id="",
            notion_obj_id=page_id,
            notion_page_type=page_type,
            notion_access_token=credential.get("integration_secret"),
            tenant_id=current_tenant_id,
        )

        text_docs = extractor.extract()
        return {"content": "\n".join([doc.page_content for doc in text_docs])}, 200

    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.expect(console_ns.models[NotionEstimatePayload.__name__])
    def post(self):
        _, current_tenant_id = current_account_with_tenant()

        payload = NotionEstimatePayload.model_validate(console_ns.payload or {})
        args = payload.model_dump()
        # validate args
        DocumentService.estimate_args_validate(args)
        notion_info_list = payload.notion_info_list
        extract_settings = []
        for notion_info in notion_info_list:
            workspace_id = notion_info["workspace_id"]
            credential_id = notion_info.get("credential_id")
            for page in notion_info["pages"]:
                extract_setting = ExtractSetting(
                    datasource_type=DatasourceType.NOTION,
                    notion_info=NotionInfo.model_validate(
                        {
                            "credential_id": credential_id,
                            "notion_workspace_id": workspace_id,
                            "notion_obj_id": page["page_id"],
                            "notion_page_type": page["type"],
                            "tenant_id": current_tenant_id,
                        }
                    ),
                    document_model=args["doc_form"],
                )
                extract_settings.append(extract_setting)
        indexing_runner = IndexingRunner()
        response = indexing_runner.indexing_estimate(
            current_tenant_id,
            extract_settings,
            args["process_rule"],
            args["doc_form"],
            args["doc_language"],
        )
        return response.model_dump(), 200


@console_ns.route("/datasets/<uuid:dataset_id>/notion/sync")
class DataSourceNotionDatasetSyncApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        documents = DocumentService.get_document_by_dataset_id(dataset_id_str)
        for document in documents:
            document_indexing_sync_task.delay(dataset_id_str, document.id)
        return {"result": "success"}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/notion/sync")
class DataSourceNotionDocumentSyncApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id, document_id):
        dataset_id_str = str(dataset_id)
        document_id_str = str(document_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        document = DocumentService.get_document(dataset_id_str, document_id_str)
        if document is None:
            raise NotFound("Document not found.")
        document_indexing_sync_task.delay(dataset_id_str, document_id_str)
        return {"result": "success"}, 200

```

### api/controllers/console/datasets/metadata.py
```py
from typing import Literal

from flask_restx import Resource, marshal_with
from pydantic import BaseModel
from werkzeug.exceptions import NotFound

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, enterprise_license_required, setup_required
from fields.dataset_fields import dataset_metadata_fields
from libs.login import current_account_with_tenant, login_required
from services.dataset_service import DatasetService
from services.entities.knowledge_entities.knowledge_entities import (
    DocumentMetadataOperation,
    MetadataArgs,
    MetadataDetail,
    MetadataOperationData,
)
from services.metadata_service import MetadataService


class MetadataUpdatePayload(BaseModel):
    name: str


register_schema_models(
    console_ns, MetadataArgs, MetadataOperationData, MetadataUpdatePayload, DocumentMetadataOperation, MetadataDetail
)


@console_ns.route("/datasets/<uuid:dataset_id>/metadata")
class DatasetMetadataCreateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    @marshal_with(dataset_metadata_fields)
    @console_ns.expect(console_ns.models[MetadataArgs.__name__])
    def post(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        metadata_args = MetadataArgs.model_validate(console_ns.payload or {})

        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        DatasetService.check_dataset_permission(dataset, current_user)

        metadata = MetadataService.create_metadata(dataset_id_str, metadata_args)
        return metadata, 201

    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self, dataset_id):
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        return MetadataService.get_dataset_metadatas(dataset), 200


@console_ns.route("/datasets/<uuid:dataset_id>/metadata/<uuid:metadata_id>")
class DatasetMetadataApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    @marshal_with(dataset_metadata_fields)
    @console_ns.expect(console_ns.models[MetadataUpdatePayload.__name__])
    def patch(self, dataset_id, metadata_id):
        current_user, _ = current_account_with_tenant()
        payload = MetadataUpdatePayload.model_validate(console_ns.payload or {})
        name = payload.name

        dataset_id_str = str(dataset_id)
        metadata_id_str = str(metadata_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        DatasetService.check_dataset_permission(dataset, current_user)

        metadata = MetadataService.update_metadata_name(dataset_id_str, metadata_id_str, name)
        return metadata, 200

    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def delete(self, dataset_id, metadata_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        metadata_id_str = str(metadata_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        DatasetService.check_dataset_permission(dataset, current_user)

        MetadataService.delete_metadata(dataset_id_str, metadata_id_str)
        return {"result": "success"}, 204


@console_ns.route("/datasets/metadata/built-in")
class DatasetMetadataBuiltInFieldApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self):
        built_in_fields = MetadataService.get_built_in_fields()
        return {"fields": built_in_fields}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/metadata/built-in/<string:action>")
class DatasetMetadataBuiltInFieldActionApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def post(self, dataset_id, action: Literal["enable", "disable"]):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        DatasetService.check_dataset_permission(dataset, current_user)

        match action:
            case "enable":
                MetadataService.enable_built_in_field(dataset)
            case "disable":
                MetadataService.disable_built_in_field(dataset)
        return {"result": "success"}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/documents/metadata")
class DocumentMetadataEditApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    @console_ns.expect(console_ns.models[MetadataOperationData.__name__])
    def post(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        DatasetService.check_dataset_permission(dataset, current_user)

        metadata_args = MetadataOperationData.model_validate(console_ns.payload or {})

        MetadataService.update_documents_metadata(dataset, metadata_args)

        return {"result": "success"}, 200

```

### api/controllers/console/datasets/error.py
```py
from libs.exception import BaseHTTPException


class DatasetNotInitializedError(BaseHTTPException):
    error_code = "dataset_not_initialized"
    description = "The dataset is still being initialized or indexing. Please wait a moment."
    code = 400


class ArchivedDocumentImmutableError(BaseHTTPException):
    error_code = "archived_document_immutable"
    description = "The archived document is not editable."
    code = 403


class DatasetNameDuplicateError(BaseHTTPException):
    error_code = "dataset_name_duplicate"
    description = "The dataset name already exists. Please modify your dataset name."
    code = 409


class InvalidActionError(BaseHTTPException):
    error_code = "invalid_action"
    description = "Invalid action."
    code = 400


class DocumentAlreadyFinishedError(BaseHTTPException):
    error_code = "document_already_finished"
    description = "The document has been processed. Please refresh the page or go to the document details."
    code = 400


class DocumentIndexingError(BaseHTTPException):
    error_code = "document_indexing"
    description = "The document is being processed and cannot be edited."
    code = 400


class InvalidMetadataError(BaseHTTPException):
    error_code = "invalid_metadata"
    description = "The metadata content is incorrect. Please check and verify."
    code = 400


class WebsiteCrawlError(BaseHTTPException):
    error_code = "crawl_failed"
    description = "{message}"
    code = 500


class DatasetInUseError(BaseHTTPException):
    error_code = "dataset_in_use"
    description = "The dataset is being used by some apps. Please remove the dataset from the apps before deleting it."
    code = 409


class IndexingEstimateError(BaseHTTPException):
    error_code = "indexing_estimate_error"
    description = "Knowledge indexing estimate failed: {message}"
    code = 500


class ChildChunkIndexingError(BaseHTTPException):
    error_code = "child_chunk_indexing_error"
    description = "Create child chunk index failed: {message}"
    code = 500


class ChildChunkDeleteIndexError(BaseHTTPException):
    error_code = "child_chunk_delete_index_error"
    description = "Delete child chunk index failed: {message}"
    code = 500


class PipelineNotFoundError(BaseHTTPException):
    error_code = "pipeline_not_found"
    description = "Pipeline not found."
    code = 404

```

### api/controllers/console/datasets/datasets.py
```py
from typing import Any, cast

from flask import request
from flask_restx import Resource, fields, marshal, marshal_with
from graphon.model_runtime.entities.model_entities import ModelType
from pydantic import BaseModel, Field, field_validator
from sqlalchemy import func, select
from werkzeug.exceptions import Forbidden, NotFound

import services
from configs import dify_config
from controllers.common.schema import get_or_create_model, register_schema_models
from controllers.console import console_ns
from controllers.console.apikey import (
    api_key_item_model,
    api_key_list_model,
)
from controllers.console.app.error import ProviderNotInitializeError
from controllers.console.datasets.error import DatasetInUseError, DatasetNameDuplicateError, IndexingEstimateError
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_rate_limit_check,
    enterprise_license_required,
    is_admin_or_owner_required,
    setup_required,
)
from core.errors.error import LLMBadRequestError, ProviderTokenNotInitError
from core.indexing_runner import IndexingRunner
from core.plugin.impl.model_runtime_factory import create_plugin_provider_manager
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.extractor.entity.datasource_type import DatasourceType
from core.rag.extractor.entity.extract_setting import ExtractSetting, NotionInfo, WebsiteInfo
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from extensions.ext_database import db
from fields.app_fields import app_detail_kernel_fields, related_app_list
from fields.dataset_fields import (
    content_fields,
    dataset_detail_fields,
    dataset_fields,
    dataset_query_detail_fields,
    dataset_retrieval_model_fields,
    doc_metadata_fields,
    external_knowledge_info_fields,
    external_retrieval_model_fields,
    file_info_fields,
    icon_info_fields,
    keyword_setting_fields,
    reranking_model_fields,
    tag_fields,
    vector_setting_fields,
    weighted_score_fields,
)
from fields.document_fields import document_status_fields
from libs.login import current_account_with_tenant, login_required
from models import ApiToken, Dataset, Document, DocumentSegment, UploadFile
from models.dataset import DatasetPermission, DatasetPermissionEnum
from models.enums import ApiTokenType, SegmentStatus
from models.provider_ids import ModelProviderID
from services.api_token_service import ApiTokenCache
from services.dataset_service import DatasetPermissionService, DatasetService, DocumentService

# Register models for flask_restx to avoid dict type issues in Swagger
dataset_base_model = get_or_create_model("DatasetBase", dataset_fields)

tag_model = get_or_create_model("Tag", tag_fields)

keyword_setting_model = get_or_create_model("DatasetKeywordSetting", keyword_setting_fields)
vector_setting_model = get_or_create_model("DatasetVectorSetting", vector_setting_fields)

weighted_score_fields_copy = weighted_score_fields.copy()
weighted_score_fields_copy["keyword_setting"] = fields.Nested(keyword_setting_model)
weighted_score_fields_copy["vector_setting"] = fields.Nested(vector_setting_model)
weighted_score_model = get_or_create_model("DatasetWeightedScore", weighted_score_fields_copy)

reranking_model = get_or_create_model("DatasetRerankingModel", reranking_model_fields)

dataset_retrieval_model_fields_copy = dataset_retrieval_model_fields.copy()
dataset_retrieval_model_fields_copy["reranking_model"] = fields.Nested(reranking_model)
dataset_retrieval_model_fields_copy["weights"] = fields.Nested(weighted_score_model, allow_null=True)
dataset_retrieval_model = get_or_create_model("DatasetRetrievalModel", dataset_retrieval_model_fields_copy)

external_knowledge_info_model = get_or_create_model("ExternalKnowledgeInfo", external_knowledge_info_fields)

external_retrieval_model = get_or_create_model("ExternalRetrievalModel", external_retrieval_model_fields)

doc_metadata_model = get_or_create_model("DatasetDocMetadata", doc_metadata_fields)

icon_info_model = get_or_create_model("DatasetIconInfo", icon_info_fields)

dataset_detail_fields_copy = dataset_detail_fields.copy()
dataset_detail_fields_copy["retrieval_model_dict"] = fields.Nested(dataset_retrieval_model)
dataset_detail_fields_copy["tags"] = fields.List(fields.Nested(tag_model))
dataset_detail_fields_copy["external_knowledge_info"] = fields.Nested(external_knowledge_info_model)
dataset_detail_fields_copy["external_retrieval_model"] = fields.Nested(external_retrieval_model, allow_null=True)
dataset_detail_fields_copy["doc_metadata"] = fields.List(fields.Nested(doc_metadata_model))
dataset_detail_fields_copy["icon_info"] = fields.Nested(icon_info_model)
dataset_detail_model = get_or_create_model("DatasetDetail", dataset_detail_fields_copy)

file_info_model = get_or_create_model("DatasetFileInfo", file_info_fields)

content_fields_copy = content_fields.copy()
content_fields_copy["file_info"] = fields.Nested(file_info_model, allow_null=True)
content_model = get_or_create_model("DatasetContent", content_fields_copy)

dataset_query_detail_fields_copy = dataset_query_detail_fields.copy()
dataset_query_detail_fields_copy["queries"] = fields.Nested(content_model)
dataset_query_detail_model = get_or_create_model("DatasetQueryDetail", dataset_query_detail_fields_copy)

app_detail_kernel_model = get_or_create_model("AppDetailKernel", app_detail_kernel_fields)
related_app_list_copy = related_app_list.copy()
related_app_list_copy["data"] = fields.List(fields.Nested(app_detail_kernel_model))
related_app_list_model = get_or_create_model("RelatedAppList", related_app_list_copy)


def _validate_indexing_technique(value: str | None) -> str | None:
    if value is None:
        return value
    if value not in Dataset.INDEXING_TECHNIQUE_LIST:
        raise ValueError("Invalid indexing technique.")
    return value


def _validate_doc_form(value: str | None) -> str | None:
    if value is None:
        return value
    if value not in Dataset.DOC_FORM_LIST:
        raise ValueError("Invalid doc_form.")
    return value


class DatasetCreatePayload(BaseModel):
    name: str = Field(..., min_length=1, max_length=40)
    description: str = Field("", max_length=400)
    indexing_technique: str | None = None
    permission: DatasetPermissionEnum | None = DatasetPermissionEnum.ONLY_ME
    provider: str = "vendor"
    external_knowledge_api_id: str | None = None
    external_knowledge_id: str | None = None

    @field_validator("indexing_technique")
    @classmethod
    def validate_indexing(cls, value: str | None) -> str | None:
        return _validate_indexing_technique(value)

    @field_validator("provider")
    @classmethod
    def validate_provider(cls, value: str) -> str:
        if value not in Dataset.PROVIDER_LIST:
            raise ValueError("Invalid provider.")
        return value


class DatasetUpdatePayload(BaseModel):
    name: str | None = Field(None, min_length=1, max_length=40)
    description: str | None = Field(None, max_length=400)
    permission: DatasetPermissionEnum | None = None
    indexing_technique: str | None = None
    embedding_model: str | None = None
    embedding_model_provider: str | None = None
    retrieval_model: dict[str, Any] | None = None
    summary_index_setting: dict[str, Any] | None = None
    partial_member_list: list[dict[str, str]] | None = None
    external_retrieval_model: dict[str, Any] | None = None
    external_knowledge_id: str | None = None
    external_knowledge_api_id: str | None = None
    icon_info: dict[str, Any] | None = None
    is_multimodal: bool | None = False

    @field_validator("indexing_technique")
    @classmethod
    def validate_indexing(cls, value: str | None) -> str | None:
        return _validate_indexing_technique(value)


class IndexingEstimatePayload(BaseModel):
    info_list: dict[str, Any]
    process_rule: dict[str, Any]
    indexing_technique: str
    doc_form: str = "text_model"
    dataset_id: str | None = None
    doc_language: str = "English"

    @field_validator("indexing_technique")
    @classmethod
    def validate_indexing(cls, value: str) -> str:
        result = _validate_indexing_technique(value)
        if result is None:
            raise ValueError("indexing_technique is required.")
        return result

    @field_validator("doc_form")
    @classmethod
    def validate_doc_form(cls, value: str) -> str:
        result = _validate_doc_form(value)
        if result is None:
            return "text_model"
        return result


class ConsoleDatasetListQuery(BaseModel):
    page: int = Field(default=1, description="Page number")
    limit: int = Field(default=20, description="Number of items per page")
    keyword: str | None = Field(default=None, description="Search keyword")
    include_all: bool = Field(default=False, description="Include all datasets")
    ids: list[str] = Field(default_factory=list, description="Filter by dataset IDs")
    tag_ids: list[str] = Field(default_factory=list, description="Filter by tag IDs")


register_schema_models(
    console_ns, DatasetCreatePayload, DatasetUpdatePayload, IndexingEstimatePayload, ConsoleDatasetListQuery
)


def _get_retrieval_methods_by_vector_type(vector_type: str | None, is_mock: bool = False) -> dict[str, list[str]]:
    """
    Get supported retrieval methods based on vector database type.

    Args:
        vector_type: Vector database type, can be None
        is_mock: Whether this is a Mock API, affects MILVUS handling

    Returns:
        Dictionary containing supported retrieval methods

    Raises:
        ValueError: If vector_type is None or unsupported
    """
    if vector_type is None:
        raise ValueError("Vector store type is not configured.")

    # Define vector database types that only support semantic search
    semantic_only_types = {
        VectorType.RELYT,
        VectorType.TIDB_VECTOR,
        VectorType.CHROMA,
        VectorType.PGVECTO_RS,
        VectorType.VIKINGDB,
        VectorType.UPSTASH,
    }

    # Define vector database types that support all retrieval methods
    full_search_types = {
        VectorType.QDRANT,
        VectorType.WEAVIATE,
        VectorType.OPENSEARCH,
        VectorType.ANALYTICDB,
        VectorType.MYSCALE,
        VectorType.ORACLE,
        VectorType.ELASTICSEARCH,
        VectorType.ELASTICSEARCH_JA,
        VectorType.PGVECTOR,
        VectorType.VASTBASE,
        VectorType.TIDB_ON_QDRANT,
        VectorType.LINDORM,
        VectorType.COUCHBASE,
        VectorType.OPENGAUSS,
        VectorType.OCEANBASE,
        VectorType.SEEKDB,
        VectorType.TABLESTORE,
        VectorType.HUAWEI_CLOUD,
        VectorType.TENCENT,
        VectorType.MATRIXONE,
        VectorType.CLICKZETTA,
        VectorType.BAIDU,
        VectorType.ALIBABACLOUD_MYSQL,
        VectorType.IRIS,
        VectorType.HOLOGRES,
    }

    semantic_methods = {"retrieval_method": [RetrievalMethod.SEMANTIC_SEARCH.value]}
    full_methods = {
        "retrieval_method": [
            RetrievalMethod.SEMANTIC_SEARCH.value,
            RetrievalMethod.FULL_TEXT_SEARCH.value,
            RetrievalMethod.HYBRID_SEARCH.value,
        ]
    }

    if vector_type == VectorType.MILVUS:
        return semantic_methods if is_mock else full_methods

    if vector_type in semantic_only_types:
        return semantic_methods
    elif vector_type in full_search_types:
        return full_methods
    else:
        raise ValueError(f"Unsupported vector db type {vector_type}.")


@console_ns.route("/datasets")
class DatasetListApi(Resource):
    @console_ns.doc("get_datasets")
    @console_ns.doc(description="Get list of datasets")
    @console_ns.doc(
        params={
            "page": "Page number (default: 1)",
            "limit": "Number of items per page (default: 20)",
            "ids": "Filter by dataset IDs (list)",
            "keyword": "Search keyword",
            "tag_ids": "Filter by tag IDs (list)",
            "include_all": "Include all datasets (default: false)",
        }
    )
    @console_ns.response(200, "Datasets retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        # Convert query parameters to dict, handling list parameters correctly
        query_params: dict[str, str | list[str]] = dict(request.args.to_dict())
        # Handle ids and tag_ids as lists (Flask request.args.getlist returns list even for single value)
        if "ids" in request.args:
            query_params["ids"] = request.args.getlist("ids")
        if "tag_ids" in request.args:
            query_params["tag_ids"] = request.args.getlist("tag_ids")
        query = ConsoleDatasetListQuery.model_validate(query_params)
        # provider = request.args.get("provider", default="vendor")
        if query.ids:
            datasets, total = DatasetService.get_datasets_by_ids(query.ids, current_tenant_id)
        else:
            datasets, total = DatasetService.get_datasets(
                query.page,
                query.limit,
                current_tenant_id,
                current_user,
                query.keyword,
                query.tag_ids,
                query.include_all,
            )

        # check embedding setting
        provider_manager = create_plugin_provider_manager(tenant_id=current_tenant_id)
        configurations = provider_manager.get_configurations(tenant_id=current_tenant_id)

        embedding_models = configurations.get_models(model_type=ModelType.TEXT_EMBEDDING, only_active=True)

        model_names = []
        for embedding_model in embedding_models:
            model_names.append(f"{embedding_model.model}:{embedding_model.provider.provider}")

        data = cast(list[dict[str, Any]], marshal(datasets, dataset_detail_fields))
        dataset_ids = [item["id"] for item in data if item.get("permission") == "partial_members"]
        partial_members_map: dict[str, list[str]] = {}
        if dataset_ids:
            permissions = db.session.execute(
                select(DatasetPermission.dataset_id, DatasetPermission.account_id).where(
                    DatasetPermission.dataset_id.in_(dataset_ids)
                )
            ).all()

            for dataset_id, account_id in permissions:
                partial_members_map.setdefault(dataset_id, []).append(account_id)

        for item in data:
            # convert embedding_model_provider to plugin standard format
            if item["indexing_technique"] == IndexTechniqueType.HIGH_QUALITY and item["embedding_model_provider"]:
                item["embedding_model_provider"] = str(ModelProviderID(item["embedding_model_provider"]))
                item_model = f"{item['embedding_model']}:{item['embedding_model_provider']}"
                if item_model in model_names:
                    item["embedding_available"] = True
                else:
                    item["embedding_available"] = False
            else:
                item["embedding_available"] = True

            if item.get("permission") == "partial_members":
                item.update({"partial_member_list": partial_members_map.get(item["id"], [])})
            else:
                item.update({"partial_member_list": []})

        response = {
            "data": data,
            "has_more": len(datasets) == query.limit,
            "limit": query.limit,
            "total": total,
            "page": query.page,
        }
        return response, 200

    @console_ns.doc("create_dataset")
    @console_ns.doc(description="Create a new dataset")
    @console_ns.expect(console_ns.models[DatasetCreatePayload.__name__])
    @console_ns.response(201, "Dataset created successfully")
    @console_ns.response(400, "Invalid request parameters")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def post(self):
        payload = DatasetCreatePayload.model_validate(console_ns.payload or {})
        current_user, current_tenant_id = current_account_with_tenant()

        # The role of the current user in the ta table must be admin, owner, or editor, or dataset_operator
        if not current_user.is_dataset_editor:
            raise Forbidden()

        try:
            dataset = DatasetService.create_empty_dataset(
                tenant_id=current_tenant_id,
                name=payload.name,
                description=payload.description,
                indexing_technique=payload.indexing_technique,
                account=current_user,
                permission=payload.permission or DatasetPermissionEnum.ONLY_ME,
                provider=payload.provider,
                external_knowledge_api_id=payload.external_knowledge_api_id,
                external_knowledge_id=payload.external_knowledge_id,
            )
        except services.errors.dataset.DatasetNameDuplicateError:
            raise DatasetNameDuplicateError()

        return marshal(dataset, dataset_detail_fields), 201


@console_ns.route("/datasets/<uuid:dataset_id>")
class DatasetApi(Resource):
    @console_ns.doc("get_dataset")
    @console_ns.doc(description="Get dataset details")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Dataset retrieved successfully", dataset_detail_model)
    @console_ns.response(404, "Dataset not found")
    @console_ns.response(403, "Permission denied")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        current_user, current_tenant_id = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        data = cast(dict[str, Any], marshal(dataset, dataset_detail_fields))
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            if dataset.embedding_model_provider:
                provider_id = ModelProviderID(dataset.embedding_model_provider)
                data["embedding_model_provider"] = str(provider_id)
        if data.get("permission") == "partial_members":
            part_users_list = DatasetPermissionService.get_dataset_partial_member_list(dataset_id_str)
            data.update({"partial_member_list": part_users_list})

        # check embedding setting
        provider_manager = create_plugin_provider_manager(tenant_id=current_tenant_id)
        configurations = provider_manager.get_configurations(tenant_id=current_tenant_id)

        embedding_models = configurations.get_models(model_type=ModelType.TEXT_EMBEDDING, only_active=True)

        model_names = []
        for embedding_model in embedding_models:
            model_names.append(f"{embedding_model.model}:{embedding_model.provider.provider}")

        if data["indexing_technique"] == IndexTechniqueType.HIGH_QUALITY:
            item_model = f"{data['embedding_model']}:{data['embedding_model_provider']}"
            if item_model in model_names:
                data["embedding_available"] = True
            else:
                data["embedding_available"] = False
        else:
            data["embedding_available"] = True

        return data, 200

    @console_ns.doc("update_dataset")
    @console_ns.doc(description="Update dataset details")
    @console_ns.expect(console_ns.models[DatasetUpdatePayload.__name__])
    @console_ns.response(200, "Dataset updated successfully", dataset_detail_model)
    @console_ns.response(404, "Dataset not found")
    @console_ns.response(403, "Permission denied")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def patch(self, dataset_id):
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        payload = DatasetUpdatePayload.model_validate(console_ns.payload or {})
        current_user, current_tenant_id = current_account_with_tenant()
        # check embedding model setting
        if (
            payload.indexing_technique == IndexTechniqueType.HIGH_QUALITY
            and payload.embedding_model_provider is not None
            and payload.embedding_model is not None
        ):
            is_multimodal = DatasetService.check_is_multimodal_model(
                dataset.tenant_id, payload.embedding_model_provider, payload.embedding_model
            )
            payload.is_multimodal = is_multimodal
        payload_data = payload.model_dump(exclude_unset=True)
        # The role of the current user in the ta table must be admin, owner, editor, or dataset_operator
        DatasetPermissionService.check_permission(
            current_user, dataset, payload.permission, payload.partial_member_list
        )

        dataset = DatasetService.update_dataset(dataset_id_str, payload_data, current_user)

        if dataset is None:
            raise NotFound("Dataset not found.")

        result_data = cast(dict[str, Any], marshal(dataset, dataset_detail_fields))
        tenant_id = current_tenant_id

        if payload.partial_member_list is not None and payload.permission == DatasetPermissionEnum.PARTIAL_TEAM:
            DatasetPermissionService.update_partial_member_list(tenant_id, dataset_id_str, payload.partial_member_list)
        # clear partial member list when permission is only_me or all_team_members
        elif payload.permission in {DatasetPermissionEnum.ONLY_ME, DatasetPermissionEnum.ALL_TEAM}:
            DatasetPermissionService.clear_partial_member_list(dataset_id_str)

        partial_member_list = DatasetPermissionService.get_dataset_partial_member_list(dataset_id_str)
        result_data.update({"partial_member_list": partial_member_list})

        return result_data, 200

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def delete(self, dataset_id):
        dataset_id_str = str(dataset_id)
        current_user, _ = current_account_with_tenant()

        if not (current_user.has_edit_permission or current_user.is_dataset_operator):
            raise Forbidden()

        try:
            if DatasetService.delete_dataset(dataset_id_str, current_user):
                DatasetPermissionService.clear_partial_member_list(dataset_id_str)
                return {"result": "success"}, 204
            else:
                raise NotFound("Dataset not found.")
        except services.errors.dataset.DatasetInUseError:
            raise DatasetInUseError()


@console_ns.route("/datasets/<uuid:dataset_id>/use-check")
class DatasetUseCheckApi(Resource):
    @console_ns.doc("check_dataset_use")
    @console_ns.doc(description="Check if dataset is in use")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Dataset use status retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        dataset_id_str = str(dataset_id)

        dataset_is_using = DatasetService.dataset_use_check(dataset_id_str)
        return {"is_using": dataset_is_using}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/queries")
class DatasetQueryApi(Resource):
    @console_ns.doc("get_dataset_queries")
    @console_ns.doc(description="Get dataset query history")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Query history retrieved successfully", dataset_query_detail_model)
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        page = request.args.get("page", default=1, type=int)
        limit = request.args.get("limit", default=20, type=int)

        dataset_queries, total = DatasetService.get_dataset_queries(dataset_id=dataset.id, page=page, per_page=limit)

        response = {
            "data": marshal(dataset_queries, dataset_query_detail_model),
            "has_more": len(dataset_queries) == limit,
            "limit": limit,
            "total": total,
            "page": page,
        }
        return response, 200


@console_ns.route("/datasets/indexing-estimate")
class DatasetIndexingEstimateApi(Resource):
    @console_ns.doc("estimate_dataset_indexing")
    @console_ns.doc(description="Estimate dataset indexing cost")
    @console_ns.response(200, "Indexing estimate calculated successfully")
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.expect(console_ns.models[IndexingEstimatePayload.__name__])
    def post(self):
        payload = IndexingEstimatePayload.model_validate(console_ns.payload or {})
        args = payload.model_dump()
        _, current_tenant_id = current_account_with_tenant()
        # validate args
        DocumentService.estimate_args_validate(args)
        extract_settings = []
        if args["info_list"]["data_source_type"] == "upload_file":
            file_ids = args["info_list"]["file_info_list"]["file_ids"]
            file_details = db.session.scalars(
                select(UploadFile).where(UploadFile.tenant_id == current_tenant_id, UploadFile.id.in_(file_ids))
            ).all()

            if file_details is None:
                raise NotFound("File not found.")

            if file_details:
                for file_detail in file_details:
                    extract_setting = ExtractSetting(
                        datasource_type=DatasourceType.FILE,
                        upload_file=file_detail,
                        document_model=args["doc_form"],
                    )
                    extract_settings.append(extract_setting)
        elif args["info_list"]["data_source_type"] == "notion_import":
            notion_info_list = args["info_list"]["notion_info_list"]
            for notion_info in notion_info_list:
                workspace_id = notion_info["workspace_id"]
                credential_id = notion_info.get("credential_id")
                for page in notion_info["pages"]:
                    extract_setting = ExtractSetting(
                        datasource_type=DatasourceType.NOTION,
                        notion_info=NotionInfo.model_validate(
                            {
                                "credential_id": credential_id,
                                "notion_workspace_id": workspace_id,
                                "notion_obj_id": page["page_id"],
                                "notion_page_type": page["type"],
                                "tenant_id": current_tenant_id,
                            }
                        ),
                        document_model=args["doc_form"],
                    )
                    extract_settings.append(extract_setting)
        elif args["info_list"]["data_source_type"] == "website_crawl":
            website_info_list = args["info_list"]["website_info_list"]
            for url in website_info_list["urls"]:
                extract_setting = ExtractSetting(
                    datasource_type=DatasourceType.WEBSITE,
                    website_info=WebsiteInfo.model_validate(
                        {
                            "provider": website_info_list["provider"],
                            "job_id": website_info_list["job_id"],
                            "url": url,
                            "tenant_id": current_tenant_id,
                            "mode": "crawl",
                            "only_main_content": website_info_list["only_main_content"],
                        }
                    ),
                    document_model=args["doc_form"],
                )
                extract_settings.append(extract_setting)
        else:
            raise ValueError("Data source type not support")
        indexing_runner = IndexingRunner()
        try:
            response = indexing_runner.indexing_estimate(
                current_tenant_id,
                extract_settings,
                args["process_rule"],
                args["doc_form"],
                args["doc_language"],
                args["dataset_id"],
                args["indexing_technique"],
            )
        except LLMBadRequestError:
            raise ProviderNotInitializeError(
                "No Embedding Model available. Please configure a valid provider in the Settings -> Model Provider."
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except Exception as e:
            raise IndexingEstimateError(str(e))

        return response.model_dump(), 200


@console_ns.route("/datasets/<uuid:dataset_id>/related-apps")
class DatasetRelatedAppListApi(Resource):
    @console_ns.doc("get_dataset_related_apps")
    @console_ns.doc(description="Get applications related to dataset")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Related apps retrieved successfully", related_app_list_model)
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(related_app_list_model)
    def get(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        app_dataset_joins = DatasetService.get_related_apps(dataset.id)

        related_apps = []
        for app_dataset_join in app_dataset_joins:
            app_model = app_dataset_join.app
            if app_model:
                related_apps.append(app_model)

        return {"data": related_apps, "total": len(related_apps)}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/indexing-status")
class DatasetIndexingStatusApi(Resource):
    @console_ns.doc("get_dataset_indexing_status")
    @console_ns.doc(description="Get dataset indexing status")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Indexing status retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        _, current_tenant_id = current_account_with_tenant()
        dataset_id = str(dataset_id)
        documents = db.session.scalars(
            select(Document).where(Document.dataset_id == dataset_id, Document.tenant_id == current_tenant_id)
        ).all()
        documents_status = []
        for document in documents:
            completed_segments = (
                db.session.scalar(
                    select(func.count(DocumentSegment.id)).where(
                        DocumentSegment.completed_at.isnot(None),
                        DocumentSegment.document_id == str(document.id),
                        DocumentSegment.status != SegmentStatus.RE_SEGMENT,
                    )
                )
                or 0
            )
            total_segments = (
                db.session.scalar(
                    select(func.count(DocumentSegment.id)).where(
                        DocumentSegment.document_id == str(document.id),
                        DocumentSegment.status != SegmentStatus.RE_SEGMENT,
                    )
                )
                or 0
            )
            # Create a dictionary with document attributes and additional fields
            document_dict = {
                "id": document.id,
                "indexing_status": document.indexing_status,
                "processing_started_at": document.processing_started_at,
                "parsing_completed_at": document.parsing_completed_at,
                "cleaning_completed_at": document.cleaning_completed_at,
                "splitting_completed_at": document.splitting_completed_at,
                "completed_at": document.completed_at,
                "paused_at": document.paused_at,
                "error": document.error,
                "stopped_at": document.stopped_at,
                "completed_segments": completed_segments,
                "total_segments": total_segments,
            }
            documents_status.append(marshal(document_dict, document_status_fields))
        data = {"data": documents_status}
        return data, 200


@console_ns.route("/datasets/api-keys")
class DatasetApiKeyApi(Resource):
    max_keys = 10
    token_prefix = "dataset-"
    resource_type = ApiTokenType.DATASET

    @console_ns.doc("get_dataset_api_keys")
    @console_ns.doc(description="Get dataset API keys")
    @console_ns.response(200, "API keys retrieved successfully", api_key_list_model)
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(api_key_list_model)
    def get(self):
        _, current_tenant_id = current_account_with_tenant()
        keys = db.session.scalars(
            select(ApiToken).where(ApiToken.type == self.resource_type, ApiToken.tenant_id == current_tenant_id)
        ).all()
        return {"items": keys}

    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    @marshal_with(api_key_item_model)
    def post(self):
        _, current_tenant_id = current_account_with_tenant()

        current_key_count = (
            db.session.scalar(
                select(func.count(ApiToken.id)).where(
                    ApiToken.type == self.resource_type, ApiToken.tenant_id == current_tenant_id
                )
            )
            or 0
        )

        if current_key_count >= self.max_keys:
            console_ns.abort(
                400,
                message=f"Cannot create more than {self.max_keys} API keys for this resource type.",
                custom="max_keys_exceeded",
            )

        key = ApiToken.generate_api_key(self.token_prefix, 24)
        api_token = ApiToken()
        api_token.tenant_id = current_tenant_id
        api_token.token = key
        api_token.type = self.resource_type
        db.session.add(api_token)
        db.session.commit()
        return api_token, 200


@console_ns.route("/datasets/api-keys/<uuid:api_key_id>")
class DatasetApiDeleteApi(Resource):
    resource_type = ApiTokenType.DATASET

    @console_ns.doc("delete_dataset_api_key")
    @console_ns.doc(description="Delete dataset API key")
    @console_ns.doc(params={"api_key_id": "API key ID"})
    @console_ns.response(204, "API key deleted successfully")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def delete(self, api_key_id):
        _, current_tenant_id = current_account_with_tenant()
        api_key_id = str(api_key_id)
        key = db.session.scalar(
            select(ApiToken)
            .where(
                ApiToken.tenant_id == current_tenant_id,
                ApiToken.type == self.resource_type,
                ApiToken.id == api_key_id,
            )
            .limit(1)
        )

        if key is None:
            console_ns.abort(404, message="API key not found")

        # Invalidate cache before deleting from database
        # Type assertion: key is guaranteed to be non-None here because abort() raises
        assert key is not None  # nosec - for type checker only
        ApiTokenCache.delete(key.token, key.type)

        db.session.delete(key)
        db.session.commit()

        return {"result": "success"}, 204


@console_ns.route("/datasets/<uuid:dataset_id>/api-keys/<string:status>")
class DatasetEnableApiApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, dataset_id, status):
        dataset_id_str = str(dataset_id)

        DatasetService.update_dataset_api_status(dataset_id_str, status == "enable")

        return {"result": "success"}, 200


@console_ns.route("/datasets/api-base-info")
class DatasetApiBaseUrlApi(Resource):
    @console_ns.doc("get_dataset_api_base_info")
    @console_ns.doc(description="Get dataset API base information")
    @console_ns.response(200, "API base info retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        return {"api_base_url": (dify_config.SERVICE_API_URL or request.host_url.rstrip("/")) + "/v1"}


@console_ns.route("/datasets/retrieval-setting")
class DatasetRetrievalSettingApi(Resource):
    @console_ns.doc("get_dataset_retrieval_setting")
    @console_ns.doc(description="Get dataset retrieval settings")
    @console_ns.response(200, "Retrieval settings retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        vector_type = dify_config.VECTOR_STORE
        return _get_retrieval_methods_by_vector_type(vector_type, is_mock=False)


@console_ns.route("/datasets/retrieval-setting/<string:vector_type>")
class DatasetRetrievalSettingMockApi(Resource):
    @console_ns.doc("get_dataset_retrieval_setting_mock")
    @console_ns.doc(description="Get mock dataset retrieval settings by vector type")
    @console_ns.doc(params={"vector_type": "Vector store type"})
    @console_ns.response(200, "Mock retrieval settings retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, vector_type):
        return _get_retrieval_methods_by_vector_type(vector_type, is_mock=True)


@console_ns.route("/datasets/<uuid:dataset_id>/error-docs")
class DatasetErrorDocs(Resource):
    @console_ns.doc("get_dataset_error_docs")
    @console_ns.doc(description="Get dataset error documents")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Error documents retrieved successfully")
    @console_ns.response(404, "Dataset not found")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        results = DocumentService.get_error_documents_by_dataset_id(dataset_id_str)

        return {"data": [marshal(item, document_status_fields) for item in results], "total": len(results)}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/permission-part-users")
class DatasetPermissionUserListApi(Resource):
    @console_ns.doc("get_dataset_permission_users")
    @console_ns.doc(description="Get dataset permission user list")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Permission users retrieved successfully")
    @console_ns.response(404, "Dataset not found")
    @console_ns.response(403, "Permission denied")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        partial_members_list = DatasetPermissionService.get_dataset_partial_member_list(dataset_id_str)

        return {
            "data": partial_members_list,
        }, 200


@console_ns.route("/datasets/<uuid:dataset_id>/auto-disable-logs")
class DatasetAutoDisableLogApi(Resource):
    @console_ns.doc("get_dataset_auto_disable_logs")
    @console_ns.doc(description="Get dataset auto disable logs")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.response(200, "Auto disable logs retrieved successfully")
    @console_ns.response(404, "Dataset not found")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id):
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")
        return DatasetService.get_dataset_auto_disable_logs(dataset_id_str), 200

```

### api/controllers/console/datasets/datasets_segments.py
```py
import uuid

from flask import request
from flask_restx import Resource, marshal
from graphon.model_runtime.entities.model_entities import ModelType
from pydantic import BaseModel, Field
from sqlalchemy import String, cast, func, or_, select
from sqlalchemy.dialects.postgresql import JSONB
from werkzeug.exceptions import Forbidden, NotFound

import services
from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.app.error import ProviderNotInitializeError
from controllers.console.datasets.error import (
    ChildChunkDeleteIndexError,
    ChildChunkIndexingError,
    InvalidActionError,
)
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_knowledge_limit_check,
    cloud_edition_billing_rate_limit_check,
    cloud_edition_billing_resource_check,
    setup_required,
)
from core.errors.error import LLMBadRequestError, ProviderTokenNotInitError
from core.model_manager import ModelManager
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from fields.segment_fields import child_chunk_fields, segment_fields
from libs.helper import escape_like_pattern
from libs.login import current_account_with_tenant, login_required
from models.dataset import ChildChunk, DocumentSegment
from models.model import UploadFile
from services.dataset_service import DatasetService, DocumentService, SegmentService
from services.entities.knowledge_entities.knowledge_entities import ChildChunkUpdateArgs, SegmentUpdateArgs
from services.errors.chunk import ChildChunkDeleteIndexError as ChildChunkDeleteIndexServiceError
from services.errors.chunk import ChildChunkIndexingError as ChildChunkIndexingServiceError
from tasks.batch_create_segment_to_index_task import batch_create_segment_to_index_task


def _get_segment_with_summary(segment, dataset_id):
    """Helper function to marshal segment and add summary information."""
    from services.summary_index_service import SummaryIndexService

    segment_dict = dict(marshal(segment, segment_fields))  # type: ignore
    # Query summary for this segment (only enabled summaries)
    summary = SummaryIndexService.get_segment_summary(segment_id=segment.id, dataset_id=dataset_id)
    segment_dict["summary"] = summary.summary_content if summary else None
    return segment_dict


class SegmentListQuery(BaseModel):
    limit: int = Field(default=20, ge=1, le=100)
    status: list[str] = Field(default_factory=list)
    hit_count_gte: int | None = None
    enabled: str = Field(default="all")
    keyword: str | None = None
    page: int = Field(default=1, ge=1)


class SegmentCreatePayload(BaseModel):
    content: str
    answer: str | None = None
    keywords: list[str] | None = None
    attachment_ids: list[str] | None = None


class SegmentUpdatePayload(BaseModel):
    content: str
    answer: str | None = None
    keywords: list[str] | None = None
    regenerate_child_chunks: bool = False
    attachment_ids: list[str] | None = None
    summary: str | None = None  # Summary content for summary index


class BatchImportPayload(BaseModel):
    upload_file_id: str


class ChildChunkCreatePayload(BaseModel):
    content: str


class ChildChunkUpdatePayload(BaseModel):
    content: str


class ChildChunkBatchUpdatePayload(BaseModel):
    chunks: list[ChildChunkUpdateArgs]


register_schema_models(
    console_ns,
    SegmentListQuery,
    SegmentCreatePayload,
    SegmentUpdatePayload,
    BatchImportPayload,
    ChildChunkCreatePayload,
    ChildChunkUpdatePayload,
    ChildChunkBatchUpdatePayload,
    ChildChunkUpdateArgs,
)


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segments")
class DatasetDocumentSegmentListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id, document_id):
        current_user, current_tenant_id = current_account_with_tenant()

        dataset_id = str(dataset_id)
        document_id = str(document_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        document = DocumentService.get_document(dataset_id, document_id)

        if not document:
            raise NotFound("Document not found.")

        args = SegmentListQuery.model_validate(
            {
                **request.args.to_dict(),
                "status": request.args.getlist("status"),
            }
        )

        page = args.page
        limit = min(args.limit, 100)
        status_list = args.status
        hit_count_gte = args.hit_count_gte
        keyword = args.keyword

        query = (
            select(DocumentSegment)
            .where(
                DocumentSegment.document_id == str(document_id),
                DocumentSegment.tenant_id == current_tenant_id,
            )
            .order_by(DocumentSegment.position.asc())
        )

        if status_list:
            query = query.where(DocumentSegment.status.in_(status_list))

        if hit_count_gte is not None:
            query = query.where(DocumentSegment.hit_count >= hit_count_gte)

        if keyword:
            # Escape special characters in keyword to prevent SQL injection via LIKE wildcards
            escaped_keyword = escape_like_pattern(keyword)
            # Search in both content and keywords fields
            # Use database-specific methods for JSON array search
            if dify_config.SQLALCHEMY_DATABASE_URI_SCHEME == "postgresql":
                # PostgreSQL: Use jsonb_array_elements_text to properly handle Unicode/Chinese text
                keywords_condition = func.array_to_string(
                    func.array(
                        select(func.jsonb_array_elements_text(cast(DocumentSegment.keywords, JSONB)))
                        .correlate(DocumentSegment)
                        .scalar_subquery()
                    ),
                    ",",
                ).ilike(f"%{escaped_keyword}%", escape="\\")
            else:
                # MySQL: Cast JSON to string for pattern matching
                # MySQL stores Chinese text directly in JSON without Unicode escaping
                keywords_condition = cast(DocumentSegment.keywords, String).ilike(f"%{escaped_keyword}%", escape="\\")

            query = query.where(
                or_(
                    DocumentSegment.content.ilike(f"%{escaped_keyword}%", escape="\\"),
                    keywords_condition,
                )
            )

        if args.enabled.lower() != "all":
            if args.enabled.lower() == "true":
                query = query.where(DocumentSegment.enabled == True)
            elif args.enabled.lower() == "false":
                query = query.where(DocumentSegment.enabled == False)

        segments = db.paginate(select=query, page=page, per_page=limit, max_per_page=100, error_out=False)

        # Query summaries for all segments in this page (batch query for efficiency)
        segment_ids = [segment.id for segment in segments.items]
        summaries = {}
        if segment_ids:
            from services.summary_index_service import SummaryIndexService

            summary_records = SummaryIndexService.get_segments_summaries(segment_ids=segment_ids, dataset_id=dataset_id)
            # Only include enabled summaries (already filtered by service)
            summaries = {chunk_id: summary.summary_content for chunk_id, summary in summary_records.items()}

        # Add summary to each segment
        segments_with_summary = []
        for segment in segments.items:
            segment_dict = dict(marshal(segment, segment_fields))  # type: ignore
            segment_dict["summary"] = summaries.get(segment.id)
            segments_with_summary.append(segment_dict)

        response = {
            "data": segments_with_summary,
            "limit": limit,
            "total": segments.total,
            "total_pages": segments.pages,
            "page": page,
        }
        return response, 200

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def delete(self, dataset_id, document_id):
        current_user, _ = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        segment_ids = request.args.getlist("segment_id")

        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        SegmentService.delete_segments(segment_ids, document, dataset)
        return {"result": "success"}, 204


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segment/<string:action>")
class DatasetDocumentSegmentApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_rate_limit_check("knowledge")
    def patch(self, dataset_id, document_id, action):
        current_user, current_tenant_id = current_account_with_tenant()

        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            # check embedding model setting
            try:
                model_manager = ModelManager.for_tenant(tenant_id=current_tenant_id)
                model_manager.get_model_instance(
                    tenant_id=current_tenant_id,
                    provider=dataset.embedding_model_provider,
                    model_type=ModelType.TEXT_EMBEDDING,
                    model=dataset.embedding_model,
                )
            except LLMBadRequestError:
                raise ProviderNotInitializeError(
                    "No Embedding Model available. Please configure a valid provider in the Settings -> Model Provider."
                )
            except ProviderTokenNotInitError as ex:
                raise ProviderNotInitializeError(ex.description)
        segment_ids = request.args.getlist("segment_id")

        document_indexing_cache_key = f"document_{document.id}_indexing"
        cache_result = redis_client.get(document_indexing_cache_key)
        if cache_result is not None:
            raise InvalidActionError("Document is being indexed, please try again later")
        try:
            SegmentService.update_segments_status(segment_ids, action, dataset, document)
        except Exception as e:
            raise InvalidActionError(str(e))
        return {"result": "success"}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segment")
class DatasetDocumentSegmentAddApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_knowledge_limit_check("add_segment")
    @cloud_edition_billing_rate_limit_check("knowledge")
    @console_ns.expect(console_ns.models[SegmentCreatePayload.__name__])
    def post(self, dataset_id, document_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        if not current_user.is_dataset_editor:
            raise Forbidden()
        # check embedding model setting
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            try:
                model_manager = ModelManager.for_tenant(tenant_id=current_tenant_id)
                model_manager.get_model_instance(
                    tenant_id=current_tenant_id,
                    provider=dataset.embedding_model_provider,
                    model_type=ModelType.TEXT_EMBEDDING,
                    model=dataset.embedding_model,
                )
            except LLMBadRequestError:
                raise ProviderNotInitializeError(
                    "No Embedding Model available. Please configure a valid provider in the Settings -> Model Provider."
                )
            except ProviderTokenNotInitError as ex:
                raise ProviderNotInitializeError(ex.description)
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        # validate args
        payload = SegmentCreatePayload.model_validate(console_ns.payload or {})
        payload_dict = payload.model_dump(exclude_none=True)
        SegmentService.segment_create_args_validate(payload_dict, document)
        segment = SegmentService.create_segment(payload_dict, document, dataset)
        return {"data": _get_segment_with_summary(segment, dataset_id), "doc_form": document.doc_form}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segments/<uuid:segment_id>")
class DatasetDocumentSegmentUpdateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_rate_limit_check("knowledge")
    @console_ns.expect(console_ns.models[SegmentUpdatePayload.__name__])
    def patch(self, dataset_id, document_id, segment_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            # check embedding model setting
            try:
                model_manager = ModelManager.for_tenant(tenant_id=current_tenant_id)
                model_manager.get_model_instance(
                    tenant_id=current_tenant_id,
                    provider=dataset.embedding_model_provider,
                    model_type=ModelType.TEXT_EMBEDDING,
                    model=dataset.embedding_model,
                )
            except LLMBadRequestError:
                raise ProviderNotInitializeError(
                    "No Embedding Model available. Please configure a valid provider in the Settings -> Model Provider."
                )
            except ProviderTokenNotInitError as ex:
                raise ProviderNotInitializeError(ex.description)
            # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        # validate args
        payload = SegmentUpdatePayload.model_validate(console_ns.payload or {})
        payload_dict = payload.model_dump(exclude_none=True)
        SegmentService.segment_create_args_validate(payload_dict, document)

        # Update segment (summary update with change detection is handled in SegmentService.update_segment)
        segment = SegmentService.update_segment(
            SegmentUpdateArgs.model_validate(payload.model_dump(exclude_none=True)), segment, document, dataset
        )
        return {"data": _get_segment_with_summary(segment, dataset_id), "doc_form": document.doc_form}, 200

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def delete(self, dataset_id, document_id, segment_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        SegmentService.delete_segment(segment, document, dataset)
        return {"result": "success"}, 204


@console_ns.route(
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segments/batch_import",
    "/datasets/batch_import_status/<uuid:job_id>",
)
class DatasetDocumentSegmentBatchImportApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_knowledge_limit_check("add_segment")
    @cloud_edition_billing_rate_limit_check("knowledge")
    @console_ns.expect(console_ns.models[BatchImportPayload.__name__])
    def post(self, dataset_id, document_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")

        payload = BatchImportPayload.model_validate(console_ns.payload or {})
        upload_file_id = payload.upload_file_id

        upload_file = db.session.scalar(select(UploadFile).where(UploadFile.id == upload_file_id).limit(1))
        if not upload_file:
            raise NotFound("UploadFile not found.")

        # check file type
        if not upload_file.name or not upload_file.name.lower().endswith(".csv"):
            raise ValueError("Invalid file type. Only CSV files are allowed")

        try:
            # async job
            job_id = str(uuid.uuid4())
            indexing_cache_key = f"segment_batch_import_{str(job_id)}"
            # send batch add segments task
            redis_client.setnx(indexing_cache_key, "waiting")
            batch_create_segment_to_index_task.delay(
                str(job_id),
                upload_file_id,
                dataset_id,
                document_id,
                current_tenant_id,
                current_user.id,
            )
        except Exception as e:
            return {"error": str(e)}, 500
        return {"job_id": job_id, "job_status": "waiting"}, 200

    @setup_required
    @login_required
    @account_initialization_required
    def get(self, job_id=None, dataset_id=None, document_id=None):
        if job_id is None:
            raise NotFound("The job does not exist.")
        job_id = str(job_id)
        indexing_cache_key = f"segment_batch_import_{job_id}"
        cache_result = redis_client.get(indexing_cache_key)
        if cache_result is None:
            raise ValueError("The job does not exist.")

        return {"job_id": job_id, "job_status": cache_result.decode()}, 200


@console_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segments/<uuid:segment_id>/child_chunks")
class ChildChunkAddApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_knowledge_limit_check("add_segment")
    @cloud_edition_billing_rate_limit_check("knowledge")
    @console_ns.expect(console_ns.models[ChildChunkCreatePayload.__name__])
    def post(self, dataset_id, document_id, segment_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        if not current_user.is_dataset_editor:
            raise Forbidden()
        # check embedding model setting
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            try:
                model_manager = ModelManager.for_tenant(tenant_id=current_tenant_id)
                model_manager.get_model_instance(
                    tenant_id=current_tenant_id,
                    provider=dataset.embedding_model_provider,
                    model_type=ModelType.TEXT_EMBEDDING,
                    model=dataset.embedding_model,
                )
            except LLMBadRequestError:
                raise ProviderNotInitializeError(
                    "No Embedding Model available. Please configure a valid provider in the Settings -> Model Provider."
                )
            except ProviderTokenNotInitError as ex:
                raise ProviderNotInitializeError(ex.description)
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        # validate args
        try:
            payload = ChildChunkCreatePayload.model_validate(console_ns.payload or {})
            child_chunk = SegmentService.create_child_chunk(payload.content, segment, document, dataset)
        except ChildChunkIndexingServiceError as e:
            raise ChildChunkIndexingError(str(e))
        return {"data": marshal(child_chunk, child_chunk_fields)}, 200

    @setup_required
    @login_required
    @account_initialization_required
    def get(self, dataset_id, document_id, segment_id):
        _, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        args = SegmentListQuery.model_validate(
            {
                "limit": request.args.get("limit", default=20, type=int),
                "keyword": request.args.get("keyword"),
                "page": request.args.get("page", default=1, type=int),
            }
        )

        page = args.page
        limit = min(args.limit, 100)
        keyword = args.keyword

        child_chunks = SegmentService.get_child_chunks(segment_id, document_id, dataset_id, page, limit, keyword)
        return {
            "data": marshal(child_chunks.items, child_chunk_fields),
            "total": child_chunks.total,
            "total_pages": child_chunks.pages,
            "page": page,
            "limit": limit,
        }, 200

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_rate_limit_check("knowledge")
    def patch(self, dataset_id, document_id, segment_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
            # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        # validate args
        payload = ChildChunkBatchUpdatePayload.model_validate(console_ns.payload or {})
        try:
            child_chunks = SegmentService.update_child_chunks(payload.chunks, segment, document, dataset)
        except ChildChunkIndexingServiceError as e:
            raise ChildChunkIndexingError(str(e))
        return {"data": marshal(child_chunks, child_chunk_fields)}, 200


@console_ns.route(
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/segments/<uuid:segment_id>/child_chunks/<uuid:child_chunk_id>"
)
class ChildChunkUpdateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def delete(self, dataset_id, document_id, segment_id, child_chunk_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
        # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        # check child chunk
        child_chunk_id = str(child_chunk_id)
        child_chunk = db.session.scalar(
            select(ChildChunk)
            .where(
                ChildChunk.id == str(child_chunk_id),
                ChildChunk.tenant_id == current_tenant_id,
                ChildChunk.segment_id == segment.id,
                ChildChunk.document_id == document_id,
            )
            .limit(1)
        )
        if not child_chunk:
            raise NotFound("Child chunk not found.")
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        try:
            SegmentService.delete_child_chunk(child_chunk, dataset)
        except ChildChunkDeleteIndexServiceError as e:
            raise ChildChunkDeleteIndexError(str(e))
        return {"result": "success"}, 204

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("vector_space")
    @cloud_edition_billing_rate_limit_check("knowledge")
    @console_ns.expect(console_ns.models[ChildChunkUpdatePayload.__name__])
    def patch(self, dataset_id, document_id, segment_id, child_chunk_id):
        current_user, current_tenant_id = current_account_with_tenant()

        # check dataset
        dataset_id = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id)
        if not dataset:
            raise NotFound("Dataset not found.")
        # check user's model setting
        DatasetService.check_dataset_model_setting(dataset)
        # check document
        document_id = str(document_id)
        document = DocumentService.get_document(dataset_id, document_id)
        if not document:
            raise NotFound("Document not found.")
            # check segment
        segment_id = str(segment_id)
        segment = db.session.scalar(
            select(DocumentSegment)
            .where(DocumentSegment.id == str(segment_id), DocumentSegment.tenant_id == current_tenant_id)
            .limit(1)
        )
        if not segment:
            raise NotFound("Segment not found.")
        # check child chunk
        child_chunk_id = str(child_chunk_id)
        child_chunk = db.session.scalar(
            select(ChildChunk)
            .where(
                ChildChunk.id == str(child_chunk_id),
                ChildChunk.tenant_id == current_tenant_id,
                ChildChunk.segment_id == segment.id,
                ChildChunk.document_id == document_id,
            )
            .limit(1)
        )
        if not child_chunk:
            raise NotFound("Child chunk not found.")
        # The role of the current user in the ta table must be admin, owner, dataset_operator, or editor
        if not current_user.is_dataset_editor:
            raise Forbidden()
        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))
        # validate args
        try:
            payload = ChildChunkUpdatePayload.model_validate(console_ns.payload or {})
            child_chunk = SegmentService.update_child_chunk(payload.content, child_chunk, segment, document, dataset)
        except ChildChunkIndexingServiceError as e:
            raise ChildChunkIndexingError(str(e))
        return {"data": marshal(child_chunk, child_chunk_fields)}, 200

```

### api/controllers/console/datasets/hit_testing.py
```py
from flask_restx import Resource, fields

from controllers.common.schema import register_schema_model
from fields.hit_testing_fields import (
    child_chunk_fields,
    document_fields,
    files_fields,
    hit_testing_record_fields,
    segment_fields,
)
from libs.login import login_required

from .. import console_ns
from ..datasets.hit_testing_base import DatasetsHitTestingBase, HitTestingPayload
from ..wraps import (
    account_initialization_required,
    cloud_edition_billing_rate_limit_check,
    setup_required,
)

register_schema_model(console_ns, HitTestingPayload)


def _get_or_create_model(model_name: str, field_def):
    """Get or create a flask_restx model to avoid dict type issues in Swagger."""
    existing = console_ns.models.get(model_name)
    if existing is None:
        existing = console_ns.model(model_name, field_def)
    return existing


# Register models for flask_restx to avoid dict type issues in Swagger
document_model = _get_or_create_model("HitTestingDocument", document_fields)

segment_fields_copy = segment_fields.copy()
segment_fields_copy["document"] = fields.Nested(document_model)
segment_model = _get_or_create_model("HitTestingSegment", segment_fields_copy)

child_chunk_model = _get_or_create_model("HitTestingChildChunk", child_chunk_fields)
files_model = _get_or_create_model("HitTestingFile", files_fields)

hit_testing_record_fields_copy = hit_testing_record_fields.copy()
hit_testing_record_fields_copy["segment"] = fields.Nested(segment_model)
hit_testing_record_fields_copy["child_chunks"] = fields.List(fields.Nested(child_chunk_model))
hit_testing_record_fields_copy["files"] = fields.List(fields.Nested(files_model))
hit_testing_record_model = _get_or_create_model("HitTestingRecord", hit_testing_record_fields_copy)

# Response model for hit testing API
hit_testing_response_fields = {
    "query": fields.String,
    "records": fields.List(fields.Nested(hit_testing_record_model)),
}
hit_testing_response_model = _get_or_create_model("HitTestingResponse", hit_testing_response_fields)


@console_ns.route("/datasets/<uuid:dataset_id>/hit-testing")
class HitTestingApi(Resource, DatasetsHitTestingBase):
    @console_ns.doc("test_dataset_retrieval")
    @console_ns.doc(description="Test dataset knowledge retrieval")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.expect(console_ns.models[HitTestingPayload.__name__])
    @console_ns.response(200, "Hit testing completed successfully", model=hit_testing_response_model)
    @console_ns.response(404, "Dataset not found")
    @console_ns.response(400, "Invalid parameters")
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_rate_limit_check("knowledge")
    def post(self, dataset_id):
        dataset_id_str = str(dataset_id)

        dataset = self.get_and_validate_dataset(dataset_id_str)
        payload = HitTestingPayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)
        self.hit_testing_args_check(args)

        return self.perform_hit_testing(dataset, args)

```

### api/controllers/console/datasets/website.py
```py
from typing import Literal

from flask import request
from flask_restx import Resource
from pydantic import BaseModel

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.datasets.error import WebsiteCrawlError
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import login_required
from services.website_service import WebsiteCrawlApiRequest, WebsiteCrawlStatusApiRequest, WebsiteService


class WebsiteCrawlPayload(BaseModel):
    provider: Literal["firecrawl", "watercrawl", "jinareader"]
    url: str
    options: dict[str, object]


class WebsiteCrawlStatusQuery(BaseModel):
    provider: Literal["firecrawl", "watercrawl", "jinareader"]


register_schema_models(console_ns, WebsiteCrawlPayload, WebsiteCrawlStatusQuery)


@console_ns.route("/website/crawl")
class WebsiteCrawlApi(Resource):
    @console_ns.doc("crawl_website")
    @console_ns.doc(description="Crawl website content")
    @console_ns.expect(console_ns.models[WebsiteCrawlPayload.__name__])
    @console_ns.response(200, "Website crawl initiated successfully")
    @console_ns.response(400, "Invalid crawl parameters")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = WebsiteCrawlPayload.model_validate(console_ns.payload or {})

        # Create typed request and validate
        try:
            api_request = WebsiteCrawlApiRequest.from_args(payload.model_dump())
        except ValueError as e:
            raise WebsiteCrawlError(str(e))

        # Crawl URL using typed request
        try:
            result = WebsiteService.crawl_url(api_request)
        except Exception as e:
            raise WebsiteCrawlError(str(e))
        return result, 200


@console_ns.route("/website/crawl/status/<string:job_id>")
class WebsiteCrawlStatusApi(Resource):
    @console_ns.doc("get_crawl_status")
    @console_ns.doc(description="Get website crawl status")
    @console_ns.doc(params={"job_id": "Crawl job ID", "provider": "Crawl provider (firecrawl/watercrawl/jinareader)"})
    @console_ns.expect(console_ns.models[WebsiteCrawlStatusQuery.__name__])
    @console_ns.response(200, "Crawl status retrieved successfully")
    @console_ns.response(404, "Crawl job not found")
    @console_ns.response(400, "Invalid provider")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, job_id: str):
        args = WebsiteCrawlStatusQuery.model_validate(request.args.to_dict())

        # Create typed request and validate
        try:
            api_request = WebsiteCrawlStatusApiRequest.from_args(args.model_dump(), job_id)
        except ValueError as e:
            raise WebsiteCrawlError(str(e))

        # Get crawl status using typed request
        try:
            result = WebsiteService.get_crawl_status_typed(api_request)
        except Exception as e:
            raise WebsiteCrawlError(str(e))
        return result, 200

```

### api/controllers/console/datasets/external.py
```py
from flask import request
from flask_restx import Resource, fields, marshal
from pydantic import BaseModel, Field
from werkzeug.exceptions import Forbidden, InternalServerError, NotFound

import services
from controllers.common.schema import get_or_create_model, register_schema_models
from controllers.console import console_ns
from controllers.console.datasets.error import DatasetNameDuplicateError
from controllers.console.wraps import account_initialization_required, edit_permission_required, setup_required
from fields.dataset_fields import (
    dataset_detail_fields,
    dataset_retrieval_model_fields,
    doc_metadata_fields,
    external_knowledge_info_fields,
    external_retrieval_model_fields,
    icon_info_fields,
    keyword_setting_fields,
    reranking_model_fields,
    tag_fields,
    vector_setting_fields,
    weighted_score_fields,
)
from libs.login import current_account_with_tenant, login_required
from services.dataset_service import DatasetService
from services.external_knowledge_service import ExternalDatasetService
from services.hit_testing_service import HitTestingService
from services.knowledge_service import BedrockRetrievalSetting, ExternalDatasetTestService


def _build_dataset_detail_model():
    keyword_setting_model = get_or_create_model("DatasetKeywordSetting", keyword_setting_fields)
    vector_setting_model = get_or_create_model("DatasetVectorSetting", vector_setting_fields)

    weighted_score_fields_copy = weighted_score_fields.copy()
    weighted_score_fields_copy["keyword_setting"] = fields.Nested(keyword_setting_model)
    weighted_score_fields_copy["vector_setting"] = fields.Nested(vector_setting_model)
    weighted_score_model = get_or_create_model("DatasetWeightedScore", weighted_score_fields_copy)

    reranking_model = get_or_create_model("DatasetRerankingModel", reranking_model_fields)

    dataset_retrieval_model_fields_copy = dataset_retrieval_model_fields.copy()
    dataset_retrieval_model_fields_copy["reranking_model"] = fields.Nested(reranking_model)
    dataset_retrieval_model_fields_copy["weights"] = fields.Nested(weighted_score_model, allow_null=True)
    dataset_retrieval_model = get_or_create_model("DatasetRetrievalModel", dataset_retrieval_model_fields_copy)

    tag_model = get_or_create_model("Tag", tag_fields)
    doc_metadata_model = get_or_create_model("DatasetDocMetadata", doc_metadata_fields)
    external_knowledge_info_model = get_or_create_model("ExternalKnowledgeInfo", external_knowledge_info_fields)
    external_retrieval_model = get_or_create_model("ExternalRetrievalModel", external_retrieval_model_fields)
    icon_info_model = get_or_create_model("DatasetIconInfo", icon_info_fields)

    dataset_detail_fields_copy = dataset_detail_fields.copy()
    dataset_detail_fields_copy["retrieval_model_dict"] = fields.Nested(dataset_retrieval_model)
    dataset_detail_fields_copy["tags"] = fields.List(fields.Nested(tag_model))
    dataset_detail_fields_copy["external_knowledge_info"] = fields.Nested(external_knowledge_info_model)
    dataset_detail_fields_copy["external_retrieval_model"] = fields.Nested(external_retrieval_model, allow_null=True)
    dataset_detail_fields_copy["doc_metadata"] = fields.List(fields.Nested(doc_metadata_model))
    dataset_detail_fields_copy["icon_info"] = fields.Nested(icon_info_model)
    return get_or_create_model("DatasetDetail", dataset_detail_fields_copy)


try:
    dataset_detail_model = console_ns.models["DatasetDetail"]
except KeyError:
    dataset_detail_model = _build_dataset_detail_model()


class ExternalKnowledgeApiPayload(BaseModel):
    name: str = Field(..., min_length=1, max_length=40)
    settings: dict[str, object]


class ExternalDatasetCreatePayload(BaseModel):
    external_knowledge_api_id: str
    external_knowledge_id: str
    name: str = Field(..., min_length=1, max_length=100)
    description: str | None = Field(None, max_length=400)
    external_retrieval_model: dict[str, object] | None = None


class ExternalHitTestingPayload(BaseModel):
    query: str
    external_retrieval_model: dict[str, object] | None = None
    metadata_filtering_conditions: dict[str, object] | None = None


class BedrockRetrievalPayload(BaseModel):
    retrieval_setting: "BedrockRetrievalSetting"
    query: str
    knowledge_id: str


class ExternalApiTemplateListQuery(BaseModel):
    page: int = Field(default=1, description="Page number")
    limit: int = Field(default=20, description="Number of items per page")
    keyword: str | None = Field(default=None, description="Search keyword")


register_schema_models(
    console_ns,
    ExternalKnowledgeApiPayload,
    ExternalDatasetCreatePayload,
    ExternalHitTestingPayload,
    BedrockRetrievalPayload,
    ExternalApiTemplateListQuery,
)


@console_ns.route("/datasets/external-knowledge-api")
class ExternalApiTemplateListApi(Resource):
    @console_ns.doc("get_external_api_templates")
    @console_ns.doc(description="Get external knowledge API templates")
    @console_ns.doc(
        params={
            "page": "Page number (default: 1)",
            "limit": "Number of items per page (default: 20)",
            "keyword": "Search keyword",
        }
    )
    @console_ns.response(200, "External API templates retrieved successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()
        query = ExternalApiTemplateListQuery.model_validate(request.args.to_dict())

        external_knowledge_apis, total = ExternalDatasetService.get_external_knowledge_apis(
            query.page, query.limit, current_tenant_id, query.keyword
        )
        response = {
            "data": [item.to_dict() for item in external_knowledge_apis],
            "has_more": len(external_knowledge_apis) == query.limit,
            "limit": query.limit,
            "total": total,
            "page": query.page,
        }
        return response, 200

    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.expect(console_ns.models[ExternalKnowledgeApiPayload.__name__])
    def post(self):
        current_user, current_tenant_id = current_account_with_tenant()
        payload = ExternalKnowledgeApiPayload.model_validate(console_ns.payload or {})

        ExternalDatasetService.validate_api_list(payload.settings)

        # The role of the current user in the ta table must be admin, owner, or editor, or dataset_operator
        if not current_user.is_dataset_editor:
            raise Forbidden()

        try:
            external_knowledge_api = ExternalDatasetService.create_external_knowledge_api(
                tenant_id=current_tenant_id, user_id=current_user.id, args=payload.model_dump()
            )
        except services.errors.dataset.DatasetNameDuplicateError:
            raise DatasetNameDuplicateError()

        return external_knowledge_api.to_dict(), 201


@console_ns.route("/datasets/external-knowledge-api/<uuid:external_knowledge_api_id>")
class ExternalApiTemplateApi(Resource):
    @console_ns.doc("get_external_api_template")
    @console_ns.doc(description="Get external knowledge API template details")
    @console_ns.doc(params={"external_knowledge_api_id": "External knowledge API ID"})
    @console_ns.response(200, "External API template retrieved successfully")
    @console_ns.response(404, "Template not found")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, external_knowledge_api_id):
        _, current_tenant_id = current_account_with_tenant()
        external_knowledge_api_id = str(external_knowledge_api_id)
        external_knowledge_api = ExternalDatasetService.get_external_knowledge_api(
            external_knowledge_api_id, current_tenant_id
        )
        if external_knowledge_api is None:
            raise NotFound("API template not found.")

        return external_knowledge_api.to_dict(), 200

    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.expect(console_ns.models[ExternalKnowledgeApiPayload.__name__])
    def patch(self, external_knowledge_api_id):
        current_user, current_tenant_id = current_account_with_tenant()
        external_knowledge_api_id = str(external_knowledge_api_id)

        payload = ExternalKnowledgeApiPayload.model_validate(console_ns.payload or {})
        ExternalDatasetService.validate_api_list(payload.settings)

        external_knowledge_api = ExternalDatasetService.update_external_knowledge_api(
            tenant_id=current_tenant_id,
            user_id=current_user.id,
            external_knowledge_api_id=external_knowledge_api_id,
            args=payload.model_dump(),
        )

        return external_knowledge_api.to_dict(), 200

    @setup_required
    @login_required
    @account_initialization_required
    def delete(self, external_knowledge_api_id):
        current_user, current_tenant_id = current_account_with_tenant()
        external_knowledge_api_id = str(external_knowledge_api_id)

        if not (current_user.has_edit_permission or current_user.is_dataset_operator):
            raise Forbidden()

        ExternalDatasetService.delete_external_knowledge_api(current_tenant_id, external_knowledge_api_id)
        return {"result": "success"}, 204


@console_ns.route("/datasets/external-knowledge-api/<uuid:external_knowledge_api_id>/use-check")
class ExternalApiUseCheckApi(Resource):
    @console_ns.doc("check_external_api_usage")
    @console_ns.doc(description="Check if external knowledge API is being used")
    @console_ns.doc(params={"external_knowledge_api_id": "External knowledge API ID"})
    @console_ns.response(200, "Usage check completed successfully")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, external_knowledge_api_id):
        external_knowledge_api_id = str(external_knowledge_api_id)

        external_knowledge_api_is_using, count = ExternalDatasetService.external_knowledge_api_use_check(
            external_knowledge_api_id
        )
        return {"is_using": external_knowledge_api_is_using, "count": count}, 200


@console_ns.route("/datasets/external")
class ExternalDatasetCreateApi(Resource):
    @console_ns.doc("create_external_dataset")
    @console_ns.doc(description="Create external knowledge dataset")
    @console_ns.expect(console_ns.models[ExternalDatasetCreatePayload.__name__])
    @console_ns.response(201, "External dataset created successfully", dataset_detail_model)
    @console_ns.response(400, "Invalid parameters")
    @console_ns.response(403, "Permission denied")
    @setup_required
    @login_required
    @account_initialization_required
    @edit_permission_required
    def post(self):
        # The role of the current user in the ta table must be admin, owner, or editor
        current_user, current_tenant_id = current_account_with_tenant()
        payload = ExternalDatasetCreatePayload.model_validate(console_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        # The role of the current user in the ta table must be admin, owner, or editor, or dataset_operator
        if not current_user.is_dataset_editor:
            raise Forbidden()

        try:
            dataset = ExternalDatasetService.create_external_dataset(
                tenant_id=current_tenant_id,
                user_id=current_user.id,
                args=args,
            )
        except services.errors.dataset.DatasetNameDuplicateError:
            raise DatasetNameDuplicateError()

        return marshal(dataset, dataset_detail_fields), 201


@console_ns.route("/datasets/<uuid:dataset_id>/external-hit-testing")
class ExternalKnowledgeHitTestingApi(Resource):
    @console_ns.doc("test_external_knowledge_retrieval")
    @console_ns.doc(description="Test external knowledge retrieval for dataset")
    @console_ns.doc(params={"dataset_id": "Dataset ID"})
    @console_ns.expect(console_ns.models[ExternalHitTestingPayload.__name__])
    @console_ns.response(200, "External hit testing completed successfully")
    @console_ns.response(404, "Dataset not found")
    @console_ns.response(400, "Invalid parameters")
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, dataset_id):
        current_user, _ = current_account_with_tenant()
        dataset_id_str = str(dataset_id)
        dataset = DatasetService.get_dataset(dataset_id_str)
        if dataset is None:
            raise NotFound("Dataset not found.")

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        payload = ExternalHitTestingPayload.model_validate(console_ns.payload or {})
        HitTestingService.hit_testing_args_check(payload.model_dump())

        try:
            response = HitTestingService.external_retrieve(
                dataset=dataset,
                query=payload.query,
                account=current_user,
                external_retrieval_model=payload.external_retrieval_model,
                metadata_filtering_conditions=payload.metadata_filtering_conditions,
            )

            return response
        except Exception as e:
            raise InternalServerError(str(e))


@console_ns.route("/test/retrieval")
class BedrockRetrievalApi(Resource):
    # this api is only for internal testing
    @console_ns.doc("bedrock_retrieval_test")
    @console_ns.doc(description="Bedrock retrieval test (internal use only)")
    @console_ns.expect(console_ns.models[BedrockRetrievalPayload.__name__])
    @console_ns.response(200, "Bedrock retrieval test completed")
    def post(self):
        payload = BedrockRetrievalPayload.model_validate(console_ns.payload or {})

        # Call the knowledge retrieval service
        result = ExternalDatasetTestService.knowledge_retrieval(
            payload.retrieval_setting, payload.query, payload.knowledge_id
        )
        return result, 200

```

### api/controllers/console/datasets/wraps.py
```py
from collections.abc import Callable
from functools import wraps

from sqlalchemy import select

from controllers.console.datasets.error import PipelineNotFoundError
from extensions.ext_database import db
from libs.login import current_account_with_tenant
from models.dataset import Pipeline


def get_rag_pipeline[**P, R](view_func: Callable[P, R]) -> Callable[P, R]:
    @wraps(view_func)
    def decorated_view(*args: P.args, **kwargs: P.kwargs) -> R:
        if not kwargs.get("pipeline_id"):
            raise ValueError("missing pipeline_id in path parameters")

        _, current_tenant_id = current_account_with_tenant()

        pipeline_id = kwargs.get("pipeline_id")
        pipeline_id = str(pipeline_id)

        del kwargs["pipeline_id"]

        pipeline = db.session.scalar(
            select(Pipeline).where(Pipeline.id == pipeline_id, Pipeline.tenant_id == current_tenant_id).limit(1)
        )

        if not pipeline:
            raise PipelineNotFoundError()

        kwargs["pipeline"] = pipeline

        return view_func(*args, **kwargs)

    return decorated_view

```

### api/controllers/console/datasets/hit_testing_base.py
```py
import logging
from typing import Any

from flask_restx import marshal
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from werkzeug.exceptions import Forbidden, InternalServerError, NotFound

import services
from controllers.console.app.error import (
    CompletionRequestError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.console.datasets.error import DatasetNotInitializedError
from core.errors.error import (
    LLMBadRequestError,
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from fields.hit_testing_fields import hit_testing_record_fields
from libs.login import current_user
from models.account import Account
from services.dataset_service import DatasetService
from services.entities.knowledge_entities.knowledge_entities import RetrievalModel
from services.hit_testing_service import HitTestingService

logger = logging.getLogger(__name__)


class HitTestingPayload(BaseModel):
    query: str = Field(max_length=250)
    retrieval_model: RetrievalModel | None = None
    external_retrieval_model: dict[str, Any] | None = None
    attachment_ids: list[str] | None = None


class DatasetsHitTestingBase:
    @staticmethod
    def get_and_validate_dataset(dataset_id: str):
        assert isinstance(current_user, Account)
        dataset = DatasetService.get_dataset(dataset_id)
        if dataset is None:
            raise NotFound("Dataset not found.")

        try:
            DatasetService.check_dataset_permission(dataset, current_user)
        except services.errors.account.NoPermissionError as e:
            raise Forbidden(str(e))

        return dataset

    @staticmethod
    def hit_testing_args_check(args: dict[str, Any]):
        HitTestingService.hit_testing_args_check(args)

    @staticmethod
    def parse_args(payload: dict[str, Any]) -> dict[str, Any]:
        """Validate and return hit-testing arguments from an incoming payload."""
        hit_testing_payload = HitTestingPayload.model_validate(payload or {})
        return hit_testing_payload.model_dump(exclude_none=True)

    @staticmethod
    def perform_hit_testing(dataset, args):
        assert isinstance(current_user, Account)
        try:
            response = HitTestingService.retrieve(
                dataset=dataset,
                query=args.get("query"),
                account=current_user,
                retrieval_model=args.get("retrieval_model"),
                external_retrieval_model=args.get("external_retrieval_model"),
                attachment_ids=args.get("attachment_ids"),
                limit=10,
            )
            return {"query": response["query"], "records": marshal(response["records"], hit_testing_record_fields)}
        except services.errors.index.IndexNotInitializedError:
            raise DatasetNotInitializedError()
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except LLMBadRequestError:
            raise ProviderNotInitializeError(
                "No Embedding Model or Reranking Model available. Please configure a valid provider "
                "in the Settings -> Model Provider."
            )
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except ValueError as e:
            raise ValueError(str(e))
        except Exception as e:
            logger.exception("Hit testing failed.")
            raise InternalServerError(str(e))

```

### api/services/rag_pipeline/pipeline_template/pipeline_template_factory.py
```py
from services.rag_pipeline.pipeline_template.built_in.built_in_retrieval import BuiltInPipelineTemplateRetrieval
from services.rag_pipeline.pipeline_template.customized.customized_retrieval import CustomizedPipelineTemplateRetrieval
from services.rag_pipeline.pipeline_template.database.database_retrieval import DatabasePipelineTemplateRetrieval
from services.rag_pipeline.pipeline_template.pipeline_template_base import PipelineTemplateRetrievalBase
from services.rag_pipeline.pipeline_template.pipeline_template_type import PipelineTemplateType
from services.rag_pipeline.pipeline_template.remote.remote_retrieval import RemotePipelineTemplateRetrieval


class PipelineTemplateRetrievalFactory:
    @staticmethod
    def get_pipeline_template_factory(mode: str) -> type[PipelineTemplateRetrievalBase]:
        match mode:
            case PipelineTemplateType.REMOTE:
                return RemotePipelineTemplateRetrieval
            case PipelineTemplateType.CUSTOMIZED:
                return CustomizedPipelineTemplateRetrieval
            case PipelineTemplateType.DATABASE:
                return DatabasePipelineTemplateRetrieval
            case PipelineTemplateType.BUILTIN:
                return BuiltInPipelineTemplateRetrieval
            case _:
                raise ValueError(f"invalid fetch recommended apps mode: {mode}")

    @staticmethod
    def get_built_in_pipeline_template_retrieval():
        return BuiltInPipelineTemplateRetrieval

```

### api/services/rag_pipeline/pipeline_template/database/database_retrieval.py
```py
import yaml
from sqlalchemy import select

from extensions.ext_database import db
from models.dataset import PipelineBuiltInTemplate
from services.rag_pipeline.pipeline_template.pipeline_template_base import PipelineTemplateRetrievalBase
from services.rag_pipeline.pipeline_template.pipeline_template_type import PipelineTemplateType


class DatabasePipelineTemplateRetrieval(PipelineTemplateRetrievalBase):
    """
    Retrieval pipeline   template from database
    """

    def get_pipeline_templates(self, language: str) -> dict:
        result = self.fetch_pipeline_templates_from_db(language)
        return result

    def get_pipeline_template_detail(self, template_id: str):
        result = self.fetch_pipeline_template_detail_from_db(template_id)
        return result

    def get_type(self) -> str:
        return PipelineTemplateType.DATABASE

    @classmethod
    def fetch_pipeline_templates_from_db(cls, language: str) -> dict:
        """
        Fetch pipeline templates from db.
        :param language: language
        :return:
        """

        pipeline_built_in_templates = list(
            db.session.scalars(
                select(PipelineBuiltInTemplate).where(PipelineBuiltInTemplate.language == language)
            ).all()
        )

        recommended_pipelines_results = []
        for pipeline_built_in_template in pipeline_built_in_templates:
            recommended_pipeline_result = {
                "id": pipeline_built_in_template.id,
                "name": pipeline_built_in_template.name,
                "description": pipeline_built_in_template.description,
                "icon": pipeline_built_in_template.icon,
                "copyright": pipeline_built_in_template.copyright,
                "privacy_policy": pipeline_built_in_template.privacy_policy,
                "position": pipeline_built_in_template.position,
                "chunk_structure": pipeline_built_in_template.chunk_structure,
            }
            recommended_pipelines_results.append(recommended_pipeline_result)

        return {"pipeline_templates": recommended_pipelines_results}

    @classmethod
    def fetch_pipeline_template_detail_from_db(cls, template_id: str) -> dict | None:
        """
        Fetch pipeline template detail from db.
        :param pipeline_id: Pipeline ID
        :return:
        """
        # is in public recommended list
        pipeline_template = db.session.get(PipelineBuiltInTemplate, template_id)

        if not pipeline_template:
            return None
        dsl_data = yaml.safe_load(pipeline_template.yaml_content)
        graph_data = dsl_data.get("workflow", {}).get("graph", {})
        return {
            "id": pipeline_template.id,
            "name": pipeline_template.name,
            "icon_info": pipeline_template.icon,
            "description": pipeline_template.description,
            "chunk_structure": pipeline_template.chunk_structure,
            "export_data": pipeline_template.yaml_content,
            "graph": graph_data,
        }

```

### api/services/rag_pipeline/pipeline_template/database/__init__.py
```py

```

### api/services/rag_pipeline/pipeline_template/pipeline_template_base.py
```py
from abc import ABC, abstractmethod


class PipelineTemplateRetrievalBase(ABC):
    """Interface for pipeline template retrieval."""

    @abstractmethod
    def get_pipeline_templates(self, language: str) -> dict:
        raise NotImplementedError

    @abstractmethod
    def get_pipeline_template_detail(self, template_id: str) -> dict | None:
        raise NotImplementedError

    @abstractmethod
    def get_type(self) -> str:
        raise NotImplementedError

```

### api/services/rag_pipeline/pipeline_template/__init__.py
```py

```

### api/services/rag_pipeline/pipeline_template/built_in/__init__.py
```py

```

### api/services/rag_pipeline/pipeline_template/built_in/built_in_retrieval.py
```py
import json
from os import path
from pathlib import Path

from flask import current_app

from services.rag_pipeline.pipeline_template.pipeline_template_base import PipelineTemplateRetrievalBase
from services.rag_pipeline.pipeline_template.pipeline_template_type import PipelineTemplateType


class BuiltInPipelineTemplateRetrieval(PipelineTemplateRetrievalBase):
    """
    Retrieval pipeline template from built-in, the location  is constants/pipeline_templates.json
    """

    builtin_data: dict | None = None

    def get_type(self) -> str:
        return PipelineTemplateType.BUILTIN

    def get_pipeline_templates(self, language: str) -> dict:
        result = self.fetch_pipeline_templates_from_builtin(language)
        return result

    def get_pipeline_template_detail(self, template_id: str):
        result = self.fetch_pipeline_template_detail_from_builtin(template_id)
        return result

    @classmethod
    def _get_builtin_data(cls) -> dict:
        """
        Get builtin data.
        :return:
        """
        if cls.builtin_data:
            return cls.builtin_data

        root_path = current_app.root_path
        cls.builtin_data = json.loads(
            Path(path.join(root_path, "constants", "pipeline_templates.json")).read_text(encoding="utf-8")
        )

        return cls.builtin_data or {}

    @classmethod
    def fetch_pipeline_templates_from_builtin(cls, language: str) -> dict:
        """
        Fetch pipeline templates from builtin.
        :param language: language
        :return:
        """
        builtin_data: dict[str, dict[str, dict]] = cls._get_builtin_data()
        return builtin_data.get("pipeline_templates", {}).get(language, {})

    @classmethod
    def fetch_pipeline_template_detail_from_builtin(cls, template_id: str) -> dict | None:
        """
        Fetch pipeline template detail from builtin.
        :param template_id: Template ID
        :return:
        """
        builtin_data: dict[str, dict[str, dict]] = cls._get_builtin_data()
        return builtin_data.get("pipeline_templates", {}).get(template_id)

```

### api/services/rag_pipeline/pipeline_template/pipeline_template_type.py
```py
from enum import StrEnum


class PipelineTemplateType(StrEnum):
    REMOTE = "remote"
    DATABASE = "database"
    CUSTOMIZED = "customized"
    BUILTIN = "builtin"

```

### api/services/rag_pipeline/pipeline_template/customized/__init__.py
```py

```

### api/services/rag_pipeline/pipeline_template/customized/customized_retrieval.py
```py
import yaml
from sqlalchemy import select

from extensions.ext_database import db
from libs.login import current_account_with_tenant
from models.dataset import PipelineCustomizedTemplate
from services.rag_pipeline.pipeline_template.pipeline_template_base import PipelineTemplateRetrievalBase
from services.rag_pipeline.pipeline_template.pipeline_template_type import PipelineTemplateType


class CustomizedPipelineTemplateRetrieval(PipelineTemplateRetrievalBase):
    """
    Retrieval recommended app from database
    """

    def get_pipeline_templates(self, language: str) -> dict:
        _, current_tenant_id = current_account_with_tenant()
        result = self.fetch_pipeline_templates_from_customized(tenant_id=current_tenant_id, language=language)
        return result

    def get_pipeline_template_detail(self, template_id: str):
        result = self.fetch_pipeline_template_detail_from_db(template_id)
        return result

    def get_type(self) -> str:
        return PipelineTemplateType.CUSTOMIZED

    @classmethod
    def fetch_pipeline_templates_from_customized(cls, tenant_id: str, language: str) -> dict:
        """
        Fetch pipeline templates from db.
        :param tenant_id: tenant id
        :param language: language
        :return:
        """
        pipeline_customized_templates = db.session.scalars(
            select(PipelineCustomizedTemplate)
            .where(PipelineCustomizedTemplate.tenant_id == tenant_id, PipelineCustomizedTemplate.language == language)
            .order_by(PipelineCustomizedTemplate.position.asc(), PipelineCustomizedTemplate.created_at.desc())
        ).all()
        recommended_pipelines_results = []
        for pipeline_customized_template in pipeline_customized_templates:
            recommended_pipeline_result = {
                "id": pipeline_customized_template.id,
                "name": pipeline_customized_template.name,
                "description": pipeline_customized_template.description,
                "icon": pipeline_customized_template.icon,
                "position": pipeline_customized_template.position,
                "chunk_structure": pipeline_customized_template.chunk_structure,
            }
            recommended_pipelines_results.append(recommended_pipeline_result)

        return {"pipeline_templates": recommended_pipelines_results}

    @classmethod
    def fetch_pipeline_template_detail_from_db(cls, template_id: str) -> dict | None:
        """
        Fetch pipeline template detail from db.
        :param template_id: Template ID
        :return:
        """
        pipeline_template = db.session.get(PipelineCustomizedTemplate, template_id)
        if not pipeline_template:
            return None

        dsl_data = yaml.safe_load(pipeline_template.yaml_content)
        graph_data = dsl_data.get("workflow", {}).get("graph", {})

        return {
            "id": pipeline_template.id,
            "name": pipeline_template.name,
            "icon_info": pipeline_template.icon,
            "description": pipeline_template.description,
            "chunk_structure": pipeline_template.chunk_structure,
            "export_data": pipeline_template.yaml_content,
            "graph": graph_data,
            "created_by": pipeline_template.created_user_name,
        }

```

### api/services/rag_pipeline/pipeline_template/remote/__init__.py
```py

```

### api/services/rag_pipeline/pipeline_template/remote/remote_retrieval.py
```py
import logging

import httpx

from configs import dify_config
from services.rag_pipeline.pipeline_template.database.database_retrieval import DatabasePipelineTemplateRetrieval
from services.rag_pipeline.pipeline_template.pipeline_template_base import PipelineTemplateRetrievalBase
from services.rag_pipeline.pipeline_template.pipeline_template_type import PipelineTemplateType

logger = logging.getLogger(__name__)


class RemotePipelineTemplateRetrieval(PipelineTemplateRetrievalBase):
    """
    Retrieval recommended app from dify official
    """

    def get_pipeline_template_detail(self, template_id: str) -> dict | None:
        result: dict | None
        try:
            result = self.fetch_pipeline_template_detail_from_dify_official(template_id)
        except Exception as e:
            logger.warning("fetch recommended app detail from dify official failed: %r, switch to database.", e)
            result = DatabasePipelineTemplateRetrieval.fetch_pipeline_template_detail_from_db(template_id)
        return result

    def get_pipeline_templates(self, language: str) -> dict:
        try:
            result = self.fetch_pipeline_templates_from_dify_official(language)
        except Exception as e:
            logger.warning("fetch pipeline templates from dify official failed: %r, switch to database.", e)
            result = DatabasePipelineTemplateRetrieval.fetch_pipeline_templates_from_db(language)
        return result

    def get_type(self) -> str:
        return PipelineTemplateType.REMOTE

    @classmethod
    def fetch_pipeline_template_detail_from_dify_official(cls, template_id: str) -> dict:
        """
        Fetch pipeline template detail from dify official.

        :param template_id: Pipeline template ID
        :return: Template detail dict
        :raises ValueError: When upstream returns a non-200 status code
        """
        domain = dify_config.HOSTED_FETCH_PIPELINE_TEMPLATES_REMOTE_DOMAIN
        url = f"{domain}/pipeline-templates/{template_id}"
        response = httpx.get(url, timeout=httpx.Timeout(10.0, connect=3.0))
        if response.status_code != 200:
            raise ValueError(
                "fetch pipeline template detail failed,"
                + f" status_code: {response.status_code},"
                + f" response: {response.text[:1000]}"
            )
        data: dict = response.json()
        return data

    @classmethod
    def fetch_pipeline_templates_from_dify_official(cls, language: str) -> dict:
        """
        Fetch pipeline templates from dify official.
        :param language: language
        :return:
        """
        domain = dify_config.HOSTED_FETCH_PIPELINE_TEMPLATES_REMOTE_DOMAIN
        url = f"{domain}/pipeline-templates?language={language}"
        response = httpx.get(url, timeout=httpx.Timeout(10.0, connect=3.0))
        if response.status_code != 200:
            raise ValueError(f"fetch pipeline templates failed, status code: {response.status_code}")

        result: dict = response.json()

        return result

```

### api/services/rag_pipeline/entity/pipeline_service_api_entities.py
```py
from collections.abc import Mapping
from typing import Any

from pydantic import BaseModel


class DatasourceNodeRunApiEntity(BaseModel):
    pipeline_id: str
    node_id: str
    inputs: dict[str, Any]
    datasource_type: str
    credential_id: str | None = None
    is_published: bool


class PipelineRunApiEntity(BaseModel):
    inputs: Mapping[str, Any]
    datasource_type: str
    datasource_info_list: list[Mapping[str, Any]]
    start_node_id: str
    is_published: bool
    response_mode: str

```

### api/services/rag_pipeline/pipeline_generate_service.py
```py
from collections.abc import Mapping
from typing import Any, Union

from configs import dify_config
from core.app.apps.pipeline.pipeline_generator import PipelineGenerator
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from models.dataset import Document, Pipeline
from models.enums import IndexingStatus
from models.model import Account, App, EndUser
from models.workflow import Workflow
from services.rag_pipeline.rag_pipeline import RagPipelineService


class PipelineGenerateService:
    @classmethod
    def generate(
        cls,
        pipeline: Pipeline,
        user: Union[Account, EndUser],
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: bool = True,
    ):
        """
        Pipeline Content Generate
        :param pipeline: pipeline
        :param user: user
        :param args: args
        :param invoke_from: invoke from
        :param streaming: streaming
        :return:
        """
        try:
            workflow = cls._get_workflow(pipeline, invoke_from)
            if original_document_id := args.get("original_document_id"):
                # update document status to waiting
                cls.update_document_status(original_document_id)
            return PipelineGenerator.convert_to_event_stream(
                PipelineGenerator().generate(
                    pipeline=pipeline,
                    workflow=workflow,
                    user=user,
                    args=args,
                    invoke_from=invoke_from,
                    streaming=streaming,
                    call_depth=0,
                    workflow_thread_pool_id=None,
                ),
            )

        except Exception:
            raise

    @staticmethod
    def _get_max_active_requests(app_model: App) -> int:
        app_limit = app_model.max_active_requests or dify_config.APP_DEFAULT_ACTIVE_REQUESTS
        config_limit = dify_config.APP_MAX_ACTIVE_REQUESTS
        # Filter out infinite (0) values and return the minimum, or 0 if both are infinite
        limits = [limit for limit in [app_limit, config_limit] if limit > 0]
        return min(limits) if limits else 0

    @classmethod
    def generate_single_iteration(
        cls, pipeline: Pipeline, user: Account, node_id: str, args: Any, streaming: bool = True
    ):
        workflow = cls._get_workflow(pipeline, InvokeFrom.DEBUGGER)
        return PipelineGenerator.convert_to_event_stream(
            PipelineGenerator().single_iteration_generate(
                pipeline=pipeline, workflow=workflow, node_id=node_id, user=user, args=args, streaming=streaming
            )
        )

    @classmethod
    def generate_single_loop(cls, pipeline: Pipeline, user: Account, node_id: str, args: Any, streaming: bool = True):
        workflow = cls._get_workflow(pipeline, InvokeFrom.DEBUGGER)
        return PipelineGenerator.convert_to_event_stream(
            PipelineGenerator().single_loop_generate(
                pipeline=pipeline, workflow=workflow, node_id=node_id, user=user, args=args, streaming=streaming
            )
        )

    @classmethod
    def _get_workflow(cls, pipeline: Pipeline, invoke_from: InvokeFrom) -> Workflow:
        """
        Get workflow
        :param pipeline: pipeline
        :param invoke_from: invoke from
        :return:
        """
        rag_pipeline_service = RagPipelineService()
        if invoke_from == InvokeFrom.DEBUGGER:
            # fetch draft workflow by app_model
            workflow = rag_pipeline_service.get_draft_workflow(pipeline=pipeline)

            if not workflow:
                raise ValueError("Workflow not initialized")
        else:
            # fetch published workflow by app_model
            workflow = rag_pipeline_service.get_published_workflow(pipeline=pipeline)

            if not workflow:
                raise ValueError("Workflow not published")

        return workflow

    @classmethod
    def update_document_status(cls, document_id: str):
        """
        Update document status to waiting
        :param document_id: document id
        """
        document = db.session.get(Document, document_id)
        if document:
            document.indexing_status = IndexingStatus.WAITING
            db.session.add(document)
            db.session.commit()

```

### api/services/rag_pipeline/rag_pipeline_task_proxy.py
```py
import json
import logging
from collections.abc import Callable, Sequence
from functools import cached_property

from core.app.entities.rag_pipeline_invoke_entities import RagPipelineInvokeEntity
from core.rag.pipeline.queue import TenantIsolatedTaskQueue
from enums.cloud_plan import CloudPlan
from extensions.ext_database import db
from services.feature_service import FeatureService
from services.file_service import FileService
from tasks.rag_pipeline.priority_rag_pipeline_run_task import priority_rag_pipeline_run_task
from tasks.rag_pipeline.rag_pipeline_run_task import rag_pipeline_run_task

logger = logging.getLogger(__name__)


class RagPipelineTaskProxy:
    # Default uploaded file name for rag pipeline invoke entities
    _RAG_PIPELINE_INVOKE_ENTITIES_FILE_NAME = "rag_pipeline_invoke_entities.json"

    def __init__(
        self, dataset_tenant_id: str, user_id: str, rag_pipeline_invoke_entities: Sequence[RagPipelineInvokeEntity]
    ):
        self._dataset_tenant_id = dataset_tenant_id
        self._user_id = user_id
        self._rag_pipeline_invoke_entities = rag_pipeline_invoke_entities
        self._tenant_isolated_task_queue = TenantIsolatedTaskQueue(dataset_tenant_id, "pipeline")

    @cached_property
    def features(self):
        return FeatureService.get_features(self._dataset_tenant_id)

    def _upload_invoke_entities(self) -> str:
        text = [item.model_dump() for item in self._rag_pipeline_invoke_entities]
        # Convert list to proper JSON string
        json_text = json.dumps(text)
        upload_file = FileService(db.engine).upload_text(
            json_text, self._RAG_PIPELINE_INVOKE_ENTITIES_FILE_NAME, self._user_id, self._dataset_tenant_id
        )
        logger.info(
            "tenant %s upload %d invoke entities", self._dataset_tenant_id, len(self._rag_pipeline_invoke_entities)
        )
        return upload_file.id

    def _send_to_direct_queue(self, upload_file_id: str, task_func: Callable[[str, str], None]):
        logger.info("tenant %s send file %s to direct queue", self._dataset_tenant_id, upload_file_id)
        task_func.delay(  # type: ignore
            rag_pipeline_invoke_entities_file_id=upload_file_id,
            tenant_id=self._dataset_tenant_id,
        )

    def _send_to_tenant_queue(self, upload_file_id: str, task_func: Callable[[str, str], None]):
        logger.info("tenant %s send file %s to tenant queue", self._dataset_tenant_id, upload_file_id)
        if self._tenant_isolated_task_queue.get_task_key():
            # Add to waiting queue using List operations (lpush)
            self._tenant_isolated_task_queue.push_tasks([upload_file_id])
            logger.info("tenant %s push tasks: %s", self._dataset_tenant_id, upload_file_id)
        else:
            # Set flag and execute task
            self._tenant_isolated_task_queue.set_task_waiting_time()
            task_func.delay(  # type: ignore
                rag_pipeline_invoke_entities_file_id=upload_file_id,
                tenant_id=self._dataset_tenant_id,
            )
            logger.info("tenant %s init tasks: %s", self._dataset_tenant_id, upload_file_id)

    def _send_to_default_tenant_queue(self, upload_file_id: str):
        self._send_to_tenant_queue(upload_file_id, rag_pipeline_run_task)

    def _send_to_priority_tenant_queue(self, upload_file_id: str):
        self._send_to_tenant_queue(upload_file_id, priority_rag_pipeline_run_task)

    def _send_to_priority_direct_queue(self, upload_file_id: str):
        self._send_to_direct_queue(upload_file_id, priority_rag_pipeline_run_task)

    def _dispatch(self):
        upload_file_id = self._upload_invoke_entities()
        if not upload_file_id:
            raise ValueError("upload_file_id is empty")

        logger.info(
            "dispatch args: %s - %s - %s",
            self._dataset_tenant_id,
            self.features.billing.enabled,
            self.features.billing.subscription.plan,
        )

        # dispatch to different pipeline queue with tenant isolation when billing enabled
        if self.features.billing.enabled:
            if self.features.billing.subscription.plan == CloudPlan.SANDBOX:
                # dispatch to normal pipeline queue with tenant isolation for sandbox plan
                self._send_to_default_tenant_queue(upload_file_id)
            else:
                # dispatch to priority pipeline queue with tenant isolation for other plans
                self._send_to_priority_tenant_queue(upload_file_id)
        else:
            # dispatch to priority pipeline queue without tenant isolation for others, e.g.: self-hosted or enterprise
            self._send_to_priority_direct_queue(upload_file_id)

    def delay(self):
        if not self._rag_pipeline_invoke_entities:
            logger.warning(
                "Received empty rag pipeline invoke entities, no tasks delivered: %s %s",
                self._dataset_tenant_id,
                self._user_id,
            )
            return
        self._dispatch()

```

### api/services/rag_pipeline/rag_pipeline_manage_service.py
```py
from core.plugin.entities.plugin_daemon import PluginDatasourceProviderEntity
from core.plugin.impl.datasource import PluginDatasourceManager
from services.datasource_provider_service import DatasourceProviderService


class RagPipelineManageService:
    @staticmethod
    def list_rag_pipeline_datasources(tenant_id: str) -> list[PluginDatasourceProviderEntity]:
        """
        list rag pipeline datasources
        """

        # get all builtin providers
        manager = PluginDatasourceManager()
        datasources = manager.fetch_datasource_providers(tenant_id)
        for datasource in datasources:
            datasource_provider_service = DatasourceProviderService()
            credentials = datasource_provider_service.get_datasource_credentials(
                tenant_id=tenant_id, provider=datasource.provider, plugin_id=datasource.plugin_id
            )
            if credentials:
                datasource.is_authorized = True
        return datasources

```

### api/services/rag_pipeline/rag_pipeline_transform_service.py
```py
import json
import logging
from datetime import UTC, datetime
from pathlib import Path
from uuid import uuid4

import yaml
from flask_login import current_user
from sqlalchemy import select

from constants import DOCUMENT_EXTENSIONS
from core.plugin.impl.plugin import PluginInstaller
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from extensions.ext_database import db
from factories import variable_factory
from models.dataset import Dataset, Document, DocumentPipelineExecutionLog, Pipeline
from models.enums import DatasetRuntimeMode, DataSourceType
from models.model import UploadFile
from models.workflow import Workflow, WorkflowType
from services.entities.knowledge_entities.rag_pipeline_entities import KnowledgeConfiguration, RetrievalSetting
from services.plugin.plugin_migration import PluginMigration
from services.plugin.plugin_service import PluginService

logger = logging.getLogger(__name__)


class RagPipelineTransformService:
    def transform_dataset(self, dataset_id: str):
        dataset = db.session.get(Dataset, dataset_id)
        if not dataset:
            raise ValueError("Dataset not found")
        if dataset.pipeline_id and dataset.runtime_mode == DatasetRuntimeMode.RAG_PIPELINE:
            return {
                "pipeline_id": dataset.pipeline_id,
                "dataset_id": dataset_id,
                "status": "success",
            }
        if dataset.provider != "vendor":
            raise ValueError("External dataset is not supported")
        datasource_type = dataset.data_source_type
        indexing_technique = dataset.indexing_technique

        if not datasource_type and not indexing_technique:
            return self._transform_to_empty_pipeline(dataset)

        doc_form = dataset.doc_form
        if not doc_form:
            return self._transform_to_empty_pipeline(dataset)
        retrieval_model = RetrievalSetting.model_validate(dataset.retrieval_model) if dataset.retrieval_model else None
        pipeline_yaml = self._get_transform_yaml(doc_form, datasource_type, indexing_technique)
        # deal dependencies
        self._deal_dependencies(pipeline_yaml, dataset.tenant_id)
        # Extract app data
        workflow_data = pipeline_yaml.get("workflow")
        if not workflow_data:
            raise ValueError("Missing workflow data for rag pipeline")
        graph = workflow_data.get("graph", {})
        nodes = graph.get("nodes", [])
        new_nodes = []

        for node in nodes:
            if (
                node.get("data", {}).get("type") == "datasource"
                and node.get("data", {}).get("provider_type") == "local_file"
            ):
                node = self._deal_file_extensions(node)
            if node.get("data", {}).get("type") == "knowledge-index":
                knowledge_configuration = KnowledgeConfiguration.model_validate(node.get("data", {}))
                if dataset.tenant_id != current_user.current_tenant_id:
                    raise ValueError("Unauthorized")
                node = self._deal_knowledge_index(
                    knowledge_configuration, dataset, indexing_technique, retrieval_model, node
                )
            new_nodes.append(node)
        if new_nodes:
            graph["nodes"] = new_nodes
            workflow_data["graph"] = graph
            pipeline_yaml["workflow"] = workflow_data
        # create pipeline
        pipeline = self._create_pipeline(pipeline_yaml)

        # save chunk structure to dataset
        if doc_form == IndexStructureType.PARENT_CHILD_INDEX:
            dataset.chunk_structure = "hierarchical_model"
        elif doc_form == IndexStructureType.PARAGRAPH_INDEX:
            dataset.chunk_structure = "text_model"
        else:
            raise ValueError("Unsupported doc form")

        dataset.runtime_mode = DatasetRuntimeMode.RAG_PIPELINE
        dataset.pipeline_id = pipeline.id

        # deal document data
        self._deal_document_data(dataset)

        db.session.commit()
        return {
            "pipeline_id": pipeline.id,
            "dataset_id": dataset_id,
            "status": "success",
        }

    def _get_transform_yaml(self, doc_form: str, datasource_type: str, indexing_technique: str | None):
        pipeline_yaml = {}
        if doc_form == IndexStructureType.PARAGRAPH_INDEX:
            match datasource_type:
                case DataSourceType.UPLOAD_FILE:
                    if indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                        # get graph from transform.file-general-high-quality.yml
                        with open(f"{Path(__file__).parent}/transform/file-general-high-quality.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                    if indexing_technique == IndexTechniqueType.ECONOMY:
                        # get graph from transform.file-general-economy.yml
                        with open(f"{Path(__file__).parent}/transform/file-general-economy.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                case DataSourceType.NOTION_IMPORT:
                    if indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                        # get graph from transform.notion-general-high-quality.yml
                        with open(f"{Path(__file__).parent}/transform/notion-general-high-quality.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                    if indexing_technique == IndexTechniqueType.ECONOMY:
                        # get graph from transform.notion-general-economy.yml
                        with open(f"{Path(__file__).parent}/transform/notion-general-economy.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                case DataSourceType.WEBSITE_CRAWL:
                    if indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                        # get graph from transform.website-crawl-general-high-quality.yml
                        with open(f"{Path(__file__).parent}/transform/website-crawl-general-high-quality.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                    if indexing_technique == IndexTechniqueType.ECONOMY:
                        # get graph from transform.website-crawl-general-economy.yml
                        with open(f"{Path(__file__).parent}/transform/website-crawl-general-economy.yml") as f:
                            pipeline_yaml = yaml.safe_load(f)
                case _:
                    raise ValueError("Unsupported datasource type")
        elif doc_form == IndexStructureType.PARENT_CHILD_INDEX:
            match datasource_type:
                case DataSourceType.UPLOAD_FILE:
                    # get graph from transform.file-parentchild.yml
                    with open(f"{Path(__file__).parent}/transform/file-parentchild.yml") as f:
                        pipeline_yaml = yaml.safe_load(f)
                case DataSourceType.NOTION_IMPORT:
                    # get graph from transform.notion-parentchild.yml
                    with open(f"{Path(__file__).parent}/transform/notion-parentchild.yml") as f:
                        pipeline_yaml = yaml.safe_load(f)
                case DataSourceType.WEBSITE_CRAWL:
                    # get graph from transform.website-crawl-parentchild.yml
                    with open(f"{Path(__file__).parent}/transform/website-crawl-parentchild.yml") as f:
                        pipeline_yaml = yaml.safe_load(f)
                case _:
                    raise ValueError("Unsupported datasource type")
        else:
            raise ValueError("Unsupported doc form")
        return pipeline_yaml

    def _deal_file_extensions(self, node: dict):
        file_extensions = node.get("data", {}).get("fileExtensions", [])
        if not file_extensions:
            return node
        node["data"]["fileExtensions"] = [ext.lower() for ext in file_extensions if ext in DOCUMENT_EXTENSIONS]
        return node

    def _deal_knowledge_index(
        self,
        knowledge_configuration: KnowledgeConfiguration,
        dataset: Dataset,
        indexing_technique: str | None,
        retrieval_model: RetrievalSetting | None,
        node: dict,
    ):
        knowledge_configuration_dict = node.get("data", {})

        if indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            knowledge_configuration.embedding_model = dataset.embedding_model
            knowledge_configuration.embedding_model_provider = dataset.embedding_model_provider
        if retrieval_model:
            if indexing_technique == IndexTechniqueType.ECONOMY:
                retrieval_model.search_method = RetrievalMethod.KEYWORD_SEARCH
            knowledge_configuration.retrieval_model = retrieval_model
        else:
            dataset.retrieval_model = knowledge_configuration.retrieval_model.model_dump()

        # Copy summary_index_setting from dataset to knowledge_index node configuration
        if dataset.summary_index_setting:
            knowledge_configuration.summary_index_setting = dataset.summary_index_setting

        knowledge_configuration_dict.update(knowledge_configuration.model_dump())
        node["data"] = knowledge_configuration_dict
        return node

    def _create_pipeline(
        self,
        data: dict,
    ) -> Pipeline:
        """Create a new app or update an existing one."""
        pipeline_data = data.get("rag_pipeline", {})
        # Initialize pipeline based on mode
        workflow_data = data.get("workflow")
        if not workflow_data or not isinstance(workflow_data, dict):
            raise ValueError("Missing workflow data for rag pipeline")

        environment_variables_list = workflow_data.get("environment_variables", [])
        environment_variables = [
            variable_factory.build_environment_variable_from_mapping(obj) for obj in environment_variables_list
        ]
        conversation_variables_list = workflow_data.get("conversation_variables", [])
        conversation_variables = [
            variable_factory.build_conversation_variable_from_mapping(obj) for obj in conversation_variables_list
        ]
        rag_pipeline_variables_list = workflow_data.get("rag_pipeline_variables", [])

        graph = workflow_data.get("graph", {})

        # Create new app
        pipeline = Pipeline(
            tenant_id=current_user.current_tenant_id,
            name=pipeline_data.get("name", ""),
            description=pipeline_data.get("description", ""),
            created_by=current_user.id,
            updated_by=current_user.id,
            is_published=True,
            is_public=True,
        )
        pipeline.id = str(uuid4())

        db.session.add(pipeline)
        db.session.flush()
        # create draft workflow
        draft_workflow = Workflow(
            tenant_id=pipeline.tenant_id,
            app_id=pipeline.id,
            features="{}",
            type=WorkflowType.RAG_PIPELINE,
            version="draft",
            graph=json.dumps(graph),
            created_by=current_user.id,
            environment_variables=environment_variables,
            conversation_variables=conversation_variables,
            rag_pipeline_variables=rag_pipeline_variables_list,
        )
        published_workflow = Workflow(
            tenant_id=pipeline.tenant_id,
            app_id=pipeline.id,
            features="{}",
            type=WorkflowType.RAG_PIPELINE,
            version=str(datetime.now(UTC).replace(tzinfo=None)),
            graph=json.dumps(graph),
            created_by=current_user.id,
            environment_variables=environment_variables,
            conversation_variables=conversation_variables,
            rag_pipeline_variables=rag_pipeline_variables_list,
        )
        db.session.add(draft_workflow)
        db.session.add(published_workflow)
        db.session.flush()
        pipeline.workflow_id = published_workflow.id
        db.session.add(pipeline)
        return pipeline

    def _deal_dependencies(self, pipeline_yaml: dict, tenant_id: str):
        installer_manager = PluginInstaller()
        installed_plugins = installer_manager.list_plugins(tenant_id)

        plugin_migration = PluginMigration()

        installed_plugins_ids = [plugin.plugin_id for plugin in installed_plugins]
        dependencies = pipeline_yaml.get("dependencies", [])
        need_install_plugin_unique_identifiers = []
        for dependency in dependencies:
            if dependency.get("type") == "marketplace":
                plugin_unique_identifier = dependency.get("value", {}).get("plugin_unique_identifier")
                plugin_id = plugin_unique_identifier.split(":")[0]
                if plugin_id not in installed_plugins_ids:
                    plugin_unique_identifier = plugin_migration._fetch_plugin_unique_identifier(plugin_id)  # type: ignore
                    if plugin_unique_identifier:
                        need_install_plugin_unique_identifiers.append(plugin_unique_identifier)
        if need_install_plugin_unique_identifiers:
            logger.debug("Installing missing pipeline plugins %s", need_install_plugin_unique_identifiers)
            PluginService.install_from_marketplace_pkg(tenant_id, need_install_plugin_unique_identifiers)

    def _transform_to_empty_pipeline(self, dataset: Dataset):
        pipeline = Pipeline(
            tenant_id=dataset.tenant_id,
            name=dataset.name,
            description=dataset.description,
            created_by=current_user.id,
        )
        db.session.add(pipeline)
        db.session.flush()

        dataset.pipeline_id = pipeline.id
        dataset.runtime_mode = DatasetRuntimeMode.RAG_PIPELINE
        dataset.updated_by = current_user.id
        dataset.updated_at = datetime.now(UTC).replace(tzinfo=None)
        db.session.add(dataset)
        db.session.commit()
        return {
            "pipeline_id": pipeline.id,
            "dataset_id": dataset.id,
            "status": "success",
        }

    def _deal_document_data(self, dataset: Dataset):
        file_node_id = "1752479895761"
        notion_node_id = "1752489759475"
        jina_node_id = "1752491761974"
        firecrawl_node_id = "1752565402678"

        documents = db.session.scalars(select(Document).where(Document.dataset_id == dataset.id)).all()

        for document in documents:
            data_source_info_dict = document.data_source_info_dict
            if not data_source_info_dict:
                continue
            if document.data_source_type == DataSourceType.UPLOAD_FILE:
                document.data_source_type = DataSourceType.LOCAL_FILE
                file_id = data_source_info_dict.get("upload_file_id")
                if file_id:
                    file = db.session.get(UploadFile, file_id)
                    if file:
                        data_source_info = json.dumps(
                            {
                                "real_file_id": file_id,
                                "name": file.name,
                                "size": file.size,
                                "extension": file.extension,
                                "mime_type": file.mime_type,
                                "url": "",
                                "transfer_method": "local_file",
                            }
                        )
                        document.data_source_info = data_source_info
                        document_pipeline_execution_log = DocumentPipelineExecutionLog(
                            document_id=document.id,
                            pipeline_id=dataset.pipeline_id,
                            datasource_type=DataSourceType.LOCAL_FILE,
                            datasource_info=data_source_info,
                            input_data={},
                            created_by=document.created_by,
                            datasource_node_id=file_node_id,
                        )
                        document_pipeline_execution_log.created_at = document.created_at
                        db.session.add(document)
                        db.session.add(document_pipeline_execution_log)
            elif document.data_source_type == DataSourceType.NOTION_IMPORT:
                document.data_source_type = DataSourceType.ONLINE_DOCUMENT
                data_source_info = json.dumps(
                    {
                        "workspace_id": data_source_info_dict.get("notion_workspace_id"),
                        "page": {
                            "page_id": data_source_info_dict.get("notion_page_id"),
                            "page_name": document.name,
                            "page_icon": data_source_info_dict.get("notion_page_icon"),
                            "type": data_source_info_dict.get("type"),
                            "last_edited_time": data_source_info_dict.get("last_edited_time"),
                            "parent_id": None,
                        },
                    }
                )
                document.data_source_info = data_source_info
                document_pipeline_execution_log = DocumentPipelineExecutionLog(
                    document_id=document.id,
                    pipeline_id=dataset.pipeline_id,
                    datasource_type=DataSourceType.ONLINE_DOCUMENT,
                    datasource_info=data_source_info,
                    input_data={},
                    created_by=document.created_by,
                    datasource_node_id=notion_node_id,
                )
                document_pipeline_execution_log.created_at = document.created_at
                db.session.add(document)
                db.session.add(document_pipeline_execution_log)
            elif document.data_source_type == DataSourceType.WEBSITE_CRAWL:
                data_source_info = json.dumps(
                    {
                        "source_url": data_source_info_dict.get("url"),
                        "content": "",
                        "title": document.name,
                        "description": "",
                    }
                )
                document.data_source_info = data_source_info
                if data_source_info_dict.get("provider") == "firecrawl":
                    datasource_node_id = firecrawl_node_id
                elif data_source_info_dict.get("provider") == "jinareader":
                    datasource_node_id = jina_node_id
                else:
                    continue
                document_pipeline_execution_log = DocumentPipelineExecutionLog(
                    document_id=document.id,
                    pipeline_id=dataset.pipeline_id,
                    datasource_type=DataSourceType.WEBSITE_CRAWL,
                    datasource_info=data_source_info,
                    input_data={},
                    created_by=document.created_by,
                    datasource_node_id=datasource_node_id,
                )
                document_pipeline_execution_log.created_at = document.created_at
                db.session.add(document)
                db.session.add(document_pipeline_execution_log)

```

### api/services/rag_pipeline/transform/notion-general-economy.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/notion_datasource:0.0.1@2dd49c2c3ffff976be8d22efb1ac0f63522a8d0f24ef8c44729d0a50a94ec039
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: ''
  icon_type: emoji
  name: notion-general-economy
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752482151668-source-1752477924228-target
      source: '1752482151668'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: tool
      id: 1752489759475-source-1752482151668-target
      source: '1752489759475'
      sourceHandle: source
      target: '1752482151668'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752482151668'
        - result
        indexing_technique: economy
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: keyword_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: true
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 1444.5503479271906
        y: 281.3910724383104
      positionAbsolute:
        x: 1444.5503479271906
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos pedaços.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: Delimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長
            pt_BR: The chunk overlap length.
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Chunk Overlap Length
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace consecutive spaces, newlines and tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace Consecutive Spaces, Newlines and Tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete all URLs and email addresses
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete All URLs and Email Addresses
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752489759475.content#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752482151668'
      position:
        x: 1063.6922916384628
        y: 281.3910724383104
      positionAbsolute:
        x: 1063.6922916384628
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Notion数据源
        datasource_name: notion_datasource
        datasource_parameters: {}
        plugin_id: langgenius/notion_datasource
        provider_name: notion_datasource
        provider_type: online_document
        selected: false
        title: Notion数据源
        type: datasource
      height: 52
      id: '1752489759475'
      position:
        x: 736.9082104000458
        y: 281.3910724383104
      positionAbsolute:
        x: 736.9082104000458
        y: 281.3910724383104
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -838.569649323166
      y: -168.94656489167426
      zoom: 1.286925643857699
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Chunk overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/notion-parentchild.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/parentchild_chunker:0.0.1@b1a28a27e33fec442ce494da2a7814edd7eb9d646c81f38bccfcf1133d486e40
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/notion_datasource:0.0.1@2dd49c2c3ffff976be8d22efb1ac0f63522a8d0f24ef8c44729d0a50a94ec039
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: ''
  icon_type: emoji
  name: notion-parentchild
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: tool
      id: 1752489759475-source-1752490343805-target
      source: '1752489759475'
      sourceHandle: source
      target: '1752490343805'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752490343805-source-1752477924228-target
      source: '1752490343805'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: hierarchical_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752490343805'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: false
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 1486.2052698032674
        y: 281.3910724383104
      positionAbsolute:
        x: 1486.2052698032674
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Notion数据源
        datasource_name: notion_datasource
        datasource_parameters: {}
        plugin_id: langgenius/notion_datasource
        provider_name: notion_datasource
        provider_type: online_document
        selected: false
        title: Notion数据源
        type: datasource
      height: 52
      id: '1752489759475'
      position:
        x: 736.9082104000458
        y: 281.3910724383104
      positionAbsolute:
        x: 736.9082104000458
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: Parent child chunks result
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input text
            ja_JP: 入力テキスト
            pt_BR: Texto de entrada
            zh_Hans: 输入文本
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_text
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 1024
          form: llm
          human_description:
            en_US: Maximum length for chunking
            ja_JP: チャンク分割の最大長
            pt_BR: Comprimento máximo para divisão
            zh_Hans: 用于分块的最大长度
          label:
            en_US: Maximum Length
            ja_JP: 最大長
            pt_BR: Comprimento Máximo
            zh_Hans: 最大长度
          llm_description: Maximum length allowed per chunk
          max: null
          min: null
          name: max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '


            '
          form: llm
          human_description:
            en_US: Separator used for chunking
            ja_JP: チャンク分割に使用する区切り文字
            pt_BR: Separador usado para divisão
            zh_Hans: 用于分块的分隔符
          label:
            en_US: Chunk Separator
            ja_JP: チャンク区切り文字
            pt_BR: Separador de Divisão
            zh_Hans: 分块分隔符
          llm_description: The separator used to split chunks
          max: null
          min: null
          name: separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 512
          form: llm
          human_description:
            en_US: Maximum length for subchunking
            ja_JP: サブチャンク分割の最大長
            pt_BR: Comprimento máximo para subdivisão
            zh_Hans: 用于子分块的最大长度
          label:
            en_US: Subchunk Maximum Length
            ja_JP: サブチャンク最大長
            pt_BR: Comprimento Máximo de Subdivisão
            zh_Hans: 子分块最大长度
          llm_description: Maximum length allowed per subchunk
          max: null
          min: null
          name: subchunk_max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '. '
          form: llm
          human_description:
            en_US: Separator used for subchunking
            ja_JP: サブチャンク分割に使用する区切り文字
            pt_BR: Separador usado para subdivisão
            zh_Hans: 用于子分块的分隔符
          label:
            en_US: Subchunk Separator
            ja_JP: サブチャンキング用セパレーター
            pt_BR: Separador de Subdivisão
            zh_Hans: 子分块分隔符
          llm_description: The separator used to split subchunks
          max: null
          min: null
          name: subchunk_separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: paragraph
          form: llm
          human_description:
            en_US: Split text into paragraphs based on separator and maximum chunk
              length, using split text as parent block or entire document as parent
              block and directly retrieve.
            ja_JP: セパレーターと最大チャンク長に基づいてテキストを段落に分割し、分割されたテキスト
              を親ブロックとして使用するか、文書全体を親ブロックとして使用して直接取得します。
            pt_BR: Dividir texto em parágrafos com base no separador e no comprimento
              máximo do bloco, usando o texto dividido como bloco pai ou documento
              completo como bloco pai e diretamente recuperá-lo.
            zh_Hans: 根据分隔符和最大块长度将文本拆分为段落，使用拆分文本作为检索的父块或整个文档用作父块并直接检索。
          label:
            en_US: Parent Mode
            ja_JP: 親子モード
            pt_BR: Modo Pai
            zh_Hans: 父块模式
          llm_description: Split text into paragraphs based on separator and maximum
            chunk length, using split text as parent block or entire document as parent
            block and directly retrieve.
          max: null
          min: null
          name: parent_mode
          options:
          - icon: ''
            label:
              en_US: Paragraph
              ja_JP: 段落
              pt_BR: Parágrafo
              zh_Hans: 段落
            value: paragraph
          - icon: ''
            label:
              en_US: Full Document
              ja_JP: 全文
              pt_BR: Documento Completo
              zh_Hans: 全文
            value: full_doc
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: select
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove extra spaces in the text
            ja_JP: テキスト内の余分なスペースを削除するかどうか
            pt_BR: Se deve remover espaços extras no texto
            zh_Hans: 是否移除文本中的多余空格
          label:
            en_US: Remove Extra Spaces
            ja_JP: 余分なスペースを削除
            pt_BR: Remover Espaços Extras
            zh_Hans: 移除多余空格
          llm_description: Whether to remove extra spaces in the text
          max: null
          min: null
          name: remove_extra_spaces
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove URLs and emails in the text
            ja_JP: テキスト内のURLやメールアドレスを削除するかどうか
            pt_BR: Se deve remover URLs e e-mails no texto
            zh_Hans: 是否移除文本中的URL和电子邮件地址
          label:
            en_US: Remove URLs and Emails
            ja_JP: URLとメールアドレスを削除
            pt_BR: Remover URLs e E-mails
            zh_Hans: 移除URL和电子邮件地址
          llm_description: Whether to remove URLs and emails in the text
          max: null
          min: null
          name: remove_urls_emails
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          input_text: ''
          max_length: ''
          parent_mode: ''
          remove_extra_spaces: ''
          remove_urls_emails: ''
          separator: ''
          subchunk_max_length: ''
          subchunk_separator: ''
        provider_id: langgenius/parentchild_chunker/parentchild_chunker
        provider_name: langgenius/parentchild_chunker/parentchild_chunker
        provider_type: builtin
        selected: true
        title: Parent-child Chunker
        tool_configurations: {}
        tool_description: Parent-child Chunk Structure
        tool_label: Parent-child Chunker
        tool_name: parentchild_chunker
        tool_parameters:
          input_text:
            type: mixed
            value: '{{#1752489759475.content#}}'
          max_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          parent_mode:
            type: variable
            value:
            - rag
            - shared
            - parent_mode
          remove_extra_spaces:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
          remove_urls_emails:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          separator:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          subchunk_max_length:
            type: variable
            value:
            - rag
            - shared
            - child_max_chunk_length
          subchunk_separator:
            type: mixed
            value: '{{#rag.shared.child_delimiter#}}'
        type: tool
      height: 52
      id: '1752490343805'
      position:
        x: 1077.0240183162543
        y: 281.3910724383104
      positionAbsolute:
        x: 1077.0240183162543
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -487.2912544090391
      y: -54.7029301848807
      zoom: 0.9994011715768695
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 1024
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n
    label: Child delimiter
    max_length: 199
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: child_delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 512
    label: Child max chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: child_max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: paragraph
    label: Parent mode
    max_length: 48
    options:
    - full_doc
    - paragraph
    placeholder: null
    required: true
    tooltips: null
    type: select
    unit: null
    variable: parent_mode
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/website-crawl-general-economy.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/firecrawl_datasource:0.0.1@f7aed0a26df0e5f4b9555371b5c9fa6db3c7dcf6a46dd1583245697bd90a539a
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/jina_datasource:0.0.1@cf23afb2c3eeccc5a187763a1947f583f0bb10aa56461e512ac4141bf930d608
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: ''
  icon_type: emoji
  name: website-crawl-general-economy
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752491761974-source-1752565435219-target
      source: '1752491761974'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752565402678-source-1752565435219-target
      source: '1752565402678'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752565435219-source-1752569675978-target
      source: '1752565435219'
      sourceHandle: source
      target: '1752569675978'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752569675978-source-1752477924228-target
      source: '1752569675978'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752569675978'
        - result
        indexing_technique: economy
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: keyword_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: true
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 2140.4053851189346
        y: 281.3910724383104
      positionAbsolute:
        x: 2140.4053851189346
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Jina Reader
        datasource_name: jina_reader
        datasource_parameters:
          crawl_sub_pages:
            type: mixed
            value: '{{#rag.1752491761974.jina_crawl_sub_pages#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752491761974'
            - jina_limit
          url:
            type: mixed
            value: '{{#rag.1752491761974.jina_url#}}'
          use_sitemap:
            type: mixed
            value: '{{#rag.1752491761974.jina_use_sitemap#}}'
        plugin_id: langgenius/jina_datasource
        provider_name: jinareader
        provider_type: website_crawl
        selected: false
        title: Jina Reader
        type: datasource
      height: 52
      id: '1752491761974'
      position:
        x: 1067.7526055798794
        y: 281.3910724383104
      positionAbsolute:
        x: 1067.7526055798794
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Firecrawl
        datasource_name: crawl
        datasource_parameters:
          crawl_subpages:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_crawl_sub_pages#}}'
          exclude_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_exclude_paths#}}'
          include_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_include_only_paths#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_limit
          max_depth:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_max_depth
          only_main_content:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_extract_main_content#}}'
          url:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_url#}}'
        plugin_id: langgenius/firecrawl_datasource
        provider_name: firecrawl
        provider_type: website_crawl
        selected: false
        title: Firecrawl
        type: datasource
      height: 52
      id: '1752565402678'
      position:
        x: 1067.7526055798794
        y: 417.32608398342404
      positionAbsolute:
        x: 1067.7526055798794
        y: 417.32608398342404
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752491761974'
          - content
        - - '1752565402678'
          - content
      height: 129
      id: '1752565435219'
      position:
        x: 1505.4306671642219
        y: 281.3910724383104
      positionAbsolute:
        x: 1505.4306671642219
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos pedaços.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: Delimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長
            pt_BR: The chunk overlap length.
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Chunk Overlap Length
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace consecutive spaces, newlines and tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace Consecutive Spaces, Newlines and Tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete all URLs and email addresses
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete All URLs and Email Addresses
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752565435219.output#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752569675978'
      position:
        x: 1807.4306671642219
        y: 281.3910724383104
      positionAbsolute:
        x: 1807.4306671642219
        y: 281.3910724383104
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -707.721097109337
      y: -93.07807382100896
      zoom: 0.9350632198875476
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: jina_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: jina_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: jina_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Use sitemap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: Follow the sitemap to crawl the site. If not, Jina Reader will crawl
      iteratively based on page relevance, yielding fewer but higher-quality pages.
    type: checkbox
    unit: null
    variable: jina_use_sitemap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: true
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: firecrawl_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Max depth
    max_length: 48
    options: []
    placeholder: ''
    required: false
    tooltips: Maximum depth to crawl relative to the entered URL. Depth 0 just scrapes
      the page of the entered url, depth 1 scrapes the url and everything after enteredURL
      + one /, and so on.
    type: number
    unit: null
    variable: firecrawl_max_depth
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Exclude paths
    max_length: 256
    options: []
    placeholder: blog/*, /about/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_exclude_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Include only paths
    max_length: 256
    options: []
    placeholder: articles/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_include_only_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: firecrawl_extract_main_content
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_extract_main_content
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 1024
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 50
    label: chunk_overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: Setting the chunk overlap can maintain the semantic relevance between
      them, enhancing the retrieve effect. It is recommended to set 10%–25% of the
      maximum chunk size.
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: replace_consecutive_spaces
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/website-crawl-general-high-quality.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/firecrawl_datasource:0.0.1@f7aed0a26df0e5f4b9555371b5c9fa6db3c7dcf6a46dd1583245697bd90a539a
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/jina_datasource:0.0.1@cf23afb2c3eeccc5a187763a1947f583f0bb10aa56461e512ac4141bf930d608
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: '#FFF4ED'
  icon_type: emoji
  name: website-crawl-general-high-quality
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752491761974-source-1752565435219-target
      source: '1752491761974'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752565402678-source-1752565435219-target
      source: '1752565402678'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752565435219-source-1752569675978-target
      source: '1752565435219'
      sourceHandle: source
      target: '1752569675978'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752569675978-source-1752477924228-target
      source: '1752569675978'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752569675978'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: false
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 2140.4053851189346
        y: 281.3910724383104
      positionAbsolute:
        x: 2140.4053851189346
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Jina Reader
        datasource_name: jina_reader
        datasource_parameters:
          crawl_sub_pages:
            type: mixed
            value: '{{#rag.1752491761974.jina_crawl_sub_pages#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752491761974'
            - jina_limit
          url:
            type: mixed
            value: '{{#rag.1752491761974.jina_url#}}'
          use_sitemap:
            type: mixed
            value: '{{#rag.1752491761974.jina_use_sitemap#}}'
        plugin_id: langgenius/jina_datasource
        provider_name: jinareader
        provider_type: website_crawl
        selected: false
        title: Jina Reader
        type: datasource
      height: 52
      id: '1752491761974'
      position:
        x: 1067.7526055798794
        y: 281.3910724383104
      positionAbsolute:
        x: 1067.7526055798794
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Firecrawl
        datasource_name: crawl
        datasource_parameters:
          crawl_subpages:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_crawl_sub_pages#}}'
          exclude_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_exclude_paths#}}'
          include_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_include_only_paths#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_limit
          max_depth:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_max_depth
          only_main_content:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_extract_main_content#}}'
          url:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_url#}}'
        plugin_id: langgenius/firecrawl_datasource
        provider_name: firecrawl
        provider_type: website_crawl
        selected: false
        title: Firecrawl
        type: datasource
      height: 52
      id: '1752565402678'
      position:
        x: 1067.7526055798794
        y: 417.32608398342404
      positionAbsolute:
        x: 1067.7526055798794
        y: 417.32608398342404
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752491761974'
          - content
        - - '1752565402678'
          - content
      height: 129
      id: '1752565435219'
      position:
        x: 1505.4306671642219
        y: 281.3910724383104
      positionAbsolute:
        x: 1505.4306671642219
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos pedaços.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: Delimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長。
            pt_BR: The chunk overlap length.
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Chunk Overlap Length
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace consecutive spaces, newlines and tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace Consecutive Spaces, Newlines and Tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete all URLs and email addresses
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete All URLs and Email Addresses
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker  
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752565435219.output#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752569675978'
      position:
        x: 1807.4306671642219
        y: 281.3910724383104
      positionAbsolute:
        x: 1807.4306671642219
        y: 281.3910724383104
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -707.721097109337
      y: -93.07807382100896
      zoom: 0.9350632198875476
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: jina_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: jina_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: jina_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Use sitemap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: Follow the sitemap to crawl the site. If not, Jina Reader will crawl
      iteratively based on page relevance, yielding fewer but higher-quality pages.
    type: checkbox
    unit: null
    variable: jina_use_sitemap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: true
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: firecrawl_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Max depth
    max_length: 48
    options: []
    placeholder: ''
    required: false
    tooltips: Maximum depth to crawl relative to the entered URL. Depth 0 just scrapes
      the page of the entered url, depth 1 scrapes the url and everything after enteredURL
      + one /, and so on.
    type: number
    unit: null
    variable: firecrawl_max_depth
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Exclude paths
    max_length: 256
    options: []
    placeholder: blog/*, /about/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_exclude_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Include only paths
    max_length: 256
    options: []
    placeholder: articles/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_include_only_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: firecrawl_extract_main_content
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_extract_main_content
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 1024
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 50
    label: chunk_overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: Setting the chunk overlap can maintain the semantic relevance between
      them, enhancing the retrieve effect. It is recommended to set 10%–25% of the
      maximum chunk size.
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: replace_consecutive_spaces
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/website-crawl-parentchild.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/parentchild_chunker:0.0.1@b1a28a27e33fec442ce494da2a7814edd7eb9d646c81f38bccfcf1133d486e40
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/firecrawl_datasource:0.0.1@f7aed0a26df0e5f4b9555371b5c9fa6db3c7dcf6a46dd1583245697bd90a539a
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/jina_datasource:0.0.1@cf23afb2c3eeccc5a187763a1947f583f0bb10aa56461e512ac4141bf930d608
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: ''
  icon_type: emoji
  name: website-crawl-parentchild
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752490343805-source-1752477924228-target
      source: '1752490343805'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752491761974-source-1752565435219-target
      source: '1752491761974'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752565435219-source-1752490343805-target
      source: '1752565435219'
      sourceHandle: source
      target: '1752490343805'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: datasource
        targetType: variable-aggregator
      id: 1752565402678-source-1752565435219-target
      source: '1752565402678'
      sourceHandle: source
      target: '1752565435219'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: hierarchical_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752490343805'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: false
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 2215.5544306817387
        y: 281.3910724383104
      positionAbsolute:
        x: 2215.5544306817387
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: Parent child chunks result
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input text
            ja_JP: 入力テキスト
            pt_BR: Texto de entrada
            zh_Hans: 输入文本
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_text
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 1024
          form: llm
          human_description:
            en_US: Maximum length for chunking
            ja_JP: チャンク分割の最大長
            pt_BR: Comprimento máximo para divisão
            zh_Hans: 用于分块的最大长度
          label:
            en_US: Maximum Length
            ja_JP: 最大長
            pt_BR: Comprimento Máximo
            zh_Hans: 最大长度
          llm_description: Maximum length allowed per chunk
          max: null
          min: null
          name: max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '


            '
          form: llm
          human_description:
            en_US: Separator used for chunking
            ja_JP: チャンク分割に使用する区切り文字
            pt_BR: Separador usado para divisão
            zh_Hans: 用于分块的分隔符
          label:
            en_US: Chunk Separator
            ja_JP: チャンク区切り文字
            pt_BR: Separador de Divisão
            zh_Hans: 分块分隔符
          llm_description: The separator used to split chunks
          max: null
          min: null
          name: separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 512
          form: llm
          human_description:
            en_US: Maximum length for subchunking
            ja_JP: サブチャンク分割の最大長
            pt_BR: Comprimento máximo para subdivisão
            zh_Hans: 用于子分块的最大长度
          label:
            en_US: Subchunk Maximum Length
            ja_JP: サブチャンク最大長
            pt_BR: Comprimento Máximo de Subdivisão
            zh_Hans: 子分块最大长度
          llm_description: Maximum length allowed per subchunk
          max: null
          min: null
          name: subchunk_max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '. '
          form: llm
          human_description:
            en_US: Separator used for subchunking
            ja_JP: サブチャンク分割に使用する区切り文字
            pt_BR: Separador usado para subdivisão
            zh_Hans: 用于子分块的分隔符
          label:
            en_US: Subchunk Separator
            ja_JP: サブチャンキング用セパレーター
            pt_BR: Separador de Subdivisão
            zh_Hans: 子分块分隔符
          llm_description: The separator used to split subchunks
          max: null
          min: null
          name: subchunk_separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: paragraph
          form: llm
          human_description:
            en_US: Split text into paragraphs based on separator and maximum chunk
              length, using split text as parent block or entire document as parent
              block and directly retrieve.
            ja_JP: セパレーターと最大チャンク長に基づいてテキストを段落に分割し、分割されたテキスト
              を親ブロックとして使用するか、文書全体を親ブロックとして使用して直接取得します。
            pt_BR: Dividir texto em parágrafos com base no separador e no comprimento
              máximo do bloco, usando o texto dividido como bloco pai ou documento
              completo como bloco pai e diretamente recuperá-lo.
            zh_Hans: 根据分隔符和最大块长度将文本拆分为段落，使用拆分文本作为检索的父块或整个文档用作父块并直接检索。
          label:
            en_US: Parent Mode
            ja_JP: 親子モード
            pt_BR: Modo Pai
            zh_Hans: 父块模式
          llm_description: Split text into paragraphs based on separator and maximum
            chunk length, using split text as parent block or entire document as parent
            block and directly retrieve.
          max: null
          min: null
          name: parent_mode
          options:
          - icon: ''
            label:
              en_US: Paragraph
              ja_JP: 段落
              pt_BR: Parágrafo
              zh_Hans: 段落
            value: paragraph
          - icon: ''
            label:
              en_US: Full Document
              ja_JP: 全文
              pt_BR: Documento Completo
              zh_Hans: 全文
            value: full_doc
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: select
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove extra spaces in the text
            ja_JP: テキスト内の余分なスペースを削除するかどうか
            pt_BR: Se deve remover espaços extras no texto
            zh_Hans: 是否移除文本中的多余空格
          label:
            en_US: Remove Extra Spaces
            ja_JP: 余分なスペースを削除
            pt_BR: Remover Espaços Extras
            zh_Hans: 移除多余空格
          llm_description: Whether to remove extra spaces in the text
          max: null
          min: null
          name: remove_extra_spaces
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove URLs and emails in the text
            ja_JP: テキスト内のURLやメールアドレスを削除するかどうか
            pt_BR: Se deve remover URLs e e-mails no texto
            zh_Hans: 是否移除文本中的URL和电子邮件地址
          label:
            en_US: Remove URLs and Emails
            ja_JP: URLとメールアドレスを削除
            pt_BR: Remover URLs e E-mails
            zh_Hans: 移除URL和电子邮件地址
          llm_description: Whether to remove URLs and emails in the text
          max: null
          min: null
          name: remove_urls_emails
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          input_text: ''
          max_length: ''
          parent_mode: ''
          remove_extra_spaces: ''
          remove_urls_emails: ''
          separator: ''
          subchunk_max_length: ''
          subchunk_separator: ''
        provider_id: langgenius/parentchild_chunker/parentchild_chunker
        provider_name: langgenius/parentchild_chunker/parentchild_chunker
        provider_type: builtin
        selected: true
        title: Parent-child Chunker
        tool_configurations: {}
        tool_description: Parent-child Chunk Structure
        tool_label: Parent-child Chunker
        tool_name: parentchild_chunker
        tool_parameters:
          input_text:
            type: mixed
            value: '{{#1752565435219.output#}}'
          max_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          parent_mode:
            type: variable
            value:
            - rag
            - shared
            - parent_mode
          remove_extra_spaces:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
          remove_urls_emails:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          separator:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          subchunk_max_length:
            type: variable
            value:
            - rag
            - shared
            - child_max_chunk_length
          subchunk_separator:
            type: mixed
            value: '{{#rag.shared.child_delimiter#}}'
        type: tool
      height: 52
      id: '1752490343805'
      position:
        x: 1853.5260563244174
        y: 281.3910724383104
      positionAbsolute:
        x: 1853.5260563244174
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Jina Reader
        datasource_name: jina_reader
        datasource_parameters:
          crawl_sub_pages:
            type: mixed
            value: '{{#rag.1752491761974.jina_crawl_sub_pages#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752491761974'
            - jina_limit
          url:
            type: mixed
            value: '{{#rag.1752491761974.jina_url#}}'
          use_sitemap:
            type: mixed
            value: '{{#rag.1752491761974.jina_use_sitemap#}}'
        plugin_id: langgenius/jina_datasource
        provider_name: jinareader
        provider_type: website_crawl
        selected: false
        title: Jina Reader
        type: datasource
      height: 52
      id: '1752491761974'
      position:
        x: 1067.7526055798794
        y: 281.3910724383104
      positionAbsolute:
        x: 1067.7526055798794
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Firecrawl
        datasource_name: crawl
        datasource_parameters:
          crawl_subpages:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_crawl_sub_pages#}}'
          exclude_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_exclude_paths#}}'
          include_paths:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_include_only_paths#}}'
          limit:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_limit
          max_depth:
            type: variable
            value:
            - rag
            - '1752565402678'
            - firecrawl_max_depth
          only_main_content:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_extract_main_content#}}'
          url:
            type: mixed
            value: '{{#rag.1752565402678.firecrawl_url#}}'
        plugin_id: langgenius/firecrawl_datasource
        provider_name: firecrawl
        provider_type: website_crawl
        selected: false
        title: Firecrawl
        type: datasource
      height: 52
      id: '1752565402678'
      position:
        x: 1067.7526055798794
        y: 417.32608398342404
      positionAbsolute:
        x: 1067.7526055798794
        y: 417.32608398342404
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752491761974'
          - content
        - - '1752565402678'
          - content
      height: 129
      id: '1752565435219'
      position:
        x: 1505.4306671642219
        y: 281.3910724383104
      positionAbsolute:
        x: 1505.4306671642219
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -826.1791044466438
      y: -71.91725474841303
      zoom: 0.9980166672552107
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: jina_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: jina_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: jina_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752491761974'
    default_value: null
    label: Use sitemap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: Follow the sitemap to crawl the site. If not, Jina Reader will crawl
      iteratively based on page relevance, yielding fewer but higher-quality pages.
    type: checkbox
    unit: null
    variable: jina_use_sitemap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: URL
    max_length: 256
    options: []
    placeholder: https://docs.dify.ai/en/
    required: true
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_url
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: true
    label: Crawl sub-pages
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_crawl_sub_pages
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: 10
    label: Limit
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: null
    variable: firecrawl_limit
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Max depth
    max_length: 48
    options: []
    placeholder: ''
    required: false
    tooltips: Maximum depth to crawl relative to the entered URL. Depth 0 just scrapes
      the page of the entered url, depth 1 scrapes the url and everything after enteredURL
      + one /, and so on.
    type: number
    unit: null
    variable: firecrawl_max_depth
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Exclude paths
    max_length: 256
    options: []
    placeholder: blog/*, /about/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_exclude_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: Include only paths
    max_length: 256
    options: []
    placeholder: articles/*
    required: false
    tooltips: null
    type: text-input
    unit: null
    variable: firecrawl_include_only_paths
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: '1752565402678'
    default_value: null
    label: firecrawl_extract_main_content
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: firecrawl_extract_main_content
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 1024
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n
    label: Child delimiter
    max_length: 199
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: child_delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 512
    label: Child max chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: child_max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: paragraph
    label: Parent mode
    max_length: 48
    options:
    - full_doc
    - paragraph
    placeholder: null
    required: true
    tooltips: null
    type: select
    unit: null
    variable: parent_mode
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/notion-general-high-quality.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/notion_datasource:0.0.1@2dd49c2c3ffff976be8d22efb1ac0f63522a8d0f24ef8c44729d0a50a94ec039
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: '#FFF4ED'
  icon_type: emoji
  name: notion-general-high-quality
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752482151668-source-1752477924228-target
      source: '1752482151668'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: tool
      id: 1752489759475-source-1752482151668-target
      source: '1752489759475'
      sourceHandle: source
      target: '1752482151668'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752482151668'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: true
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 1444.5503479271906
        y: 281.3910724383104
      positionAbsolute:
        x: 1444.5503479271906
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos pedaços.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: Delimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長
            pt_BR: The chunk overlap length.
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Chunk Overlap Length
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace consecutive spaces, newlines and tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace Consecutive Spaces, Newlines and Tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete all URLs and email addresses
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete All URLs and Email Addresses
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752489759475.content#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752482151668'
      position:
        x: 1063.6922916384628
        y: 281.3910724383104
      positionAbsolute:
        x: 1063.6922916384628
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: Notion数据源
        datasource_name: notion_datasource
        datasource_parameters: {}
        plugin_id: langgenius/notion_datasource
        provider_name: notion_datasource
        provider_type: online_document
        selected: false
        title: Notion数据源
        type: datasource
      height: 52
      id: '1752489759475'
      position:
        x: 736.9082104000458
        y: 281.3910724383104
      positionAbsolute:
        x: 736.9082104000458
        y: 281.3910724383104
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: -838.569649323166
      y: -168.94656489167426
      zoom: 1.286925643857699
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Chunk overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/file-parentchild.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/parentchild_chunker:0.0.1@b1a28a27e33fec442ce494da2a7814edd7eb9d646c81f38bccfcf1133d486e40
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/dify_extractor:0.0.1@50103421d4e002f059b662d21ad2d7a1cf34869abdbe320299d7e382516ebb1c
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: '#FFF4ED'
  icon_type: emoji
  name: file-parentchild
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: if-else
      id: 1752479895761-source-1752481129417-target
      source: '1752479895761'
      sourceHandle: source
      target: '1752481129417'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: tool
      id: 1752481129417-24e47cad-f1e2-4f74-9884-3f49d5bb37b7-1752480460682-target
      source: '1752481129417'
      sourceHandle: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
      target: '1752480460682'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: document-extractor
      id: 1752481129417-false-1752481112180-target
      source: '1752481129417'
      sourceHandle: 'false'
      target: '1752481112180'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: variable-aggregator
      id: 1752480460682-source-1752482022496-target
      source: '1752480460682'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: document-extractor
        targetType: variable-aggregator
      id: 1752481112180-source-1752482022496-target
      source: '1752481112180'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752482022496-source-1752575473519-target
      source: '1752482022496'
      sourceHandle: source
      target: '1752575473519'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752575473519-source-1752477924228-target
      source: '1752575473519'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: hierarchical_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752575473519'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: false
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 994.3774545394483
        y: 281.3910724383104
      positionAbsolute:
        x: 994.3774545394483
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: File
        datasource_name: upload-file
        datasource_parameters: {}
        fileExtensions:
        - txt
        - markdown
        - mdx
        - pdf
        - html
        - xlsx
        - xls
        - vtt
        - properties
        - doc
        - docx
        - csv
        - eml
        - msg
        - pptx
        - xml
        - epub
        - ppt
        - md
        plugin_id: langgenius/file
        provider_name: file
        provider_type: local_file
        selected: false
        title: File
        type: datasource
      height: 52
      id: '1752479895761'
      position:
        x: -839.8603427660498
        y: 251.3910724383104
      positionAbsolute:
        x: -839.8603427660498
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            documents:
              description: the documents extracted from the file
              items:
                type: object
              type: array
            images:
              description: The images extracted from the file
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: the file to be parsed(support pdf, ppt, pptx, doc, docx, png, jpg,
              jpeg)
            ja_JP: 解析するファイル(pdf, ppt, pptx, doc, docx, png, jpg, jpegをサポート)
            pt_BR: o arquivo a ser analisado (suporta pdf, ppt, pptx, doc, docx, png,
              jpg, jpeg)
            zh_Hans: 用于解析的文件(支持 pdf, ppt, pptx, doc, docx, png, jpg, jpeg)
          label:
            en_US: file
            ja_JP: ファイル
            pt_BR: arquivo
            zh_Hans: file
          llm_description: the file to be parsed (support pdf, ppt, pptx, doc, docx,
            png, jpg, jpeg)
          max: null
          min: null
          name: file
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: file
        params:
          file: ''
        provider_id: langgenius/dify_extractor/dify_extractor
        provider_name: langgenius/dify_extractor/dify_extractor
        provider_type: builtin
        selected: false
        title: Dify Extractor
        tool_configurations: {}
        tool_description: Dify Extractor
        tool_label: Dify Extractor
        tool_name: dify_extractor
        tool_parameters:
          file:
            type: variable
            value:
            - '1752479895761'
            - file
        type: tool
      height: 52
      id: '1752480460682'
      position:
        x: -108.28652292656551
        y: 281.3910724383104
      positionAbsolute:
        x: -108.28652292656551
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_array_file: false
        selected: false
        title: 文档提取器
        type: document-extractor
        variable_selector:
        - '1752479895761'
        - file
      height: 90
      id: '1752481112180'
      position:
        x: -108.28652292656551
        y: 390.6576481692478
      positionAbsolute:
        x: -108.28652292656551
        y: 390.6576481692478
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        cases:
        - case_id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          conditions:
          - comparison_operator: is
            id: 9da88d93-3ff6-463f-abfd-6bcafbf2554d
            value: .xlsx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: d0e88f5e-dfe3-4bae-af0c-dbec267500de
            value: .xls
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: a957e91e-1ed7-4c6b-9c80-2f0948858f1d
            value: .md
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 870c3c39-8d3f-474a-ab8b-9c0ccf53db73
            value: .markdown
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: f9541513-1e71-4dc1-9db5-35dc84a39e3c
            value: .mdx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 4c7f455b-ac20-40ca-9495-6cc44ffcb35d
            value: .html
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 2e12d9c7-8057-4a09-8851-f9fd1d0718d1
            value: .htm
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 73a995a9-d8b9-4aef-89f7-306e2ddcbce2
            value: .docx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 8a2e8772-0426-458b-a1f9-9eaaec0f27c8
            value: .csv
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: aa2cb6b6-a2fc-462a-a9f5-c9c3f33a1602
            value: .txt
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          logical_operator: or
        selected: false
        title: IF/ELSE
        type: if-else
      height: 358
      id: '1752481129417'
      position:
        x: -512.2335487893622
        y: 251.3910724383104
      positionAbsolute:
        x: -512.2335487893622
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        advanced_settings:
          group_enabled: false
          groups:
          - groupId: f4cf07b4-914d-4544-8ef8-0c5d9e4f21a7
            group_name: Group1
            output_type: string
            variables:
            - - '1752481112180'
              - text
            - - '1752480460682'
              - text
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752481112180'
          - text
        - - '1752480460682'
          - text
      height: 129
      id: '1752482022496'
      position:
        x: 319.441649575055
        y: 281.3910724383104
      positionAbsolute:
        x: 319.441649575055
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: Parent child chunks result
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input text
            ja_JP: 入力テキスト
            pt_BR: Texto de entrada
            zh_Hans: 输入文本
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_text
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 1024
          form: llm
          human_description:
            en_US: Maximum length for chunking
            ja_JP: チャンク分割の最大長
            pt_BR: Comprimento máximo para divisão
            zh_Hans: 用于分块的最大长度
          label:
            en_US: Maximum Length
            ja_JP: 最大長
            pt_BR: Comprimento Máximo
            zh_Hans: 最大长度
          llm_description: Maximum length allowed per chunk
          max: null
          min: null
          name: max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '


            '
          form: llm
          human_description:
            en_US: Separator used for chunking
            ja_JP: チャンク分割に使用する区切り文字
            pt_BR: Separador usado para divisão
            zh_Hans: 用于分块的分隔符
          label:
            en_US: Chunk Separator
            ja_JP: チャンク区切り文字
            pt_BR: Separador de Divisão
            zh_Hans: 分块分隔符
          llm_description: The separator used to split chunks
          max: null
          min: null
          name: separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: 512
          form: llm
          human_description:
            en_US: Maximum length for subchunking
            ja_JP: サブチャンク分割の最大長
            pt_BR: Comprimento máximo para subdivisão
            zh_Hans: 用于子分块的最大长度
          label:
            en_US: Subchunk Maximum Length
            ja_JP: サブチャンク最大長
            pt_BR: Comprimento Máximo de Subdivisão
            zh_Hans: 子分块最大长度
          llm_description: Maximum length allowed per subchunk
          max: null
          min: null
          name: subchunk_max_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: '. '
          form: llm
          human_description:
            en_US: Separator used for subchunking
            ja_JP: サブチャンク分割に使用する区切り文字
            pt_BR: Separador usado para subdivisão
            zh_Hans: 用于子分块的分隔符
          label:
            en_US: Subchunk Separator
            ja_JP: サブチャンキング用セパレーター
            pt_BR: Separador de Subdivisão
            zh_Hans: 子分块分隔符
          llm_description: The separator used to split subchunks
          max: null
          min: null
          name: subchunk_separator
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: paragraph
          form: llm
          human_description:
            en_US: Split text into paragraphs based on separator and maximum chunk
              length, using split text as parent block or entire document as parent
              block and directly retrieve.
            ja_JP: セパレーターと最大チャンク長に基づいてテキストを段落に分割し、分割されたテキスト
              を親ブロックとして使用するか、文書全体を親ブロックとして使用して直接取得します。
            pt_BR: Dividir texto em parágrafos com base no separador e no comprimento
              máximo do bloco, usando o texto dividido como bloco pai ou documento
              completo como bloco pai e diretamente recuperá-lo.
            zh_Hans: 根据分隔符和最大块长度将文本拆分为段落，使用拆分文本作为检索的父块或整个文档用作父块并直接检索。
          label:
            en_US: Parent Mode
            ja_JP: 親子モード
            pt_BR: Modo Pai
            zh_Hans: 父块模式
          llm_description: Split text into paragraphs based on separator and maximum
            chunk length, using split text as parent block or entire document as parent
            block and directly retrieve.
          max: null
          min: null
          name: parent_mode
          options:
          - icon: ''
            label:
              en_US: Paragraph
              ja_JP: 段落
              pt_BR: Parágrafo
              zh_Hans: 段落
            value: paragraph
          - icon: ''
            label:
              en_US: Full Document
              ja_JP: 全文
              pt_BR: Documento Completo
              zh_Hans: 全文
            value: full_doc
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: select
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove extra spaces in the text
            ja_JP: テキスト内の余分なスペースを削除するかどうか
            pt_BR: Se deve remover espaços extras no texto
            zh_Hans: 是否移除文本中的多余空格
          label:
            en_US: Remove Extra Spaces
            ja_JP: 余分なスペースを削除
            pt_BR: Remover Espaços Extras
            zh_Hans: 移除多余空格
          llm_description: Whether to remove extra spaces in the text
          max: null
          min: null
          name: remove_extra_spaces
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: 0
          form: llm
          human_description:
            en_US: Whether to remove URLs and emails in the text
            ja_JP: テキスト内のURLやメールアドレスを削除するかどうか
            pt_BR: Se deve remover URLs e e-mails no texto
            zh_Hans: 是否移除文本中的URL和电子邮件地址
          label:
            en_US: Remove URLs and Emails
            ja_JP: URLとメールアドレスを削除
            pt_BR: Remover URLs e E-mails
            zh_Hans: 移除URL和电子邮件地址
          llm_description: Whether to remove URLs and emails in the text
          max: null
          min: null
          name: remove_urls_emails
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          input_text: ''
          max_length: ''
          parent_mode: ''
          remove_extra_spaces: ''
          remove_urls_emails: ''
          separator: ''
          subchunk_max_length: ''
          subchunk_separator: ''
        provider_id: langgenius/parentchild_chunker/parentchild_chunker
        provider_name: langgenius/parentchild_chunker/parentchild_chunker
        provider_type: builtin
        selected: false
        title: Parent-child Chunker
        tool_configurations: {}
        tool_description: Parent-child Chunk Structure
        tool_label: Parent-child Chunker
        tool_name: parentchild_chunker
        tool_parameters:
          input_text:
            type: mixed
            value: '{{#1752482022496.output#}}'
          max_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          parent_mode:
            type: variable
            value:
            - rag
            - shared
            - parent_mode
          remove_extra_spaces:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
          remove_urls_emails:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          separator:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          subchunk_max_length:
            type: variable
            value:
            - rag
            - shared
            - child_max_chunk_length
          subchunk_separator:
            type: mixed
            value: '{{#rag.shared.child_delimiter#}}'
        type: tool
      height: 52
      id: '1752575473519'
      position:
        x: 637.9241611063885
        y: 281.3910724383104
      positionAbsolute:
        x: 637.9241611063885
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: 948.6766333808323
      y: -102.06757184183238
      zoom: 0.8375774577380971
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 256
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 1024
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n
    label: Child delimiter
    max_length: 256
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: child_delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: 512
    label: Child max chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: child_max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: paragraph
    label: Parent mode
    max_length: 48
    options:
    - full_doc
    - paragraph
    placeholder: null
    required: true
    tooltips: null
    type: select
    unit: null
    variable: parent_mode
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/file-general-economy.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/dify_extractor:0.0.1@50103421d4e002f059b662d21ad2d7a1cf34869abdbe320299d7e382516ebb1c
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: ''
  icon_type: emoji
  name: file-general-economy
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: if-else
      id: 1752479895761-source-1752481129417-target
      source: '1752479895761'
      sourceHandle: source
      target: '1752481129417'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: tool
      id: 1752481129417-24e47cad-f1e2-4f74-9884-3f49d5bb37b7-1752480460682-target
      source: '1752481129417'
      sourceHandle: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
      target: '1752480460682'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: document-extractor
      id: 1752481129417-false-1752481112180-target
      source: '1752481129417'
      sourceHandle: 'false'
      target: '1752481112180'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: variable-aggregator
      id: 1752480460682-source-1752482022496-target
      source: '1752480460682'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: document-extractor
        targetType: variable-aggregator
      id: 1752481112180-source-1752482022496-target
      source: '1752481112180'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752482022496-source-1752482151668-target
      source: '1752482022496'
      sourceHandle: source
      target: '1752482151668'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752482151668-source-1752477924228-target
      source: '1752482151668'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752482151668'
        - result
        indexing_technique: economy
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: keyword_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: true
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 1076.4656678451215
        y: 281.3910724383104
      positionAbsolute:
        x: 1076.4656678451215
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: File
        datasource_name: upload-file
        datasource_parameters: {}
        fileExtensions:
        - txt
        - markdown
        - mdx
        - pdf
        - html
        - xlsx
        - xls
        - vtt
        - properties
        - doc
        - docx
        - csv
        - eml
        - msg
        - pptx
        - xml
        - epub
        - ppt
        - md
        plugin_id: langgenius/file
        provider_name: file
        provider_type: local_file
        selected: false
        title: File
        type: datasource
      height: 52
      id: '1752479895761'
      position:
        x: -839.8603427660498
        y: 251.3910724383104
      positionAbsolute:
        x: -839.8603427660498
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            documents:
              description: the documents extracted from the file
              items:
                type: object
              type: array
            images:
              description: The images extracted from the file
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: the file to be parsed(support pdf, ppt, pptx, doc, docx, png, jpg,
              jpeg)
            ja_JP: 解析するファイル(pdf, ppt, pptx, doc, docx, png, jpg, jpegをサポート)
            pt_BR: o arquivo a ser analisado (suporta pdf, ppt, pptx, doc, docx, png,
              jpg, jpeg)
            zh_Hans: 用于解析的文件(支持 pdf, ppt, pptx, doc, docx, png, jpg, jpeg)
          label:
            en_US: file
            ja_JP: ファイル
            pt_BR: arquivo
            zh_Hans: file
          llm_description: the file to be parsed (support pdf, ppt, pptx, doc, docx,
            png, jpg, jpeg)
          max: null
          min: null
          name: file
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: file
        params:
          file: ''
        provider_id: langgenius/dify_extractor/dify_extractor
        provider_name: langgenius/dify_extractor/dify_extractor
        provider_type: builtin
        selected: false
        title: Dify Extractor
        tool_configurations: {}
        tool_description: Dify Extractor
        tool_label: Dify Extractor
        tool_name: dify_extractor
        tool_parameters:
          file:
            type: variable
            value:
            - '1752479895761'
            - file
        type: tool
      height: 52
      id: '1752480460682'
      position:
        x: -108.28652292656551
        y: 281.3910724383104
      positionAbsolute:
        x: -108.28652292656551
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_array_file: false
        selected: false
        title: 文档提取器
        type: document-extractor
        variable_selector:
        - '1752479895761'
        - file
      height: 90
      id: '1752481112180'
      position:
        x: -108.28652292656551
        y: 390.6576481692478
      positionAbsolute:
        x: -108.28652292656551
        y: 390.6576481692478
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        cases:
        - case_id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          conditions:
          - comparison_operator: is
            id: 9da88d93-3ff6-463f-abfd-6bcafbf2554d
            value: .xlsx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: d0e88f5e-dfe3-4bae-af0c-dbec267500de
            value: .xls
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: a957e91e-1ed7-4c6b-9c80-2f0948858f1d
            value: .md
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 870c3c39-8d3f-474a-ab8b-9c0ccf53db73
            value: .markdown
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: f9541513-1e71-4dc1-9db5-35dc84a39e3c
            value: .mdx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 4c7f455b-ac20-40ca-9495-6cc44ffcb35d
            value: .html
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 2e12d9c7-8057-4a09-8851-f9fd1d0718d1
            value: .htm
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 73a995a9-d8b9-4aef-89f7-306e2ddcbce2
            value: .docx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 8a2e8772-0426-458b-a1f9-9eaaec0f27c8
            value: .csv
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: aa2cb6b6-a2fc-462a-a9f5-c9c3f33a1602
            value: .txt
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          logical_operator: or
        selected: false
        title: IF/ELSE
        type: if-else
      height: 358
      id: '1752481129417'
      position:
        x: -489.57009543377865
        y: 251.3910724383104
      positionAbsolute:
        x: -489.57009543377865
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        advanced_settings:
          group_enabled: false
          groups:
          - groupId: f4cf07b4-914d-4544-8ef8-0c5d9e4f21a7
            group_name: Group1
            output_type: string
            variables:
            - - '1752481112180'
              - text
            - - '1752480460682'
              - text
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752481112180'
          - text
        - - '1752480460682'
          - text
      height: 129
      id: '1752482022496'
      position:
        x: 319.441649575055
        y: 281.3910724383104
      positionAbsolute:
        x: 319.441649575055
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos blocos.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: DDelimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長
            pt_BR: O comprimento de sobreposição dos fragmentos
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Comprimento de sobreposição do bloco
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Substituir espaços consecutivos, novas linhas e tabulações
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Substituir espaços consecutivos, novas linhas e tabulações
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Excluir todos os URLs e endereços de e-mail
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Excluir todos os URLs e endereços de e-mail
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752482022496.output#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752482151668'
      position:
        x: 693.5300771507484
        y: 281.3910724383104
      positionAbsolute:
        x: 693.5300771507484
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: 701.4999626224237
      y: 128.33739021504016
      zoom: 0.48941689643726966
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Chunk overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/transform/file-general-high-quality.yml
```yml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/general_chunker:0.0.1@e3da408b7277866404c3f884d599261f9d0b9003ea4ef7eb3b64489bdf39d18b
- current_identifier: null
  type: marketplace
  value:
    plugin_unique_identifier: langgenius/dify_extractor:0.0.1@50103421d4e002f059b662d21ad2d7a1cf34869abdbe320299d7e382516ebb1c
kind: rag_pipeline
rag_pipeline:
  description: ''
  icon: 📙
  icon_background: '#FFF4ED'
  icon_type: emoji
  name: file-general-high-quality
version: 0.1.0
workflow:
  conversation_variables: []
  environment_variables: []
  features: {}
  graph:
    edges:
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: datasource
        targetType: if-else
      id: 1752479895761-source-1752481129417-target
      source: '1752479895761'
      sourceHandle: source
      target: '1752481129417'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: tool
      id: 1752481129417-24e47cad-f1e2-4f74-9884-3f49d5bb37b7-1752480460682-target
      source: '1752481129417'
      sourceHandle: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
      target: '1752480460682'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: if-else
        targetType: document-extractor
      id: 1752481129417-false-1752481112180-target
      source: '1752481129417'
      sourceHandle: 'false'
      target: '1752481112180'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: variable-aggregator
      id: 1752480460682-source-1752482022496-target
      source: '1752480460682'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInLoop: false
        sourceType: document-extractor
        targetType: variable-aggregator
      id: 1752481112180-source-1752482022496-target
      source: '1752481112180'
      sourceHandle: source
      target: '1752482022496'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: variable-aggregator
        targetType: tool
      id: 1752482022496-source-1752482151668-target
      source: '1752482022496'
      sourceHandle: source
      target: '1752482151668'
      targetHandle: target
      type: custom
      zIndex: 0
    - data:
        isInIteration: false
        isInLoop: false
        sourceType: tool
        targetType: knowledge-index
      id: 1752482151668-source-1752477924228-target
      source: '1752482151668'
      sourceHandle: source
      target: '1752477924228'
      targetHandle: target
      type: custom
      zIndex: 0
    nodes:
    - data:
        chunk_structure: text_model
        embedding_model: text-embedding-ada-002
        embedding_model_provider: langgenius/openai/openai
        index_chunk_variable_selector:
        - '1752482151668'
        - result
        indexing_technique: high_quality
        keyword_number: 10
        retrieval_model:
          score_threshold: 0.5
          score_threshold_enabled: false
          search_method: semantic_search
          top_k: 3
          vector_setting:
            embedding_model_name: text-embedding-ada-002
            embedding_provider_name: langgenius/openai/openai
        selected: false
        title: Knowledge Base
        type: knowledge-index
      height: 114
      id: '1752477924228'
      position:
        x: 1076.4656678451215
        y: 281.3910724383104
      positionAbsolute:
        x: 1076.4656678451215
        y: 281.3910724383104
      selected: true
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        datasource_configurations: {}
        datasource_label: File
        datasource_name: upload-file
        datasource_parameters: {}
        fileExtensions:
        - txt
        - markdown
        - mdx
        - pdf
        - html
        - xlsx
        - xls
        - vtt
        - properties
        - doc
        - docx
        - csv
        - eml
        - msg
        - pptx
        - xml
        - epub
        - ppt
        - md
        plugin_id: langgenius/file
        provider_name: file
        provider_type: local_file
        selected: false
        title: File
        type: datasource
      height: 52
      id: '1752479895761'
      position:
        x: -839.8603427660498
        y: 251.3910724383104
      positionAbsolute:
        x: -839.8603427660498
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            documents:
              description: the documents extracted from the file
              items:
                type: object
              type: array
            images:
              description: The images extracted from the file
              items:
                type: object
              type: array
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: the file to be parsed(support pdf, ppt, pptx, doc, docx, png, jpg,
              jpeg)
            ja_JP: 解析するファイル(pdf, ppt, pptx, doc, docx, png, jpg, jpegをサポート)
            pt_BR: o arquivo a ser analisado (suporta pdf, ppt, pptx, doc, docx, png,
              jpg, jpeg)
            zh_Hans: 用于解析的文件(支持 pdf, ppt, pptx, doc, docx, png, jpg, jpeg)
          label:
            en_US: file
            ja_JP: ファイル
            pt_BR: arquivo
            zh_Hans: file
          llm_description: the file to be parsed (support pdf, ppt, pptx, doc, docx,
            png, jpg, jpeg)
          max: null
          min: null
          name: file
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: file
        params:
          file: ''
        provider_id: langgenius/dify_extractor/dify_extractor
        provider_name: langgenius/dify_extractor/dify_extractor
        provider_type: builtin
        selected: false
        title: Dify Extractor
        tool_configurations: {}
        tool_description: Dify Extractor
        tool_label: Dify Extractor
        tool_name: dify_extractor
        tool_parameters:
          file:
            type: variable
            value:
            - '1752479895761'
            - file
        type: tool
      height: 52
      id: '1752480460682'
      position:
        x: -108.28652292656551
        y: 281.3910724383104
      positionAbsolute:
        x: -108.28652292656551
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_array_file: false
        selected: false
        title: 文档提取器
        type: document-extractor
        variable_selector:
        - '1752479895761'
        - file
      height: 90
      id: '1752481112180'
      position:
        x: -108.28652292656551
        y: 390.6576481692478
      positionAbsolute:
        x: -108.28652292656551
        y: 390.6576481692478
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        cases:
        - case_id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          conditions:
          - comparison_operator: is
            id: 9da88d93-3ff6-463f-abfd-6bcafbf2554d
            value: .xlsx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: d0e88f5e-dfe3-4bae-af0c-dbec267500de
            value: .xls
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: a957e91e-1ed7-4c6b-9c80-2f0948858f1d
            value: .md
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 870c3c39-8d3f-474a-ab8b-9c0ccf53db73
            value: .markdown
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: f9541513-1e71-4dc1-9db5-35dc84a39e3c
            value: .mdx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 4c7f455b-ac20-40ca-9495-6cc44ffcb35d
            value: .html
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 2e12d9c7-8057-4a09-8851-f9fd1d0718d1
            value: .htm
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 73a995a9-d8b9-4aef-89f7-306e2ddcbce2
            value: .docx
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: 8a2e8772-0426-458b-a1f9-9eaaec0f27c8
            value: .csv
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          - comparison_operator: is
            id: aa2cb6b6-a2fc-462a-a9f5-c9c3f33a1602
            value: .txt
            varType: file
            variable_selector:
            - '1752479895761'
            - file
            - extension
          id: 24e47cad-f1e2-4f74-9884-3f49d5bb37b7
          logical_operator: or
        selected: false
        title: IF/ELSE
        type: if-else
      height: 358
      id: '1752481129417'
      position:
        x: -489.57009543377865
        y: 251.3910724383104
      positionAbsolute:
        x: -489.57009543377865
        y: 251.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        advanced_settings:
          group_enabled: false
          groups:
          - groupId: f4cf07b4-914d-4544-8ef8-0c5d9e4f21a7
            group_name: Group1
            output_type: string
            variables:
            - - '1752481112180'
              - text
            - - '1752480460682'
              - text
        output_type: string
        selected: false
        title: Variable Aggregator
        type: variable-aggregator
        variables:
        - - '1752481112180'
          - text
        - - '1752480460682'
          - text
      height: 129
      id: '1752482022496'
      position:
        x: 319.441649575055
        y: 281.3910724383104
      positionAbsolute:
        x: 319.441649575055
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    - data:
        is_team_authorization: true
        output_schema:
          properties:
            result:
              description: The result of the general chunk tool.
              properties:
                general_chunks:
                  items:
                    description: The chunk of the text.
                    type: string
                  type: array
              type: object
          type: object
        paramSchemas:
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The text you want to chunk.
            ja_JP: チャンク化したいテキスト。
            pt_BR: O texto que você deseja dividir.
            zh_Hans: 你想要分块的文本。
          label:
            en_US: Input Variable
            ja_JP: 入力変数
            pt_BR: Variável de entrada
            zh_Hans: 输入变量
          llm_description: The text you want to chunk.
          max: null
          min: null
          name: input_variable
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The delimiter of the chunks.
            ja_JP: チャンクの区切り記号。
            pt_BR: O delimitador dos pedaços.
            zh_Hans: 块的分隔符。
          label:
            en_US: Delimiter
            ja_JP: 区切り記号
            pt_BR: Delimitador
            zh_Hans: 分隔符
          llm_description: The delimiter of the chunks, the format of the delimiter
            must be a string.
          max: null
          min: null
          name: delimiter
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: string
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The maximum chunk length.
            ja_JP: 最大長のチャンク。
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度。
          label:
            en_US: Maximum Chunk Length
            ja_JP: チャンク最大長
            pt_BR: O comprimento máximo do bloco
            zh_Hans: 最大块的长度
          llm_description: The maximum chunk length, the format of the chunk size
            must be an integer.
          max: null
          min: null
          name: max_chunk_length
          options: []
          placeholder: null
          precision: null
          required: true
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: The chunk overlap length.
            ja_JP: チャンクの重複長
            pt_BR: The chunk overlap length.
            zh_Hans: 块的重叠长度。
          label:
            en_US: Chunk Overlap Length
            ja_JP: チャンク重複長
            pt_BR: Chunk Overlap Length
            zh_Hans: 块的重叠长度
          llm_description: The chunk overlap length, the format of the chunk overlap
            length must be an integer.
          max: null
          min: null
          name: chunk_overlap_length
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: number
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Replace consecutive spaces, newlines and tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace consecutive spaces, newlines and tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          label:
            en_US: Replace Consecutive Spaces, Newlines and Tabs
            ja_JP: 連続のスペース、改行、まだはタブを置換する
            pt_BR: Replace Consecutive Spaces, Newlines and Tabs
            zh_Hans: 替换连续的空格、换行符和制表符
          llm_description: Replace consecutive spaces, newlines and tabs, the format
            of the replace must be a boolean.
          max: null
          min: null
          name: replace_consecutive_spaces_newlines_tabs
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        - auto_generate: null
          default: null
          form: llm
          human_description:
            en_US: Delete all URLs and email addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete all URLs and email addresses
            zh_Hans: 删除所有URL和电子邮件地址
          label:
            en_US: Delete All URLs and Email Addresses
            ja_JP: すべてのURLとメールアドレスを削除する
            pt_BR: Delete All URLs and Email Addresses
            zh_Hans: 删除所有URL和电子邮件地址
          llm_description: Delete all URLs and email addresses, the format of the
            delete must be a boolean.
          max: null
          min: null
          name: delete_all_urls_and_email_addresses
          options: []
          placeholder: null
          precision: null
          required: false
          scope: null
          template: null
          type: boolean
        params:
          chunk_overlap_length: ''
          delete_all_urls_and_email_addresses: ''
          delimiter: ''
          input_variable: ''
          max_chunk_length: ''
          replace_consecutive_spaces_newlines_tabs: ''
        provider_id: langgenius/general_chunker/general_chunker
        provider_name: langgenius/general_chunker/general_chunker
        provider_type: builtin
        selected: false
        title: General Chunker
        tool_configurations: {}
        tool_description: A tool for general text chunking mode, the chunks retrieved and recalled are the same.
        tool_label: General Chunker
        tool_name: general_chunker
        tool_parameters:
          chunk_overlap_length:
            type: variable
            value:
            - rag
            - shared
            - chunk_overlap
          delete_all_urls_and_email_addresses:
            type: mixed
            value: '{{#rag.shared.delete_urls_email#}}'
          delimiter:
            type: mixed
            value: '{{#rag.shared.delimiter#}}'
          input_variable:
            type: mixed
            value: '{{#1752482022496.output#}}'
          max_chunk_length:
            type: variable
            value:
            - rag
            - shared
            - max_chunk_length
          replace_consecutive_spaces_newlines_tabs:
            type: mixed
            value: '{{#rag.shared.replace_consecutive_spaces#}}'
        type: tool
      height: 52
      id: '1752482151668'
      position:
        x: 693.5300771507484
        y: 281.3910724383104
      positionAbsolute:
        x: 693.5300771507484
        y: 281.3910724383104
      selected: false
      sourcePosition: right
      targetPosition: left
      type: custom
      width: 242
    viewport:
      x: 701.4999626224237
      y: 128.33739021504016
      zoom: 0.48941689643726966
  rag_pipeline_variables:
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: \n\n
    label: Delimiter
    max_length: 100
    options: []
    placeholder: null
    required: true
    tooltips: A delimiter is the character used to separate text. \n\n is recommended
      for splitting the original document into large parent chunks. You can also use
      special delimiters defined by yourself.
    type: text-input
    unit: null
    variable: delimiter
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Maximum chunk length
    max_length: 48
    options: []
    placeholder: null
    required: true
    tooltips: null
    type: number
    unit: characters
    variable: max_chunk_length
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Chunk overlap
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: number
    unit: characters
    variable: chunk_overlap
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Replace consecutive spaces, newlines and tabs
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: replace_consecutive_spaces
  - allow_file_extension: null
    allow_file_upload_methods: null
    allowed_file_types: null
    belong_to_node_id: shared
    default_value: null
    label: Delete all URLs and email addresses
    max_length: 48
    options: []
    placeholder: null
    required: false
    tooltips: null
    type: checkbox
    unit: null
    variable: delete_urls_email

```

### api/services/rag_pipeline/rag_pipeline_dsl_service.py
```py
import base64
import hashlib
import json
import logging
import uuid
from collections.abc import Mapping
from datetime import UTC, datetime
from enum import StrEnum
from typing import cast
from urllib.parse import urlparse
from uuid import uuid4

import yaml  # type: ignore
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
from flask_login import current_user
from graphon.enums import BuiltinNodeTypes
from graphon.model_runtime.utils.encoders import jsonable_encoder
from graphon.nodes.llm.entities import LLMNodeData
from graphon.nodes.parameter_extractor.entities import ParameterExtractorNodeData
from graphon.nodes.question_classifier.entities import QuestionClassifierNodeData
from graphon.nodes.tool.entities import ToolNodeData
from packaging import version
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from core.helper import ssrf_proxy
from core.helper.name_generator import generate_incremental_name
from core.plugin.entities.plugin import PluginDependency
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.workflow.nodes.datasource.entities import DatasourceNodeData
from core.workflow.nodes.knowledge_index import KNOWLEDGE_INDEX_NODE_TYPE
from core.workflow.nodes.knowledge_retrieval.entities import KnowledgeRetrievalNodeData
from extensions.ext_redis import redis_client
from factories import variable_factory
from models import Account
from models.dataset import Dataset, DatasetCollectionBinding, Pipeline
from models.enums import CollectionBindingType, DatasetRuntimeMode
from models.workflow import Workflow, WorkflowType
from services.entities.knowledge_entities.rag_pipeline_entities import (
    IconInfo,
    KnowledgeConfiguration,
    RagPipelineDatasetCreateEntity,
)
from services.plugin.dependencies_analysis import DependenciesAnalysisService

logger = logging.getLogger(__name__)

IMPORT_INFO_REDIS_KEY_PREFIX = "app_import_info:"
CHECK_DEPENDENCIES_REDIS_KEY_PREFIX = "app_check_dependencies:"
IMPORT_INFO_REDIS_EXPIRY = 10 * 60  # 10 minutes
DSL_MAX_SIZE = 10 * 1024 * 1024  # 10MB
CURRENT_DSL_VERSION = "0.1.0"


class ImportMode(StrEnum):
    YAML_CONTENT = "yaml-content"
    YAML_URL = "yaml-url"


class ImportStatus(StrEnum):
    COMPLETED = "completed"
    COMPLETED_WITH_WARNINGS = "completed-with-warnings"
    PENDING = "pending"
    FAILED = "failed"


class RagPipelineImportInfo(BaseModel):
    id: str
    status: ImportStatus
    pipeline_id: str | None = None
    current_dsl_version: str = CURRENT_DSL_VERSION
    imported_dsl_version: str = ""
    error: str = ""
    dataset_id: str | None = None


class CheckDependenciesResult(BaseModel):
    leaked_dependencies: list[PluginDependency] = Field(default_factory=list)


def _check_version_compatibility(imported_version: str) -> ImportStatus:
    """Determine import status based on version comparison"""
    try:
        current_ver = version.parse(CURRENT_DSL_VERSION)
        imported_ver = version.parse(imported_version)
    except version.InvalidVersion:
        return ImportStatus.FAILED

    # If imported version is newer than current, always return PENDING
    if imported_ver > current_ver:
        return ImportStatus.PENDING

    # If imported version is older than current's major, return PENDING
    if imported_ver.major < current_ver.major:
        return ImportStatus.PENDING

    # If imported version is older than current's minor, return COMPLETED_WITH_WARNINGS
    if imported_ver.minor < current_ver.minor:
        return ImportStatus.COMPLETED_WITH_WARNINGS

    # If imported version equals or is older than current's micro, return COMPLETED
    return ImportStatus.COMPLETED


class RagPipelinePendingData(BaseModel):
    import_mode: str
    yaml_content: str
    pipeline_id: str | None


class CheckDependenciesPendingData(BaseModel):
    dependencies: list[PluginDependency]
    pipeline_id: str | None


class RagPipelineDslService:
    def __init__(self, session: Session):
        self._session = session

    def import_rag_pipeline(
        self,
        *,
        account: Account,
        import_mode: str,
        yaml_content: str | None = None,
        yaml_url: str | None = None,
        pipeline_id: str | None = None,
        dataset: Dataset | None = None,
        dataset_name: str | None = None,
        icon_info: IconInfo | None = None,
    ) -> RagPipelineImportInfo:
        """Import an app from YAML content or URL."""
        import_id = str(uuid.uuid4())

        # Validate import mode
        try:
            mode = ImportMode(import_mode)
        except ValueError:
            raise ValueError(f"Invalid import_mode: {import_mode}")

        # Get YAML content
        content: str = ""
        if mode == ImportMode.YAML_URL:
            if not yaml_url:
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error="yaml_url is required when import_mode is yaml-url",
                )
            try:
                parsed_url = urlparse(yaml_url)
                if (
                    parsed_url.scheme == "https"
                    and parsed_url.netloc == "github.com"
                    and parsed_url.path.endswith((".yml", ".yaml"))
                ):
                    yaml_url = yaml_url.replace("https://github.com", "https://raw.githubusercontent.com")
                    yaml_url = yaml_url.replace("/blob/", "/")
                response = ssrf_proxy.get(yaml_url.strip(), follow_redirects=True, timeout=(10, 10))
                response.raise_for_status()
                content = response.content.decode()

                if len(content) > DSL_MAX_SIZE:
                    return RagPipelineImportInfo(
                        id=import_id,
                        status=ImportStatus.FAILED,
                        error="File size exceeds the limit of 10MB",
                    )

                if not content:
                    return RagPipelineImportInfo(
                        id=import_id,
                        status=ImportStatus.FAILED,
                        error="Empty content from url",
                    )
            except Exception as e:
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error=f"Error fetching YAML from URL: {str(e)}",
                )
        elif mode == ImportMode.YAML_CONTENT:
            if not yaml_content:
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error="yaml_content is required when import_mode is yaml-content",
                )
            content = yaml_content

        # Process YAML content
        try:
            # Parse YAML to validate format
            data = yaml.safe_load(content)
            if not isinstance(data, dict):
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error="Invalid YAML format: content must be a mapping",
                )

            # Validate and fix DSL version
            if not data.get("version"):
                data["version"] = "0.1.0"
            if not data.get("kind") or data.get("kind") != "rag_pipeline":
                data["kind"] = "rag_pipeline"

            imported_version = data.get("version", "0.1.0")
            # check if imported_version is a float-like string
            if not isinstance(imported_version, str):
                raise ValueError(f"Invalid version type, expected str, got {type(imported_version)}")
            status = _check_version_compatibility(imported_version)

            # Extract app data
            pipeline_data = data.get("rag_pipeline")
            if not pipeline_data:
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error="Missing rag_pipeline data in YAML content",
                )

            # If app_id is provided, check if it exists
            pipeline = None
            if pipeline_id:
                stmt = select(Pipeline).where(
                    Pipeline.id == pipeline_id,
                    Pipeline.tenant_id == account.current_tenant_id,
                )
                pipeline = self._session.scalar(stmt)

                if not pipeline:
                    return RagPipelineImportInfo(
                        id=import_id,
                        status=ImportStatus.FAILED,
                        error="Pipeline not found",
                    )
                dataset = pipeline.retrieve_dataset(session=self._session)

            # If major version mismatch, store import info in Redis
            if status == ImportStatus.PENDING:
                pending_data = RagPipelinePendingData(
                    import_mode=import_mode,
                    yaml_content=content,
                    pipeline_id=pipeline_id,
                )
                redis_client.setex(
                    f"{IMPORT_INFO_REDIS_KEY_PREFIX}{import_id}",
                    IMPORT_INFO_REDIS_EXPIRY,
                    pending_data.model_dump_json(),
                )

                return RagPipelineImportInfo(
                    id=import_id,
                    status=status,
                    pipeline_id=pipeline_id,
                    imported_dsl_version=imported_version,
                )

            # Extract dependencies
            dependencies = data.get("dependencies", [])
            check_dependencies_pending_data = None
            if dependencies:
                check_dependencies_pending_data = [PluginDependency.model_validate(d) for d in dependencies]

            # Create or update pipeline
            pipeline = self._create_or_update_pipeline(
                pipeline=pipeline,
                data=data,
                account=account,
                dependencies=check_dependencies_pending_data,
            )
            # create dataset
            name = pipeline.name or "Untitled"
            description = pipeline.description
            if icon_info:
                icon_type = icon_info.icon_type
                icon = icon_info.icon
                icon_background = icon_info.icon_background
                icon_url = icon_info.icon_url
            else:
                icon_type = data.get("rag_pipeline", {}).get("icon_type")
                icon = data.get("rag_pipeline", {}).get("icon")
                icon_background = data.get("rag_pipeline", {}).get("icon_background")
                icon_url = data.get("rag_pipeline", {}).get("icon_url")
            workflow = data.get("workflow", {})
            graph = workflow.get("graph", {})
            nodes = graph.get("nodes", [])
            dataset_id = None
            for node in nodes:
                if node.get("data", {}).get("type") == KNOWLEDGE_INDEX_NODE_TYPE:
                    knowledge_configuration = KnowledgeConfiguration.model_validate(node.get("data", {}))
                    if (
                        dataset
                        and pipeline.is_published
                        and dataset.chunk_structure != knowledge_configuration.chunk_structure
                    ):
                        raise ValueError("Chunk structure is not compatible with the published pipeline")
                    if not dataset:
                        datasets = self._session.query(Dataset).filter_by(tenant_id=account.current_tenant_id).all()
                        names = [dataset.name for dataset in datasets]
                        generate_name = generate_incremental_name(names, name)
                        dataset = Dataset(
                            tenant_id=account.current_tenant_id,
                            name=generate_name,
                            description=description,
                            icon_info={
                                "icon_type": icon_type,
                                "icon": icon,
                                "icon_background": icon_background,
                                "icon_url": icon_url,
                            },
                            indexing_technique=IndexTechniqueType(knowledge_configuration.indexing_technique),
                            created_by=account.id,
                            retrieval_model=knowledge_configuration.retrieval_model.model_dump(),
                            runtime_mode=DatasetRuntimeMode.RAG_PIPELINE,
                            chunk_structure=knowledge_configuration.chunk_structure,
                        )
                    if knowledge_configuration.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                        dataset_collection_binding = (
                            self._session.query(DatasetCollectionBinding)
                            .where(
                                DatasetCollectionBinding.provider_name
                                == knowledge_configuration.embedding_model_provider,
                                DatasetCollectionBinding.model_name == knowledge_configuration.embedding_model,
                                DatasetCollectionBinding.type == CollectionBindingType.DATASET,
                            )
                            .order_by(DatasetCollectionBinding.created_at)
                            .first()
                        )

                        if not dataset_collection_binding:
                            dataset_collection_binding = DatasetCollectionBinding(
                                provider_name=knowledge_configuration.embedding_model_provider,
                                model_name=knowledge_configuration.embedding_model,
                                collection_name=Dataset.gen_collection_name_by_id(str(uuid.uuid4())),
                                type=CollectionBindingType.DATASET,
                            )
                            self._session.add(dataset_collection_binding)
                            self._session.commit()
                        dataset_collection_binding_id = dataset_collection_binding.id
                        dataset.collection_binding_id = dataset_collection_binding_id
                        dataset.embedding_model = knowledge_configuration.embedding_model
                        dataset.embedding_model_provider = knowledge_configuration.embedding_model_provider
                    elif knowledge_configuration.indexing_technique == IndexTechniqueType.ECONOMY:
                        dataset.keyword_number = knowledge_configuration.keyword_number
                    # Update summary_index_setting if provided
                    if knowledge_configuration.summary_index_setting is not None:
                        dataset.summary_index_setting = knowledge_configuration.summary_index_setting
                    dataset.pipeline_id = pipeline.id
                    self._session.add(dataset)
                    self._session.commit()
                    dataset_id = dataset.id
            if not dataset_id:
                raise ValueError("DSL is not valid, please check the Knowledge Index node.")

            return RagPipelineImportInfo(
                id=import_id,
                status=status,
                pipeline_id=pipeline.id,
                dataset_id=dataset_id,
                imported_dsl_version=imported_version,
            )

        except yaml.YAMLError as e:
            return RagPipelineImportInfo(
                id=import_id,
                status=ImportStatus.FAILED,
                error=f"Invalid YAML format: {str(e)}",
            )

        except Exception as e:
            logger.exception("Failed to import app")
            return RagPipelineImportInfo(
                id=import_id,
                status=ImportStatus.FAILED,
                error=str(e),
            )

    def confirm_import(self, *, import_id: str, account: Account) -> RagPipelineImportInfo:
        """
        Confirm an import that requires confirmation
        """
        redis_key = f"{IMPORT_INFO_REDIS_KEY_PREFIX}{import_id}"
        pending_data = redis_client.get(redis_key)

        if not pending_data:
            return RagPipelineImportInfo(
                id=import_id,
                status=ImportStatus.FAILED,
                error="Import information expired or does not exist",
            )

        try:
            if not isinstance(pending_data, str | bytes):
                return RagPipelineImportInfo(
                    id=import_id,
                    status=ImportStatus.FAILED,
                    error="Invalid import information",
                )
            pending_data = RagPipelinePendingData.model_validate_json(pending_data)
            data = yaml.safe_load(pending_data.yaml_content)

            pipeline = None
            if pending_data.pipeline_id:
                stmt = select(Pipeline).where(
                    Pipeline.id == pending_data.pipeline_id,
                    Pipeline.tenant_id == account.current_tenant_id,
                )
                pipeline = self._session.scalar(stmt)

            # Create or update app
            pipeline = self._create_or_update_pipeline(
                pipeline=pipeline,
                data=data,
                account=account,
            )
            dataset = pipeline.retrieve_dataset(session=self._session)

            # create dataset
            name = pipeline.name
            description = pipeline.description
            icon_type = data.get("rag_pipeline", {}).get("icon_type")
            icon = data.get("rag_pipeline", {}).get("icon")
            icon_background = data.get("rag_pipeline", {}).get("icon_background")
            icon_url = data.get("rag_pipeline", {}).get("icon_url")
            workflow = data.get("workflow", {})
            graph = workflow.get("graph", {})
            nodes = graph.get("nodes", [])
            dataset_id = None
            for node in nodes:
                if node.get("data", {}).get("type") == KNOWLEDGE_INDEX_NODE_TYPE:
                    knowledge_configuration = KnowledgeConfiguration.model_validate(node.get("data", {}))
                    if not dataset:
                        dataset = Dataset(
                            tenant_id=account.current_tenant_id,
                            name=name,
                            description=description,
                            icon_info={
                                "icon_type": icon_type,
                                "icon": icon,
                                "icon_background": icon_background,
                                "icon_url": icon_url,
                            },
                            indexing_technique=IndexTechniqueType(knowledge_configuration.indexing_technique),
                            created_by=account.id,
                            retrieval_model=knowledge_configuration.retrieval_model.model_dump(),
                            runtime_mode=DatasetRuntimeMode.RAG_PIPELINE,
                            chunk_structure=knowledge_configuration.chunk_structure,
                        )
                    else:
                        dataset.indexing_technique = IndexTechniqueType(knowledge_configuration.indexing_technique)
                        dataset.retrieval_model = knowledge_configuration.retrieval_model.model_dump()
                        dataset.runtime_mode = DatasetRuntimeMode.RAG_PIPELINE
                        dataset.chunk_structure = knowledge_configuration.chunk_structure
                    if knowledge_configuration.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                        dataset_collection_binding = (
                            self._session.query(DatasetCollectionBinding)
                            .where(
                                DatasetCollectionBinding.provider_name
                                == knowledge_configuration.embedding_model_provider,
                                DatasetCollectionBinding.model_name == knowledge_configuration.embedding_model,
                                DatasetCollectionBinding.type == CollectionBindingType.DATASET,
                            )
                            .order_by(DatasetCollectionBinding.created_at)
                            .first()
                        )

                        if not dataset_collection_binding:
                            dataset_collection_binding = DatasetCollectionBinding(
                                provider_name=knowledge_configuration.embedding_model_provider,
                                model_name=knowledge_configuration.embedding_model,
                                collection_name=Dataset.gen_collection_name_by_id(str(uuid.uuid4())),
                                type=CollectionBindingType.DATASET,
                            )
                            self._session.add(dataset_collection_binding)
                            self._session.commit()
                        dataset_collection_binding_id = dataset_collection_binding.id
                        dataset.collection_binding_id = dataset_collection_binding_id
                        dataset.embedding_model = knowledge_configuration.embedding_model
                        dataset.embedding_model_provider = knowledge_configuration.embedding_model_provider
                    elif knowledge_configuration.indexing_technique == IndexTechniqueType.ECONOMY:
                        dataset.keyword_number = knowledge_configuration.keyword_number
                    # Update summary_index_setting if provided
                    if knowledge_configuration.summary_index_setting is not None:
                        dataset.summary_index_setting = knowledge_configuration.summary_index_setting
                    dataset.pipeline_id = pipeline.id
                    self._session.add(dataset)
                    self._session.commit()
                    dataset_id = dataset.id
            if not dataset_id:
                raise ValueError("DSL is not valid, please check the Knowledge Index node.")

            # Delete import info from Redis
            redis_client.delete(redis_key)

            return RagPipelineImportInfo(
                id=import_id,
                status=ImportStatus.COMPLETED,
                pipeline_id=pipeline.id,
                dataset_id=dataset_id,
                current_dsl_version=CURRENT_DSL_VERSION,
                imported_dsl_version=data.get("version", "0.1.0"),
            )

        except Exception as e:
            logger.exception("Error confirming import")
            return RagPipelineImportInfo(
                id=import_id,
                status=ImportStatus.FAILED,
                error=str(e),
            )

    def check_dependencies(
        self,
        *,
        pipeline: Pipeline,
    ) -> CheckDependenciesResult:
        """Check dependencies"""
        # Get dependencies from Redis
        redis_key = f"{CHECK_DEPENDENCIES_REDIS_KEY_PREFIX}{pipeline.id}"
        dependencies = redis_client.get(redis_key)
        if not dependencies:
            return CheckDependenciesResult()

        # Extract dependencies
        dependencies = CheckDependenciesPendingData.model_validate_json(dependencies)

        # Get leaked dependencies
        leaked_dependencies = DependenciesAnalysisService.get_leaked_dependencies(
            tenant_id=pipeline.tenant_id, dependencies=dependencies.dependencies
        )
        return CheckDependenciesResult(
            leaked_dependencies=leaked_dependencies,
        )

    def _create_or_update_pipeline(
        self,
        *,
        pipeline: Pipeline | None,
        data: dict,
        account: Account,
        dependencies: list[PluginDependency] | None = None,
    ) -> Pipeline:
        """Create a new app or update an existing one."""
        if not account.current_tenant_id:
            raise ValueError("Tenant id is required")
        pipeline_data = data.get("rag_pipeline", {})
        # Initialize pipeline based on mode
        workflow_data = data.get("workflow")
        if not workflow_data or not isinstance(workflow_data, dict):
            raise ValueError("Missing workflow data for rag pipeline")

        environment_variables_list = workflow_data.get("environment_variables", [])
        environment_variables = [
            variable_factory.build_environment_variable_from_mapping(obj) for obj in environment_variables_list
        ]
        conversation_variables_list = workflow_data.get("conversation_variables", [])
        conversation_variables = [
            variable_factory.build_conversation_variable_from_mapping(obj) for obj in conversation_variables_list
        ]
        rag_pipeline_variables_list = workflow_data.get("rag_pipeline_variables", [])

        graph = workflow_data.get("graph", {})
        for node in graph.get("nodes", []):
            if node.get("data", {}).get("type", "") == BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL:
                dataset_ids = node["data"].get("dataset_ids", [])
                node["data"]["dataset_ids"] = [
                    decrypted_id
                    for dataset_id in dataset_ids
                    if (
                        decrypted_id := self.decrypt_dataset_id(
                            encrypted_data=dataset_id,
                            tenant_id=account.current_tenant_id,
                        )
                    )
                ]

        if pipeline:
            # Update existing pipeline
            pipeline.name = pipeline_data.get("name", pipeline.name)
            pipeline.description = pipeline_data.get("description", pipeline.description)
            pipeline.updated_by = account.id

        else:
            if account.current_tenant_id is None:
                raise ValueError("Current tenant is not set")

            # Create new app
            pipeline = Pipeline(
                tenant_id=account.current_tenant_id,
                name=pipeline_data.get("name", ""),
                description=pipeline_data.get("description", ""),
                created_by=account.id,
                updated_by=account.id,
            )
            pipeline.id = str(uuid4())

            self._session.add(pipeline)
            self._session.commit()
        # save dependencies
        if dependencies:
            redis_client.setex(
                f"{CHECK_DEPENDENCIES_REDIS_KEY_PREFIX}{pipeline.id}",
                IMPORT_INFO_REDIS_EXPIRY,
                CheckDependenciesPendingData(pipeline_id=pipeline.id, dependencies=dependencies).model_dump_json(),
            )
        workflow = (
            self._session.query(Workflow)
            .where(
                Workflow.tenant_id == pipeline.tenant_id,
                Workflow.app_id == pipeline.id,
                Workflow.version == "draft",
            )
            .first()
        )

        # create draft workflow if not found
        if not workflow:
            workflow = Workflow(
                tenant_id=pipeline.tenant_id,
                app_id=pipeline.id,
                features="{}",
                type=WorkflowType.RAG_PIPELINE,
                version="draft",
                graph=json.dumps(graph),
                created_by=account.id,
                environment_variables=environment_variables,
                conversation_variables=conversation_variables,
                rag_pipeline_variables=rag_pipeline_variables_list,
            )
            self._session.add(workflow)
            self._session.flush()
            pipeline.workflow_id = workflow.id
        else:
            workflow.graph = json.dumps(graph)
            workflow.updated_by = account.id
            workflow.updated_at = datetime.now(UTC).replace(tzinfo=None)
            workflow.environment_variables = environment_variables
            workflow.conversation_variables = conversation_variables
            workflow.rag_pipeline_variables = rag_pipeline_variables_list
        # commit db session changes
        self._session.commit()

        return pipeline

    def export_rag_pipeline_dsl(self, pipeline: Pipeline, include_secret: bool = False) -> str:
        """
        Export pipeline
        :param pipeline: Pipeline instance
        :param include_secret: Whether include secret variable
        :return:
        """
        dataset = pipeline.retrieve_dataset(session=self._session)
        if not dataset:
            raise ValueError("Missing dataset for rag pipeline")
        icon_info = dataset.icon_info
        export_data = {
            "version": CURRENT_DSL_VERSION,
            "kind": "rag_pipeline",
            "rag_pipeline": {
                "name": dataset.name,
                "icon": icon_info.get("icon", "📙") if icon_info else "📙",
                "icon_type": icon_info.get("icon_type", "emoji") if icon_info else "emoji",
                "icon_background": icon_info.get("icon_background", "#FFEAD5") if icon_info else "#FFEAD5",
                "icon_url": icon_info.get("icon_url") if icon_info else None,
                "description": pipeline.description,
            },
        }

        self._append_workflow_export_data(export_data=export_data, pipeline=pipeline, include_secret=include_secret)

        return yaml.dump(export_data, allow_unicode=True)  # type: ignore

    def _append_workflow_export_data(self, *, export_data: dict, pipeline: Pipeline, include_secret: bool) -> None:
        """
        Append workflow export data
        :param export_data: export data
        :param pipeline: Pipeline instance
        """

        workflow = (
            self._session.query(Workflow)
            .where(
                Workflow.tenant_id == pipeline.tenant_id,
                Workflow.app_id == pipeline.id,
                Workflow.version == "draft",
            )
            .first()
        )
        if not workflow:
            raise ValueError("Missing draft workflow configuration, please check.")

        workflow_dict = workflow.to_dict(include_secret=include_secret)
        for node in workflow_dict.get("graph", {}).get("nodes", []):
            node_data = node.get("data", {})
            if not node_data:
                continue
            data_type = node_data.get("type", "")
            if data_type == BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL:
                dataset_ids = node_data.get("dataset_ids", [])
                node["data"]["dataset_ids"] = [
                    self.encrypt_dataset_id(dataset_id=dataset_id, tenant_id=pipeline.tenant_id)
                    for dataset_id in dataset_ids
                ]
            # filter credential id from tool node
            if not include_secret and data_type == BuiltinNodeTypes.TOOL:
                node_data.pop("credential_id", None)
            # filter credential id from agent node
            if not include_secret and data_type == BuiltinNodeTypes.AGENT:
                for tool in node_data.get("agent_parameters", {}).get("tools", {}).get("value", []):
                    tool.pop("credential_id", None)

        export_data["workflow"] = workflow_dict
        dependencies = self._extract_dependencies_from_workflow(workflow)
        export_data["dependencies"] = [
            jsonable_encoder(d.model_dump())
            for d in DependenciesAnalysisService.generate_dependencies(
                tenant_id=pipeline.tenant_id, dependencies=dependencies
            )
        ]

    def _extract_dependencies_from_workflow(self, workflow: Workflow) -> list[str]:
        """
        Extract dependencies from workflow
        :param workflow: Workflow instance
        :return: dependencies list format like ["langgenius/google"]
        """
        graph = workflow.graph_dict
        dependencies = self._extract_dependencies_from_workflow_graph(graph)
        return dependencies

    def _extract_dependencies_from_workflow_graph(self, graph: Mapping) -> list[str]:
        """
        Extract dependencies from workflow graph
        :param graph: Workflow graph
        :return: dependencies list format like ["langgenius/google"]
        """
        dependencies = []
        for node in graph.get("nodes", []):
            try:
                typ = node.get("data", {}).get("type")
                match typ:
                    case BuiltinNodeTypes.TOOL:
                        tool_entity = ToolNodeData.model_validate(node["data"])
                        dependencies.append(
                            DependenciesAnalysisService.analyze_tool_dependency(tool_entity.provider_id),
                        )
                    case BuiltinNodeTypes.DATASOURCE:
                        datasource_entity = DatasourceNodeData.model_validate(node["data"])
                        if datasource_entity.provider_type != "local_file":
                            dependencies.append(datasource_entity.plugin_id)
                    case BuiltinNodeTypes.LLM:
                        llm_entity = LLMNodeData.model_validate(node["data"])
                        dependencies.append(
                            DependenciesAnalysisService.analyze_model_provider_dependency(llm_entity.model.provider),
                        )
                    case BuiltinNodeTypes.QUESTION_CLASSIFIER:
                        question_classifier_entity = QuestionClassifierNodeData.model_validate(node["data"])
                        dependencies.append(
                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                question_classifier_entity.model.provider
                            ),
                        )
                    case BuiltinNodeTypes.PARAMETER_EXTRACTOR:
                        parameter_extractor_entity = ParameterExtractorNodeData.model_validate(node["data"])
                        dependencies.append(
                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                parameter_extractor_entity.model.provider
                            ),
                        )
                    case _ if typ == KNOWLEDGE_INDEX_NODE_TYPE:
                        knowledge_index_entity = KnowledgeConfiguration.model_validate(node["data"])
                        if knowledge_index_entity.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                            if knowledge_index_entity.embedding_model_provider:
                                dependencies.append(
                                    DependenciesAnalysisService.analyze_model_provider_dependency(
                                        knowledge_index_entity.embedding_model_provider
                                    ),
                                )
                        if knowledge_index_entity.retrieval_model.reranking_mode == "reranking_model":
                            if knowledge_index_entity.retrieval_model.reranking_enable:
                                if (
                                    knowledge_index_entity.retrieval_model.reranking_model
                                    and knowledge_index_entity.retrieval_model.reranking_mode == "reranking_model"
                                ):
                                    if knowledge_index_entity.retrieval_model.reranking_model.reranking_provider_name:
                                        dependencies.append(
                                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                                knowledge_index_entity.retrieval_model.reranking_model.reranking_provider_name
                                            ),
                                        )
                    case BuiltinNodeTypes.KNOWLEDGE_RETRIEVAL:
                        knowledge_retrieval_entity = KnowledgeRetrievalNodeData.model_validate(node["data"])
                        if knowledge_retrieval_entity.retrieval_mode == "multiple":
                            if knowledge_retrieval_entity.multiple_retrieval_config:
                                if (
                                    knowledge_retrieval_entity.multiple_retrieval_config.reranking_mode
                                    == "reranking_model"
                                ):
                                    if knowledge_retrieval_entity.multiple_retrieval_config.reranking_model:
                                        dependencies.append(
                                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                                knowledge_retrieval_entity.multiple_retrieval_config.reranking_model.provider
                                            ),
                                        )
                                elif (
                                    knowledge_retrieval_entity.multiple_retrieval_config.reranking_mode
                                    == "weighted_score"
                                ):
                                    if knowledge_retrieval_entity.multiple_retrieval_config.weights:
                                        vector_setting = (
                                            knowledge_retrieval_entity.multiple_retrieval_config.weights.vector_setting
                                        )
                                        dependencies.append(
                                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                                vector_setting.embedding_provider_name
                                            ),
                                        )
                        elif knowledge_retrieval_entity.retrieval_mode == "single":
                            model_config = knowledge_retrieval_entity.single_retrieval_config
                            if model_config:
                                dependencies.append(
                                    DependenciesAnalysisService.analyze_model_provider_dependency(
                                        model_config.model.provider
                                    ),
                                )
                    case _:
                        # TODO: Handle default case or unknown node types
                        pass
            except Exception as e:
                logger.exception("Error extracting node dependency", exc_info=e)

        return dependencies

    @classmethod
    def _extract_dependencies_from_model_config(cls, model_config: Mapping) -> list[str]:
        """
        Extract dependencies from model config
        :param model_config: model config dict
        :return: dependencies list format like ["langgenius/google"]
        """
        dependencies = []

        try:
            # completion model
            model_dict = model_config.get("model", {})
            if model_dict:
                dependencies.append(
                    DependenciesAnalysisService.analyze_model_provider_dependency(model_dict.get("provider", ""))
                )

            # reranking model
            dataset_configs = model_config.get("dataset_configs", {})
            if dataset_configs:
                for dataset_config in dataset_configs.get("datasets", {}).get("datasets", []):
                    if dataset_config.get("reranking_model"):
                        dependencies.append(
                            DependenciesAnalysisService.analyze_model_provider_dependency(
                                dataset_config.get("reranking_model", {})
                                .get("reranking_provider_name", {})
                                .get("provider")
                            )
                        )

            # tools
            agent_configs = model_config.get("agent_mode", {})
            if agent_configs:
                for agent_config in agent_configs.get("tools", []):
                    dependencies.append(
                        DependenciesAnalysisService.analyze_tool_dependency(agent_config.get("provider_id"))
                    )

        except Exception as e:
            logger.exception("Error extracting model config dependency", exc_info=e)

        return dependencies

    @classmethod
    def get_leaked_dependencies(
        cls, tenant_id: str, dsl_dependencies: list[PluginDependency]
    ) -> list[PluginDependency]:
        """
        Returns the leaked dependencies in current workspace
        """
        if not dsl_dependencies:
            return []

        return DependenciesAnalysisService.get_leaked_dependencies(tenant_id=tenant_id, dependencies=dsl_dependencies)

    def _generate_aes_key(self, tenant_id: str) -> bytes:
        """Generate AES key based on tenant_id"""
        return hashlib.sha256(tenant_id.encode()).digest()

    def encrypt_dataset_id(self, dataset_id: str, tenant_id: str) -> str:
        """Encrypt dataset_id using AES-CBC mode"""
        key = self._generate_aes_key(tenant_id)
        iv = key[:16]
        cipher = AES.new(key, AES.MODE_CBC, iv)
        ct_bytes = cipher.encrypt(pad(dataset_id.encode(), AES.block_size))
        return base64.b64encode(ct_bytes).decode()

    def decrypt_dataset_id(self, encrypted_data: str, tenant_id: str) -> str | None:
        """AES decryption"""
        try:
            key = self._generate_aes_key(tenant_id)
            iv = key[:16]
            cipher = AES.new(key, AES.MODE_CBC, iv)
            pt = unpad(cipher.decrypt(base64.b64decode(encrypted_data)), AES.block_size)
            return pt.decode()
        except Exception:
            return None

    def create_rag_pipeline_dataset(
        self,
        tenant_id: str,
        rag_pipeline_dataset_create_entity: RagPipelineDatasetCreateEntity,
    ):
        if rag_pipeline_dataset_create_entity.name:
            # check if dataset name already exists
            if (
                self._session.query(Dataset)
                .filter_by(name=rag_pipeline_dataset_create_entity.name, tenant_id=tenant_id)
                .first()
            ):
                raise ValueError(f"Dataset with name {rag_pipeline_dataset_create_entity.name} already exists.")
        else:
            # generate a random name as Untitled 1 2 3 ...
            datasets = self._session.query(Dataset).filter_by(tenant_id=tenant_id).all()
            names = [dataset.name for dataset in datasets]
            rag_pipeline_dataset_create_entity.name = generate_incremental_name(
                names,
                "Untitled",
            )

        account = cast(Account, current_user)
        rag_pipeline_import_info: RagPipelineImportInfo = self.import_rag_pipeline(
            account=account,
            import_mode=ImportMode.YAML_CONTENT,
            yaml_content=rag_pipeline_dataset_create_entity.yaml_content,
            dataset=None,
            dataset_name=rag_pipeline_dataset_create_entity.name,
            icon_info=rag_pipeline_dataset_create_entity.icon_info,
        )
        return {
            "id": rag_pipeline_import_info.id,
            "dataset_id": rag_pipeline_import_info.dataset_id,
            "pipeline_id": rag_pipeline_import_info.pipeline_id,
            "status": rag_pipeline_import_info.status,
            "imported_dsl_version": rag_pipeline_import_info.imported_dsl_version,
            "current_dsl_version": rag_pipeline_import_info.current_dsl_version,
            "error": rag_pipeline_import_info.error,
        }

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-005.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
