You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-012
- **Kind**: endpoint
- **Identifier**: Console Workspace and Billing Management
- **Description**: Workspace member management and billing endpoints. Risk of privilege escalation (member to admin), IDOR on workspace resources, and billing manipulation.
- **Locations**: api/controllers/console/workspace/, api/controllers/console/billing/

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

### api/controllers/console/workspace/tool_providers.py
```py
import io
import logging
from typing import Any, Literal
from urllib.parse import urlparse

from flask import make_response, redirect, request, send_file
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field, HttpUrl, field_validator, model_validator
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Forbidden

from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import (
    account_initialization_required,
    enterprise_license_required,
    is_admin_or_owner_required,
    setup_required,
)
from core.db.session_factory import session_factory
from core.entities.mcp_provider import MCPAuthentication, MCPConfiguration
from core.mcp.auth.auth_flow import auth, handle_callback
from core.mcp.error import MCPAuthError, MCPError, MCPRefreshTokenError
from core.mcp.mcp_client import MCPClient
from core.plugin.entities.plugin_daemon import CredentialType
from core.plugin.impl.oauth import OAuthHandler
from core.tools.entities.tool_entities import ApiProviderSchemaType, WorkflowToolParameterConfiguration
from extensions.ext_database import db
from libs.helper import alphanumeric, uuid_value
from libs.login import current_account_with_tenant, login_required
from models.provider_ids import ToolProviderID

# from models.provider_ids import ToolProviderID
from services.plugin.oauth_service import OAuthProxyService
from services.tools.api_tools_manage_service import ApiToolManageService
from services.tools.builtin_tools_manage_service import BuiltinToolManageService
from services.tools.mcp_tools_manage_service import MCPToolManageService, OAuthDataType
from services.tools.tool_labels_service import ToolLabelsService
from services.tools.tools_manage_service import ToolCommonService
from services.tools.tools_transform_service import ToolTransformService
from services.tools.workflow_tools_manage_service import WorkflowToolManageService

logger = logging.getLogger(__name__)


def is_valid_url(url: str) -> bool:
    if not url:
        return False

    try:
        parsed = urlparse(url)
        return all([parsed.scheme, parsed.netloc]) and parsed.scheme in ["http", "https"]
    except (ValueError, TypeError):
        return False


class ToolProviderListQuery(BaseModel):
    type: Literal["builtin", "model", "api", "workflow", "mcp"] | None = None


class BuiltinToolCredentialDeletePayload(BaseModel):
    credential_id: str


class BuiltinToolAddPayload(BaseModel):
    credentials: dict[str, Any]
    name: str | None = Field(default=None, max_length=30)
    type: CredentialType


class BuiltinToolUpdatePayload(BaseModel):
    credential_id: str
    credentials: dict[str, Any] | None = None
    name: str | None = Field(default=None, max_length=30)


class ApiToolProviderBasePayload(BaseModel):
    credentials: dict[str, Any]
    schema_type: ApiProviderSchemaType
    schema_: str = Field(alias="schema")
    provider: str
    icon: dict[str, Any]
    privacy_policy: str | None = None
    labels: list[str] | None = None
    custom_disclaimer: str = ""


class ApiToolProviderAddPayload(ApiToolProviderBasePayload):
    pass


class ApiToolProviderUpdatePayload(ApiToolProviderBasePayload):
    original_provider: str


class UrlQuery(BaseModel):
    url: HttpUrl


class ProviderQuery(BaseModel):
    provider: str


class ApiToolProviderDeletePayload(BaseModel):
    provider: str


class ApiToolSchemaPayload(BaseModel):
    schema_: str = Field(alias="schema")


class ApiToolTestPayload(BaseModel):
    tool_name: str
    provider_name: str | None = None
    credentials: dict[str, Any]
    parameters: dict[str, Any]
    schema_type: ApiProviderSchemaType
    schema_: str = Field(alias="schema")


class WorkflowToolBasePayload(BaseModel):
    name: str
    label: str
    description: str
    icon: dict[str, Any]
    parameters: list[WorkflowToolParameterConfiguration] = Field(default_factory=list)
    privacy_policy: str | None = ""
    labels: list[str] | None = None

    @field_validator("name")
    @classmethod
    def validate_name(cls, value: str) -> str:
        return alphanumeric(value)


class WorkflowToolCreatePayload(WorkflowToolBasePayload):
    workflow_app_id: str

    @field_validator("workflow_app_id")
    @classmethod
    def validate_workflow_app_id(cls, value: str) -> str:
        return uuid_value(value)


class WorkflowToolUpdatePayload(WorkflowToolBasePayload):
    workflow_tool_id: str

    @field_validator("workflow_tool_id")
    @classmethod
    def validate_workflow_tool_id(cls, value: str) -> str:
        return uuid_value(value)


class WorkflowToolDeletePayload(BaseModel):
    workflow_tool_id: str

    @field_validator("workflow_tool_id")
    @classmethod
    def validate_workflow_tool_id(cls, value: str) -> str:
        return uuid_value(value)


class WorkflowToolGetQuery(BaseModel):
    workflow_tool_id: str | None = None
    workflow_app_id: str | None = None

    @field_validator("workflow_tool_id", "workflow_app_id")
    @classmethod
    def validate_ids(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)

    @model_validator(mode="after")
    def ensure_one(self) -> "WorkflowToolGetQuery":
        if not self.workflow_tool_id and not self.workflow_app_id:
            raise ValueError("workflow_tool_id or workflow_app_id is required")
        return self


class WorkflowToolListQuery(BaseModel):
    workflow_tool_id: str

    @field_validator("workflow_tool_id")
    @classmethod
    def validate_workflow_tool_id(cls, value: str) -> str:
        return uuid_value(value)


class BuiltinProviderDefaultCredentialPayload(BaseModel):
    id: str


class ToolOAuthCustomClientPayload(BaseModel):
    client_params: dict[str, Any] | None = None
    enable_oauth_custom_client: bool | None = True


class MCPProviderBasePayload(BaseModel):
    server_url: str
    name: str
    icon: str
    icon_type: str
    icon_background: str = ""
    server_identifier: str
    configuration: dict[str, Any] | None = Field(default_factory=dict)
    headers: dict[str, Any] | None = Field(default_factory=dict)
    authentication: dict[str, Any] | None = Field(default_factory=dict)


class MCPProviderCreatePayload(MCPProviderBasePayload):
    pass


class MCPProviderUpdatePayload(MCPProviderBasePayload):
    provider_id: str


class MCPProviderDeletePayload(BaseModel):
    provider_id: str


class MCPAuthPayload(BaseModel):
    provider_id: str
    authorization_code: str | None = None


class MCPCallbackQuery(BaseModel):
    code: str
    state: str


register_schema_models(
    console_ns,
    BuiltinToolCredentialDeletePayload,
    BuiltinToolAddPayload,
    BuiltinToolUpdatePayload,
    ApiToolProviderAddPayload,
    ApiToolProviderUpdatePayload,
    ApiToolProviderDeletePayload,
    ApiToolSchemaPayload,
    ApiToolTestPayload,
    WorkflowToolCreatePayload,
    WorkflowToolUpdatePayload,
    WorkflowToolDeletePayload,
    BuiltinProviderDefaultCredentialPayload,
    ToolOAuthCustomClientPayload,
    MCPProviderCreatePayload,
    MCPProviderUpdatePayload,
    MCPProviderDeletePayload,
    MCPAuthPayload,
)


@console_ns.route("/workspaces/current/tool-providers")
class ToolProviderListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = ToolProviderListQuery.model_validate(raw_args)

        return ToolCommonService.list_tool_providers(user_id, tenant_id, query.type)  # type: ignore


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/tools")
class ToolBuiltinProviderListToolsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(
            BuiltinToolManageService.list_builtin_tool_provider_tools(
                tenant_id,
                provider,
            )
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/info")
class ToolBuiltinProviderInfoApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(BuiltinToolManageService.get_builtin_tool_provider_info(tenant_id, provider))


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/delete")
class ToolBuiltinProviderDeleteApi(Resource):
    @console_ns.expect(console_ns.models[BuiltinToolCredentialDeletePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider):
        _, tenant_id = current_account_with_tenant()

        payload = BuiltinToolCredentialDeletePayload.model_validate(console_ns.payload or {})

        return BuiltinToolManageService.delete_builtin_tool_provider(
            tenant_id,
            provider,
            payload.credential_id,
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/add")
class ToolBuiltinProviderAddApi(Resource):
    @console_ns.expect(console_ns.models[BuiltinToolAddPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = BuiltinToolAddPayload.model_validate(console_ns.payload or {})

        return BuiltinToolManageService.add_builtin_tool_provider(
            user_id=user_id,
            tenant_id=tenant_id,
            provider=provider,
            credentials=payload.credentials,
            name=payload.name,
            api_type=CredentialType.of(payload.type),
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/update")
class ToolBuiltinProviderUpdateApi(Resource):
    @console_ns.expect(console_ns.models[BuiltinToolUpdatePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider):
        user, tenant_id = current_account_with_tenant()
        user_id = user.id

        payload = BuiltinToolUpdatePayload.model_validate(console_ns.payload or {})

        result = BuiltinToolManageService.update_builtin_tool_provider(
            user_id=user_id,
            tenant_id=tenant_id,
            provider=provider,
            credential_id=payload.credential_id,
            credentials=payload.credentials,
            name=payload.name or "",
        )
        return result


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/credentials")
class ToolBuiltinProviderGetCredentialsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(
            BuiltinToolManageService.get_builtin_tool_provider_credentials(
                tenant_id=tenant_id,
                provider_name=provider,
            )
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/icon")
class ToolBuiltinProviderIconApi(Resource):
    @setup_required
    def get(self, provider):
        icon_bytes, mimetype = BuiltinToolManageService.get_builtin_tool_provider_icon(provider)
        icon_cache_max_age = dify_config.TOOL_ICON_CACHE_MAX_AGE
        return send_file(io.BytesIO(icon_bytes), mimetype=mimetype, max_age=icon_cache_max_age)


@console_ns.route("/workspaces/current/tool-provider/api/add")
class ToolApiProviderAddApi(Resource):
    @console_ns.expect(console_ns.models[ApiToolProviderAddPayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = ApiToolProviderAddPayload.model_validate(console_ns.payload or {})

        return ApiToolManageService.create_api_tool_provider(
            user_id,
            tenant_id,
            payload.provider,
            payload.icon,
            payload.credentials,
            payload.schema_type,
            payload.schema_,
            payload.privacy_policy or "",
            payload.custom_disclaimer or "",
            payload.labels or [],
        )


@console_ns.route("/workspaces/current/tool-provider/api/remote")
class ToolApiProviderGetRemoteSchemaApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = UrlQuery.model_validate(raw_args)

        return ApiToolManageService.get_api_tool_provider_remote_schema(
            user_id,
            tenant_id,
            str(query.url),
        )


@console_ns.route("/workspaces/current/tool-provider/api/tools")
class ToolApiProviderListToolsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = ProviderQuery.model_validate(raw_args)

        return jsonable_encoder(
            ApiToolManageService.list_api_tool_provider_tools(
                user_id,
                tenant_id,
                query.provider,
            )
        )


@console_ns.route("/workspaces/current/tool-provider/api/update")
class ToolApiProviderUpdateApi(Resource):
    @console_ns.expect(console_ns.models[ApiToolProviderUpdatePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = ApiToolProviderUpdatePayload.model_validate(console_ns.payload or {})

        return ApiToolManageService.update_api_tool_provider(
            user_id,
            tenant_id,
            payload.provider,
            payload.original_provider,
            payload.icon,
            payload.credentials,
            payload.schema_type,
            payload.schema_,
            payload.privacy_policy,
            payload.custom_disclaimer,
            payload.labels or [],
        )


@console_ns.route("/workspaces/current/tool-provider/api/delete")
class ToolApiProviderDeleteApi(Resource):
    @console_ns.expect(console_ns.models[ApiToolProviderDeletePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = ApiToolProviderDeletePayload.model_validate(console_ns.payload or {})

        return ApiToolManageService.delete_api_tool_provider(
            user_id,
            tenant_id,
            payload.provider,
        )


@console_ns.route("/workspaces/current/tool-provider/api/get")
class ToolApiProviderGetApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = ProviderQuery.model_validate(raw_args)

        return ApiToolManageService.get_api_tool_provider(
            user_id,
            tenant_id,
            query.provider,
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/credential/schema/<path:credential_type>")
class ToolBuiltinProviderCredentialsSchemaApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider, credential_type):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(
            BuiltinToolManageService.list_builtin_provider_credentials_schema(
                provider, CredentialType.of(credential_type), tenant_id
            )
        )


@console_ns.route("/workspaces/current/tool-provider/api/schema")
class ToolApiProviderSchemaApi(Resource):
    @console_ns.expect(console_ns.models[ApiToolSchemaPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = ApiToolSchemaPayload.model_validate(console_ns.payload or {})

        return ApiToolManageService.parser_api_schema(
            schema=payload.schema_,
        )


@console_ns.route("/workspaces/current/tool-provider/api/test/pre")
class ToolApiProviderPreviousTestApi(Resource):
    @console_ns.expect(console_ns.models[ApiToolTestPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = ApiToolTestPayload.model_validate(console_ns.payload or {})
        _, current_tenant_id = current_account_with_tenant()
        return ApiToolManageService.test_api_tool_preview(
            current_tenant_id,
            payload.provider_name or "",
            payload.tool_name,
            payload.credentials,
            payload.parameters,
            payload.schema_type,
            payload.schema_,
        )


@console_ns.route("/workspaces/current/tool-provider/workflow/create")
class ToolWorkflowProviderCreateApi(Resource):
    @console_ns.expect(console_ns.models[WorkflowToolCreatePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = WorkflowToolCreatePayload.model_validate(console_ns.payload or {})

        return WorkflowToolManageService.create_workflow_tool(
            user_id=user_id,
            tenant_id=tenant_id,
            workflow_app_id=payload.workflow_app_id,
            name=payload.name,
            label=payload.label,
            icon=payload.icon,
            description=payload.description,
            parameters=payload.parameters,
            privacy_policy=payload.privacy_policy or "",
            labels=payload.labels or [],
        )


@console_ns.route("/workspaces/current/tool-provider/workflow/update")
class ToolWorkflowProviderUpdateApi(Resource):
    @console_ns.expect(console_ns.models[WorkflowToolUpdatePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()
        user_id = user.id

        payload = WorkflowToolUpdatePayload.model_validate(console_ns.payload or {})

        return WorkflowToolManageService.update_workflow_tool(
            user_id,
            tenant_id,
            payload.workflow_tool_id,
            payload.name,
            payload.label,
            payload.icon,
            payload.description,
            payload.parameters,
            payload.privacy_policy or "",
            payload.labels or [],
        )


@console_ns.route("/workspaces/current/tool-provider/workflow/delete")
class ToolWorkflowProviderDeleteApi(Resource):
    @console_ns.expect(console_ns.models[WorkflowToolDeletePayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        payload = WorkflowToolDeletePayload.model_validate(console_ns.payload or {})

        return WorkflowToolManageService.delete_workflow_tool(
            user_id,
            tenant_id,
            payload.workflow_tool_id,
        )


@console_ns.route("/workspaces/current/tool-provider/workflow/get")
class ToolWorkflowProviderGetApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = WorkflowToolGetQuery.model_validate(raw_args)

        if query.workflow_tool_id:
            tool = WorkflowToolManageService.get_workflow_tool_by_tool_id(
                user_id,
                tenant_id,
                query.workflow_tool_id,
            )
        elif query.workflow_app_id:
            tool = WorkflowToolManageService.get_workflow_tool_by_app_id(
                user_id,
                tenant_id,
                query.workflow_app_id,
            )
        else:
            raise ValueError("incorrect workflow_tool_id or workflow_app_id")

        return jsonable_encoder(tool)


@console_ns.route("/workspaces/current/tool-provider/workflow/tools")
class ToolWorkflowProviderListToolApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        raw_args = request.args.to_dict()
        query = WorkflowToolListQuery.model_validate(raw_args)

        return jsonable_encoder(
            WorkflowToolManageService.list_single_workflow_tools(
                user_id,
                tenant_id,
                query.workflow_tool_id,
            )
        )


@console_ns.route("/workspaces/current/tools/builtin")
class ToolBuiltinListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        return jsonable_encoder(
            [
                provider.to_dict()
                for provider in BuiltinToolManageService.list_builtin_tools(
                    user_id,
                    tenant_id,
                )
            ]
        )


@console_ns.route("/workspaces/current/tools/api")
class ToolApiListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(
            [
                provider.to_dict()
                for provider in ApiToolManageService.list_api_tools(
                    tenant_id,
                )
            ]
        )


@console_ns.route("/workspaces/current/tools/workflow")
class ToolWorkflowListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        user_id = user.id

        return jsonable_encoder(
            [
                provider.to_dict()
                for provider in WorkflowToolManageService.list_tenant_workflow_tools(
                    user_id,
                    tenant_id,
                )
            ]
        )


@console_ns.route("/workspaces/current/tool-labels")
class ToolLabelsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @enterprise_license_required
    def get(self):
        return jsonable_encoder(ToolLabelsService.list_tool_labels())


@console_ns.route("/oauth/plugin/<path:provider>/tool/authorization-url")
class ToolPluginOAuthApi(Resource):
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def get(self, provider):
        tool_provider = ToolProviderID(provider)
        plugin_id = tool_provider.plugin_id
        provider_name = tool_provider.provider_name

        user, tenant_id = current_account_with_tenant()

        oauth_client_params = BuiltinToolManageService.get_oauth_client(tenant_id=tenant_id, provider=provider)
        if oauth_client_params is None:
            raise Forbidden("no oauth available client config found for this tool provider")

        oauth_handler = OAuthHandler()
        context_id = OAuthProxyService.create_proxy_context(
            user_id=user.id, tenant_id=tenant_id, plugin_id=plugin_id, provider=provider_name
        )
        redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider}/tool/callback"
        authorization_url_response = oauth_handler.get_authorization_url(
            tenant_id=tenant_id,
            user_id=user.id,
            plugin_id=plugin_id,
            provider=provider_name,
            redirect_uri=redirect_uri,
            system_credentials=oauth_client_params,
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


@console_ns.route("/oauth/plugin/<path:provider>/tool/callback")
class ToolOAuthCallback(Resource):
    @setup_required
    def get(self, provider):
        context_id = request.cookies.get("context_id")
        if not context_id:
            raise Forbidden("context_id not found")

        context = OAuthProxyService.use_proxy_context(context_id)
        if context is None:
            raise Forbidden("Invalid context_id")

        tool_provider = ToolProviderID(provider)
        plugin_id = tool_provider.plugin_id
        provider_name = tool_provider.provider_name
        user_id: str = context["user_id"]
        tenant_id: str = context["tenant_id"]

        oauth_handler = OAuthHandler()
        oauth_client_params = BuiltinToolManageService.get_oauth_client(tenant_id, provider)
        if oauth_client_params is None:
            raise Forbidden("no oauth available client config found for this tool provider")

        redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider}/tool/callback"
        credentials_response = oauth_handler.get_credentials(
            tenant_id=tenant_id,
            user_id=user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            redirect_uri=redirect_uri,
            system_credentials=oauth_client_params,
            request=request,
        )

        credentials = credentials_response.credentials
        expires_at = credentials_response.expires_at

        if not credentials:
            raise Exception("the plugin credentials failed")

        # add credentials to database
        BuiltinToolManageService.add_builtin_tool_provider(
            user_id=user_id,
            tenant_id=tenant_id,
            provider=provider,
            credentials=dict(credentials),
            expires_at=expires_at,
            api_type=CredentialType.OAUTH2,
        )
        return redirect(f"{dify_config.CONSOLE_WEB_URL}/oauth-callback")


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/default-credential")
class ToolBuiltinProviderSetDefaultApi(Resource):
    @console_ns.expect(console_ns.models[BuiltinProviderDefaultCredentialPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider):
        current_user, current_tenant_id = current_account_with_tenant()
        payload = BuiltinProviderDefaultCredentialPayload.model_validate(console_ns.payload or {})
        return BuiltinToolManageService.set_default_provider(
            tenant_id=current_tenant_id, user_id=current_user.id, provider=provider, id=payload.id
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/oauth/custom-client")
class ToolOAuthCustomClient(Resource):
    @console_ns.expect(console_ns.models[ToolOAuthCustomClientPayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        payload = ToolOAuthCustomClientPayload.model_validate(console_ns.payload or {})

        _, tenant_id = current_account_with_tenant()

        return BuiltinToolManageService.save_custom_oauth_client_params(
            tenant_id=tenant_id,
            provider=provider,
            client_params=payload.client_params or {},
            enable_oauth_custom_client=payload.enable_oauth_custom_client
            if payload.enable_oauth_custom_client is not None
            else True,
        )

    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, current_tenant_id = current_account_with_tenant()
        return jsonable_encoder(
            BuiltinToolManageService.get_custom_oauth_client_params(tenant_id=current_tenant_id, provider=provider)
        )

    @setup_required
    @login_required
    @account_initialization_required
    def delete(self, provider):
        _, current_tenant_id = current_account_with_tenant()
        return jsonable_encoder(
            BuiltinToolManageService.delete_custom_oauth_client_params(tenant_id=current_tenant_id, provider=provider)
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/oauth/client-schema")
class ToolBuiltinProviderGetOauthClientSchemaApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, current_tenant_id = current_account_with_tenant()
        return jsonable_encoder(
            BuiltinToolManageService.get_builtin_tool_provider_oauth_client_schema(
                tenant_id=current_tenant_id, provider_name=provider
            )
        )


@console_ns.route("/workspaces/current/tool-provider/builtin/<path:provider>/credential/info")
class ToolBuiltinProviderGetCredentialInfoApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, tenant_id = current_account_with_tenant()

        return jsonable_encoder(
            BuiltinToolManageService.get_builtin_tool_provider_credential_info(
                tenant_id=tenant_id,
                provider=provider,
            )
        )


@console_ns.route("/workspaces/current/tool-provider/mcp")
class ToolProviderMCPApi(Resource):
    @console_ns.expect(console_ns.models[MCPProviderCreatePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = MCPProviderCreatePayload.model_validate(console_ns.payload or {})
        user, tenant_id = current_account_with_tenant()

        # Parse and validate models
        configuration = MCPConfiguration.model_validate(payload.configuration or {})
        authentication = MCPAuthentication.model_validate(payload.authentication) if payload.authentication else None

        # 1) Create provider in a short transaction (no network I/O inside)
        with session_factory.create_session() as session, session.begin():
            service = MCPToolManageService(session=session)
            result = service.create_provider(
                tenant_id=tenant_id,
                user_id=user.id,
                server_url=payload.server_url,
                name=payload.name,
                icon=payload.icon,
                icon_type=payload.icon_type,
                icon_background=payload.icon_background,
                server_identifier=payload.server_identifier,
                headers=payload.headers or {},
                configuration=configuration,
                authentication=authentication,
            )

        # 2) Try to fetch tools immediately after creation so they appear without a second save.
        #    Perform network I/O outside any DB session to avoid holding locks.
        try:
            reconnect = MCPToolManageService.reconnect_with_url(
                server_url=payload.server_url,
                headers=payload.headers or {},
                timeout=configuration.timeout,
                sse_read_timeout=configuration.sse_read_timeout,
            )
            # Update just-created provider with authed/tools in a new short transaction
            with session_factory.create_session() as session, session.begin():
                service = MCPToolManageService(session=session)
                db_provider = service.get_provider(provider_id=result.id, tenant_id=tenant_id)
                db_provider.authed = reconnect.authed
                db_provider.tools = reconnect.tools

                result = ToolTransformService.mcp_provider_to_user_provider(db_provider, for_list=True)
        except Exception:
            # Best-effort: if initial fetch fails (e.g., auth required), return created provider as-is
            logger.warning("Failed to fetch MCP tools after creation", exc_info=True)

        return jsonable_encoder(result)

    @console_ns.expect(console_ns.models[MCPProviderUpdatePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def put(self):
        payload = MCPProviderUpdatePayload.model_validate(console_ns.payload or {})
        configuration = MCPConfiguration.model_validate(payload.configuration or {})
        authentication = MCPAuthentication.model_validate(payload.authentication) if payload.authentication else None
        _, current_tenant_id = current_account_with_tenant()

        # Step 1: Get provider data for URL validation (short-lived session, no network I/O)
        validation_data = None
        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            validation_data = service.get_provider_for_url_validation(
                tenant_id=current_tenant_id, provider_id=payload.provider_id
            )

        # Step 2: Perform URL validation with network I/O OUTSIDE of any database session
        # This prevents holding database locks during potentially slow network operations
        validation_result = MCPToolManageService.validate_server_url_standalone(
            tenant_id=current_tenant_id,
            new_server_url=payload.server_url,
            validation_data=validation_data,
        )

        # Step 3: Perform database update in a transaction
        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            service.update_provider(
                tenant_id=current_tenant_id,
                provider_id=payload.provider_id,
                server_url=payload.server_url,
                name=payload.name,
                icon=payload.icon,
                icon_type=payload.icon_type,
                icon_background=payload.icon_background,
                server_identifier=payload.server_identifier,
                headers=payload.headers or {},
                configuration=configuration,
                authentication=authentication,
                validation_result=validation_result,
            )

        return {"result": "success"}

    @console_ns.expect(console_ns.models[MCPProviderDeletePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def delete(self):
        payload = MCPProviderDeletePayload.model_validate(console_ns.payload or {})
        _, current_tenant_id = current_account_with_tenant()

        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            service.delete_provider(tenant_id=current_tenant_id, provider_id=payload.provider_id)

        return {"result": "success"}


@console_ns.route("/workspaces/current/tool-provider/mcp/auth")
class ToolMCPAuthApi(Resource):
    @console_ns.expect(console_ns.models[MCPAuthPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = MCPAuthPayload.model_validate(console_ns.payload or {})
        provider_id = payload.provider_id
        _, tenant_id = current_account_with_tenant()

        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            db_provider = service.get_provider(provider_id=provider_id, tenant_id=tenant_id)
            if not db_provider:
                raise ValueError("provider not found")

            # Convert to entity
            provider_entity = db_provider.to_entity()
            server_url = provider_entity.decrypt_server_url()
            headers = provider_entity.decrypt_authentication()

        # Try to connect without active transaction
        try:
            # Use MCPClientWithAuthRetry to handle authentication automatically
            with MCPClient(
                server_url=server_url,
                headers=headers,
                timeout=provider_entity.timeout,
                sse_read_timeout=provider_entity.sse_read_timeout,
            ):
                # Update credentials in new transaction
                with sessionmaker(db.engine).begin() as session:
                    service = MCPToolManageService(session=session)
                    service.update_provider_credentials(
                        provider_id=provider_id,
                        tenant_id=tenant_id,
                        credentials=provider_entity.credentials,
                        authed=True,
                    )
                return {"result": "success"}
        except MCPAuthError as e:
            try:
                # Pass the extracted OAuth metadata hints to auth()
                auth_result = auth(
                    provider_entity,
                    payload.authorization_code,
                    resource_metadata_url=e.resource_metadata_url,
                    scope_hint=e.scope_hint,
                )
                with sessionmaker(db.engine).begin() as session:
                    service = MCPToolManageService(session=session)
                    response = service.execute_auth_actions(auth_result)
                    return response
            except MCPRefreshTokenError as e:
                with sessionmaker(db.engine).begin() as session:
                    service = MCPToolManageService(session=session)
                    service.clear_provider_credentials(provider_id=provider_id, tenant_id=tenant_id)
                raise ValueError(f"Failed to refresh token, please try to authorize again: {e}") from e
        except (MCPError, ValueError) as e:
            with sessionmaker(db.engine).begin() as session:
                service = MCPToolManageService(session=session)
                service.clear_provider_credentials(provider_id=provider_id, tenant_id=tenant_id)
            raise ValueError(f"Failed to connect to MCP server: {e}") from e


@console_ns.route("/workspaces/current/tool-provider/mcp/tools/<path:provider_id>")
class ToolMCPDetailApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider_id):
        _, tenant_id = current_account_with_tenant()
        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            provider = service.get_provider(provider_id=provider_id, tenant_id=tenant_id)
            return jsonable_encoder(ToolTransformService.mcp_provider_to_user_provider(provider, for_list=True))


@console_ns.route("/workspaces/current/tools/mcp")
class ToolMCPListAllApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()

        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            # Skip sensitive data decryption for list view to improve performance
            tools = service.list_providers(tenant_id=tenant_id, include_sensitive=False)

            return [tool.to_dict() for tool in tools]


@console_ns.route("/workspaces/current/tool-provider/mcp/update/<path:provider_id>")
class ToolMCPUpdateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider_id):
        _, tenant_id = current_account_with_tenant()
        with sessionmaker(db.engine).begin() as session:
            service = MCPToolManageService(session=session)
            tools = service.list_provider_tools(
                tenant_id=tenant_id,
                provider_id=provider_id,
            )
            return jsonable_encoder(tools)


@console_ns.route("/mcp/oauth/callback")
class ToolMCPCallbackApi(Resource):
    def get(self):
        raw_args = request.args.to_dict()
        query = MCPCallbackQuery.model_validate(raw_args)
        state_key = query.state
        authorization_code = query.code

        # Create service instance for handle_callback
        with sessionmaker(db.engine).begin() as session:
            mcp_service = MCPToolManageService(session=session)
            # handle_callback now returns state data and tokens
            state_data, tokens = handle_callback(state_key, authorization_code)
            # Save tokens using the service layer
            mcp_service.save_oauth_data(
                state_data.provider_id, state_data.tenant_id, tokens.model_dump(), OAuthDataType.TOKENS
            )

        return redirect(f"{dify_config.CONSOLE_WEB_URL}/oauth-callback")

```

