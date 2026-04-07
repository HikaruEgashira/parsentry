You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-018
- **Kind**: endpoint
- **Identifier**: Tool System (Builtin, Custom, MCP, Plugin, Workflow-as-Tool)
- **Description**: Extensible tool system allowing builtin, custom API, MCP, plugin, and workflow-based tools. Risk of tool parameter injection, unauthorized API calls via custom tools, and credential exposure in tool configurations.
- **Locations**: api/core/tools/builtin_tool/, api/core/tools/custom_tool/, api/core/tools/mcp_tool/, api/core/tools/plugin_tool/, api/core/tools/workflow_as_tool/

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

### api/core/tools/builtin_tool/provider.py
```py
from abc import abstractmethod
from os import listdir, path
from typing import Any

from core.entities.provider_entities import ProviderConfig
from core.helper.module_import_helper import load_single_subclass_from_source
from core.plugin.entities.plugin_daemon import CredentialType
from core.tools.__base.tool_provider import ToolProviderController
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import (
    OAuthSchema,
    ToolEntity,
    ToolProviderEntity,
    ToolProviderType,
)
from core.tools.entities.values import ToolLabelEnum, default_tool_label_dict
from core.tools.errors import (
    ToolProviderNotFoundError,
)
from core.tools.utils.yaml_utils import load_yaml_file_cached


class BuiltinToolProviderController(ToolProviderController):
    tools: list[BuiltinTool]

    def __init__(self, **data: Any):
        self.tools = []

        # load provider yaml
        provider = self.__class__.__module__.split(".")[-1]
        yaml_path = path.join(path.dirname(path.realpath(__file__)), "providers", provider, f"{provider}.yaml")
        try:
            provider_yaml = load_yaml_file_cached(yaml_path)
        except Exception as e:
            raise ToolProviderNotFoundError(f"can not load provider yaml for {provider}: {e}")

        if "credentials_for_provider" in provider_yaml and provider_yaml["credentials_for_provider"] is not None:
            # set credentials name
            for credential_name in provider_yaml["credentials_for_provider"]:
                provider_yaml["credentials_for_provider"][credential_name]["name"] = credential_name

        credentials_schema = []
        for credential in provider_yaml.get("credentials_for_provider", {}):
            credential_dict = provider_yaml.get("credentials_for_provider", {}).get(credential, {})
            credentials_schema.append(credential_dict)

        oauth_schema = None
        if provider_yaml.get("oauth_schema", None) is not None:
            oauth_schema = OAuthSchema(
                client_schema=provider_yaml.get("oauth_schema", {}).get("client_schema", []),
                credentials_schema=provider_yaml.get("oauth_schema", {}).get("credentials_schema", []),
            )

        super().__init__(
            entity=ToolProviderEntity(
                identity=provider_yaml["identity"],
                credentials_schema=credentials_schema,
                oauth_schema=oauth_schema,
            ),
        )

        self._load_tools()

    def _load_tools(self):
        provider = self.entity.identity.name
        tool_path = path.join(path.dirname(path.realpath(__file__)), "providers", provider, "tools")
        # get all the yaml files in the tool path
        tool_files = list(filter(lambda x: x.endswith(".yaml") and not x.startswith("__"), listdir(tool_path)))
        tools = []
        for tool_file in tool_files:
            # get tool name
            tool_name = tool_file.split(".")[0]
            tool = load_yaml_file_cached(path.join(tool_path, tool_file))

            # get tool class, import the module
            assistant_tool_class: type = load_single_subclass_from_source(
                module_name=f"core.tools.builtin_tool.providers.{provider}.tools.{tool_name}",
                script_path=path.join(
                    path.dirname(path.realpath(__file__)),
                    "builtin_tool",
                    "providers",
                    provider,
                    "tools",
                    f"{tool_name}.py",
                ),
                parent_type=BuiltinTool,
            )
            tool["identity"]["provider"] = provider
            tools.append(
                assistant_tool_class(
                    provider=provider,
                    entity=ToolEntity.model_validate(tool),
                    runtime=ToolRuntime(tenant_id=""),
                )
            )

        self.tools = tools

    def _get_builtin_tools(self) -> list[BuiltinTool]:
        """
        returns a list of tools that the provider can provide

        :return: list of tools
        """
        return self.tools

    def get_credentials_schema(self) -> list[ProviderConfig]:
        """
        returns the credentials schema of the provider

        :return: the credentials schema
        """
        return self.get_credentials_schema_by_type(CredentialType.API_KEY)

    def get_credentials_schema_by_type(self, credential_type: CredentialType | str) -> list[ProviderConfig]:
        """
        returns the credentials schema of the provider

        :param credential_type: the type of the credential, as CredentialType or str; str values
            are normalized via CredentialType.of and may raise ValueError for invalid values.
        :return: list[ProviderConfig] for CredentialType.OAUTH2 or CredentialType.API_KEY, an
            empty list for CredentialType.UNAUTHORIZED or missing schemas.

        Reads from self.entity.oauth_schema and self.entity.credentials_schema.
        Raises ValueError for invalid credential types.
        """
        if isinstance(credential_type, str):
            credential_type = CredentialType.of(credential_type)
        if credential_type == CredentialType.OAUTH2:
            return self.entity.oauth_schema.credentials_schema.copy() if self.entity.oauth_schema else []
        if credential_type == CredentialType.API_KEY:
            return self.entity.credentials_schema.copy() if self.entity.credentials_schema else []
        if credential_type == CredentialType.UNAUTHORIZED:
            return []
        raise ValueError(f"Invalid credential type: {credential_type}")

    def get_oauth_client_schema(self) -> list[ProviderConfig]:
        """
        returns the oauth client schema of the provider

        :return: the oauth client schema
        """
        return self.entity.oauth_schema.client_schema.copy() if self.entity.oauth_schema else []

    def get_supported_credential_types(self) -> list[CredentialType]:
        """
        returns the credential support type of the provider
        """
        types = []
        if self.entity.credentials_schema is not None and len(self.entity.credentials_schema) > 0:
            types.append(CredentialType.API_KEY)
        if self.entity.oauth_schema is not None and len(self.entity.oauth_schema.credentials_schema) > 0:
            types.append(CredentialType.OAUTH2)
        return types

    def get_tools(self) -> list[BuiltinTool]:
        """
        returns a list of tools that the provider can provide

        :return: list of tools
        """
        return self._get_builtin_tools()

    def get_tool(self, tool_name: str) -> BuiltinTool | None:  # type: ignore
        """
        returns the tool that the provider can provide
        """
        return next(filter(lambda x: x.entity.identity.name == tool_name, self.get_tools()), None)

    @property
    def need_credentials(self) -> bool:
        """
        returns whether the provider needs credentials

        :return: whether the provider needs credentials
        """
        return (
            self.entity.credentials_schema is not None
            and len(self.entity.credentials_schema) != 0
            or (self.entity.oauth_schema is not None and len(self.entity.oauth_schema.credentials_schema) != 0)
        )

    @property
    def provider_type(self) -> ToolProviderType:
        """
        returns the type of the provider

        :return: type of the provider
        """
        return ToolProviderType.BUILT_IN

    @property
    def tool_labels(self) -> list[str]:
        """
        returns the labels of the provider

        :return: labels of the provider
        """
        label_enums = self._get_tool_labels()
        return [default_tool_label_dict[label].name for label in label_enums]

    def _get_tool_labels(self) -> list[ToolLabelEnum]:
        """
        returns the labels of the provider
        """
        return self.entity.identity.tags or []

    def validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        """
        validate the credentials of the provider

        :param user_id: use id
        :param credentials: the credentials of the tool
        """
        # validate credentials format
        self.validate_credentials_format(credentials)

        # validate credentials
        self._validate_credentials(user_id, credentials)

    @abstractmethod
    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        """
        validate the credentials of the provider

        :param user_id: use id
        :param credentials: the credentials of the tool
        """
        pass

```

### api/core/tools/builtin_tool/_position.yaml
```yaml
- audio
- code
- time
- webscraper

```

### api/core/tools/builtin_tool/providers/_positions.py
```py
import os.path

from core.helper.position_helper import get_tool_position_map, sort_by_position_map
from core.tools.entities.api_entities import ToolProviderApiEntity


class BuiltinToolProviderSort:
    _position: dict[str, int] = {}

    @classmethod
    def sort(cls, providers: list[ToolProviderApiEntity]) -> list[ToolProviderApiEntity]:
        if not cls._position:
            cls._position = get_tool_position_map(os.path.join(os.path.dirname(__file__), ".."))

        def name_func(provider: ToolProviderApiEntity) -> str:
            return provider.name

        sorted_providers = sort_by_position_map(cls._position, providers, name_func)

        return sorted_providers

```

### api/core/tools/builtin_tool/providers/webscraper/webscraper.yaml
```yaml
identity:
  author: Dify
  name: webscraper
  label:
    en_US: WebScraper
    zh_Hans: 网页抓取
    pt_BR: WebScraper
  description:
    en_US: Web Scraper tool kit is used to scrape web
    zh_Hans: 一个用于抓取网页的工具。
    pt_BR: Web Scraper tool kit is used to scrape web
  icon: icon.svg
  tags:
    - productivity
credentials_for_provider: {}

```

### api/core/tools/builtin_tool/providers/webscraper/tools/webscraper.yaml
```yaml
identity:
  name: webscraper
  author: Dify
  label:
    en_US: Web Scraper
    zh_Hans: 网页爬虫
    pt_BR: Web Scraper
description:
  human:
    en_US: A tool for scraping webpages.
    zh_Hans: 一个用于爬取网页的工具。
    pt_BR: A tool for scraping webpages.
  llm: A tool for scraping webpages. Input should be a URL.
parameters:
  - name: url
    type: string
    required: true
    label:
      en_US: URL
      zh_Hans: 网页链接
      pt_BR: URL
    human_description:
      en_US: used for linking to webpages
      zh_Hans: 用于链接到网页
      pt_BR: used for linking to webpages
    llm_description: url for scraping
    form: llm
  - name: user_agent
    type: string
    required: false
    label:
      en_US: User Agent
      zh_Hans: User Agent
      pt_BR: User Agent
    human_description:
      en_US: used for identifying the browser.
      zh_Hans: 用于识别浏览器。
      pt_BR: used for identifying the browser.
    form: form
    default: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.1000.0 Safari/537.36
  - name: generate_summary
    type: boolean
    required: false
    label:
      en_US: Whether to generate summary
      zh_Hans: 是否生成摘要
    human_description:
      en_US: If true, the crawler will only return the page summary content.
      zh_Hans: 如果启用，爬虫将仅返回页面摘要内容。
    form: form
    options:
      - value: "true"
        label:
          en_US: "Yes"
          zh_Hans: 是
      - value: "false"
        label:
          en_US: "No"
          zh_Hans: 否
    default: "false"

```

