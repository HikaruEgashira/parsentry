You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-019
- **Kind**: endpoint
- **Identifier**: POST /console/api/datasets/<dataset_id>/documents
- **Description**: Document creation endpoints accepting file uploads and URL sources for knowledge base indexing; untrusted content flows into embedding and retrieval pipelines
- **Locations**: api/controllers/console/datasets/datasets.py, api/controllers/service_api/dataset/document.py

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

### api/controllers/service_api/dataset/document.py
```py
import json
from contextlib import ExitStack
from typing import Self
from uuid import UUID

from flask import request, send_file
from flask_restx import marshal
from pydantic import BaseModel, Field, field_validator, model_validator
from sqlalchemy import desc, func, select
from werkzeug.exceptions import Forbidden, NotFound

import services
from controllers.common.errors import (
    FilenameNotExistsError,
    FileTooLargeError,
    NoFileUploadedError,
    TooManyFilesError,
    UnsupportedFileTypeError,
)
from controllers.common.schema import register_enum_models, register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import ProviderNotInitializeError
from controllers.service_api.dataset.error import (
    ArchivedDocumentImmutableError,
    DocumentIndexingError,
    InvalidMetadataError,
)
from controllers.service_api.wraps import (
    DatasetApiResource,
    cloud_edition_billing_rate_limit_check,
    cloud_edition_billing_resource_check,
)
from core.errors.error import ProviderTokenNotInitError
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from extensions.ext_database import db
from fields.document_fields import document_fields, document_status_fields
from libs.login import current_user
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import SegmentStatus
from services.dataset_service import DatasetService, DocumentService
from services.entities.knowledge_entities.knowledge_entities import (
    KnowledgeConfig,
    PreProcessingRule,
    ProcessRule,
    RetrievalModel,
    Rule,
    Segmentation,
)
from services.file_service import FileService
from services.summary_index_service import SummaryIndexService


class DocumentTextCreatePayload(BaseModel):
    name: str
    text: str
    process_rule: ProcessRule | None = None
    original_document_id: str | None = None
    doc_form: str = Field(default="text_model")
    doc_language: str = Field(default="English")
    indexing_technique: str | None = None
    retrieval_model: RetrievalModel | None = None
    embedding_model: str | None = None
    embedding_model_provider: str | None = None

    @field_validator("doc_form")
    @classmethod
    def validate_doc_form(cls, value: str) -> str:
        if value not in Dataset.DOC_FORM_LIST:
            raise ValueError("Invalid doc_form.")
        return value


DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class DocumentTextUpdate(BaseModel):
    name: str | None = None
    text: str | None = None
    process_rule: ProcessRule | None = None
    doc_form: str = "text_model"
    doc_language: str = "English"
    retrieval_model: RetrievalModel | None = None

    @field_validator("doc_form")
    @classmethod
    def validate_doc_form(cls, value: str) -> str:
        if value not in Dataset.DOC_FORM_LIST:
            raise ValueError("Invalid doc_form.")
        return value

    @model_validator(mode="after")
    def check_text_and_name(self) -> Self:
        if self.text is not None and self.name is None:
            raise ValueError("name is required when text is provided")
        return self


class DocumentListQuery(BaseModel):
    page: int = Field(default=1, description="Page number")
    limit: int = Field(default=20, description="Number of items per page")
    keyword: str | None = Field(default=None, description="Search keyword")
    status: str | None = Field(default=None, description="Document status filter")


DOCUMENT_BATCH_DOWNLOAD_ZIP_MAX_DOCS = 100


class DocumentBatchDownloadZipPayload(BaseModel):
    """Request payload for bulk downloading uploaded documents as a ZIP archive."""

    document_ids: list[UUID] = Field(..., min_length=1, max_length=DOCUMENT_BATCH_DOWNLOAD_ZIP_MAX_DOCS)


register_enum_models(service_api_ns, RetrievalMethod)

register_schema_models(
    service_api_ns,
    ProcessRule,
    RetrievalModel,
    DocumentTextCreatePayload,
    DocumentTextUpdate,
    DocumentListQuery,
    DocumentBatchDownloadZipPayload,
    Rule,
    PreProcessingRule,
    Segmentation,
)


@service_api_ns.route(
    "/datasets/<uuid:dataset_id>/document/create_by_text",
    "/datasets/<uuid:dataset_id>/document/create-by-text",
)
class DocumentAddByTextApi(DatasetApiResource):
    """Resource for documents."""

    @service_api_ns.expect(service_api_ns.models[DocumentTextCreatePayload.__name__])
    @service_api_ns.doc("create_document_by_text")
    @service_api_ns.doc(description="Create a new document by providing text content")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID"})
    @service_api_ns.doc(
        responses={
            200: "Document created successfully",
            401: "Unauthorized - invalid API token",
            400: "Bad request - invalid parameters",
        }
    )
    @cloud_edition_billing_resource_check("vector_space", "dataset")
    @cloud_edition_billing_resource_check("documents", "dataset")
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def post(self, tenant_id, dataset_id):
        """Create document by text."""
        payload = DocumentTextCreatePayload.model_validate(service_api_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        dataset_id = str(dataset_id)
        tenant_id = str(tenant_id)
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )

        if not dataset:
            raise ValueError("Dataset does not exist.")

        if not dataset.indexing_technique and not args["indexing_technique"]:
            raise ValueError("indexing_technique is required.")

        embedding_model_provider = payload.embedding_model_provider
        embedding_model = payload.embedding_model
        if embedding_model_provider and embedding_model:
            DatasetService.check_embedding_model_setting(tenant_id, embedding_model_provider, embedding_model)

        retrieval_model = payload.retrieval_model
        if (
            retrieval_model
            and retrieval_model.reranking_model
            and retrieval_model.reranking_model.reranking_provider_name
            and retrieval_model.reranking_model.reranking_model_name
        ):
            DatasetService.check_reranking_model_setting(
                tenant_id,
                retrieval_model.reranking_model.reranking_provider_name,
                retrieval_model.reranking_model.reranking_model_name,
            )

        if not current_user:
            raise ValueError("current_user is required")

        upload_file = FileService(db.engine).upload_text(
            text=payload.text, text_name=payload.name, user_id=current_user.id, tenant_id=tenant_id
        )
        data_source = {
            "type": "upload_file",
            "info_list": {"data_source_type": "upload_file", "file_info_list": {"file_ids": [upload_file.id]}},
        }
        args["data_source"] = data_source
        knowledge_config = KnowledgeConfig.model_validate(args)
        # validate args
        DocumentService.document_create_args_validate(knowledge_config)

        if not current_user:
            raise ValueError("current_user is required")

        try:
            documents, batch = DocumentService.save_document_with_dataset_id(
                dataset=dataset,
                knowledge_config=knowledge_config,
                account=current_user,
                dataset_process_rule=dataset.latest_process_rule if "process_rule" not in args else None,
                created_from="api",
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        document = documents[0]

        documents_and_batch_fields = {"document": marshal(document, document_fields), "batch": batch}
        return documents_and_batch_fields, 200


@service_api_ns.route(
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/update_by_text",
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/update-by-text",
)
class DocumentUpdateByTextApi(DatasetApiResource):
    """Resource for update documents."""

    @service_api_ns.expect(service_api_ns.models[DocumentTextUpdate.__name__])
    @service_api_ns.doc("update_document_by_text")
    @service_api_ns.doc(description="Update an existing document by providing text content")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "document_id": "Document ID"})
    @service_api_ns.doc(
        responses={
            200: "Document updated successfully",
            401: "Unauthorized - invalid API token",
            404: "Document not found",
        }
    )
    @cloud_edition_billing_resource_check("vector_space", "dataset")
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def post(self, tenant_id: str, dataset_id: UUID, document_id: UUID):
        """Update document by text."""
        payload = DocumentTextUpdate.model_validate(service_api_ns.payload or {})
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == str(dataset_id)).limit(1)
        )
        args = payload.model_dump(exclude_none=True)
        if not dataset:
            raise ValueError("Dataset does not exist.")

        retrieval_model = payload.retrieval_model
        if (
            retrieval_model
            and retrieval_model.reranking_model
            and retrieval_model.reranking_model.reranking_provider_name
            and retrieval_model.reranking_model.reranking_model_name
        ):
            DatasetService.check_reranking_model_setting(
                tenant_id,
                retrieval_model.reranking_model.reranking_provider_name,
                retrieval_model.reranking_model.reranking_model_name,
            )

        # indexing_technique is already set in dataset since this is an update
        args["indexing_technique"] = dataset.indexing_technique

        if args.get("text"):
            text = args.get("text")
            name = args.get("name")
            if not current_user:
                raise ValueError("current_user is required")
            upload_file = FileService(db.engine).upload_text(
                text=str(text), text_name=str(name), user_id=current_user.id, tenant_id=tenant_id
            )
            data_source = {
                "type": "upload_file",
                "info_list": {"data_source_type": "upload_file", "file_info_list": {"file_ids": [upload_file.id]}},
            }
            args["data_source"] = data_source
        # validate args
        args["original_document_id"] = str(document_id)
        knowledge_config = KnowledgeConfig.model_validate(args)
        DocumentService.document_create_args_validate(knowledge_config)

        try:
            documents, batch = DocumentService.save_document_with_dataset_id(
                dataset=dataset,
                knowledge_config=knowledge_config,
                account=current_user,
                dataset_process_rule=dataset.latest_process_rule if "process_rule" not in args else None,
                created_from="api",
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        document = documents[0]

        documents_and_batch_fields = {"document": marshal(document, document_fields), "batch": batch}
        return documents_and_batch_fields, 200


@service_api_ns.route(
    "/datasets/<uuid:dataset_id>/document/create_by_file",
    "/datasets/<uuid:dataset_id>/document/create-by-file",
)
class DocumentAddByFileApi(DatasetApiResource):
    """Resource for documents."""

    @service_api_ns.doc("create_document_by_file")
    @service_api_ns.doc(description="Create a new document by uploading a file")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID"})
    @service_api_ns.doc(
        responses={
            200: "Document created successfully",
            401: "Unauthorized - invalid API token",
            400: "Bad request - invalid file or parameters",
        }
    )
    @cloud_edition_billing_resource_check("vector_space", "dataset")
    @cloud_edition_billing_resource_check("documents", "dataset")
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def post(self, tenant_id, dataset_id):
        """Create document by upload file."""
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )

        if not dataset:
            raise ValueError("Dataset does not exist.")

        if dataset.provider == "external":
            raise ValueError("External datasets are not supported.")

        args = {}
        if "data" in request.form:
            args = json.loads(request.form["data"])
        if "doc_form" not in args:
            args["doc_form"] = dataset.chunk_structure or "text_model"
        if "doc_language" not in args:
            args["doc_language"] = "English"

        # get dataset info
        dataset_id = str(dataset_id)
        tenant_id = str(tenant_id)

        indexing_technique = args.get("indexing_technique") or dataset.indexing_technique
        if not indexing_technique:
            raise ValueError("indexing_technique is required.")
        args["indexing_technique"] = indexing_technique

        if "embedding_model_provider" in args:
            DatasetService.check_embedding_model_setting(
                tenant_id, args["embedding_model_provider"], args["embedding_model"]
            )
        if (
            "retrieval_model" in args
            and args["retrieval_model"].get("reranking_model")
            and args["retrieval_model"].get("reranking_model").get("reranking_provider_name")
        ):
            DatasetService.check_reranking_model_setting(
                tenant_id,
                args["retrieval_model"].get("reranking_model").get("reranking_provider_name"),
                args["retrieval_model"].get("reranking_model").get("reranking_model_name"),
            )

        # check file
        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()

        # save file info
        file = request.files["file"]
        if not file.filename:
            raise FilenameNotExistsError

        if not current_user:
            raise ValueError("current_user is required")
        upload_file = FileService(db.engine).upload_file(
            filename=file.filename,
            content=file.read(),
            mimetype=file.mimetype,
            user=current_user,
            source="datasets",
        )
        data_source = {
            "type": "upload_file",
            "info_list": {"data_source_type": "upload_file", "file_info_list": {"file_ids": [upload_file.id]}},
        }
        args["data_source"] = data_source
        # validate args
        knowledge_config = KnowledgeConfig.model_validate(args)
        DocumentService.document_create_args_validate(knowledge_config)

        dataset_process_rule = dataset.latest_process_rule if "process_rule" not in args else None
        if not knowledge_config.original_document_id and not dataset_process_rule and not knowledge_config.process_rule:
            raise ValueError("process_rule is required.")

        try:
            documents, batch = DocumentService.save_document_with_dataset_id(
                dataset=dataset,
                knowledge_config=knowledge_config,
                account=dataset.created_by_account,
                dataset_process_rule=dataset_process_rule,
                created_from="api",
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        document = documents[0]
        documents_and_batch_fields = {"document": marshal(document, document_fields), "batch": batch}
        return documents_and_batch_fields, 200


@service_api_ns.route(
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/update_by_file",
    "/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/update-by-file",
)
class DocumentUpdateByFileApi(DatasetApiResource):
    """Resource for update documents."""

    @service_api_ns.doc("update_document_by_file")
    @service_api_ns.doc(description="Update an existing document by uploading a file")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "document_id": "Document ID"})
    @service_api_ns.doc(
        responses={
            200: "Document updated successfully",
            401: "Unauthorized - invalid API token",
            404: "Document not found",
        }
    )
    @cloud_edition_billing_resource_check("vector_space", "dataset")
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def post(self, tenant_id, dataset_id, document_id):
        """Update document by upload file."""
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )

        if not dataset:
            raise ValueError("Dataset does not exist.")

        if dataset.provider == "external":
            raise ValueError("External datasets are not supported.")

        args = {}
        if "data" in request.form:
            args = json.loads(request.form["data"])
        if "doc_form" not in args:
            args["doc_form"] = dataset.chunk_structure or "text_model"
        if "doc_language" not in args:
            args["doc_language"] = "English"

        # get dataset info
        dataset_id = str(dataset_id)
        tenant_id = str(tenant_id)

        # indexing_technique is already set in dataset since this is an update
        args["indexing_technique"] = dataset.indexing_technique

        if "file" in request.files:
            # save file info
            file = request.files["file"]

            if len(request.files) > 1:
                raise TooManyFilesError()

            if not file.filename:
                raise FilenameNotExistsError

            if not current_user:
                raise ValueError("current_user is required")

            try:
                upload_file = FileService(db.engine).upload_file(
                    filename=file.filename,
                    content=file.read(),
                    mimetype=file.mimetype,
                    user=current_user,
                    source="datasets",
                )
            except services.errors.file.FileTooLargeError as file_too_large_error:
                raise FileTooLargeError(file_too_large_error.description)
            except services.errors.file.UnsupportedFileTypeError:
                raise UnsupportedFileTypeError()
            data_source = {
                "type": "upload_file",
                "info_list": {"data_source_type": "upload_file", "file_info_list": {"file_ids": [upload_file.id]}},
            }
            args["data_source"] = data_source
        # validate args
        args["original_document_id"] = str(document_id)

        knowledge_config = KnowledgeConfig.model_validate(args)
        DocumentService.document_create_args_validate(knowledge_config)

        try:
            documents, _ = DocumentService.save_document_with_dataset_id(
                dataset=dataset,
                knowledge_config=knowledge_config,
                account=dataset.created_by_account,
                dataset_process_rule=dataset.latest_process_rule if "process_rule" not in args else None,
                created_from="api",
            )
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        document = documents[0]
        documents_and_batch_fields = {"document": marshal(document, document_fields), "batch": document.batch}
        return documents_and_batch_fields, 200


@service_api_ns.route("/datasets/<uuid:dataset_id>/documents")
class DocumentListApi(DatasetApiResource):
    @service_api_ns.doc("list_documents")
    @service_api_ns.doc(description="List all documents in a dataset")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID"})
    @service_api_ns.doc(
        responses={
            200: "Documents retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Dataset not found",
        }
    )
    def get(self, tenant_id, dataset_id):
        dataset_id = str(dataset_id)
        tenant_id = str(tenant_id)
        query_params = DocumentListQuery.model_validate(request.args.to_dict())
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )
        if not dataset:
            raise NotFound("Dataset not found.")

        query = select(Document).filter_by(dataset_id=str(dataset_id), tenant_id=tenant_id)

        if query_params.status:
            query = DocumentService.apply_display_status_filter(query, query_params.status)

        if query_params.keyword:
            search = f"%{query_params.keyword}%"
            query = query.where(Document.name.like(search))

        query = query.order_by(desc(Document.created_at), desc(Document.position))

        paginated_documents = db.paginate(
            select=query, page=query_params.page, per_page=query_params.limit, max_per_page=100, error_out=False
        )
        documents = paginated_documents.items

        DocumentService.enrich_documents_with_summary_index_status(
            documents=documents,
            dataset=dataset,
            tenant_id=tenant_id,
        )

        response = {
            "data": marshal(documents, document_fields),
            "has_more": len(documents) == query_params.limit,
            "limit": query_params.limit,
            "total": paginated_documents.total,
            "page": query_params.page,
        }

        return response


@service_api_ns.route("/datasets/<uuid:dataset_id>/documents/download-zip")
class DocumentBatchDownloadZipApi(DatasetApiResource):
    """Download multiple uploaded-file documents as a single ZIP archive."""

    @service_api_ns.expect(service_api_ns.models[DocumentBatchDownloadZipPayload.__name__])
    @service_api_ns.doc("download_documents_as_zip")
    @service_api_ns.doc(description="Download selected uploaded documents as a single ZIP archive")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID"})
    @service_api_ns.doc(
        responses={
            200: "ZIP archive generated successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - insufficient permissions",
            404: "Document or dataset not found",
        }
    )
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def post(self, tenant_id, dataset_id):
        payload = DocumentBatchDownloadZipPayload.model_validate(service_api_ns.payload or {})

        upload_files, download_name = DocumentService.prepare_document_batch_download_zip(
            dataset_id=str(dataset_id),
            document_ids=[str(document_id) for document_id in payload.document_ids],
            tenant_id=str(tenant_id),
            current_user=current_user,
        )

        with ExitStack() as stack:
            zip_path = stack.enter_context(FileService.build_upload_files_zip_tempfile(upload_files=upload_files))
            response = send_file(
                zip_path,
                mimetype="application/zip",
                as_attachment=True,
                download_name=download_name,
            )
            cleanup = stack.pop_all()
            response.call_on_close(cleanup.close)
        return response


@service_api_ns.route("/datasets/<uuid:dataset_id>/documents/<string:batch>/indexing-status")
class DocumentIndexingStatusApi(DatasetApiResource):
    @service_api_ns.doc("get_document_indexing_status")
    @service_api_ns.doc(description="Get indexing status for documents in a batch")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "batch": "Batch ID"})
    @service_api_ns.doc(
        responses={
            200: "Indexing status retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Dataset or documents not found",
        }
    )
    def get(self, tenant_id, dataset_id, batch):
        dataset_id = str(dataset_id)
        batch = str(batch)
        tenant_id = str(tenant_id)
        # get dataset
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )
        if not dataset:
            raise NotFound("Dataset not found.")
        # get documents
        documents = DocumentService.get_batch_documents(dataset_id, batch)
        if not documents:
            raise NotFound("Documents not found.")
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
                "indexing_status": "paused" if document.is_paused else document.indexing_status,
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
        return data


@service_api_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>/download")
class DocumentDownloadApi(DatasetApiResource):
    """Return a signed download URL for a document's original uploaded file."""

    @service_api_ns.doc("get_document_download_url")
    @service_api_ns.doc(description="Get a signed download URL for a document's original uploaded file")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "document_id": "Document ID"})
    @service_api_ns.doc(
        responses={
            200: "Download URL generated successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - insufficient permissions",
            404: "Document or upload file not found",
        }
    )
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def get(self, tenant_id, dataset_id, document_id):
        dataset = self.get_dataset(str(dataset_id), str(tenant_id))
        document = DocumentService.get_document(dataset.id, str(document_id))

        if not document:
            raise NotFound("Document not found.")

        if document.tenant_id != str(tenant_id):
            raise Forbidden("No permission.")

        return {"url": DocumentService.get_document_download_url(document)}


@service_api_ns.route("/datasets/<uuid:dataset_id>/documents/<uuid:document_id>")
class DocumentApi(DatasetApiResource):
    METADATA_CHOICES = {"all", "only", "without"}

    @service_api_ns.doc("get_document")
    @service_api_ns.doc(description="Get a specific document by ID")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "document_id": "Document ID"})
    @service_api_ns.doc(
        responses={
            200: "Document retrieved successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - insufficient permissions",
            404: "Document not found",
        }
    )
    def get(self, tenant_id, dataset_id, document_id):
        dataset_id = str(dataset_id)
        document_id = str(document_id)

        dataset = self.get_dataset(dataset_id, tenant_id)

        document = DocumentService.get_document(dataset.id, document_id)

        if not document:
            raise NotFound("Document not found.")

        if document.tenant_id != str(tenant_id):
            raise Forbidden("No permission.")

        metadata = request.args.get("metadata", "all")
        if metadata not in self.METADATA_CHOICES:
            raise InvalidMetadataError(f"Invalid metadata value: {metadata}")

        # Calculate summary_index_status if needed
        summary_index_status = None
        has_summary_index = dataset.summary_index_setting and dataset.summary_index_setting.get("enable") is True
        if has_summary_index and document.need_summary is True:
            summary_index_status = SummaryIndexService.get_document_summary_index_status(
                document_id=document_id,
                dataset_id=dataset_id,
                tenant_id=tenant_id,
            )

        if metadata == "only":
            response = {"id": document.id, "doc_type": document.doc_type, "doc_metadata": document.doc_metadata_details}
        elif metadata == "without":
            dataset_process_rules = DatasetService.get_process_rules(dataset_id)
            document_process_rules = document.dataset_process_rule.to_dict() if document.dataset_process_rule else {}
            data_source_info = document.data_source_detail_dict
            response = {
                "id": document.id,
                "position": document.position,
                "data_source_type": document.data_source_type,
                "data_source_info": data_source_info,
                "dataset_process_rule_id": document.dataset_process_rule_id,
                "dataset_process_rule": dataset_process_rules,
                "document_process_rule": document_process_rules,
                "name": document.name,
                "created_from": document.created_from,
                "created_by": document.created_by,
                "created_at": int(document.created_at.timestamp()),
                "tokens": document.tokens,
                "indexing_status": document.indexing_status,
                "completed_at": int(document.completed_at.timestamp()) if document.completed_at else None,
                "updated_at": int(document.updated_at.timestamp()) if document.updated_at else None,
                "indexing_latency": document.indexing_latency,
                "error": document.error,
                "enabled": document.enabled,
                "disabled_at": int(document.disabled_at.timestamp()) if document.disabled_at else None,
                "disabled_by": document.disabled_by,
                "archived": document.archived,
                "segment_count": document.segment_count,
                "average_segment_length": document.average_segment_length,
                "hit_count": document.hit_count,
                "display_status": document.display_status,
                "doc_form": document.doc_form,
                "doc_language": document.doc_language,
                "summary_index_status": summary_index_status,
                "need_summary": document.need_summary if document.need_summary is not None else False,
            }
        else:
            dataset_process_rules = DatasetService.get_process_rules(dataset_id)
            document_process_rules = document.dataset_process_rule.to_dict() if document.dataset_process_rule else {}
            data_source_info = document.data_source_detail_dict
            response = {
                "id": document.id,
                "position": document.position,
                "data_source_type": document.data_source_type,
                "data_source_info": data_source_info,
                "dataset_process_rule_id": document.dataset_process_rule_id,
                "dataset_process_rule": dataset_process_rules,
                "document_process_rule": document_process_rules,
                "name": document.name,
                "created_from": document.created_from,
                "created_by": document.created_by,
                "created_at": int(document.created_at.timestamp()),
                "tokens": document.tokens,
                "indexing_status": document.indexing_status,
                "completed_at": int(document.completed_at.timestamp()) if document.completed_at else None,
                "updated_at": int(document.updated_at.timestamp()) if document.updated_at else None,
                "indexing_latency": document.indexing_latency,
                "error": document.error,
                "enabled": document.enabled,
                "disabled_at": int(document.disabled_at.timestamp()) if document.disabled_at else None,
                "disabled_by": document.disabled_by,
                "archived": document.archived,
                "doc_type": document.doc_type,
                "doc_metadata": document.doc_metadata_details,
                "segment_count": document.segment_count,
                "average_segment_length": document.average_segment_length,
                "hit_count": document.hit_count,
                "display_status": document.display_status,
                "doc_form": document.doc_form,
                "doc_language": document.doc_language,
                "summary_index_status": summary_index_status,
                "need_summary": document.need_summary if document.need_summary is not None else False,
            }

        return response

    @service_api_ns.doc("delete_document")
    @service_api_ns.doc(description="Delete a document")
    @service_api_ns.doc(params={"dataset_id": "Dataset ID", "document_id": "Document ID"})
    @service_api_ns.doc(
        responses={
            204: "Document deleted successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - document is archived",
            404: "Document not found",
        }
    )
    @cloud_edition_billing_rate_limit_check("knowledge", "dataset")
    def delete(self, tenant_id, dataset_id, document_id):
        """Delete document."""
        document_id = str(document_id)
        dataset_id = str(dataset_id)
        tenant_id = str(tenant_id)

        # get dataset info
        dataset = db.session.scalar(
            select(Dataset).where(Dataset.tenant_id == tenant_id, Dataset.id == dataset_id).limit(1)
        )

        if not dataset:
            raise ValueError("Dataset does not exist.")

        document = DocumentService.get_document(dataset.id, document_id)

        # 404 if document not found
        if document is None:
            raise NotFound("Document Not Exists.")

        # 403 if document is archived
        if DocumentService.check_archived(document):
            raise ArchivedDocumentImmutableError()

        try:
            # delete document
            DocumentService.delete_document(document)
        except services.errors.document.DocumentIndexingError:
            raise DocumentIndexingError("Cannot delete document during indexing.")

        return "", 204

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