### api/controllers/console/workspace/model_providers.py
```py
import io
from typing import Any, Literal

from flask import request, send_file
from flask_restx import Resource
from graphon.model_runtime.entities.model_entities import ModelType
from graphon.model_runtime.errors.validate import CredentialsValidateFailedError
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field, field_validator

from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, is_admin_or_owner_required, setup_required
from libs.helper import uuid_value
from libs.login import current_account_with_tenant, login_required
from services.billing_service import BillingService
from services.model_provider_service import ModelProviderService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ParserModelList(BaseModel):
    model_type: ModelType | None = None


class ParserCredentialId(BaseModel):
    credential_id: str | None = None

    @field_validator("credential_id")
    @classmethod
    def validate_optional_credential_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class ParserCredentialCreate(BaseModel):
    credentials: dict[str, Any]
    name: str | None = Field(default=None, max_length=30)


class ParserCredentialUpdate(BaseModel):
    credential_id: str
    credentials: dict[str, Any]
    name: str | None = Field(default=None, max_length=30)

    @field_validator("credential_id")
    @classmethod
    def validate_update_credential_id(cls, value: str) -> str:
        return uuid_value(value)


class ParserCredentialDelete(BaseModel):
    credential_id: str

    @field_validator("credential_id")
    @classmethod
    def validate_delete_credential_id(cls, value: str) -> str:
        return uuid_value(value)


class ParserCredentialSwitch(BaseModel):
    credential_id: str

    @field_validator("credential_id")
    @classmethod
    def validate_switch_credential_id(cls, value: str) -> str:
        return uuid_value(value)


class ParserCredentialValidate(BaseModel):
    credentials: dict[str, Any]


class ParserPreferredProviderType(BaseModel):
    preferred_provider_type: Literal["system", "custom"]


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(ParserModelList)
reg(ParserCredentialId)
reg(ParserCredentialCreate)
reg(ParserCredentialUpdate)
reg(ParserCredentialDelete)
reg(ParserCredentialSwitch)
reg(ParserCredentialValidate)
reg(ParserPreferredProviderType)


@console_ns.route("/workspaces/current/model-providers")
class ModelProviderListApi(Resource):
    @console_ns.expect(console_ns.models[ParserModelList.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()
        tenant_id = current_tenant_id

        payload = request.args.to_dict(flat=True)
        args = ParserModelList.model_validate(payload)

        model_provider_service = ModelProviderService()
        provider_list = model_provider_service.get_provider_list(tenant_id=tenant_id, model_type=args.model_type)

        return jsonable_encoder({"data": provider_list})


@console_ns.route("/workspaces/current/model-providers/<path:provider>/credentials")
class ModelProviderCredentialApi(Resource):
    @console_ns.expect(console_ns.models[ParserCredentialId.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        tenant_id = current_tenant_id
        # if credential_id is not provided, return current used credential
        payload = request.args.to_dict(flat=True)
        args = ParserCredentialId.model_validate(payload)

        model_provider_service = ModelProviderService()
        credentials = model_provider_service.get_provider_credential(
            tenant_id=tenant_id, provider=provider, credential_id=args.credential_id
        )

        return {"credentials": credentials}

    @console_ns.expect(console_ns.models[ParserCredentialCreate.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = ParserCredentialCreate.model_validate(payload)

        model_provider_service = ModelProviderService()

        try:
            model_provider_service.create_provider_credential(
                tenant_id=current_tenant_id,
                provider=provider,
                credentials=args.credentials,
                credential_name=args.name,
            )
        except CredentialsValidateFailedError as ex:
            raise ValueError(str(ex))

        return {"result": "success"}, 201

    @console_ns.expect(console_ns.models[ParserCredentialUpdate.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def put(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()

        payload = console_ns.payload or {}
        args = ParserCredentialUpdate.model_validate(payload)

        model_provider_service = ModelProviderService()

        try:
            model_provider_service.update_provider_credential(
                tenant_id=current_tenant_id,
                provider=provider,
                credentials=args.credentials,
                credential_id=args.credential_id,
                credential_name=args.name,
            )
        except CredentialsValidateFailedError as ex:
            raise ValueError(str(ex))

        return {"result": "success"}

    @console_ns.expect(console_ns.models[ParserCredentialDelete.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def delete(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = ParserCredentialDelete.model_validate(payload)

        model_provider_service = ModelProviderService()
        model_provider_service.remove_provider_credential(
            tenant_id=current_tenant_id, provider=provider, credential_id=args.credential_id
        )

        return {"result": "success"}, 204


@console_ns.route("/workspaces/current/model-providers/<path:provider>/credentials/switch")
class ModelProviderCredentialSwitchApi(Resource):
    @console_ns.expect(console_ns.models[ParserCredentialSwitch.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = ParserCredentialSwitch.model_validate(payload)

        service = ModelProviderService()
        service.switch_active_provider_credential(
            tenant_id=current_tenant_id,
            provider=provider,
            credential_id=args.credential_id,
        )
        return {"result": "success"}


@console_ns.route("/workspaces/current/model-providers/<path:provider>/credentials/validate")
class ModelProviderValidateApi(Resource):
    @console_ns.expect(console_ns.models[ParserCredentialValidate.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = ParserCredentialValidate.model_validate(payload)

        tenant_id = current_tenant_id

        model_provider_service = ModelProviderService()

        result = True
        error = ""

        try:
            model_provider_service.validate_provider_credentials(
                tenant_id=tenant_id, provider=provider, credentials=args.credentials
            )
        except CredentialsValidateFailedError as ex:
            result = False
            error = str(ex)

        response = {"result": "success" if result else "error"}

        if not result:
            response["error"] = error or "Unknown error"

        return response


@console_ns.route("/workspaces/<string:tenant_id>/model-providers/<path:provider>/<string:icon_type>/<string:lang>")
class ModelProviderIconApi(Resource):
    """
    Get model provider icon
    """

    def get(self, tenant_id: str, provider: str, icon_type: str, lang: str):
        model_provider_service = ModelProviderService()
        icon, mimetype = model_provider_service.get_model_provider_icon(
            tenant_id=tenant_id,
            provider=provider,
            icon_type=icon_type,
            lang=lang,
        )
        if icon is None:
            raise ValueError(f"icon not found for provider {provider}, icon_type {icon_type}, lang {lang}")
        return send_file(io.BytesIO(icon), mimetype=mimetype)


@console_ns.route("/workspaces/current/model-providers/<path:provider>/preferred-provider-type")
class PreferredProviderTypeUpdateApi(Resource):
    @console_ns.expect(console_ns.models[ParserPreferredProviderType.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()

        tenant_id = current_tenant_id

        payload = console_ns.payload or {}
        args = ParserPreferredProviderType.model_validate(payload)

        model_provider_service = ModelProviderService()
        model_provider_service.switch_preferred_provider(
            tenant_id=tenant_id, provider=provider, preferred_provider_type=args.preferred_provider_type
        )

        return {"result": "success"}


@console_ns.route("/workspaces/current/model-providers/<path:provider>/checkout-url")
class ModelProviderPaymentCheckoutUrlApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider: str):
        if provider != "anthropic":
            raise ValueError(f"provider name {provider} is invalid")
        current_user, current_tenant_id = current_account_with_tenant()
        BillingService.is_tenant_owner_or_admin(current_user)
        data = BillingService.get_model_provider_payment_link(
            provider_name=provider,
            tenant_id=current_tenant_id,
            account_id=current_user.id,
            prefilled_email=current_user.email,
        )
        return data

```