### api/core/tools/builtin_tool/providers/webscraper/tools/webscraper.py
```py
from collections.abc import Generator
from typing import Any

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.errors import ToolInvokeError
from core.tools.utils.web_reader_tool import get_url


class WebscraperTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke tools
        """
        try:
            url = tool_parameters.get("url", "")
            user_agent = tool_parameters.get("user_agent", "")
            if not url:
                yield self.create_text_message("Please input url")
                return

            # get webpage
            result = get_url(url, user_agent=user_agent)

            if tool_parameters.get("generate_summary"):
                # summarize and return
                yield self.create_text_message(self.summary(user_id=user_id, content=result))
            else:
                # return full webpage
                yield self.create_text_message(result)
        except Exception as e:
            raise ToolInvokeError(str(e))

```

### api/core/tools/builtin_tool/providers/webscraper/webscraper.py
```py
from typing import Any

from core.tools.builtin_tool.provider import BuiltinToolProviderController


class WebscraperProvider(BuiltinToolProviderController):
    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        """
        Validate credentials
        """
        pass

```

### api/core/tools/builtin_tool/providers/code/code.yaml
```yaml
identity:
  author: Dify
  name: code
  label:
    en_US: Code Interpreter
    zh_Hans: 代码解释器
    pt_BR: Interpretador de Código
  description:
    en_US: Run a piece of code and get the result back.
    zh_Hans: 运行一段代码并返回结果。
    pt_BR: Execute um trecho de código e obtenha o resultado de volta.
  icon: icon.svg
  tags:
    - productivity

```

### api/core/tools/builtin_tool/providers/code/tools/simple_code.py
```py
from collections.abc import Generator
from typing import Any

from core.helper.code_executor.code_executor import CodeExecutor, CodeLanguage
from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.errors import ToolInvokeError


class SimpleCode(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke simple code
        """

        language = tool_parameters.get("language", CodeLanguage.PYTHON3)
        code = tool_parameters.get("code", "")

        if language not in {CodeLanguage.PYTHON3, CodeLanguage.JAVASCRIPT}:
            raise ValueError(f"Only python3 and javascript are supported, not {language}")

        try:
            result = CodeExecutor.execute_code(language, "", code)
            yield self.create_text_message(result)
        except Exception as e:
            raise ToolInvokeError(str(e))

```

### api/core/tools/builtin_tool/providers/code/tools/simple_code.yaml
```yaml
identity:
  name: simple_code
  author: Dify
  label:
    en_US: Code Interpreter
    zh_Hans: 代码解释器
    pt_BR: Interpretador de Código
description:
  human:
    en_US: Run code and get the result back. When you're using a lower quality model, please make sure there are some tips help LLM to understand how to write the code.
    zh_Hans: 运行一段代码并返回结果。当您使用较低质量的模型时，请确保有一些提示帮助 LLM 理解如何编写代码。
    pt_BR: Execute um trecho de código e obtenha o resultado de volta. quando você estiver usando um modelo de qualidade inferior, certifique-se de que existam algumas dicas para ajudar o LLM a entender como escrever o código.
  llm: A tool for running code and getting the result back. Only native packages are allowed, network/IO operations are disabled. and you must use print() or console.log() to output the result or result will be empty.
parameters:
  - name: language
    type: string
    required: true
    label:
      en_US: Language
      zh_Hans: 语言
      pt_BR: Idioma
    human_description:
      en_US: The programming language of the code
      zh_Hans: 代码的编程语言
      pt_BR: A linguagem de programação do código
    llm_description: language of the code, only "python3" and "javascript" are supported
    form: llm
    options:
      - value: python3
        label:
          en_US: Python3
          zh_Hans: Python3
          pt_BR: Python3
      - value: javascript
        label:
          en_US: JavaScript
          zh_Hans: JavaScript
          pt_BR: JavaScript
  - name: code
    type: string
    required: true
    label:
      en_US: Code
      zh_Hans: 代码
      pt_BR: Código
    human_description:
      en_US: The code to be executed
      zh_Hans: 要执行的代码
      pt_BR: O código a ser executado
    llm_description: code to be executed, only native packages are allowed, network/IO operations are disabled.
    form: llm

```

### api/core/tools/builtin_tool/providers/code/code.py
```py
from typing import Any

from core.tools.builtin_tool.provider import BuiltinToolProviderController


class CodeToolProvider(BuiltinToolProviderController):
    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        pass

```

### api/core/tools/builtin_tool/providers/__init__.py
```py

```

### api/core/tools/builtin_tool/providers/time/time.py
```py
from typing import Any

from core.tools.builtin_tool.provider import BuiltinToolProviderController


class WikiPediaProvider(BuiltinToolProviderController):
    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        pass

```

### api/core/tools/builtin_tool/providers/time/tools/current_time.py
```py
from collections.abc import Generator
from datetime import UTC, datetime
from typing import Any

from pytz import timezone as pytz_timezone  # type: ignore[import-untyped]

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage


class CurrentTimeTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke tools
        """
        # get timezone
        tz = tool_parameters.get("timezone", "UTC")
        fm = tool_parameters.get("format") or "%Y-%m-%d %H:%M:%S %Z"
        if tz == "UTC":
            yield self.create_text_message(f"{datetime.now(UTC).strftime(fm)}")
            return

        try:
            tz = pytz_timezone(tz)
        except Exception:
            yield self.create_text_message(f"Invalid timezone: {tz}")
            return
        yield self.create_text_message(f"{datetime.now(tz).strftime(fm)}")

```

### api/core/tools/builtin_tool/providers/time/tools/current_time.yaml
```yaml
identity:
  name: current_time
  author: Dify
  label:
    en_US: Current Time
    zh_Hans: 获取当前时间
    pt_BR: Current Time
description:
  human:
    en_US: A tool for getting the current time.
    zh_Hans: 一个用于获取当前时间的工具。
    pt_BR: A tool for getting the current time.
  llm: A tool for getting the current time.
parameters:
  - name: format
    type: string
    required: false
    label:
      en_US: Format
      zh_Hans: 格式
      pt_BR: Format
    human_description:
      en_US: Time format in strftime standard.
      zh_Hans: strftime 标准的时间格式。
      pt_BR: Time format in strftime standard.
    form: form
    default: "%Y-%m-%d %H:%M:%S"
  - name: timezone
    type: select
    required: false
    label:
      en_US: Timezone
      zh_Hans: 时区
      pt_BR: Timezone
    human_description:
      en_US: Timezone
      zh_Hans: 时区
      pt_BR: Timezone
    form: form
    default: UTC
    options:
      - value: UTC
        label:
          en_US: UTC
          zh_Hans: UTC
          pt_BR: UTC
      - value: America/New_York
        label:
          en_US: America/New_York
          zh_Hans: 美洲/纽约
          pt_BR: America/New_York
      - value: America/Los_Angeles
        label:
          en_US: America/Los_Angeles
          zh_Hans: 美洲/洛杉矶
          pt_BR: America/Los_Angeles
      - value: America/Chicago
        label:
          en_US: America/Chicago
          zh_Hans: 美洲/芝加哥
          pt_BR: America/Chicago
      - value: America/Sao_Paulo
        label:
          en_US: America/Sao_Paulo
          zh_Hans: 美洲/圣保罗
          pt_BR: América/São Paulo
      - value: Asia/Shanghai
        label:
          en_US: Asia/Shanghai
          zh_Hans: 亚洲/上海
          pt_BR: Asia/Shanghai
      - value: Asia/Ho_Chi_Minh
        label:
          en_US: Asia/Ho_Chi_Minh
          zh_Hans: 亚洲/胡志明市
          pt_BR: Ásia/Ho Chi Minh
      - value: Asia/Tokyo
        label:
          en_US: Asia/Tokyo
          zh_Hans: 亚洲/东京
          pt_BR: Asia/Tokyo
      - value: Asia/Dubai
        label:
          en_US: Asia/Dubai
          zh_Hans: 亚洲/迪拜
          pt_BR: Asia/Dubai
      - value: Asia/Kolkata
        label:
          en_US: Asia/Kolkata
          zh_Hans: 亚洲/加尔各答
          pt_BR: Asia/Kolkata
      - value: Asia/Seoul
        label:
          en_US: Asia/Seoul
          zh_Hans: 亚洲/首尔
          pt_BR: Asia/Seoul
      - value: Asia/Singapore
        label:
          en_US: Asia/Singapore
          zh_Hans: 亚洲/新加坡
          pt_BR: Asia/Singapore
      - value: Europe/London
        label:
          en_US: Europe/London
          zh_Hans: 欧洲/伦敦
          pt_BR: Europe/London
      - value: Europe/Berlin
        label:
          en_US: Europe/Berlin
          zh_Hans: 欧洲/柏林
          pt_BR: Europe/Berlin
      - value: Europe/Moscow
        label:
          en_US: Europe/Moscow
          zh_Hans: 欧洲/莫斯科
          pt_BR: Europe/Moscow
      - value: Australia/Sydney
        label:
          en_US: Australia/Sydney
          zh_Hans: 澳大利亚/悉尼
          pt_BR: Australia/Sydney
      - value: Pacific/Auckland
        label:
          en_US: Pacific/Auckland
          zh_Hans: 太平洋/奥克兰
          pt_BR: Pacific/Auckland
      - value: Africa/Cairo
        label:
          en_US: Africa/Cairo
          zh_Hans: 非洲/开罗
          pt_BR: Africa/Cairo

```

### api/core/tools/builtin_tool/providers/time/tools/timezone_conversion.yaml
```yaml
identity:
  name: timezone_conversion
  author: zhuhao
  label:
    en_US: convert time to equivalent time zone
    zh_Hans: 时区转换
description:
  human:
    en_US: A tool to convert time to equivalent time zone
    zh_Hans: 时区转换
  llm: A tool to convert time to equivalent time zone
parameters:
  - name: current_time
    type: string
    required: true
    form: llm
    label:
      en_US: current time
      zh_Hans: 当前时间
    human_description:
      en_US: current time, such as 2024-1-1 0:0:0
      zh_Hans: 当前时间，比如 2024-1-1 0:0:0
  - name: current_timezone
    type: string
    required: true
    form: llm
    label:
      en_US: Current Timezone
      zh_Hans: 当前时区
    human_description:
      en_US: Current Timezone, such as Asia/Shanghai
      zh_Hans: 当前时区，比如 Asia/Shanghai
    default: Asia/Shanghai
  - name: target_timezone
    type: string
    required: true
    form: llm
    label:
      en_US: Target Timezone
      zh_Hans: 目标时区
    human_description:
      en_US: Target Timezone, such as Asia/Tokyo
      zh_Hans: 目标时区，比如 Asia/Tokyo
    default: Asia/Tokyo

```

### api/core/tools/builtin_tool/providers/time/tools/timestamp_to_localtime.yaml
```yaml
identity:
  name: timestamp_to_localtime
  author: zhuhao
  label:
    en_US: Timestamp to localtime
    zh_Hans: 时间戳转换
description:
  human:
    en_US: A tool for timestamp convert to localtime
    zh_Hans: 时间戳转换
  llm: A tool for timestamp convert to localtime
parameters:
  - name: timestamp
    type: number
    required: true
    form: llm
    label:
      en_US: Timestamp
      zh_Hans: 时间戳
    human_description:
      en_US: Timestamp
      zh_Hans: 时间戳
  - name: timezone
    type: string
    required: false
    form: llm
    label:
      en_US: Timezone
      zh_Hans: 时区
    human_description:
      en_US: Timezone, such as Asia/Shanghai
      zh_Hans: 时区，比如 Asia/Shanghai
    default: Asia/Shanghai

```

