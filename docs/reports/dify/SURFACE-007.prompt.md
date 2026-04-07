You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-007
- **Kind**: endpoint
- **Identifier**: Code Execution Engine
- **Description**: Executes user-defined code (Python/JavaScript) within workflows. Critical RCE surface — sandbox escape, resource exhaustion, and data exfiltration via code execution are primary risks.
- **Locations**: api/core/helper/code_executor/

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

### api/core/helper/code_executor/code_executor.py
```py
import logging
from collections.abc import Mapping
from threading import Lock
from typing import Any

import httpx
from graphon.nodes.code.entities import CodeLanguage
from pydantic import BaseModel
from yarl import URL

from configs import dify_config
from core.helper.code_executor.javascript.javascript_transformer import NodeJsTemplateTransformer
from core.helper.code_executor.jinja2.jinja2_transformer import Jinja2TemplateTransformer
from core.helper.code_executor.python3.python3_transformer import Python3TemplateTransformer
from core.helper.code_executor.template_transformer import TemplateTransformer
from core.helper.http_client_pooling import get_pooled_http_client

logger = logging.getLogger(__name__)
code_execution_endpoint_url = URL(str(dify_config.CODE_EXECUTION_ENDPOINT))
CODE_EXECUTION_SSL_VERIFY = dify_config.CODE_EXECUTION_SSL_VERIFY
_CODE_EXECUTOR_CLIENT_LIMITS = httpx.Limits(
    max_connections=dify_config.CODE_EXECUTION_POOL_MAX_CONNECTIONS,
    max_keepalive_connections=dify_config.CODE_EXECUTION_POOL_MAX_KEEPALIVE_CONNECTIONS,
    keepalive_expiry=dify_config.CODE_EXECUTION_POOL_KEEPALIVE_EXPIRY,
)
_CODE_EXECUTOR_CLIENT_KEY = "code_executor:http_client"


class CodeExecutionError(Exception):
    pass


class CodeExecutionResponse(BaseModel):
    class Data(BaseModel):
        stdout: str | None = None
        error: str | None = None

    code: int
    message: str
    data: Data


def _build_code_executor_client() -> httpx.Client:
    return httpx.Client(
        verify=CODE_EXECUTION_SSL_VERIFY,
        limits=_CODE_EXECUTOR_CLIENT_LIMITS,
    )


class CodeExecutor:
    dependencies_cache: dict[str, str] = {}
    dependencies_cache_lock = Lock()

    code_template_transformers: dict[CodeLanguage, type[TemplateTransformer]] = {
        CodeLanguage.PYTHON3: Python3TemplateTransformer,
        CodeLanguage.JINJA2: Jinja2TemplateTransformer,
        CodeLanguage.JAVASCRIPT: NodeJsTemplateTransformer,
    }

    code_language_to_running_language = {
        CodeLanguage.JAVASCRIPT: "nodejs",
        CodeLanguage.JINJA2: CodeLanguage.PYTHON3,
        CodeLanguage.PYTHON3: CodeLanguage.PYTHON3,
    }

    supported_dependencies_languages: set[CodeLanguage] = {CodeLanguage.PYTHON3}

    @classmethod
    def execute_code(cls, language: CodeLanguage, preload: str, code: str) -> str:
        """
        Execute code
        :param language: code language
        :param preload: the preload script
        :param code: code
        :return:
        """
        url = code_execution_endpoint_url / "v1" / "sandbox" / "run"

        headers = {"X-Api-Key": dify_config.CODE_EXECUTION_API_KEY}

        data = {
            "language": cls.code_language_to_running_language.get(language),
            "code": code,
            "preload": preload,
            "enable_network": True,
        }

        timeout = httpx.Timeout(
            connect=dify_config.CODE_EXECUTION_CONNECT_TIMEOUT,
            read=dify_config.CODE_EXECUTION_READ_TIMEOUT,
            write=dify_config.CODE_EXECUTION_WRITE_TIMEOUT,
            pool=None,
        )

        client = get_pooled_http_client(_CODE_EXECUTOR_CLIENT_KEY, _build_code_executor_client)

        try:
            response = client.post(
                str(url),
                json=data,
                headers=headers,
                timeout=timeout,
            )
            if response.status_code == 503:
                raise CodeExecutionError("Code execution service is unavailable")
            elif response.status_code != 200:
                raise Exception(
                    f"Failed to execute code, got status code {response.status_code},"
                    f" please check if the sandbox service is running"
                )
        except CodeExecutionError as e:
            raise e
        except Exception as e:
            raise CodeExecutionError(
                "Failed to execute code, which is likely a network issue,"
                " please check if the sandbox service is running."
                f" ( Error: {str(e)} )"
            )

        try:
            response_data = response.json()
        except Exception as e:
            raise CodeExecutionError("Failed to parse response") from e

        if (code := response_data.get("code")) != 0:
            raise CodeExecutionError(f"Got error code: {code}. Got error msg: {response_data.get('message')}")

        response_code = CodeExecutionResponse.model_validate(response_data)

        if response_code.data.error:
            raise CodeExecutionError(response_code.data.error)

        return response_code.data.stdout or ""

    @classmethod
    def execute_workflow_code_template(cls, language: CodeLanguage, code: str, inputs: Mapping[str, Any]):
        """
        Execute code
        :param language: code language
        :param code: code
        :param inputs: inputs
        :return:
        """
        template_transformer = cls.code_template_transformers.get(language)
        if not template_transformer:
            raise CodeExecutionError(f"Unsupported language {language}")

        runner, preload = template_transformer.transform_caller(code, inputs)
        response = cls.execute_code(language, preload, runner)
        return template_transformer.transform_response(response)

```