### api/controllers/console/workspace/error.py
```py
from libs.exception import BaseHTTPException


class RepeatPasswordNotMatchError(BaseHTTPException):
    error_code = "repeat_password_not_match"
    description = "New password and repeat password does not match."
    code = 400


class CurrentPasswordIncorrectError(BaseHTTPException):
    error_code = "current_password_incorrect"
    description = "Current password is incorrect."
    code = 400


class InvalidInvitationCodeError(BaseHTTPException):
    error_code = "invalid_invitation_code"
    description = "Invalid invitation code."
    code = 400


class AccountAlreadyInitedError(BaseHTTPException):
    error_code = "account_already_inited"
    description = "The account has been initialized. Please refresh the page."
    code = 400


class AccountNotInitializedError(BaseHTTPException):
    error_code = "account_not_initialized"
    description = "The account has not been initialized yet. Please proceed with the initialization process first."
    code = 400


class InvalidAccountDeletionCodeError(BaseHTTPException):
    error_code = "invalid_account_deletion_code"
    description = "Invalid account deletion code."
    code = 400

```

### api/controllers/console/workspace/models.py
```py
import logging
from typing import Any, cast

from flask import request
from flask_restx import Resource
from graphon.model_runtime.entities.model_entities import ModelType
from graphon.model_runtime.errors.validate import CredentialsValidateFailedError
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field, field_validator

from controllers.common.schema import register_enum_models, register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, is_admin_or_owner_required, setup_required
from libs.helper import uuid_value
from libs.login import current_account_with_tenant, login_required
from services.model_load_balancing_service import ModelLoadBalancingService
from services.model_provider_service import ModelProviderService

logger = logging.getLogger(__name__)
DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ParserGetDefault(BaseModel):
    model_type: ModelType


class Inner(BaseModel):
    model_type: ModelType
    model: str | None = None
    provider: str | None = None


class ParserPostDefault(BaseModel):
    model_settings: list[Inner]


class ParserDeleteModels(BaseModel):
    model: str
    model_type: ModelType


class LoadBalancingPayload(BaseModel):
    configs: list[dict[str, Any]] | None = None
    enabled: bool | None = None


class ParserPostModels(BaseModel):
    model: str
    model_type: ModelType
    load_balancing: LoadBalancingPayload | None = None
    config_from: str | None = None
    credential_id: str | None = None

    @field_validator("credential_id")
    @classmethod
    def validate_credential_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class ParserGetCredentials(BaseModel):
    model: str
    model_type: ModelType
    config_from: str | None = None
    credential_id: str | None = None

    @field_validator("credential_id")
    @classmethod
    def validate_get_credential_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class ParserCredentialBase(BaseModel):
    model: str
    model_type: ModelType


class ParserCreateCredential(ParserCredentialBase):
    name: str | None = Field(default=None, max_length=30)
    credentials: dict[str, Any]


class ParserUpdateCredential(ParserCredentialBase):
    credential_id: str
    credentials: dict[str, Any]
    name: str | None = Field(default=None, max_length=30)

    @field_validator("credential_id")
    @classmethod
    def validate_update_credential_id(cls, value: str) -> str:
        return uuid_value(value)


class ParserDeleteCredential(ParserCredentialBase):
    credential_id: str

    @field_validator("credential_id")
    @classmethod
    def validate_delete_credential_id(cls, value: str) -> str:
        return uuid_value(value)


class ParserParameter(BaseModel):
    model: str


register_schema_models(
    console_ns,
    ParserGetDefault,
    ParserPostDefault,
    ParserDeleteModels,
    ParserPostModels,
    ParserGetCredentials,
    ParserCreateCredential,
    ParserUpdateCredential,
    ParserDeleteCredential,
    ParserParameter,
    Inner,
)

register_enum_models(console_ns, ModelType)


@console_ns.route("/workspaces/current/default-model")
class DefaultModelApi(Resource):
    @console_ns.expect(console_ns.models[ParserGetDefault.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserGetDefault.model_validate(request.args.to_dict(flat=True))  # type: ignore

        model_provider_service = ModelProviderService()
        default_model_entity = model_provider_service.get_default_model_of_model_type(
            tenant_id=tenant_id, model_type=args.model_type
        )

        return jsonable_encoder({"data": default_model_entity})

    @console_ns.expect(console_ns.models[ParserPostDefault.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserPostDefault.model_validate(console_ns.payload)
        model_provider_service = ModelProviderService()
        model_settings = args.model_settings
        for model_setting in model_settings:
            if model_setting.provider is None:
                continue

            try:
                model_provider_service.update_default_model_of_model_type(
                    tenant_id=tenant_id,
                    model_type=model_setting.model_type,
                    provider=model_setting.provider,
                    model=cast(str, model_setting.model),
                )
            except Exception as ex:
                logger.exception(
                    "Failed to update default model, model type: %s, model: %s",
                    model_setting.model_type,
                    model_setting.model,
                )
                raise ex

        return {"result": "success"}


@console_ns.route("/workspaces/current/model-providers/<path:provider>/models")
class ModelProviderModelApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        _, tenant_id = current_account_with_tenant()

        model_provider_service = ModelProviderService()
        models = model_provider_service.get_models_by_provider(tenant_id=tenant_id, provider=provider)

        return jsonable_encoder({"data": models})

    @console_ns.expect(console_ns.models[ParserPostModels.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        # To save the model's load balance configs
        _, tenant_id = current_account_with_tenant()
        args = ParserPostModels.model_validate(console_ns.payload)

        if args.config_from == "custom-model":
            if not args.credential_id:
                raise ValueError("credential_id is required when configuring a custom-model")
            service = ModelProviderService()
            service.switch_active_custom_model_credential(
                tenant_id=tenant_id,
                provider=provider,
                model_type=args.model_type,
                model=args.model,
                credential_id=args.credential_id,
            )

        model_load_balancing_service = ModelLoadBalancingService()

        if args.load_balancing and args.load_balancing.configs:
            # save load balancing configs
            model_load_balancing_service.update_load_balancing_configs(
                tenant_id=tenant_id,
                provider=provider,
                model=args.model,
                model_type=args.model_type,
                configs=args.load_balancing.configs,
                config_from=args.config_from or "",
            )

            if args.load_balancing.enabled:
                model_load_balancing_service.enable_model_load_balancing(
                    tenant_id=tenant_id, provider=provider, model=args.model, model_type=args.model_type
                )
            else:
                model_load_balancing_service.disable_model_load_balancing(
                    tenant_id=tenant_id, provider=provider, model=args.model, model_type=args.model_type
                )

        return {"result": "success"}, 200

    @console_ns.expect(console_ns.models[ParserDeleteModels.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def delete(self, provider: str):
        _, tenant_id = current_account_with_tenant()

        args = ParserDeleteModels.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()
        model_provider_service.remove_model(
            tenant_id=tenant_id, provider=provider, model=args.model, model_type=args.model_type
        )

        return {"result": "success"}, 204


@console_ns.route("/workspaces/current/model-providers/<path:provider>/models/credentials")
class ModelProviderModelCredentialApi(Resource):
    @console_ns.expect(console_ns.models[ParserGetCredentials.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider: str):
        _, tenant_id = current_account_with_tenant()

        args = ParserGetCredentials.model_validate(request.args.to_dict(flat=True))  # type: ignore

        model_provider_service = ModelProviderService()
        current_credential = model_provider_service.get_model_credential(
            tenant_id=tenant_id,
            provider=provider,
            model_type=args.model_type,
            model=args.model,
            credential_id=args.credential_id,
        )

        model_load_balancing_service = ModelLoadBalancingService()
        is_load_balancing_enabled, load_balancing_configs = model_load_balancing_service.get_load_balancing_configs(
            tenant_id=tenant_id,
            provider=provider,
            model=args.model,
            model_type=args.model_type,
            config_from=args.config_from or "",
        )

        if args.config_from == "predefined-model":
            available_credentials = model_provider_service.get_provider_available_credentials(
                tenant_id=tenant_id,
                provider=provider,
            )
        else:
            available_credentials = model_provider_service.get_provider_model_available_credentials(
                tenant_id=tenant_id,
                provider=provider,
                model_type=args.model_type,
                model=args.model,
            )

        return jsonable_encoder(
            {
                "credentials": current_credential.get("credentials") if current_credential else {},
                "current_credential_id": current_credential.get("current_credential_id")
                if current_credential
                else None,
                "current_credential_name": current_credential.get("current_credential_name")
                if current_credential
                else None,
                "load_balancing": {"enabled": is_load_balancing_enabled, "configs": load_balancing_configs},
                "available_credentials": available_credentials,
            }
        )

    @console_ns.expect(console_ns.models[ParserCreateCredential.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        _, tenant_id = current_account_with_tenant()

        args = ParserCreateCredential.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()

        try:
            model_provider_service.create_model_credential(
                tenant_id=tenant_id,
                provider=provider,
                model=args.model,
                model_type=args.model_type,
                credentials=args.credentials,
                credential_name=args.name,
            )
        except CredentialsValidateFailedError as ex:
            logger.exception(
                "Failed to save model credentials, tenant_id: %s, model: %s, model_type: %s",
                tenant_id,
                args.model,
                args.model_type,
            )
            raise ValueError(str(ex))

        return {"result": "success"}, 201

    @console_ns.expect(console_ns.models[ParserUpdateCredential.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def put(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        args = ParserUpdateCredential.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()

        try:
            model_provider_service.update_model_credential(
                tenant_id=current_tenant_id,
                provider=provider,
                model_type=args.model_type,
                model=args.model,
                credentials=args.credentials,
                credential_id=args.credential_id,
                credential_name=args.name,
            )
        except CredentialsValidateFailedError as ex:
            raise ValueError(str(ex))

        return {"result": "success"}

    @console_ns.expect(console_ns.models[ParserDeleteCredential.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def delete(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        args = ParserDeleteCredential.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()
        model_provider_service.remove_model_credential(
            tenant_id=current_tenant_id,
            provider=provider,
            model_type=args.model_type,
            model=args.model,
            credential_id=args.credential_id,
        )

        return {"result": "success"}, 204


class ParserSwitch(BaseModel):
    model: str
    model_type: ModelType
    credential_id: str


console_ns.schema_model(
    ParserSwitch.__name__, ParserSwitch.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/workspaces/current/model-providers/<path:provider>/models/credentials/switch")
class ModelProviderModelCredentialSwitchApi(Resource):
    @console_ns.expect(console_ns.models[ParserSwitch.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider: str):
        _, current_tenant_id = current_account_with_tenant()
        args = ParserSwitch.model_validate(console_ns.payload)

        service = ModelProviderService()
        service.add_model_credential_to_model_list(
            tenant_id=current_tenant_id,
            provider=provider,
            model_type=args.model_type,
            model=args.model,
            credential_id=args.credential_id,
        )
        return {"result": "success"}


@console_ns.route(
    "/workspaces/current/model-providers/<path:provider>/models/enable", endpoint="model-provider-model-enable"
)
class ModelProviderModelEnableApi(Resource):
    @console_ns.expect(console_ns.models[ParserDeleteModels.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def patch(self, provider: str):
        _, tenant_id = current_account_with_tenant()

        args = ParserDeleteModels.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()
        model_provider_service.enable_model(
            tenant_id=tenant_id, provider=provider, model=args.model, model_type=args.model_type
        )

        return {"result": "success"}


@console_ns.route(
    "/workspaces/current/model-providers/<path:provider>/models/disable", endpoint="model-provider-model-disable"
)
class ModelProviderModelDisableApi(Resource):
    @console_ns.expect(console_ns.models[ParserDeleteModels.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def patch(self, provider: str):
        _, tenant_id = current_account_with_tenant()

        args = ParserDeleteModels.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()
        model_provider_service.disable_model(
            tenant_id=tenant_id, provider=provider, model=args.model, model_type=args.model_type
        )

        return {"result": "success"}


class ParserValidate(BaseModel):
    model: str
    model_type: ModelType
    credentials: dict


console_ns.schema_model(
    ParserValidate.__name__, ParserValidate.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@console_ns.route("/workspaces/current/model-providers/<path:provider>/models/credentials/validate")
class ModelProviderModelValidateApi(Resource):
    @console_ns.expect(console_ns.models[ParserValidate.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider: str):
        _, tenant_id = current_account_with_tenant()
        args = ParserValidate.model_validate(console_ns.payload)

        model_provider_service = ModelProviderService()

        result = True
        error = ""

        try:
            model_provider_service.validate_model_credentials(
                tenant_id=tenant_id,
                provider=provider,
                model=args.model,
                model_type=args.model_type,
                credentials=args.credentials,
            )
        except CredentialsValidateFailedError as ex:
            result = False
            error = str(ex)

        response = {"result": "success" if result else "error"}

        if not result:
            response["error"] = error or ""

        return response


@console_ns.route("/workspaces/current/model-providers/<path:provider>/models/parameter-rules")
class ModelProviderModelParameterRuleApi(Resource):
    @console_ns.expect(console_ns.models[ParserParameter.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider: str):
        args = ParserParameter.model_validate(request.args.to_dict(flat=True))  # type: ignore
        _, tenant_id = current_account_with_tenant()

        model_provider_service = ModelProviderService()
        parameter_rules = model_provider_service.get_model_parameter_rules(
            tenant_id=tenant_id, provider=provider, model=args.model
        )

        return jsonable_encoder({"data": parameter_rules})


@console_ns.route("/workspaces/current/models/model-types/<string:model_type>")
class ModelProviderAvailableModelApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, model_type):
        _, tenant_id = current_account_with_tenant()
        model_provider_service = ModelProviderService()
        models = model_provider_service.get_models_by_model_type(tenant_id=tenant_id, model_type=model_type)

        return jsonable_encoder({"data": models})

```

