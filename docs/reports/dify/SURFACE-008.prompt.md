You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-008
- **Kind**: endpoint
- **Identifier**: Plugin System (Install, Execute, Backwards Invocation)
- **Description**: Plugin installation and execution framework with backwards invocation allowing plugins to call back into the platform. Risk of malicious plugin code execution, privilege escalation via backwards invocation, and supply chain attacks.
- **Locations**: api/core/plugin/, api/core/plugin/backwards_invocation/, api/core/plugin/endpoint/, api/services/plugin/, api/controllers/console/app/

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

### api/core/plugin/impl/model_runtime_factory.py
```py
from __future__ import annotations

from typing import TYPE_CHECKING

from graphon.model_runtime.model_providers.model_provider_factory import ModelProviderFactory

from core.plugin.impl.model import PluginModelClient

if TYPE_CHECKING:
    from core.model_manager import ModelManager
    from core.plugin.impl.model_runtime import PluginModelRuntime
    from core.provider_manager import ProviderManager


class PluginModelAssembly:
    """Compose request-scoped model views on top of a single plugin runtime."""

    tenant_id: str
    user_id: str | None
    _model_runtime: PluginModelRuntime | None
    _model_provider_factory: ModelProviderFactory | None
    _provider_manager: ProviderManager | None
    _model_manager: ModelManager | None

    def __init__(self, *, tenant_id: str, user_id: str | None = None) -> None:
        self.tenant_id = tenant_id
        self.user_id = user_id
        self._model_runtime = None
        self._model_provider_factory = None
        self._provider_manager = None
        self._model_manager = None

    @property
    def model_runtime(self) -> PluginModelRuntime:
        if self._model_runtime is None:
            self._model_runtime = create_plugin_model_runtime(tenant_id=self.tenant_id, user_id=self.user_id)
        return self._model_runtime

    @property
    def model_provider_factory(self) -> ModelProviderFactory:
        if self._model_provider_factory is None:
            self._model_provider_factory = ModelProviderFactory(model_runtime=self.model_runtime)
        return self._model_provider_factory

    @property
    def provider_manager(self) -> ProviderManager:
        if self._provider_manager is None:
            from core.provider_manager import ProviderManager

            self._provider_manager = ProviderManager(model_runtime=self.model_runtime)
        return self._provider_manager

    @property
    def model_manager(self) -> ModelManager:
        if self._model_manager is None:
            from core.model_manager import ModelManager

            self._model_manager = ModelManager(provider_manager=self.provider_manager)
        return self._model_manager


def create_plugin_model_assembly(*, tenant_id: str, user_id: str | None = None) -> PluginModelAssembly:
    """Create a request-scoped assembly that shares one plugin runtime across model views."""
    return PluginModelAssembly(tenant_id=tenant_id, user_id=user_id)


def create_plugin_model_runtime(*, tenant_id: str, user_id: str | None = None) -> PluginModelRuntime:
    """Create a plugin runtime with its client dependency fully composed."""
    from core.plugin.impl.model_runtime import PluginModelRuntime

    return PluginModelRuntime(
        tenant_id=tenant_id,
        user_id=user_id,
        client=PluginModelClient(),
    )


def create_plugin_model_provider_factory(*, tenant_id: str, user_id: str | None = None) -> ModelProviderFactory:
    """Create a tenant-bound model provider factory for service flows."""
    return create_plugin_model_assembly(tenant_id=tenant_id, user_id=user_id).model_provider_factory


def create_plugin_provider_manager(*, tenant_id: str, user_id: str | None = None) -> ProviderManager:
    """Create a tenant-bound provider manager for service flows."""
    return create_plugin_model_assembly(tenant_id=tenant_id, user_id=user_id).provider_manager


def create_plugin_model_manager(*, tenant_id: str, user_id: str | None = None) -> ModelManager:
    """Create a tenant-bound model manager for service flows."""
    return create_plugin_model_assembly(tenant_id=tenant_id, user_id=user_id).model_manager

```

### api/core/plugin/impl/asset.py
```py
from core.plugin.impl.base import BasePluginClient


class PluginAssetManager(BasePluginClient):
    def fetch_asset(self, tenant_id: str, id: str) -> bytes:
        """
        Fetch an asset by id.
        """
        response = self._request(method="GET", path=f"plugin/{tenant_id}/asset/{id}")
        if response.status_code != 200:
            raise ValueError(f"can not found asset {id}")
        return response.content

    def extract_asset(self, tenant_id: str, plugin_unique_identifier: str, filename: str) -> bytes:
        response = self._request(
            method="GET",
            path=f"plugin/{tenant_id}/extract-asset/",
            params={"plugin_unique_identifier": plugin_unique_identifier, "file_path": filename},
        )
        if response.status_code != 200:
            raise ValueError(f"can not found asset {plugin_unique_identifier}, {str(response.status_code)}")
        return response.content

```

### api/core/plugin/impl/dynamic_select.py
```py
from collections.abc import Mapping
from typing import Any

from core.plugin.entities.plugin_daemon import PluginDynamicSelectOptionsResponse
from core.plugin.impl.base import BasePluginClient
from models.provider_ids import GenericProviderID


class DynamicSelectClient(BasePluginClient):
    def fetch_dynamic_select_options(
        self,
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        action: str,
        credentials: Mapping[str, Any],
        credential_type: str,
        parameter: str,
    ) -> PluginDynamicSelectOptionsResponse:
        """
        Fetch dynamic select options for a plugin parameter.
        """
        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/dynamic_select/fetch_parameter_options",
            PluginDynamicSelectOptionsResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": GenericProviderID(provider).provider_name,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "provider_action": action,
                    "parameter": parameter,
                },
            },
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for options in response:
            return options

        raise ValueError(f"Plugin service returned no options for parameter '{parameter}' in provider '{provider}'")

```

### api/core/plugin/impl/debugging.py
```py
from pydantic import BaseModel

from core.plugin.impl.base import BasePluginClient


class PluginDebuggingClient(BasePluginClient):
    def get_debugging_key(self, tenant_id: str) -> str:
        """
        Get the debugging key for the given tenant.
        """

        class Response(BaseModel):
            key: str

        response = self._request_with_plugin_daemon_response("POST", f"plugin/{tenant_id}/debugging/key", Response)

        return response.key

```

### api/core/plugin/impl/model.py
```py
import binascii
from collections.abc import Generator, Sequence
from typing import IO, Any

from graphon.model_runtime.entities.llm_entities import LLMResultChunk
from graphon.model_runtime.entities.message_entities import PromptMessage, PromptMessageTool
from graphon.model_runtime.entities.model_entities import AIModelEntity
from graphon.model_runtime.entities.rerank_entities import MultimodalRerankInput, RerankResult
from graphon.model_runtime.entities.text_embedding_entities import EmbeddingResult
from graphon.model_runtime.utils.encoders import jsonable_encoder

from core.plugin.entities.plugin_daemon import (
    PluginBasicBooleanResponse,
    PluginDaemonInnerError,
    PluginLLMNumTokensResponse,
    PluginModelProviderEntity,
    PluginModelSchemaEntity,
    PluginStringResultResponse,
    PluginTextEmbeddingNumTokensResponse,
    PluginVoicesResponse,
)
from core.plugin.impl.base import BasePluginClient


class PluginModelClient(BasePluginClient):
    @staticmethod
    def _dispatch_payload(*, user_id: str | None, data: dict[str, Any]) -> dict[str, Any]:
        payload: dict[str, Any] = {"data": data}
        if user_id is not None:
            payload["user_id"] = user_id
        return payload

    def fetch_model_providers(self, tenant_id: str) -> Sequence[PluginModelProviderEntity]:
        """
        Fetch model providers for the given tenant.
        """
        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/models",
            list[PluginModelProviderEntity],
            params={"page": 1, "page_size": 256},
        )
        return response

    def get_model_schema(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model_type: str,
        model: str,
        credentials: dict,
    ) -> AIModelEntity | None:
        """
        Get model schema
        """
        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/model/schema",
            PluginModelSchemaEntity,
            data=self._dispatch_payload(
                user_id=user_id,
                data={
                    "provider": provider,
                    "model_type": model_type,
                    "model": model,
                    "credentials": credentials,
                },
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.model_schema

        return None

    def validate_provider_credentials(
        self, tenant_id: str, user_id: str | None, plugin_id: str, provider: str, credentials: dict
    ) -> bool:
        """
        validate the credentials of the provider
        """
        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/model/validate_provider_credentials",
            PluginBasicBooleanResponse,
            data=self._dispatch_payload(
                user_id=user_id,
                data={
                    "provider": provider,
                    "credentials": credentials,
                },
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            if resp.credentials and isinstance(resp.credentials, dict):
                credentials.update(resp.credentials)

            return resp.result

        return False

    def validate_model_credentials(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model_type: str,
        model: str,
        credentials: dict,
    ) -> bool:
        """
        validate the credentials of the provider
        """
        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/model/validate_model_credentials",
            PluginBasicBooleanResponse,
            data=self._dispatch_payload(
                user_id=user_id,
                data={
                    "provider": provider,
                    "model_type": model_type,
                    "model": model,
                    "credentials": credentials,
                },
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            if resp.credentials and isinstance(resp.credentials, dict):
                credentials.update(resp.credentials)

            return resp.result

        return False

    def invoke_llm(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        prompt_messages: list[PromptMessage],
        model_parameters: dict | None = None,
        tools: list[PromptMessageTool] | None = None,
        stop: list[str] | None = None,
        stream: bool = True,
    ) -> Generator[LLMResultChunk, None, None]:
        """
        Invoke llm
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/llm/invoke",
            type_=LLMResultChunk,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "llm",
                        "model": model,
                        "credentials": credentials,
                        "prompt_messages": prompt_messages,
                        "model_parameters": model_parameters,
                        "tools": tools,
                        "stop": stop,
                        "stream": stream,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        try:
            yield from response
        except PluginDaemonInnerError as e:
            raise ValueError(e.message + str(e.code))

    def get_llm_num_tokens(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model_type: str,
        model: str,
        credentials: dict,
        prompt_messages: list[PromptMessage],
        tools: list[PromptMessageTool] | None = None,
    ) -> int:
        """
        Get number of tokens for llm
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/llm/num_tokens",
            type_=PluginLLMNumTokensResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": model_type,
                        "model": model,
                        "credentials": credentials,
                        "prompt_messages": prompt_messages,
                        "tools": tools,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.num_tokens

        return 0

    def invoke_text_embedding(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        texts: list[str],
        input_type: str,
    ) -> EmbeddingResult:
        """
        Invoke text embedding
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/text_embedding/invoke",
            type_=EmbeddingResult,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "text-embedding",
                        "model": model,
                        "credentials": credentials,
                        "texts": texts,
                        "input_type": input_type,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("Failed to invoke text embedding")

    def invoke_multimodal_embedding(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        documents: list[dict],
        input_type: str,
    ) -> EmbeddingResult:
        """
        Invoke file embedding
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/multimodal_embedding/invoke",
            type_=EmbeddingResult,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "text-embedding",
                        "model": model,
                        "credentials": credentials,
                        "documents": documents,
                        "input_type": input_type,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("Failed to invoke file embedding")

    def get_text_embedding_num_tokens(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        texts: list[str],
    ) -> list[int]:
        """
        Get number of tokens for text embedding
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/text_embedding/num_tokens",
            type_=PluginTextEmbeddingNumTokensResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "text-embedding",
                        "model": model,
                        "credentials": credentials,
                        "texts": texts,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.num_tokens

        return []

    def invoke_rerank(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        query: str,
        docs: list[str],
        score_threshold: float | None = None,
        top_n: int | None = None,
    ) -> RerankResult:
        """
        Invoke rerank
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/rerank/invoke",
            type_=RerankResult,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "rerank",
                        "model": model,
                        "credentials": credentials,
                        "query": query,
                        "docs": docs,
                        "score_threshold": score_threshold,
                        "top_n": top_n,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("Failed to invoke rerank")

    def invoke_multimodal_rerank(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        query: MultimodalRerankInput,
        docs: list[MultimodalRerankInput],
        score_threshold: float | None = None,
        top_n: int | None = None,
    ) -> RerankResult:
        """
        Invoke multimodal rerank
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/multimodal_rerank/invoke",
            type_=RerankResult,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "rerank",
                        "model": model,
                        "credentials": credentials,
                        "query": query,
                        "docs": docs,
                        "score_threshold": score_threshold,
                        "top_n": top_n,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )
        for resp in response:
            return resp

        raise ValueError("Failed to invoke multimodal rerank")

    def invoke_tts(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        content_text: str,
        voice: str,
    ) -> Generator[bytes, None, None]:
        """
        Invoke tts
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/tts/invoke",
            type_=PluginStringResultResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "tts",
                        "model": model,
                        "credentials": credentials,
                        "tenant_id": tenant_id,
                        "content_text": content_text,
                        "voice": voice,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        try:
            for result in response:
                hex_str = result.result
                yield binascii.unhexlify(hex_str)
        except PluginDaemonInnerError as e:
            raise ValueError(e.message + str(e.code))

    def get_tts_model_voices(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        language: str | None = None,
    ):
        """
        Get tts model voices
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/tts/model/voices",
            type_=PluginVoicesResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "tts",
                        "model": model,
                        "credentials": credentials,
                        "language": language,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            voices = []
            for voice in resp.voices:
                voices.append({"name": voice.name, "value": voice.value})

            return voices

        return []

    def invoke_speech_to_text(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        file: IO[bytes],
    ) -> str:
        """
        Invoke speech to text
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/speech2text/invoke",
            type_=PluginStringResultResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "speech2text",
                        "model": model,
                        "credentials": credentials,
                        "file": binascii.hexlify(file.read()).decode(),
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        raise ValueError("Failed to invoke speech to text")

    def invoke_moderation(
        self,
        tenant_id: str,
        user_id: str | None,
        plugin_id: str,
        provider: str,
        model: str,
        credentials: dict,
        text: str,
    ) -> bool:
        """
        Invoke moderation
        """
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/moderation/invoke",
            type_=PluginBasicBooleanResponse,
            data=jsonable_encoder(
                self._dispatch_payload(
                    user_id=user_id,
                    data={
                        "provider": provider,
                        "model_type": "moderation",
                        "model": model,
                        "credentials": credentials,
                        "text": text,
                    },
                )
            ),
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        raise ValueError("Failed to invoke moderation")

```

### api/core/plugin/impl/datasource.py
```py
from collections.abc import Generator, Mapping
from typing import Any

from core.datasource.entities.datasource_entities import (
    DatasourceMessage,
    GetOnlineDocumentPageContentRequest,
    OnlineDocumentPagesMessage,
    OnlineDriveBrowseFilesRequest,
    OnlineDriveBrowseFilesResponse,
    OnlineDriveDownloadFileRequest,
    WebsiteCrawlMessage,
)
from core.plugin.entities.plugin_daemon import (
    PluginBasicBooleanResponse,
    PluginDatasourceProviderEntity,
)
from core.plugin.impl.base import BasePluginClient
from core.schemas.resolver import resolve_dify_schema_refs
from models.provider_ids import DatasourceProviderID, GenericProviderID
from services.tools.tools_transform_service import ToolTransformService


class PluginDatasourceManager(BasePluginClient):
    def fetch_datasource_providers(self, tenant_id: str) -> list[PluginDatasourceProviderEntity]:
        """
        Fetch datasource providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]) -> dict:
            if json_response.get("data"):
                for provider in json_response.get("data", []):
                    declaration = provider.get("declaration", {}) or {}
                    provider_name = declaration.get("identity", {}).get("name")
                    for datasource in declaration.get("datasources", []):
                        datasource["identity"]["provider"] = provider_name
                        # resolve refs
                        if datasource.get("output_schema"):
                            datasource["output_schema"] = resolve_dify_schema_refs(datasource["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/datasources",
            list[PluginDatasourceProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )
        local_file_datasource_provider = PluginDatasourceProviderEntity.model_validate(
            self._get_local_file_datasource_provider()
        )

        for provider in response:
            ToolTransformService.repack_provider(tenant_id=tenant_id, provider=provider)
        all_response = [local_file_datasource_provider] + response

        for provider in all_response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each tool to plugin_id/provider_name
            for tool in provider.declaration.datasources:
                tool.identity.provider = provider.declaration.identity.name

        return all_response

    def fetch_installed_datasource_providers(self, tenant_id: str) -> list[PluginDatasourceProviderEntity]:
        """
        Fetch datasource providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]) -> dict:
            if json_response.get("data"):
                for provider in json_response.get("data", []):
                    declaration = provider.get("declaration", {}) or {}
                    provider_name = declaration.get("identity", {}).get("name")
                    for datasource in declaration.get("datasources", []):
                        datasource["identity"]["provider"] = provider_name
                        # resolve refs
                        if datasource.get("output_schema"):
                            datasource["output_schema"] = resolve_dify_schema_refs(datasource["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/datasources",
            list[PluginDatasourceProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )

        for provider in response:
            ToolTransformService.repack_provider(tenant_id=tenant_id, provider=provider)

        for provider in response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each tool to plugin_id/provider_name
            for tool in provider.declaration.datasources:
                tool.identity.provider = provider.declaration.identity.name

        return response

    def fetch_datasource_provider(self, tenant_id: str, provider_id: str) -> PluginDatasourceProviderEntity:
        """
        Fetch datasource provider for the given tenant and plugin.
        """
        if provider_id == "langgenius/file/file":
            return PluginDatasourceProviderEntity.model_validate(self._get_local_file_datasource_provider())

        tool_provider_id = DatasourceProviderID(provider_id)

        def transformer(json_response: dict[str, Any]) -> dict:
            data = json_response.get("data")
            if data:
                for datasource in data.get("declaration", {}).get("datasources", []):
                    datasource["identity"]["provider"] = tool_provider_id.provider_name
                    if datasource.get("output_schema"):
                        datasource["output_schema"] = resolve_dify_schema_refs(datasource["output_schema"])
            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/datasource",
            PluginDatasourceProviderEntity,
            params={"provider": tool_provider_id.provider_name, "plugin_id": tool_provider_id.plugin_id},
            transformer=transformer,
        )

        response.declaration.identity.name = f"{response.plugin_id}/{response.declaration.identity.name}"

        # override the provider name for each tool to plugin_id/provider_name
        for datasource in response.declaration.datasources:
            datasource.identity.provider = response.declaration.identity.name

        return response

    def get_website_crawl(
        self,
        tenant_id: str,
        user_id: str,
        datasource_provider: str,
        datasource_name: str,
        credentials: dict[str, Any],
        datasource_parameters: Mapping[str, Any],
        provider_type: str,
    ) -> Generator[WebsiteCrawlMessage, None, None]:
        """
        Invoke the datasource with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        datasource_provider_id = GenericProviderID(datasource_provider)

        return self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/get_website_crawl",
            WebsiteCrawlMessage,
            data={
                "user_id": user_id,
                "data": {
                    "provider": datasource_provider_id.provider_name,
                    "datasource": datasource_name,
                    "credentials": credentials,
                    "datasource_parameters": datasource_parameters,
                },
            },
            headers={
                "X-Plugin-ID": datasource_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

    def get_online_document_pages(
        self,
        tenant_id: str,
        user_id: str,
        datasource_provider: str,
        datasource_name: str,
        credentials: dict[str, Any],
        datasource_parameters: Mapping[str, Any],
        provider_type: str,
    ) -> Generator[OnlineDocumentPagesMessage, None, None]:
        """
        Invoke the datasource with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        datasource_provider_id = GenericProviderID(datasource_provider)

        return self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/get_online_document_pages",
            OnlineDocumentPagesMessage,
            data={
                "user_id": user_id,
                "data": {
                    "provider": datasource_provider_id.provider_name,
                    "datasource": datasource_name,
                    "credentials": credentials,
                    "datasource_parameters": datasource_parameters,
                },
            },
            headers={
                "X-Plugin-ID": datasource_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

    def get_online_document_page_content(
        self,
        tenant_id: str,
        user_id: str,
        datasource_provider: str,
        datasource_name: str,
        credentials: dict[str, Any],
        datasource_parameters: GetOnlineDocumentPageContentRequest,
        provider_type: str,
    ) -> Generator[DatasourceMessage, None, None]:
        """
        Invoke the datasource with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        datasource_provider_id = GenericProviderID(datasource_provider)

        return self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/get_online_document_page_content",
            DatasourceMessage,
            data={
                "user_id": user_id,
                "data": {
                    "provider": datasource_provider_id.provider_name,
                    "datasource": datasource_name,
                    "credentials": credentials,
                    "page": datasource_parameters.model_dump(),
                },
            },
            headers={
                "X-Plugin-ID": datasource_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

    def online_drive_browse_files(
        self,
        tenant_id: str,
        user_id: str,
        datasource_provider: str,
        datasource_name: str,
        credentials: dict[str, Any],
        request: OnlineDriveBrowseFilesRequest,
        provider_type: str,
    ) -> Generator[OnlineDriveBrowseFilesResponse, None, None]:
        """
        Invoke the datasource with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        datasource_provider_id = GenericProviderID(datasource_provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/online_drive_browse_files",
            OnlineDriveBrowseFilesResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": datasource_provider_id.provider_name,
                    "datasource": datasource_name,
                    "credentials": credentials,
                    "request": request.model_dump(),
                },
            },
            headers={
                "X-Plugin-ID": datasource_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )
        yield from response

    def online_drive_download_file(
        self,
        tenant_id: str,
        user_id: str,
        datasource_provider: str,
        datasource_name: str,
        credentials: dict[str, Any],
        request: OnlineDriveDownloadFileRequest,
        provider_type: str,
    ) -> Generator[DatasourceMessage, None, None]:
        """
        Invoke the datasource with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        datasource_provider_id = GenericProviderID(datasource_provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/online_drive_download_file",
            DatasourceMessage,
            data={
                "user_id": user_id,
                "data": {
                    "provider": datasource_provider_id.provider_name,
                    "datasource": datasource_name,
                    "credentials": credentials,
                    "request": request.model_dump(),
                },
            },
            headers={
                "X-Plugin-ID": datasource_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )
        yield from response

    def validate_provider_credentials(
        self, tenant_id: str, user_id: str, provider: str, plugin_id: str, credentials: dict[str, Any]
    ) -> bool:
        """
        validate the credentials of the provider
        """
        # datasource_provider_id = GenericProviderID(provider_id)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/validate_credentials",
            PluginBasicBooleanResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": provider,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        return False

    def _get_local_file_datasource_provider(self) -> dict[str, Any]:
        return {
            "id": "langgenius/file/file",
            "plugin_id": "langgenius/file",
            "provider": "file",
            "plugin_unique_identifier": "langgenius/file:0.0.1@dify",
            "declaration": {
                "identity": {
                    "author": "langgenius",
                    "name": "file",
                    "label": {"zh_Hans": "File", "en_US": "File", "pt_BR": "File", "ja_JP": "File"},
                    "icon": "https://assets.dify.ai/images/File%20Upload.svg",
                    "description": {"zh_Hans": "File", "en_US": "File", "pt_BR": "File", "ja_JP": "File"},
                },
                "credentials_schema": [],
                "provider_type": "local_file",
                "datasources": [
                    {
                        "identity": {
                            "author": "langgenius",
                            "name": "upload-file",
                            "provider": "file",
                            "label": {"zh_Hans": "File", "en_US": "File", "pt_BR": "File", "ja_JP": "File"},
                        },
                        "parameters": [],
                        "description": {"zh_Hans": "File", "en_US": "File", "pt_BR": "File", "ja_JP": "File"},
                    }
                ],
            },
        }

```

### api/core/plugin/impl/endpoint.py
```py
from core.plugin.entities.endpoint import EndpointEntityWithInstance
from core.plugin.impl.base import BasePluginClient
from core.plugin.impl.exc import PluginDaemonInternalServerError


class PluginEndpointClient(BasePluginClient):
    def create_endpoint(
        self, tenant_id: str, user_id: str, plugin_unique_identifier: str, name: str, settings: dict
    ) -> bool:
        """
        Create an endpoint for the given plugin.

        Errors will be raised if any error occurs.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/setup",
            bool,
            headers={
                "Content-Type": "application/json",
            },
            data={
                "user_id": user_id,
                "plugin_unique_identifier": plugin_unique_identifier,
                "settings": settings,
                "name": name,
            },
        )

    def list_endpoints(self, tenant_id: str, user_id: str, page: int, page_size: int):
        """
        List all endpoints for the given tenant and user.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/endpoint/list",
            list[EndpointEntityWithInstance],
            params={"page": page, "page_size": page_size},
        )

    def list_endpoints_for_single_plugin(self, tenant_id: str, user_id: str, plugin_id: str, page: int, page_size: int):
        """
        List all endpoints for the given tenant, user and plugin.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/endpoint/list/plugin",
            list[EndpointEntityWithInstance],
            params={"plugin_id": plugin_id, "page": page, "page_size": page_size},
        )

    def update_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str, name: str, settings: dict):
        """
        Update the settings of the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/update",
            bool,
            data={
                "user_id": user_id,
                "endpoint_id": endpoint_id,
                "name": name,
                "settings": settings,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

    def delete_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Delete the given endpoint.

        This operation is idempotent: if the endpoint is already deleted (record not found),
        it will return True instead of raising an error.
        """
        try:
            return self._request_with_plugin_daemon_response(
                "POST",
                f"plugin/{tenant_id}/endpoint/remove",
                bool,
                data={
                    "endpoint_id": endpoint_id,
                },
                headers={
                    "Content-Type": "application/json",
                },
            )
        except PluginDaemonInternalServerError as e:
            # Make delete idempotent: if record is not found, consider it a success
            if "record not found" in str(e.description).lower():
                return True
            raise

    def enable_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Enable the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/enable",
            bool,
            data={
                "endpoint_id": endpoint_id,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

    def disable_endpoint(self, tenant_id: str, user_id: str, endpoint_id: str):
        """
        Disable the given endpoint.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/endpoint/disable",
            bool,
            data={
                "endpoint_id": endpoint_id,
            },
            headers={
                "Content-Type": "application/json",
            },
        )

```