### api/core/tools/builtin_tool/providers/time/tools/timestamp_to_localtime.py
```py
from collections.abc import Generator
from datetime import datetime
from typing import Any

import pytz  # type: ignore[import-untyped]

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.errors import ToolInvokeError


class TimestampToLocaltimeTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Convert timestamp to localtime
        """
        timestamp: int = tool_parameters.get("timestamp", 0)
        timezone = tool_parameters.get("timezone", "Asia/Shanghai")
        if not timezone:
            timezone = None
        time_format = "%Y-%m-%d %H:%M:%S"

        locatime = self.timestamp_to_localtime(timestamp, timezone)
        if not locatime:
            yield self.create_text_message(f"Invalid timestamp: {timestamp}")
            return

        localtime_format = locatime.strftime(time_format)

        yield self.create_text_message(f"{localtime_format}")

    @staticmethod
    def timestamp_to_localtime(timestamp: int, local_tz=None) -> datetime | None:
        try:
            if local_tz is None:
                local_tz = datetime.now().astimezone().tzinfo
            if isinstance(local_tz, str):
                local_tz = pytz.timezone(local_tz)
            local_time = datetime.fromtimestamp(timestamp, local_tz)
            return local_time
        except Exception as e:
            raise ToolInvokeError(str(e))

```

### api/core/tools/builtin_tool/providers/time/tools/weekday.yaml
```yaml
identity:
  name: weekday
  author: Bowen Liang
  label:
    en_US: Weekday Calculator
    zh_Hans: 星期几计算器
description:
  human:
    en_US: A tool for calculating the weekday of a given date.
    zh_Hans: 计算指定日期为星期几的工具。
  llm: A tool for calculating the weekday of a given date by year, month and day.
parameters:
  - name: year
    type: number
    required: true
    form: llm
    label:
      en_US: Year
      zh_Hans: 年
    human_description:
      en_US: Year
      zh_Hans: 年
  - name: month
    type: number
    required: true
    form: llm
    label:
      en_US: Month
      zh_Hans: 月
    human_description:
      en_US: Month
      zh_Hans: 月
  - name: day
    type: number
    required: true
    form: llm
    label:
      en_US: day
      zh_Hans: 日
    human_description:
      en_US: day
      zh_Hans: 日

```

### api/core/tools/builtin_tool/providers/time/tools/timezone_conversion.py
```py
from collections.abc import Generator
from datetime import datetime
from typing import Any

import pytz  # type: ignore[import-untyped]

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.errors import ToolInvokeError


class TimezoneConversionTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Convert time to equivalent time zone
        """
        current_time = tool_parameters.get("current_time")
        current_timezone = tool_parameters.get("current_timezone", "Asia/Shanghai")
        target_timezone = tool_parameters.get("target_timezone", "Asia/Tokyo")
        target_time = self.timezone_convert(current_time, current_timezone, target_timezone)  # type: ignore
        if not target_time:
            yield self.create_text_message(
                f"Invalid datetime and timezone: {current_time},{current_timezone},{target_timezone}"
            )
            return

        yield self.create_text_message(f"{target_time}")

    @staticmethod
    def timezone_convert(current_time: str, source_timezone: str, target_timezone: str) -> str:
        """
        Convert a time string from source timezone to target timezone.
        """
        time_format = "%Y-%m-%d %H:%M:%S"
        try:
            # get source timezone
            input_timezone = pytz.timezone(source_timezone)
            # get target timezone
            output_timezone = pytz.timezone(target_timezone)
            local_time = datetime.strptime(current_time, time_format)
            datetime_with_tz = input_timezone.localize(local_time)
            # timezone convert
            converted_datetime = datetime_with_tz.astimezone(output_timezone)
            return converted_datetime.strftime(time_format)
        except Exception as e:
            raise ToolInvokeError(str(e))

```

### api/core/tools/builtin_tool/providers/time/tools/weekday.py
```py
import calendar
from collections.abc import Generator
from datetime import datetime
from typing import Any

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage


class WeekdayTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Calculate the day of the week for a given date
        """
        year = tool_parameters.get("year")
        month = tool_parameters.get("month")
        if month is None:
            raise ValueError("Month is required")
        day = tool_parameters.get("day")

        date_obj = self.convert_datetime(year, month, day)
        if not date_obj:
            yield self.create_text_message(f"Invalid date: Year {year}, Month {month}, Day {day}.")
            return

        weekday_name = calendar.day_name[date_obj.weekday()]
        month_name = calendar.month_name[month]
        readable_date = f"{month_name} {date_obj.day}, {date_obj.year}"
        yield self.create_text_message(f"{readable_date} is {weekday_name}.")

    @staticmethod
    def convert_datetime(year, month, day) -> datetime | None:
        try:
            # allowed range in datetime module
            if not (year >= 1 and 1 <= month <= 12 and 1 <= day <= 31):
                return None

            year = int(year)
            month = int(month)
            day = int(day)
            return datetime(year, month, day)
        except ValueError:
            return None

```

### api/core/tools/builtin_tool/providers/time/tools/localtime_to_timestamp.yaml
```yaml
identity:
  name: localtime_to_timestamp
  author: zhuhao
  label:
    en_US: localtime to timestamp
    zh_Hans: 获取时间戳
description:
  human:
    en_US: A tool for localtime convert to timestamp
    zh_Hans: 获取时间戳
  llm: A tool for localtime convert to timestamp
parameters:
  - name: localtime
    type: string
    required: true
    form: llm
    label:
      en_US: localtime
      zh_Hans: 本地时间
    human_description:
      en_US: localtime, such as 2024-1-1 0:0:0
      zh_Hans: 本地时间，比如 2024-1-1 0:0:0
  - name: timezone
    type: string
    required: false
    form: llm
    label:
      en_US: Timezone
      zh_Hans: 时区
    human_description:
      en_US: Timezone, such as Asia/Shanghai
      zh_Hans: 时区，比如 Asia/Shanghai
    default: Asia/Shanghai

```

### api/core/tools/builtin_tool/providers/time/tools/localtime_to_timestamp.py
```py
from collections.abc import Generator
from datetime import datetime
from typing import Any

import pytz  # type: ignore[import-untyped]

from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.tool_entities import ToolInvokeMessage
from core.tools.errors import ToolInvokeError


class LocaltimeToTimestampTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Convert localtime to timestamp
        """
        localtime = tool_parameters.get("localtime")
        timezone = tool_parameters.get("timezone", "Asia/Shanghai")
        if not timezone:
            timezone = None
        time_format = "%Y-%m-%d %H:%M:%S"

        timestamp = self.localtime_to_timestamp(localtime, time_format, timezone)  # type: ignore
        if not timestamp:
            yield self.create_text_message(f"Invalid localtime: {localtime}")
            return

        yield self.create_text_message(f"{timestamp}")

    # TODO: this method's type is messy
    @staticmethod
    def localtime_to_timestamp(localtime: str, time_format: str, local_tz=None) -> int | None:
        try:
            local_time = datetime.strptime(localtime, time_format)
            if local_tz is None:
                localtime = local_time.astimezone()  # type: ignore
            elif isinstance(local_tz, str):
                local_tz = pytz.timezone(local_tz)
                localtime = local_tz.localize(local_time)  # type: ignore
            timestamp = int(localtime.timestamp())  # type: ignore
            return timestamp
        except Exception as e:
            raise ToolInvokeError(str(e))

```

### api/core/tools/builtin_tool/providers/time/time.yaml
```yaml
identity:
  author: Dify
  name: time
  label:
    en_US: CurrentTime
    zh_Hans: 时间
    pt_BR: CurrentTime
  description:
    en_US: A tool for getting the current time.
    zh_Hans: 一个用于获取当前时间的工具。
    pt_BR: A tool for getting the current time.
  icon: icon.svg
  tags:
    - utilities

```

### api/core/tools/builtin_tool/providers/audio/tools/tts.yaml
```yaml
identity:
  name: tts
  author: hjlarry
  label:
    en_US: Text To Speech
description:
  human:
    en_US: Convert text to audio file.
    zh_Hans: 将文本转换为音频文件。
  llm: Convert text to audio file.
parameters:
  - name: text
    type: string
    required: true
    label:
      en_US: Text
      zh_Hans: 文本
    human_description:
      en_US: The text to be converted.
      zh_Hans: 要转换的文本。
    llm_description: The text to be converted.
    form: llm

```

### api/core/tools/builtin_tool/providers/audio/tools/tts.py
```py
import io
from collections.abc import Generator
from typing import Any

from graphon.model_runtime.entities.model_entities import ModelPropertyKey, ModelType

from core.model_manager import ModelManager
from core.plugin.entities.parameters import PluginParameterOption
from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolParameter
from services.model_provider_service import ModelProviderService


class TTSTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        if not self.runtime:
            raise ValueError("Runtime is required")
        runtime = self.runtime
        provider, model = tool_parameters.get("model").split("#")  # type: ignore
        voice = tool_parameters.get(f"voice#{provider}#{model}")
        model_manager = ModelManager.for_tenant(tenant_id=runtime.tenant_id, user_id=user_id)
        model_instance = model_manager.get_model_instance(
            tenant_id=runtime.tenant_id or "",
            provider=provider,
            model_type=ModelType.TTS,
            model=model,
        )
        if not voice:
            voices = model_instance.get_tts_voices()
            if voices:
                voice = voices[0].get("value")
                if not voice:
                    raise ValueError("Sorry, no voice available.")
            else:
                raise ValueError("Sorry, no voice available.")
        tts = model_instance.invoke_tts(content_text=tool_parameters.get("text"), voice=voice)  # type: ignore[arg-type]
        buffer = io.BytesIO()
        for chunk in tts:
            buffer.write(chunk)

        wav_bytes = buffer.getvalue()
        yield self.create_text_message("Audio generated successfully")
        yield self.create_blob_message(
            blob=wav_bytes,
            meta={"mime_type": "audio/x-wav"},
        )

    def get_available_models(self) -> list[tuple[str, str, list[Any]]]:
        if not self.runtime:
            raise ValueError("Runtime is required")
        model_provider_service = ModelProviderService()
        tid: str = self.runtime.tenant_id or ""
        models = model_provider_service.get_models_by_model_type(tenant_id=tid, model_type="tts")
        items = []
        for provider_model in models:
            provider = provider_model.provider
            for model in provider_model.models:
                voices = model.model_properties.get(ModelPropertyKey.VOICES, [])
                items.append((provider, model.model, voices))
        return items

    def get_runtime_parameters(
        self,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> list[ToolParameter]:
        parameters = []

        options = []
        for provider, model, voices in self.get_available_models():
            option = PluginParameterOption(value=f"{provider}#{model}", label=I18nObject(en_US=f"{model}({provider})"))
            options.append(option)
            parameters.append(
                ToolParameter(
                    name=f"voice#{provider}#{model}",
                    label=I18nObject(en_US=f"Voice of {model}({provider})"),
                    human_description=I18nObject(en_US=f"Select a voice for {model} model"),
                    placeholder=I18nObject(en_US="Select a voice"),
                    type=ToolParameter.ToolParameterType.SELECT,
                    form=ToolParameter.ToolParameterForm.FORM,
                    options=[
                        PluginParameterOption(value=voice.get("mode"), label=I18nObject(en_US=voice.get("name")))
                        for voice in voices
                    ],
                )
            )

        parameters.insert(
            0,
            ToolParameter(
                name="model",
                label=I18nObject(en_US="Model", zh_Hans="Model"),
                human_description=I18nObject(
                    en_US="All available TTS models. You can config model in the Model Provider of Settings.",
                    zh_Hans="所有可用的 TTS 模型。你可以在设置中的模型供应商里配置。",
                ),
                type=ToolParameter.ToolParameterType.SELECT,
                form=ToolParameter.ToolParameterForm.FORM,
                required=True,
                placeholder=I18nObject(en_US="Select a model", zh_Hans="选择模型"),
                options=options,
            ),
        )
        return parameters

```

