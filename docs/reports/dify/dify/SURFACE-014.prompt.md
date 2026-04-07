You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-014
- **Kind**: external_call
- **Identifier**: Custom API Tool HTTP Invocation
- **Description**: Executes HTTP requests to external APIs with configurable auth (API key, Bearer token); dynamic URL construction with user-controlled path/query/body parameters
- **Locations**: api/core/tools/custom_tool/tool.py

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

### api/core/tools/custom_tool/tool.py
```py
import json
from collections.abc import Generator
from dataclasses import dataclass
from os import getenv
from typing import Any, Union
from urllib.parse import urlencode

import httpx
from graphon.file.file_manager import download

from core.helper import ssrf_proxy
from core.tools.__base.tool import Tool
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.tool_bundle import ApiToolBundle
from core.tools.entities.tool_entities import ToolEntity, ToolInvokeMessage, ToolProviderType
from core.tools.errors import ToolInvokeError, ToolParameterValidationError, ToolProviderCredentialValidationError

API_TOOL_DEFAULT_TIMEOUT = (
    int(getenv("API_TOOL_DEFAULT_CONNECT_TIMEOUT", "10")),
    int(getenv("API_TOOL_DEFAULT_READ_TIMEOUT", "60")),
)


@dataclass
class ParsedResponse:
    """Represents a parsed HTTP response with type information"""

    content: Union[str, dict]
    is_json: bool

    def to_string(self) -> str:
        """Convert response to string format for credential validation"""
        if isinstance(self.content, dict):
            return json.dumps(self.content, ensure_ascii=False)
        return str(self.content)


class ApiTool(Tool):
    """
    Api tool
    """

    def __init__(self, entity: ToolEntity, api_bundle: ApiToolBundle, runtime: ToolRuntime, provider_id: str):
        super().__init__(entity, runtime)
        self.api_bundle = api_bundle
        self.provider_id = provider_id

    def fork_tool_runtime(self, runtime: ToolRuntime):
        """
        fork a new tool with metadata
        :return: the new tool
        """
        if self.api_bundle is None:
            raise ValueError("api_bundle is required")
        return self.__class__(
            entity=self.entity,
            api_bundle=self.api_bundle.model_copy(),
            runtime=runtime,
            provider_id=self.provider_id,
        )

    def validate_credentials(
        self, credentials: dict[str, Any], parameters: dict[str, Any], format_only: bool = False
    ) -> str:
        """
        validate the credentials for Api tool
        """
        # assemble validate request and request parameters
        headers = self.assembling_request(parameters)

        if format_only:
            return ""

        response = self.do_http_request(self.api_bundle.server_url, self.api_bundle.method, headers, parameters)
        # validate response
        parsed_response = self.validate_and_parse_response(response)
        # For credential validation, always return as string
        return parsed_response.to_string()

    def tool_provider_type(self) -> ToolProviderType:
        return ToolProviderType.API

    def assembling_request(self, parameters: dict[str, Any]) -> dict[str, Any]:
        headers = {}
        if self.runtime is None:
            raise ToolProviderCredentialValidationError("runtime not initialized")

        credentials = self.runtime.credentials or {}
        if "auth_type" not in credentials:
            raise ToolProviderCredentialValidationError("Missing auth_type")

        if credentials["auth_type"] in ("api_key_header", "api_key"):  # backward compatibility:
            api_key_header = "Authorization"

            if "api_key_header" in credentials:
                api_key_header = credentials["api_key_header"]

            if "api_key_value" not in credentials:
                raise ToolProviderCredentialValidationError("Missing api_key_value")
            elif not isinstance(credentials["api_key_value"], str):
                raise ToolProviderCredentialValidationError("api_key_value must be a string")

            if "api_key_header_prefix" in credentials:
                api_key_header_prefix = credentials["api_key_header_prefix"]
                if api_key_header_prefix == "basic" and credentials["api_key_value"]:
                    credentials["api_key_value"] = f"Basic {credentials['api_key_value']}"
                elif api_key_header_prefix == "bearer" and credentials["api_key_value"]:
                    credentials["api_key_value"] = f"Bearer {credentials['api_key_value']}"
                elif api_key_header_prefix == "custom":
                    pass

            headers[api_key_header] = credentials["api_key_value"]

        elif credentials["auth_type"] == "api_key_query":
            # For query parameter authentication, we don't add anything to headers
            # The query parameter will be added in do_http_request method
            pass

        needed_parameters = [parameter for parameter in (self.api_bundle.parameters or []) if parameter.required]
        for parameter in needed_parameters:
            if parameter.required and parameter.name not in parameters:
                if parameter.default is not None:
                    parameters[parameter.name] = parameter.default
                else:
                    raise ToolParameterValidationError(f"Missing required parameter {parameter.name}")

        return headers

    def validate_and_parse_response(self, response: httpx.Response) -> ParsedResponse:
        """
        validate the response and return parsed content with type information

        :return: ParsedResponse with content and is_json flag
        """
        if isinstance(response, httpx.Response):
            if response.status_code >= 400:
                raise ToolInvokeError(f"Request failed with status code {response.status_code} and {response.text}")
            if not response.content:
                return ParsedResponse(
                    "Empty response from the tool, please check your parameters and try again.", False
                )

            # Check content type
            content_type = response.headers.get("content-type", "").lower()
            is_json_content_type = "application/json" in content_type

            # Try to parse as JSON
            try:
                json_response = response.json()
                # If content-type indicates JSON, return as JSON object
                if is_json_content_type:
                    return ParsedResponse(json_response, True)
                else:
                    # If content-type doesn't indicate JSON, treat as text regardless of content
                    return ParsedResponse(response.text, False)
            except Exception:
                # Not valid JSON, return as text
                return ParsedResponse(response.text, False)
        else:
            raise ValueError(f"Invalid response type {type(response)}")

    @staticmethod
    def get_parameter_value(parameter, parameters):
        if parameter["name"] in parameters:
            return parameters[parameter["name"]]
        elif parameter.get("required", False):
            raise ToolParameterValidationError(f"Missing required parameter {parameter['name']}")
        else:
            return (parameter.get("schema", {}) or {}).get("default", "")

    def do_http_request(
        self, url: str, method: str, headers: dict[str, Any], parameters: dict[str, Any]
    ) -> httpx.Response:
        """
        do http request depending on api bundle
        """
        method = method.lower()

        params = {}
        path_params = {}
        # FIXME: body should be a dict[str, Any] but it changed a lot in this function
        body: Any = {}
        cookies = {}
        files = []

        # Add API key to query parameters if auth_type is api_key_query
        if self.runtime and self.runtime.credentials:
            credentials = self.runtime.credentials
            if credentials.get("auth_type") == "api_key_query":
                api_key_query_param = credentials.get("api_key_query_param", "key")
                api_key_value = credentials.get("api_key_value")
                if api_key_value:
                    params[api_key_query_param] = api_key_value

        # check parameters
        for parameter in self.api_bundle.openapi.get("parameters", []):
            value = self.get_parameter_value(parameter, parameters)
            if parameter["in"] == "path":
                path_params[parameter["name"]] = value

            elif parameter["in"] == "query":
                if value != "":
                    params[parameter["name"]] = value

            elif parameter["in"] == "cookie":
                cookies[parameter["name"]] = value

            elif parameter["in"] == "header":
                headers[parameter["name"]] = str(value)

        # check if there is a request body and handle it
        if "requestBody" in self.api_bundle.openapi and self.api_bundle.openapi["requestBody"] is not None:
            # handle json request body
            if "content" in self.api_bundle.openapi["requestBody"]:
                for content_type in self.api_bundle.openapi["requestBody"]["content"]:
                    headers["Content-Type"] = content_type
                    body_schema = self.api_bundle.openapi["requestBody"]["content"][content_type]["schema"]

                    # handle ref schema
                    if "$ref" in body_schema:
                        ref_path = body_schema["$ref"].split("/")
                        ref_name = ref_path[-1]
                        if (
                            "components" in self.api_bundle.openapi
                            and "schemas" in self.api_bundle.openapi["components"]
                        ):
                            if ref_name in self.api_bundle.openapi["components"]["schemas"]:
                                body_schema = self.api_bundle.openapi["components"]["schemas"][ref_name]

                    required = body_schema.get("required", [])
                    properties = body_schema.get("properties", {})
                    for name, property in properties.items():
                        if name in parameters:
                            # multiple file upload: if the type is array and the items have format as binary
                            if property.get("type") == "array" and property.get("items", {}).get("format") == "binary":
                                # parameters[name] should be a list of file objects.
                                for f in parameters[name]:
                                    files.append((name, (f.filename, download(f), f.mime_type)))
                            elif property.get("format") == "binary":
                                f = parameters[name]
                                files.append((name, (f.filename, download(f), f.mime_type)))
                            elif "$ref" in property:
                                body[name] = parameters[name]
                            else:
                                # convert type
                                body[name] = self._convert_body_property_type(property, parameters[name])
                        elif name in required:
                            raise ToolParameterValidationError(
                                f"Missing required parameter {name} in operation {self.api_bundle.operation_id}"
                            )
                        elif "default" in property:
                            body[name] = property["default"]
                        else:
                            # omit optional parameters that weren't provided, instead of setting them to None
                            pass
                    break

        # replace path parameters
        for name, value in path_params.items():
            url = url.replace(f"{{{name}}}", f"{value}")

        # parse http body data if needed
        if "Content-Type" in headers:
            if headers["Content-Type"] == "application/json":
                body = json.dumps(body)
            elif headers["Content-Type"] == "application/x-www-form-urlencoded":
                body = urlencode(body)
            else:
                body = body

        # if there is a file upload, remove the Content-Type header
        # so that httpx can automatically generate the boundary header required for multipart/form-data.
        # issue: https://github.com/langgenius/dify/issues/13684
        # reference: https://stackoverflow.com/questions/39280438/fetch-missing-boundary-in-multipart-form-data-post
        if files:
            headers.pop("Content-Type", None)

        _METHOD_MAP = {
            "get": ssrf_proxy.get,
            "head": ssrf_proxy.head,
            "post": ssrf_proxy.post,
            "put": ssrf_proxy.put,
            "delete": ssrf_proxy.delete,
            "patch": ssrf_proxy.patch,
        }
        method_lc = method.lower()
        if method_lc not in _METHOD_MAP:
            raise ValueError(f"Invalid http method {method}")
        response: httpx.Response = _METHOD_MAP[
            method_lc
        ](  # https://discuss.python.org/t/type-inference-for-function-return-types/42926
            url,
            max_retries=0,
            params=params,
            headers=headers,
            cookies=cookies,
            data=body,
            files=files,
            timeout=API_TOOL_DEFAULT_TIMEOUT,
            follow_redirects=True,
        )
        return response

    def _convert_body_property_any_of(
        self, property: dict[str, Any], value: Any, any_of: list[dict[str, Any]], max_recursive=10
    ):
        if max_recursive <= 0:
            raise Exception("Max recursion depth reached")
        for option in any_of or []:
            try:
                if "type" in option:
                    # Attempt to convert the value based on the type.
                    if option["type"] == "integer" or option["type"] == "int":
                        return int(value)
                    elif option["type"] == "number":
                        if "." in str(value):
                            return float(value)
                        else:
                            return int(value)
                    elif option["type"] == "string":
                        return str(value)
                    elif option["type"] == "boolean":
                        if str(value).lower() in {"true", "1"}:
                            return True
                        elif str(value).lower() in {"false", "0"}:
                            return False
                        else:
                            continue  # Not a boolean, try next option
                    elif option["type"] == "null" and not value:
                        return None
                    else:
                        continue  # Unsupported type, try next option
                elif "anyOf" in option and isinstance(option["anyOf"], list):
                    # Recursive call to handle nested anyOf
                    return self._convert_body_property_any_of(property, value, option["anyOf"], max_recursive - 1)
            except ValueError:
                continue  # Conversion failed, try next option
        # If no option succeeded, you might want to return the value as is or raise an error
        return value  # or raise ValueError(f"Cannot convert value '{value}' to any specified type in anyOf")

    def _convert_body_property_type(self, property: dict[str, Any], value: Any):
        try:
            if "type" in property:
                if property["type"] == "integer" or property["type"] == "int":
                    return int(value)
                elif property["type"] == "number":
                    # check if it is a float
                    if "." in str(value):
                        return float(value)
                    else:
                        return int(value)
                elif property["type"] == "string":
                    return str(value)
                elif property["type"] == "boolean":
                    return bool(value)
                elif property["type"] == "null":
                    if value is None:
                        return None
                elif property["type"] == "object" or property["type"] == "array":
                    if isinstance(value, str):
                        try:
                            return json.loads(value)
                        except ValueError:
                            return value
                    elif isinstance(value, dict):
                        return value
                    else:
                        return value
                else:
                    raise ValueError(f"Invalid type {property['type']} for property {property}")
            elif "anyOf" in property and isinstance(property["anyOf"], list):
                return self._convert_body_property_any_of(property, value, property["anyOf"])
        except ValueError:
            return value

    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke http request
        """
        response: httpx.Response | str = ""
        # assemble request
        headers = self.assembling_request(tool_parameters)

        # do http request
        response = self.do_http_request(self.api_bundle.server_url, self.api_bundle.method, headers, tool_parameters)

        # validate response
        parsed_response = self.validate_and_parse_response(response)

        # assemble invoke message based on response type
        if parsed_response.is_json:
            if isinstance(parsed_response.content, dict):
                yield self.create_json_message(parsed_response.content)

            # The yield below must be preserved to keep backward compatibility.
            #
            # ref: https://github.com/langgenius/dify/pull/23456#issuecomment-3182413088
            yield self.create_text_message(response.text)
        else:
            # Convert to string if needed and create text message
            text_response = (
                parsed_response.content if isinstance(parsed_response.content, str) else str(parsed_response.content)
            )
            yield self.create_text_message(text_response)

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