### api/core/plugin/impl/exc.py
```py
from collections.abc import Mapping

from pydantic import TypeAdapter

from extensions.ext_logging import get_request_id


class PluginDaemonError(Exception):
    """Base class for all plugin daemon errors."""

    def __init__(self, description: str):
        self.description = description

    def __str__(self) -> str:
        # returns the class name and description
        return f"req_id: {get_request_id()} {self.__class__.__name__}: {self.description}"


class PluginDaemonInternalError(PluginDaemonError):
    pass


class PluginDaemonClientSideError(PluginDaemonError):
    pass


class PluginDaemonInternalServerError(PluginDaemonInternalError):
    description: str = "Internal Server Error"


class PluginDaemonUnauthorizedError(PluginDaemonInternalError):
    description: str = "Unauthorized"


class PluginDaemonNotFoundError(PluginDaemonInternalError):
    description: str = "Not Found"


class PluginDaemonBadRequestError(PluginDaemonClientSideError):
    description: str = "Bad Request"


class PluginInvokeError(PluginDaemonClientSideError, ValueError):
    description: str = "Invoke Error"

    def _get_error_object(self) -> Mapping:
        try:
            return TypeAdapter(Mapping).validate_json(self.description)
        except Exception:
            return {}

    def get_error_type(self) -> str:
        return self._get_error_object().get("error_type", "unknown")

    def get_error_message(self) -> str:
        try:
            return self._get_error_object().get("message", "unknown")
        except Exception:
            return self.description

    def to_user_friendly_error(self, plugin_name: str = "currently running plugin") -> str:
        """
        Convert the error to a user-friendly error message.

        :param plugin_name: The name of the plugin that caused the error.
        :return: A user-friendly error message.
        """
        return (
            f"An error occurred in the {plugin_name}, "
            f"please contact the author of {plugin_name} for help, "
            f"error type: {self.get_error_type()}, "
            f"error details: {self.get_error_message()}"
        )


class PluginUniqueIdentifierError(PluginDaemonClientSideError):
    description: str = "Unique Identifier Error"


class PluginNotFoundError(PluginDaemonClientSideError):
    description: str = "Plugin Not Found"


class PluginPermissionDeniedError(PluginDaemonClientSideError):
    description: str = "Permission Denied"

```

### api/core/plugin/impl/agent.py
```py
from collections.abc import Generator
from typing import Any

from core.agent.entities import AgentInvokeMessage
from core.plugin.entities.plugin_daemon import (
    PluginAgentProviderEntity,
)
from core.plugin.entities.request import PluginInvokeContext
from core.plugin.impl.base import BasePluginClient
from core.plugin.utils.chunk_merger import merge_blob_chunks
from models.provider_ids import GenericProviderID


class PluginAgentClient(BasePluginClient):
    def fetch_agent_strategy_providers(self, tenant_id: str) -> list[PluginAgentProviderEntity]:
        """
        Fetch agent providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]):
            for provider in json_response.get("data", []):
                declaration = provider.get("declaration", {}) or {}
                provider_name = declaration.get("identity", {}).get("name")
                for strategy in declaration.get("strategies", []):
                    strategy["identity"]["provider"] = provider_name

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/agent_strategies",
            list[PluginAgentProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )

        for provider in response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each tool to plugin_id/provider_name
            for strategy in provider.declaration.strategies:
                strategy.identity.provider = provider.declaration.identity.name

        return response

    def fetch_agent_strategy_provider(self, tenant_id: str, provider: str) -> PluginAgentProviderEntity:
        """
        Fetch tool provider for the given tenant and plugin.
        """
        agent_provider_id = GenericProviderID(provider)

        def transformer(json_response: dict[str, Any]):
            # skip if error occurs
            if json_response.get("data") is None or json_response.get("data", {}).get("declaration") is None:
                return json_response

            for strategy in json_response.get("data", {}).get("declaration", {}).get("strategies", []):
                strategy["identity"]["provider"] = agent_provider_id.provider_name

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/agent_strategy",
            PluginAgentProviderEntity,
            params={"provider": agent_provider_id.provider_name, "plugin_id": agent_provider_id.plugin_id},
            transformer=transformer,
        )

        response.declaration.identity.name = f"{response.plugin_id}/{response.declaration.identity.name}"

        # override the provider name for each tool to plugin_id/provider_name
        for strategy in response.declaration.strategies:
            strategy.identity.provider = response.declaration.identity.name

        return response

    def invoke(
        self,
        tenant_id: str,
        user_id: str,
        agent_provider: str,
        agent_strategy: str,
        agent_params: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
        context: PluginInvokeContext | None = None,
    ) -> Generator[AgentInvokeMessage, None, None]:
        """
        Invoke the agent with the given tenant, user, plugin, provider, name and parameters.
        """

        agent_provider_id = GenericProviderID(agent_provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/agent_strategy/invoke",
            AgentInvokeMessage,
            data={
                "user_id": user_id,
                "conversation_id": conversation_id,
                "app_id": app_id,
                "message_id": message_id,
                "context": context.model_dump() if context else {},
                "data": {
                    "agent_strategy_provider": agent_provider_id.provider_name,
                    "agent_strategy": agent_strategy,
                    "agent_strategy_params": agent_params,
                },
            },
            headers={
                "X-Plugin-ID": agent_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )
        return merge_blob_chunks(response)

```

### api/core/plugin/impl/plugin.py
```py
from collections.abc import Sequence

from requests import HTTPError

from core.plugin.entities.bundle import PluginBundleDependency
from core.plugin.entities.plugin import (
    MissingPluginDependency,
    PluginDeclaration,
    PluginEntity,
    PluginInstallation,
    PluginInstallationSource,
)
from core.plugin.entities.plugin_daemon import (
    PluginDecodeResponse,
    PluginInstallTask,
    PluginInstallTaskStartResponse,
    PluginListResponse,
    PluginReadmeResponse,
)
from core.plugin.impl.base import BasePluginClient
from models.provider_ids import GenericProviderID


class PluginInstaller(BasePluginClient):
    def fetch_plugin_readme(self, tenant_id: str, plugin_unique_identifier: str, language: str) -> str:
        """
        Fetch plugin readme
        """
        try:
            response = self._request_with_plugin_daemon_response(
                "GET",
                f"plugin/{tenant_id}/management/fetch/readme",
                PluginReadmeResponse,
                params={
                    "tenant_id": tenant_id,
                    "plugin_unique_identifier": plugin_unique_identifier,
                    "language": language,
                },
            )
            return response.content
        except HTTPError as e:
            message = e.args[0]
            if "404" in message:
                return ""
            raise e

    def fetch_plugin_by_identifier(
        self,
        tenant_id: str,
        identifier: str,
    ) -> bool:
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/fetch/identifier",
            bool,
            params={"plugin_unique_identifier": identifier},
        )

    def list_plugins(self, tenant_id: str) -> list[PluginEntity]:
        result = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/list",
            PluginListResponse,
            params={"page": 1, "page_size": 256, "response_type": "paged"},
        )
        return result.list

    def list_plugins_with_total(self, tenant_id: str, page: int, page_size: int) -> PluginListResponse:
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/list",
            PluginListResponse,
            params={"page": page, "page_size": page_size, "response_type": "paged"},
        )

    def upload_pkg(
        self,
        tenant_id: str,
        pkg: bytes,
        verify_signature: bool = False,
    ) -> PluginDecodeResponse:
        """
        Upload a plugin package and return the plugin unique identifier.
        """
        body = {
            "dify_pkg": ("dify_pkg", pkg, "application/octet-stream"),
        }

        data = {
            "verify_signature": "true" if verify_signature else "false",
        }

        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upload/package",
            PluginDecodeResponse,
            files=body,
            data=data,
        )

    def upload_bundle(
        self,
        tenant_id: str,
        bundle: bytes,
        verify_signature: bool = False,
    ) -> Sequence[PluginBundleDependency]:
        """
        Upload a plugin bundle and return the dependencies.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upload/bundle",
            list[PluginBundleDependency],
            files={"dify_bundle": ("dify_bundle", bundle, "application/octet-stream")},
            data={"verify_signature": "true" if verify_signature else "false"},
        )

    def install_from_identifiers(
        self,
        tenant_id: str,
        identifiers: Sequence[str],
        source: PluginInstallationSource,
        metas: list[dict],
    ) -> PluginInstallTaskStartResponse:
        """
        Install a plugin from an identifier.
        """
        # exception will be raised if the request failed
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/identifiers",
            PluginInstallTaskStartResponse,
            data={
                "plugin_unique_identifiers": identifiers,
                "source": source,
                "metas": metas,
            },
            headers={"Content-Type": "application/json"},
        )

    def fetch_plugin_installation_tasks(self, tenant_id: str, page: int, page_size: int) -> Sequence[PluginInstallTask]:
        """
        Fetch plugin installation tasks.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/install/tasks",
            list[PluginInstallTask],
            params={"page": page, "page_size": page_size},
        )

    def fetch_plugin_installation_task(self, tenant_id: str, task_id: str) -> PluginInstallTask:
        """
        Fetch a plugin installation task.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}",
            PluginInstallTask,
        )

    def delete_plugin_installation_task(self, tenant_id: str, task_id: str) -> bool:
        """
        Delete a plugin installation task.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}/delete",
            bool,
        )

    def delete_all_plugin_installation_task_items(self, tenant_id: str) -> bool:
        """
        Delete all plugin installation task items.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/delete_all",
            bool,
        )

    def delete_plugin_installation_task_item(self, tenant_id: str, task_id: str, identifier: str) -> bool:
        """
        Delete a plugin installation task item.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/tasks/{task_id}/delete/{identifier}",
            bool,
        )

    def fetch_plugin_manifest(self, tenant_id: str, plugin_unique_identifier: str) -> PluginDeclaration:
        """
        Fetch a plugin manifest.
        """

        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/fetch/manifest",
            PluginDeclaration,
            params={"plugin_unique_identifier": plugin_unique_identifier},
        )

    def decode_plugin_from_identifier(self, tenant_id: str, plugin_unique_identifier: str) -> PluginDecodeResponse:
        """
        Decode a plugin from an identifier.
        """
        return self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/decode/from_identifier",
            PluginDecodeResponse,
            params={"plugin_unique_identifier": plugin_unique_identifier},
        )

    def fetch_plugin_installation_by_ids(
        self, tenant_id: str, plugin_ids: Sequence[str]
    ) -> Sequence[PluginInstallation]:
        """
        Fetch plugin installations by ids.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/installation/fetch/batch",
            list[PluginInstallation],
            data={"plugin_ids": plugin_ids},
            headers={"Content-Type": "application/json"},
        )

    def fetch_missing_dependencies(
        self, tenant_id: str, plugin_unique_identifiers: list[str]
    ) -> list[MissingPluginDependency]:
        """
        Fetch missing dependencies
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/installation/missing",
            list[MissingPluginDependency],
            data={"plugin_unique_identifiers": plugin_unique_identifiers},
            headers={"Content-Type": "application/json"},
        )

    def uninstall(self, tenant_id: str, plugin_installation_id: str) -> bool:
        """
        Uninstall a plugin.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/uninstall",
            bool,
            data={
                "plugin_installation_id": plugin_installation_id,
            },
            headers={"Content-Type": "application/json"},
        )

    def upgrade_plugin(
        self,
        tenant_id: str,
        original_plugin_unique_identifier: str,
        new_plugin_unique_identifier: str,
        source: PluginInstallationSource,
        meta: dict,
    ) -> PluginInstallTaskStartResponse:
        """
        Upgrade a plugin.
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/install/upgrade",
            PluginInstallTaskStartResponse,
            data={
                "original_plugin_unique_identifier": original_plugin_unique_identifier,
                "new_plugin_unique_identifier": new_plugin_unique_identifier,
                "source": source,
                "meta": meta,
            },
            headers={"Content-Type": "application/json"},
        )

    def check_tools_existence(self, tenant_id: str, provider_ids: Sequence[GenericProviderID]) -> Sequence[bool]:
        """
        Check if the tools exist
        """
        return self._request_with_plugin_daemon_response(
            "POST",
            f"plugin/{tenant_id}/management/tools/check_existence",
            list[bool],
            data={
                "provider_ids": [
                    {
                        "plugin_id": provider_id.plugin_id,
                        "provider_name": provider_id.provider_name,
                    }
                    for provider_id in provider_ids
                ]
            },
            headers={"Content-Type": "application/json"},
        )

```

### api/core/plugin/impl/model_runtime.py
```py
from __future__ import annotations

import hashlib
import logging
from collections.abc import Generator, Iterable, Sequence
from threading import Lock
from typing import IO, Any, Union

from graphon.model_runtime.entities.llm_entities import LLMResult, LLMResultChunk
from graphon.model_runtime.entities.message_entities import PromptMessage, PromptMessageTool
from graphon.model_runtime.entities.model_entities import AIModelEntity, ModelType
from graphon.model_runtime.entities.provider_entities import ProviderEntity
from graphon.model_runtime.entities.rerank_entities import MultimodalRerankInput, RerankResult
from graphon.model_runtime.entities.text_embedding_entities import EmbeddingInputType, EmbeddingResult
from graphon.model_runtime.runtime import ModelRuntime
from pydantic import ValidationError
from redis import RedisError

from configs import dify_config
from core.plugin.entities.plugin_daemon import PluginModelProviderEntity
from core.plugin.impl.asset import PluginAssetManager
from core.plugin.impl.model import PluginModelClient
from extensions.ext_redis import redis_client
from models.provider_ids import ModelProviderID

logger = logging.getLogger(__name__)

# `TS` means tenant scope
TENANT_SCOPE_SCHEMA_CACHE_USER_ID = "__DIFY_TS__"


class PluginModelRuntime(ModelRuntime):
    """Plugin-backed runtime adapter bound to tenant context and optional caller scope."""

    tenant_id: str
    user_id: str | None
    client: PluginModelClient
    _provider_entities: tuple[ProviderEntity, ...] | None
    _provider_entities_lock: Lock

    def __init__(self, tenant_id: str, user_id: str | None, client: PluginModelClient) -> None:
        if client is None:
            raise ValueError("client is required.")
        self.tenant_id = tenant_id
        self.user_id = user_id
        self.client = client
        self._provider_entities = None
        self._provider_entities_lock = Lock()

    def fetch_model_providers(self) -> Sequence[ProviderEntity]:
        if self._provider_entities is not None:
            return self._provider_entities

        with self._provider_entities_lock:
            if self._provider_entities is None:
                self._provider_entities = tuple(
                    self._to_provider_entity(provider) for provider in self.client.fetch_model_providers(self.tenant_id)
                )

        return self._provider_entities

    def get_provider_icon(self, *, provider: str, icon_type: str, lang: str) -> tuple[bytes, str]:
        provider_schema = self._get_provider_schema(provider)

        if icon_type.lower() == "icon_small":
            if not provider_schema.icon_small:
                raise ValueError(f"Provider {provider} does not have small icon.")
            file_name = (
                provider_schema.icon_small.zh_Hans if lang.lower() == "zh_hans" else provider_schema.icon_small.en_US
            )
        elif icon_type.lower() == "icon_small_dark":
            if not provider_schema.icon_small_dark:
                raise ValueError(f"Provider {provider} does not have small dark icon.")
            file_name = (
                provider_schema.icon_small_dark.zh_Hans
                if lang.lower() == "zh_hans"
                else provider_schema.icon_small_dark.en_US
            )
        else:
            raise ValueError(f"Unsupported icon type: {icon_type}.")

        if not file_name:
            raise ValueError(f"Provider {provider} does not have icon.")

        image_mime_types = {
            "jpg": "image/jpeg",
            "jpeg": "image/jpeg",
            "png": "image/png",
            "gif": "image/gif",
            "bmp": "image/bmp",
            "tiff": "image/tiff",
            "tif": "image/tiff",
            "webp": "image/webp",
            "svg": "image/svg+xml",
            "ico": "image/vnd.microsoft.icon",
            "heif": "image/heif",
            "heic": "image/heic",
        }

        extension = file_name.split(".")[-1]
        mime_type = image_mime_types.get(extension, "image/png")
        return PluginAssetManager().fetch_asset(tenant_id=self.tenant_id, id=file_name), mime_type

    def validate_provider_credentials(self, *, provider: str, credentials: dict[str, Any]) -> None:
        plugin_id, provider_name = self._split_provider(provider)
        self.client.validate_provider_credentials(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            credentials=credentials,
        )

    def validate_model_credentials(
        self,
        *,
        provider: str,
        model_type: ModelType,
        model: str,
        credentials: dict[str, Any],
    ) -> None:
        plugin_id, provider_name = self._split_provider(provider)
        self.client.validate_model_credentials(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model_type=model_type.value,
            model=model,
            credentials=credentials,
        )

    def get_model_schema(
        self,
        *,
        provider: str,
        model_type: ModelType,
        model: str,
        credentials: dict[str, Any],
    ) -> AIModelEntity | None:
        cache_key = self._get_schema_cache_key(
            provider=provider,
            model_type=model_type,
            model=model,
            credentials=credentials,
        )

        cached_schema_json = None
        try:
            cached_schema_json = redis_client.get(cache_key)
        except (RedisError, RuntimeError) as exc:
            logger.warning(
                "Failed to read plugin model schema cache for model %s: %s",
                model,
                str(exc),
                exc_info=True,
            )

        if cached_schema_json:
            try:
                return AIModelEntity.model_validate_json(cached_schema_json)
            except ValidationError:
                logger.warning("Failed to validate cached plugin model schema for model %s", model, exc_info=True)
                try:
                    redis_client.delete(cache_key)
                except (RedisError, RuntimeError) as exc:
                    logger.warning(
                        "Failed to delete invalid plugin model schema cache for model %s: %s",
                        model,
                        str(exc),
                        exc_info=True,
                    )

        plugin_id, provider_name = self._split_provider(provider)
        schema = self.client.get_model_schema(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model_type=model_type.value,
            model=model,
            credentials=credentials,
        )

        if schema:
            try:
                redis_client.setex(cache_key, dify_config.PLUGIN_MODEL_SCHEMA_CACHE_TTL, schema.model_dump_json())
            except (RedisError, RuntimeError) as exc:
                logger.warning(
                    "Failed to write plugin model schema cache for model %s: %s",
                    model,
                    str(exc),
                    exc_info=True,
                )

        return schema

    def invoke_llm(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        model_parameters: dict[str, Any],
        prompt_messages: Sequence[PromptMessage],
        tools: list[PromptMessageTool] | None,
        stop: Sequence[str] | None,
        stream: bool,
    ) -> Union[LLMResult, Generator[LLMResultChunk, None, None]]:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_llm(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            model_parameters=model_parameters,
            prompt_messages=list(prompt_messages),
            tools=tools,
            stop=list(stop) if stop else None,
            stream=stream,
        )

    def get_llm_num_tokens(
        self,
        *,
        provider: str,
        model_type: ModelType,
        model: str,
        credentials: dict[str, Any],
        prompt_messages: Sequence[PromptMessage],
        tools: Sequence[PromptMessageTool] | None,
    ) -> int:
        if not dify_config.PLUGIN_BASED_TOKEN_COUNTING_ENABLED:
            return 0

        plugin_id, provider_name = self._split_provider(provider)
        return self.client.get_llm_num_tokens(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model_type=model_type.value,
            model=model,
            credentials=credentials,
            prompt_messages=list(prompt_messages),
            tools=list(tools) if tools else None,
        )

    def invoke_text_embedding(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        texts: list[str],
        input_type: EmbeddingInputType,
    ) -> EmbeddingResult:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_text_embedding(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            texts=texts,
            input_type=input_type,
        )

    def invoke_multimodal_embedding(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        documents: list[dict[str, Any]],
        input_type: EmbeddingInputType,
    ) -> EmbeddingResult:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_multimodal_embedding(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            documents=documents,
            input_type=input_type,
        )

    def get_text_embedding_num_tokens(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        texts: list[str],
    ) -> list[int]:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.get_text_embedding_num_tokens(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            texts=texts,
        )

    def invoke_rerank(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        query: str,
        docs: list[str],
        score_threshold: float | None,
        top_n: int | None,
    ) -> RerankResult:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_rerank(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            query=query,
            docs=docs,
            score_threshold=score_threshold,
            top_n=top_n,
        )

    def invoke_multimodal_rerank(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        query: MultimodalRerankInput,
        docs: list[MultimodalRerankInput],
        score_threshold: float | None,
        top_n: int | None,
    ) -> RerankResult:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_multimodal_rerank(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            query=query,
            docs=docs,
            score_threshold=score_threshold,
            top_n=top_n,
        )

    def invoke_tts(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        content_text: str,
        voice: str,
    ) -> Iterable[bytes]:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_tts(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            content_text=content_text,
            voice=voice,
        )

    def get_tts_model_voices(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        language: str | None,
    ) -> Any:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.get_tts_model_voices(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            language=language,
        )

    def invoke_speech_to_text(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        file: IO[bytes],
    ) -> str:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_speech_to_text(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            file=file,
        )

    def invoke_moderation(
        self,
        *,
        provider: str,
        model: str,
        credentials: dict[str, Any],
        text: str,
    ) -> bool:
        plugin_id, provider_name = self._split_provider(provider)
        return self.client.invoke_moderation(
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            plugin_id=plugin_id,
            provider=provider_name,
            model=model,
            credentials=credentials,
            text=text,
        )

    def _get_provider_short_name_alias(self, provider: PluginModelProviderEntity) -> str:
        """
        Expose a bare provider alias only for the canonical provider mapping.

        Multiple plugins can publish the same short provider slug. If every
        provider entity keeps that slug in ``provider_name``, callers that still
        resolve by short name become order-dependent. Restrict the alias to the
        provider selected by ``ModelProviderID`` so legacy short-name lookups
        remain deterministic while the runtime surface stays canonical.
        """
        try:
            canonical_provider_id = ModelProviderID(provider.provider)
        except ValueError:
            return ""

        if canonical_provider_id.plugin_id != provider.plugin_id:
            return ""
        if canonical_provider_id.provider_name != provider.provider:
            return ""

        return provider.provider

    def _to_provider_entity(self, provider: PluginModelProviderEntity) -> ProviderEntity:
        declaration = provider.declaration.model_copy(deep=True)
        declaration.provider = f"{provider.plugin_id}/{provider.provider}"
        declaration.provider_name = self._get_provider_short_name_alias(provider)
        return declaration

    def _get_provider_schema(self, provider: str) -> ProviderEntity:
        providers = self.fetch_model_providers()
        provider_entity = next((item for item in providers if item.provider == provider), None)
        if provider_entity is None:
            provider_entity = next((item for item in providers if provider == item.provider_name), None)
        if provider_entity is None:
            raise ValueError(f"Invalid provider: {provider}")
        return provider_entity

    def _get_schema_cache_key(
        self,
        *,
        provider: str,
        model_type: ModelType,
        model: str,
        credentials: dict[str, Any],
    ) -> str:
        # The plugin daemon distinguishes ``None`` from an explicit empty-string
        # caller id, so the cache must only collapse ``None`` into tenant scope.
        cache_user_id = TENANT_SCOPE_SCHEMA_CACHE_USER_ID if self.user_id is None else self.user_id
        cache_key = f"{self.tenant_id}:{provider}:{model_type.value}:{model}:{cache_user_id}"
        sorted_credentials = sorted(credentials.items()) if credentials else []
        if not sorted_credentials:
            return cache_key
        hashed_credentials = ":".join(
            [hashlib.md5(f"{key}:{value}".encode()).hexdigest() for key, value in sorted_credentials]
        )
        return f"{cache_key}:{hashed_credentials}"

    def _split_provider(self, provider: str) -> tuple[str, str]:
        provider_id = ModelProviderID(provider)
        return provider_id.plugin_id, provider_id.provider_name

```