### api/controllers/console/workspace/members.py
```py
from urllib import parse

from flask import abort, request
from flask_restx import Resource
from pydantic import BaseModel, Field, TypeAdapter

import services
from configs import dify_config
from controllers.common.schema import register_enum_models, register_schema_models
from controllers.console import console_ns
from controllers.console.auth.error import (
    CannotTransferOwnerToSelfError,
    EmailCodeError,
    InvalidEmailError,
    InvalidTokenError,
    MemberNotInTenantError,
    NotOwnerError,
    OwnerTransferLimitError,
)
from controllers.console.error import EmailSendIpLimitError, WorkspaceMembersLimitExceeded
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_resource_check,
    is_allow_transfer_owner,
    setup_required,
)
from extensions.ext_database import db
from fields.member_fields import AccountWithRole, AccountWithRoleList
from libs.helper import extract_remote_ip
from libs.login import current_account_with_tenant, login_required
from models.account import Account, TenantAccountRole
from services.account_service import AccountService, RegisterService, TenantService
from services.errors.account import AccountAlreadyInTenantError
from services.feature_service import FeatureService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class MemberInvitePayload(BaseModel):
    emails: list[str] = Field(default_factory=list)
    role: TenantAccountRole
    language: str | None = None


class MemberRoleUpdatePayload(BaseModel):
    role: str


class OwnerTransferEmailPayload(BaseModel):
    language: str | None = None


class OwnerTransferCheckPayload(BaseModel):
    code: str
    token: str


class OwnerTransferPayload(BaseModel):
    token: str


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(MemberInvitePayload)
reg(MemberRoleUpdatePayload)
reg(OwnerTransferEmailPayload)
reg(OwnerTransferCheckPayload)
reg(OwnerTransferPayload)
register_enum_models(console_ns, TenantAccountRole)
register_schema_models(console_ns, AccountWithRole, AccountWithRoleList)


@console_ns.route("/workspaces/current/members")
class MemberListApi(Resource):
    """List all members of current tenant."""

    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountWithRoleList.__name__])
    def get(self):
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        members = TenantService.get_tenant_members(current_user.current_tenant)
        member_models = TypeAdapter(list[AccountWithRole]).validate_python(members, from_attributes=True)
        response = AccountWithRoleList(accounts=member_models)
        return response.model_dump(mode="json"), 200


@console_ns.route("/workspaces/current/members/invite-email")
class MemberInviteEmailApi(Resource):
    """Invite a new member by email."""

    @console_ns.expect(console_ns.models[MemberInvitePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("members")
    def post(self):
        payload = console_ns.payload or {}
        args = MemberInvitePayload.model_validate(payload)

        invitee_emails = args.emails
        invitee_role = args.role
        interface_language = args.language
        if not TenantAccountRole.is_non_owner_role(invitee_role):
            return {"code": "invalid-role", "message": "Invalid role"}, 400
        current_user, _ = current_account_with_tenant()
        inviter = current_user
        if not inviter.current_tenant:
            raise ValueError("No current tenant")

        # Check workspace permission for member invitations
        from libs.workspace_permission import check_workspace_member_invite_permission

        check_workspace_member_invite_permission(inviter.current_tenant.id)

        invitation_results = []
        console_web_url = dify_config.CONSOLE_WEB_URL

        workspace_members = FeatureService.get_features(tenant_id=inviter.current_tenant.id).workspace_members

        if not workspace_members.is_available(len(invitee_emails)):
            raise WorkspaceMembersLimitExceeded()

        for invitee_email in invitee_emails:
            normalized_invitee_email = invitee_email.lower()
            try:
                if not inviter.current_tenant:
                    raise ValueError("No current tenant")
                token = RegisterService.invite_new_member(
                    tenant=inviter.current_tenant,
                    email=invitee_email,
                    language=interface_language,
                    role=invitee_role,
                    inviter=inviter,
                )
                encoded_invitee_email = parse.quote(normalized_invitee_email)
                invitation_results.append(
                    {
                        "status": "success",
                        "email": normalized_invitee_email,
                        "url": f"{console_web_url}/activate?email={encoded_invitee_email}&token={token}",
                    }
                )
            except AccountAlreadyInTenantError:
                invitation_results.append(
                    {"status": "success", "email": normalized_invitee_email, "url": f"{console_web_url}/signin"}
                )
            except Exception as e:
                invitation_results.append({"status": "failed", "email": normalized_invitee_email, "message": str(e)})

        return {
            "result": "success",
            "invitation_results": invitation_results,
            "tenant_id": str(inviter.current_tenant.id) if inviter.current_tenant else "",
        }, 201


@console_ns.route("/workspaces/current/members/<uuid:member_id>")
class MemberCancelInviteApi(Resource):
    """Cancel an invitation by member id."""

    @setup_required
    @login_required
    @account_initialization_required
    def delete(self, member_id):
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        member = db.session.get(Account, str(member_id))
        if member is None:
            abort(404)
        else:
            try:
                TenantService.remove_member_from_tenant(current_user.current_tenant, member, current_user)
            except services.errors.account.CannotOperateSelfError as e:
                return {"code": "cannot-operate-self", "message": str(e)}, 400
            except services.errors.account.NoPermissionError as e:
                return {"code": "forbidden", "message": str(e)}, 403
            except services.errors.account.MemberNotInTenantError as e:
                return {"code": "member-not-found", "message": str(e)}, 404
            except Exception as e:
                raise ValueError(str(e))

        return {
            "result": "success",
            "tenant_id": str(current_user.current_tenant.id) if current_user.current_tenant else "",
        }, 200


@console_ns.route("/workspaces/current/members/<uuid:member_id>/update-role")
class MemberUpdateRoleApi(Resource):
    """Update member role."""

    @console_ns.expect(console_ns.models[MemberRoleUpdatePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def put(self, member_id):
        payload = console_ns.payload or {}
        args = MemberRoleUpdatePayload.model_validate(payload)
        new_role = args.role

        if not TenantAccountRole.is_valid_role(new_role):
            return {"code": "invalid-role", "message": "Invalid role"}, 400
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        member = db.session.get(Account, str(member_id))
        if not member:
            abort(404)

        try:
            assert member is not None, "Member not found"
            TenantService.update_member_role(current_user.current_tenant, member, new_role, current_user)
        except Exception as e:
            raise ValueError(str(e))

        # todo: 403

        return {"result": "success"}


@console_ns.route("/workspaces/current/dataset-operators")
class DatasetOperatorMemberListApi(Resource):
    """List all members of current tenant."""

    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountWithRoleList.__name__])
    def get(self):
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        members = TenantService.get_dataset_operator_members(current_user.current_tenant)
        member_models = TypeAdapter(list[AccountWithRole]).validate_python(members, from_attributes=True)
        response = AccountWithRoleList(accounts=member_models)
        return response.model_dump(mode="json"), 200


@console_ns.route("/workspaces/current/members/send-owner-transfer-confirm-email")
class SendOwnerTransferEmailApi(Resource):
    """Send owner transfer email."""

    @console_ns.expect(console_ns.models[OwnerTransferEmailPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @is_allow_transfer_owner
    def post(self):
        payload = console_ns.payload or {}
        args = OwnerTransferEmailPayload.model_validate(payload)
        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()
        current_user, _ = current_account_with_tenant()
        # check if the current user is the owner of the workspace
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        if not TenantService.is_owner(current_user, current_user.current_tenant):
            raise NotOwnerError()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        email = current_user.email

        token = AccountService.send_owner_transfer_email(
            account=current_user,
            email=email,
            language=language,
            workspace_name=current_user.current_tenant.name if current_user.current_tenant else "",
        )

        return {"result": "success", "data": token}


@console_ns.route("/workspaces/current/members/owner-transfer-check")
class OwnerTransferCheckApi(Resource):
    @console_ns.expect(console_ns.models[OwnerTransferCheckPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @is_allow_transfer_owner
    def post(self):
        payload = console_ns.payload or {}
        args = OwnerTransferCheckPayload.model_validate(payload)
        # check if the current user is the owner of the workspace
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        if not TenantService.is_owner(current_user, current_user.current_tenant):
            raise NotOwnerError()

        user_email = current_user.email

        is_owner_transfer_error_rate_limit = AccountService.is_owner_transfer_error_rate_limit(user_email)
        if is_owner_transfer_error_rate_limit:
            raise OwnerTransferLimitError()

        token_data = AccountService.get_owner_transfer_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        if user_email != token_data.get("email"):
            raise InvalidEmailError()

        if args.code != token_data.get("code"):
            AccountService.add_owner_transfer_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_owner_transfer_token(args.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_owner_transfer_token(user_email, code=args.code, additional_data={})

        AccountService.reset_owner_transfer_error_rate_limit(user_email)
        return {"is_valid": True, "email": token_data.get("email"), "token": new_token}


@console_ns.route("/workspaces/current/members/<uuid:member_id>/owner-transfer")
class OwnerTransfer(Resource):
    @console_ns.expect(console_ns.models[OwnerTransferPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @is_allow_transfer_owner
    def post(self, member_id):
        payload = console_ns.payload or {}
        args = OwnerTransferPayload.model_validate(payload)

        # check if the current user is the owner of the workspace
        current_user, _ = current_account_with_tenant()
        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        if not TenantService.is_owner(current_user, current_user.current_tenant):
            raise NotOwnerError()

        if current_user.id == str(member_id):
            raise CannotTransferOwnerToSelfError()

        transfer_token_data = AccountService.get_owner_transfer_data(args.token)
        if not transfer_token_data:
            raise InvalidTokenError()

        if transfer_token_data.get("email") != current_user.email:
            raise InvalidEmailError()

        AccountService.revoke_owner_transfer_token(args.token)

        member = db.session.get(Account, str(member_id))
        if not member:
            abort(404)
            return  # Never reached, but helps type checker

        if not current_user.current_tenant:
            raise ValueError("No current tenant")
        if not TenantService.is_member(member, current_user.current_tenant):
            raise MemberNotInTenantError()

        try:
            assert member is not None, "Member not found"
            TenantService.update_member_role(current_user.current_tenant, member, "owner", current_user)

            AccountService.send_new_owner_transfer_notify_email(
                account=member,
                email=member.email,
                workspace_name=current_user.current_tenant.name if current_user.current_tenant else "",
            )

            AccountService.send_old_owner_transfer_notify_email(
                account=current_user,
                email=current_user.email,
                workspace_name=current_user.current_tenant.name if current_user.current_tenant else "",
                new_owner_email=member.email,
            )

        except Exception as e:
            raise ValueError(str(e))

        return {"result": "success"}

```

### api/controllers/console/workspace/load_balancing_config.py
```py
from flask_restx import Resource
from graphon.model_runtime.entities.model_entities import ModelType
from graphon.model_runtime.errors.validate import CredentialsValidateFailedError
from pydantic import BaseModel
from werkzeug.exceptions import Forbidden

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import current_account_with_tenant, login_required
from models import TenantAccountRole
from services.model_load_balancing_service import ModelLoadBalancingService


class LoadBalancingCredentialPayload(BaseModel):
    model: str
    model_type: ModelType
    credentials: dict[str, object]


register_schema_models(console_ns, LoadBalancingCredentialPayload)


@console_ns.route(
    "/workspaces/current/model-providers/<path:provider>/models/load-balancing-configs/credentials-validate"
)
class LoadBalancingCredentialsValidateApi(Resource):
    @console_ns.expect(console_ns.models[LoadBalancingCredentialPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider: str):
        current_user, current_tenant_id = current_account_with_tenant()
        if not TenantAccountRole.is_privileged_role(current_user.current_role):
            raise Forbidden()

        tenant_id = current_tenant_id

        payload = LoadBalancingCredentialPayload.model_validate(console_ns.payload or {})

        # validate model load balancing credentials
        model_load_balancing_service = ModelLoadBalancingService()

        result = True
        error = ""

        try:
            model_load_balancing_service.validate_load_balancing_credentials(
                tenant_id=tenant_id,
                provider=provider,
                model=payload.model,
                model_type=payload.model_type,
                credentials=payload.credentials,
            )
        except CredentialsValidateFailedError as ex:
            result = False
            error = str(ex)

        response = {"result": "success" if result else "error"}

        if not result:
            response["error"] = error

        return response


@console_ns.route(
    "/workspaces/current/model-providers/<path:provider>/models/load-balancing-configs/<string:config_id>/credentials-validate"
)
class LoadBalancingConfigCredentialsValidateApi(Resource):
    @console_ns.expect(console_ns.models[LoadBalancingCredentialPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self, provider: str, config_id: str):
        current_user, current_tenant_id = current_account_with_tenant()
        if not TenantAccountRole.is_privileged_role(current_user.current_role):
            raise Forbidden()

        tenant_id = current_tenant_id

        payload = LoadBalancingCredentialPayload.model_validate(console_ns.payload or {})

        # validate model load balancing config credentials
        model_load_balancing_service = ModelLoadBalancingService()

        result = True
        error = ""

        try:
            model_load_balancing_service.validate_load_balancing_credentials(
                tenant_id=tenant_id,
                provider=provider,
                model=payload.model,
                model_type=payload.model_type,
                credentials=payload.credentials,
                config_id=config_id,
            )
        except CredentialsValidateFailedError as ex:
            result = False
            error = str(ex)

        response = {"result": "success" if result else "error"}

        if not result:
            response["error"] = error

        return response

```

### api/controllers/console/workspace/__init__.py
```py
from collections.abc import Callable
from functools import wraps

from sqlalchemy import select
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Forbidden

from extensions.ext_database import db
from libs.login import current_account_with_tenant
from models.account import TenantPluginPermission


def plugin_permission_required(
    install_required: bool = False,
    debug_required: bool = False,
):
    def interceptor[**P, R](view: Callable[P, R]) -> Callable[P, R]:
        @wraps(view)
        def decorated(*args: P.args, **kwargs: P.kwargs) -> R:
            current_user, current_tenant_id = current_account_with_tenant()
            user = current_user
            tenant_id = current_tenant_id

            with sessionmaker(db.engine).begin() as session:
                permission = session.scalar(
                    select(TenantPluginPermission)
                    .where(
                        TenantPluginPermission.tenant_id == tenant_id,
                    )
                    .limit(1)
                )

                if not permission:
                    # no permission set, allow access for everyone
                    return view(*args, **kwargs)

                if install_required:
                    if permission.install_permission == TenantPluginPermission.InstallPermission.NOBODY:
                        raise Forbidden()
                    if permission.install_permission == TenantPluginPermission.InstallPermission.ADMINS:
                        if not user.is_admin_or_owner:
                            raise Forbidden()
                    if permission.install_permission == TenantPluginPermission.InstallPermission.EVERYONE:
                        pass

                if debug_required:
                    if permission.debug_permission == TenantPluginPermission.DebugPermission.NOBODY:
                        raise Forbidden()
                    if permission.debug_permission == TenantPluginPermission.DebugPermission.ADMINS:
                        if not user.is_admin_or_owner:
                            raise Forbidden()
                    if permission.debug_permission == TenantPluginPermission.DebugPermission.EVERYONE:
                        pass

            return view(*args, **kwargs)

        return decorated

    return interceptor

```

### api/controllers/console/workspace/agent_providers.py
```py
from flask_restx import Resource, fields
from graphon.model_runtime.utils.encoders import jsonable_encoder

from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import current_account_with_tenant, login_required
from services.agent_service import AgentService


@console_ns.route("/workspaces/current/agent-providers")
class AgentProviderListApi(Resource):
    @console_ns.doc("list_agent_providers")
    @console_ns.doc(description="Get list of available agent providers")
    @console_ns.response(
        200,
        "Success",
        fields.List(fields.Raw(description="Agent provider information")),
    )
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        user = current_user

        user_id = user.id
        tenant_id = current_tenant_id

        return jsonable_encoder(AgentService.list_agent_providers(user_id, tenant_id))


@console_ns.route("/workspaces/current/agent-provider/<path:provider_name>")
class AgentProviderApi(Resource):
    @console_ns.doc("get_agent_provider")
    @console_ns.doc(description="Get specific agent provider details")
    @console_ns.doc(params={"provider_name": "Agent provider name"})
    @console_ns.response(
        200,
        "Success",
        fields.Raw(description="Agent provider details"),
    )
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider_name: str):
        current_user, current_tenant_id = current_account_with_tenant()
        return jsonable_encoder(AgentService.get_agent_provider(current_user.id, current_tenant_id, provider_name))

```

### api/controllers/console/workspace/endpoint.py
```py
from typing import Any

from flask import request
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, is_admin_or_owner_required, setup_required
from core.plugin.impl.exc import PluginPermissionDeniedError
from libs.login import current_account_with_tenant, login_required
from services.plugin.endpoint_service import EndpointService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class EndpointCreatePayload(BaseModel):
    plugin_unique_identifier: str
    settings: dict[str, Any]
    name: str = Field(min_length=1)


class EndpointIdPayload(BaseModel):
    endpoint_id: str


class EndpointUpdatePayload(EndpointIdPayload):
    settings: dict[str, Any]
    name: str = Field(min_length=1)


class EndpointListQuery(BaseModel):
    page: int = Field(ge=1)
    page_size: int = Field(gt=0)


class EndpointListForPluginQuery(EndpointListQuery):
    plugin_id: str


class EndpointCreateResponse(BaseModel):
    success: bool = Field(description="Operation success")


class EndpointListResponse(BaseModel):
    endpoints: list[dict[str, Any]] = Field(description="Endpoint information")


class PluginEndpointListResponse(BaseModel):
    endpoints: list[dict[str, Any]] = Field(description="Endpoint information")


class EndpointDeleteResponse(BaseModel):
    success: bool = Field(description="Operation success")


class EndpointUpdateResponse(BaseModel):
    success: bool = Field(description="Operation success")


class EndpointEnableResponse(BaseModel):
    success: bool = Field(description="Operation success")


class EndpointDisableResponse(BaseModel):
    success: bool = Field(description="Operation success")


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


register_schema_models(
    console_ns,
    EndpointCreatePayload,
    EndpointIdPayload,
    EndpointUpdatePayload,
    EndpointListQuery,
    EndpointListForPluginQuery,
    EndpointCreateResponse,
    EndpointListResponse,
    PluginEndpointListResponse,
    EndpointDeleteResponse,
    EndpointUpdateResponse,
    EndpointEnableResponse,
    EndpointDisableResponse,
)


@console_ns.route("/workspaces/current/endpoints/create")
class EndpointCreateApi(Resource):
    @console_ns.doc("create_endpoint")
    @console_ns.doc(description="Create a new plugin endpoint")
    @console_ns.expect(console_ns.models[EndpointCreatePayload.__name__])
    @console_ns.response(
        200,
        "Endpoint created successfully",
        console_ns.models[EndpointCreateResponse.__name__],
    )
    @console_ns.response(403, "Admin privileges required")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointCreatePayload.model_validate(console_ns.payload)

        try:
            return {
                "success": EndpointService.create_endpoint(
                    tenant_id=tenant_id,
                    user_id=user.id,
                    plugin_unique_identifier=args.plugin_unique_identifier,
                    name=args.name,
                    settings=args.settings,
                )
            }
        except PluginPermissionDeniedError as e:
            raise ValueError(e.description) from e


@console_ns.route("/workspaces/current/endpoints/list")
class EndpointListApi(Resource):
    @console_ns.doc("list_endpoints")
    @console_ns.doc(description="List plugin endpoints with pagination")
    @console_ns.expect(console_ns.models[EndpointListQuery.__name__])
    @console_ns.response(
        200,
        "Success",
        console_ns.models[EndpointListResponse.__name__],
    )
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointListQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        page = args.page
        page_size = args.page_size

        return jsonable_encoder(
            {
                "endpoints": EndpointService.list_endpoints(
                    tenant_id=tenant_id,
                    user_id=user.id,
                    page=page,
                    page_size=page_size,
                )
            }
        )


@console_ns.route("/workspaces/current/endpoints/list/plugin")
class EndpointListForSinglePluginApi(Resource):
    @console_ns.doc("list_plugin_endpoints")
    @console_ns.doc(description="List endpoints for a specific plugin")
    @console_ns.expect(console_ns.models[EndpointListForPluginQuery.__name__])
    @console_ns.response(
        200,
        "Success",
        console_ns.models[PluginEndpointListResponse.__name__],
    )
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointListForPluginQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        page = args.page
        page_size = args.page_size
        plugin_id = args.plugin_id

        return jsonable_encoder(
            {
                "endpoints": EndpointService.list_endpoints_for_single_plugin(
                    tenant_id=tenant_id,
                    user_id=user.id,
                    plugin_id=plugin_id,
                    page=page,
                    page_size=page_size,
                )
            }
        )


@console_ns.route("/workspaces/current/endpoints/delete")
class EndpointDeleteApi(Resource):
    @console_ns.doc("delete_endpoint")
    @console_ns.doc(description="Delete a plugin endpoint")
    @console_ns.expect(console_ns.models[EndpointIdPayload.__name__])
    @console_ns.response(
        200,
        "Endpoint deleted successfully",
        console_ns.models[EndpointDeleteResponse.__name__],
    )
    @console_ns.response(403, "Admin privileges required")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointIdPayload.model_validate(console_ns.payload)

        return {
            "success": EndpointService.delete_endpoint(
                tenant_id=tenant_id, user_id=user.id, endpoint_id=args.endpoint_id
            )
        }


@console_ns.route("/workspaces/current/endpoints/update")
class EndpointUpdateApi(Resource):
    @console_ns.doc("update_endpoint")
    @console_ns.doc(description="Update a plugin endpoint")
    @console_ns.expect(console_ns.models[EndpointUpdatePayload.__name__])
    @console_ns.response(
        200,
        "Endpoint updated successfully",
        console_ns.models[EndpointUpdateResponse.__name__],
    )
    @console_ns.response(403, "Admin privileges required")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointUpdatePayload.model_validate(console_ns.payload)

        return {
            "success": EndpointService.update_endpoint(
                tenant_id=tenant_id,
                user_id=user.id,
                endpoint_id=args.endpoint_id,
                name=args.name,
                settings=args.settings,
            )
        }


@console_ns.route("/workspaces/current/endpoints/enable")
class EndpointEnableApi(Resource):
    @console_ns.doc("enable_endpoint")
    @console_ns.doc(description="Enable a plugin endpoint")
    @console_ns.expect(console_ns.models[EndpointIdPayload.__name__])
    @console_ns.response(
        200,
        "Endpoint enabled successfully",
        console_ns.models[EndpointEnableResponse.__name__],
    )
    @console_ns.response(403, "Admin privileges required")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointIdPayload.model_validate(console_ns.payload)

        return {
            "success": EndpointService.enable_endpoint(
                tenant_id=tenant_id, user_id=user.id, endpoint_id=args.endpoint_id
            )
        }


@console_ns.route("/workspaces/current/endpoints/disable")
class EndpointDisableApi(Resource):
    @console_ns.doc("disable_endpoint")
    @console_ns.doc(description="Disable a plugin endpoint")
    @console_ns.expect(console_ns.models[EndpointIdPayload.__name__])
    @console_ns.response(
        200,
        "Endpoint disabled successfully",
        console_ns.models[EndpointDisableResponse.__name__],
    )
    @console_ns.response(403, "Admin privileges required")
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()

        args = EndpointIdPayload.model_validate(console_ns.payload)

        return {
            "success": EndpointService.disable_endpoint(
                tenant_id=tenant_id, user_id=user.id, endpoint_id=args.endpoint_id
            )
        }

```

