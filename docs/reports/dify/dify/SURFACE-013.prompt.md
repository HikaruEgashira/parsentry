You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-013
- **Kind**: endpoint
- **Identifier**: POST /mcp/server/<server_code>/mcp
- **Description**: MCP JSON-RPC server endpoint allowing method invocation on registered MCP servers; method dispatch could be abused for unauthorized operations
- **Locations**: api/controllers/mcp/mcp.py, api/core/mcp/server/

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

### api/controllers/mcp/mcp.py
```py
from typing import Any, Union

from flask import Response
from flask_restx import Resource
from graphon.variables.input_entities import VariableEntity
from pydantic import BaseModel, Field, ValidationError
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
        mcp_server = session.query(AppMCPServer).where(AppMCPServer.server_code == server_code).first()
        if not mcp_server:
            raise MCPRequestError(mcp_types.INVALID_REQUEST, "Server Not Found")

        app = session.query(App).where(App.id == mcp_server.app_id).first()
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
            return (
                session.query(EndUser)
                .where(EndUser.tenant_id == tenant_id)
                .where(EndUser.session_id == mcp_server_id)
                .where(EndUser.type == "mcp")
                .first()
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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