### api/core/plugin/impl/tool.py
```py
from collections.abc import Generator
from typing import Any

from pydantic import BaseModel

from configs import dify_config

# from core.plugin.entities.plugin import GenericProviderID, ToolProviderID
from core.plugin.entities.plugin_daemon import CredentialType, PluginBasicBooleanResponse, PluginToolProviderEntity
from core.plugin.impl.base import BasePluginClient
from core.plugin.utils.chunk_merger import merge_blob_chunks
from core.schemas.resolver import resolve_dify_schema_refs
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolParameter
from models.provider_ids import GenericProviderID, ToolProviderID


class PluginToolManager(BasePluginClient):
    def fetch_tool_providers(self, tenant_id: str) -> list[PluginToolProviderEntity]:
        """
        Fetch tool providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]):
            for provider in json_response.get("data", []):
                declaration = provider.get("declaration", {}) or {}
                provider_name = declaration.get("identity", {}).get("name")
                for tool in declaration.get("tools", []):
                    tool["identity"]["provider"] = provider_name
                    # resolve refs
                    if tool.get("output_schema"):
                        tool["output_schema"] = resolve_dify_schema_refs(tool["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/tools",
            list[PluginToolProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )

        for provider in response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each tool to plugin_id/provider_name
            for tool in provider.declaration.tools:
                tool.identity.provider = provider.declaration.identity.name

        return response

    def fetch_tool_provider(self, tenant_id: str, provider: str) -> PluginToolProviderEntity:
        """
        Fetch tool provider for the given tenant and plugin.
        """
        tool_provider_id = ToolProviderID(provider)

        def transformer(json_response: dict[str, Any]):
            data = json_response.get("data")
            if data:
                for tool in data.get("declaration", {}).get("tools", []):
                    tool["identity"]["provider"] = tool_provider_id.provider_name
                    # resolve refs
                    if tool.get("output_schema"):
                        tool["output_schema"] = resolve_dify_schema_refs(tool["output_schema"])

            return json_response

        response = self._request_with_plugin_daemon_response(
            "GET",
            f"plugin/{tenant_id}/management/tool",
            PluginToolProviderEntity,
            params={"provider": tool_provider_id.provider_name, "plugin_id": tool_provider_id.plugin_id},
            transformer=transformer,
        )

        response.declaration.identity.name = f"{response.plugin_id}/{response.declaration.identity.name}"

        # override the provider name for each tool to plugin_id/provider_name
        for tool in response.declaration.tools:
            tool.identity.provider = response.declaration.identity.name

        return response

    def invoke(
        self,
        tenant_id: str,
        user_id: str,
        tool_provider: str,
        tool_name: str,
        credentials: dict[str, Any],
        credential_type: CredentialType,
        tool_parameters: dict[str, Any],
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        Invoke the tool with the given tenant, user, plugin, provider, name, credentials and parameters.
        """

        tool_provider_id = GenericProviderID(tool_provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/invoke",
            ToolInvokeMessage,
            data={
                "user_id": user_id,
                "conversation_id": conversation_id,
                "app_id": app_id,
                "message_id": message_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "tool": tool_name,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "tool_parameters": tool_parameters,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        return merge_blob_chunks(response, max_file_size=dify_config.PLUGIN_MAX_FILE_SIZE)

    def validate_provider_credentials(
        self, tenant_id: str, user_id: str, provider: str, credentials: dict[str, Any]
    ) -> bool:
        """
        validate the credentials of the provider
        """
        tool_provider_id = GenericProviderID(provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/validate_credentials",
            PluginBasicBooleanResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        return False

    def validate_datasource_credentials(
        self, tenant_id: str, user_id: str, provider: str, credentials: dict[str, Any]
    ) -> bool:
        """
        validate the credentials of the datasource
        """
        tool_provider_id = GenericProviderID(provider)

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/datasource/validate_credentials",
            PluginBasicBooleanResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.result

        return False

    def get_runtime_parameters(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        credentials: dict[str, Any],
        tool: str,
        conversation_id: str | None = None,
        app_id: str | None = None,
        message_id: str | None = None,
    ) -> list[ToolParameter]:
        """
        get the runtime parameters of the tool
        """
        tool_provider_id = GenericProviderID(provider)

        class RuntimeParametersResponse(BaseModel):
            parameters: list[ToolParameter]

        response = self._request_with_plugin_daemon_response_stream(
            "POST",
            f"plugin/{tenant_id}/dispatch/tool/get_runtime_parameters",
            RuntimeParametersResponse,
            data={
                "user_id": user_id,
                "conversation_id": conversation_id,
                "app_id": app_id,
                "message_id": message_id,
                "data": {
                    "provider": tool_provider_id.provider_name,
                    "tool": tool,
                    "credentials": credentials,
                },
            },
            headers={
                "X-Plugin-ID": tool_provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp.parameters

        return []

```

### api/core/plugin/impl/base.py
```py
import inspect
import json
import logging
from collections.abc import Callable, Generator
from typing import Any, cast

import httpx
from graphon.model_runtime.errors.invoke import (
    InvokeAuthorizationError,
    InvokeBadRequestError,
    InvokeConnectionError,
    InvokeRateLimitError,
    InvokeServerUnavailableError,
)
from graphon.model_runtime.errors.validate import CredentialsValidateFailedError
from pydantic import BaseModel
from yarl import URL

from configs import dify_config
from core.helper.http_client_pooling import get_pooled_http_client
from core.plugin.endpoint.exc import EndpointSetupFailedError
from core.plugin.entities.plugin_daemon import PluginDaemonBasicResponse, PluginDaemonError, PluginDaemonInnerError
from core.plugin.impl.exc import (
    PluginDaemonBadRequestError,
    PluginDaemonClientSideError,
    PluginDaemonInternalServerError,
    PluginDaemonNotFoundError,
    PluginDaemonUnauthorizedError,
    PluginInvokeError,
    PluginNotFoundError,
    PluginPermissionDeniedError,
    PluginUniqueIdentifierError,
)
from core.trigger.errors import (
    EventIgnoreError,
    TriggerInvokeError,
    TriggerPluginInvokeError,
    TriggerProviderCredentialValidationError,
)

plugin_daemon_inner_api_baseurl = URL(str(dify_config.PLUGIN_DAEMON_URL))
_plugin_daemon_timeout_config = cast(
    float | httpx.Timeout | None,
    getattr(dify_config, "PLUGIN_DAEMON_TIMEOUT", 600.0),
)
plugin_daemon_request_timeout: httpx.Timeout | None
if _plugin_daemon_timeout_config is None:
    plugin_daemon_request_timeout = None
elif isinstance(_plugin_daemon_timeout_config, httpx.Timeout):
    plugin_daemon_request_timeout = _plugin_daemon_timeout_config
else:
    plugin_daemon_request_timeout = httpx.Timeout(_plugin_daemon_timeout_config)

logger = logging.getLogger(__name__)

_httpx_client: httpx.Client = get_pooled_http_client(
    "plugin_daemon",
    lambda: httpx.Client(limits=httpx.Limits(max_keepalive_connections=50, max_connections=100), trust_env=False),
)


class BasePluginClient:
    def _request(
        self,
        method: str,
        path: str,
        headers: dict[str, str] | None = None,
        data: bytes | dict[str, Any] | str | None = None,
        params: dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
    ) -> httpx.Response:
        """
        Make a request to the plugin daemon inner API.
        """
        url, headers, prepared_data, params, files = self._prepare_request(path, headers, data, params, files)

        request_kwargs: dict[str, Any] = {
            "method": method,
            "url": url,
            "headers": headers,
            "params": params,
            "files": files,
            "timeout": plugin_daemon_request_timeout,
        }
        if isinstance(prepared_data, dict):
            request_kwargs["data"] = prepared_data
        elif prepared_data is not None:
            request_kwargs["content"] = prepared_data

        try:
            response = _httpx_client.request(**request_kwargs)
        except httpx.RequestError:
            logger.exception("Request to Plugin Daemon Service failed")
            raise PluginDaemonInnerError(code=-500, message="Request to Plugin Daemon Service failed")

        return response

    def _prepare_request(
        self,
        path: str,
        headers: dict[str, str] | None,
        data: bytes | dict[str, Any] | str | None,
        params: dict[str, Any] | None,
        files: dict[str, Any] | None,
    ) -> tuple[str, dict[str, str], bytes | dict[str, Any] | str | None, dict[str, Any] | None, dict[str, Any] | None]:
        url = plugin_daemon_inner_api_baseurl / path
        prepared_headers = dict(headers or {})
        prepared_headers["X-Api-Key"] = dify_config.PLUGIN_DAEMON_KEY
        prepared_headers.setdefault("Accept-Encoding", "gzip, deflate, br")

        # Inject traceparent header for distributed tracing
        self._inject_trace_headers(prepared_headers)

        prepared_data: bytes | dict[str, Any] | str | None = (
            data if isinstance(data, (bytes, str, dict)) or data is None else None
        )
        if isinstance(data, dict):
            if prepared_headers.get("Content-Type") == "application/json":
                prepared_data = json.dumps(data)
            else:
                prepared_data = data

        return str(url), prepared_headers, prepared_data, params, files

    def _inject_trace_headers(self, headers: dict[str, str]) -> None:
        """
        Inject W3C traceparent header for distributed tracing.

        This ensures trace context is propagated to plugin daemon even if
        HTTPXClientInstrumentor doesn't cover module-level httpx functions.
        """
        if not dify_config.ENABLE_OTEL:
            return

        import contextlib

        # Skip if already present (case-insensitive check)
        for key in headers:
            if key.lower() == "traceparent":
                return

        # Inject traceparent - works as fallback when OTEL instrumentation doesn't cover this call
        with contextlib.suppress(Exception):
            from core.helper.trace_id_helper import generate_traceparent_header

            traceparent = generate_traceparent_header()
            if traceparent:
                headers["traceparent"] = traceparent

    def _stream_request(
        self,
        method: str,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        data: bytes | dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
    ) -> Generator[str, None, None]:
        """
        Make a stream request to the plugin daemon inner API
        """
        url, headers, prepared_data, params, files = self._prepare_request(path, headers, data, params, files)

        stream_kwargs: dict[str, Any] = {
            "method": method,
            "url": url,
            "headers": headers,
            "params": params,
            "files": files,
            "timeout": plugin_daemon_request_timeout,
        }
        if isinstance(prepared_data, dict):
            stream_kwargs["data"] = prepared_data
        elif prepared_data is not None:
            stream_kwargs["content"] = prepared_data

        try:
            with _httpx_client.stream(**stream_kwargs) as response:
                for raw_line in response.iter_lines():
                    if not raw_line:
                        continue
                    line = raw_line.decode("utf-8") if isinstance(raw_line, bytes) else raw_line
                    line = line.strip()
                    if line.startswith("data:"):
                        line = line[5:].strip()
                    if line:
                        yield line
        except httpx.RequestError:
            logger.exception("Stream request to Plugin Daemon Service failed")
            raise PluginDaemonInnerError(code=-500, message="Request to Plugin Daemon Service failed")

    def _stream_request_with_model[T: BaseModel | dict[str, Any] | list[Any] | bool | str](
        self,
        method: str,
        path: str,
        type_: type[T],
        headers: dict[str, str] | None = None,
        data: bytes | dict[str, Any] | None = None,
        params: dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
    ) -> Generator[T, None, None]:
        """
        Make a stream request to the plugin daemon inner API and yield the response as a model.
        """
        for line in self._stream_request(method, path, params, headers, data, files):
            yield type_(**json.loads(line))  # type: ignore

    def _request_with_model[T: BaseModel | dict[str, Any] | list[Any] | bool | str](
        self,
        method: str,
        path: str,
        type_: type[T],
        headers: dict[str, str] | None = None,
        data: bytes | None = None,
        params: dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
    ) -> T:
        """
        Make a request to the plugin daemon inner API and return the response as a model.
        """
        response = self._request(method, path, headers, data, params, files)
        return type_(**response.json())  # type: ignore[return-value]

    def _request_with_plugin_daemon_response[T: BaseModel | dict[str, Any] | list[Any] | bool | str](
        self,
        method: str,
        path: str,
        type_: type[T],
        headers: dict[str, str] | None = None,
        data: bytes | dict[str, Any] | None = None,
        params: dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
        transformer: Callable[[dict[str, Any]], dict[str, Any]] | None = None,
    ) -> T:
        """
        Make a request to the plugin daemon inner API and return the response as a model.
        """
        try:
            response = self._request(method, path, headers, data, params, files)
            response.raise_for_status()
        except httpx.HTTPStatusError as e:
            logger.exception("Failed to request plugin daemon, status: %s, url: %s", e.response.status_code, path)
            if e.response.status_code < 500:
                raise PluginDaemonClientSideError(description=str(e))
            else:
                raise PluginDaemonInternalServerError(description=str(e))
        except Exception as e:
            msg = f"Failed to request plugin daemon, url: {path}"
            logger.exception("Failed to request plugin daemon, url: %s", path)
            raise ValueError(msg) from e

        try:
            json_response = response.json()
            if transformer:
                json_response = transformer(json_response)
            # https://stackoverflow.com/questions/59634937/variable-foo-class-is-not-valid-as-type-but-why
            rep = PluginDaemonBasicResponse[type_].model_validate(json_response)  # type: ignore
        except Exception:
            msg = (
                f"Failed to parse response from plugin daemon to PluginDaemonBasicResponse [{str(type_.__name__)}],"
                f" url: {path}"
            )
            logger.exception(msg)
            raise ValueError(msg)

        if rep.code != 0:
            try:
                error = PluginDaemonError.model_validate(json.loads(rep.message))
            except Exception:
                raise ValueError(f"{rep.message}, code: {rep.code}")

            self._handle_plugin_daemon_error(error.error_type, error.message)
        if rep.data is None:
            frame = inspect.currentframe()
            raise ValueError(f"got empty data from plugin daemon: {frame.f_lineno if frame else 'unknown'}")

        return rep.data

    def _request_with_plugin_daemon_response_stream[T: BaseModel | dict[str, Any] | list[Any] | bool | str](
        self,
        method: str,
        path: str,
        type_: type[T],
        headers: dict[str, str] | None = None,
        data: bytes | dict[str, Any] | None = None,
        params: dict[str, Any] | None = None,
        files: dict[str, Any] | None = None,
    ) -> Generator[T, None, None]:
        """
        Make a stream request to the plugin daemon inner API and yield the response as a model.
        """
        for line in self._stream_request(method, path, params, headers, data, files):
            try:
                rep = PluginDaemonBasicResponse[type_].model_validate_json(line)  # type: ignore
            except (ValueError, TypeError):
                # TODO modify this when line_data has code and message
                try:
                    line_data = json.loads(line)
                except (ValueError, TypeError):
                    raise ValueError(line)
                # If the dictionary contains the `error` key, use its value as the argument
                # for `ValueError`.
                # Otherwise, use the `line` to provide better contextual information about the error.
                raise ValueError(line_data.get("error", line))

            if rep.code != 0:
                if rep.code == -500:
                    try:
                        error = PluginDaemonError.model_validate(json.loads(rep.message))
                    except Exception:
                        raise PluginDaemonInnerError(code=rep.code, message=rep.message)

                    logger.error("Error in stream response for plugin %s", rep.__dict__)
                    self._handle_plugin_daemon_error(error.error_type, error.message)
                raise ValueError(f"plugin daemon: {rep.message}, code: {rep.code}")
            if rep.data is None:
                frame = inspect.currentframe()
                raise ValueError(f"got empty data from plugin daemon: {frame.f_lineno if frame else 'unknown'}")
            yield rep.data

    def _handle_plugin_daemon_error(self, error_type: str, message: str):
        """
        handle the error from plugin daemon
        """
        match error_type:
            case PluginDaemonInnerError.__name__:
                raise PluginDaemonInnerError(code=-500, message=message)
            case PluginInvokeError.__name__:
                error_object = json.loads(message)
                invoke_error_type = error_object.get("error_type")
                match invoke_error_type:
                    case InvokeRateLimitError.__name__:
                        raise InvokeRateLimitError(description=error_object.get("message"))
                    case InvokeAuthorizationError.__name__:
                        raise InvokeAuthorizationError(description=error_object.get("message"))
                    case InvokeBadRequestError.__name__:
                        raise InvokeBadRequestError(description=error_object.get("message"))
                    case InvokeConnectionError.__name__:
                        raise InvokeConnectionError(description=error_object.get("message"))
                    case InvokeServerUnavailableError.__name__:
                        raise InvokeServerUnavailableError(description=error_object.get("message"))
                    case CredentialsValidateFailedError.__name__:
                        raise CredentialsValidateFailedError(error_object.get("message"))
                    case EndpointSetupFailedError.__name__:
                        raise EndpointSetupFailedError(error_object.get("message"))
                    case TriggerProviderCredentialValidationError.__name__:
                        raise TriggerProviderCredentialValidationError(error_object.get("message"))
                    case TriggerPluginInvokeError.__name__:
                        raise TriggerPluginInvokeError(description=error_object.get("message"))
                    case TriggerInvokeError.__name__:
                        raise TriggerInvokeError(error_object.get("message"))
                    case EventIgnoreError.__name__:
                        raise EventIgnoreError(description=error_object.get("message"))
                    case _:
                        raise PluginInvokeError(description=message)
            case PluginDaemonInternalServerError.__name__:
                raise PluginDaemonInternalServerError(description=message)
            case PluginDaemonBadRequestError.__name__:
                raise PluginDaemonBadRequestError(description=message)
            case PluginDaemonNotFoundError.__name__:
                raise PluginDaemonNotFoundError(description=message)
            case PluginUniqueIdentifierError.__name__:
                raise PluginUniqueIdentifierError(description=message)
            case PluginNotFoundError.__name__:
                raise PluginNotFoundError(description=message)
            case PluginDaemonUnauthorizedError.__name__:
                raise PluginDaemonUnauthorizedError(description=message)
            case PluginPermissionDeniedError.__name__:
                raise PluginPermissionDeniedError(description=message)
            case _:
                raise Exception(f"got unknown error from plugin daemon: {error_type}, message: {message}")

```

### api/core/plugin/impl/oauth.py
```py
import binascii
from collections.abc import Mapping
from typing import Any

from werkzeug import Request

from core.plugin.entities.plugin_daemon import PluginOAuthAuthorizationUrlResponse, PluginOAuthCredentialsResponse
from core.plugin.impl.base import BasePluginClient


class OAuthHandler(BasePluginClient):
    def get_authorization_url(
        self,
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        redirect_uri: str,
        system_credentials: Mapping[str, Any],
    ) -> PluginOAuthAuthorizationUrlResponse:
        try:
            response = self._request_with_plugin_daemon_response_stream(
                "POST",
                f"plugin/{tenant_id}/dispatch/oauth/get_authorization_url",
                PluginOAuthAuthorizationUrlResponse,
                data={
                    "user_id": user_id,
                    "data": {
                        "provider": provider,
                        "redirect_uri": redirect_uri,
                        "system_credentials": system_credentials,
                    },
                },
                headers={
                    "X-Plugin-ID": plugin_id,
                    "Content-Type": "application/json",
                },
            )
            for resp in response:
                return resp
            raise ValueError("No response received from plugin daemon for authorization URL request.")
        except Exception as e:
            raise ValueError(f"Error getting authorization URL: {e}")

    def get_credentials(
        self,
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        redirect_uri: str,
        system_credentials: Mapping[str, Any],
        request: Request,
    ) -> PluginOAuthCredentialsResponse:
        """
        Get credentials from the given request.
        """

        try:
            # encode request to raw http request
            raw_request_bytes = self._convert_request_to_raw_data(request)
            response = self._request_with_plugin_daemon_response_stream(
                "POST",
                f"plugin/{tenant_id}/dispatch/oauth/get_credentials",
                PluginOAuthCredentialsResponse,
                data={
                    "user_id": user_id,
                    "data": {
                        "provider": provider,
                        "redirect_uri": redirect_uri,
                        "system_credentials": system_credentials,
                        # for json serialization
                        "raw_http_request": binascii.hexlify(raw_request_bytes).decode(),
                    },
                },
                headers={
                    "X-Plugin-ID": plugin_id,
                    "Content-Type": "application/json",
                },
            )
            for resp in response:
                return resp
            raise ValueError("No response received from plugin daemon for authorization URL request.")
        except Exception as e:
            raise ValueError(f"Error getting credentials: {e}")

    def refresh_credentials(
        self,
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        redirect_uri: str,
        system_credentials: Mapping[str, Any],
        credentials: Mapping[str, Any],
    ) -> PluginOAuthCredentialsResponse:
        try:
            response = self._request_with_plugin_daemon_response_stream(
                "POST",
                f"plugin/{tenant_id}/dispatch/oauth/refresh_credentials",
                PluginOAuthCredentialsResponse,
                data={
                    "user_id": user_id,
                    "data": {
                        "provider": provider,
                        "redirect_uri": redirect_uri,
                        "system_credentials": system_credentials,
                        "credentials": credentials,
                    },
                },
                headers={
                    "X-Plugin-ID": plugin_id,
                    "Content-Type": "application/json",
                },
            )
            for resp in response:
                return resp
            raise ValueError("No response received from plugin daemon for refresh credentials request.")
        except Exception as e:
            raise ValueError(f"Error refreshing credentials: {e}")

    def _convert_request_to_raw_data(self, request: Request) -> bytes:
        """
        Convert a Request object to raw HTTP data.

        Args:
            request: The Request object to convert.

        Returns:
            The raw HTTP data as bytes.
        """
        # Start with the request line
        method = request.method
        path = request.full_path
        protocol = request.headers.get("HTTP_VERSION", "HTTP/1.1")
        raw_data = f"{method} {path} {protocol}\r\n".encode()

        # Add headers
        for header_name, header_value in request.headers.items():
            raw_data += f"{header_name}: {header_value}\r\n".encode()

        # Add empty line to separate headers from body
        raw_data += b"\r\n"

        # Add body if exists
        body = request.get_data(as_text=False)
        if body:
            raw_data += body

        return raw_data

```

### api/core/plugin/impl/trigger.py
```py
import binascii
from collections.abc import Generator, Mapping
from typing import Any

from flask import Request

from core.plugin.entities.plugin_daemon import CredentialType, PluginTriggerProviderEntity
from core.plugin.entities.request import (
    TriggerDispatchResponse,
    TriggerInvokeEventResponse,
    TriggerSubscriptionResponse,
    TriggerValidateProviderCredentialsResponse,
)
from core.plugin.impl.base import BasePluginClient
from core.plugin.utils.http_parser import serialize_request
from core.trigger.entities.entities import Subscription
from models.provider_ids import TriggerProviderID


class PluginTriggerClient(BasePluginClient):
    def fetch_trigger_providers(self, tenant_id: str) -> list[PluginTriggerProviderEntity]:
        """
        Fetch trigger providers for the given tenant.
        """

        def transformer(json_response: dict[str, Any]) -> dict[str, Any]:
            for provider in json_response.get("data", []):
                declaration = provider.get("declaration", {}) or {}
                provider_id = provider.get("plugin_id") + "/" + provider.get("provider")
                for event in declaration.get("events", []):
                    event["identity"]["provider"] = provider_id

            return json_response

        response: list[PluginTriggerProviderEntity] = self._request_with_plugin_daemon_response(
            method="GET",
            path=f"plugin/{tenant_id}/management/triggers",
            type_=list[PluginTriggerProviderEntity],
            params={"page": 1, "page_size": 256},
            transformer=transformer,
        )

        for provider in response:
            provider.declaration.identity.name = f"{provider.plugin_id}/{provider.declaration.identity.name}"

            # override the provider name for each trigger to plugin_id/provider_name
            for event in provider.declaration.events:
                event.identity.provider = provider.declaration.identity.name

        return response

    def fetch_trigger_provider(self, tenant_id: str, provider_id: TriggerProviderID) -> PluginTriggerProviderEntity:
        """
        Fetch trigger provider for the given tenant and plugin.
        """

        def transformer(json_response: dict[str, Any]) -> dict[str, Any]:
            data = json_response.get("data")
            if data:
                for event in data.get("declaration", {}).get("events", []):
                    event["identity"]["provider"] = str(provider_id)

            return json_response

        response: PluginTriggerProviderEntity = self._request_with_plugin_daemon_response(
            method="GET",
            path=f"plugin/{tenant_id}/management/trigger",
            type_=PluginTriggerProviderEntity,
            params={"provider": provider_id.provider_name, "plugin_id": provider_id.plugin_id},
            transformer=transformer,
        )

        response.declaration.identity.name = str(provider_id)

        # override the provider name for each trigger to plugin_id/provider_name
        for event in response.declaration.events:
            event.identity.provider = str(provider_id)

        return response

    def invoke_trigger_event(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        event_name: str,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
        request: Request,
        parameters: Mapping[str, Any],
        subscription: Subscription,
        payload: Mapping[str, Any],
    ) -> TriggerInvokeEventResponse:
        """
        Invoke a trigger with the given parameters.
        """
        provider_id = TriggerProviderID(provider)
        response: Generator[TriggerInvokeEventResponse, None, None] = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/trigger/invoke_event",
            type_=TriggerInvokeEventResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": provider_id.provider_name,
                    "event": event_name,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "subscription": subscription.model_dump(),
                    "raw_http_request": binascii.hexlify(serialize_request(request)).decode(),
                    "parameters": parameters,
                    "payload": payload,
                },
            },
            headers={
                "X-Plugin-ID": provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("No response received from plugin daemon for invoke trigger")

    def validate_provider_credentials(
        self, tenant_id: str, user_id: str, provider: str, credentials: Mapping[str, str]
    ) -> bool:
        """
        Validate the credentials of the trigger provider.
        """
        provider_id = TriggerProviderID(provider)
        response: Generator[TriggerValidateProviderCredentialsResponse, None, None] = (
            self._request_with_plugin_daemon_response_stream(
                method="POST",
                path=f"plugin/{tenant_id}/dispatch/trigger/validate_credentials",
                type_=TriggerValidateProviderCredentialsResponse,
                data={
                    "user_id": user_id,
                    "data": {
                        "provider": provider_id.provider_name,
                        "credentials": credentials,
                    },
                },
                headers={
                    "X-Plugin-ID": provider_id.plugin_id,
                    "Content-Type": "application/json",
                },
            )
        )

        for resp in response:
            return resp.result

        raise ValueError("No response received from plugin daemon for validate provider credentials")

    def dispatch_event(
        self,
        tenant_id: str,
        provider: str,
        subscription: Mapping[str, Any],
        request: Request,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> TriggerDispatchResponse:
        """
        Dispatch an event to triggers.
        """
        provider_id = TriggerProviderID(provider)
        response = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/trigger/dispatch_event",
            type_=TriggerDispatchResponse,
            data={
                "data": {
                    "provider": provider_id.provider_name,
                    "subscription": subscription,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "raw_http_request": binascii.hexlify(serialize_request(request)).decode(),
                },
            },
            headers={
                "X-Plugin-ID": provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("No response received from plugin daemon for dispatch event")

    def subscribe(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
        endpoint: str,
        parameters: Mapping[str, Any],
    ) -> TriggerSubscriptionResponse:
        """
        Subscribe to a trigger.
        """
        provider_id = TriggerProviderID(provider)
        response: Generator[TriggerSubscriptionResponse, None, None] = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/trigger/subscribe",
            type_=TriggerSubscriptionResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": provider_id.provider_name,
                    "credentials": credentials,
                    "credential_type": credential_type,
                    "endpoint": endpoint,
                    "parameters": parameters,
                },
            },
            headers={
                "X-Plugin-ID": provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("No response received from plugin daemon for subscribe")

    def unsubscribe(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        subscription: Subscription,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> TriggerSubscriptionResponse:
        """
        Unsubscribe from a trigger.
        """
        provider_id = TriggerProviderID(provider)
        response: Generator[TriggerSubscriptionResponse, None, None] = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/trigger/unsubscribe",
            type_=TriggerSubscriptionResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": provider_id.provider_name,
                    "subscription": subscription.model_dump(),
                    "credentials": credentials,
                    "credential_type": credential_type,
                },
            },
            headers={
                "X-Plugin-ID": provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("No response received from plugin daemon for unsubscribe")

    def refresh(
        self,
        tenant_id: str,
        user_id: str,
        provider: str,
        subscription: Subscription,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> TriggerSubscriptionResponse:
        """
        Refresh a trigger subscription.
        """
        provider_id = TriggerProviderID(provider)
        response: Generator[TriggerSubscriptionResponse, None, None] = self._request_with_plugin_daemon_response_stream(
            method="POST",
            path=f"plugin/{tenant_id}/dispatch/trigger/refresh",
            type_=TriggerSubscriptionResponse,
            data={
                "user_id": user_id,
                "data": {
                    "provider": provider_id.provider_name,
                    "subscription": subscription.model_dump(),
                    "credentials": credentials,
                    "credential_type": credential_type,
                },
            },
            headers={
                "X-Plugin-ID": provider_id.plugin_id,
                "Content-Type": "application/json",
            },
        )

        for resp in response:
            return resp

        raise ValueError("No response received from plugin daemon for refresh")

```