### api/core/tools/builtin_tool/providers/audio/tools/asr.py
```py
import io
from collections.abc import Generator
from typing import Any

from graphon.file import FileType
from graphon.file.file_manager import download
from graphon.model_runtime.entities.model_entities import ModelType

from core.model_manager import ModelManager
from core.plugin.entities.parameters import PluginParameterOption
from core.tools.builtin_tool.tool import BuiltinTool
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolParameter
from services.model_provider_service import ModelProviderService


class ASRTool(BuiltinTool):
    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        if not self.runtime:
            raise ValueError("Runtime is required")
        runtime = self.runtime
        file = tool_parameters.get("audio_file")
        if file.type != FileType.AUDIO:  # type: ignore
            yield self.create_text_message("not a valid audio file")
            return
        audio_binary = io.BytesIO(download(file))  # type: ignore
        audio_binary.name = "temp.mp3"
        provider, model = tool_parameters.get("model").split("#")  # type: ignore
        model_manager = ModelManager.for_tenant(tenant_id=runtime.tenant_id, user_id=user_id)
        model_instance = model_manager.get_model_instance(
            tenant_id=runtime.tenant_id,
            provider=provider,
            model_type=ModelType.SPEECH2TEXT,
            model=model,
        )
        text = model_instance.invoke_speech2text(file=audio_binary)
        yield self.create_text_message(text)

    def get_available_models(self) -> list[tuple[str, str]]:
        if not self.runtime:
            raise ValueError("Runtime is required")
        model_provider_service = ModelProviderService()
        models = model_provider_service.get_models_by_model_type(
            tenant_id=self.runtime.tenant_id, model_type="speech2text"
        )
        items = []
        for provider_model in models:
            provider = provider_model.provider
            for model in provider_model.models:
                items.append((provider, model.model))
        return items

    def get_runtime_parameters(
        self,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> list[ToolParameter]:
        parameters = []

        options = []
        for provider, model in self.get_available_models():
            option = PluginParameterOption(value=f"{provider}#{model}", label=I18nObject(en_US=f"{model}({provider})"))
            options.append(option)

        parameters.append(
            ToolParameter(
                name="model",
                label=I18nObject(en_US="Model", zh_Hans="Model"),
                human_description=I18nObject(
                    en_US="All available ASR models. You can config model in the Model Provider of Settings.",
                    zh_Hans="所有可用的 ASR 模型。你可以在设置中的模型供应商里配置。",
                ),
                type=ToolParameter.ToolParameterType.SELECT,
                form=ToolParameter.ToolParameterForm.FORM,
                required=True,
                options=options,
            )
        )
        return parameters

```

### api/core/tools/builtin_tool/providers/audio/tools/asr.yaml
```yaml
identity:
  name: asr
  author: hjlarry
  label:
    en_US: Speech To Text
description:
  human:
    en_US: Convert audio file to text.
    zh_Hans: 将音频文件转换为文本。
  llm: Convert audio file to text.
parameters:
  - name: audio_file
    type: file
    required: true
    label:
      en_US: Audio File
      zh_Hans: 音频文件
    human_description:
      en_US: The audio file to be converted.
      zh_Hans: 要转换的音频文件。
    llm_description: The audio file to be converted.
    form: llm

```

### api/core/tools/builtin_tool/providers/audio/audio.py
```py
from typing import Any

from core.tools.builtin_tool.provider import BuiltinToolProviderController


class AudioToolProvider(BuiltinToolProviderController):
    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        pass

```

### api/core/tools/builtin_tool/providers/audio/audio.yaml
```yaml
identity:
  author: hjlarry
  name: audio
  label:
    en_US: Audio
  description:
    en_US: A tool for tts and asr.
    zh_Hans: 一个用于文本转语音和语音转文本的工具。
  icon: icon.svg
  tags:
    - utilities

```

### api/core/tools/builtin_tool/tool.py
```py
from __future__ import annotations

from graphon.model_runtime.entities.llm_entities import LLMResult
from graphon.model_runtime.entities.message_entities import PromptMessage, SystemPromptMessage, UserPromptMessage

from core.tools.__base.tool import Tool
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.tool_entities import ToolProviderType
from core.tools.utils.model_invocation_utils import ModelInvocationUtils

_SUMMARY_PROMPT = """You are a professional language researcher, you are interested in the language
and you can quickly aimed at the main point of an webpage and reproduce it in your own words but
retain the original meaning and keep the key points.
however, the text you got is too long, what you got is possible a part of the text.
Please summarize the text you got.
"""


class BuiltinTool(Tool):
    """
    Builtin tool

    :param meta: the meta data of a tool call processing
    """

    def __init__(self, provider: str, **kwargs):
        super().__init__(**kwargs)
        self.provider = provider

    def fork_tool_runtime(self, runtime: ToolRuntime) -> BuiltinTool:
        """
        fork a new tool with metadata
        :return: the new tool
        """
        return self.__class__(
            entity=self.entity.model_copy(),
            runtime=runtime,
            provider=self.provider,
        )

    def invoke_model(self, user_id: str, prompt_messages: list[PromptMessage], stop: list[str]) -> LLMResult:
        """
        invoke model

        :param user_id: the user id
        :param prompt_messages: the prompt messages
        :param stop: the stop words
        :return: the model result
        """
        # invoke model
        return ModelInvocationUtils.invoke(
            user_id=user_id,
            tenant_id=self.runtime.tenant_id or "",
            tool_type=ToolProviderType.BUILT_IN,
            tool_name=self.entity.identity.name,
            prompt_messages=prompt_messages,
            caller_user_id=self.runtime.user_id,
        )

    def tool_provider_type(self) -> ToolProviderType:
        return ToolProviderType.BUILT_IN

    def get_max_tokens(self) -> int:
        """
        get max tokens

        :return: the max tokens
        """
        if self.runtime is None:
            raise ValueError("runtime is required")

        return ModelInvocationUtils.get_max_llm_context_tokens(
            tenant_id=self.runtime.tenant_id or "",
            user_id=self.runtime.user_id,
        )

    def get_prompt_tokens(self, prompt_messages: list[PromptMessage]) -> int:
        """
        get prompt tokens

        :param prompt_messages: the prompt messages
        :return: the tokens
        """
        if self.runtime is None:
            raise ValueError("runtime is required")

        return ModelInvocationUtils.calculate_tokens(
            tenant_id=self.runtime.tenant_id or "",
            prompt_messages=prompt_messages,
            user_id=self.runtime.user_id,
        )

    def summary(self, user_id: str, content: str) -> str:
        max_tokens = self.get_max_tokens()

        if self.get_prompt_tokens(prompt_messages=[UserPromptMessage(content=content)]) < max_tokens * 0.6:
            return content

        def get_prompt_tokens(content: str) -> int:
            return self.get_prompt_tokens(
                prompt_messages=[SystemPromptMessage(content=_SUMMARY_PROMPT), UserPromptMessage(content=content)]
            )

        def summarize(content: str) -> str:
            summary = self.invoke_model(
                user_id=user_id,
                prompt_messages=[SystemPromptMessage(content=_SUMMARY_PROMPT), UserPromptMessage(content=content)],
                stop=[],
            )

            assert isinstance(summary.message.content, str)
            return summary.message.content

        lines = content.split("\n")
        new_lines = []
        # split long line into multiple lines
        for i in range(len(lines)):
            line = lines[i]
            if not line.strip():
                continue
            if len(line) < max_tokens * 0.5:
                new_lines.append(line)
            elif get_prompt_tokens(line) > max_tokens * 0.7:
                while get_prompt_tokens(line) > max_tokens * 0.7:
                    new_lines.append(line[: int(max_tokens * 0.5)])
                    line = line[int(max_tokens * 0.5) :]
                new_lines.append(line)
            else:
                new_lines.append(line)

        # merge lines into messages with max tokens
        messages: list[str] = []
        for j in new_lines:
            if len(messages) == 0:
                messages.append(j)
            else:
                if len(messages[-1]) + len(j) < max_tokens * 0.5:
                    messages[-1] += j
                if get_prompt_tokens(messages[-1] + j) > max_tokens * 0.7:
                    messages.append(j)
                else:
                    messages[-1] += j

        summaries = []
        for i in range(len(messages)):
            message = messages[i]
            summary = summarize(message)
            summaries.append(summary)

        result = "\n".join(summaries)

        if self.get_prompt_tokens(prompt_messages=[UserPromptMessage(content=result)]) > max_tokens * 0.7:
            return self.summary(user_id=user_id, content=result)

        return result

```