### api/core/helper/code_executor/python3/python3_code_provider.py
```py
from textwrap import dedent

from core.helper.code_executor.code_executor import CodeLanguage
from core.helper.code_executor.code_node_provider import CodeNodeProvider


class Python3CodeProvider(CodeNodeProvider):
    @staticmethod
    def get_language() -> str:
        return CodeLanguage.PYTHON3

    @classmethod
    def get_default_code(cls) -> str:
        return dedent(
            """
            def main(arg1: str, arg2: str):
                return {
                    "result": arg1 + arg2,
                }
            """
        )

```

### api/core/helper/code_executor/python3/__init__.py
```py

```

### api/core/helper/code_executor/python3/python3_transformer.py
```py
from textwrap import dedent

from core.helper.code_executor.template_transformer import TemplateTransformer


class Python3TemplateTransformer(TemplateTransformer):
    @classmethod
    def get_runner_script(cls) -> str:
        runner_script = dedent(f"""            {cls._code_placeholder}

            import json
            from base64 import b64decode

            # decode and prepare input dict
            inputs_obj = json.loads(b64decode('{cls._inputs_placeholder}').decode('utf-8'))

            # execute main function
            output_obj = main(**inputs_obj)

            # convert output to json and print
            output_json = json.dumps(output_obj, indent=4)
            result = f'''<<RESULT>>{{output_json}}<<RESULT>>'''
            print(result)
            """)
        return runner_script

```

### api/core/helper/code_executor/jinja2/jinja2_formatter.py
```py
from collections.abc import Mapping

from core.helper.code_executor.code_executor import CodeExecutor, CodeLanguage


class Jinja2Formatter:
    @classmethod
    def format(cls, template: str, inputs: Mapping[str, str]) -> str:
        """
        Format template
        :param template: template
        :param inputs: inputs
        :return:
        """
        result = CodeExecutor.execute_workflow_code_template(language=CodeLanguage.JINJA2, code=template, inputs=inputs)
        return str(result.get("result", ""))

```

### api/core/helper/code_executor/jinja2/__init__.py
```py

```