### api/core/plugin/utils/converter.py
```py
from typing import Any

from graphon.file import File

from core.tools.entities.tool_entities import ToolSelector


def convert_parameters_to_plugin_format(parameters: dict[str, Any]) -> dict[str, Any]:
    for parameter_name, parameter in parameters.items():
        if isinstance(parameter, File):
            parameters[parameter_name] = parameter.to_plugin_parameter()
        elif isinstance(parameter, list) and all(isinstance(p, File) for p in parameter):
            parameters[parameter_name] = []
            for p in parameter:
                parameters[parameter_name].append(p.to_plugin_parameter())
        elif isinstance(parameter, ToolSelector):
            parameters[parameter_name] = parameter.to_plugin_parameter()
        elif isinstance(parameter, list) and all(isinstance(p, ToolSelector) for p in parameter):
            parameters[parameter_name] = []
            for p in parameter:
                parameters[parameter_name].append(p.to_plugin_parameter())
    return parameters

```

### api/core/plugin/utils/chunk_merger.py
```py
from collections.abc import Generator
from dataclasses import dataclass, field

from core.agent.entities import AgentInvokeMessage
from core.tools.entities.tool_entities import ToolInvokeMessage


@dataclass
class FileChunk:
    """
    Buffer for accumulating file chunks during streaming.
    """

    total_length: int
    bytes_written: int = field(default=0, init=False)
    data: bytearray = field(init=False)

    def __post_init__(self):
        self.data = bytearray(self.total_length)


def merge_blob_chunks[T: ToolInvokeMessage | AgentInvokeMessage](
    response: Generator[T, None, None],
    max_file_size: int = 30 * 1024 * 1024,
    max_chunk_size: int = 8192,
) -> Generator[T, None, None]:
    """
    Merge streaming blob chunks into complete blob messages.

    This function processes a stream of plugin invoke messages, accumulating
    BLOB_CHUNK messages by their ID until the final chunk is received,
    then yielding a single complete BLOB message.

    Args:
        response: Generator yielding messages that may include blob chunks
        max_file_size: Maximum allowed file size in bytes (default: 30MB)
        max_chunk_size: Maximum allowed chunk size in bytes (default: 8KB)

    Yields:
        Messages from the response stream, with blob chunks merged into complete blobs

    Raises:
        ValueError: If file size exceeds max_file_size or chunk size exceeds max_chunk_size
    """
    files: dict[str, FileChunk] = {}

    for resp in response:
        if resp.type == ToolInvokeMessage.MessageType.BLOB_CHUNK:
            assert isinstance(resp.message, ToolInvokeMessage.BlobChunkMessage)
            # Get blob chunk information
            chunk_id = resp.message.id
            total_length = resp.message.total_length
            blob_data = resp.message.blob
            is_end = resp.message.end

            # Initialize buffer for this file if it doesn't exist
            if chunk_id not in files:
                files[chunk_id] = FileChunk(total_length)

            # Check if file is too large (before appending)
            if files[chunk_id].bytes_written + len(blob_data) > max_file_size:
                # Delete the file if it's too large
                del files[chunk_id]
                raise ValueError(f"File is too large which reached the limit of {max_file_size / 1024 / 1024}MB")

            # Check if single chunk is too large
            if len(blob_data) > max_chunk_size:
                raise ValueError(f"File chunk is too large which reached the limit of {max_chunk_size / 1024}KB")

            # Append the blob data to the buffer
            files[chunk_id].data[files[chunk_id].bytes_written : files[chunk_id].bytes_written + len(blob_data)] = (
                blob_data
            )
            files[chunk_id].bytes_written += len(blob_data)

            # If this is the final chunk, yield a complete blob message
            if is_end:
                # Create the appropriate message type based on the response type
                message_class = type(resp)
                merged_message = message_class(
                    type=ToolInvokeMessage.MessageType.BLOB,
                    message=ToolInvokeMessage.BlobMessage(
                        blob=bytes(files[chunk_id].data[: files[chunk_id].bytes_written])
                    ),
                    meta=resp.meta,
                )
                assert isinstance(merged_message, (ToolInvokeMessage, AgentInvokeMessage))
                yield merged_message  # type: ignore
                # Clean up the buffer
                del files[chunk_id]
        else:
            yield resp

```

### api/core/plugin/utils/http_parser.py
```py
from io import BytesIO

from flask import Request, Response
from werkzeug.datastructures import Headers


def serialize_request(request: Request) -> bytes:
    method = request.method
    path = request.full_path.rstrip("?")
    raw = f"{method} {path} HTTP/1.1\r\n".encode()

    for name, value in request.headers.items():
        raw += f"{name}: {value}\r\n".encode()

    raw += b"\r\n"

    body = request.get_data(as_text=False)
    if body:
        raw += body

    return raw


def deserialize_request(raw_data: bytes) -> Request:
    header_end = raw_data.find(b"\r\n\r\n")
    if header_end == -1:
        header_end = raw_data.find(b"\n\n")
        if header_end == -1:
            header_data = raw_data
            body = b""
        else:
            header_data = raw_data[:header_end]
            body = raw_data[header_end + 2 :]
    else:
        header_data = raw_data[:header_end]
        body = raw_data[header_end + 4 :]

    lines = header_data.split(b"\r\n")
    if len(lines) == 1 and b"\n" in lines[0]:
        lines = header_data.split(b"\n")

    if not lines or not lines[0]:
        raise ValueError("Empty HTTP request")

    request_line = lines[0].decode("utf-8", errors="ignore")
    parts = request_line.split(" ", 2)
    if len(parts) < 2:
        raise ValueError(f"Invalid request line: {request_line}")

    method = parts[0]
    full_path = parts[1]
    protocol = parts[2] if len(parts) > 2 else "HTTP/1.1"

    if "?" in full_path:
        path, query_string = full_path.split("?", 1)
    else:
        path = full_path
        query_string = ""

    headers = Headers()
    for line in lines[1:]:
        if not line:
            continue
        line_str = line.decode("utf-8", errors="ignore")
        if ":" not in line_str:
            continue
        name, value = line_str.split(":", 1)
        headers.add(name, value.strip())

    host = headers.get("Host", "localhost")
    if ":" in host:
        server_name, server_port = host.rsplit(":", 1)
    else:
        server_name = host
        server_port = "80"

    environ = {
        "REQUEST_METHOD": method,
        "PATH_INFO": path,
        "QUERY_STRING": query_string,
        "SERVER_NAME": server_name,
        "SERVER_PORT": server_port,
        "SERVER_PROTOCOL": protocol,
        "wsgi.input": BytesIO(body),
        "wsgi.url_scheme": "http",
    }

    if "Content-Type" in headers:
        content_type = headers.get("Content-Type")
        if content_type is not None:
            environ["CONTENT_TYPE"] = content_type

    if "Content-Length" in headers:
        content_length = headers.get("Content-Length")
        if content_length is not None:
            environ["CONTENT_LENGTH"] = content_length
    elif body:
        environ["CONTENT_LENGTH"] = str(len(body))

    for name, value in headers.items():
        if name.upper() in ("CONTENT-TYPE", "CONTENT-LENGTH"):
            continue
        env_name = f"HTTP_{name.upper().replace('-', '_')}"
        environ[env_name] = value

    return Request(environ)


def serialize_response(response: Response) -> bytes:
    raw = f"HTTP/1.1 {response.status}\r\n".encode()

    for name, value in response.headers.items():
        raw += f"{name}: {value}\r\n".encode()

    raw += b"\r\n"

    body = response.get_data(as_text=False)
    if body:
        raw += body

    return raw


def deserialize_response(raw_data: bytes) -> Response:
    header_end = raw_data.find(b"\r\n\r\n")
    if header_end == -1:
        header_end = raw_data.find(b"\n\n")
        if header_end == -1:
            header_data = raw_data
            body = b""
        else:
            header_data = raw_data[:header_end]
            body = raw_data[header_end + 2 :]
    else:
        header_data = raw_data[:header_end]
        body = raw_data[header_end + 4 :]

    lines = header_data.split(b"\r\n")
    if len(lines) == 1 and b"\n" in lines[0]:
        lines = header_data.split(b"\n")

    if not lines or not lines[0]:
        raise ValueError("Empty HTTP response")

    status_line = lines[0].decode("utf-8", errors="ignore")
    parts = status_line.split(" ", 2)
    if len(parts) < 2:
        raise ValueError(f"Invalid status line: {status_line}")

    status_code = int(parts[1])

    response = Response(response=body, status=status_code)

    for line in lines[1:]:
        if not line:
            continue
        line_str = line.decode("utf-8", errors="ignore")
        if ":" not in line_str:
            continue
        name, value = line_str.split(":", 1)
        response.headers[name] = value.strip()

    return response

```

### api/core/plugin/endpoint/exc.py
```py
class EndpointSetupFailedError(ValueError):
    """
    Endpoint setup failed error
    """

    pass

```

### api/core/plugin/backwards_invocation/encrypt.py
```py
from core.helper.provider_cache import SingletonProviderCredentialsCache
from core.plugin.entities.request import RequestInvokeEncrypt
from core.tools.utils.encryption import create_provider_encrypter
from models.account import Tenant


class PluginEncrypter:
    @classmethod
    def invoke_encrypt(cls, tenant: Tenant, payload: RequestInvokeEncrypt):
        encrypter, cache = create_provider_encrypter(
            tenant_id=tenant.id,
            config=payload.config,
            cache=SingletonProviderCredentialsCache(
                tenant_id=tenant.id,
                provider_type=payload.namespace,
                provider_identity=payload.identity,
            ),
        )

        if payload.opt == "encrypt":
            return {
                "data": encrypter.encrypt(payload.data),
            }
        elif payload.opt == "decrypt":
            return {
                "data": encrypter.decrypt(payload.data),
            }
        elif payload.opt == "clear":
            cache.delete()
            return {
                "data": {},
            }
        else:
            raise ValueError(f"Invalid opt: {payload.opt}")

```

### api/core/plugin/backwards_invocation/model.py
```py
import tempfile
from binascii import hexlify, unhexlify
from collections.abc import Generator

from graphon.model_runtime.entities.llm_entities import (
    LLMResult,
    LLMResultChunk,
    LLMResultChunkDelta,
    LLMResultChunkWithStructuredOutput,
    LLMResultWithStructuredOutput,
)
from graphon.model_runtime.entities.message_entities import (
    PromptMessage,
    SystemPromptMessage,
    UserPromptMessage,
)
from graphon.model_runtime.entities.model_entities import ModelType

from core.app.llm import deduct_llm_quota
from core.llm_generator.output_parser.structured_output import invoke_llm_with_structured_output
from core.model_manager import ModelManager
from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from core.plugin.entities.request import (
    RequestInvokeLLM,
    RequestInvokeLLMWithStructuredOutput,
    RequestInvokeModeration,
    RequestInvokeRerank,
    RequestInvokeSpeech2Text,
    RequestInvokeSummary,
    RequestInvokeTextEmbedding,
    RequestInvokeTTS,
)
from core.tools.entities.tool_entities import ToolProviderType
from core.tools.utils.model_invocation_utils import ModelInvocationUtils
from models.account import Tenant


class PluginModelBackwardsInvocation(BaseBackwardsInvocation):
    @staticmethod
    def _get_bound_model_instance(
        *,
        tenant_id: str,
        user_id: str | None,
        provider: str,
        model_type: ModelType,
        model: str,
    ):
        return ModelManager.for_tenant(tenant_id=tenant_id, user_id=user_id).get_model_instance(
            tenant_id=tenant_id,
            provider=provider,
            model_type=model_type,
            model=model,
        )

    @classmethod
    def invoke_llm(
        cls, user_id: str, tenant: Tenant, payload: RequestInvokeLLM
    ) -> Generator[LLMResultChunk, None, None] | LLMResult:
        """
        invoke llm
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        response = model_instance.invoke_llm(
            prompt_messages=payload.prompt_messages,
            model_parameters=payload.completion_params,
            tools=payload.tools,
            stop=payload.stop,
            stream=True if payload.stream is None else payload.stream,
        )

        if isinstance(response, Generator):

            def handle() -> Generator[LLMResultChunk, None, None]:
                for chunk in response:
                    if chunk.delta.usage:
                        deduct_llm_quota(tenant_id=tenant.id, model_instance=model_instance, usage=chunk.delta.usage)
                    chunk.prompt_messages = []
                    yield chunk

            return handle()
        else:
            if response.usage:
                deduct_llm_quota(tenant_id=tenant.id, model_instance=model_instance, usage=response.usage)

            def handle_non_streaming(response: LLMResult) -> Generator[LLMResultChunk, None, None]:
                yield LLMResultChunk(
                    model=response.model,
                    prompt_messages=[],
                    system_fingerprint=response.system_fingerprint,
                    delta=LLMResultChunkDelta(
                        index=0,
                        message=response.message,
                        usage=response.usage,
                        finish_reason="",
                    ),
                )

            return handle_non_streaming(response)

    @classmethod
    def invoke_llm_with_structured_output(
        cls, user_id: str, tenant: Tenant, payload: RequestInvokeLLMWithStructuredOutput
    ):
        """
        invoke llm with structured output
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        model_schema = model_instance.model_type_instance.get_model_schema(payload.model, model_instance.credentials)

        if not model_schema:
            raise ValueError(f"Model schema not found for {payload.model}")

        response = invoke_llm_with_structured_output(
            provider=payload.provider,
            model_schema=model_schema,
            model_instance=model_instance,
            prompt_messages=payload.prompt_messages,
            json_schema=payload.structured_output_schema,
            tools=payload.tools,
            stop=payload.stop,
            stream=True if payload.stream is None else payload.stream,
            model_parameters=payload.completion_params,
        )

        if isinstance(response, Generator):

            def handle() -> Generator[LLMResultChunkWithStructuredOutput, None, None]:
                for chunk in response:
                    if chunk.delta.usage:
                        deduct_llm_quota(tenant_id=tenant.id, model_instance=model_instance, usage=chunk.delta.usage)
                    chunk.prompt_messages = []
                    yield chunk

            return handle()
        else:
            if response.usage:
                deduct_llm_quota(tenant_id=tenant.id, model_instance=model_instance, usage=response.usage)

            def handle_non_streaming(
                response: LLMResultWithStructuredOutput,
            ) -> Generator[LLMResultChunkWithStructuredOutput, None, None]:
                yield LLMResultChunkWithStructuredOutput(
                    model=response.model,
                    prompt_messages=[],
                    system_fingerprint=response.system_fingerprint,
                    structured_output=response.structured_output,
                    delta=LLMResultChunkDelta(
                        index=0,
                        message=response.message,
                        usage=response.usage,
                        finish_reason="",
                    ),
                )

            return handle_non_streaming(response)

    @classmethod
    def invoke_text_embedding(cls, user_id: str, tenant: Tenant, payload: RequestInvokeTextEmbedding):
        """
        invoke text embedding
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        response = model_instance.invoke_text_embedding(texts=payload.texts)

        return response

    @classmethod
    def invoke_rerank(cls, user_id: str, tenant: Tenant, payload: RequestInvokeRerank):
        """
        invoke rerank
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        response = model_instance.invoke_rerank(
            query=payload.query,
            docs=payload.docs,
            score_threshold=payload.score_threshold,
            top_n=payload.top_n,
        )

        return response

    @classmethod
    def invoke_tts(cls, user_id: str, tenant: Tenant, payload: RequestInvokeTTS):
        """
        invoke tts
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        response = model_instance.invoke_tts(content_text=payload.content_text, voice=payload.voice)

        def handle() -> Generator[dict, None, None]:
            for chunk in response:
                yield {"result": hexlify(chunk).decode("utf-8")}

        return handle()

    @classmethod
    def invoke_speech2text(cls, user_id: str, tenant: Tenant, payload: RequestInvokeSpeech2Text):
        """
        invoke speech2text
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        with tempfile.NamedTemporaryFile(suffix=".mp3", mode="wb", delete=True) as temp:
            temp.write(unhexlify(payload.file))
            temp.flush()
            temp.seek(0)

            response = model_instance.invoke_speech2text(file=temp)

            return {
                "result": response,
            }

    @classmethod
    def invoke_moderation(cls, user_id: str, tenant: Tenant, payload: RequestInvokeModeration):
        """
        invoke moderation
        """
        model_instance = cls._get_bound_model_instance(
            tenant_id=tenant.id,
            user_id=user_id,
            provider=payload.provider,
            model_type=payload.model_type,
            model=payload.model,
        )

        # invoke model
        response = model_instance.invoke_moderation(text=payload.text)

        return {
            "result": response,
        }

    @classmethod
    def get_system_model_max_tokens(cls, tenant_id: str, user_id: str | None = None) -> int:
        """
        get system model max tokens
        """
        return ModelInvocationUtils.get_max_llm_context_tokens(tenant_id=tenant_id, user_id=user_id)

    @classmethod
    def get_prompt_tokens(cls, tenant_id: str, prompt_messages: list[PromptMessage], user_id: str | None = None) -> int:
        """
        get prompt tokens
        """
        return ModelInvocationUtils.calculate_tokens(
            tenant_id=tenant_id,
            prompt_messages=prompt_messages,
            user_id=user_id,
        )

    @classmethod
    def invoke_system_model(
        cls,
        user_id: str,
        tenant: Tenant,
        prompt_messages: list[PromptMessage],
    ) -> LLMResult:
        """
        invoke system model
        """
        return ModelInvocationUtils.invoke(
            user_id=user_id,
            tenant_id=tenant.id,
            tool_type=ToolProviderType.PLUGIN,
            tool_name="plugin",
            prompt_messages=prompt_messages,
            caller_user_id=user_id,
        )

    @classmethod
    def invoke_summary(cls, user_id: str, tenant: Tenant, payload: RequestInvokeSummary):
        """
        invoke summary
        """
        max_tokens = cls.get_system_model_max_tokens(tenant_id=tenant.id, user_id=user_id)
        content = payload.text

        SUMMARY_PROMPT = """You are a professional language researcher, you are interested in the language
and you can quickly aimed at the main point of an webpage and reproduce it in your own words but
retain the original meaning and keep the key points.
however, the text you got is too long, what you got is possible a part of the text.
Please summarize the text you got.

Here is the extra instruction you need to follow:
<extra_instruction>
{payload.instruction}
</extra_instruction>
"""

        if (
            cls.get_prompt_tokens(
                tenant_id=tenant.id,
                prompt_messages=[UserPromptMessage(content=content)],
                user_id=user_id,
            )
            < max_tokens * 0.6
        ):
            return content

        def get_prompt_tokens(content: str) -> int:
            return cls.get_prompt_tokens(
                tenant_id=tenant.id,
                prompt_messages=[
                    SystemPromptMessage(content=SUMMARY_PROMPT.replace("{payload.instruction}", payload.instruction)),
                    UserPromptMessage(content=content),
                ],
                user_id=user_id,
            )

        def summarize(content: str) -> str:
            summary = cls.invoke_system_model(
                user_id=user_id,
                tenant=tenant,
                prompt_messages=[
                    SystemPromptMessage(content=SUMMARY_PROMPT.replace("{payload.instruction}", payload.instruction)),
                    UserPromptMessage(content=content),
                ],
            )

            assert isinstance(summary.message.content, str)
            return summary.message.content

        lines = content.split("\n")
        new_lines: list[str] = []
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
        for line in new_lines:
            if len(messages) == 0:
                messages.append(line)
            else:
                if len(messages[-1]) + len(line) < max_tokens * 0.5:
                    messages[-1] += line
                if get_prompt_tokens(messages[-1] + line) > max_tokens * 0.7:
                    messages.append(line)
                else:
                    messages[-1] += line

        summaries = []
        for i in range(len(messages)):
            message = messages[i]
            summary = summarize(message)
            summaries.append(summary)

        result = "\n".join(summaries)

        if (
            cls.get_prompt_tokens(
                tenant_id=tenant.id,
                prompt_messages=[UserPromptMessage(content=result)],
                user_id=user_id,
            )
            > max_tokens * 0.7
        ):
            return cls.invoke_summary(
                user_id=user_id,
                tenant=tenant,
                payload=RequestInvokeSummary(text=result, instruction=payload.instruction),
            )

        return result

```

### api/core/plugin/backwards_invocation/app.py
```py
import uuid
from collections.abc import Generator, Mapping
from typing import Any, Union, cast

from sqlalchemy import select
from sqlalchemy.orm import Session

from core.app.app_config.common.parameters_mapping import get_parameters_from_feature_dict
from core.app.apps.advanced_chat.app_generator import AdvancedChatAppGenerator
from core.app.apps.agent_chat.app_generator import AgentChatAppGenerator
from core.app.apps.chat.app_generator import ChatAppGenerator
from core.app.apps.completion.app_generator import CompletionAppGenerator
from core.app.apps.workflow.app_generator import WorkflowAppGenerator
from core.app.entities.app_invoke_entities import InvokeFrom
from core.app.layers.pause_state_persist_layer import PauseStateLayerConfig
from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from extensions.ext_database import db
from models import Account
from models.model import App, AppMode, EndUser
from services.end_user_service import EndUserService


class PluginAppBackwardsInvocation(BaseBackwardsInvocation):
    @classmethod
    def fetch_app_info(cls, app_id: str, tenant_id: str) -> Mapping:
        """
        Fetch app info
        """
        app = cls._get_app(app_id, tenant_id)

        """Retrieve app parameters."""
        if app.mode in {AppMode.ADVANCED_CHAT, AppMode.WORKFLOW}:
            workflow = app.workflow
            if workflow is None:
                raise ValueError("unexpected app type")

            features_dict: dict[str, Any] = workflow.features_dict
            user_input_form = workflow.user_input_form(to_old_structure=True)
        else:
            app_model_config = app.app_model_config
            if app_model_config is None:
                raise ValueError("unexpected app type")

            features_dict = cast(dict[str, Any], app_model_config.to_dict())

            user_input_form = features_dict.get("user_input_form", [])

        return {
            "data": get_parameters_from_feature_dict(features_dict=features_dict, user_input_form=user_input_form),
        }

    @classmethod
    def invoke_app(
        cls,
        app_id: str,
        user_id: str,
        tenant_id: str,
        conversation_id: str | None,
        query: str | None,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke app
        """
        app = cls._get_app(app_id, tenant_id)
        if not user_id:
            user = EndUserService.get_or_create_end_user(app)
        else:
            user = cls._get_user(user_id)

        conversation_id = conversation_id or ""

        if app.mode in {AppMode.ADVANCED_CHAT, AppMode.AGENT_CHAT, AppMode.CHAT}:
            if not query:
                raise ValueError("missing query")

            return cls.invoke_chat_app(app, user, conversation_id, query, stream, inputs, files)
        elif app.mode == AppMode.WORKFLOW:
            return cls.invoke_workflow_app(app, user, stream, inputs, files)
        elif app.mode == AppMode.COMPLETION:
            return cls.invoke_completion_app(app, user, stream, inputs, files)

        raise ValueError("unexpected app type")

    @classmethod
    def invoke_chat_app(
        cls,
        app: App,
        user: Account | EndUser,
        conversation_id: str,
        query: str,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke chat app
        """
        if app.mode == AppMode.ADVANCED_CHAT:
            workflow = app.workflow
            if not workflow:
                raise ValueError("unexpected app type")

            pause_config = PauseStateLayerConfig(
                session_factory=db.engine,
                state_owner_user_id=workflow.created_by,
            )

            return AdvancedChatAppGenerator().generate(
                app_model=app,
                workflow=workflow,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                workflow_run_id=str(uuid.uuid4()),
                streaming=stream,
                pause_state_config=pause_config,
            )
        elif app.mode == AppMode.AGENT_CHAT:
            return AgentChatAppGenerator().generate(
                app_model=app,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=stream,
            )
        elif app.mode == AppMode.CHAT:
            return ChatAppGenerator().generate(
                app_model=app,
                user=user,
                args={
                    "inputs": inputs,
                    "query": query,
                    "files": files,
                    "conversation_id": conversation_id,
                },
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=stream,
            )
        else:
            raise ValueError("unexpected app type")

    @classmethod
    def invoke_workflow_app(
        cls,
        app: App,
        user: EndUser | Account,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke workflow app
        """
        workflow = app.workflow
        if not workflow:
            raise ValueError("unexpected app type")

        pause_config = PauseStateLayerConfig(
            session_factory=db.engine,
            state_owner_user_id=workflow.created_by,
        )

        return WorkflowAppGenerator().generate(
            app_model=app,
            workflow=workflow,
            user=user,
            args={"inputs": inputs, "files": files},
            invoke_from=InvokeFrom.SERVICE_API,
            streaming=stream,
            call_depth=1,
            pause_state_config=pause_config,
        )

    @classmethod
    def invoke_completion_app(
        cls,
        app: App,
        user: EndUser | Account,
        stream: bool,
        inputs: Mapping,
        files: list[dict],
    ) -> Generator[Mapping | str, None, None] | Mapping:
        """
        invoke completion app
        """
        return CompletionAppGenerator().generate(
            app_model=app,
            user=user,
            args={"inputs": inputs, "files": files},
            invoke_from=InvokeFrom.SERVICE_API,
            streaming=stream,
        )

    @classmethod
    def _get_user(cls, user_id: str) -> Union[EndUser, Account]:
        """
        get the user by user id
        """
        with Session(db.engine, expire_on_commit=False) as session:
            stmt = select(EndUser).where(EndUser.id == user_id)
            user = session.scalar(stmt)
            if not user:
                stmt = select(Account).where(Account.id == user_id)
                user = session.scalar(stmt)

        if not user:
            raise ValueError("user not found")

        return user

    @classmethod
    def _get_app(cls, app_id: str, tenant_id: str) -> App:
        """
        get app
        """
        try:
            app = db.session.scalar(select(App).where(App.id == app_id, App.tenant_id == tenant_id).limit(1))
        except Exception:
            raise ValueError("app not found")

        if not app:
            raise ValueError("app not found")

        return app

```