### api/core/tools/custom_tool/provider.py
```py
from __future__ import annotations

from pydantic import Field
from sqlalchemy import select

from core.entities.provider_entities import ProviderConfig
from core.tools.__base.tool_provider import ToolProviderController
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.custom_tool.tool import ApiTool
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_bundle import ApiToolBundle
from core.tools.entities.tool_entities import (
    ApiProviderAuthType,
    ToolDescription,
    ToolEntity,
    ToolIdentity,
    ToolProviderEntity,
    ToolProviderIdentity,
    ToolProviderType,
)
from extensions.ext_database import db
from models.tools import ApiToolProvider


class ApiToolProviderController(ToolProviderController):
    provider_id: str
    tenant_id: str
    tools: list[ApiTool] = Field(default_factory=list)

    def __init__(self, entity: ToolProviderEntity, provider_id: str, tenant_id: str):
        super().__init__(entity)
        self.provider_id = provider_id
        self.tenant_id = tenant_id
        self.tools = []

    @classmethod
    def from_db(cls, db_provider: ApiToolProvider, auth_type: ApiProviderAuthType) -> ApiToolProviderController:
        credentials_schema = [
            ProviderConfig(
                name="auth_type",
                required=True,
                type=ProviderConfig.Type.SELECT,
                options=[
                    ProviderConfig.Option(value="none", label=I18nObject(en_US="None", zh_Hans="无")),
                    ProviderConfig.Option(value="api_key_header", label=I18nObject(en_US="Header", zh_Hans="请求头")),
                    ProviderConfig.Option(
                        value="api_key_query", label=I18nObject(en_US="Query Param", zh_Hans="查询参数")
                    ),
                ],
                default="none",
                help=I18nObject(en_US="The auth type of the api provider", zh_Hans="api provider 的认证类型"),
            )
        ]
        if auth_type == ApiProviderAuthType.API_KEY_HEADER:
            credentials_schema = [
                *credentials_schema,
                ProviderConfig(
                    name="api_key_header",
                    required=False,
                    default="Authorization",
                    type=ProviderConfig.Type.TEXT_INPUT,
                    help=I18nObject(en_US="The header name of the api key", zh_Hans="携带 api key 的 header 名称"),
                ),
                ProviderConfig(
                    name="api_key_value",
                    required=True,
                    type=ProviderConfig.Type.SECRET_INPUT,
                    help=I18nObject(en_US="The api key", zh_Hans="api key 的值"),
                ),
                ProviderConfig(
                    name="api_key_header_prefix",
                    required=False,
                    default="basic",
                    type=ProviderConfig.Type.SELECT,
                    help=I18nObject(en_US="The prefix of the api key header", zh_Hans="api key header 的前缀"),
                    options=[
                        ProviderConfig.Option(value="basic", label=I18nObject(en_US="Basic", zh_Hans="Basic")),
                        ProviderConfig.Option(value="bearer", label=I18nObject(en_US="Bearer", zh_Hans="Bearer")),
                        ProviderConfig.Option(value="custom", label=I18nObject(en_US="Custom", zh_Hans="Custom")),
                    ],
                ),
            ]
        elif auth_type == ApiProviderAuthType.API_KEY_QUERY:
            credentials_schema = [
                *credentials_schema,
                ProviderConfig(
                    name="api_key_query_param",
                    required=False,
                    default="key",
                    type=ProviderConfig.Type.TEXT_INPUT,
                    help=I18nObject(
                        en_US="The query parameter name of the api key", zh_Hans="携带 api key 的查询参数名称"
                    ),
                ),
                ProviderConfig(
                    name="api_key_value",
                    required=True,
                    type=ProviderConfig.Type.SECRET_INPUT,
                    help=I18nObject(en_US="The api key", zh_Hans="api key 的值"),
                ),
            ]
        elif auth_type == ApiProviderAuthType.NONE:
            pass

        user = db_provider.user
        user_name = user.name if user else ""

        return ApiToolProviderController(
            entity=ToolProviderEntity(
                identity=ToolProviderIdentity(
                    author=user_name,
                    name=db_provider.name,
                    label=I18nObject(en_US=db_provider.name, zh_Hans=db_provider.name),
                    description=I18nObject(en_US=db_provider.description, zh_Hans=db_provider.description),
                    icon=db_provider.icon,
                ),
                credentials_schema=credentials_schema,
                plugin_id=None,
            ),
            provider_id=db_provider.id or "",
            tenant_id=db_provider.tenant_id or "",
        )

    @property
    def provider_type(self) -> ToolProviderType:
        return ToolProviderType.API

    def _parse_tool_bundle(self, tool_bundle: ApiToolBundle) -> ApiTool:
        """
        parse tool bundle to tool

        :param tool_bundle: the tool bundle
        :return: the tool
        """
        return ApiTool(
            api_bundle=tool_bundle,
            provider_id=self.provider_id,
            entity=ToolEntity(
                identity=ToolIdentity(
                    author=tool_bundle.author,
                    name=tool_bundle.operation_id or "default_tool",
                    label=I18nObject(
                        en_US=tool_bundle.operation_id or "default_tool",
                        zh_Hans=tool_bundle.operation_id or "default_tool",
                    ),
                    icon=self.entity.identity.icon,
                    provider=self.provider_id,
                ),
                description=ToolDescription(
                    human=I18nObject(en_US=tool_bundle.summary or "", zh_Hans=tool_bundle.summary or ""),
                    llm=tool_bundle.summary or "",
                ),
                parameters=tool_bundle.parameters or [],
            ),
            runtime=ToolRuntime(tenant_id=self.tenant_id),
        )

    def load_bundled_tools(self, tools: list[ApiToolBundle]):
        """
        load bundled tools

        :param tools: the bundled tools
        :return: the tools
        """
        self.tools = [self._parse_tool_bundle(tool) for tool in tools]

        return self.tools

    def get_tools(self, tenant_id: str) -> list[ApiTool]:
        """
        fetch tools from database

        :param tenant_id: the tenant id
        :return: the tools
        """
        if len(self.tools) > 0:
            return self.tools

        tools: list[ApiTool] = []

        # get tenant api providers
        db_providers = db.session.scalars(
            select(ApiToolProvider).where(
                ApiToolProvider.tenant_id == tenant_id, ApiToolProvider.name == self.entity.identity.name
            )
        ).all()

        if db_providers and len(db_providers) != 0:
            for db_provider in db_providers:
                for tool in db_provider.tools:
                    assistant_tool = self._parse_tool_bundle(tool)
                    tools.append(assistant_tool)

        self.tools = tools
        return tools

    def get_tool(self, tool_name: str) -> ApiTool:
        """
        get tool by name

        :param tool_name: the name of the tool
        :return: the tool
        """
        if self.tools is None:
            self.get_tools(self.tenant_id)

        for tool in self.tools:
            if tool.entity.identity.name == tool_name:
                return tool

        raise ValueError(f"tool {tool_name} not found")

```

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

### api/core/tools/mcp_tool/provider.py
```py
from typing import Any, Self

from core.entities.mcp_provider import MCPProviderEntity
from core.mcp.types import Tool as RemoteMCPTool
from core.tools.__base.tool_provider import ToolProviderController
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import (
    ToolDescription,
    ToolEntity,
    ToolIdentity,
    ToolProviderEntityWithPlugin,
    ToolProviderIdentity,
    ToolProviderType,
)
from core.tools.mcp_tool.tool import MCPTool
from models.tools import MCPToolProvider
from services.tools.tools_transform_service import ToolTransformService


class MCPToolProviderController(ToolProviderController):
    def __init__(
        self,
        entity: ToolProviderEntityWithPlugin,
        provider_id: str,
        tenant_id: str,
        server_url: str,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
        sse_read_timeout: float | None = None,
    ):
        super().__init__(entity)
        self.entity: ToolProviderEntityWithPlugin = entity
        self.tenant_id = tenant_id
        self.provider_id = provider_id
        self.server_url = server_url
        self.headers = headers or {}
        self.timeout = timeout
        self.sse_read_timeout = sse_read_timeout

    @property
    def provider_type(self) -> ToolProviderType:
        """
        returns the type of the provider

        :return: type of the provider
        """
        return ToolProviderType.MCP

    @classmethod
    def from_db(cls, db_provider: MCPToolProvider) -> Self:
        """
        from db provider
        """
        # Convert to entity first
        provider_entity = db_provider.to_entity()
        return cls.from_entity(provider_entity)

    @classmethod
    def from_entity(cls, entity: MCPProviderEntity) -> Self:
        """
        create a MCPToolProviderController from a MCPProviderEntity
        """
        remote_mcp_tools = [RemoteMCPTool(**tool) for tool in entity.tools]

        tools = [
            ToolEntity(
                identity=ToolIdentity(
                    author="Anonymous",  # Tool level author is not stored
                    name=remote_mcp_tool.name,
                    label=I18nObject(en_US=remote_mcp_tool.name, zh_Hans=remote_mcp_tool.name),
                    provider=entity.provider_id,
                    icon=entity.icon if isinstance(entity.icon, str) else "",
                ),
                parameters=ToolTransformService.convert_mcp_schema_to_parameter(remote_mcp_tool.inputSchema),
                description=ToolDescription(
                    human=I18nObject(
                        en_US=remote_mcp_tool.description or "", zh_Hans=remote_mcp_tool.description or ""
                    ),
                    llm=remote_mcp_tool.description or "",
                ),
                output_schema=remote_mcp_tool.outputSchema or {},
                has_runtime_parameters=len(remote_mcp_tool.inputSchema) > 0,
            )
            for remote_mcp_tool in remote_mcp_tools
        ]
        if not entity.icon:
            raise ValueError("Database provider icon is required")
        return cls(
            entity=ToolProviderEntityWithPlugin(
                identity=ToolProviderIdentity(
                    author="Anonymous",  # Provider level author is not stored in entity
                    name=entity.name,
                    label=I18nObject(en_US=entity.name, zh_Hans=entity.name),
                    description=I18nObject(en_US="", zh_Hans=""),
                    icon=entity.icon if isinstance(entity.icon, str) else "",
                ),
                plugin_id=None,
                credentials_schema=[],
                tools=tools,
            ),
            provider_id=entity.provider_id,
            tenant_id=entity.tenant_id,
            server_url=entity.server_url,
            headers=entity.headers,
            timeout=entity.timeout,
            sse_read_timeout=entity.sse_read_timeout,
        )

    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        """
        validate the credentials of the provider
        """
        pass

    def get_tool(self, tool_name: str) -> MCPTool:
        """
        return tool with given name
        """
        tool_entity = next(
            (tool_entity for tool_entity in self.entity.tools if tool_entity.identity.name == tool_name), None
        )

        if not tool_entity:
            raise ValueError(f"Tool with name {tool_name} not found")

        return MCPTool(
            entity=tool_entity,
            runtime=ToolRuntime(tenant_id=self.tenant_id),
            tenant_id=self.tenant_id,
            icon=self.entity.identity.icon,
            server_url=self.server_url,
            provider_id=self.provider_id,
            headers=self.headers,
            timeout=self.timeout,
            sse_read_timeout=self.sse_read_timeout,
        )

    def get_tools(self) -> list[MCPTool]:
        """
        get all tools
        """
        return [
            MCPTool(
                entity=tool_entity,
                runtime=ToolRuntime(tenant_id=self.tenant_id),
                tenant_id=self.tenant_id,
                icon=self.entity.identity.icon,
                server_url=self.server_url,
                provider_id=self.provider_id,
                headers=self.headers,
                timeout=self.timeout,
                sse_read_timeout=self.sse_read_timeout,
            )
            for tool_entity in self.entity.tools
        ]

```