### api/core/helper/code_executor/jinja2/jinja2_transformer.py
```py
from collections.abc import Mapping
from textwrap import dedent
from typing import Any

from core.helper.code_executor.template_transformer import TemplateTransformer


class Jinja2TemplateTransformer(TemplateTransformer):
    # Use separate placeholder for base64-encoded template to avoid confusion
    _template_b64_placeholder: str = "{{template_b64}}"

    @classmethod
    def transform_response(cls, response: str):
        """
        Transform response to dict
        :param response: response
        :return:
        """
        return {"result": cls.extract_result_str_from_response(response)}

    @classmethod
    def assemble_runner_script(cls, code: str, inputs: Mapping[str, Any]) -> str:
        """
        Override base class to use base64 encoding for template code.
        This prevents issues with special characters (quotes, newlines) in templates
        breaking the generated Python script. Fixes #26818.
        """
        script = cls.get_runner_script()
        # Encode template as base64 to safely embed any content including quotes
        code_b64 = cls.serialize_code(code)
        script = script.replace(cls._template_b64_placeholder, code_b64)
        inputs_str = cls.serialize_inputs(inputs)
        script = script.replace(cls._inputs_placeholder, inputs_str)
        return script

    @classmethod
    def get_runner_script(cls) -> str:
        runner_script = dedent(f"""
            import jinja2
            import json
            from base64 import b64decode

            # declare main function
            def main(**inputs):
                # Decode base64-encoded template to handle special characters safely
                template_code = b64decode('{cls._template_b64_placeholder}').decode('utf-8')
                template = jinja2.Template(template_code)
                return template.render(**inputs)

            # decode and prepare input dict
            inputs_obj = json.loads(b64decode('{cls._inputs_placeholder}').decode('utf-8'))

            # execute main function
            output = main(**inputs_obj)

            # convert output and print
            result = f'''<<RESULT>>{{output}}<<RESULT>>'''
            print(result)

            """)
        return runner_script

    @classmethod
    def get_preload_script(cls) -> str:
        preload_script = dedent("""
            import jinja2
            from base64 import b64decode

            def _jinja2_preload_():
                # prepare jinja2 environment, load template and render before to avoid sandbox issue
                template = jinja2.Template('{{s}}')
                template.render(s='a')

            if __name__ == '__main__':
                _jinja2_preload_()

            """)

        return preload_script

```

### api/core/helper/code_executor/__init__.py
```py
from .code_executor import CodeExecutor, CodeLanguage

__all__ = ["CodeExecutor", "CodeLanguage"]

```

### api/core/helper/code_executor/template_transformer.py
```py
import json
import re
from abc import ABC, abstractmethod
from base64 import b64encode
from collections.abc import Mapping
from typing import Any

from graphon.variables.utils import dumps_with_segments


class TemplateTransformer(ABC):
    _code_placeholder: str = "{{code}}"
    _inputs_placeholder: str = "{{inputs}}"
    _result_tag: str = "<<RESULT>>"

    @classmethod
    def serialize_code(cls, code: str) -> str:
        """
        Serialize template code to base64 to safely embed in generated script.
        This prevents issues with special characters like quotes breaking the script.
        """
        code_bytes = code.encode("utf-8")
        return b64encode(code_bytes).decode("utf-8")

    @classmethod
    def transform_caller(cls, code: str, inputs: Mapping[str, Any]) -> tuple[str, str]:
        """
        Transform code to python runner
        :param code: code
        :param inputs: inputs
        :return: runner, preload
        """
        runner_script = cls.assemble_runner_script(code, inputs)
        preload_script = cls.get_preload_script()

        return runner_script, preload_script

    @classmethod
    def extract_result_str_from_response(cls, response: str):
        result = re.search(rf"{cls._result_tag}(.*){cls._result_tag}", response, re.DOTALL)
        if not result:
            raise ValueError(f"Failed to parse result: no result tag found in response. Response: {response[:200]}...")
        return result.group(1)

    @classmethod
    def transform_response(cls, response: str) -> Mapping[str, Any]:
        """
        Transform response to dict
        :param response: response
        :return:
        """

        try:
            result_str = cls.extract_result_str_from_response(response)
            result = json.loads(result_str)
        except json.JSONDecodeError as e:
            raise ValueError(f"Failed to parse JSON response: {str(e)}.")
        except ValueError as e:
            # Re-raise ValueError from extract_result_str_from_response
            raise e
        except Exception as e:
            raise ValueError(f"Unexpected error during response transformation: {str(e)}")

        if not isinstance(result, dict):
            raise ValueError(f"Result must be a dict, got {type(result).__name__}")
        if not all(isinstance(k, str) for k in result):
            raise ValueError("Result keys must be strings")

        # Post-process the result to convert scientific notation strings back to numbers
        result = cls._post_process_result(result)
        return result

    @classmethod
    def _post_process_result(cls, result: dict[Any, Any]) -> dict[Any, Any]:
        """
        Post-process the result to convert scientific notation strings back to numbers
        """

        def convert_scientific_notation(value: Any) -> Any:
            if isinstance(value, str):
                # Check if the string looks like scientific notation
                if re.match(r"^-?\d+\.?\d*e[+-]\d+$", value, re.IGNORECASE):
                    try:
                        return float(value)
                    except ValueError:
                        pass
            elif isinstance(value, dict):
                return {k: convert_scientific_notation(v) for k, v in value.items()}
            elif isinstance(value, list):
                return [convert_scientific_notation(v) for v in value]
            return value

        return convert_scientific_notation(result)

    @classmethod
    @abstractmethod
    def get_runner_script(cls) -> str:
        """
        Get runner script
        """
        pass

    @classmethod
    def serialize_inputs(cls, inputs: Mapping[str, Any]) -> str:
        inputs_json_str = dumps_with_segments(inputs, ensure_ascii=False).encode()
        input_base64_encoded = b64encode(inputs_json_str).decode("utf-8")
        return input_base64_encoded

    @classmethod
    def assemble_runner_script(cls, code: str, inputs: Mapping[str, Any]) -> str:
        # assemble runner script
        script = cls.get_runner_script()
        script = script.replace(cls._code_placeholder, code)
        inputs_str = cls.serialize_inputs(inputs)
        script = script.replace(cls._inputs_placeholder, inputs_str)
        return script

    @classmethod
    def get_preload_script(cls) -> str:
        """
        Get preload script
        """
        return ""

```