### api/core/plugin/backwards_invocation/node.py
```py
from graphon.enums import BuiltinNodeTypes
from graphon.nodes.llm.entities import ModelConfig as LLMModelConfig
from graphon.nodes.parameter_extractor.entities import (
    ParameterConfig,
    ParameterExtractorNodeData,
)
from graphon.nodes.question_classifier.entities import (
    ClassConfig,
    QuestionClassifierNodeData,
)

from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from services.workflow_service import WorkflowService


class PluginNodeBackwardsInvocation(BaseBackwardsInvocation):
    @classmethod
    def invoke_parameter_extractor(
        cls,
        tenant_id: str,
        user_id: str,
        parameters: list[ParameterConfig],
        model_config: LLMModelConfig,
        instruction: str,
        query: str,
    ):
        """
        Invoke parameter extractor node.

        :param tenant_id: str
        :param user_id: str
        :param parameters: list[ParameterConfig]
        :param model_config: ModelConfig
        :param instruction: str
        :param query: str
        :return: dict
        """
        # FIXME(-LAN-): Avoid import service into core
        workflow_service = WorkflowService()
        node_id = "1919810"
        node_data = ParameterExtractorNodeData(
            title="parameter_extractor",
            desc="parameter_extractor",
            parameters=parameters,
            reasoning_mode="function_call",
            query=[node_id, "query"],
            model=model_config,
            instruction=instruction,  # instruct with variables are not supported
        )
        node_data_dict = node_data.model_dump()
        node_data_dict["type"] = BuiltinNodeTypes.PARAMETER_EXTRACTOR
        execution = workflow_service.run_free_workflow_node(
            node_data_dict,
            tenant_id=tenant_id,
            user_id=user_id,
            node_id=node_id,
            user_inputs={
                f"{node_id}.query": query,
            },
        )

        return {
            "inputs": execution.inputs,
            "outputs": execution.outputs,
            "process_data": execution.process_data,
        }

    @classmethod
    def invoke_question_classifier(
        cls,
        tenant_id: str,
        user_id: str,
        model_config: LLMModelConfig,
        classes: list[ClassConfig],
        instruction: str,
        query: str,
    ):
        """
        Invoke question classifier node.

        :param tenant_id: str
        :param user_id: str
        :param model_config: ModelConfig
        :param classes: list[ClassConfig]
        :param instruction: str
        :param query: str
        :return: dict
        """
        # FIXME(-LAN-): Avoid import service into core
        workflow_service = WorkflowService()
        node_id = "1919810"
        node_data = QuestionClassifierNodeData(
            title="question_classifier",
            desc="question_classifier",
            query_variable_selector=[node_id, "query"],
            model=model_config,
            classes=classes,
            instruction=instruction,  # instruct with variables are not supported
        )
        node_data_dict = node_data.model_dump()
        execution = workflow_service.run_free_workflow_node(
            node_data_dict,
            tenant_id=tenant_id,
            user_id=user_id,
            node_id=node_id,
            user_inputs={
                f"{node_id}.query": query,
            },
        )

        return {
            "inputs": execution.inputs,
            "outputs": execution.outputs,
            "process_data": execution.process_data,
        }

```

### api/core/plugin/backwards_invocation/tool.py
```py
from collections.abc import Generator
from typing import Any

from core.callback_handler.workflow_tool_callback_handler import DifyWorkflowCallbackHandler
from core.plugin.backwards_invocation.base import BaseBackwardsInvocation
from core.tools.entities.tool_entities import ToolInvokeMessage, ToolProviderType
from core.tools.tool_engine import ToolEngine
from core.tools.tool_manager import ToolManager
from core.tools.utils.message_transformer import ToolFileMessageTransformer


class PluginToolBackwardsInvocation(BaseBackwardsInvocation):
    """
    Backwards invocation for plugin tools.
    """

    @classmethod
    def invoke_tool(
        cls,
        tenant_id: str,
        user_id: str,
        tool_type: ToolProviderType,
        provider: str,
        tool_name: str,
        tool_parameters: dict[str, Any],
        credential_id: str | None = None,
    ) -> Generator[ToolInvokeMessage, None, None]:
        """
        invoke tool
        """
        # get tool runtime
        try:
            tool_runtime = ToolManager.get_tool_runtime_from_plugin(
                tool_type,
                tenant_id,
                provider,
                tool_name,
                tool_parameters,
                user_id=user_id,
                credential_id=credential_id,
            )
            response = ToolEngine.generic_invoke(
                tool_runtime, tool_parameters, user_id, DifyWorkflowCallbackHandler(), workflow_call_depth=1
            )

            response = ToolFileMessageTransformer.transform_tool_invoke_messages(
                response, user_id=user_id, tenant_id=tenant_id
            )

            return response
        except Exception as e:
            raise e

```

### api/core/plugin/backwards_invocation/base.py
```py
from collections.abc import Generator, Mapping

from pydantic import BaseModel


class BaseBackwardsInvocation:
    @classmethod
    def convert_to_event_stream(cls, response: Generator[BaseModel | Mapping | str, None, None] | BaseModel | Mapping):
        if isinstance(response, Generator):
            try:
                for chunk in response:
                    if isinstance(chunk, BaseModel | dict):
                        yield BaseBackwardsInvocationResponse(data=chunk).model_dump_json().encode()
            except Exception as e:
                error_message = BaseBackwardsInvocationResponse(error=str(e)).model_dump_json()
                yield error_message.encode()
        else:
            yield BaseBackwardsInvocationResponse(data=response).model_dump_json().encode()


class BaseBackwardsInvocationResponse[T: dict | Mapping | str | bool | int | BaseModel](BaseModel):
    data: T | None = None
    error: str = ""

```

### api/core/plugin/entities/bundle.py
```py
from enum import StrEnum

from pydantic import BaseModel

from core.plugin.entities.plugin import PluginDeclaration, PluginInstallationSource


class PluginBundleDependency(BaseModel):
    class Type(StrEnum):
        Github = PluginInstallationSource.Github.value
        Marketplace = PluginInstallationSource.Marketplace.value
        Package = PluginInstallationSource.Package.value

    class Github(BaseModel):
        repo_address: str
        repo: str
        release: str
        packages: str

    class Marketplace(BaseModel):
        organization: str
        plugin: str
        version: str

    class Package(BaseModel):
        unique_identifier: str
        manifest: PluginDeclaration

    type: Type
    value: Github | Marketplace | Package

```

### api/core/plugin/entities/plugin_daemon.py
```py
from __future__ import annotations

import enum
from collections.abc import Mapping, Sequence
from datetime import datetime
from enum import StrEnum
from typing import Any

from graphon.model_runtime.entities.model_entities import AIModelEntity
from graphon.model_runtime.entities.provider_entities import ProviderEntity
from pydantic import BaseModel, ConfigDict, Field

from core.agent.plugin_entities import AgentProviderEntityWithPlugin
from core.datasource.entities.datasource_entities import DatasourceProviderEntityWithPlugin
from core.plugin.entities.base import BasePluginEntity
from core.plugin.entities.parameters import PluginParameterOption
from core.plugin.entities.plugin import PluginDeclaration, PluginEntity
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import ToolProviderEntityWithPlugin
from core.trigger.entities.entities import TriggerProviderEntity


class PluginDaemonBasicResponse[T: BaseModel | dict | list | bool | str](BaseModel):
    """
    Basic response from plugin daemon.
    """

    code: int
    message: str
    data: T | None = None


class InstallPluginMessage(BaseModel):
    """
    Message for installing a plugin.
    """

    class Event(StrEnum):
        Info = "info"
        Done = "done"
        Error = "error"

    event: Event
    data: str


class PluginToolProviderEntity(BaseModel):
    provider: str
    plugin_unique_identifier: str
    plugin_id: str
    declaration: ToolProviderEntityWithPlugin


class PluginDatasourceProviderEntity(BaseModel):
    provider: str
    plugin_unique_identifier: str
    plugin_id: str
    is_authorized: bool = False
    declaration: DatasourceProviderEntityWithPlugin


class PluginAgentProviderEntity(BaseModel):
    provider: str
    plugin_unique_identifier: str
    plugin_id: str
    declaration: AgentProviderEntityWithPlugin
    meta: PluginDeclaration.Meta


class PluginBasicBooleanResponse(BaseModel):
    """
    Basic boolean response from plugin daemon.
    """

    result: bool
    credentials: dict | None = None


class PluginModelSchemaEntity(BaseModel):
    model_schema: AIModelEntity = Field(description="The model schema.")

    # pydantic configs
    model_config = ConfigDict(protected_namespaces=())


class PluginModelProviderEntity(BaseModel):
    id: str = Field(description="ID")
    created_at: datetime = Field(description="The created at time of the model provider.")
    updated_at: datetime = Field(description="The updated at time of the model provider.")
    provider: str = Field(description="The provider of the model.")
    tenant_id: str = Field(description="The tenant ID.")
    plugin_unique_identifier: str = Field(description="The plugin unique identifier.")
    plugin_id: str = Field(description="The plugin ID.")
    declaration: ProviderEntity = Field(description="The declaration of the model provider.")


class PluginTextEmbeddingNumTokensResponse(BaseModel):
    """
    Response for number of tokens.
    """

    num_tokens: list[int] = Field(description="The number of tokens.")


class PluginLLMNumTokensResponse(BaseModel):
    """
    Response for number of tokens.
    """

    num_tokens: int = Field(description="The number of tokens.")


class PluginStringResultResponse(BaseModel):
    result: str = Field(description="The result of the string.")


class PluginVoiceEntity(BaseModel):
    name: str = Field(description="The name of the voice.")
    value: str = Field(description="The value of the voice.")


class PluginVoicesResponse(BaseModel):
    voices: list[PluginVoiceEntity] = Field(description="The result of the voices.")


class PluginDaemonError(BaseModel):
    """
    Error from plugin daemon.
    """

    error_type: str
    message: str


class PluginDaemonInnerError(Exception):
    code: int
    message: str

    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message


class PluginInstallTaskStatus(StrEnum):
    Pending = "pending"
    Running = "running"
    Success = "success"
    Failed = "failed"


class PluginInstallTaskPluginStatus(BaseModel):
    plugin_unique_identifier: str = Field(description="The plugin unique identifier of the install task.")
    plugin_id: str = Field(description="The plugin ID of the install task.")
    status: PluginInstallTaskStatus = Field(description="The status of the install task.")
    message: str = Field(description="The message of the install task.")
    icon: str = Field(description="The icon of the plugin.")
    labels: I18nObject = Field(description="The labels of the plugin.")
    source: str | None = Field(default=None, description="The installation source of the plugin")


class PluginInstallTask(BasePluginEntity):
    status: PluginInstallTaskStatus = Field(description="The status of the install task.")
    total_plugins: int = Field(description="The total number of plugins to be installed.")
    completed_plugins: int = Field(description="The number of plugins that have been installed.")
    plugins: list[PluginInstallTaskPluginStatus] = Field(description="The status of the plugins.")


class PluginInstallTaskStartResponse(BaseModel):
    all_installed: bool = Field(description="Whether all plugins are installed.")
    task_id: str = Field(description="The ID of the install task.")


class PluginVerification(BaseModel):
    """
    Verification of the plugin.
    """

    class AuthorizedCategory(StrEnum):
        Langgenius = "langgenius"
        Partner = "partner"
        Community = "community"

    authorized_category: AuthorizedCategory = Field(description="The authorized category of the plugin.")


class PluginDecodeResponse(BaseModel):
    unique_identifier: str = Field(description="The unique identifier of the plugin.")
    manifest: PluginDeclaration
    verification: PluginVerification | None = Field(default=None, description="Basic verification information")


class PluginOAuthAuthorizationUrlResponse(BaseModel):
    authorization_url: str = Field(description="The URL of the authorization.")


class PluginOAuthCredentialsResponse(BaseModel):
    metadata: Mapping[str, Any] = Field(
        default_factory=dict, description="The metadata of the OAuth, like avatar url, name, etc."
    )
    expires_at: int = Field(default=-1, description="The expires at time of the credentials. UTC timestamp.")
    credentials: Mapping[str, Any] = Field(description="The credentials of the OAuth.")


class PluginListResponse(BaseModel):
    list: list[PluginEntity]
    total: int


class PluginDynamicSelectOptionsResponse(BaseModel):
    options: Sequence[PluginParameterOption] = Field(description="The options of the dynamic select.")


class PluginTriggerProviderEntity(BaseModel):
    provider: str
    plugin_unique_identifier: str
    plugin_id: str
    declaration: TriggerProviderEntity


class CredentialType(enum.StrEnum):
    API_KEY = "api-key"
    OAUTH2 = "oauth2"
    UNAUTHORIZED = "unauthorized"

    def get_name(self):
        if self == CredentialType.API_KEY:
            return "API KEY"
        elif self == CredentialType.OAUTH2:
            return "AUTH"
        elif self == CredentialType.UNAUTHORIZED:
            return "UNAUTHORIZED"
        else:
            return self.value.replace("-", " ").upper()

    def is_editable(self):
        return self == CredentialType.API_KEY

    def is_validate_allowed(self):
        return self == CredentialType.API_KEY

    @classmethod
    def values(cls):
        return [item.value for item in cls]

    @classmethod
    def of(cls, credential_type: str) -> CredentialType:
        type_name = credential_type.lower()
        if type_name in {"api-key", "api_key"}:
            return cls.API_KEY
        elif type_name in {"oauth2", "oauth"}:
            return cls.OAUTH2
        elif type_name == "unauthorized":
            return cls.UNAUTHORIZED
        else:
            raise ValueError(f"Invalid credential type: {credential_type}")


class PluginReadmeResponse(BaseModel):
    content: str = Field(description="The readme of the plugin.")
    language: str = Field(description="The language of the readme.")

```

### api/core/plugin/entities/request.py
```py
import binascii
import json
from collections.abc import Mapping
from typing import Any, Literal

from flask import Response
from graphon.model_runtime.entities.message_entities import (
    AssistantPromptMessage,
    PromptMessage,
    PromptMessageRole,
    PromptMessageTool,
    SystemPromptMessage,
    ToolPromptMessage,
    UserPromptMessage,
)
from graphon.model_runtime.entities.model_entities import ModelType
from graphon.nodes.llm.entities import ModelConfig as LLMModelConfig
from graphon.nodes.parameter_extractor.entities import (
    ParameterConfig,
)
from graphon.nodes.question_classifier.entities import (
    ClassConfig,
)
from pydantic import BaseModel, ConfigDict, Field, field_validator

from core.entities.provider_entities import BasicProviderConfig
from core.plugin.utils.http_parser import deserialize_response


class InvokeCredentials(BaseModel):
    tool_credentials: dict[str, str] = Field(
        default_factory=dict,
        description="Map of tool provider to credential id, used to store the credential id for the tool provider.",
    )


class PluginInvokeContext(BaseModel):
    credentials: InvokeCredentials | None = Field(
        default_factory=InvokeCredentials,
        description="Credentials context for the plugin invocation or backward invocation.",
    )


class RequestInvokeTool(BaseModel):
    """
    Request to invoke a tool
    """

    tool_type: Literal["builtin", "workflow", "api", "mcp"]
    provider: str
    tool: str
    tool_parameters: dict
    credential_id: str | None = None


class BaseRequestInvokeModel(BaseModel):
    provider: str
    model: str
    model_type: ModelType

    model_config = ConfigDict(protected_namespaces=())


class RequestInvokeLLM(BaseRequestInvokeModel):
    """
    Request to invoke LLM
    """

    model_type: ModelType = ModelType.LLM
    mode: str
    completion_params: dict[str, Any] = Field(default_factory=dict)
    prompt_messages: list[PromptMessage] = Field(default_factory=list)
    tools: list[PromptMessageTool] | None = Field(default_factory=list[PromptMessageTool])
    stop: list[str] | None = Field(default_factory=list[str])
    stream: bool | None = False

    model_config = ConfigDict(protected_namespaces=())

    @field_validator("prompt_messages", mode="before")
    @classmethod
    def convert_prompt_messages(cls, v):
        if not isinstance(v, list):
            raise ValueError("prompt_messages must be a list")

        for i in range(len(v)):
            if v[i]["role"] == PromptMessageRole.USER:
                v[i] = UserPromptMessage.model_validate(v[i])
            elif v[i]["role"] == PromptMessageRole.ASSISTANT:
                v[i] = AssistantPromptMessage.model_validate(v[i])
            elif v[i]["role"] == PromptMessageRole.SYSTEM:
                v[i] = SystemPromptMessage.model_validate(v[i])
            elif v[i]["role"] == PromptMessageRole.TOOL:
                v[i] = ToolPromptMessage.model_validate(v[i])
            else:
                v[i] = PromptMessage.model_validate(v[i])

        return v


class RequestInvokeLLMWithStructuredOutput(RequestInvokeLLM):
    """
    Request to invoke LLM with structured output
    """

    structured_output_schema: dict[str, Any] = Field(
        default_factory=dict, description="The schema of the structured output in JSON schema format"
    )


class RequestInvokeTextEmbedding(BaseRequestInvokeModel):
    """
    Request to invoke text embedding
    """

    model_type: ModelType = ModelType.TEXT_EMBEDDING
    texts: list[str]


class RequestInvokeRerank(BaseRequestInvokeModel):
    """
    Request to invoke rerank
    """

    model_type: ModelType = ModelType.RERANK
    query: str
    docs: list[str]
    score_threshold: float
    top_n: int


class RequestInvokeTTS(BaseRequestInvokeModel):
    """
    Request to invoke TTS
    """

    model_type: ModelType = ModelType.TTS
    content_text: str
    voice: str


class RequestInvokeSpeech2Text(BaseRequestInvokeModel):
    """
    Request to invoke speech2text
    """

    model_type: ModelType = ModelType.SPEECH2TEXT
    file: bytes

    @field_validator("file", mode="before")
    @classmethod
    def convert_file(cls, v):
        # hex string to bytes
        if isinstance(v, str):
            return bytes.fromhex(v)
        else:
            raise ValueError("file must be a hex string")


class RequestInvokeModeration(BaseRequestInvokeModel):
    """
    Request to invoke moderation
    """

    model_type: ModelType = ModelType.MODERATION
    text: str


class RequestInvokeParameterExtractorNode(BaseModel):
    """
    Request to invoke parameter extractor node
    """

    parameters: list[ParameterConfig]
    model: LLMModelConfig
    instruction: str
    query: str


class RequestInvokeQuestionClassifierNode(BaseModel):
    """
    Request to invoke question classifier node
    """

    query: str
    model: LLMModelConfig
    classes: list[ClassConfig]
    instruction: str


class RequestInvokeApp(BaseModel):
    """
    Request to invoke app
    """

    app_id: str
    inputs: dict[str, Any]
    query: str | None = None
    response_mode: Literal["blocking", "streaming"]
    conversation_id: str | None = None
    user: str | None = None
    files: list[dict] = Field(default_factory=list)


class RequestInvokeEncrypt(BaseModel):
    """
    Request to encryption
    """

    opt: Literal["encrypt", "decrypt", "clear"]
    namespace: Literal["endpoint"]
    identity: str
    data: dict = Field(default_factory=dict)
    config: list[BasicProviderConfig] = Field(default_factory=list)


class RequestInvokeSummary(BaseModel):
    """
    Request to summary
    """

    text: str
    instruction: str


class RequestRequestUploadFile(BaseModel):
    """
    Request to upload file
    """

    filename: str
    mimetype: str


class RequestFetchAppInfo(BaseModel):
    """
    Request to fetch app info
    """

    app_id: str


class TriggerInvokeEventResponse(BaseModel):
    variables: Mapping[str, Any] = Field(default_factory=dict)
    cancelled: bool = Field(default=False)

    model_config = ConfigDict(protected_namespaces=(), arbitrary_types_allowed=True)

    @field_validator("variables", mode="before")
    @classmethod
    def convert_variables(cls, v):
        if isinstance(v, str):
            return json.loads(v)
        else:
            return v


class TriggerSubscriptionResponse(BaseModel):
    subscription: dict[str, Any]


class TriggerValidateProviderCredentialsResponse(BaseModel):
    result: bool


class TriggerDispatchResponse(BaseModel):
    user_id: str
    events: list[str]
    response: Response
    payload: Mapping[str, Any] = Field(default_factory=dict)

    model_config = ConfigDict(protected_namespaces=(), arbitrary_types_allowed=True)

    @field_validator("response", mode="before")
    @classmethod
    def convert_response(cls, v: str):
        try:
            return deserialize_response(binascii.unhexlify(v.encode()))
        except Exception as e:
            raise ValueError("Failed to deserialize response from hex string") from e

```

### api/core/plugin/entities/endpoint.py
```py
from datetime import datetime

from pydantic import BaseModel, Field, model_validator

from configs import dify_config
from core.entities.provider_entities import ProviderConfig
from core.plugin.entities.base import BasePluginEntity


class EndpointDeclaration(BaseModel):
    """
    declaration of an endpoint
    """

    path: str
    method: str
    hidden: bool = Field(default=False)


class EndpointProviderDeclaration(BaseModel):
    """
    declaration of an endpoint group
    """

    settings: list[ProviderConfig] = Field(default_factory=list)
    endpoints: list[EndpointDeclaration] | None = Field(default_factory=list[EndpointDeclaration])


class EndpointEntity(BasePluginEntity):
    """
    entity of an endpoint
    """

    settings: dict
    tenant_id: str
    plugin_id: str
    expired_at: datetime
    declaration: EndpointProviderDeclaration = Field(default_factory=EndpointProviderDeclaration)


class EndpointEntityWithInstance(EndpointEntity):
    name: str
    enabled: bool
    url: str
    hook_id: str

    @model_validator(mode="before")
    @classmethod
    def render_url_template(cls, values):
        if "url" not in values:
            url_template = dify_config.ENDPOINT_URL_TEMPLATE
            values["url"] = url_template.replace("{hook_id}", values["hook_id"])
        return values

```

### api/core/plugin/entities/marketplace.py
```py
from graphon.model_runtime.entities.provider_entities import ProviderEntity
from pydantic import BaseModel, Field, computed_field, model_validator

from core.plugin.entities.endpoint import EndpointProviderDeclaration
from core.plugin.entities.plugin import PluginResourceRequirements
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import ToolProviderEntity


class MarketplacePluginDeclaration(BaseModel):
    name: str = Field(..., description="Unique identifier for the plugin within the marketplace")
    org: str = Field(..., description="Organization or developer responsible for creating and maintaining the plugin")
    plugin_id: str = Field(..., description="Globally unique identifier for the plugin across all marketplaces")
    icon: str = Field(..., description="URL or path to the plugin's visual representation")
    label: I18nObject = Field(..., description="Localized display name for the plugin in different languages")
    brief: I18nObject = Field(..., description="Short, localized description of the plugin's functionality")
    resource: PluginResourceRequirements = Field(
        ..., description="Specification of computational resources needed to run the plugin"
    )
    endpoint: EndpointProviderDeclaration | None = Field(
        None, description="Configuration for the plugin's API endpoint, if applicable"
    )
    model: ProviderEntity | None = Field(None, description="Details of the AI model used by the plugin, if any")
    tool: ToolProviderEntity | None = Field(
        None, description="Information about the tool functionality provided by the plugin, if any"
    )
    latest_version: str = Field(
        ..., description="Most recent version number of the plugin available in the marketplace"
    )
    latest_package_identifier: str = Field(
        ..., description="Unique identifier for the latest package release of the plugin"
    )
    status: str = Field(..., description="Indicate the status of marketplace plugin, enum from `active` `deleted`")
    deprecated_reason: str = Field(
        ..., description="Not empty when status='deleted', indicates the reason why this plugin is deleted(deprecated)"
    )
    alternative_plugin_id: str = Field(
        ..., description="Optional, indicates the alternative plugin for user to switch to"
    )

    @model_validator(mode="before")
    @classmethod
    def transform_declaration(cls, data: dict):
        if "endpoint" in data and not data["endpoint"]:
            del data["endpoint"]
        if "model" in data and not data["model"]:
            del data["model"]
        if "tool" in data and not data["tool"]:
            del data["tool"]
        return data


class MarketplacePluginSnapshot(BaseModel):
    org: str
    name: str
    latest_version: str
    latest_package_identifier: str
    latest_package_url: str

    @computed_field
    def plugin_id(self) -> str:
        return f"{self.org}/{self.name}"

```