### api/core/tools/mcp_tool/tool.py
```py
from __future__ import annotations

import base64
import json
import logging
from collections.abc import Generator, Mapping
from typing import Any, cast

from graphon.model_runtime.entities.llm_entities import LLMUsage, LLMUsageMetadata

from core.mcp.auth_client import MCPClientWithAuthRetry
from core.mcp.error import MCPConnectionError
from core.mcp.types import (
    AudioContent,
    BlobResourceContents,
    CallToolResult,
    EmbeddedResource,
    ImageContent,
    TextContent,
    TextResourceContents,
)
from core.tools.__base.tool import Tool
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.tool_entities import ToolEntity, ToolInvokeMessage, ToolProviderType
from core.tools.errors import ToolInvokeError

logger = logging.getLogger(__name__)


class MCPTool(Tool):
    def __init__(
        self,
        entity: ToolEntity,
        runtime: ToolRuntime,
        tenant_id: str,
        icon: str,
        server_url: str,
        provider_id: str,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
        sse_read_timeout: float | None = None,
    ):
        super().__init__(entity, runtime)
        self.tenant_id = tenant_id
        self.icon = icon
        self.server_url = server_url
        self.provider_id = provider_id
        self.headers = headers or {}
        self.timeout = timeout
        self.sse_read_timeout = sse_read_timeout
        self._latest_usage = LLMUsage.empty_usage()

    def tool_provider_type(self) -> ToolProviderType:
        return ToolProviderType.MCP

    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        result = self.invoke_remote_mcp_tool(tool_parameters)

        # Extract usage metadata from MCP protocol's _meta field
        self._latest_usage = self._derive_usage_from_result(result)

        # handle dify tool output
        for content in result.content:
            if isinstance(content, TextContent):
                yield from self._process_text_content(content)
            elif isinstance(content, ImageContent | AudioContent):
                yield self.create_blob_message(
                    blob=base64.b64decode(content.data), meta={"mime_type": content.mimeType}
                )
            elif isinstance(content, EmbeddedResource):
                resource = content.resource
                if isinstance(resource, TextResourceContents):
                    yield self.create_text_message(resource.text)
                elif isinstance(resource, BlobResourceContents):
                    mime_type = resource.mimeType or "application/octet-stream"
                    yield self.create_blob_message(blob=base64.b64decode(resource.blob), meta={"mime_type": mime_type})
                else:
                    raise ToolInvokeError(f"Unsupported embedded resource type: {type(resource)}")
            else:
                logger.warning("Unsupported content type=%s", type(content))

        # handle MCP structured output
        if self.entity.output_schema and result.structuredContent:
            for k, v in result.structuredContent.items():
                yield self.create_variable_message(k, v)

    def _process_text_content(self, content: TextContent) -> Generator[ToolInvokeMessage, None, None]:
        """Process text content and yield appropriate messages."""
        # Check if content looks like JSON before attempting to parse
        text = content.text.strip()
        if text and text[0] in ("{", "[") and text[-1] in ("}", "]"):
            try:
                content_json = json.loads(text)
                yield from self._process_json_content(content_json)
                return
            except json.JSONDecodeError:
                pass

        # If not JSON or parsing failed, treat as plain text
        yield self.create_text_message(content.text)

    def _process_json_content(self, content_json: Any) -> Generator[ToolInvokeMessage, None, None]:
        """Process JSON content based on its type."""
        if isinstance(content_json, dict):
            yield self.create_json_message(content_json)
        elif isinstance(content_json, list):
            yield from self._process_json_list(content_json)
        else:
            # For primitive types (str, int, bool, etc.), convert to string
            yield self.create_text_message(str(content_json))

    def _process_json_list(self, json_list: list) -> Generator[ToolInvokeMessage, None, None]:
        """Process a list of JSON items."""
        if any(not isinstance(item, dict) for item in json_list):
            # If the list contains any non-dict item, treat the entire list as a text message.
            yield self.create_text_message(str(json_list))
            return

        # Otherwise, process each dictionary as a separate JSON message.
        for item in json_list:
            yield self.create_json_message(item)

    @property
    def latest_usage(self) -> LLMUsage:
        return self._latest_usage

    @classmethod
    def _derive_usage_from_result(cls, result: CallToolResult) -> LLMUsage:
        """
        Extract usage metadata from MCP tool result's _meta field.

        The MCP protocol's _meta field (aliased as 'meta' in Python) can contain
        usage information such as token counts, costs, and other metadata.

        Args:
            result: The CallToolResult from MCP tool invocation

        Returns:
            LLMUsage instance with values from meta or empty_usage if not found
        """
        # Extract usage from the meta field if present
        if result.meta:
            usage_dict = cls._extract_usage_dict(result.meta)
            if usage_dict is not None:
                return LLMUsage.from_metadata(cast(LLMUsageMetadata, cast(object, dict(usage_dict))))

        return LLMUsage.empty_usage()

    @classmethod
    def _extract_usage_dict(cls, payload: Mapping[str, Any]) -> Mapping[str, Any] | None:
        """
        Recursively search for usage dictionary in the payload.

        The MCP protocol's _meta field can contain usage data in various formats:
        - Direct usage field: {"usage": {...}}
        - Nested in metadata: {"metadata": {"usage": {...}}}
        - Or nested within other fields

        Args:
            payload: The payload to search for usage data

        Returns:
            The usage dictionary if found, None otherwise
        """
        # Check for direct usage field
        usage_candidate = payload.get("usage")
        if isinstance(usage_candidate, Mapping):
            return usage_candidate

        # Check for metadata nested usage
        metadata_candidate = payload.get("metadata")
        if isinstance(metadata_candidate, Mapping):
            usage_candidate = metadata_candidate.get("usage")
            if isinstance(usage_candidate, Mapping):
                return usage_candidate

        # Check for common token counting fields directly in payload
        # Some MCP servers may include token counts directly
        if "total_tokens" in payload or "prompt_tokens" in payload or "completion_tokens" in payload:
            usage_dict: dict[str, Any] = {}
            for key in (
                "prompt_tokens",
                "completion_tokens",
                "total_tokens",
                "prompt_unit_price",
                "completion_unit_price",
                "total_price",
                "currency",
                "prompt_price_unit",
                "completion_price_unit",
                "prompt_price",
                "completion_price",
                "latency",
                "time_to_first_token",
                "time_to_generate",
            ):
                if key in payload:
                    usage_dict[key] = payload[key]
            if usage_dict:
                return usage_dict

        # Recursively search through nested structures
        for value in payload.values():
            if isinstance(value, Mapping):
                found = cls._extract_usage_dict(value)
                if found is not None:
                    return found
            elif isinstance(value, list) and not isinstance(value, (str, bytes, bytearray)):
                for item in value:
                    if isinstance(item, Mapping):
                        found = cls._extract_usage_dict(item)
                        if found is not None:
                            return found
        return None

    def fork_tool_runtime(self, runtime: ToolRuntime) -> MCPTool:
        return MCPTool(
            entity=self.entity,
            runtime=runtime,
            tenant_id=self.tenant_id,
            icon=self.icon,
            server_url=self.server_url,
            provider_id=self.provider_id,
            headers=self.headers,
            timeout=self.timeout,
            sse_read_timeout=self.sse_read_timeout,
        )

    def _handle_none_parameter(self, parameter: dict[str, Any]) -> dict[str, Any]:
        """
        in mcp tool invoke, if the parameter is empty, it will be set to None
        """
        return {
            key: value
            for key, value in parameter.items()
            if value is not None and not (isinstance(value, str) and value.strip() == "")
        }

    def invoke_remote_mcp_tool(self, tool_parameters: dict[str, Any]) -> CallToolResult:
        headers = self.headers.copy() if self.headers else {}
        tool_parameters = self._handle_none_parameter(tool_parameters)

        from sqlalchemy.orm import Session

        from extensions.ext_database import db
        from services.tools.mcp_tools_manage_service import MCPToolManageService

        # Step 1: Load provider entity and credentials in a short-lived session
        # This minimizes database connection hold time
        with Session(db.engine, expire_on_commit=False) as session:
            mcp_service = MCPToolManageService(session=session)
            provider_entity = mcp_service.get_provider_entity(self.provider_id, self.tenant_id, by_server_id=True)

            # Decrypt and prepare all credentials before closing session
            server_url = provider_entity.decrypt_server_url()
            headers = provider_entity.decrypt_headers()

            # Try to get existing token and add to headers
            if not headers:
                tokens = provider_entity.retrieve_tokens()
                if tokens and tokens.access_token:
                    headers["Authorization"] = f"{tokens.token_type.capitalize()} {tokens.access_token}"

        # Step 2: Session is now closed, perform network operations without holding database connection
        # MCPClientWithAuthRetry will create a new session lazily only if auth retry is needed
        try:
            with MCPClientWithAuthRetry(
                server_url=server_url,
                headers=headers,
                timeout=self.timeout,
                sse_read_timeout=self.sse_read_timeout,
                provider_entity=provider_entity,
            ) as mcp_client:
                return mcp_client.invoke_tool(tool_name=self.entity.identity.name, tool_args=tool_parameters)
        except MCPConnectionError as e:
            raise ToolInvokeError(f"Failed to connect to MCP server: {e}") from e
        except Exception as e:
            raise ToolInvokeError(f"Failed to invoke tool: {e}") from e

```

### api/core/tools/plugin_tool/provider.py
```py
from typing import Any

from core.plugin.impl.tool import PluginToolManager
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.builtin_tool.provider import BuiltinToolProviderController
from core.tools.entities.tool_entities import ToolProviderEntityWithPlugin, ToolProviderType
from core.tools.errors import ToolProviderCredentialValidationError
from core.tools.plugin_tool.tool import PluginTool


class PluginToolProviderController(BuiltinToolProviderController):
    entity: ToolProviderEntityWithPlugin
    tenant_id: str
    plugin_id: str
    plugin_unique_identifier: str

    def __init__(
        self, entity: ToolProviderEntityWithPlugin, plugin_id: str, plugin_unique_identifier: str, tenant_id: str
    ):
        self.entity = entity
        self.tenant_id = tenant_id
        self.plugin_id = plugin_id
        self.plugin_unique_identifier = plugin_unique_identifier

    @property
    def provider_type(self) -> ToolProviderType:
        """
        returns the type of the provider

        :return: type of the provider
        """
        return ToolProviderType.PLUGIN

    def _validate_credentials(self, user_id: str, credentials: dict[str, Any]):
        """
        validate the credentials of the provider
        """
        manager = PluginToolManager()
        if not manager.validate_provider_credentials(
            tenant_id=self.tenant_id,
            user_id=user_id,
            provider=self.entity.identity.name,
            credentials=credentials,
        ):
            raise ToolProviderCredentialValidationError("Invalid credentials")

    def get_tool(self, tool_name: str) -> PluginTool:  # type: ignore
        """
        return tool with given name
        """
        tool_entity = next(
            (tool_entity for tool_entity in self.entity.tools if tool_entity.identity.name == tool_name), None
        )

        if not tool_entity:
            raise ValueError(f"Tool with name {tool_name} not found")

        return PluginTool(
            entity=tool_entity,
            runtime=ToolRuntime(tenant_id=self.tenant_id),
            tenant_id=self.tenant_id,
            icon=self.entity.identity.icon,
            plugin_unique_identifier=self.plugin_unique_identifier,
        )

    def get_tools(self) -> list[PluginTool]:  # type: ignore
        """
        get all tools
        """
        return [
            PluginTool(
                entity=tool_entity,
                runtime=ToolRuntime(tenant_id=self.tenant_id),
                tenant_id=self.tenant_id,
                icon=self.entity.identity.icon,
                plugin_unique_identifier=self.plugin_unique_identifier,
            )
            for tool_entity in self.entity.tools
        ]

```