### api/core/helper/code_executor/code_node_provider.py
```py
from abc import ABC, abstractmethod
from collections.abc import Mapping, Sequence
from typing import TypedDict

from pydantic import BaseModel


class VariableConfig(TypedDict):
    variable: str
    value_selector: Sequence[str | int]


class OutputConfig(TypedDict):
    type: str
    children: None


class CodeConfig(TypedDict):
    variables: Sequence[VariableConfig]
    code_language: str
    code: str
    outputs: Mapping[str, OutputConfig]


class DefaultConfig(TypedDict):
    type: str
    config: CodeConfig


class CodeNodeProvider(BaseModel, ABC):
    @staticmethod
    @abstractmethod
    def get_language() -> str:
        pass

    @classmethod
    def is_accept_language(cls, language: str) -> bool:
        return language == cls.get_language()

    @classmethod
    @abstractmethod
    def get_default_code(cls) -> str:
        """
        get default code in specific programming language for the code node
        """
        pass

    @classmethod
    def get_default_config(cls) -> DefaultConfig:
        variables: list[VariableConfig] = [
            {"variable": "arg1", "value_selector": []},
            {"variable": "arg2", "value_selector": []},
        ]
        outputs: dict[str, OutputConfig] = {"result": {"type": "string", "children": None}}

        config: CodeConfig = {
            "variables": variables,
            "code_language": cls.get_language(),
            "code": cls.get_default_code(),
            "outputs": outputs,
        }
        return {"type": "code", "config": config}

```

### api/core/helper/code_executor/javascript/__init__.py
```py

```

### api/core/helper/code_executor/javascript/javascript_transformer.py
```py
from textwrap import dedent

from core.helper.code_executor.template_transformer import TemplateTransformer


class NodeJsTemplateTransformer(TemplateTransformer):
    @classmethod
    def get_runner_script(cls) -> str:
        runner_script = dedent(f"""            {cls._code_placeholder}

            // decode and prepare input object
            var inputs_obj = JSON.parse(Buffer.from('{cls._inputs_placeholder}', 'base64').toString('utf-8'))

            // execute main function
            var output_obj = main(inputs_obj)

            // convert output to json and print
            var output_json = JSON.stringify(output_obj)
            var result = `<<RESULT>>${{output_json}}<<RESULT>>`
            console.log(result)
            """)
        return runner_script

```

### api/core/helper/code_executor/javascript/javascript_code_provider.py
```py
from textwrap import dedent

from core.helper.code_executor.code_executor import CodeLanguage
from core.helper.code_executor.code_node_provider import CodeNodeProvider


class JavascriptCodeProvider(CodeNodeProvider):
    @staticmethod
    def get_language() -> str:
        return CodeLanguage.JAVASCRIPT

    @classmethod
    def get_default_code(cls) -> str:
        return dedent(
            """
            function main({arg1, arg2}) {
                return {
                    result: arg1 + arg2
                }
            }
            """
        )

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-007.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