### api/core/plugin/entities/parameters.py
```py
import json
from enum import StrEnum, auto
from typing import Any, Union

from pydantic import BaseModel, Field, field_validator

from core.entities.parameter_entities import CommonParameterType
from core.tools.entities.common_entities import I18nObject


class PluginParameterOption(BaseModel):
    value: str = Field(..., description="The value of the option")
    label: I18nObject = Field(..., description="The label of the option")
    icon: str | None = Field(default=None, description="The icon of the option, can be a url or a base64 encoded image")

    @field_validator("value", mode="before")
    @classmethod
    def transform_id_to_str(cls, value) -> str:
        if not isinstance(value, str):
            return str(value)
        else:
            return value


class PluginParameterType(StrEnum):
    """
    all available parameter types
    """

    STRING = CommonParameterType.STRING
    NUMBER = CommonParameterType.NUMBER
    BOOLEAN = CommonParameterType.BOOLEAN
    SELECT = CommonParameterType.SELECT
    SECRET_INPUT = CommonParameterType.SECRET_INPUT
    FILE = CommonParameterType.FILE
    FILES = CommonParameterType.FILES
    APP_SELECTOR = CommonParameterType.APP_SELECTOR
    MODEL_SELECTOR = CommonParameterType.MODEL_SELECTOR
    TOOLS_SELECTOR = CommonParameterType.TOOLS_SELECTOR
    ANY = CommonParameterType.ANY
    DYNAMIC_SELECT = CommonParameterType.DYNAMIC_SELECT
    CHECKBOX = CommonParameterType.CHECKBOX
    # deprecated, should not use.
    SYSTEM_FILES = CommonParameterType.SYSTEM_FILES

    # MCP object and array type parameters
    ARRAY = CommonParameterType.ARRAY
    OBJECT = CommonParameterType.OBJECT


class MCPServerParameterType(StrEnum):
    """
    MCP server got complex parameter types
    """

    ARRAY = auto()
    OBJECT = auto()


class PluginParameterAutoGenerate(BaseModel):
    class Type(StrEnum):
        PROMPT_INSTRUCTION = auto()

    type: Type


class PluginParameterTemplate(BaseModel):
    enabled: bool = Field(default=False, description="Whether the parameter is jinja enabled")


class PluginParameter(BaseModel):
    name: str = Field(..., description="The name of the parameter")
    label: I18nObject = Field(..., description="The label presented to the user")
    placeholder: I18nObject | None = Field(default=None, description="The placeholder presented to the user")
    scope: str | None = None
    auto_generate: PluginParameterAutoGenerate | None = None
    template: PluginParameterTemplate | None = None
    required: bool = False
    default: Union[float, int, str, bool, list, dict] | None = None
    min: Union[float, int] | None = None
    max: Union[float, int] | None = None
    precision: int | None = None
    options: list[PluginParameterOption] = Field(default_factory=list)

    @field_validator("options", mode="before")
    @classmethod
    def transform_options(cls, v):
        if not isinstance(v, list):
            return []
        return v


def as_normal_type(typ: StrEnum):
    if typ.value in {
        PluginParameterType.SECRET_INPUT,
        PluginParameterType.SELECT,
        PluginParameterType.CHECKBOX,
    }:
        return "string"
    return typ.value


def cast_parameter_value(typ: StrEnum, value: Any, /):
    try:
        match typ.value:
            case (
                PluginParameterType.STRING
                | PluginParameterType.SECRET_INPUT
                | PluginParameterType.SELECT
                | PluginParameterType.CHECKBOX
                | PluginParameterType.DYNAMIC_SELECT
            ):
                if value is None:
                    return ""
                else:
                    return value if isinstance(value, str) else str(value)

            case PluginParameterType.BOOLEAN:
                if value is None:
                    return False
                elif isinstance(value, str):
                    # Allowed YAML boolean value strings: https://yaml.org/type/bool.html
                    # and also '0' for False and '1' for True
                    match value.lower():
                        case "true" | "yes" | "y" | "1":
                            return True
                        case "false" | "no" | "n" | "0":
                            return False
                        case _:
                            return bool(value)
                else:
                    return value if isinstance(value, bool) else bool(value)

            case PluginParameterType.NUMBER:
                if isinstance(value, int | float):
                    return value
                elif isinstance(value, str) and value:
                    if "." in value:
                        return float(value)
                    else:
                        return int(value)
            case PluginParameterType.SYSTEM_FILES | PluginParameterType.FILES:
                if not isinstance(value, list):
                    return [value]
                return value
            case PluginParameterType.FILE:
                if isinstance(value, list):
                    if len(value) != 1:
                        raise ValueError("This parameter only accepts one file but got multiple files while invoking.")
                    else:
                        return value[0]
                return value
            case PluginParameterType.MODEL_SELECTOR | PluginParameterType.APP_SELECTOR:
                if not isinstance(value, dict):
                    raise ValueError("The selector must be a dictionary.")
                return value
            case PluginParameterType.TOOLS_SELECTOR:
                if value and not isinstance(value, list):
                    raise ValueError("The tools selector must be a list.")
                return value
            case PluginParameterType.ANY:
                if value and not isinstance(value, str | dict | list | int | float):
                    raise ValueError("The var selector must be a string, dictionary, list or number.")
                return value
            case PluginParameterType.ARRAY:
                if not isinstance(value, list):
                    # Try to parse JSON string for arrays
                    if isinstance(value, str):
                        try:
                            parsed_value = json.loads(value)
                            if isinstance(parsed_value, list):
                                return parsed_value
                        except (json.JSONDecodeError, ValueError):
                            pass
                    return [value]
                return value
            case PluginParameterType.OBJECT:
                if not isinstance(value, dict):
                    # Try to parse JSON string for objects
                    if isinstance(value, str):
                        try:
                            parsed_value = json.loads(value)
                            if isinstance(parsed_value, dict):
                                return parsed_value
                        except (json.JSONDecodeError, ValueError):
                            pass
                    return {}
                return value
            case _:
                return str(value)
    except ValueError:
        raise
    except Exception:
        raise ValueError(f"The tool parameter value {repr(value)} is not in correct type of {as_normal_type(typ)}.")


def init_frontend_parameter(rule: PluginParameter, type: StrEnum, value: Any):
    """
    init frontend parameter by rule
    """
    parameter_value = value
    if not parameter_value and parameter_value != 0:
        # get default value
        parameter_value = rule.default
        if not parameter_value and rule.required:
            raise ValueError(f"tool parameter {rule.name} not found in tool config")

    if type == PluginParameterType.SELECT:
        # check if tool_parameter_config in options
        options = [x.value for x in rule.options]
        if parameter_value is not None and parameter_value not in options:
            raise ValueError(f"tool parameter {rule.name} value {parameter_value} not in options {options}")

    return cast_parameter_value(type, parameter_value)

```

### api/core/plugin/entities/plugin.py
```py
import datetime
from collections.abc import Mapping
from enum import StrEnum, auto
from typing import Any

from graphon.model_runtime.entities.provider_entities import ProviderEntity
from packaging.version import InvalidVersion, Version
from pydantic import BaseModel, Field, field_validator, model_validator

from core.agent.plugin_entities import AgentStrategyProviderEntity
from core.datasource.entities.datasource_entities import DatasourceProviderEntity
from core.plugin.entities.base import BasePluginEntity
from core.plugin.entities.endpoint import EndpointProviderDeclaration
from core.tools.entities.common_entities import I18nObject
from core.tools.entities.tool_entities import ToolProviderEntity
from core.trigger.entities.entities import TriggerProviderEntity


class PluginInstallationSource(StrEnum):
    Github = auto()
    Marketplace = auto()
    Package = auto()
    Remote = auto()


class PluginResourceRequirements(BaseModel):
    memory: int

    class Permission(BaseModel):
        class Tool(BaseModel):
            enabled: bool | None = Field(default=False)

        class Model(BaseModel):
            enabled: bool | None = Field(default=False)
            llm: bool | None = Field(default=False)
            text_embedding: bool | None = Field(default=False)
            rerank: bool | None = Field(default=False)
            tts: bool | None = Field(default=False)
            speech2text: bool | None = Field(default=False)
            moderation: bool | None = Field(default=False)

        class Node(BaseModel):
            enabled: bool | None = Field(default=False)

        class Endpoint(BaseModel):
            enabled: bool | None = Field(default=False)

        class Storage(BaseModel):
            enabled: bool | None = Field(default=False)
            size: int = Field(ge=1024, le=1073741824, default=1048576)

        tool: Tool | None = Field(default=None)
        model: Model | None = Field(default=None)
        node: Node | None = Field(default=None)
        endpoint: Endpoint | None = Field(default=None)
        storage: Storage | None = Field(default=None)

    permission: Permission | None = Field(default=None)


class PluginCategory(StrEnum):
    Tool = auto()
    Model = auto()
    Extension = auto()
    AgentStrategy = "agent-strategy"
    Datasource = "datasource"
    Trigger = "trigger"


class PluginDeclaration(BaseModel):
    class Plugins(BaseModel):
        tools: list[str] | None = Field(default_factory=list[str])
        models: list[str] | None = Field(default_factory=list[str])
        endpoints: list[str] | None = Field(default_factory=list[str])
        datasources: list[str] | None = Field(default_factory=list[str])
        triggers: list[str] | None = Field(default_factory=list[str])

    class Meta(BaseModel):
        minimum_dify_version: str | None = Field(default=None)
        version: str | None = Field(default=None)

        @field_validator("minimum_dify_version")
        @classmethod
        def validate_minimum_dify_version(cls, v: str | None) -> str | None:
            if v is None:
                return v
            try:
                Version(v)
                return v
            except InvalidVersion as e:
                raise ValueError(f"Invalid version format: {v}") from e

    version: str = Field(...)
    author: str | None = Field(..., pattern=r"^[a-zA-Z0-9_-]{1,64}$")
    name: str = Field(..., pattern=r"^[a-z0-9_-]{1,128}$")
    description: I18nObject
    icon: str
    icon_dark: str | None = Field(default=None)
    label: I18nObject
    category: PluginCategory
    created_at: datetime.datetime
    resource: PluginResourceRequirements
    plugins: Plugins
    tags: list[str] = Field(default_factory=list)
    repo: str | None = Field(default=None)
    verified: bool = Field(default=False)
    tool: ToolProviderEntity | None = None
    model: ProviderEntity | None = None
    endpoint: EndpointProviderDeclaration | None = None
    agent_strategy: AgentStrategyProviderEntity | None = None
    datasource: DatasourceProviderEntity | None = None
    trigger: TriggerProviderEntity | None = None
    meta: Meta

    @field_validator("version")
    @classmethod
    def validate_version(cls, v: str) -> str:
        try:
            Version(v)
            return v
        except InvalidVersion as e:
            raise ValueError(f"Invalid version format: {v}") from e

    @model_validator(mode="before")
    @classmethod
    def validate_category(cls, values: dict):
        # auto detect category
        if values.get("tool"):
            values["category"] = PluginCategory.Tool
        elif values.get("model"):
            values["category"] = PluginCategory.Model
        elif values.get("datasource"):
            values["category"] = PluginCategory.Datasource
        elif values.get("agent_strategy"):
            values["category"] = PluginCategory.AgentStrategy
        elif values.get("trigger"):
            values["category"] = PluginCategory.Trigger
        else:
            values["category"] = PluginCategory.Extension
        return values


class PluginInstallation(BasePluginEntity):
    tenant_id: str
    endpoints_setups: int
    endpoints_active: int
    runtime_type: str
    source: PluginInstallationSource
    meta: Mapping[str, Any]
    plugin_id: str
    plugin_unique_identifier: str
    version: str
    checksum: str
    declaration: PluginDeclaration


class PluginEntity(PluginInstallation):
    name: str
    installation_id: str
    version: str

    @model_validator(mode="after")
    def set_plugin_id(self):
        if self.declaration.tool:
            self.declaration.tool.plugin_id = self.plugin_id
        return self


class PluginDependency(BaseModel):
    class Type(StrEnum):
        Github = PluginInstallationSource.Github
        Marketplace = PluginInstallationSource.Marketplace
        Package = PluginInstallationSource.Package

    class Github(BaseModel):
        repo: str
        version: str
        package: str
        github_plugin_unique_identifier: str

        @property
        def plugin_unique_identifier(self) -> str:
            return self.github_plugin_unique_identifier

    class Marketplace(BaseModel):
        marketplace_plugin_unique_identifier: str
        version: str | None = None

        @property
        def plugin_unique_identifier(self) -> str:
            return self.marketplace_plugin_unique_identifier

    class Package(BaseModel):
        plugin_unique_identifier: str
        version: str | None = None

    type: Type
    value: Github | Marketplace | Package
    current_identifier: str | None = None


class MissingPluginDependency(BaseModel):
    plugin_unique_identifier: str
    current_identifier: str | None = None

```

### api/core/plugin/entities/base.py
```py
from datetime import datetime

from pydantic import BaseModel


class BasePluginEntity(BaseModel):
    id: str
    created_at: datetime
    updated_at: datetime

```

### api/core/plugin/entities/oauth.py
```py
from collections.abc import Sequence

from pydantic import BaseModel, Field

from core.entities.provider_entities import ProviderConfig


class OAuthSchema(BaseModel):
    """
    OAuth schema
    """

    client_schema: Sequence[ProviderConfig] = Field(
        default_factory=list,
        description="client schema like client_id, client_secret, etc.",
    )

    credentials_schema: Sequence[ProviderConfig] = Field(
        default_factory=list,
        description="credentials schema like access_token, refresh_token, etc.",
    )

```

### api/services/plugin/plugin_service.py
```py
import logging
from collections.abc import Mapping, Sequence
from mimetypes import guess_type

from pydantic import BaseModel
from sqlalchemy import delete, select, update
from sqlalchemy.orm import Session
from yarl import URL

from configs import dify_config
from core.helper import marketplace
from core.helper.download import download_with_size_limit
from core.helper.marketplace import download_plugin_pkg
from core.helper.model_provider_cache import ProviderCredentialsCache, ProviderCredentialsCacheType
from core.plugin.entities.bundle import PluginBundleDependency
from core.plugin.entities.plugin import (
    PluginDeclaration,
    PluginEntity,
    PluginInstallation,
    PluginInstallationSource,
)
from core.plugin.entities.plugin_daemon import (
    PluginDecodeResponse,
    PluginInstallTask,
    PluginListResponse,
    PluginVerification,
)
from core.plugin.impl.asset import PluginAssetManager
from core.plugin.impl.debugging import PluginDebuggingClient
from core.plugin.impl.plugin import PluginInstaller
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from models.provider import Provider, ProviderCredential, TenantPreferredModelProvider
from models.provider_ids import GenericProviderID
from services.enterprise.plugin_manager_service import (
    PluginManagerService,
    PreUninstallPluginRequest,
)
from services.errors.plugin import PluginInstallationForbiddenError
from services.feature_service import FeatureService, PluginInstallationScope

logger = logging.getLogger(__name__)


class PluginService:
    class LatestPluginCache(BaseModel):
        plugin_id: str
        version: str
        unique_identifier: str
        status: str
        deprecated_reason: str
        alternative_plugin_id: str

    REDIS_KEY_PREFIX = "plugin_service:latest_plugin:"
    REDIS_TTL = 60 * 5  # 5 minutes

    @staticmethod
    def fetch_latest_plugin_version(plugin_ids: Sequence[str]) -> Mapping[str, LatestPluginCache | None]:
        """
        Fetch the latest plugin version
        """
        result: dict[str, PluginService.LatestPluginCache | None] = {}

        try:
            cache_not_exists = []

            # Try to get from Redis first
            for plugin_id in plugin_ids:
                cached_data = redis_client.get(f"{PluginService.REDIS_KEY_PREFIX}{plugin_id}")
                if cached_data:
                    result[plugin_id] = PluginService.LatestPluginCache.model_validate_json(cached_data)
                else:
                    cache_not_exists.append(plugin_id)

            if cache_not_exists:
                manifests = {
                    manifest.plugin_id: manifest
                    for manifest in marketplace.batch_fetch_plugin_manifests(cache_not_exists)
                }

                for plugin_id, manifest in manifests.items():
                    latest_plugin = PluginService.LatestPluginCache(
                        plugin_id=plugin_id,
                        version=manifest.latest_version,
                        unique_identifier=manifest.latest_package_identifier,
                        status=manifest.status,
                        deprecated_reason=manifest.deprecated_reason,
                        alternative_plugin_id=manifest.alternative_plugin_id,
                    )

                    # Store in Redis
                    redis_client.setex(
                        f"{PluginService.REDIS_KEY_PREFIX}{plugin_id}",
                        PluginService.REDIS_TTL,
                        latest_plugin.model_dump_json(),
                    )

                    result[plugin_id] = latest_plugin

                    # pop plugin_id from cache_not_exists
                    cache_not_exists.remove(plugin_id)

                for plugin_id in cache_not_exists:
                    result[plugin_id] = None

            return result
        except Exception:
            logger.exception("failed to fetch latest plugin version")
            return result

    @staticmethod
    def _check_marketplace_only_permission():
        """
        Check if the marketplace only permission is enabled
        """
        features = FeatureService.get_system_features()
        if features.plugin_installation_permission.restrict_to_marketplace_only:
            raise PluginInstallationForbiddenError("Plugin installation is restricted to marketplace only")

    @staticmethod
    def _check_plugin_installation_scope(plugin_verification: PluginVerification | None):
        """
        Check the plugin installation scope
        """
        features = FeatureService.get_system_features()

        match features.plugin_installation_permission.plugin_installation_scope:
            case PluginInstallationScope.OFFICIAL_ONLY:
                if (
                    plugin_verification is None
                    or plugin_verification.authorized_category != PluginVerification.AuthorizedCategory.Langgenius
                ):
                    raise PluginInstallationForbiddenError("Plugin installation is restricted to official only")
            case PluginInstallationScope.OFFICIAL_AND_SPECIFIC_PARTNERS:
                if plugin_verification is None or plugin_verification.authorized_category not in [
                    PluginVerification.AuthorizedCategory.Langgenius,
                    PluginVerification.AuthorizedCategory.Partner,
                ]:
                    raise PluginInstallationForbiddenError(
                        "Plugin installation is restricted to official and specific partners"
                    )
            case PluginInstallationScope.NONE:
                raise PluginInstallationForbiddenError("Installing plugins is not allowed")
            case PluginInstallationScope.ALL:
                pass

    @staticmethod
    def get_debugging_key(tenant_id: str) -> str:
        """
        get the debugging key of the tenant
        """
        manager = PluginDebuggingClient()
        return manager.get_debugging_key(tenant_id)

    @staticmethod
    def list_latest_versions(plugin_ids: Sequence[str]) -> Mapping[str, LatestPluginCache | None]:
        """
        List the latest versions of the plugins
        """
        return PluginService.fetch_latest_plugin_version(plugin_ids)

    @staticmethod
    def list(tenant_id: str) -> list[PluginEntity]:
        """
        list all plugins of the tenant
        """
        manager = PluginInstaller()
        plugins = manager.list_plugins(tenant_id)
        return plugins

    @staticmethod
    def list_with_total(tenant_id: str, page: int, page_size: int) -> PluginListResponse:
        """
        list all plugins of the tenant
        """
        manager = PluginInstaller()
        plugins = manager.list_plugins_with_total(tenant_id, page, page_size)
        return plugins

    @staticmethod
    def list_installations_from_ids(tenant_id: str, ids: Sequence[str]) -> Sequence[PluginInstallation]:
        """
        List plugin installations from ids
        """
        manager = PluginInstaller()
        return manager.fetch_plugin_installation_by_ids(tenant_id, ids)

    @classmethod
    def get_plugin_icon_url(cls, tenant_id: str, filename: str) -> str:
        url_prefix = (
            URL(dify_config.CONSOLE_API_URL or "/") / "console" / "api" / "workspaces" / "current" / "plugin" / "icon"
        )
        return str(url_prefix % {"tenant_id": tenant_id, "filename": filename})

    @staticmethod
    def get_asset(tenant_id: str, asset_file: str) -> tuple[bytes, str]:
        """
        get the asset file of the plugin
        """
        manager = PluginAssetManager()
        # guess mime type
        mime_type, _ = guess_type(asset_file)
        return manager.fetch_asset(tenant_id, asset_file), mime_type or "application/octet-stream"

    @staticmethod
    def extract_asset(tenant_id: str, plugin_unique_identifier: str, file_name: str) -> bytes:
        manager = PluginAssetManager()
        return manager.extract_asset(tenant_id, plugin_unique_identifier, file_name)

    @staticmethod
    def check_plugin_unique_identifier(tenant_id: str, plugin_unique_identifier: str) -> bool:
        """
        check if the plugin unique identifier is already installed by other tenant
        """
        manager = PluginInstaller()
        return manager.fetch_plugin_by_identifier(tenant_id, plugin_unique_identifier)

    @staticmethod
    def fetch_plugin_manifest(tenant_id: str, plugin_unique_identifier: str) -> PluginDeclaration:
        """
        Fetch plugin manifest
        """
        manager = PluginInstaller()
        return manager.fetch_plugin_manifest(tenant_id, plugin_unique_identifier)

    @staticmethod
    def is_plugin_verified(tenant_id: str, plugin_unique_identifier: str) -> bool:
        """
        Check if the plugin is verified
        """
        manager = PluginInstaller()
        try:
            return manager.fetch_plugin_manifest(tenant_id, plugin_unique_identifier).verified
        except Exception:
            return False

    @staticmethod
    def fetch_install_tasks(tenant_id: str, page: int, page_size: int) -> Sequence[PluginInstallTask]:
        """
        Fetch plugin installation tasks
        """
        manager = PluginInstaller()
        return manager.fetch_plugin_installation_tasks(tenant_id, page, page_size)

    @staticmethod
    def fetch_install_task(tenant_id: str, task_id: str) -> PluginInstallTask:
        manager = PluginInstaller()
        return manager.fetch_plugin_installation_task(tenant_id, task_id)

    @staticmethod
    def delete_install_task(tenant_id: str, task_id: str) -> bool:
        """
        Delete a plugin installation task
        """
        manager = PluginInstaller()
        return manager.delete_plugin_installation_task(tenant_id, task_id)

    @staticmethod
    def delete_all_install_task_items(
        tenant_id: str,
    ) -> bool:
        """
        Delete all plugin installation task items
        """
        manager = PluginInstaller()
        return manager.delete_all_plugin_installation_task_items(tenant_id)

    @staticmethod
    def delete_install_task_item(tenant_id: str, task_id: str, identifier: str) -> bool:
        """
        Delete a plugin installation task item
        """
        manager = PluginInstaller()
        return manager.delete_plugin_installation_task_item(tenant_id, task_id, identifier)

    @staticmethod
    def upgrade_plugin_with_marketplace(
        tenant_id: str, original_plugin_unique_identifier: str, new_plugin_unique_identifier: str
    ):
        """
        Upgrade plugin with marketplace
        """
        if not dify_config.MARKETPLACE_ENABLED:
            raise ValueError("marketplace is not enabled")

        if original_plugin_unique_identifier == new_plugin_unique_identifier:
            raise ValueError("you should not upgrade plugin with the same plugin")

        # check if plugin pkg is already downloaded
        manager = PluginInstaller()

        features = FeatureService.get_system_features()

        try:
            manager.fetch_plugin_manifest(tenant_id, new_plugin_unique_identifier)
            # already downloaded, skip, and record install event
            marketplace.record_install_plugin_event(new_plugin_unique_identifier)
        except Exception:
            # plugin not installed, download and upload pkg
            pkg = download_plugin_pkg(new_plugin_unique_identifier)
            response = manager.upload_pkg(
                tenant_id,
                pkg,
                verify_signature=features.plugin_installation_permission.restrict_to_marketplace_only,
            )

            # check if the plugin is available to install
            PluginService._check_plugin_installation_scope(response.verification)

        return manager.upgrade_plugin(
            tenant_id,
            original_plugin_unique_identifier,
            new_plugin_unique_identifier,
            PluginInstallationSource.Marketplace,
            {
                "plugin_unique_identifier": new_plugin_unique_identifier,
            },
        )

    @staticmethod
    def upgrade_plugin_with_github(
        tenant_id: str,
        original_plugin_unique_identifier: str,
        new_plugin_unique_identifier: str,
        repo: str,
        version: str,
        package: str,
    ):
        """
        Upgrade plugin with github
        """
        PluginService._check_marketplace_only_permission()
        manager = PluginInstaller()
        return manager.upgrade_plugin(
            tenant_id,
            original_plugin_unique_identifier,
            new_plugin_unique_identifier,
            PluginInstallationSource.Github,
            {
                "repo": repo,
                "version": version,
                "package": package,
            },
        )

    @staticmethod
    def upload_pkg(tenant_id: str, pkg: bytes, verify_signature: bool = False) -> PluginDecodeResponse:
        """
        Upload plugin package files

        returns: plugin_unique_identifier
        """
        PluginService._check_marketplace_only_permission()
        manager = PluginInstaller()
        features = FeatureService.get_system_features()
        response = manager.upload_pkg(
            tenant_id,
            pkg,
            verify_signature=features.plugin_installation_permission.restrict_to_marketplace_only,
        )
        PluginService._check_plugin_installation_scope(response.verification)

        return response

    @staticmethod
    def upload_pkg_from_github(
        tenant_id: str, repo: str, version: str, package: str, verify_signature: bool = False
    ) -> PluginDecodeResponse:
        """
        Install plugin from github release package files,
        returns plugin_unique_identifier
        """
        PluginService._check_marketplace_only_permission()
        pkg = download_with_size_limit(
            f"https://github.com/{repo}/releases/download/{version}/{package}", dify_config.PLUGIN_MAX_PACKAGE_SIZE
        )
        features = FeatureService.get_system_features()

        manager = PluginInstaller()
        response = manager.upload_pkg(
            tenant_id,
            pkg,
            verify_signature=features.plugin_installation_permission.restrict_to_marketplace_only,
        )
        PluginService._check_plugin_installation_scope(response.verification)

        return response

    @staticmethod
    def upload_bundle(
        tenant_id: str, bundle: bytes, verify_signature: bool = False
    ) -> Sequence[PluginBundleDependency]:
        """
        Upload a plugin bundle and return the dependencies.
        """
        manager = PluginInstaller()
        PluginService._check_marketplace_only_permission()
        return manager.upload_bundle(tenant_id, bundle, verify_signature)

    @staticmethod
    def install_from_local_pkg(tenant_id: str, plugin_unique_identifiers: Sequence[str]):
        PluginService._check_marketplace_only_permission()

        manager = PluginInstaller()

        for plugin_unique_identifier in plugin_unique_identifiers:
            resp = manager.decode_plugin_from_identifier(tenant_id, plugin_unique_identifier)
            PluginService._check_plugin_installation_scope(resp.verification)

        return manager.install_from_identifiers(
            tenant_id,
            plugin_unique_identifiers,
            PluginInstallationSource.Package,
            [{}],
        )

    @staticmethod
    def install_from_github(tenant_id: str, plugin_unique_identifier: str, repo: str, version: str, package: str):
        """
        Install plugin from github release package files,
        returns plugin_unique_identifier
        """
        PluginService._check_marketplace_only_permission()

        manager = PluginInstaller()
        plugin_decode_response = manager.decode_plugin_from_identifier(tenant_id, plugin_unique_identifier)
        PluginService._check_plugin_installation_scope(plugin_decode_response.verification)

        return manager.install_from_identifiers(
            tenant_id,
            [plugin_unique_identifier],
            PluginInstallationSource.Github,
            [
                {
                    "repo": repo,
                    "version": version,
                    "package": package,
                }
            ],
        )

    @staticmethod
    def fetch_marketplace_pkg(tenant_id: str, plugin_unique_identifier: str) -> PluginDeclaration:
        """
        Fetch marketplace package
        """
        if not dify_config.MARKETPLACE_ENABLED:
            raise ValueError("marketplace is not enabled")

        features = FeatureService.get_system_features()

        manager = PluginInstaller()
        try:
            declaration = manager.fetch_plugin_manifest(tenant_id, plugin_unique_identifier)
        except Exception:
            pkg = download_plugin_pkg(plugin_unique_identifier)
            response = manager.upload_pkg(
                tenant_id,
                pkg,
                verify_signature=features.plugin_installation_permission.restrict_to_marketplace_only,
            )
            # check if the plugin is available to install
            PluginService._check_plugin_installation_scope(response.verification)
            declaration = response.manifest

        return declaration

    @staticmethod
    def install_from_marketplace_pkg(tenant_id: str, plugin_unique_identifiers: Sequence[str]):
        """
        Install plugin from marketplace package files,
        returns installation task id
        """
        if not dify_config.MARKETPLACE_ENABLED:
            raise ValueError("marketplace is not enabled")

        manager = PluginInstaller()

        # collect actual plugin_unique_identifiers
        actual_plugin_unique_identifiers = []
        metas = []
        features = FeatureService.get_system_features()

        # check if already downloaded
        for plugin_unique_identifier in plugin_unique_identifiers:
            try:
                manager.fetch_plugin_manifest(tenant_id, plugin_unique_identifier)
                plugin_decode_response = manager.decode_plugin_from_identifier(tenant_id, plugin_unique_identifier)
                # check if the plugin is available to install
                PluginService._check_plugin_installation_scope(plugin_decode_response.verification)
                # already downloaded, skip
                actual_plugin_unique_identifiers.append(plugin_unique_identifier)
                metas.append({"plugin_unique_identifier": plugin_unique_identifier})
            except Exception:
                # plugin not installed, download and upload pkg
                pkg = download_plugin_pkg(plugin_unique_identifier)
                response = manager.upload_pkg(
                    tenant_id,
                    pkg,
                    verify_signature=features.plugin_installation_permission.restrict_to_marketplace_only,
                )
                # check if the plugin is available to install
                PluginService._check_plugin_installation_scope(response.verification)
                # use response plugin_unique_identifier
                actual_plugin_unique_identifiers.append(response.unique_identifier)
                metas.append({"plugin_unique_identifier": response.unique_identifier})

        return manager.install_from_identifiers(
            tenant_id,
            actual_plugin_unique_identifiers,
            PluginInstallationSource.Marketplace,
            metas,
        )

    @staticmethod
    def uninstall(tenant_id: str, plugin_installation_id: str) -> bool:
        manager = PluginInstaller()

        # Get plugin info before uninstalling to delete associated credentials
        plugins = manager.list_plugins(tenant_id)
        plugin = next((p for p in plugins if p.installation_id == plugin_installation_id), None)

        if not plugin:
            return manager.uninstall(tenant_id, plugin_installation_id)

        if dify_config.ENTERPRISE_ENABLED:
            PluginManagerService.try_pre_uninstall_plugin(
                PreUninstallPluginRequest(
                    tenant_id=tenant_id,
                    plugin_unique_identifier=plugin.plugin_unique_identifier,
                )
            )
        with Session(db.engine) as session, session.begin():
            plugin_id = plugin.plugin_id
            logger.info("Deleting credentials for plugin: %s", plugin_id)

            session.execute(
                delete(TenantPreferredModelProvider).where(
                    TenantPreferredModelProvider.tenant_id == tenant_id,
                    TenantPreferredModelProvider.provider_name.like(f"{plugin_id}/%"),
                )
            )

            # Delete provider credentials that match this plugin
            credential_ids = session.scalars(
                select(ProviderCredential.id).where(
                    ProviderCredential.tenant_id == tenant_id,
                    ProviderCredential.provider_name.like(f"{plugin_id}/%"),
                )
            ).all()

            if not credential_ids:
                logger.info("No credentials found for plugin: %s", plugin_id)
                return manager.uninstall(tenant_id, plugin_installation_id)

            provider_ids = session.scalars(
                select(Provider.id).where(
                    Provider.tenant_id == tenant_id,
                    Provider.provider_name.like(f"{plugin_id}/%"),
                    Provider.credential_id.in_(credential_ids),
                )
            ).all()

            session.execute(update(Provider).where(Provider.id.in_(provider_ids)).values(credential_id=None))

            for provider_id in provider_ids:
                ProviderCredentialsCache(
                    tenant_id=tenant_id,
                    identity_id=provider_id,
                    cache_type=ProviderCredentialsCacheType.PROVIDER,
                ).delete()

            session.execute(
                delete(ProviderCredential).where(
                    ProviderCredential.id.in_(credential_ids),
                )
            )

            logger.info(
                "Completed deleting credentials and cleaning provider associations for plugin: %s",
                plugin_id,
            )

        return manager.uninstall(tenant_id, plugin_installation_id)

    @staticmethod
    def check_tools_existence(tenant_id: str, provider_ids: Sequence[GenericProviderID]) -> Sequence[bool]:
        """
        Check if the tools exist
        """
        manager = PluginInstaller()
        return manager.check_tools_existence(tenant_id, provider_ids)

    @staticmethod
    def fetch_plugin_readme(tenant_id: str, plugin_unique_identifier: str, language: str) -> str:
        """
        Fetch plugin readme
        """
        manager = PluginInstaller()
        return manager.fetch_plugin_readme(tenant_id, plugin_unique_identifier, language)

```