### api/controllers/console/workspace/plugin.py
```py
import io
from collections.abc import Mapping
from typing import Any, Literal

from flask import request, send_file
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, Field
from werkzeug.datastructures import FileStorage
from werkzeug.exceptions import Forbidden

from configs import dify_config
from controllers.common.schema import register_enum_models, register_schema_models
from controllers.console import console_ns
from controllers.console.workspace import plugin_permission_required
from controllers.console.wraps import account_initialization_required, is_admin_or_owner_required, setup_required
from core.plugin.impl.exc import PluginDaemonClientSideError
from libs.login import current_account_with_tenant, login_required
from models.account import TenantPluginAutoUpgradeStrategy, TenantPluginPermission
from services.plugin.plugin_auto_upgrade_service import PluginAutoUpgradeService
from services.plugin.plugin_parameter_service import PluginParameterService
from services.plugin.plugin_permission_service import PluginPermissionService
from services.plugin.plugin_service import PluginService


class ParserList(BaseModel):
    page: int = Field(default=1, ge=1, description="Page number")
    page_size: int = Field(default=256, ge=1, le=256, description="Page size (1-256)")


class ParserLatest(BaseModel):
    plugin_ids: list[str]


class ParserIcon(BaseModel):
    tenant_id: str
    filename: str


class ParserAsset(BaseModel):
    plugin_unique_identifier: str
    file_name: str


class ParserGithubUpload(BaseModel):
    repo: str
    version: str
    package: str


class ParserPluginIdentifiers(BaseModel):
    plugin_unique_identifiers: list[str]


class ParserGithubInstall(BaseModel):
    plugin_unique_identifier: str
    repo: str
    version: str
    package: str


class ParserPluginIdentifierQuery(BaseModel):
    plugin_unique_identifier: str


class ParserTasks(BaseModel):
    page: int = Field(default=1, ge=1, description="Page number")
    page_size: int = Field(default=256, ge=1, le=256, description="Page size (1-256)")


class ParserMarketplaceUpgrade(BaseModel):
    original_plugin_unique_identifier: str
    new_plugin_unique_identifier: str


class ParserGithubUpgrade(BaseModel):
    original_plugin_unique_identifier: str
    new_plugin_unique_identifier: str
    repo: str
    version: str
    package: str


class ParserUninstall(BaseModel):
    plugin_installation_id: str


class ParserPermissionChange(BaseModel):
    install_permission: TenantPluginPermission.InstallPermission
    debug_permission: TenantPluginPermission.DebugPermission


class ParserDynamicOptions(BaseModel):
    plugin_id: str
    provider: str
    action: str
    parameter: str
    credential_id: str | None = None
    provider_type: Literal["tool", "trigger"]


class ParserDynamicOptionsWithCredentials(BaseModel):
    plugin_id: str
    provider: str
    action: str
    parameter: str
    credential_id: str
    credentials: Mapping[str, Any]


class PluginPermissionSettingsPayload(BaseModel):
    install_permission: TenantPluginPermission.InstallPermission = TenantPluginPermission.InstallPermission.EVERYONE
    debug_permission: TenantPluginPermission.DebugPermission = TenantPluginPermission.DebugPermission.EVERYONE


class PluginAutoUpgradeSettingsPayload(BaseModel):
    strategy_setting: TenantPluginAutoUpgradeStrategy.StrategySetting = (
        TenantPluginAutoUpgradeStrategy.StrategySetting.FIX_ONLY
    )
    upgrade_time_of_day: int = 0
    upgrade_mode: TenantPluginAutoUpgradeStrategy.UpgradeMode = TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE
    exclude_plugins: list[str] = Field(default_factory=list)
    include_plugins: list[str] = Field(default_factory=list)


class ParserPreferencesChange(BaseModel):
    permission: PluginPermissionSettingsPayload
    auto_upgrade: PluginAutoUpgradeSettingsPayload


class ParserExcludePlugin(BaseModel):
    plugin_id: str


class ParserReadme(BaseModel):
    plugin_unique_identifier: str
    language: str = Field(default="en-US")


register_schema_models(
    console_ns,
    ParserList,
    PluginAutoUpgradeSettingsPayload,
    PluginPermissionSettingsPayload,
    ParserLatest,
    ParserIcon,
    ParserAsset,
    ParserGithubUpload,
    ParserPluginIdentifiers,
    ParserGithubInstall,
    ParserPluginIdentifierQuery,
    ParserTasks,
    ParserMarketplaceUpgrade,
    ParserGithubUpgrade,
    ParserUninstall,
    ParserPermissionChange,
    ParserDynamicOptions,
    ParserDynamicOptionsWithCredentials,
    ParserPreferencesChange,
    ParserExcludePlugin,
    ParserReadme,
)

register_enum_models(
    console_ns,
    TenantPluginPermission.DebugPermission,
    TenantPluginAutoUpgradeStrategy.UpgradeMode,
    TenantPluginAutoUpgradeStrategy.StrategySetting,
    TenantPluginPermission.InstallPermission,
)


def _read_upload_content(file: FileStorage, max_size: int) -> bytes:
    """
    Read the uploaded file and validate its actual size before delegating to the plugin service.

    FileStorage.content_length is not reliable for multipart test uploads and may be zero even when
    content exists, so the controllers validate against the loaded bytes instead.
    """
    content = file.read()
    if len(content) > max_size:
        raise ValueError("File size exceeds the maximum allowed size")

    return content


@console_ns.route("/workspaces/current/plugin/debugging-key")
class PluginDebuggingKeyApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(debug_required=True)
    def get(self):
        _, tenant_id = current_account_with_tenant()

        try:
            return {
                "key": PluginService.get_debugging_key(tenant_id),
                "host": dify_config.PLUGIN_REMOTE_INSTALL_HOST,
                "port": dify_config.PLUGIN_REMOTE_INSTALL_PORT,
            }
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/list")
class PluginListApi(Resource):
    @console_ns.expect(console_ns.models[ParserList.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()
        args = ParserList.model_validate(request.args.to_dict(flat=True))  # type: ignore
        try:
            plugins_with_total = PluginService.list_with_total(tenant_id, args.page, args.page_size)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder({"plugins": plugins_with_total.list, "total": plugins_with_total.total})


@console_ns.route("/workspaces/current/plugin/list/latest-versions")
class PluginListLatestVersionsApi(Resource):
    @console_ns.expect(console_ns.models[ParserLatest.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        args = ParserLatest.model_validate(console_ns.payload)

        try:
            versions = PluginService.list_latest_versions(args.plugin_ids)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder({"versions": versions})


@console_ns.route("/workspaces/current/plugin/list/installations/ids")
class PluginListInstallationsFromIdsApi(Resource):
    @console_ns.expect(console_ns.models[ParserLatest.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserLatest.model_validate(console_ns.payload)

        try:
            plugins = PluginService.list_installations_from_ids(tenant_id, args.plugin_ids)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder({"plugins": plugins})


@console_ns.route("/workspaces/current/plugin/icon")
class PluginIconApi(Resource):
    @console_ns.expect(console_ns.models[ParserIcon.__name__])
    @setup_required
    def get(self):
        args = ParserIcon.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            icon_bytes, mimetype = PluginService.get_asset(args.tenant_id, args.filename)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        icon_cache_max_age = dify_config.TOOL_ICON_CACHE_MAX_AGE
        return send_file(io.BytesIO(icon_bytes), mimetype=mimetype, max_age=icon_cache_max_age)


@console_ns.route("/workspaces/current/plugin/asset")
class PluginAssetApi(Resource):
    @console_ns.expect(console_ns.models[ParserAsset.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        args = ParserAsset.model_validate(request.args.to_dict(flat=True))  # type: ignore

        _, tenant_id = current_account_with_tenant()
        try:
            binary = PluginService.extract_asset(tenant_id, args.plugin_unique_identifier, args.file_name)
            return send_file(io.BytesIO(binary), mimetype="application/octet-stream")
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/upload/pkg")
class PluginUploadFromPkgApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        file = request.files["pkg"]
        content = _read_upload_content(file, dify_config.PLUGIN_MAX_PACKAGE_SIZE)
        try:
            response = PluginService.upload_pkg(tenant_id, content)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/upload/github")
class PluginUploadFromGithubApi(Resource):
    @console_ns.expect(console_ns.models[ParserGithubUpload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserGithubUpload.model_validate(console_ns.payload)

        try:
            response = PluginService.upload_pkg_from_github(tenant_id, args.repo, args.version, args.package)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/upload/bundle")
class PluginUploadFromBundleApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        file = request.files["bundle"]
        content = _read_upload_content(file, dify_config.PLUGIN_MAX_BUNDLE_SIZE)
        try:
            response = PluginService.upload_bundle(tenant_id, content)
        except PluginDaemonClientSideError as e:
            raise ValueError(e)

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/install/pkg")
class PluginInstallFromPkgApi(Resource):
    @console_ns.expect(console_ns.models[ParserPluginIdentifiers.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()
        args = ParserPluginIdentifiers.model_validate(console_ns.payload)

        try:
            response = PluginService.install_from_local_pkg(tenant_id, args.plugin_unique_identifiers)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/install/github")
class PluginInstallFromGithubApi(Resource):
    @console_ns.expect(console_ns.models[ParserGithubInstall.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserGithubInstall.model_validate(console_ns.payload)

        try:
            response = PluginService.install_from_github(
                tenant_id,
                args.plugin_unique_identifier,
                args.repo,
                args.version,
                args.package,
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/install/marketplace")
class PluginInstallFromMarketplaceApi(Resource):
    @console_ns.expect(console_ns.models[ParserPluginIdentifiers.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserPluginIdentifiers.model_validate(console_ns.payload)

        try:
            response = PluginService.install_from_marketplace_pkg(tenant_id, args.plugin_unique_identifiers)
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder(response)


@console_ns.route("/workspaces/current/plugin/marketplace/pkg")
class PluginFetchMarketplacePkgApi(Resource):
    @console_ns.expect(console_ns.models[ParserPluginIdentifierQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def get(self):
        _, tenant_id = current_account_with_tenant()
        args = ParserPluginIdentifierQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            return jsonable_encoder(
                {
                    "manifest": PluginService.fetch_marketplace_pkg(
                        tenant_id,
                        args.plugin_unique_identifier,
                    )
                }
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/fetch-manifest")
class PluginFetchManifestApi(Resource):
    @console_ns.expect(console_ns.models[ParserPluginIdentifierQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def get(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserPluginIdentifierQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            return jsonable_encoder(
                {"manifest": PluginService.fetch_plugin_manifest(tenant_id, args.plugin_unique_identifier).model_dump()}
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/tasks")
class PluginFetchInstallTasksApi(Resource):
    @console_ns.expect(console_ns.models[ParserTasks.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def get(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserTasks.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            return jsonable_encoder({"tasks": PluginService.fetch_install_tasks(tenant_id, args.page, args.page_size)})
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/tasks/<task_id>")
class PluginFetchInstallTaskApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def get(self, task_id: str):
        _, tenant_id = current_account_with_tenant()

        try:
            return jsonable_encoder({"task": PluginService.fetch_install_task(tenant_id, task_id)})
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/tasks/<task_id>/delete")
class PluginDeleteInstallTaskApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self, task_id: str):
        _, tenant_id = current_account_with_tenant()

        try:
            return {"success": PluginService.delete_install_task(tenant_id, task_id)}
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/tasks/delete_all")
class PluginDeleteAllInstallTaskItemsApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        try:
            return {"success": PluginService.delete_all_install_task_items(tenant_id)}
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/tasks/<task_id>/delete/<path:identifier>")
class PluginDeleteInstallTaskItemApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self, task_id: str, identifier: str):
        _, tenant_id = current_account_with_tenant()

        try:
            return {"success": PluginService.delete_install_task_item(tenant_id, task_id, identifier)}
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/upgrade/marketplace")
class PluginUpgradeFromMarketplaceApi(Resource):
    @console_ns.expect(console_ns.models[ParserMarketplaceUpgrade.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserMarketplaceUpgrade.model_validate(console_ns.payload)

        try:
            return jsonable_encoder(
                PluginService.upgrade_plugin_with_marketplace(
                    tenant_id, args.original_plugin_unique_identifier, args.new_plugin_unique_identifier
                )
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/upgrade/github")
class PluginUpgradeFromGithubApi(Resource):
    @console_ns.expect(console_ns.models[ParserGithubUpgrade.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        _, tenant_id = current_account_with_tenant()

        args = ParserGithubUpgrade.model_validate(console_ns.payload)

        try:
            return jsonable_encoder(
                PluginService.upgrade_plugin_with_github(
                    tenant_id,
                    args.original_plugin_unique_identifier,
                    args.new_plugin_unique_identifier,
                    args.repo,
                    args.version,
                    args.package,
                )
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/uninstall")
class PluginUninstallApi(Resource):
    @console_ns.expect(console_ns.models[ParserUninstall.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @plugin_permission_required(install_required=True)
    def post(self):
        args = ParserUninstall.model_validate(console_ns.payload)

        _, tenant_id = current_account_with_tenant()

        try:
            return {"success": PluginService.uninstall(tenant_id, args.plugin_installation_id)}
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400


@console_ns.route("/workspaces/current/plugin/permission/change")
class PluginChangePermissionApi(Resource):
    @console_ns.expect(console_ns.models[ParserPermissionChange.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        current_user, current_tenant_id = current_account_with_tenant()
        user = current_user
        if not user.is_admin_or_owner:
            raise Forbidden()

        args = ParserPermissionChange.model_validate(console_ns.payload)

        tenant_id = current_tenant_id

        return {
            "success": PluginPermissionService.change_permission(
                tenant_id, args.install_permission, args.debug_permission
            )
        }


@console_ns.route("/workspaces/current/plugin/permission/fetch")
class PluginFetchPermissionApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()

        permission = PluginPermissionService.get_permission(tenant_id)
        if not permission:
            return jsonable_encoder(
                {
                    "install_permission": TenantPluginPermission.InstallPermission.EVERYONE,
                    "debug_permission": TenantPluginPermission.DebugPermission.EVERYONE,
                }
            )

        return jsonable_encoder(
            {
                "install_permission": permission.install_permission,
                "debug_permission": permission.debug_permission,
            }
        )


@console_ns.route("/workspaces/current/plugin/parameters/dynamic-options")
class PluginFetchDynamicSelectOptionsApi(Resource):
    @console_ns.expect(console_ns.models[ParserDynamicOptions.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def get(self):
        current_user, tenant_id = current_account_with_tenant()
        user_id = current_user.id

        args = ParserDynamicOptions.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            options = PluginParameterService.get_dynamic_select_options(
                tenant_id=tenant_id,
                user_id=user_id,
                plugin_id=args.plugin_id,
                provider=args.provider,
                action=args.action,
                parameter=args.parameter,
                credential_id=args.credential_id,
                provider_type=args.provider_type,
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder({"options": options})


@console_ns.route("/workspaces/current/plugin/parameters/dynamic-options-with-credentials")
class PluginFetchDynamicSelectOptionsWithCredentialsApi(Resource):
    @console_ns.expect(console_ns.models[ParserDynamicOptionsWithCredentials.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self):
        """Fetch dynamic options using credentials directly (for edit mode)."""
        current_user, tenant_id = current_account_with_tenant()
        user_id = current_user.id

        args = ParserDynamicOptionsWithCredentials.model_validate(console_ns.payload)

        try:
            options = PluginParameterService.get_dynamic_select_options_with_credentials(
                tenant_id=tenant_id,
                user_id=user_id,
                plugin_id=args.plugin_id,
                provider=args.provider,
                action=args.action,
                parameter=args.parameter,
                credential_id=args.credential_id,
                credentials=args.credentials,
            )
        except PluginDaemonClientSideError as e:
            return {"code": "plugin_error", "message": e.description}, 400

        return jsonable_encoder({"options": options})


@console_ns.route("/workspaces/current/plugin/preferences/change")
class PluginChangePreferencesApi(Resource):
    @console_ns.expect(console_ns.models[ParserPreferencesChange.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        user, tenant_id = current_account_with_tenant()
        if not user.is_admin_or_owner:
            raise Forbidden()

        args = ParserPreferencesChange.model_validate(console_ns.payload)

        permission = args.permission

        install_permission = permission.install_permission
        debug_permission = permission.debug_permission

        auto_upgrade = args.auto_upgrade

        strategy_setting = auto_upgrade.strategy_setting
        upgrade_time_of_day = auto_upgrade.upgrade_time_of_day
        upgrade_mode = auto_upgrade.upgrade_mode
        exclude_plugins = auto_upgrade.exclude_plugins
        include_plugins = auto_upgrade.include_plugins

        # set permission
        set_permission_result = PluginPermissionService.change_permission(
            tenant_id,
            install_permission,
            debug_permission,
        )
        if not set_permission_result:
            return jsonable_encoder({"success": False, "message": "Failed to set permission"})

        # set auto upgrade strategy
        set_auto_upgrade_strategy_result = PluginAutoUpgradeService.change_strategy(
            tenant_id,
            strategy_setting,
            upgrade_time_of_day,
            upgrade_mode,
            exclude_plugins,
            include_plugins,
        )
        if not set_auto_upgrade_strategy_result:
            return jsonable_encoder({"success": False, "message": "Failed to set auto upgrade strategy"})

        return jsonable_encoder({"success": True})


@console_ns.route("/workspaces/current/plugin/preferences/fetch")
class PluginFetchPreferencesApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()

        permission = PluginPermissionService.get_permission(tenant_id)
        permission_dict = {
            "install_permission": TenantPluginPermission.InstallPermission.EVERYONE,
            "debug_permission": TenantPluginPermission.DebugPermission.EVERYONE,
        }

        if permission:
            permission_dict["install_permission"] = permission.install_permission
            permission_dict["debug_permission"] = permission.debug_permission

        auto_upgrade = PluginAutoUpgradeService.get_strategy(tenant_id)
        auto_upgrade_dict = {
            "strategy_setting": TenantPluginAutoUpgradeStrategy.StrategySetting.DISABLED,
            "upgrade_time_of_day": 0,
            "upgrade_mode": TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE,
            "exclude_plugins": [],
            "include_plugins": [],
        }

        if auto_upgrade:
            auto_upgrade_dict = {
                "strategy_setting": auto_upgrade.strategy_setting,
                "upgrade_time_of_day": auto_upgrade.upgrade_time_of_day,
                "upgrade_mode": auto_upgrade.upgrade_mode,
                "exclude_plugins": auto_upgrade.exclude_plugins,
                "include_plugins": auto_upgrade.include_plugins,
            }

        return jsonable_encoder({"permission": permission_dict, "auto_upgrade": auto_upgrade_dict})


@console_ns.route("/workspaces/current/plugin/preferences/autoupgrade/exclude")
class PluginAutoUpgradeExcludePluginApi(Resource):
    @console_ns.expect(console_ns.models[ParserExcludePlugin.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        # exclude one single plugin
        _, tenant_id = current_account_with_tenant()

        args = ParserExcludePlugin.model_validate(console_ns.payload)

        return jsonable_encoder({"success": PluginAutoUpgradeService.exclude_plugin(tenant_id, args.plugin_id)})


@console_ns.route("/workspaces/current/plugin/readme")
class PluginReadmeApi(Resource):
    @console_ns.expect(console_ns.models[ParserReadme.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, tenant_id = current_account_with_tenant()
        args = ParserReadme.model_validate(request.args.to_dict(flat=True))  # type: ignore
        return jsonable_encoder(
            {"readme": PluginService.fetch_plugin_readme(tenant_id, args.plugin_unique_identifier, args.language)}
        )

```