### api/core/tools/plugin_tool/tool.py
```py
from __future__ import annotations

from collections.abc import Generator
from typing import Any

from core.plugin.impl.tool import PluginToolManager
from core.plugin.utils.converter import convert_parameters_to_plugin_format
from core.tools.__base.tool import Tool
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.tool_entities import ToolEntity, ToolInvokeMessage, ToolParameter, ToolProviderType


class PluginTool(Tool):
    def __init__(
        self, entity: ToolEntity, runtime: ToolRuntime, tenant_id: str, icon: str, plugin_unique_identifier: str
    ):
        super().__init__(entity, runtime)
        self.tenant_id = tenant_id
        self.icon = icon
        self.plugin_unique_identifier = plugin_unique_identifier
        self.runtime_parameters: list[ToolParameter] | None = None

    def tool_provider_type(self) -> ToolProviderType:
        return ToolProviderType.PLUGIN

    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        manager = PluginToolManager()

        tool_parameters = convert_parameters_to_plugin_format(tool_parameters)

        yield from manager.invoke(
            tenant_id=self.tenant_id,
            user_id=user_id,
            tool_provider=self.entity.identity.provider,
            tool_name=self.entity.identity.name,
            credentials=self.runtime.credentials,
            credential_type=self.runtime.credential_type,
            tool_parameters=tool_parameters,
            conversation_id=conversation_id,
            app_id=app_id,
            message_id=message_id,
        )

    def fork_tool_runtime(self, runtime: ToolRuntime) -> PluginTool:
        return PluginTool(
            entity=self.entity,
            runtime=runtime,
            tenant_id=self.tenant_id,
            icon=self.icon,
            plugin_unique_identifier=self.plugin_unique_identifier,
        )

    def get_runtime_parameters(
        self,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> list[ToolParameter]:
        """
        get the runtime parameters
        """
        if not self.entity.has_runtime_parameters:
            return self.entity.parameters

        if self.runtime_parameters is not None:
            return self.runtime_parameters

        manager = PluginToolManager()
        self.runtime_parameters = manager.get_runtime_parameters(
            tenant_id=self.tenant_id,
            user_id="",
            provider=self.entity.identity.provider,
            tool=self.entity.identity.name,
            credentials=self.runtime.credentials,
            conversation_id=conversation_id,
            app_id=app_id,
            message_id=message_id,
        )

        return self.runtime_parameters

```

### api/core/tools/workflow_as_tool/provider.py
```py
from __future__ import annotations

from collections.abc import Mapping

from graphon.variables.input_entities import VariableEntity, VariableEntityType
from pydantic import Field
from sqlalchemy.orm import Session

from core.app.apps.workflow.app_config_manager import WorkflowAppConfigManager
from core.db.session_factory import session_factory
from core.plugin.entities.parameters import PluginParameterOption
from core.tools.__base.tool_provider import ToolProviderController
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import (
    ToolDescription,
    ToolEntity,
    ToolIdentity,
    ToolParameter,
    ToolProviderEntity,
    ToolProviderIdentity,
    ToolProviderType,
)
from core.tools.utils.workflow_configuration_sync import WorkflowToolConfigurationUtils
from core.tools.workflow_as_tool.tool import WorkflowTool
from extensions.ext_database import db
from models.account import Account
from models.model import App, AppMode
from models.tools import WorkflowToolProvider
from models.workflow import Workflow

VARIABLE_TO_PARAMETER_TYPE_MAPPING = {
    VariableEntityType.TEXT_INPUT: ToolParameter.ToolParameterType.STRING,
    VariableEntityType.PARAGRAPH: ToolParameter.ToolParameterType.STRING,
    VariableEntityType.SELECT: ToolParameter.ToolParameterType.SELECT,
    VariableEntityType.NUMBER: ToolParameter.ToolParameterType.NUMBER,
    VariableEntityType.CHECKBOX: ToolParameter.ToolParameterType.BOOLEAN,
    VariableEntityType.FILE: ToolParameter.ToolParameterType.FILE,
    VariableEntityType.FILE_LIST: ToolParameter.ToolParameterType.FILES,
    VariableEntityType.JSON_OBJECT: ToolParameter.ToolParameterType.OBJECT,
}


class WorkflowToolProviderController(ToolProviderController):
    provider_id: str
    tools: list[WorkflowTool] = Field(default_factory=list)

    def __init__(self, entity: ToolProviderEntity, provider_id: str):
        super().__init__(entity=entity)
        self.provider_id = provider_id

    @classmethod
    def from_db(cls, db_provider: WorkflowToolProvider) -> WorkflowToolProviderController:
        with session_factory.create_session() as session, session.begin():
            app = session.get(App, db_provider.app_id)
            if not app:
                raise ValueError("app not found")

            user = session.get(Account, db_provider.user_id) if db_provider.user_id else None
            controller = WorkflowToolProviderController(
                entity=ToolProviderEntity(
                    identity=ToolProviderIdentity(
                        author=user.name if user else "",
                        name=db_provider.label,
                        label=I18nObject(en_US=db_provider.label, zh_Hans=db_provider.label),
                        description=I18nObject(en_US=db_provider.description, zh_Hans=db_provider.description),
                        icon=db_provider.icon,
                    ),
                    credentials_schema=[],
                    plugin_id=None,
                ),
                provider_id=db_provider.id,
            )

            controller.tools = [
                controller._get_db_provider_tool(db_provider, app, session=session, user=user),
            ]

        return controller

    @property
    def provider_type(self) -> ToolProviderType:
        return ToolProviderType.WORKFLOW

    def _get_db_provider_tool(
        self,
        db_provider: WorkflowToolProvider,
        app: App,
        *,
        session: Session,
        user: Account | None = None,
    ) -> WorkflowTool:
        """
        get db provider tool
        :param db_provider: the db provider
        :param app: the app
        :return: the tool
        """
        workflow: Workflow | None = (
            session.query(Workflow)
            .where(Workflow.app_id == db_provider.app_id, Workflow.version == db_provider.version)
            .first()
        )

        if not workflow:
            raise ValueError("workflow not found")

        # fetch start node
        graph: Mapping = workflow.graph_dict
        features_dict: Mapping = workflow.features_dict
        features = WorkflowAppConfigManager.convert_features(config_dict=features_dict, app_mode=AppMode.WORKFLOW)

        parameters = db_provider.parameter_configurations
        variables = WorkflowToolConfigurationUtils.get_workflow_graph_variables(graph)

        def fetch_workflow_variable(variable_name: str) -> VariableEntity | None:
            return next(filter(lambda x: x.variable == variable_name, variables), None)

        workflow_tool_parameters = []
        for parameter in parameters:
            variable = fetch_workflow_variable(parameter.name)
            if variable:
                parameter_type = None
                options = []
                if variable.type not in VARIABLE_TO_PARAMETER_TYPE_MAPPING:
                    raise ValueError(f"unsupported variable type {variable.type}")
                parameter_type = VARIABLE_TO_PARAMETER_TYPE_MAPPING[variable.type]

                if variable.type == VariableEntityType.SELECT and variable.options:
                    options = [
                        PluginParameterOption(value=option, label=I18nObject(en_US=option, zh_Hans=option))
                        for option in variable.options
                    ]

                workflow_tool_parameters.append(
                    ToolParameter(
                        name=parameter.name,
                        label=I18nObject(en_US=variable.label, zh_Hans=variable.label),
                        human_description=I18nObject(en_US=parameter.description, zh_Hans=parameter.description),
                        type=parameter_type,
                        form=parameter.form,
                        llm_description=parameter.description,
                        required=variable.required,
                        default=variable.default,
                        options=options,
                        placeholder=I18nObject(en_US="", zh_Hans=""),
                    )
                )
            elif features.file_upload:
                workflow_tool_parameters.append(
                    ToolParameter(
                        name=parameter.name,
                        label=I18nObject(en_US=parameter.name, zh_Hans=parameter.name),
                        human_description=I18nObject(en_US=parameter.description, zh_Hans=parameter.description),
                        type=ToolParameter.ToolParameterType.SYSTEM_FILES,
                        llm_description=parameter.description,
                        required=False,
                        form=parameter.form,
                        placeholder=I18nObject(en_US="", zh_Hans=""),
                    )
                )
            else:
                raise ValueError("variable not found")

        # get output schema from workflow
        outputs = WorkflowToolConfigurationUtils.get_workflow_graph_output(graph)

        reserved_keys = {"json", "text", "files"}

        properties = {}
        for output in outputs:
            if output.variable not in reserved_keys:
                properties[output.variable] = {
                    "type": output.value_type,
                    "description": "",
                }
        output_schema = {"type": "object", "properties": properties}

        return WorkflowTool(
            workflow_as_tool_id=db_provider.id,
            entity=ToolEntity(
                identity=ToolIdentity(
                    author=user.name if user else "",
                    name=db_provider.name,
                    label=I18nObject(en_US=db_provider.label, zh_Hans=db_provider.label),
                    provider=self.provider_id,
                    icon=db_provider.icon,
                ),
                description=ToolDescription(
                    human=I18nObject(en_US=db_provider.description, zh_Hans=db_provider.description),
                    llm=db_provider.description,
                ),
                parameters=workflow_tool_parameters,
                output_schema=output_schema,
            ),
            runtime=ToolRuntime(
                tenant_id=db_provider.tenant_id,
            ),
            workflow_app_id=app.id,
            workflow_entities={
                "app": app,
                "workflow": workflow,
            },
            version=db_provider.version,
            workflow_call_depth=0,
            label=db_provider.label,
        )

    def get_tools(self, tenant_id: str) -> list[WorkflowTool]:
        """
        fetch tools from database

        :param tenant_id: the tenant id
        :return: the tools
        """
        if self.tools is not None:
            return self.tools

        with Session(db.engine, expire_on_commit=False) as session, session.begin():
            db_provider: WorkflowToolProvider | None = (
                session.query(WorkflowToolProvider)
                .where(
                    WorkflowToolProvider.tenant_id == tenant_id,
                    WorkflowToolProvider.id == self.provider_id,
                )
                .first()
            )

            if not db_provider:
                return []

            app = session.get(App, db_provider.app_id)
            if not app:
                raise ValueError("app not found")

            user = session.get(Account, db_provider.user_id) if db_provider.user_id else None
            self.tools = [self._get_db_provider_tool(db_provider, app, session=session, user=user)]

        return self.tools

    def get_tool(self, tool_name: str) -> WorkflowTool | None:  # type: ignore
        """
        get tool by name

        :param tool_name: the name of the tool
        :return: the tool
        """
        if self.tools is None:
            return None

        for tool in self.tools:
            if tool.entity.identity.name == tool_name:
                return tool

        return None

```