### api/services/plugin/dependencies_analysis.py
```py
import re

from configs import dify_config
from core.helper import marketplace
from core.plugin.entities.plugin import PluginDependency, PluginInstallationSource
from core.plugin.impl.plugin import PluginInstaller
from models.provider_ids import ModelProviderID, ToolProviderID

# Compile regex pattern for version extraction at module level for better performance
_VERSION_REGEX = re.compile(r":(?P<version>[0-9]+(?:\.[0-9]+){2}(?:[+-][0-9A-Za-z.-]+)?)(?:@|$)")


class DependenciesAnalysisService:
    @classmethod
    def analyze_tool_dependency(cls, tool_id: str) -> str:
        """
        Analyze the dependency of a tool.

        Convert the tool id to the plugin_id
        """
        try:
            return ToolProviderID(tool_id).plugin_id
        except Exception as e:
            raise e

    @classmethod
    def analyze_model_provider_dependency(cls, model_provider_id: str) -> str:
        """
        Analyze the dependency of a model provider.

        Convert the model provider id to the plugin_id
        """
        try:
            return ModelProviderID(model_provider_id).plugin_id
        except Exception as e:
            raise e

    @classmethod
    def get_leaked_dependencies(cls, tenant_id: str, dependencies: list[PluginDependency]) -> list[PluginDependency]:
        """
        Check dependencies, returns the leaked dependencies in current workspace
        """
        required_plugin_unique_identifiers = []
        for dependency in dependencies:
            required_plugin_unique_identifiers.append(dependency.value.plugin_unique_identifier)

        manager = PluginInstaller()

        # get leaked dependencies
        missing_plugins = manager.fetch_missing_dependencies(tenant_id, required_plugin_unique_identifiers)
        missing_plugin_unique_identifiers = {plugin.plugin_unique_identifier: plugin for plugin in missing_plugins}

        leaked_dependencies = []
        for dependency in dependencies:
            unique_identifier = dependency.value.plugin_unique_identifier
            if unique_identifier in missing_plugin_unique_identifiers:
                # Extract version for Marketplace dependencies
                if dependency.type == PluginDependency.Type.Marketplace:
                    version_match = _VERSION_REGEX.search(unique_identifier)
                    if version_match:
                        dependency.value.version = version_match.group("version")

                # Create and append the dependency (same for all types)
                leaked_dependencies.append(
                    PluginDependency(
                        type=dependency.type,
                        value=dependency.value,
                        current_identifier=missing_plugin_unique_identifiers[unique_identifier].current_identifier,
                    )
                )

        return leaked_dependencies

    @classmethod
    def generate_dependencies(cls, tenant_id: str, dependencies: list[str]) -> list[PluginDependency]:
        """
        Generate dependencies through the list of plugin ids
        """
        dependencies = list(set(dependencies))
        manager = PluginInstaller()
        plugins = manager.fetch_plugin_installation_by_ids(tenant_id, dependencies)
        result = []
        for plugin in plugins:
            if plugin.source == PluginInstallationSource.Github:
                result.append(
                    PluginDependency(
                        type=PluginDependency.Type.Github,
                        value=PluginDependency.Github(
                            repo=plugin.meta["repo"],
                            version=plugin.meta["version"],
                            package=plugin.meta["package"],
                            github_plugin_unique_identifier=plugin.plugin_unique_identifier,
                        ),
                    )
                )
            elif plugin.source == PluginInstallationSource.Marketplace:
                result.append(
                    PluginDependency(
                        type=PluginDependency.Type.Marketplace,
                        value=PluginDependency.Marketplace(
                            marketplace_plugin_unique_identifier=plugin.plugin_unique_identifier
                        ),
                    )
                )
            elif plugin.source == PluginInstallationSource.Package:
                result.append(
                    PluginDependency(
                        type=PluginDependency.Type.Package,
                        value=PluginDependency.Package(plugin_unique_identifier=plugin.plugin_unique_identifier),
                    )
                )
            elif plugin.source == PluginInstallationSource.Remote:
                raise ValueError(
                    f"You used a remote plugin: {plugin.plugin_unique_identifier} in the app, please remove it first"
                    " if you want to export the DSL."
                )
            else:
                raise ValueError(f"Unknown plugin source: {plugin.source}")

        return result

    @classmethod
    def generate_latest_dependencies(cls, dependencies: list[str]) -> list[PluginDependency]:
        """
        Generate the latest version of dependencies
        """
        dependencies = list(set(dependencies))
        if not dify_config.MARKETPLACE_ENABLED:
            return []
        deps = marketplace.batch_fetch_plugin_manifests(dependencies)
        return [
            PluginDependency(
                type=PluginDependency.Type.Marketplace,
                value=PluginDependency.Marketplace(marketplace_plugin_unique_identifier=dep.latest_package_identifier),
            )
            for dep in deps
        ]

```

### api/services/plugin/oauth_service.py
```py
import json
import uuid

from core.plugin.impl.base import BasePluginClient
from extensions.ext_redis import redis_client


class OAuthProxyService(BasePluginClient):
    # Default max age for proxy context parameter in seconds
    __MAX_AGE__ = 5 * 60  # 5 minutes
    __KEY_PREFIX__ = "oauth_proxy_context:"

    @staticmethod
    def create_proxy_context(
        user_id: str,
        tenant_id: str,
        plugin_id: str,
        provider: str,
        extra_data: dict = {},
        credential_id: str | None = None,
    ):
        """
        Create a proxy context for an OAuth 2.0 authorization request.

        This parameter is a crucial security measure to prevent Cross-Site Request
        Forgery (CSRF) attacks. It works by generating a unique nonce and storing it
        in a distributed cache (Redis) along with the user's session context.

        The returned nonce should be included as the 'proxy_context' parameter in the
        authorization URL. Upon callback, the `use_proxy_context` method
        is used to verify the state, ensuring the request's integrity and authenticity,
        and mitigating replay attacks.
        """
        context_id = str(uuid.uuid4())
        data = {
            **extra_data,
            "user_id": user_id,
            "plugin_id": plugin_id,
            "tenant_id": tenant_id,
            "provider": provider,
        }
        if credential_id:
            data["credential_id"] = credential_id
        redis_client.setex(
            f"{OAuthProxyService.__KEY_PREFIX__}{context_id}",
            OAuthProxyService.__MAX_AGE__,
            json.dumps(data),
        )
        return context_id

    @staticmethod
    def use_proxy_context(context_id: str):
        """
        Validate the proxy context parameter.
        This checks if the context_id is valid and not expired.
        """
        if not context_id:
            raise ValueError("context_id is required")
        # get data from redis
        key = f"{OAuthProxyService.__KEY_PREFIX__}{context_id}"
        data = redis_client.get(key)
        if not data:
            raise ValueError("context_id is invalid")
        redis_client.delete(key)
        return json.loads(data)

```

### api/services/plugin/__init__.py
```py

```

### api/services/plugin/plugin_migration.py
```py
import datetime
import json
import logging
import time
from collections.abc import Mapping, Sequence
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import TypedDict
from uuid import uuid4

import click
import sqlalchemy as sa
import tqdm
from flask import Flask, current_app
from pydantic import TypeAdapter
from sqlalchemy.orm import Session

from core.agent.entities import AgentToolEntity
from core.helper import marketplace
from core.plugin.entities.plugin import PluginInstallationSource
from core.plugin.entities.plugin_daemon import PluginInstallTaskStatus
from core.plugin.impl.plugin import PluginInstaller
from core.tools.entities.tool_entities import ToolProviderType
from extensions.ext_database import db
from models.account import Tenant
from models.model import App, AppMode, AppModelConfig
from models.provider_ids import ModelProviderID, ToolProviderID
from models.tools import BuiltinToolProvider
from models.workflow import Workflow
from services.plugin.plugin_service import PluginService

logger = logging.getLogger(__name__)

excluded_providers = ["time", "audio", "code", "webscraper"]


class _TenantPluginRecord(TypedDict):
    tenant_id: str
    plugins: list[str]


_tenant_plugin_adapter: TypeAdapter[_TenantPluginRecord] = TypeAdapter(_TenantPluginRecord)


class ExtractedPluginsDict(TypedDict):
    plugins: dict[str, str]
    plugin_not_exist: list[str]


class PluginInstallResultDict(TypedDict):
    success: list[str]
    failed: list[str]


class PluginMigration:
    @classmethod
    def extract_plugins(cls, filepath: str, workers: int):
        """
        Migrate plugin.
        """
        from threading import Lock

        click.echo(click.style("Migrating models/tools to new plugin Mechanism", fg="white"))
        ended_at = datetime.datetime.now()
        started_at = datetime.datetime(2023, 4, 3, 8, 59, 24)
        current_time = started_at

        with Session(db.engine) as session:
            total_tenant_count = session.query(Tenant.id).count()

        click.echo(click.style(f"Total tenant count: {total_tenant_count}", fg="white"))

        handled_tenant_count = 0
        file_lock = Lock()
        counter_lock = Lock()

        thread_pool = ThreadPoolExecutor(max_workers=workers)

        def process_tenant(flask_app: Flask, tenant_id: str):
            with flask_app.app_context():
                nonlocal handled_tenant_count
                try:
                    plugins = cls.extract_installed_plugin_ids(tenant_id)
                    # Use lock when writing to file
                    with file_lock:
                        with open(filepath, "a") as f:
                            f.write(json.dumps({"tenant_id": tenant_id, "plugins": plugins}) + "\n")

                    # Use lock when updating counter
                    with counter_lock:
                        nonlocal handled_tenant_count
                        handled_tenant_count += 1
                        click.echo(
                            click.style(
                                f"[{datetime.datetime.now()}] "
                                f"Processed {handled_tenant_count} tenants "
                                f"({(handled_tenant_count / total_tenant_count) * 100:.1f}%), "
                                f"{handled_tenant_count}/{total_tenant_count}",
                                fg="green",
                            )
                        )
                except Exception:
                    logger.exception("Failed to process tenant %s", tenant_id)

        futures = []

        while current_time < ended_at:
            click.echo(click.style(f"Current time: {current_time}, Started at: {datetime.datetime.now()}", fg="white"))
            # Initial interval of 1 day, will be dynamically adjusted based on tenant count
            interval = datetime.timedelta(days=1)
            # Process tenants in this batch
            with Session(db.engine) as session:
                # Calculate tenant count in next batch with current interval
                # Try different intervals until we find one with a reasonable tenant count
                test_intervals = [
                    datetime.timedelta(days=1),
                    datetime.timedelta(hours=12),
                    datetime.timedelta(hours=6),
                    datetime.timedelta(hours=3),
                    datetime.timedelta(hours=1),
                ]

                tenant_count = 0
                for test_interval in test_intervals:
                    tenant_count = (
                        session.query(Tenant.id)
                        .where(Tenant.created_at.between(current_time, current_time + test_interval))
                        .count()
                    )
                    if tenant_count <= 100:
                        interval = test_interval
                        break
                else:
                    # If all intervals have too many tenants, use minimum interval
                    interval = datetime.timedelta(hours=1)

                # Adjust interval to target ~100 tenants per batch
                if tenant_count > 0:
                    # Scale interval based on ratio to target count
                    interval = min(
                        datetime.timedelta(days=1),  # Max 1 day
                        max(
                            datetime.timedelta(hours=1),  # Min 1 hour
                            interval * (100 / tenant_count),  # Scale to target 100
                        ),
                    )

                batch_end = min(current_time + interval, ended_at)

                rs = (
                    session.query(Tenant.id)
                    .where(Tenant.created_at.between(current_time, batch_end))
                    .order_by(Tenant.created_at)
                )

                tenants = []
                for row in rs:
                    tenant_id = str(row.id)
                    try:
                        tenants.append(tenant_id)
                    except Exception:
                        logger.exception("Failed to process tenant %s", tenant_id)
                        continue

                    futures.append(
                        thread_pool.submit(
                            process_tenant,
                            current_app._get_current_object(),  # type: ignore
                            tenant_id,
                        )
                    )

            current_time = batch_end

        # wait for all threads to finish
        for future in futures:
            future.result()

    @classmethod
    def extract_installed_plugin_ids(cls, tenant_id: str) -> Sequence[str]:
        """
        Extract installed plugin ids.
        """
        tools = cls.extract_tool_tables(tenant_id)
        models = cls.extract_model_tables(tenant_id)
        workflows = cls.extract_workflow_tables(tenant_id)
        apps = cls.extract_app_tables(tenant_id)

        return list({*tools, *models, *workflows, *apps})

    @classmethod
    def extract_model_tables(cls, tenant_id: str) -> Sequence[str]:
        """
        Extract model tables.

        """
        models: list[str] = []
        table_pairs = [
            ("providers", "provider_name"),
            ("provider_models", "provider_name"),
            ("provider_orders", "provider_name"),
            ("tenant_default_models", "provider_name"),
            ("tenant_preferred_model_providers", "provider_name"),
            ("provider_model_settings", "provider_name"),
            ("load_balancing_model_configs", "provider_name"),
        ]

        for table, column in table_pairs:
            models.extend(cls.extract_model_table(tenant_id, table, column))

        # duplicate models
        models = list(set(models))

        return models

    @classmethod
    def extract_model_table(cls, tenant_id: str, table: str, column: str) -> Sequence[str]:
        """
        Extract model table.
        """
        with Session(db.engine) as session:
            rs = session.execute(
                sa.text(f"SELECT DISTINCT {column} FROM {table} WHERE tenant_id = :tenant_id"), {"tenant_id": tenant_id}
            )
            result = []
            for row in rs:
                provider_name = str(row[0])
                result.append(ModelProviderID(provider_name).plugin_id)

            return result

    @classmethod
    def extract_tool_tables(cls, tenant_id: str) -> Sequence[str]:
        """
        Extract tool tables.
        """
        with Session(db.engine) as session:
            rs = session.query(BuiltinToolProvider).where(BuiltinToolProvider.tenant_id == tenant_id).all()
            result = []
            for row in rs:
                result.append(ToolProviderID(row.provider).plugin_id)

            return result

    @classmethod
    def extract_workflow_tables(cls, tenant_id: str) -> Sequence[str]:
        """
        Extract workflow tables, only ToolNode is required.
        """

        with Session(db.engine) as session:
            rs = session.query(Workflow).where(Workflow.tenant_id == tenant_id).all()
            result = []
            for row in rs:
                graph = row.graph_dict
                # get nodes
                nodes = graph.get("nodes", [])

                for node in nodes:
                    data = node.get("data", {})
                    if data.get("type") == "tool":
                        provider_name = data.get("provider_name")
                        provider_type = data.get("provider_type")
                        if provider_name not in excluded_providers and provider_type == ToolProviderType.BUILT_IN:
                            result.append(ToolProviderID(provider_name).plugin_id)

            return result

    @classmethod
    def extract_app_tables(cls, tenant_id: str) -> Sequence[str]:
        """
        Extract app tables.
        """
        with Session(db.engine) as session:
            apps = session.query(App).where(App.tenant_id == tenant_id).all()
            if not apps:
                return []

            agent_app_model_config_ids = [
                app.app_model_config_id for app in apps if app.is_agent or app.mode == AppMode.AGENT_CHAT
            ]

            rs = session.query(AppModelConfig).where(AppModelConfig.id.in_(agent_app_model_config_ids)).all()
            result = []
            for row in rs:
                agent_config = row.agent_mode_dict
                if "tools" in agent_config and isinstance(agent_config["tools"], list):
                    for tool in agent_config["tools"]:
                        if isinstance(tool, dict):
                            try:
                                tool_entity = AgentToolEntity.model_validate(tool)
                                if (
                                    tool_entity.provider_type == ToolProviderType.BUILT_IN
                                    and tool_entity.provider_id not in excluded_providers
                                ):
                                    result.append(ToolProviderID(tool_entity.provider_id).plugin_id)

                            except Exception:
                                logger.exception("Failed to process tool %s", tool)
                                continue

            return result

    @classmethod
    def _fetch_plugin_unique_identifier(cls, plugin_id: str) -> str | None:
        """
        Fetch plugin unique identifier using plugin id.
        """
        plugin_manifest = marketplace.batch_fetch_plugin_manifests([plugin_id])
        if not plugin_manifest:
            return None

        return plugin_manifest[0].latest_package_identifier

    @classmethod
    def extract_unique_plugins_to_file(cls, extracted_plugins: str, output_file: str):
        """
        Extract unique plugins.
        """
        Path(output_file).write_text(json.dumps(cls.extract_unique_plugins(extracted_plugins)))

    @classmethod
    def extract_unique_plugins(cls, extracted_plugins: str) -> ExtractedPluginsDict:
        plugins: dict[str, str] = {}
        plugin_ids = []
        plugin_not_exist = []
        logger.info("Extracting unique plugins from %s", extracted_plugins)
        with open(extracted_plugins) as f:
            for line in f:
                data = _tenant_plugin_adapter.validate_json(line)
                for plugin_id in data["plugins"]:
                    if plugin_id not in plugin_ids:
                        plugin_ids.append(plugin_id)

        def fetch_plugin(plugin_id):
            try:
                unique_identifier = cls._fetch_plugin_unique_identifier(plugin_id)
                if unique_identifier:
                    plugins[plugin_id] = unique_identifier
                else:
                    plugin_not_exist.append(plugin_id)
            except Exception:
                logger.exception("Failed to fetch plugin unique identifier for %s", plugin_id)
                plugin_not_exist.append(plugin_id)

        with ThreadPoolExecutor(max_workers=10) as executor:
            list(tqdm.tqdm(executor.map(fetch_plugin, plugin_ids), total=len(plugin_ids)))

        return {"plugins": plugins, "plugin_not_exist": plugin_not_exist}

    @classmethod
    def install_plugins(cls, extracted_plugins: str, output_file: str, workers: int = 100):
        """
        Install plugins.
        """
        manager = PluginInstaller()

        plugins = cls.extract_unique_plugins(extracted_plugins)
        not_installed = []
        plugin_install_failed = []

        # use a fake tenant id to install all the plugins
        fake_tenant_id = uuid4().hex
        logger.info("Installing %s plugin instances for fake tenant %s", len(plugins["plugins"]), fake_tenant_id)

        thread_pool = ThreadPoolExecutor(max_workers=workers)

        response = cls.handle_plugin_instance_install(fake_tenant_id, plugins["plugins"])
        if response.get("failed"):
            plugin_install_failed.extend(response.get("failed", []))

        def install(tenant_id: str, plugin_ids: list[str]):
            logger.info("Installing %s plugins for tenant %s", len(plugin_ids), tenant_id)
            # fetch plugin already installed
            installed_plugins = manager.list_plugins(tenant_id)
            installed_plugins_ids = [plugin.plugin_id for plugin in installed_plugins]
            # at most 64 plugins one batch
            for i in range(0, len(plugin_ids), 64):
                batch_plugin_ids = plugin_ids[i : i + 64]
                batch_plugin_identifiers = [
                    plugins["plugins"][plugin_id]
                    for plugin_id in batch_plugin_ids
                    if plugin_id not in installed_plugins_ids and plugin_id in plugins["plugins"]
                ]
                manager.install_from_identifiers(
                    tenant_id,
                    batch_plugin_identifiers,
                    PluginInstallationSource.Marketplace,
                    metas=[
                        {
                            "plugin_unique_identifier": identifier,
                        }
                        for identifier in batch_plugin_identifiers
                    ],
                )

        with open(extracted_plugins) as f:
            """
            Read line by line, and install plugins for each tenant.
            """
            for line in f:
                data = _tenant_plugin_adapter.validate_json(line)
                tenant_id = data["tenant_id"]
                plugin_ids = data["plugins"]
                plugin_not_exist: list[str] = []
                # get plugin unique identifier
                for plugin_id in plugin_ids:
                    unique_identifier = plugins.get(plugin_id)
                    if unique_identifier:
                        plugin_not_exist.append(plugin_id)

                if plugin_not_exist:
                    not_installed.append(
                        {
                            "tenant_id": tenant_id,
                            "plugin_not_exist": plugin_not_exist,
                        }
                    )

                thread_pool.submit(install, tenant_id, plugin_ids)

        thread_pool.shutdown(wait=True)

        logger.info("Uninstall plugins")

        # get installation
        try:
            installation = manager.list_plugins(fake_tenant_id)
            while installation:
                for plugin in installation:
                    manager.uninstall(fake_tenant_id, plugin.installation_id)

                installation = manager.list_plugins(fake_tenant_id)
        except Exception:
            logger.exception("Failed to get installation for tenant %s", fake_tenant_id)

        Path(output_file).write_text(
            json.dumps(
                {
                    "not_installed": not_installed,
                    "plugin_install_failed": plugin_install_failed,
                }
            )
        )

    @classmethod
    def install_rag_pipeline_plugins(cls, extracted_plugins: str, output_file: str, workers: int = 100) -> None:
        """
        Install rag pipeline plugins.
        """
        manager = PluginInstaller()

        plugins = cls.extract_unique_plugins(extracted_plugins)
        plugin_install_failed = []

        # use a fake tenant id to install all the plugins
        fake_tenant_id = uuid4().hex
        logger.info("Installing %s plugin instances for fake tenant %s", len(plugins["plugins"]), fake_tenant_id)

        thread_pool = ThreadPoolExecutor(max_workers=workers)

        response = cls.handle_plugin_instance_install(fake_tenant_id, plugins["plugins"])
        if response.get("failed"):
            plugin_install_failed.extend(response.get("failed", []))

        def install(
            tenant_id: str, plugin_ids: dict[str, str], total_success_tenant: int, total_failed_tenant: int
        ) -> None:
            logger.info("Installing %s plugins for tenant %s", len(plugin_ids), tenant_id)
            try:
                # fetch plugin already installed
                installed_plugins = manager.list_plugins(tenant_id)
                installed_plugins_ids = [plugin.plugin_id for plugin in installed_plugins]
                # at most 64 plugins one batch
                for i in range(0, len(plugin_ids), 64):
                    batch_plugin_ids = list(plugin_ids.keys())[i : i + 64]
                    batch_plugin_identifiers = [
                        plugin_ids[plugin_id]
                        for plugin_id in batch_plugin_ids
                        if plugin_id not in installed_plugins_ids and plugin_id in plugin_ids
                    ]
                    PluginService.install_from_marketplace_pkg(tenant_id, batch_plugin_identifiers)

                total_success_tenant += 1
            except Exception:
                logger.exception("Failed to install plugins for tenant %s", tenant_id)
                total_failed_tenant += 1

        page = 1
        total_success_tenant = 0
        total_failed_tenant = 0
        while True:
            # paginate
            tenants = db.paginate(sa.select(Tenant).order_by(Tenant.created_at.desc()), page=page, per_page=100)
            if tenants.items is None or len(tenants.items) == 0:
                break

            for tenant in tenants:
                tenant_id = tenant.id
                # get plugin unique identifier
                thread_pool.submit(
                    install,
                    tenant_id,
                    plugins.get("plugins", {}),
                    total_success_tenant,
                    total_failed_tenant,
                )

            page += 1

        thread_pool.shutdown(wait=True)

        # uninstall all the plugins for fake tenant
        try:
            installation = manager.list_plugins(fake_tenant_id)
            while installation:
                for plugin in installation:
                    manager.uninstall(fake_tenant_id, plugin.installation_id)

                installation = manager.list_plugins(fake_tenant_id)
        except Exception:
            logger.exception("Failed to get installation for tenant %s", fake_tenant_id)

        Path(output_file).write_text(
            json.dumps(
                {
                    "total_success_tenant": total_success_tenant,
                    "total_failed_tenant": total_failed_tenant,
                    "plugin_install_failed": plugin_install_failed,
                }
            )
        )

    @classmethod
    def handle_plugin_instance_install(
        cls, tenant_id: str, plugin_identifiers_map: Mapping[str, str]
    ) -> PluginInstallResultDict:
        """
        Install plugins for a tenant.
        """
        manager = PluginInstaller()

        # download all the plugins and upload
        thread_pool = ThreadPoolExecutor(max_workers=10)
        futures = []

        for plugin_id, plugin_identifier in plugin_identifiers_map.items():

            def download_and_upload(tenant_id, plugin_id, plugin_identifier):
                plugin_package = marketplace.download_plugin_pkg(plugin_identifier)
                if not plugin_package:
                    raise Exception(f"Failed to download plugin {plugin_identifier}")

                # upload
                manager.upload_pkg(tenant_id, plugin_package, verify_signature=True)

            futures.append(thread_pool.submit(download_and_upload, tenant_id, plugin_id, plugin_identifier))

        # Wait for all downloads to complete
        for future in futures:
            future.result()  # This will raise any exceptions that occurred

        thread_pool.shutdown(wait=True)
        success = []
        failed = []

        reverse_map = {v: k for k, v in plugin_identifiers_map.items()}

        # at most 8 plugins one batch
        for i in range(0, len(plugin_identifiers_map), 8):
            batch_plugin_ids = list(plugin_identifiers_map.keys())[i : i + 8]
            batch_plugin_identifiers = [plugin_identifiers_map[plugin_id] for plugin_id in batch_plugin_ids]

            try:
                response = manager.install_from_identifiers(
                    tenant_id=tenant_id,
                    identifiers=batch_plugin_identifiers,
                    source=PluginInstallationSource.Marketplace,
                    metas=[
                        {
                            "plugin_unique_identifier": identifier,
                        }
                        for identifier in batch_plugin_identifiers
                    ],
                )
            except Exception:
                # add to failed
                failed.extend(batch_plugin_identifiers)
                continue

            if response.all_installed:
                success.extend(batch_plugin_identifiers)
                continue

            task_id = response.task_id
            done = False
            while not done:
                status = manager.fetch_plugin_installation_task(tenant_id, task_id)
                if status.status in [PluginInstallTaskStatus.Failed, PluginInstallTaskStatus.Success]:
                    for plugin in status.plugins:
                        if plugin.status == PluginInstallTaskStatus.Success:
                            success.append(reverse_map[plugin.plugin_unique_identifier])
                        else:
                            failed.append(reverse_map[plugin.plugin_unique_identifier])
                            logger.error(
                                "Failed to install plugin %s, error: %s",
                                plugin.plugin_unique_identifier,
                                plugin.message,
                            )

                    done = True
                else:
                    time.sleep(1)

        return {"success": success, "failed": failed}

```