### api/controllers/console/workspace/workspace.py
```py
import logging

from flask import request
from flask_restx import Resource, fields, marshal, marshal_with
from pydantic import BaseModel, Field
from sqlalchemy import select
from werkzeug.exceptions import Unauthorized

import services
from configs import dify_config
from controllers.common.errors import (
    FilenameNotExistsError,
    FileTooLargeError,
    NoFileUploadedError,
    TooManyFilesError,
    UnsupportedFileTypeError,
)
from controllers.console import console_ns
from controllers.console.admin import admin_required
from controllers.console.error import AccountNotLinkTenantError
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_resource_check,
    only_edition_enterprise,
    setup_required,
)
from enums.cloud_plan import CloudPlan
from extensions.ext_database import db
from libs.helper import TimestampField
from libs.login import current_account_with_tenant, login_required
from models.account import Tenant, TenantStatus
from services.account_service import TenantService
from services.billing_service import BillingService, SubscriptionPlan
from services.enterprise.enterprise_service import EnterpriseService
from services.feature_service import FeatureService
from services.file_service import FileService
from services.workspace_service import WorkspaceService

logger = logging.getLogger(__name__)
DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class WorkspaceListQuery(BaseModel):
    page: int = Field(default=1, ge=1, le=99999)
    limit: int = Field(default=20, ge=1, le=100)


class SwitchWorkspacePayload(BaseModel):
    tenant_id: str


class WorkspaceCustomConfigPayload(BaseModel):
    remove_webapp_brand: bool | None = None
    replace_webapp_logo: str | None = None


class WorkspaceInfoPayload(BaseModel):
    name: str


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(WorkspaceListQuery)
reg(SwitchWorkspacePayload)
reg(WorkspaceCustomConfigPayload)
reg(WorkspaceInfoPayload)

provider_fields = {
    "provider_name": fields.String,
    "provider_type": fields.String,
    "is_valid": fields.Boolean,
    "token_is_set": fields.Boolean,
}

tenant_fields = {
    "id": fields.String,
    "name": fields.String,
    "plan": fields.String,
    "status": fields.String,
    "created_at": TimestampField,
    "role": fields.String,
    "in_trial": fields.Boolean,
    "trial_end_reason": fields.String,
    "custom_config": fields.Raw(attribute="custom_config"),
    "trial_credits": fields.Integer,
    "trial_credits_used": fields.Integer,
    "next_credit_reset_date": fields.Integer,
}

tenants_fields = {
    "id": fields.String,
    "name": fields.String,
    "plan": fields.String,
    "status": fields.String,
    "created_at": TimestampField,
    "current": fields.Boolean,
}

workspace_fields = {"id": fields.String, "name": fields.String, "status": fields.String, "created_at": TimestampField}


@console_ns.route("/workspaces")
class TenantListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        tenants = TenantService.get_join_tenants(current_user)
        tenant_dicts = []
        is_enterprise_only = dify_config.ENTERPRISE_ENABLED and not dify_config.BILLING_ENABLED
        is_saas = dify_config.EDITION == "CLOUD" and dify_config.BILLING_ENABLED
        tenant_plans: dict[str, SubscriptionPlan] = {}

        if is_saas:
            tenant_ids = [tenant.id for tenant in tenants]
            if tenant_ids:
                tenant_plans = BillingService.get_plan_bulk(tenant_ids)
                if not tenant_plans:
                    logger.warning("get_plan_bulk returned empty result, falling back to legacy feature path")

        for tenant in tenants:
            plan: str = CloudPlan.SANDBOX
            if is_saas:
                tenant_plan = tenant_plans.get(tenant.id)
                if tenant_plan:
                    plan = tenant_plan["plan"] or CloudPlan.SANDBOX
                else:
                    features = FeatureService.get_features(tenant.id)
                    plan = features.billing.subscription.plan or CloudPlan.SANDBOX
            elif not is_enterprise_only:
                features = FeatureService.get_features(tenant.id)
                plan = features.billing.subscription.plan or CloudPlan.SANDBOX

            # Create a dictionary with tenant attributes
            tenant_dict = {
                "id": tenant.id,
                "name": tenant.name,
                "status": tenant.status,
                "created_at": tenant.created_at,
                "plan": plan,
                "current": tenant.id == current_tenant_id if current_tenant_id else False,
            }

            tenant_dicts.append(tenant_dict)

        return {"workspaces": marshal(tenant_dicts, tenants_fields)}, 200


@console_ns.route("/all-workspaces")
class WorkspaceListApi(Resource):
    @console_ns.expect(console_ns.models[WorkspaceListQuery.__name__])
    @setup_required
    @admin_required
    def get(self):
        payload = request.args.to_dict(flat=True)
        args = WorkspaceListQuery.model_validate(payload)

        stmt = select(Tenant).order_by(Tenant.created_at.desc())
        tenants = db.paginate(select=stmt, page=args.page, per_page=args.limit, error_out=False)
        has_more = False

        if tenants.has_next:
            has_more = True

        return {
            "data": marshal(tenants.items, workspace_fields),
            "has_more": has_more,
            "limit": args.limit,
            "page": args.page,
            "total": tenants.total,
        }, 200


@console_ns.route("/workspaces/current", endpoint="workspaces_current")
@console_ns.route("/info", endpoint="info")  # Deprecated
class TenantApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(tenant_fields)
    def post(self):
        if request.path == "/info":
            logger.warning("Deprecated URL /info was used.")

        current_user, _ = current_account_with_tenant()
        tenant = current_user.current_tenant
        if not tenant:
            raise ValueError("No current tenant")

        if tenant.status == TenantStatus.ARCHIVE:
            tenants = TenantService.get_join_tenants(current_user)
            # if there is any tenant, switch to the first one
            if len(tenants) > 0:
                TenantService.switch_tenant(current_user, tenants[0].id)
                tenant = tenants[0]
            # else, raise Unauthorized
            else:
                raise Unauthorized("workspace is archived")

        return WorkspaceService.get_tenant_info(tenant), 200


@console_ns.route("/workspaces/switch")
class SwitchWorkspaceApi(Resource):
    @console_ns.expect(console_ns.models[SwitchWorkspacePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = SwitchWorkspacePayload.model_validate(payload)

        # check if tenant_id is valid, 403 if not
        try:
            TenantService.switch_tenant(current_user, args.tenant_id)
        except Exception:
            raise AccountNotLinkTenantError("Account not link tenant")

        new_tenant = db.session.get(Tenant, args.tenant_id)  # Get new tenant
        if new_tenant is None:
            raise ValueError("Tenant not found")

        return {"result": "success", "new_tenant": marshal(WorkspaceService.get_tenant_info(new_tenant), tenant_fields)}


@console_ns.route("/workspaces/custom-config")
class CustomConfigWorkspaceApi(Resource):
    @console_ns.expect(console_ns.models[WorkspaceCustomConfigPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("workspace_custom")
    def post(self):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = WorkspaceCustomConfigPayload.model_validate(payload)
        tenant = db.get_or_404(Tenant, current_tenant_id)

        custom_config_dict = {
            "remove_webapp_brand": args.remove_webapp_brand,
            "replace_webapp_logo": args.replace_webapp_logo
            if args.replace_webapp_logo is not None
            else tenant.custom_config_dict.get("replace_webapp_logo"),
        }

        tenant.custom_config_dict = custom_config_dict
        db.session.commit()

        return {"result": "success", "tenant": marshal(WorkspaceService.get_tenant_info(tenant), tenant_fields)}


@console_ns.route("/workspaces/custom-config/webapp-logo/upload")
class WebappLogoWorkspaceApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("workspace_custom")
    def post(self):
        current_user, _ = current_account_with_tenant()
        # check file
        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()

        # get file from request
        file = request.files["file"]
        if not file.filename:
            raise FilenameNotExistsError

        extension = file.filename.split(".")[-1]
        if extension.lower() not in {"svg", "png"}:
            raise UnsupportedFileTypeError()

        try:
            upload_file = FileService(db.engine).upload_file(
                filename=file.filename,
                content=file.read(),
                mimetype=file.mimetype,
                user=current_user,
            )

        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        return {"id": upload_file.id}, 201


@console_ns.route("/workspaces/info")
class WorkspaceInfoApi(Resource):
    @console_ns.expect(console_ns.models[WorkspaceInfoPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    # Change workspace name
    def post(self):
        _, current_tenant_id = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = WorkspaceInfoPayload.model_validate(payload)

        if not current_tenant_id:
            raise ValueError("No current tenant")
        tenant = db.get_or_404(Tenant, current_tenant_id)
        tenant.name = args.name
        db.session.commit()

        return {"result": "success", "tenant": marshal(WorkspaceService.get_tenant_info(tenant), tenant_fields)}


@console_ns.route("/workspaces/current/permission")
class WorkspacePermissionApi(Resource):
    """Get workspace permissions for the current workspace."""

    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_enterprise
    def get(self):
        """
        Get workspace permission settings.
        Returns permission flags that control workspace features like member invitations and owner transfer.
        """
        _, current_tenant_id = current_account_with_tenant()

        if not current_tenant_id:
            raise ValueError("No current tenant")

        # Get workspace permissions from enterprise service
        permission = EnterpriseService.WorkspacePermissionService.get_permission(current_tenant_id)

        return {
            "workspace_id": permission.workspace_id,
            "allow_member_invite": permission.allow_member_invite,
            "allow_owner_transfer": permission.allow_owner_transfer,
        }, 200

```