### api/core/tools/workflow_as_tool/tool.py
```py
from __future__ import annotations

import json
import logging
from collections.abc import Generator, Mapping, Sequence
from typing import Any, cast

from graphon.file import FILE_MODEL_IDENTITY, File, FileTransferMethod
from graphon.model_runtime.entities.llm_entities import LLMUsage, LLMUsageMetadata
from sqlalchemy import select

from core.app.file_access import DatabaseFileAccessController
from core.db.session_factory import session_factory
from core.tools.__base.tool import Tool
from core.tools.__base.tool_runtime import ToolRuntime
from core.tools.entities.tool_entities import (
    ToolEntity,
    ToolInvokeMessage,
    ToolParameter,
    ToolProviderType,
)
from core.tools.errors import ToolInvokeError
from core.workflow.file_reference import resolve_file_record_id
from factories.file_factory import build_from_mapping
from models import Account, Tenant
from models.model import App, EndUser
from models.utils.file_input_compat import build_file_from_stored_mapping
from models.workflow import Workflow

logger = logging.getLogger(__name__)
_file_access_controller = DatabaseFileAccessController()


class WorkflowTool(Tool):
    """
    Workflow tool.
    """

    def __init__(
        self,
        workflow_app_id: str,
        workflow_as_tool_id: str,
        version: str,
        workflow_entities: dict[str, Any],
        workflow_call_depth: int,
        entity: ToolEntity,
        runtime: ToolRuntime,
        label: str = "Workflow",
    ):
        self.workflow_app_id = workflow_app_id
        self.workflow_as_tool_id = workflow_as_tool_id
        self.version = version
        self.workflow_entities = workflow_entities
        self.workflow_call_depth = workflow_call_depth
        self.label = label
        self._latest_usage = LLMUsage.empty_usage()

        super().__init__(entity=entity, runtime=runtime)

    def tool_provider_type(self) -> ToolProviderType:
        """
        get the tool provider type

        :return: the tool provider type
        """
        return ToolProviderType.WORKFLOW

    def _invoke(
        self,
        user_id: str,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke the tool
        """
        app = self._get_app(app_id=self.workflow_app_id)
        workflow = self._get_workflow(app_id=self.workflow_app_id, version=self.version)

        # transform the tool parameters
        tool_parameters, files = self._transform_args(tool_parameters=tool_parameters)

        from core.app.apps.workflow.app_generator import WorkflowAppGenerator

        generator = WorkflowAppGenerator()
        assert self.runtime is not None
        assert self.runtime.invoke_from is not None

        user = self._resolve_user(user_id=user_id)
        if user is None:
            raise ToolInvokeError("User not found")

        self._latest_usage = LLMUsage.empty_usage()

        result = generator.generate(
            app_model=app,
            workflow=workflow,
            user=user,
            args={"inputs": tool_parameters, "files": files},
            invoke_from=self.runtime.invoke_from,
            streaming=False,
            call_depth=self.workflow_call_depth + 1,
            # NOTE(QuantumGhost): We explicitly set `pause_state_config` to `None`
            # because workflow pausing mechanisms (such as HumanInput) are not
            # supported within WorkflowTool execution context.
            pause_state_config=None,
        )
        assert isinstance(result, dict)
        data = result.get("data", {})

        if err := data.get("error"):
            raise ToolInvokeError(err)

        outputs = data.get("outputs")
        if outputs is None:
            outputs = {}
        else:
            outputs, files = self._extract_files(outputs)  # type: ignore
            for file in files:
                yield self.create_file_message(file)  # type: ignore

        # traverse `outputs` field and create variable messages
        for key, value in outputs.items():
            if key not in {"text", "json", "files"}:
                yield self.create_variable_message(variable_name=key, variable_value=value)

        self._latest_usage = self._derive_usage_from_result(data)

        yield self.create_text_message(json.dumps(outputs, ensure_ascii=False))
        yield self.create_json_message(outputs, suppress_output=True)

    @property
    def latest_usage(self) -> LLMUsage:
        return self._latest_usage

    @classmethod
    def _derive_usage_from_result(cls, data: Mapping[str, Any]) -> LLMUsage:
        usage_dict = cls._extract_usage_dict(data)
        if usage_dict is not None:
            return LLMUsage.from_metadata(cast(LLMUsageMetadata, dict(usage_dict)))

        total_tokens = data.get("total_tokens")
        total_price = data.get("total_price")
        if total_tokens is None and total_price is None:
            return LLMUsage.empty_usage()

        usage_metadata: dict[str, Any] = {}
        if total_tokens is not None:
            try:
                usage_metadata["total_tokens"] = int(str(total_tokens))
            except (TypeError, ValueError):
                pass
        if total_price is not None:
            usage_metadata["total_price"] = str(total_price)
        currency = data.get("currency")
        if currency is not None:
            usage_metadata["currency"] = currency

        if not usage_metadata:
            return LLMUsage.empty_usage()

        return LLMUsage.from_metadata(cast(LLMUsageMetadata, usage_metadata))

    @classmethod
    def _extract_usage_dict(cls, payload: Mapping[str, Any]) -> Mapping[str, Any] | None:
        usage_candidate = payload.get("usage")
        if isinstance(usage_candidate, Mapping):
            return usage_candidate

        metadata_candidate = payload.get("metadata")
        if isinstance(metadata_candidate, Mapping):
            usage_candidate = metadata_candidate.get("usage")
            if isinstance(usage_candidate, Mapping):
                return usage_candidate

        for value in payload.values():
            if isinstance(value, Mapping):
                found = cls._extract_usage_dict(value)
                if found is not None:
                    return found
            elif isinstance(value, Sequence) and not isinstance(value, (str, bytes, bytearray)):
                for item in value:
                    if isinstance(item, Mapping):
                        found = cls._extract_usage_dict(item)
                        if found is not None:
                            return found
        return None

    def fork_tool_runtime(self, runtime: ToolRuntime) -> WorkflowTool:
        """
        fork a new tool with metadata

        :return: the new tool
        """
        return self.__class__(
            entity=self.entity.model_copy(),
            runtime=runtime,
            workflow_app_id=self.workflow_app_id,
            workflow_as_tool_id=self.workflow_as_tool_id,
            workflow_entities=self.workflow_entities,
            workflow_call_depth=self.workflow_call_depth,
            version=self.version,
            label=self.label,
        )

    def _resolve_user(self, user_id: str) -> Account | EndUser | None:
        """
        Resolve user object in both HTTP and worker contexts.

        In HTTP context: dereference the current_user LocalProxy (can return Account or EndUser).
        In worker context: load Account(knowledge pipeline) or EndUser(trigger) from database by user_id.

        Returns:
            Account | EndUser | None: The resolved user object, or None if resolution fails.
        """
        return self._resolve_user_from_database(user_id=user_id)

    def _resolve_user_from_database(self, user_id: str) -> Account | EndUser | None:
        """
        Resolve user from database (worker/Celery context).
        """
        with session_factory.create_session() as session:
            tenant_stmt = select(Tenant).where(Tenant.id == self.runtime.tenant_id)
            tenant = session.scalar(tenant_stmt)
            if not tenant:
                return None

            user_stmt = select(Account).where(Account.id == user_id)
            user = session.scalar(user_stmt)
            if user:
                user.current_tenant = tenant
                session.expunge(user)
                return user

            end_user_stmt = select(EndUser).where(EndUser.id == user_id, EndUser.tenant_id == tenant.id)
            end_user = session.scalar(end_user_stmt)
            if end_user:
                session.expunge(end_user)
                return end_user

            return None

    def _get_workflow(self, app_id: str, version: str) -> Workflow:
        """
        get the workflow by app id and version
        """
        with session_factory.create_session() as session, session.begin():
            if not version:
                stmt = (
                    select(Workflow)
                    .where(Workflow.app_id == app_id, Workflow.version != Workflow.VERSION_DRAFT)
                    .order_by(Workflow.created_at.desc())
                )
                workflow = session.scalars(stmt).first()
            else:
                stmt = select(Workflow).where(Workflow.app_id == app_id, Workflow.version == version)
                workflow = session.scalar(stmt)

            if not workflow:
                raise ValueError("workflow not found or not published")

            session.expunge(workflow)
            return workflow

    def _get_app(self, app_id: str) -> App:
        """
        get the app by app id
        """
        stmt = select(App).where(App.id == app_id)
        with session_factory.create_session() as session, session.begin():
            app = session.scalar(stmt)
            if not app:
                raise ValueError("app not found")

            session.expunge(app)
            return app

    def _transform_args(self, tool_parameters: dict) -> tuple[dict, list[dict]]:
        """
        transform the tool parameters

        :param tool_parameters: the tool parameters
        :return: tool_parameters, files
        """
        parameter_rules = self.get_merged_runtime_parameters()
        parameters_result = {}
        files = []
        for parameter in parameter_rules:
            if parameter.type == ToolParameter.ToolParameterType.SYSTEM_FILES:
                file = tool_parameters.get(parameter.name)
                if file:
                    try:
                        file_var_list = [
                            build_file_from_stored_mapping(
                                file_mapping=cast(Mapping[str, Any], f),
                                tenant_id=str(self.runtime.tenant_id),
                            )
                            for f in file
                            if isinstance(f, Mapping)
                        ]
                        for file in file_var_list:
                            file_dict: dict[str, str | None] = {
                                "transfer_method": file.transfer_method.value,
                                "type": file.type.value,
                            }
                            if file.transfer_method == FileTransferMethod.TOOL_FILE:
                                file_dict["tool_file_id"] = resolve_file_record_id(file.reference)
                            elif file.transfer_method == FileTransferMethod.LOCAL_FILE:
                                file_dict["upload_file_id"] = resolve_file_record_id(file.reference)
                            elif file.transfer_method == FileTransferMethod.DATASOURCE_FILE:
                                file_dict["datasource_file_id"] = resolve_file_record_id(file.reference)
                            elif file.transfer_method == FileTransferMethod.REMOTE_URL:
                                file_dict["url"] = file.generate_url()

                            files.append(file_dict)
                    except Exception:
                        logger.exception("Failed to transform file %s", file)
            else:
                parameters_result[parameter.name] = tool_parameters.get(parameter.name)

        return parameters_result, files

    def _extract_files(self, outputs: dict) -> tuple[dict, list[File]]:
        """
        extract files from the result

        :return: the result, files
        """
        files: list[File] = []
        result = {}
        for key, value in outputs.items():
            if isinstance(value, list):
                for item in value:
                    if isinstance(item, dict) and item.get("dify_model_identity") == FILE_MODEL_IDENTITY:
                        item = self._update_file_mapping(item)
                        file = build_from_mapping(
                            mapping=item,
                            tenant_id=str(self.runtime.tenant_id),
                            access_controller=_file_access_controller,
                        )
                        files.append(file)
            elif isinstance(value, dict) and value.get("dify_model_identity") == FILE_MODEL_IDENTITY:
                value = self._update_file_mapping(value)
                file = build_from_mapping(
                    mapping=value,
                    tenant_id=str(self.runtime.tenant_id),
                    access_controller=_file_access_controller,
                )
                files.append(file)

            result[key] = value

        return result, files

    def _update_file_mapping(self, file_dict: dict):
        file_id = resolve_file_record_id(file_dict.get("reference") or file_dict.get("related_id"))
        transfer_method = FileTransferMethod.value_of(file_dict.get("transfer_method"))
        if transfer_method == FileTransferMethod.TOOL_FILE:
            file_dict["tool_file_id"] = file_id
        elif transfer_method == FileTransferMethod.LOCAL_FILE:
            file_dict["upload_file_id"] = file_id
        return file_dict

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-018.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