### api/services/plugin/endpoint_service.py
```py
from core.plugin.impl.endpoint import PluginEndpointClient


class EndpointService:
    @classmethod
    def create_endpoint(cls, tenant_id: str, user_id: str, plugin_unique_identifier: str, name: str, settings: dict):
        return PluginEndpointClient().create_endpoint(
            tenant_id=tenant_id,
            user_id=user_id,
            plugin_unique_identifier=plugin_unique_identifier,
            name=name,
            settings=settings,
        )

    @classmethod
    def list_endpoints(cls, tenant_id: str, user_id: str, page: int, page_size: int):
        return PluginEndpointClient().list_endpoints(
            tenant_id=tenant_id,
            user_id=user_id,
            page=page,
            page_size=page_size,
        )

    @classmethod
    def list_endpoints_for_single_plugin(cls, tenant_id: str, user_id: str, plugin_id: str, page: int, page_size: int):
        return PluginEndpointClient().list_endpoints_for_single_plugin(
            tenant_id=tenant_id,
            user_id=user_id,
            plugin_id=plugin_id,
            page=page,
            page_size=page_size,
        )

    @classmethod
    def update_endpoint(cls, tenant_id: str, user_id: str, endpoint_id: str, name: str, settings: dict):
        return PluginEndpointClient().update_endpoint(
            tenant_id=tenant_id,
            user_id=user_id,
            endpoint_id=endpoint_id,
            name=name,
            settings=settings,
        )

    @classmethod
    def delete_endpoint(cls, tenant_id: str, user_id: str, endpoint_id: str):
        return PluginEndpointClient().delete_endpoint(
            tenant_id=tenant_id,
            user_id=user_id,
            endpoint_id=endpoint_id,
        )

    @classmethod
    def enable_endpoint(cls, tenant_id: str, user_id: str, endpoint_id: str):
        return PluginEndpointClient().enable_endpoint(
            tenant_id=tenant_id,
            user_id=user_id,
            endpoint_id=endpoint_id,
        )

    @classmethod
    def disable_endpoint(cls, tenant_id: str, user_id: str, endpoint_id: str):
        return PluginEndpointClient().disable_endpoint(
            tenant_id=tenant_id,
            user_id=user_id,
            endpoint_id=endpoint_id,
        )

```

### api/services/plugin/data_migration.py
```py
import json
import logging

import click
import sqlalchemy as sa

from extensions.ext_database import db
from models.provider_ids import GenericProviderID, ModelProviderID, ToolProviderID

logger = logging.getLogger(__name__)


class PluginDataMigration:
    @classmethod
    def migrate(cls):
        cls.migrate_db_records("providers", "provider_name", ModelProviderID)  # large table
        cls.migrate_db_records("provider_models", "provider_name", ModelProviderID)
        cls.migrate_db_records("provider_orders", "provider_name", ModelProviderID)
        cls.migrate_db_records("tenant_default_models", "provider_name", ModelProviderID)
        cls.migrate_db_records("tenant_preferred_model_providers", "provider_name", ModelProviderID)
        cls.migrate_db_records("provider_model_settings", "provider_name", ModelProviderID)
        cls.migrate_db_records("load_balancing_model_configs", "provider_name", ModelProviderID)
        cls.migrate_datasets()
        cls.migrate_db_records("embeddings", "provider_name", ModelProviderID)  # large table
        cls.migrate_db_records("dataset_collection_bindings", "provider_name", ModelProviderID)
        cls.migrate_db_records("tool_builtin_providers", "provider", ToolProviderID)

    @classmethod
    def migrate_datasets(cls):
        table_name = "datasets"
        provider_column_name = "embedding_model_provider"

        click.echo(click.style(f"Migrating [{table_name}] data for plugin", fg="white"))

        processed_count = 0
        failed_ids = []
        while True:
            sql = f"""select id, {provider_column_name} as provider_name, retrieval_model from {table_name}
where {provider_column_name} not like '%/%' and {provider_column_name} is not null and {provider_column_name} != ''
limit 1000"""
            with db.engine.begin() as conn:
                rs = conn.execute(sa.text(sql))

                current_iter_count = 0
                for i in rs:
                    record_id = str(i.id)
                    provider_name = str(i.provider_name)
                    retrieval_model = i.retrieval_model
                    logger.debug(
                        "Processing dataset %s with retrieval model of type %s",
                        record_id,
                        type(retrieval_model),
                    )

                    if record_id in failed_ids:
                        continue

                    retrieval_model_changed = False
                    if retrieval_model:
                        if (
                            "reranking_model" in retrieval_model
                            and "reranking_provider_name" in retrieval_model["reranking_model"]
                            and retrieval_model["reranking_model"]["reranking_provider_name"]
                            and "/" not in retrieval_model["reranking_model"]["reranking_provider_name"]
                        ):
                            click.echo(
                                click.style(
                                    f"[{processed_count}] Migrating {table_name} {record_id} "
                                    f"(reranking_provider_name: "
                                    f"{retrieval_model['reranking_model']['reranking_provider_name']})",
                                    fg="white",
                                )
                            )
                            # update google to langgenius/gemini/google etc.
                            retrieval_model["reranking_model"]["reranking_provider_name"] = ModelProviderID(
                                retrieval_model["reranking_model"]["reranking_provider_name"]
                            ).to_string()
                            retrieval_model_changed = True

                    click.echo(
                        click.style(
                            f"[{processed_count}] Migrating [{table_name}] {record_id} ({provider_name})",
                            fg="white",
                        )
                    )

                    try:
                        # update provider name append with "langgenius/{provider_name}/{provider_name}"
                        params = {"record_id": record_id}
                        update_retrieval_model_sql = ""
                        if retrieval_model and retrieval_model_changed:
                            update_retrieval_model_sql = ", retrieval_model = :retrieval_model"
                            params["retrieval_model"] = json.dumps(retrieval_model)

                        params["provider_name"] = ModelProviderID(provider_name).to_string()

                        sql = f"""update {table_name}
                        set {provider_column_name} =
                        :provider_name
                        {update_retrieval_model_sql}
                        where id = :record_id"""
                        conn.execute(sa.text(sql), params)
                        click.echo(
                            click.style(
                                f"[{processed_count}] Migrated [{table_name}] {record_id} ({provider_name})",
                                fg="green",
                            )
                        )
                    except Exception:
                        failed_ids.append(record_id)
                        click.echo(
                            click.style(
                                f"[{processed_count}] Failed to migrate [{table_name}] {record_id} ({provider_name})",
                                fg="red",
                            )
                        )
                        logger.exception(
                            "[%s] Failed to migrate [%s] %s (%s)", processed_count, table_name, record_id, provider_name
                        )
                        continue

                    current_iter_count += 1
                    processed_count += 1

            if not current_iter_count:
                break

        click.echo(
            click.style(f"Migrate [{table_name}] data for plugin completed, total: {processed_count}", fg="green")
        )

    @classmethod
    def migrate_db_records(cls, table_name: str, provider_column_name: str, provider_cls: type[GenericProviderID]):
        click.echo(click.style(f"Migrating [{table_name}] data for plugin", fg="white"))

        processed_count = 0
        failed_ids = []
        last_id = "00000000-0000-0000-0000-000000000000"

        while True:
            sql = f"""
                SELECT id, {provider_column_name} AS provider_name
                FROM {table_name}
                WHERE {provider_column_name} NOT LIKE '%/%'
                    AND {provider_column_name} IS NOT NULL
                    AND {provider_column_name} != ''
                    AND id > :last_id
                ORDER BY id ASC
                LIMIT 5000
            """
            params = {"last_id": last_id or ""}

            with db.engine.begin() as conn:
                rs = conn.execute(sa.text(sql), params)

                current_iter_count = 0
                batch_updates = []

                for i in rs:
                    current_iter_count += 1
                    processed_count += 1
                    record_id = str(i.id)
                    last_id = record_id
                    provider_name = str(i.provider_name)

                    if record_id in failed_ids:
                        continue

                    click.echo(
                        click.style(
                            f"[{processed_count}] Migrating [{table_name}] {record_id} ({provider_name})",
                            fg="white",
                        )
                    )

                    try:
                        # update jina to langgenius/jina_tool/jina etc.
                        updated_value = provider_cls(provider_name).to_string()
                        batch_updates.append((updated_value, record_id))
                    except Exception:
                        failed_ids.append(record_id)
                        click.echo(
                            click.style(
                                f"[{processed_count}] Failed to migrate [{table_name}] {record_id} ({provider_name})",
                                fg="red",
                            )
                        )
                        logger.exception(
                            "[%s] Failed to migrate [%s] %s (%s)", processed_count, table_name, record_id, provider_name
                        )
                        continue

                if batch_updates:
                    update_sql = f"""
                        UPDATE {table_name}
                        SET {provider_column_name} = :updated_value
                        WHERE id = :record_id
                    """
                    conn.execute(sa.text(update_sql), [{"updated_value": u, "record_id": r} for u, r in batch_updates])
                    click.echo(
                        click.style(
                            f"[{processed_count}] Batch migrated [{len(batch_updates)}] records from [{table_name}]",
                            fg="green",
                        )
                    )

            if not current_iter_count:
                break

        click.echo(
            click.style(f"Migrate [{table_name}] data for plugin completed, total: {processed_count}", fg="green")
        )

```

### api/services/plugin/plugin_permission_service.py
```py
from sqlalchemy.orm import Session

from extensions.ext_database import db
from models.account import TenantPluginPermission


class PluginPermissionService:
    @staticmethod
    def get_permission(tenant_id: str) -> TenantPluginPermission | None:
        with Session(db.engine) as session:
            return session.query(TenantPluginPermission).where(TenantPluginPermission.tenant_id == tenant_id).first()

    @staticmethod
    def change_permission(
        tenant_id: str,
        install_permission: TenantPluginPermission.InstallPermission,
        debug_permission: TenantPluginPermission.DebugPermission,
    ):
        with Session(db.engine) as session:
            permission = (
                session.query(TenantPluginPermission).where(TenantPluginPermission.tenant_id == tenant_id).first()
            )
            if not permission:
                permission = TenantPluginPermission(
                    tenant_id=tenant_id, install_permission=install_permission, debug_permission=debug_permission
                )

                session.add(permission)
            else:
                permission.install_permission = install_permission
                permission.debug_permission = debug_permission

            session.commit()
            return True

```

### api/services/plugin/plugin_auto_upgrade_service.py
```py
from sqlalchemy.orm import Session

from extensions.ext_database import db
from models.account import TenantPluginAutoUpgradeStrategy


class PluginAutoUpgradeService:
    @staticmethod
    def get_strategy(tenant_id: str) -> TenantPluginAutoUpgradeStrategy | None:
        with Session(db.engine) as session:
            return (
                session.query(TenantPluginAutoUpgradeStrategy)
                .where(TenantPluginAutoUpgradeStrategy.tenant_id == tenant_id)
                .first()
            )

    @staticmethod
    def change_strategy(
        tenant_id: str,
        strategy_setting: TenantPluginAutoUpgradeStrategy.StrategySetting,
        upgrade_time_of_day: int,
        upgrade_mode: TenantPluginAutoUpgradeStrategy.UpgradeMode,
        exclude_plugins: list[str],
        include_plugins: list[str],
    ) -> bool:
        with Session(db.engine) as session:
            exist_strategy = (
                session.query(TenantPluginAutoUpgradeStrategy)
                .where(TenantPluginAutoUpgradeStrategy.tenant_id == tenant_id)
                .first()
            )
            if not exist_strategy:
                strategy = TenantPluginAutoUpgradeStrategy(
                    tenant_id=tenant_id,
                    strategy_setting=strategy_setting,
                    upgrade_time_of_day=upgrade_time_of_day,
                    upgrade_mode=upgrade_mode,
                    exclude_plugins=exclude_plugins,
                    include_plugins=include_plugins,
                )
                session.add(strategy)
            else:
                exist_strategy.strategy_setting = strategy_setting
                exist_strategy.upgrade_time_of_day = upgrade_time_of_day
                exist_strategy.upgrade_mode = upgrade_mode
                exist_strategy.exclude_plugins = exclude_plugins
                exist_strategy.include_plugins = include_plugins

            session.commit()
            return True

    @staticmethod
    def exclude_plugin(tenant_id: str, plugin_id: str) -> bool:
        with Session(db.engine) as session:
            exist_strategy = (
                session.query(TenantPluginAutoUpgradeStrategy)
                .where(TenantPluginAutoUpgradeStrategy.tenant_id == tenant_id)
                .first()
            )
            if not exist_strategy:
                # create for this tenant
                PluginAutoUpgradeService.change_strategy(
                    tenant_id,
                    TenantPluginAutoUpgradeStrategy.StrategySetting.FIX_ONLY,
                    0,
                    TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE,
                    [plugin_id],
                    [],
                )
                return True
            else:
                if exist_strategy.upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE:
                    if plugin_id not in exist_strategy.exclude_plugins:
                        new_exclude_plugins = exist_strategy.exclude_plugins.copy()
                        new_exclude_plugins.append(plugin_id)
                        exist_strategy.exclude_plugins = new_exclude_plugins
                elif exist_strategy.upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.PARTIAL:
                    if plugin_id in exist_strategy.include_plugins:
                        new_include_plugins = exist_strategy.include_plugins.copy()
                        new_include_plugins.remove(plugin_id)
                        exist_strategy.include_plugins = new_include_plugins
                elif exist_strategy.upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.ALL:
                    exist_strategy.upgrade_mode = TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE
                    exist_strategy.exclude_plugins = [plugin_id]

                session.commit()
                return True

```

### api/services/plugin/plugin_parameter_service.py
```py
from collections.abc import Mapping, Sequence
from typing import Any, Literal

from sqlalchemy import select
from sqlalchemy.orm import Session

from core.plugin.entities.parameters import PluginParameterOption
from core.plugin.entities.plugin_daemon import CredentialType
from core.plugin.impl.dynamic_select import DynamicSelectClient
from core.tools.tool_manager import ToolManager
from core.tools.utils.encryption import create_tool_provider_encrypter
from core.trigger.entities.api_entities import TriggerProviderSubscriptionApiEntity
from core.trigger.entities.entities import SubscriptionBuilder
from extensions.ext_database import db
from models.tools import BuiltinToolProvider
from services.trigger.trigger_provider_service import TriggerProviderService
from services.trigger.trigger_subscription_builder_service import TriggerSubscriptionBuilderService


class PluginParameterService:
    @staticmethod
    def get_dynamic_select_options(
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        action: str,
        parameter: str,
        credential_id: str | None,
        provider_type: Literal["tool", "trigger"],
    ) -> Sequence[PluginParameterOption]:
        """
        Get dynamic select options for a plugin parameter.

        Args:
            tenant_id: The tenant ID.
            plugin_id: The plugin ID.
            provider: The provider name.
            action: The action name.
            parameter: The parameter name.
        """
        credentials: Mapping[str, Any] = {}
        credential_type: str = CredentialType.UNAUTHORIZED.value
        match provider_type:
            case "tool":
                provider_controller = ToolManager.get_builtin_provider(provider, tenant_id)
                # init tool configuration
                encrypter, _ = create_tool_provider_encrypter(
                    tenant_id=tenant_id,
                    controller=provider_controller,
                )

                # check if credentials are required
                if not provider_controller.need_credentials:
                    credentials = {}
                else:
                    # fetch credentials from db
                    with Session(db.engine) as session:
                        if credential_id:
                            db_record = session.scalar(
                                select(BuiltinToolProvider)
                                .where(
                                    BuiltinToolProvider.tenant_id == tenant_id,
                                    BuiltinToolProvider.provider == provider,
                                    BuiltinToolProvider.id == credential_id,
                                )
                                .limit(1)
                            )
                        else:
                            db_record = session.scalar(
                                select(BuiltinToolProvider)
                                .where(
                                    BuiltinToolProvider.tenant_id == tenant_id,
                                    BuiltinToolProvider.provider == provider,
                                )
                                .order_by(BuiltinToolProvider.is_default.desc(), BuiltinToolProvider.created_at.asc())
                                .limit(1)
                            )

                    if db_record is None:
                        raise ValueError(f"Builtin provider {provider} not found when fetching credentials")

                    credentials = encrypter.decrypt(db_record.credentials)
                    credential_type = db_record.credential_type
            case "trigger":
                subscription: TriggerProviderSubscriptionApiEntity | SubscriptionBuilder | None
                if credential_id:
                    subscription = TriggerSubscriptionBuilderService.get_subscription_builder(credential_id)
                    if not subscription:
                        trigger_subscription = TriggerProviderService.get_subscription_by_id(tenant_id, credential_id)
                        subscription = trigger_subscription.to_api_entity() if trigger_subscription else None
                else:
                    trigger_subscription = TriggerProviderService.get_subscription_by_id(tenant_id)
                    subscription = trigger_subscription.to_api_entity() if trigger_subscription else None

                if subscription is None:
                    raise ValueError(f"Subscription {credential_id} not found")

                credentials = subscription.credentials
                credential_type = subscription.credential_type or CredentialType.UNAUTHORIZED

        return (
            DynamicSelectClient()
            .fetch_dynamic_select_options(
                tenant_id, user_id, plugin_id, provider, action, credentials, credential_type, parameter
            )
            .options
        )

    @staticmethod
    def get_dynamic_select_options_with_credentials(
        tenant_id: str,
        user_id: str,
        plugin_id: str,
        provider: str,
        action: str,
        parameter: str,
        credential_id: str,
        credentials: Mapping[str, Any],
    ) -> Sequence[PluginParameterOption]:
        """
        Get dynamic select options using provided credentials directly.
        Used for edit mode when credentials have been modified but not yet saved.

        Security: credential_id is validated against tenant_id to ensure
        users can only access their own credentials.
        """
        from constants import HIDDEN_VALUE

        # Get original subscription to replace hidden values (with tenant_id check for security)
        original_subscription = TriggerProviderService.get_subscription_by_id(tenant_id, credential_id)
        if not original_subscription:
            raise ValueError(f"Subscription {credential_id} not found")

        # Replace [__HIDDEN__] with original values
        resolved_credentials: dict[str, Any] = {
            key: (original_subscription.credentials.get(key) if value == HIDDEN_VALUE else value)
            for key, value in credentials.items()
        }

        return (
            DynamicSelectClient()
            .fetch_dynamic_select_options(
                tenant_id,
                user_id,
                plugin_id,
                provider,
                action,
                resolved_credentials,
                original_subscription.credential_type or CredentialType.UNAUTHORIZED.value,
                parameter,
            )
            .options
        )

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-008.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