### api/controllers/console/workspace/trigger_providers.py
```py
import logging
from typing import Any

from flask import make_response, redirect, request
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel, model_validator
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, Forbidden

from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.web.error import NotFoundError
from core.plugin.entities.plugin_daemon import CredentialType
from core.plugin.impl.oauth import OAuthHandler
from core.trigger.entities.entities import SubscriptionBuilderUpdater
from core.trigger.trigger_manager import TriggerManager
from extensions.ext_database import db
from libs.login import current_user, login_required
from models.account import Account
from models.provider_ids import TriggerProviderID
from services.plugin.oauth_service import OAuthProxyService
from services.trigger.trigger_provider_service import TriggerProviderService
from services.trigger.trigger_subscription_builder_service import TriggerSubscriptionBuilderService
from services.trigger.trigger_subscription_operator_service import TriggerSubscriptionOperatorService

from .. import console_ns
from ..wraps import (
    account_initialization_required,
    edit_permission_required,
    is_admin_or_owner_required,
    setup_required,
)

logger = logging.getLogger(__name__)


class TriggerSubscriptionBuilderCreatePayload(BaseModel):
    credential_type: str = CredentialType.UNAUTHORIZED


class TriggerSubscriptionBuilderVerifyPayload(BaseModel):
    credentials: dict[str, Any]


class TriggerSubscriptionBuilderUpdatePayload(BaseModel):
    name: str | None = None
    parameters: dict[str, Any] | None = None
    properties: dict[str, Any] | None = None
    credentials: dict[str, Any] | None = None

    @model_validator(mode="after")
    def check_at_least_one_field(self):
        if all(v is None for v in self.model_dump().values()):
            raise ValueError("At least one of name, credentials, parameters, or properties must be provided")
        return self


class TriggerOAuthClientPayload(BaseModel):
    client_params: dict[str, Any] | None = None
    enabled: bool | None = None


register_schema_models(
    console_ns,
    TriggerSubscriptionBuilderCreatePayload,
    TriggerSubscriptionBuilderVerifyPayload,
    TriggerSubscriptionBuilderUpdatePayload,
    TriggerOAuthClientPayload,
)


@console_ns.route("/workspaces/current/trigger-provider/<path:provider>/icon")
class TriggerProviderIconApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None

        return TriggerManager.get_trigger_plugin_icon(tenant_id=user.current_tenant_id, provider_id=provider)


@console_ns.route("/workspaces/current/triggers")
class TriggerProviderListApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        """List all trigger providers for the current tenant"""
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None
        return jsonable_encoder(TriggerProviderService.list_trigger_providers(user.current_tenant_id))


@console_ns.route("/workspaces/current/trigger-provider/<path:provider>/info")
class TriggerProviderInfoApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        """Get info for a trigger provider"""
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None
        return jsonable_encoder(
            TriggerProviderService.get_trigger_provider(user.current_tenant_id, TriggerProviderID(provider))
        )


@console_ns.route("/workspaces/current/trigger-provider/<path:provider>/subscriptions/list")
class TriggerSubscriptionListApi(Resource):
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def get(self, provider):
        """List all trigger subscriptions for the current tenant's provider"""
        user = current_user
        assert user.current_tenant_id is not None

        try:
            return jsonable_encoder(
                TriggerProviderService.list_trigger_provider_subscriptions(
                    tenant_id=user.current_tenant_id, provider_id=TriggerProviderID(provider)
                )
            )
        except ValueError as e:
            return jsonable_encoder({"error": str(e)}), 404
        except Exception as e:
            logger.exception("Error listing trigger providers", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/create",
)
class TriggerSubscriptionBuilderCreateApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderCreatePayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, provider):
        """Add a new subscription instance for a trigger provider"""
        user = current_user
        assert user.current_tenant_id is not None

        payload = TriggerSubscriptionBuilderCreatePayload.model_validate(console_ns.payload or {})

        try:
            credential_type = CredentialType.of(payload.credential_type)
            subscription_builder = TriggerSubscriptionBuilderService.create_trigger_subscription_builder(
                tenant_id=user.current_tenant_id,
                user_id=user.id,
                provider_id=TriggerProviderID(provider),
                credential_type=credential_type,
            )
            return jsonable_encoder({"subscription_builder": subscription_builder})
        except Exception as e:
            logger.exception("Error adding provider credential", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/<path:subscription_builder_id>",
)
class TriggerSubscriptionBuilderGetApi(Resource):
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def get(self, provider, subscription_builder_id):
        """Get a subscription instance for a trigger provider"""
        return jsonable_encoder(
            TriggerSubscriptionBuilderService.get_subscription_builder_by_id(subscription_builder_id)
        )


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/verify-and-update/<path:subscription_builder_id>",
)
class TriggerSubscriptionBuilderVerifyApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderVerifyPayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, provider, subscription_builder_id):
        """Verify and update a subscription instance for a trigger provider"""
        user = current_user
        assert user.current_tenant_id is not None

        payload = TriggerSubscriptionBuilderVerifyPayload.model_validate(console_ns.payload or {})

        try:
            # Use atomic update_and_verify to prevent race conditions
            return TriggerSubscriptionBuilderService.update_and_verify_builder(
                tenant_id=user.current_tenant_id,
                user_id=user.id,
                provider_id=TriggerProviderID(provider),
                subscription_builder_id=subscription_builder_id,
                subscription_builder_updater=SubscriptionBuilderUpdater(
                    credentials=payload.credentials,
                ),
            )
        except Exception as e:
            logger.exception("Error verifying provider credential", exc_info=e)
            raise ValueError(str(e)) from e


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/update/<path:subscription_builder_id>",
)
class TriggerSubscriptionBuilderUpdateApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderUpdatePayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, provider, subscription_builder_id):
        """Update a subscription instance for a trigger provider"""
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None

        payload = TriggerSubscriptionBuilderUpdatePayload.model_validate(console_ns.payload or {})
        try:
            return jsonable_encoder(
                TriggerSubscriptionBuilderService.update_trigger_subscription_builder(
                    tenant_id=user.current_tenant_id,
                    provider_id=TriggerProviderID(provider),
                    subscription_builder_id=subscription_builder_id,
                    subscription_builder_updater=SubscriptionBuilderUpdater(
                        name=payload.name,
                        parameters=payload.parameters,
                        properties=payload.properties,
                        credentials=payload.credentials,
                    ),
                )
            )
        except Exception as e:
            logger.exception("Error updating provider credential", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/logs/<path:subscription_builder_id>",
)
class TriggerSubscriptionBuilderLogsApi(Resource):
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def get(self, provider, subscription_builder_id):
        """Get the request logs for a subscription instance for a trigger provider"""
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None

        try:
            logs = TriggerSubscriptionBuilderService.list_logs(subscription_builder_id)
            return jsonable_encoder({"logs": [log.model_dump(mode="json") for log in logs]})
        except Exception as e:
            logger.exception("Error getting request logs for subscription builder", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/builder/build/<path:subscription_builder_id>",
)
class TriggerSubscriptionBuilderBuildApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderUpdatePayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, provider, subscription_builder_id):
        """Build a subscription instance for a trigger provider"""
        user = current_user
        assert user.current_tenant_id is not None
        payload = TriggerSubscriptionBuilderUpdatePayload.model_validate(console_ns.payload or {})
        try:
            # Use atomic update_and_build to prevent race conditions
            TriggerSubscriptionBuilderService.update_and_build_builder(
                tenant_id=user.current_tenant_id,
                user_id=user.id,
                provider_id=TriggerProviderID(provider),
                subscription_builder_id=subscription_builder_id,
                subscription_builder_updater=SubscriptionBuilderUpdater(
                    name=payload.name,
                    parameters=payload.parameters,
                    properties=payload.properties,
                ),
            )
            return 200
        except Exception as e:
            logger.exception("Error building provider credential", exc_info=e)
            raise ValueError(str(e)) from e


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:subscription_id>/subscriptions/update",
)
class TriggerSubscriptionUpdateApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderUpdatePayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, subscription_id: str):
        """Update a subscription instance"""
        user = current_user
        assert user.current_tenant_id is not None

        request = TriggerSubscriptionBuilderUpdatePayload.model_validate(console_ns.payload or {})

        subscription = TriggerProviderService.get_subscription_by_id(
            tenant_id=user.current_tenant_id,
            subscription_id=subscription_id,
        )
        if not subscription:
            raise NotFoundError(f"Subscription {subscription_id} not found")

        provider_id = TriggerProviderID(subscription.provider_id)

        try:
            # For rename only, just update the name
            rename = request.name is not None and not any((request.credentials, request.parameters, request.properties))
            # When credential type is UNAUTHORIZED, it indicates the subscription was manually created
            # For Manually created subscription, they dont have credentials, parameters
            # They only have name and properties(which is input by user)
            manually_created = subscription.credential_type == CredentialType.UNAUTHORIZED
            if rename or manually_created:
                TriggerProviderService.update_trigger_subscription(
                    tenant_id=user.current_tenant_id,
                    subscription_id=subscription_id,
                    name=request.name,
                    properties=request.properties,
                )
                return 200

            # For the rest cases(API_KEY, OAUTH2)
            # we need to call third party provider(e.g. GitHub) to rebuild the subscription
            TriggerProviderService.rebuild_trigger_subscription(
                tenant_id=user.current_tenant_id,
                name=request.name,
                provider_id=provider_id,
                subscription_id=subscription_id,
                credentials=request.credentials or subscription.credentials,
                parameters=request.parameters or subscription.parameters,
            )
            return 200
        except ValueError as e:
            raise BadRequest(str(e))
        except Exception as e:
            logger.exception("Error updating subscription", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:subscription_id>/subscriptions/delete",
)
class TriggerSubscriptionDeleteApi(Resource):
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, subscription_id: str):
        """Delete a subscription instance"""
        user = current_user
        assert user.current_tenant_id is not None

        try:
            with sessionmaker(db.engine).begin() as session:
                # Delete trigger provider subscription
                TriggerProviderService.delete_trigger_provider(
                    session=session,
                    tenant_id=user.current_tenant_id,
                    subscription_id=subscription_id,
                )
                # Delete plugin triggers
                TriggerSubscriptionOperatorService.delete_plugin_trigger_by_subscription(
                    session=session,
                    tenant_id=user.current_tenant_id,
                    subscription_id=subscription_id,
                )
            return {"result": "success"}
        except ValueError as e:
            raise BadRequest(str(e))
        except Exception as e:
            logger.exception("Error deleting provider credential", exc_info=e)
            raise


@console_ns.route("/workspaces/current/trigger-provider/<path:provider>/subscriptions/oauth/authorize")
class TriggerOAuthAuthorizeApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider):
        """Initiate OAuth authorization flow for a trigger provider"""
        user = current_user
        assert isinstance(user, Account)
        assert user.current_tenant_id is not None

        try:
            provider_id = TriggerProviderID(provider)
            plugin_id = provider_id.plugin_id
            provider_name = provider_id.provider_name
            tenant_id = user.current_tenant_id

            # Get OAuth client configuration
            oauth_client_params = TriggerProviderService.get_oauth_client(
                tenant_id=tenant_id,
                provider_id=provider_id,
            )

            if oauth_client_params is None:
                raise NotFoundError("No OAuth client configuration found for this trigger provider")

            # Create subscription builder
            subscription_builder = TriggerSubscriptionBuilderService.create_trigger_subscription_builder(
                tenant_id=tenant_id,
                user_id=user.id,
                provider_id=provider_id,
                credential_type=CredentialType.OAUTH2,
            )

            # Create OAuth handler and proxy context
            oauth_handler = OAuthHandler()
            context_id = OAuthProxyService.create_proxy_context(
                user_id=user.id,
                tenant_id=tenant_id,
                plugin_id=plugin_id,
                provider=provider_name,
                extra_data={
                    "subscription_builder_id": subscription_builder.id,
                },
            )

            # Build redirect URI for callback
            redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider}/trigger/callback"

            # Get authorization URL
            authorization_url_response = oauth_handler.get_authorization_url(
                tenant_id=tenant_id,
                user_id=user.id,
                plugin_id=plugin_id,
                provider=provider_name,
                redirect_uri=redirect_uri,
                system_credentials=oauth_client_params,
            )

            # Create response with cookie
            response = make_response(
                jsonable_encoder(
                    {
                        "authorization_url": authorization_url_response.authorization_url,
                        "subscription_builder_id": subscription_builder.id,
                        "subscription_builder": subscription_builder,
                    }
                )
            )
            response.set_cookie(
                "context_id",
                context_id,
                httponly=True,
                samesite="Lax",
                max_age=OAuthProxyService.__MAX_AGE__,
            )

            return response

        except Exception as e:
            logger.exception("Error initiating OAuth flow", exc_info=e)
            raise


@console_ns.route("/oauth/plugin/<path:provider>/trigger/callback")
class TriggerOAuthCallbackApi(Resource):
    @setup_required
    def get(self, provider):
        """Handle OAuth callback for trigger provider"""
        context_id = request.cookies.get("context_id")
        if not context_id:
            raise Forbidden("context_id not found")

        # Use and validate proxy context
        context = OAuthProxyService.use_proxy_context(context_id)
        if context is None:
            raise Forbidden("Invalid context_id")

        # Parse provider ID
        provider_id = TriggerProviderID(provider)
        plugin_id = provider_id.plugin_id
        provider_name = provider_id.provider_name
        user_id: str = context["user_id"]
        tenant_id: str = context["tenant_id"]
        subscription_builder_id: str = context["subscription_builder_id"]

        # Get OAuth client configuration
        oauth_client_params = TriggerProviderService.get_oauth_client(
            tenant_id=tenant_id,
            provider_id=provider_id,
        )

        if oauth_client_params is None:
            raise Forbidden("No OAuth client configuration found for this trigger provider")

        # Get OAuth credentials from callback
        oauth_handler = OAuthHandler()
        redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider}/trigger/callback"

        credentials_response = oauth_handler.get_credentials(
            tenant_id=tenant_id,
            user_id=user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            redirect_uri=redirect_uri,
            system_credentials=oauth_client_params,
            request=request,
        )

        credentials = credentials_response.credentials
        expires_at = credentials_response.expires_at

        if not credentials:
            raise ValueError("Failed to get OAuth credentials from the provider.")

        # Update subscription builder
        TriggerSubscriptionBuilderService.update_trigger_subscription_builder(
            tenant_id=tenant_id,
            provider_id=provider_id,
            subscription_builder_id=subscription_builder_id,
            subscription_builder_updater=SubscriptionBuilderUpdater(
                credentials=credentials,
                credential_expires_at=expires_at,
            ),
        )
        # Redirect to OAuth callback page
        return redirect(f"{dify_config.CONSOLE_WEB_URL}/oauth-callback")


@console_ns.route("/workspaces/current/trigger-provider/<path:provider>/oauth/client")
class TriggerOAuthClientManageApi(Resource):
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def get(self, provider):
        """Get OAuth client configuration for a provider"""
        user = current_user
        assert user.current_tenant_id is not None

        try:
            provider_id = TriggerProviderID(provider)

            # Get custom OAuth client params if exists
            custom_params = TriggerProviderService.get_custom_oauth_client_params(
                tenant_id=user.current_tenant_id,
                provider_id=provider_id,
            )

            # Check if custom client is enabled
            is_custom_enabled = TriggerProviderService.is_oauth_custom_client_enabled(
                tenant_id=user.current_tenant_id,
                provider_id=provider_id,
            )
            system_client_exists = TriggerProviderService.is_oauth_system_client_exists(
                tenant_id=user.current_tenant_id,
                provider_id=provider_id,
            )
            provider_controller = TriggerManager.get_trigger_provider(user.current_tenant_id, provider_id)
            redirect_uri = f"{dify_config.CONSOLE_API_URL}/console/api/oauth/plugin/{provider}/trigger/callback"
            return jsonable_encoder(
                {
                    "configured": bool(custom_params or system_client_exists),
                    "system_configured": system_client_exists,
                    "custom_configured": bool(custom_params),
                    "oauth_client_schema": provider_controller.get_oauth_client_schema(),
                    "custom_enabled": is_custom_enabled,
                    "redirect_uri": redirect_uri,
                    "params": custom_params or {},
                }
            )

        except Exception as e:
            logger.exception("Error getting OAuth client", exc_info=e)
            raise

    @console_ns.expect(console_ns.models[TriggerOAuthClientPayload.__name__])
    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def post(self, provider):
        """Configure custom OAuth client for a provider"""
        user = current_user
        assert user.current_tenant_id is not None

        payload = TriggerOAuthClientPayload.model_validate(console_ns.payload or {})

        try:
            provider_id = TriggerProviderID(provider)
            return TriggerProviderService.save_custom_oauth_client_params(
                tenant_id=user.current_tenant_id,
                provider_id=provider_id,
                client_params=payload.client_params,
                enabled=payload.enabled,
            )

        except ValueError as e:
            raise BadRequest(str(e))
        except Exception as e:
            logger.exception("Error configuring OAuth client", exc_info=e)
            raise

    @setup_required
    @login_required
    @is_admin_or_owner_required
    @account_initialization_required
    def delete(self, provider):
        """Remove custom OAuth client configuration"""
        user = current_user
        assert user.current_tenant_id is not None

        try:
            provider_id = TriggerProviderID(provider)

            return TriggerProviderService.delete_custom_oauth_client_params(
                tenant_id=user.current_tenant_id,
                provider_id=provider_id,
            )
        except ValueError as e:
            raise BadRequest(str(e))
        except Exception as e:
            logger.exception("Error removing OAuth client", exc_info=e)
            raise


@console_ns.route(
    "/workspaces/current/trigger-provider/<path:provider>/subscriptions/verify/<path:subscription_id>",
)
class TriggerSubscriptionVerifyApi(Resource):
    @console_ns.expect(console_ns.models[TriggerSubscriptionBuilderVerifyPayload.__name__])
    @setup_required
    @login_required
    @edit_permission_required
    @account_initialization_required
    def post(self, provider, subscription_id):
        """Verify credentials for an existing subscription (edit mode only)"""
        user = current_user
        assert user.current_tenant_id is not None

        verify_request = TriggerSubscriptionBuilderVerifyPayload.model_validate(console_ns.payload or {})

        try:
            result = TriggerProviderService.verify_subscription_credentials(
                tenant_id=user.current_tenant_id,
                user_id=user.id,
                provider_id=TriggerProviderID(provider),
                subscription_id=subscription_id,
                credentials=verify_request.credentials,
            )
            return result
        except ValueError as e:
            logger.warning("Credential verification failed", exc_info=e)
            raise BadRequest(str(e)) from e
        except Exception as e:
            logger.exception("Error verifying subscription credentials", exc_info=e)
            raise BadRequest(str(e)) from e

```

### api/controllers/console/workspace/account.py
```py
from __future__ import annotations

from datetime import datetime
from typing import Literal

import pytz
from flask import request
from flask_restx import Resource, fields, marshal_with
from pydantic import BaseModel, Field, field_validator, model_validator
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from constants.languages import supported_language
from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.auth.error import (
    EmailAlreadyInUseError,
    EmailChangeLimitError,
    EmailCodeError,
    InvalidEmailError,
    InvalidTokenError,
)
from controllers.console.error import AccountInFreezeError, AccountNotFound, EmailSendIpLimitError
from controllers.console.workspace.error import (
    AccountAlreadyInitedError,
    CurrentPasswordIncorrectError,
    InvalidAccountDeletionCodeError,
    InvalidInvitationCodeError,
    RepeatPasswordNotMatchError,
)
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_enabled,
    enable_change_email,
    enterprise_license_required,
    only_edition_cloud,
    setup_required,
)
from extensions.ext_database import db
from fields.member_fields import Account as AccountResponse
from libs.datetime_utils import naive_utc_now
from libs.helper import EmailStr, TimestampField, extract_remote_ip, timezone
from libs.login import current_account_with_tenant, login_required
from models import AccountIntegrate, InvitationCode
from models.account import AccountStatus, InvitationCodeStatus
from services.account_service import AccountService
from services.billing_service import BillingService
from services.errors.account import CurrentPasswordIncorrectError as ServiceCurrentPasswordIncorrectError

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class AccountInitPayload(BaseModel):
    interface_language: str
    timezone: str
    invitation_code: str | None = None

    @field_validator("interface_language")
    @classmethod
    def validate_language(cls, value: str) -> str:
        return supported_language(value)

    @field_validator("timezone")
    @classmethod
    def validate_timezone(cls, value: str) -> str:
        return timezone(value)


class AccountNamePayload(BaseModel):
    name: str = Field(min_length=3, max_length=30)


class AccountAvatarPayload(BaseModel):
    avatar: str


class AccountInterfaceLanguagePayload(BaseModel):
    interface_language: str

    @field_validator("interface_language")
    @classmethod
    def validate_language(cls, value: str) -> str:
        return supported_language(value)


class AccountInterfaceThemePayload(BaseModel):
    interface_theme: Literal["light", "dark"]


class AccountTimezonePayload(BaseModel):
    timezone: str

    @field_validator("timezone")
    @classmethod
    def validate_timezone(cls, value: str) -> str:
        return timezone(value)


class AccountPasswordPayload(BaseModel):
    password: str | None = None
    new_password: str
    repeat_new_password: str

    @model_validator(mode="after")
    def check_passwords_match(self) -> AccountPasswordPayload:
        if self.new_password != self.repeat_new_password:
            raise RepeatPasswordNotMatchError()
        return self


class AccountDeletePayload(BaseModel):
    token: str
    code: str


class AccountDeletionFeedbackPayload(BaseModel):
    email: EmailStr
    feedback: str


class EducationActivatePayload(BaseModel):
    token: str
    institution: str
    role: str


class EducationAutocompleteQuery(BaseModel):
    keywords: str
    page: int = 0
    limit: int = 20


class ChangeEmailSendPayload(BaseModel):
    email: EmailStr
    language: str | None = None
    phase: str | None = None
    token: str | None = None


class ChangeEmailValidityPayload(BaseModel):
    email: EmailStr
    code: str
    token: str


class ChangeEmailResetPayload(BaseModel):
    new_email: EmailStr
    token: str


class CheckEmailUniquePayload(BaseModel):
    email: EmailStr


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(AccountInitPayload)
reg(AccountNamePayload)
reg(AccountAvatarPayload)
reg(AccountInterfaceLanguagePayload)
reg(AccountInterfaceThemePayload)
reg(AccountTimezonePayload)
reg(AccountPasswordPayload)
reg(AccountDeletePayload)
reg(AccountDeletionFeedbackPayload)
reg(EducationActivatePayload)
reg(EducationAutocompleteQuery)
reg(ChangeEmailSendPayload)
reg(ChangeEmailValidityPayload)
reg(ChangeEmailResetPayload)
reg(CheckEmailUniquePayload)
register_schema_models(console_ns, AccountResponse)


def _serialize_account(account) -> dict:
    return AccountResponse.model_validate(account, from_attributes=True).model_dump(mode="json")


integrate_fields = {
    "provider": fields.String,
    "created_at": TimestampField,
    "is_bound": fields.Boolean,
    "link": fields.String,
}

integrate_model = console_ns.model("AccountIntegrate", integrate_fields)
integrate_list_model = console_ns.model(
    "AccountIntegrateList",
    {"data": fields.List(fields.Nested(integrate_model))},
)


@console_ns.route("/account/init")
class AccountInitApi(Resource):
    @console_ns.expect(console_ns.models[AccountInitPayload.__name__])
    @setup_required
    @login_required
    def post(self):
        account, _ = current_account_with_tenant()

        if account.status == "active":
            raise AccountAlreadyInitedError()

        payload = console_ns.payload or {}
        args = AccountInitPayload.model_validate(payload)

        if dify_config.EDITION == "CLOUD":
            if not args.invitation_code:
                raise ValueError("invitation_code is required")

            # check invitation code
            invitation_code = db.session.scalar(
                select(InvitationCode)
                .where(
                    InvitationCode.code == args.invitation_code,
                    InvitationCode.status == InvitationCodeStatus.UNUSED,
                )
                .limit(1)
            )

            if not invitation_code:
                raise InvalidInvitationCodeError()

            invitation_code.status = InvitationCodeStatus.USED
            invitation_code.used_at = naive_utc_now()
            invitation_code.used_by_tenant_id = account.current_tenant_id
            invitation_code.used_by_account_id = account.id

        account.interface_language = args.interface_language
        account.timezone = args.timezone
        account.interface_theme = "light"
        account.status = AccountStatus.ACTIVE
        account.initialized_at = naive_utc_now()
        db.session.commit()

        return {"result": "success"}


@console_ns.route("/account/profile")
class AccountProfileApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    @enterprise_license_required
    def get(self):
        current_user, _ = current_account_with_tenant()
        return _serialize_account(current_user)


@console_ns.route("/account/name")
class AccountNameApi(Resource):
    @console_ns.expect(console_ns.models[AccountNamePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountNamePayload.model_validate(payload)
        updated_account = AccountService.update_account(current_user, name=args.name)

        return _serialize_account(updated_account)


@console_ns.route("/account/avatar")
class AccountAvatarApi(Resource):
    @console_ns.expect(console_ns.models[AccountAvatarPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountAvatarPayload.model_validate(payload)

        updated_account = AccountService.update_account(current_user, avatar=args.avatar)

        return _serialize_account(updated_account)


@console_ns.route("/account/interface-language")
class AccountInterfaceLanguageApi(Resource):
    @console_ns.expect(console_ns.models[AccountInterfaceLanguagePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountInterfaceLanguagePayload.model_validate(payload)

        updated_account = AccountService.update_account(current_user, interface_language=args.interface_language)

        return _serialize_account(updated_account)


@console_ns.route("/account/interface-theme")
class AccountInterfaceThemeApi(Resource):
    @console_ns.expect(console_ns.models[AccountInterfaceThemePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountInterfaceThemePayload.model_validate(payload)

        updated_account = AccountService.update_account(current_user, interface_theme=args.interface_theme)

        return _serialize_account(updated_account)


@console_ns.route("/account/timezone")
class AccountTimezoneApi(Resource):
    @console_ns.expect(console_ns.models[AccountTimezonePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountTimezonePayload.model_validate(payload)

        updated_account = AccountService.update_account(current_user, timezone=args.timezone)

        return _serialize_account(updated_account)


@console_ns.route("/account/password")
class AccountPasswordApi(Resource):
    @console_ns.expect(console_ns.models[AccountPasswordPayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = AccountPasswordPayload.model_validate(payload)

        try:
            AccountService.update_account_password(current_user, args.password, args.new_password)
        except ServiceCurrentPasswordIncorrectError:
            raise CurrentPasswordIncorrectError()

        return _serialize_account(current_user)


@console_ns.route("/account/integrates")
class AccountIntegrateApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @marshal_with(integrate_list_model)
    def get(self):
        account, _ = current_account_with_tenant()

        account_integrates = db.session.scalars(
            select(AccountIntegrate).where(AccountIntegrate.account_id == account.id)
        ).all()

        base_url = request.url_root.rstrip("/")
        oauth_base_path = "/console/api/oauth/login"
        providers = ["github", "google"]

        integrate_data = []
        for provider in providers:
            existing_integrate = next((ai for ai in account_integrates if ai.provider == provider), None)
            if existing_integrate:
                integrate_data.append(
                    {
                        "id": existing_integrate.id,
                        "provider": provider,
                        "created_at": existing_integrate.created_at,
                        "is_bound": True,
                        "link": None,
                    }
                )
            else:
                integrate_data.append(
                    {
                        "id": None,
                        "provider": provider,
                        "created_at": None,
                        "is_bound": False,
                        "link": f"{base_url}{oauth_base_path}/{provider}",
                    }
                )

        return {"data": integrate_data}


@console_ns.route("/account/delete/verify")
class AccountDeleteVerifyApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        account, _ = current_account_with_tenant()

        token, code = AccountService.generate_account_deletion_verification_code(account)
        AccountService.send_account_deletion_verification_email(account, code)

        return {"result": "success", "data": token}


@console_ns.route("/account/delete")
class AccountDeleteApi(Resource):
    @console_ns.expect(console_ns.models[AccountDeletePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        account, _ = current_account_with_tenant()

        payload = console_ns.payload or {}
        args = AccountDeletePayload.model_validate(payload)

        if not AccountService.verify_account_deletion_code(args.token, args.code):
            raise InvalidAccountDeletionCodeError()

        AccountService.delete_account(account)

        return {"result": "success"}


@console_ns.route("/account/delete/feedback")
class AccountDeleteUpdateFeedbackApi(Resource):
    @console_ns.expect(console_ns.models[AccountDeletionFeedbackPayload.__name__])
    @setup_required
    def post(self):
        payload = console_ns.payload or {}
        args = AccountDeletionFeedbackPayload.model_validate(payload)

        BillingService.update_account_deletion_feedback(args.email, args.feedback)

        return {"result": "success"}


@console_ns.route("/account/education/verify")
class EducationVerifyApi(Resource):
    verify_fields = {
        "token": fields.String,
    }

    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    @cloud_edition_billing_enabled
    @marshal_with(verify_fields)
    def get(self):
        account, _ = current_account_with_tenant()

        return BillingService.EducationIdentity.verify(account.id, account.email)


@console_ns.route("/account/education")
class EducationApi(Resource):
    status_fields = {
        "result": fields.Boolean,
        "is_student": fields.Boolean,
        "expire_at": TimestampField,
        "allow_refresh": fields.Boolean,
    }

    @console_ns.expect(console_ns.models[EducationActivatePayload.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    @cloud_edition_billing_enabled
    def post(self):
        account, _ = current_account_with_tenant()

        payload = console_ns.payload or {}
        args = EducationActivatePayload.model_validate(payload)

        return BillingService.EducationIdentity.activate(account, args.token, args.institution, args.role)

    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    @cloud_edition_billing_enabled
    @marshal_with(status_fields)
    def get(self):
        account, _ = current_account_with_tenant()

        res = BillingService.EducationIdentity.status(account.id)
        # convert expire_at to UTC timestamp from isoformat
        if res and "expire_at" in res:
            res["expire_at"] = datetime.fromisoformat(res["expire_at"]).astimezone(pytz.utc)
        return res


@console_ns.route("/account/education/autocomplete")
class EducationAutoCompleteApi(Resource):
    data_fields = {
        "data": fields.List(fields.String),
        "curr_page": fields.Integer,
        "has_next": fields.Boolean,
    }

    @console_ns.expect(console_ns.models[EducationAutocompleteQuery.__name__])
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    @cloud_edition_billing_enabled
    @marshal_with(data_fields)
    def get(self):
        payload = request.args.to_dict(flat=True)
        args = EducationAutocompleteQuery.model_validate(payload)

        return BillingService.EducationIdentity.autocomplete(args.keywords, args.page, args.limit)


@console_ns.route("/account/change-email")
class ChangeEmailSendEmailApi(Resource):
    @console_ns.expect(console_ns.models[ChangeEmailSendPayload.__name__])
    @enable_change_email
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        current_user, _ = current_account_with_tenant()
        payload = console_ns.payload or {}
        args = ChangeEmailSendPayload.model_validate(payload)

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"
        account = None
        user_email = None
        email_for_sending = args.email.lower()
        if args.phase is not None and args.phase == "new_email":
            if args.token is None:
                raise InvalidTokenError()

            reset_data = AccountService.get_change_email_data(args.token)
            if reset_data is None:
                raise InvalidTokenError()
            user_email = reset_data.get("email", "")

            if user_email.lower() != current_user.email.lower():
                raise InvalidEmailError()

            user_email = current_user.email
        else:
            with sessionmaker(db.engine).begin() as session:
                account = AccountService.get_account_by_email_with_case_fallback(args.email, session=session)
            if account is None:
                raise AccountNotFound()
            email_for_sending = account.email
            user_email = account.email

        token = AccountService.send_change_email_email(
            account=account,
            email=email_for_sending,
            old_email=user_email,
            language=language,
            phase=args.phase,
        )
        return {"result": "success", "data": token}


@console_ns.route("/account/change-email/validity")
class ChangeEmailCheckApi(Resource):
    @console_ns.expect(console_ns.models[ChangeEmailValidityPayload.__name__])
    @enable_change_email
    @setup_required
    @login_required
    @account_initialization_required
    def post(self):
        payload = console_ns.payload or {}
        args = ChangeEmailValidityPayload.model_validate(payload)

        user_email = args.email.lower()

        is_change_email_error_rate_limit = AccountService.is_change_email_error_rate_limit(user_email)
        if is_change_email_error_rate_limit:
            raise EmailChangeLimitError()

        token_data = AccountService.get_change_email_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        normalized_token_email = token_email.lower() if isinstance(token_email, str) else token_email
        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if args.code != token_data.get("code"):
            AccountService.add_change_email_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_change_email_token(args.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_change_email_token(
            user_email, code=args.code, old_email=token_data.get("old_email"), additional_data={}
        )

        AccountService.reset_change_email_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@console_ns.route("/account/change-email/reset")
class ChangeEmailResetApi(Resource):
    @console_ns.expect(console_ns.models[ChangeEmailResetPayload.__name__])
    @enable_change_email
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[AccountResponse.__name__])
    def post(self):
        payload = console_ns.payload or {}
        args = ChangeEmailResetPayload.model_validate(payload)
        normalized_new_email = args.new_email.lower()

        if AccountService.is_account_in_freeze(normalized_new_email):
            raise AccountInFreezeError()

        if not AccountService.check_email_unique(normalized_new_email):
            raise EmailAlreadyInUseError()

        reset_data = AccountService.get_change_email_data(args.token)
        if not reset_data:
            raise InvalidTokenError()

        AccountService.revoke_change_email_token(args.token)

        old_email = reset_data.get("old_email", "")
        current_user, _ = current_account_with_tenant()
        if current_user.email.lower() != old_email.lower():
            raise AccountNotFound()

        updated_account = AccountService.update_account_email(current_user, email=normalized_new_email)

        AccountService.send_change_email_completed_notify_email(
            email=normalized_new_email,
        )

        return _serialize_account(updated_account)


@console_ns.route("/account/change-email/check-email-unique")
class CheckEmailUnique(Resource):
    @console_ns.expect(console_ns.models[CheckEmailUniquePayload.__name__])
    @setup_required
    def post(self):
        payload = console_ns.payload or {}
        args = CheckEmailUniquePayload.model_validate(payload)
        normalized_email = args.email.lower()
        if AccountService.is_account_in_freeze(normalized_email):
            raise AccountInFreezeError()
        if not AccountService.check_email_unique(normalized_email):
            raise EmailAlreadyInUseError()
        return {"result": "success"}

```

### api/controllers/console/billing/billing.py
```py
import base64
from typing import Literal

from flask import request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field
from werkzeug.exceptions import BadRequest

from controllers.console import console_ns
from controllers.console.wraps import account_initialization_required, only_edition_cloud, setup_required
from enums.cloud_plan import CloudPlan
from libs.login import current_account_with_tenant, login_required
from services.billing_service import BillingService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class SubscriptionQuery(BaseModel):
    plan: Literal[CloudPlan.PROFESSIONAL, CloudPlan.TEAM] = Field(..., description="Subscription plan")
    interval: Literal["month", "year"] = Field(..., description="Billing interval")


class PartnerTenantsPayload(BaseModel):
    click_id: str = Field(..., description="Click Id from partner referral link")


for model in (SubscriptionQuery, PartnerTenantsPayload):
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/billing/subscription")
class Subscription(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        args = SubscriptionQuery.model_validate(request.args.to_dict(flat=True))
        BillingService.is_tenant_owner_or_admin(current_user)
        return BillingService.get_subscription(args.plan, args.interval, current_user.email, current_tenant_id)


@console_ns.route("/billing/invoices")
class Invoices(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        BillingService.is_tenant_owner_or_admin(current_user)
        return BillingService.get_invoices(current_user.email, current_tenant_id)


@console_ns.route("/billing/partners/<string:partner_key>/tenants")
class PartnerTenants(Resource):
    @console_ns.doc("sync_partner_tenants_bindings")
    @console_ns.doc(description="Sync partner tenants bindings")
    @console_ns.doc(params={"partner_key": "Partner key"})
    @console_ns.expect(
        console_ns.model(
            "SyncPartnerTenantsBindingsRequest",
            {"click_id": fields.String(required=True, description="Click Id from partner referral link")},
        )
    )
    @console_ns.response(200, "Tenants synced to partner successfully")
    @console_ns.response(400, "Invalid partner information")
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    def put(self, partner_key: str):
        current_user, _ = current_account_with_tenant()

        try:
            args = PartnerTenantsPayload.model_validate(console_ns.payload or {})
            click_id = args.click_id
            decoded_partner_key = base64.b64decode(partner_key).decode("utf-8")
        except Exception:
            raise BadRequest("Invalid partner_key")

        if not click_id or not decoded_partner_key or not current_user.id:
            raise BadRequest("Invalid partner information")

        return BillingService.sync_partner_tenants_bindings(current_user.id, decoded_partner_key, click_id)

```

### api/controllers/console/billing/compliance.py
```py
from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field

from libs.helper import extract_remote_ip
from libs.login import current_account_with_tenant, login_required
from services.billing_service import BillingService

from .. import console_ns
from ..wraps import account_initialization_required, only_edition_cloud, setup_required


class ComplianceDownloadQuery(BaseModel):
    doc_name: str = Field(..., description="Compliance document name")


console_ns.schema_model(
    ComplianceDownloadQuery.__name__,
    ComplianceDownloadQuery.model_json_schema(ref_template="#/definitions/{model}"),
)


@console_ns.route("/compliance/download")
class ComplianceApi(Resource):
    @console_ns.expect(console_ns.models[ComplianceDownloadQuery.__name__])
    @console_ns.doc("download_compliance_document")
    @console_ns.doc(description="Get compliance document download link")
    @setup_required
    @login_required
    @account_initialization_required
    @only_edition_cloud
    def get(self):
        current_user, current_tenant_id = current_account_with_tenant()
        args = ComplianceDownloadQuery.model_validate(request.args.to_dict(flat=True))

        ip_address = extract_remote_ip(request)
        device_info = request.headers.get("User-Agent", "Unknown device")
        return BillingService.get_compliance_download_link(
            doc_name=args.doc_name,
            account_id=current_user.id,
            tenant_id=current_tenant_id,
            ip=ip_address,
            device_info=device_info,
        )

```

### api/controllers/console/billing/__init__.py
```py

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-012.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
