You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-013
- **Kind**: external_call
- **Identifier**: RAG Pipeline (Embedding, Retrieval, Rerank, Vector DB)
- **Description**: Retrieval-Augmented Generation pipeline processing user documents through embedding, chunking, and vector storage. Risk of data poisoning via malicious document content, injection through retrieved context, and information disclosure across tenant boundaries.
- **Locations**: api/core/rag/, api/core/rag/embedding/, api/core/rag/retrieval/, api/core/rag/rerank/, api/core/rag/datasource/

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

### api/core/rag/datasource/__init__.py
```py

```

### api/core/rag/datasource/vdb/field.py
```py
from enum import StrEnum, auto
from typing import Any

from pydantic import TypeAdapter

_metadata_adapter: TypeAdapter[dict[str, Any]] = TypeAdapter(dict[str, Any])


def parse_metadata_json(raw: Any) -> dict[str, Any]:
    """Parse metadata from a JSON string or pass through an existing dict.

    Many VDB drivers return metadata as either a JSON string or an already-
    decoded dict depending on the column type and driver version.
    """
    if raw is None or raw in ("", b""):
        return {}
    if isinstance(raw, dict):
        return raw
    if not isinstance(raw, (str, bytes, bytearray)):
        return {}
    return _metadata_adapter.validate_json(raw)


class Field(StrEnum):
    CONTENT_KEY = "page_content"
    METADATA_KEY = "metadata"
    GROUP_KEY = "group_id"
    VECTOR = auto()
    # Sparse Vector aims to support full text search
    SPARSE_VECTOR = auto()
    TEXT_KEY = "text"
    PRIMARY_KEY = "id"
    DOC_ID = "metadata.doc_id"
    DOCUMENT_ID = "metadata.document_id"

```

### api/core/rag/datasource/vdb/hologres/hologres_vector.py
```py
import json
import logging
import time
from typing import Any

import holo_search_sdk as holo  # type: ignore
from holo_search_sdk.types import BaseQuantizationType, DistanceType, TokenizerType
from psycopg import sql as psql
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class HologresVectorConfig(BaseModel):
    """
    Configuration for Hologres vector database connection.

    In Hologres, access_key_id is used as the PostgreSQL username,
    and access_key_secret is used as the PostgreSQL password.
    """

    host: str
    port: int = 80
    database: str
    access_key_id: str
    access_key_secret: str
    schema_name: str = "public"
    tokenizer: TokenizerType = "jieba"
    distance_method: DistanceType = "Cosine"
    base_quantization_type: BaseQuantizationType = "rabitq"
    max_degree: int = 64
    ef_construction: int = 400

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values.get("host"):
            raise ValueError("config HOLOGRES_HOST is required")
        if not values.get("database"):
            raise ValueError("config HOLOGRES_DATABASE is required")
        if not values.get("access_key_id"):
            raise ValueError("config HOLOGRES_ACCESS_KEY_ID is required")
        if not values.get("access_key_secret"):
            raise ValueError("config HOLOGRES_ACCESS_KEY_SECRET is required")
        return values


class HologresVector(BaseVector):
    """
    Hologres vector storage implementation using holo-search-sdk.

    Supports semantic search (vector), full-text search, and hybrid search.
    """

    def __init__(self, collection_name: str, config: HologresVectorConfig):
        super().__init__(collection_name)
        self._config = config
        self._client = self._init_client(config)
        self.table_name = f"embedding_{collection_name}".lower()

    def _init_client(self, config: HologresVectorConfig):
        """Initialize and return a holo-search-sdk client."""
        client = holo.connect(
            host=config.host,
            port=config.port,
            database=config.database,
            access_key_id=config.access_key_id,
            access_key_secret=config.access_key_secret,
            schema=config.schema_name,
        )
        client.connect()
        return client

    def get_type(self) -> str:
        return VectorType.HOLOGRES

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        """Create collection table with vector and full-text indexes, then add texts."""
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        """Add texts with embeddings to the collection using batch upsert."""
        if not documents:
            return []

        pks: list[str] = []
        batch_size = 100
        for i in range(0, len(documents), batch_size):
            batch_docs = documents[i : i + batch_size]
            batch_embeddings = embeddings[i : i + batch_size]

            values = []
            column_names = ["id", "text", "meta", "embedding"]

            for j, doc in enumerate(batch_docs):
                doc_id = doc.metadata.get("doc_id", "") if doc.metadata else ""
                pks.append(doc_id)
                values.append(
                    [
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata or {}),
                        batch_embeddings[j],
                    ]
                )

            table = self._client.open_table(self.table_name)
            table.upsert_multi(
                index_column="id",
                values=values,
                column_names=column_names,
                update=True,
                update_columns=["text", "meta", "embedding"],
            )

        return pks

    def text_exists(self, id: str) -> bool:
        """Check if a text with the given doc_id exists in the collection."""
        if not self._client.check_table_exist(self.table_name):
            return False

        result = self._client.execute(
            psql.SQL("SELECT 1 FROM {} WHERE id = {} LIMIT 1").format(
                psql.Identifier(self.table_name), psql.Literal(id)
            ),
            fetch_result=True,
        )
        return bool(result)

    def get_ids_by_metadata_field(self, key: str, value: str) -> list[str] | None:
        """Get document IDs by metadata field key and value."""
        result = self._client.execute(
            psql.SQL("SELECT id FROM {} WHERE meta->>{} = {}").format(
                psql.Identifier(self.table_name), psql.Literal(key), psql.Literal(value)
            ),
            fetch_result=True,
        )
        if result:
            return [row[0] for row in result]
        return None

    def delete_by_ids(self, ids: list[str]):
        """Delete documents by their doc_id list."""
        if not ids:
            return
        if not self._client.check_table_exist(self.table_name):
            return

        self._client.execute(
            psql.SQL("DELETE FROM {} WHERE id IN ({})").format(
                psql.Identifier(self.table_name),
                psql.SQL(", ").join(psql.Literal(id) for id in ids),
            )
        )

    def delete_by_metadata_field(self, key: str, value: str):
        """Delete documents by metadata field key and value."""
        if not self._client.check_table_exist(self.table_name):
            return

        self._client.execute(
            psql.SQL("DELETE FROM {} WHERE meta->>{} = {}").format(
                psql.Identifier(self.table_name), psql.Literal(key), psql.Literal(value)
            )
        )

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """Search for documents by vector similarity."""
        if not self._client.check_table_exist(self.table_name):
            return []

        top_k = kwargs.get("top_k", 4)
        score_threshold = float(kwargs.get("score_threshold") or 0.0)

        table = self._client.open_table(self.table_name)
        query = (
            table.search_vector(
                vector=query_vector,
                column="embedding",
                distance_method=self._config.distance_method,
                output_name="distance",
            )
            .select(["id", "text", "meta"])
            .limit(top_k)
        )

        # Apply document_ids_filter if provided
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            filter_sql = psql.SQL("meta->>'document_id' IN ({})").format(
                psql.SQL(", ").join(psql.Literal(id) for id in document_ids_filter)
            )
            query = query.where(filter_sql)

        results = query.fetchall()
        return self._process_vector_results(results, score_threshold)

    def _process_vector_results(self, results: list, score_threshold: float) -> list[Document]:
        """Process vector search results into Document objects."""
        docs = []
        for row in results:
            # row format: (distance, id, text, meta)
            # distance is first because search_vector() adds the computed column before selected columns
            distance = row[0]
            text = row[2]
            meta = row[3]

            meta = parse_metadata_json(meta)

            # Convert distance to similarity score (consistent with pgvector)
            score = 1 - distance
            meta["score"] = score

            if score >= score_threshold:
                docs.append(Document(page_content=text, metadata=meta))

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """Search for documents by full-text search."""
        if not self._client.check_table_exist(self.table_name):
            return []

        top_k = kwargs.get("top_k", 4)

        table = self._client.open_table(self.table_name)
        search_query = table.search_text(
            column="text",
            expression=query,
            return_score=True,
            return_score_name="score",
            return_all_columns=True,
        ).limit(top_k)

        # Apply document_ids_filter if provided
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            filter_sql = psql.SQL("meta->>'document_id' IN ({})").format(
                psql.SQL(", ").join(psql.Literal(id) for id in document_ids_filter)
            )
            search_query = search_query.where(filter_sql)

        results = search_query.fetchall()
        return self._process_full_text_results(results)

    def _process_full_text_results(self, results: list) -> list[Document]:
        """Process full-text search results into Document objects."""
        docs = []
        for row in results:
            # row format: (id, text, meta, embedding, score)
            text = row[1]
            meta = row[2]
            score = row[-1]  # score is the last column from return_score

            meta = parse_metadata_json(meta)

            meta["score"] = score
            docs.append(Document(page_content=text, metadata=meta))

        return docs

    def delete(self):
        """Delete the entire collection table."""
        if self._client.check_table_exist(self.table_name):
            self._client.drop_table(self.table_name)

    def _create_collection(self, dimension: int):
        """Create the collection table with vector and full-text indexes."""
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            if not self._client.check_table_exist(self.table_name):
                # Create table via SQL with CHECK constraint for vector dimension
                create_table_sql = psql.SQL("""
                    CREATE TABLE IF NOT EXISTS {} (
                        id TEXT PRIMARY KEY,
                        text TEXT NOT NULL,
                        meta JSONB NOT NULL,
                        embedding float4[] NOT NULL
                            CHECK (array_ndims(embedding) = 1
                                   AND array_length(embedding, 1) = {})
                    );
                """).format(psql.Identifier(self.table_name), psql.Literal(dimension))
                self._client.execute(create_table_sql)

                # Wait for table to be fully ready before creating indexes
                max_wait_seconds = 30
                poll_interval = 2
                for _ in range(max_wait_seconds // poll_interval):
                    if self._client.check_table_exist(self.table_name):
                        break
                    time.sleep(poll_interval)
                else:
                    raise RuntimeError(f"Table {self.table_name} was not ready after {max_wait_seconds}s")

                # Open table and set vector index
                table = self._client.open_table(self.table_name)
                table.set_vector_index(
                    column="embedding",
                    distance_method=self._config.distance_method,
                    base_quantization_type=self._config.base_quantization_type,
                    max_degree=self._config.max_degree,
                    ef_construction=self._config.ef_construction,
                    use_reorder=self._config.base_quantization_type == "rabitq",
                )

                # Create full-text search index
                table.create_text_index(
                    index_name=f"ft_idx_{self._collection_name}",
                    column="text",
                    tokenizer=self._config.tokenizer,
                )

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class HologresVectorFactory(AbstractVectorFactory):
    """Factory class for creating HologresVector instances."""

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> HologresVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.HOLOGRES, collection_name))

        return HologresVector(
            collection_name=collection_name,
            config=HologresVectorConfig(
                host=dify_config.HOLOGRES_HOST or "",
                port=dify_config.HOLOGRES_PORT,
                database=dify_config.HOLOGRES_DATABASE or "",
                access_key_id=dify_config.HOLOGRES_ACCESS_KEY_ID or "",
                access_key_secret=dify_config.HOLOGRES_ACCESS_KEY_SECRET or "",
                schema_name=dify_config.HOLOGRES_SCHEMA,
                tokenizer=dify_config.HOLOGRES_TOKENIZER,
                distance_method=dify_config.HOLOGRES_DISTANCE_METHOD,
                base_quantization_type=dify_config.HOLOGRES_BASE_QUANTIZATION_TYPE,
                max_degree=dify_config.HOLOGRES_MAX_DEGREE,
                ef_construction=dify_config.HOLOGRES_EF_CONSTRUCTION,
            ),
        )

```

### api/core/rag/datasource/vdb/hologres/__init__.py
```py

```

### api/core/rag/datasource/vdb/chroma/__init__.py
```py

```

### api/core/rag/datasource/vdb/chroma/chroma_vector.py
```py
import json
from typing import Any

import chromadb
from chromadb import QueryResult, Settings
from pydantic import BaseModel

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset


class ChromaConfig(BaseModel):
    host: str
    port: int
    tenant: str
    database: str
    auth_provider: str | None = None
    auth_credentials: str | None = None

    def to_chroma_params(self):
        settings = Settings(
            # auth
            chroma_client_auth_provider=self.auth_provider,
            chroma_client_auth_credentials=self.auth_credentials,
        )

        return {
            "host": self.host,
            "port": self.port,
            "ssl": False,
            "tenant": self.tenant,
            "database": self.database,
            "settings": settings,
        }


class ChromaVector(BaseVector):
    def __init__(self, collection_name: str, config: ChromaConfig):
        super().__init__(collection_name)
        self._client_config = config
        self._client = chromadb.HttpClient(**self._client_config.to_chroma_params())

    def get_type(self) -> str:
        return VectorType.CHROMA

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        if texts:
            # create collection
            self.create_collection(self._collection_name)

            self.add_texts(texts, embeddings, **kwargs)

    def create_collection(self, collection_name: str):
        lock_name = f"vector_indexing_lock_{collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            self._client.get_or_create_collection(collection_name)
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs) -> list[str]:
        uuids = self._get_uuids(documents)
        texts = [d.page_content for d in documents]
        metadatas = [d.metadata for d in documents]

        collection = self._client.get_or_create_collection(self._collection_name)
        # FIXME: chromadb using numpy array, fix the type error later
        collection.upsert(ids=uuids, documents=texts, embeddings=embeddings, metadatas=metadatas)  # type: ignore
        return uuids

    def delete_by_metadata_field(self, key: str, value: str):
        collection = self._client.get_or_create_collection(self._collection_name)
        # FIXME: fix the type error later
        collection.delete(where={key: {"$eq": value}})  # type: ignore

    def delete(self):
        self._client.delete_collection(self._collection_name)

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        collection = self._client.get_or_create_collection(self._collection_name)
        collection.delete(ids=ids)

    def text_exists(self, id: str) -> bool:
        collection = self._client.get_or_create_collection(self._collection_name)
        response = collection.get(ids=[id])
        return len(response) > 0

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        collection = self._client.get_or_create_collection(self._collection_name)
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            results: QueryResult = collection.query(
                query_embeddings=query_vector,
                n_results=kwargs.get("top_k", 4),
                where={"document_id": {"$in": document_ids_filter}},  # type: ignore
            )
        else:
            results: QueryResult = collection.query(query_embeddings=query_vector, n_results=kwargs.get("top_k", 4))  # type: ignore
        score_threshold = float(kwargs.get("score_threshold") or 0.0)

        # Check if results contain data
        if not results["ids"] or not results["documents"] or not results["metadatas"] or not results["distances"]:
            return []

        ids = results["ids"][0]
        documents = results["documents"][0]
        metadatas = results["metadatas"][0]
        distances = results["distances"][0]

        docs = []
        for index in range(len(ids)):
            distance = distances[index]
            metadata = dict(metadatas[index])
            score = 1 - distance
            if score >= score_threshold:
                metadata["score"] = score
                doc = Document(
                    page_content=documents[index],
                    metadata=metadata,
                )
                docs.append(doc)
        # Sort the documents by score in descending order
        docs = sorted(docs, key=lambda x: x.metadata["score"] if x.metadata is not None else 0, reverse=True)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        # chroma does not support BM25 full text searching
        return []


class ChromaVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> BaseVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            index_struct_dict = {"type": VectorType.CHROMA, "vector_store": {"class_prefix": collection_name}}
            dataset.index_struct = json.dumps(index_struct_dict)

        return ChromaVector(
            collection_name=collection_name,
            config=ChromaConfig(
                host=dify_config.CHROMA_HOST or "",
                port=dify_config.CHROMA_PORT,
                tenant=dify_config.CHROMA_TENANT or chromadb.DEFAULT_TENANT,
                database=dify_config.CHROMA_DATABASE or chromadb.DEFAULT_DATABASE,
                auth_provider=dify_config.CHROMA_AUTH_PROVIDER,
                auth_credentials=dify_config.CHROMA_AUTH_CREDENTIALS,
            ),
        )

```

### api/core/rag/datasource/vdb/huawei/huawei_cloud_vector.py
```py
import json
import logging
import ssl
from typing import Any

from elasticsearch import Elasticsearch
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


def create_ssl_context() -> ssl.SSLContext:
    ssl_context = ssl.create_default_context()
    ssl_context.check_hostname = False
    ssl_context.verify_mode = ssl.CERT_NONE
    return ssl_context


class HuaweiCloudVectorConfig(BaseModel):
    hosts: str
    username: str | None = None
    password: str | None = None

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["hosts"]:
            raise ValueError("config HOSTS is required")
        return values

    def to_elasticsearch_params(self) -> dict[str, Any]:
        params = {
            "hosts": self.hosts.split(","),
            "verify_certs": False,
            "ssl_show_warn": False,
            "request_timeout": 30000,
            "retry_on_timeout": True,
            "max_retries": 10,
        }
        if self.username and self.password:
            params["basic_auth"] = (self.username, self.password)
        return params


class HuaweiCloudVector(BaseVector):
    def __init__(self, index_name: str, config: HuaweiCloudVectorConfig):
        super().__init__(index_name.lower())
        self._client = Elasticsearch(**config.to_elasticsearch_params())

    def get_type(self) -> str:
        return VectorType.HUAWEI_CLOUD

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)
        for i in range(len(documents)):
            self._client.index(
                index=self._collection_name,
                id=uuids[i],
                document={
                    Field.CONTENT_KEY: documents[i].page_content,
                    Field.VECTOR: embeddings[i] or None,
                    Field.METADATA_KEY: documents[i].metadata or {},
                },
            )
        self._client.indices.refresh(index=self._collection_name)
        return uuids

    def text_exists(self, id: str) -> bool:
        return bool(self._client.exists(index=self._collection_name, id=id))

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        for id in ids:
            self._client.delete(index=self._collection_name, id=id)

    def delete_by_metadata_field(self, key: str, value: str):
        query_str = {"query": {"match": {f"metadata.{key}": f"{value}"}}}
        results = self._client.search(index=self._collection_name, body=query_str)
        ids = [hit["_id"] for hit in results["hits"]["hits"]]
        if ids:
            self.delete_by_ids(ids)

    def delete(self):
        self._client.indices.delete(index=self._collection_name)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)

        query = {
            "size": top_k,
            "query": {
                "vector": {
                    Field.VECTOR: {
                        "vector": query_vector,
                        "topk": top_k,
                    }
                }
            },
        }

        results = self._client.search(index=self._collection_name, body=query)

        docs_and_scores = []
        for hit in results["hits"]["hits"]:
            docs_and_scores.append(
                (
                    Document(
                        page_content=hit["_source"][Field.CONTENT_KEY],
                        vector=hit["_source"][Field.VECTOR],
                        metadata=hit["_source"][Field.METADATA_KEY],
                    ),
                    hit["_score"],
                )
            )

        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        docs = []
        for doc, score in docs_and_scores:
            if score >= score_threshold:
                if doc.metadata is not None:
                    doc.metadata["score"] = score
                    docs.append(doc)

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        query_str = {"match": {Field.CONTENT_KEY: query}}
        results = self._client.search(index=self._collection_name, query=query_str, size=kwargs.get("top_k", 4))
        docs = []
        for hit in results["hits"]["hits"]:
            docs.append(
                Document(
                    page_content=hit["_source"][Field.CONTENT_KEY],
                    vector=hit["_source"][Field.VECTOR],
                    metadata=hit["_source"][Field.METADATA_KEY],
                )
            )

        return docs

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        metadatas = [d.metadata if d.metadata is not None else {} for d in texts]
        self.create_collection(embeddings, metadatas)
        self.add_texts(texts, embeddings, **kwargs)

    def create_collection(
        self,
        embeddings: list[list[float]],
        metadatas: list[dict[Any, Any]] | None = None,
        index_params: dict | None = None,
    ):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name)
                return

            if not self._client.indices.exists(index=self._collection_name):
                dim = len(embeddings[0])
                mappings = {
                    "properties": {
                        Field.CONTENT_KEY: {"type": "text"},
                        Field.VECTOR: {  # Make sure the dimension is correct here
                            "type": "vector",
                            "dimension": dim,
                            "indexing": True,
                            "algorithm": "GRAPH",
                            "metric": "cosine",
                            "neighbors": 32,
                            "efc": 128,
                        },
                        Field.METADATA_KEY: {
                            "type": "object",
                            "properties": {
                                "doc_id": {"type": "keyword"}  # Map doc_id to keyword type
                            },
                        },
                    }
                }
                settings = {"index.vector": True}
                self._client.indices.create(index=self._collection_name, mappings=mappings, settings=settings)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class HuaweiCloudVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> HuaweiCloudVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.HUAWEI_CLOUD, collection_name))

        return HuaweiCloudVector(
            index_name=collection_name,
            config=HuaweiCloudVectorConfig(
                hosts=dify_config.HUAWEI_CLOUD_HOSTS or "http://localhost:9200",
                username=dify_config.HUAWEI_CLOUD_USER,
                password=dify_config.HUAWEI_CLOUD_PASSWORD,
            ),
        )

```

### api/core/rag/datasource/vdb/huawei/__init__.py
```py

```

### api/core/rag/datasource/vdb/baidu/__init__.py
```py

```

### api/core/rag/datasource/vdb/baidu/baidu_vector.py
```py
import json
import logging
import time
import uuid
from typing import Any

import numpy as np
from pydantic import BaseModel, model_validator
from pymochow import MochowClient  # type: ignore
from pymochow.auth.bce_credentials import BceCredentials  # type: ignore
from pymochow.configuration import Configuration  # type: ignore
from pymochow.exception import ServerError  # type: ignore
from pymochow.model.database import Database
from pymochow.model.enum import FieldType, IndexState, IndexType, MetricType, ServerErrCode, TableState  # type: ignore
from pymochow.model.schema import (
    AutoBuildRowCountIncrement,
    Field,
    FilteringIndex,
    HNSWParams,
    InvertedIndex,
    InvertedIndexAnalyzer,
    InvertedIndexFieldAttribute,
    InvertedIndexParams,
    InvertedIndexParseMode,
    Schema,
    VectorIndex,
)  # type: ignore
from pymochow.model.table import AnnSearch, BM25SearchRequest, HNSWSearchParams, Partition, Row  # type: ignore

from configs import dify_config
from core.rag.datasource.vdb.field import Field as VDBField
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class BaiduConfig(BaseModel):
    endpoint: str
    connection_timeout_in_mills: int = 30 * 1000
    account: str
    api_key: str
    database: str
    index_type: str = "HNSW"
    metric_type: str = "IP"
    shard: int = 1
    replicas: int = 3
    inverted_index_analyzer: str = "DEFAULT_ANALYZER"
    inverted_index_parser_mode: str = "COARSE_MODE"
    auto_build_row_count_increment: int = 500
    auto_build_row_count_increment_ratio: float = 0.05
    rebuild_index_timeout_in_seconds: int = 300

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["endpoint"]:
            raise ValueError("config BAIDU_VECTOR_DB_ENDPOINT is required")
        if not values["account"]:
            raise ValueError("config BAIDU_VECTOR_DB_ACCOUNT is required")
        if not values["api_key"]:
            raise ValueError("config BAIDU_VECTOR_DB_API_KEY is required")
        if not values["database"]:
            raise ValueError("config BAIDU_VECTOR_DB_DATABASE is required")
        return values


class BaiduVector(BaseVector):
    vector_index: str = "vector_idx"
    filtering_index: str = "filtering_idx"
    inverted_index: str = "content_inverted_idx"

    def __init__(self, collection_name: str, config: BaiduConfig):
        super().__init__(collection_name)
        self._client_config = config
        self._client = self._init_client(config)
        self._db = self._init_database()

    def get_type(self) -> str:
        return VectorType.BAIDU

    def to_index_struct(self):
        return {"type": self.get_type(), "vector_store": {"class_prefix": self._collection_name}}

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self._create_table(len(embeddings[0]))
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        total_count = len(documents)
        batch_size = 1000

        # upsert texts and embeddings batch by batch
        table = self._db.table(self._collection_name)
        for start in range(0, total_count, batch_size):
            end = min(start + batch_size, total_count)
            rows = []
            for i in range(start, end, 1):
                metadata = documents[i].metadata
                row = Row(
                    id=metadata.get("doc_id", str(uuid.uuid4())),
                    page_content=documents[i].page_content,
                    metadata=metadata,
                    vector=embeddings[i],
                )
                rows.append(row)
            table.upsert(rows=rows)

    def text_exists(self, id: str) -> bool:
        res = self._db.table(self._collection_name).query(primary_key={VDBField.PRIMARY_KEY: id})
        if res and res.code == 0:
            return True
        return False

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        quoted_ids = [f"'{id}'" for id in ids]
        self._db.table(self._collection_name).delete(filter=f"{VDBField.PRIMARY_KEY} IN({', '.join(quoted_ids)})")

    def delete_by_metadata_field(self, key: str, value: str):
        # Escape double quotes in value to prevent injection
        escaped_value = value.replace('"', '\\"')
        self._db.table(self._collection_name).delete(filter=f'metadata["{key}"] = "{escaped_value}"')

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        query_vector = [float(val) if isinstance(val, np.float64) else val for val in query_vector]
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            filter = f'metadata["document_id"] IN({document_ids})'
        anns = AnnSearch(
            vector_field=VDBField.VECTOR,
            vector_floats=query_vector,
            params=HNSWSearchParams(ef=kwargs.get("ef", 20), limit=kwargs.get("top_k", 4)),
            filter=filter,
        )
        res = self._db.table(self._collection_name).search(
            anns=anns,
            projections=[VDBField.CONTENT_KEY, VDBField.METADATA_KEY],
            retrieve_vector=False,
        )
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._get_search_res(res, score_threshold)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        # document ids filter
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            filter = f'metadata["document_id"] IN({document_ids})'

        request = BM25SearchRequest(
            index_name=self.inverted_index, search_text=query, limit=kwargs.get("top_k", 4), filter=filter
        )
        res = self._db.table(self._collection_name).bm25_search(
            request=request, projections=[VDBField.CONTENT_KEY, VDBField.METADATA_KEY]
        )
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._get_search_res(res, score_threshold)

    def _get_search_res(self, res, score_threshold) -> list[Document]:
        docs = []
        for row in res.rows:
            row_data = row.get("row", {})
            score = row.get("score", 0.0)
            meta = row_data.get(VDBField.METADATA_KEY, {})

            try:
                meta = parse_metadata_json(meta)
            except (ValueError, TypeError):
                meta = {}

            if score >= score_threshold:
                meta["score"] = score
                doc = Document(page_content=row_data.get(VDBField.CONTENT_KEY), metadata=meta)
                docs.append(doc)
        return docs

    def delete(self):
        try:
            self._db.drop_table(table_name=self._collection_name)
        except ServerError as e:
            if e.code == ServerErrCode.TABLE_NOT_EXIST:
                pass
            else:
                raise

    def _init_client(self, config) -> MochowClient:
        config = Configuration(
            credentials=BceCredentials(config.account, config.api_key),
            endpoint=config.endpoint,
            connection_timeout_in_mills=config.connection_timeout_in_mills,
        )
        client = MochowClient(config)
        return client

    def _init_database(self) -> Database:
        exists = False
        for db in self._client.list_databases():
            if db.database_name == self._client_config.database:
                exists = True
                break
        # Create database if not existed
        if exists:
            return self._client.database(self._client_config.database)
        else:
            try:
                self._client.create_database(database_name=self._client_config.database)
            except ServerError as e:
                if e.code == ServerErrCode.DB_ALREADY_EXIST:
                    return self._client.database(self._client_config.database)
                else:
                    raise
            return self._client.database(self._client_config.database)

    def _table_existed(self) -> bool:
        try:
            table = self._db.table(self._collection_name)
        except ServerError as e:
            if e.code == ServerErrCode.TABLE_NOT_EXIST:
                return False
            else:
                raise
        return True

    def _create_table(self, dimension: int):
        # Try to grab distributed lock and create table
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=60):
            table_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(table_exist_cache_key):
                return

            if self._table_existed():
                return

            self.delete()

            # check IndexType and MetricType
            index_type = None
            for k, v in IndexType.__members__.items():
                if k == self._client_config.index_type:
                    index_type = v
            if index_type is None:
                raise ValueError("unsupported index_type")
            metric_type = None
            for k, v in MetricType.__members__.items():
                if k == self._client_config.metric_type:
                    metric_type = v
            if metric_type is None:
                raise ValueError("unsupported metric_type")

            # Construct field schema
            fields = []
            fields.append(
                Field(
                    VDBField.PRIMARY_KEY,
                    FieldType.STRING,
                    primary_key=True,
                    partition_key=True,
                    auto_increment=False,
                    not_null=True,
                )
            )
            fields.append(Field(VDBField.CONTENT_KEY, FieldType.TEXT, not_null=False))
            fields.append(Field(VDBField.METADATA_KEY, FieldType.JSON, not_null=False))
            fields.append(Field(VDBField.VECTOR, FieldType.FLOAT_VECTOR, not_null=True, dimension=dimension))

            # Construct vector index params
            indexes = []
            indexes.append(
                VectorIndex(
                    index_name=self.vector_index,
                    index_type=index_type,
                    field=VDBField.VECTOR,
                    metric_type=metric_type,
                    params=HNSWParams(m=16, efconstruction=200),
                    auto_build=True,
                    auto_build_index_policy=AutoBuildRowCountIncrement(
                        row_count_increment=self._client_config.auto_build_row_count_increment,
                        row_count_increment_ratio=self._client_config.auto_build_row_count_increment_ratio,
                    ),
                )
            )

            # Filtering index
            indexes.append(
                FilteringIndex(
                    index_name=self.filtering_index,
                    fields=[VDBField.METADATA_KEY],
                )
            )

            # Get analyzer and parse_mode from config
            analyzer = getattr(
                InvertedIndexAnalyzer,
                self._client_config.inverted_index_analyzer,
                InvertedIndexAnalyzer.DEFAULT_ANALYZER,
            )

            parse_mode = getattr(
                InvertedIndexParseMode,
                self._client_config.inverted_index_parser_mode,
                InvertedIndexParseMode.COARSE_MODE,
            )

            # Inverted index
            indexes.append(
                InvertedIndex(
                    index_name=self.inverted_index,
                    fields=[VDBField.CONTENT_KEY],
                    params=InvertedIndexParams(
                        analyzer=analyzer,
                        parse_mode=parse_mode,
                        case_sensitive=True,
                    ),
                    field_attributes=[InvertedIndexFieldAttribute.ANALYZED],
                )
            )

            # Create table
            self._db.create_table(
                table_name=self._collection_name,
                replication=self._client_config.replicas,
                partition=Partition(partition_num=self._client_config.shard),
                schema=Schema(fields=fields, indexes=indexes),
                description="Table for Dify",
            )

            # Wait for table created
            timeout = self._client_config.rebuild_index_timeout_in_seconds  # default 5 minutes timeout
            start_time = time.time()
            while True:
                time.sleep(1)
                table = self._db.describe_table(self._collection_name)
                if table.state == TableState.NORMAL:
                    break
                if time.time() - start_time > timeout:
                    raise TimeoutError(f"Table creation timeout after {timeout} seconds")
            redis_client.set(table_exist_cache_key, 1, ex=3600)
            # rebuild vector index immediately after table created, make sure index is ready
            table.rebuild_index(self.vector_index)
            timeout = 3600  # 1 hour timeout
            self._wait_for_index_ready(table, timeout)

    def _wait_for_index_ready(self, table, timeout: int = 3600):
        start_time = time.time()
        while True:
            time.sleep(1)
            index = table.describe_index(self.vector_index)
            if index.state == IndexState.NORMAL:
                break
            if time.time() - start_time > timeout:
                raise TimeoutError(f"Index rebuild timeout after {timeout} seconds")


class BaiduVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> BaiduVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.BAIDU, collection_name))

        return BaiduVector(
            collection_name=collection_name,
            config=BaiduConfig(
                endpoint=dify_config.BAIDU_VECTOR_DB_ENDPOINT or "",
                connection_timeout_in_mills=dify_config.BAIDU_VECTOR_DB_CONNECTION_TIMEOUT_MS,
                account=dify_config.BAIDU_VECTOR_DB_ACCOUNT or "",
                api_key=dify_config.BAIDU_VECTOR_DB_API_KEY or "",
                database=dify_config.BAIDU_VECTOR_DB_DATABASE or "",
                shard=dify_config.BAIDU_VECTOR_DB_SHARD,
                replicas=dify_config.BAIDU_VECTOR_DB_REPLICAS,
                inverted_index_analyzer=dify_config.BAIDU_VECTOR_DB_INVERTED_INDEX_ANALYZER,
                inverted_index_parser_mode=dify_config.BAIDU_VECTOR_DB_INVERTED_INDEX_PARSER_MODE,
                auto_build_row_count_increment=dify_config.BAIDU_VECTOR_DB_AUTO_BUILD_ROW_COUNT_INCREMENT,
                auto_build_row_count_increment_ratio=dify_config.BAIDU_VECTOR_DB_AUTO_BUILD_ROW_COUNT_INCREMENT_RATIO,
                rebuild_index_timeout_in_seconds=dify_config.BAIDU_VECTOR_DB_REBUILD_INDEX_TIMEOUT_IN_SECONDS,
            ),
        )

```

### api/core/rag/datasource/vdb/iris/iris_vector.py
```py
"""InterSystems IRIS vector database implementation for Dify.

This module provides vector storage and retrieval using IRIS native VECTOR type
with HNSW indexing for efficient similarity search.
"""

from __future__ import annotations

import json
import logging
import threading
import uuid
from contextlib import contextmanager
from typing import TYPE_CHECKING, Any

from configs import dify_config
from configs.middleware.vdb.iris_config import IrisVectorConfig
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

if TYPE_CHECKING:
    import iris
else:
    try:
        import iris
    except ImportError:
        iris = None  # type: ignore[assignment]

logger = logging.getLogger(__name__)

# Singleton connection pool to minimize IRIS license usage
_pool_lock = threading.Lock()
_pool_instance: IrisConnectionPool | None = None


def get_iris_pool(config: IrisVectorConfig) -> IrisConnectionPool:
    """Get or create the global IRIS connection pool (singleton pattern)."""
    global _pool_instance  # pylint: disable=global-statement
    with _pool_lock:
        if _pool_instance is None:
            logger.info("Initializing IRIS connection pool")
            _pool_instance = IrisConnectionPool(config)
        return _pool_instance


class IrisConnectionPool:
    """Thread-safe connection pool for IRIS database."""

    def __init__(self, config: IrisVectorConfig) -> None:
        self.config = config
        self._pool: list[Any] = []
        self._lock = threading.Lock()
        self._min_size = config.IRIS_MIN_CONNECTION
        self._max_size = config.IRIS_MAX_CONNECTION
        self._in_use = 0
        self._schemas_initialized: set[str] = set()  # Cache for initialized schemas
        self._initialize_pool()

    def _initialize_pool(self) -> None:
        for _ in range(self._min_size):
            self._pool.append(self._create_connection())

    def _create_connection(self) -> Any:
        return iris.connect(
            hostname=self.config.IRIS_HOST,
            port=self.config.IRIS_SUPER_SERVER_PORT,
            namespace=self.config.IRIS_DATABASE,
            username=self.config.IRIS_USER,
            password=self.config.IRIS_PASSWORD,
        )

    def get_connection(self) -> Any:
        """Get a connection from pool or create new if available."""
        with self._lock:
            if self._pool:
                conn = self._pool.pop()
                self._in_use += 1
                return conn
            if self._in_use < self._max_size:
                conn = self._create_connection()
                self._in_use += 1
                return conn
            raise RuntimeError("Connection pool exhausted")

    def return_connection(self, conn: Any) -> None:
        """Return connection to pool after validating it."""
        if not conn:
            return

        # Validate connection health
        is_valid = False
        try:
            cursor = conn.cursor()
            cursor.execute("SELECT 1")
            cursor.close()
            is_valid = True
        except (OSError, RuntimeError) as e:
            logger.debug("Connection validation failed: %s", e)
            try:
                conn.close()
            except (OSError, RuntimeError):
                pass

        with self._lock:
            self._pool.append(conn if is_valid else self._create_connection())
            self._in_use -= 1

    def ensure_schema_exists(self, schema: str) -> None:
        """Ensure schema exists in IRIS database.

        This method is idempotent and thread-safe. It uses a memory cache to avoid
        redundant database queries for already-verified schemas.

        Args:
            schema: Schema name to ensure exists

        Raises:
            Exception: If schema creation fails
        """
        # Fast path: check cache first (no lock needed for read-only set lookup)
        if schema in self._schemas_initialized:
            return

        # Slow path: acquire lock and check again (double-checked locking)
        with self._lock:
            if schema in self._schemas_initialized:
                return

            # Get a connection to check/create schema
            conn = self._pool[0] if self._pool else self._create_connection()
            cursor = conn.cursor()
            try:
                # Check if schema exists using INFORMATION_SCHEMA
                check_sql = """
                    SELECT COUNT(*) FROM INFORMATION_SCHEMA.SCHEMATA
                    WHERE SCHEMA_NAME = ?
                """
                cursor.execute(check_sql, (schema,))  # Must be tuple or list
                exists = cursor.fetchone()[0] > 0

                if not exists:
                    # Schema doesn't exist, create it
                    cursor.execute(f"CREATE SCHEMA {schema}")
                    conn.commit()
                    logger.info("Created schema: %s", schema)
                else:
                    logger.debug("Schema already exists: %s", schema)

                # Add to cache to skip future checks
                self._schemas_initialized.add(schema)

            except Exception:
                conn.rollback()
                logger.exception("Failed to ensure schema %s exists", schema)
                raise
            finally:
                cursor.close()

    def close_all(self) -> None:
        """Close all connections (application shutdown only)."""
        with self._lock:
            for conn in self._pool:
                try:
                    conn.close()
                except (OSError, RuntimeError):
                    pass
            self._pool.clear()
            self._in_use = 0
            self._schemas_initialized.clear()


class IrisVector(BaseVector):
    """IRIS vector database implementation using native VECTOR type and HNSW indexing."""

    # Fallback score for full-text search when Rank function unavailable or TEXT_INDEX disabled
    _FULL_TEXT_FALLBACK_SCORE = 0.5

    def __init__(self, collection_name: str, config: IrisVectorConfig) -> None:
        super().__init__(collection_name)
        self.config = config
        self.table_name = f"embedding_{collection_name}".upper()
        self.schema = config.IRIS_SCHEMA or "dify"
        self.pool = get_iris_pool(config)

    def get_type(self) -> str:
        return VectorType.IRIS

    @contextmanager
    def _get_cursor(self):
        """Context manager for database cursor with connection pooling."""
        conn = self.pool.get_connection()
        cursor = conn.cursor()
        try:
            yield cursor
            conn.commit()
        except Exception:
            conn.rollback()
            raise
        finally:
            cursor.close()
            self.pool.return_connection(conn)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs) -> list[str]:
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **_kwargs) -> list[str]:
        """Add documents with embeddings to the collection."""
        added_ids = []
        with self._get_cursor() as cursor:
            for i, doc in enumerate(documents):
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4())) if doc.metadata else str(uuid.uuid4())
                metadata = json.dumps(doc.metadata) if doc.metadata else "{}"
                embedding_str = json.dumps(embeddings[i])

                sql = f"INSERT INTO {self.schema}.{self.table_name} (id, text, meta, embedding) VALUES (?, ?, ?, ?)"
                cursor.execute(sql, (doc_id, doc.page_content, metadata, embedding_str))
                added_ids.append(doc_id)

        return added_ids

    def text_exists(self, id: str) -> bool:  # pylint: disable=redefined-builtin
        try:
            with self._get_cursor() as cursor:
                sql = f"SELECT 1 FROM {self.schema}.{self.table_name} WHERE id = ?"
                cursor.execute(sql, (id,))
                return cursor.fetchone() is not None
        except (OSError, RuntimeError, ValueError):
            return False

    def delete_by_ids(self, ids: list[str]) -> None:
        if not ids:
            return

        with self._get_cursor() as cursor:
            placeholders = ",".join(["?" for _ in ids])
            sql = f"DELETE FROM {self.schema}.{self.table_name} WHERE id IN ({placeholders})"
            cursor.execute(sql, ids)

    def delete_by_metadata_field(self, key: str, value: str) -> None:
        """Delete documents by metadata field (JSON LIKE pattern matching)."""
        with self._get_cursor() as cursor:
            pattern = f'%"{key}": "{value}"%'
            sql = f"DELETE FROM {self.schema}.{self.table_name} WHERE meta LIKE ?"
            cursor.execute(sql, (pattern,))

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """Search similar documents using VECTOR_COSINE with HNSW index."""
        top_k = kwargs.get("top_k", 4)
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        embedding_str = json.dumps(query_vector)

        with self._get_cursor() as cursor:
            sql = f"""
                SELECT TOP {top_k} id, text, meta, VECTOR_COSINE(embedding, ?) as score
                FROM {self.schema}.{self.table_name}
                ORDER BY score DESC
            """
            cursor.execute(sql, (embedding_str,))

            docs = []
            for row in cursor.fetchall():
                if len(row) >= 4:
                    text, meta_str, score = row[1], row[2], float(row[3])
                    if score >= score_threshold:
                        metadata = parse_metadata_json(meta_str)
                        metadata["score"] = score
                        docs.append(Document(page_content=text, metadata=metadata))
            return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """Search documents by full-text using iFind index with BM25 relevance scoring.

        When IRIS_TEXT_INDEX is enabled, this method uses the auto-generated Rank
        function from %iFind.Index.Basic to calculate BM25 relevance scores. The Rank
        function is automatically created with naming: {schema}.{table_name}_{index}Rank

        Args:
            query: Search query string
            **kwargs: Optional parameters including top_k, document_ids_filter

        Returns:
            List of Document objects with relevance scores in metadata["score"]
        """
        top_k = kwargs.get("top_k", 5)
        document_ids_filter = kwargs.get("document_ids_filter")

        with self._get_cursor() as cursor:
            if self.config.IRIS_TEXT_INDEX:
                # Use iFind full-text search with auto-generated Rank function
                text_index_name = f"idx_{self.table_name}_text"
                # IRIS removes underscores from function names
                table_no_underscore = self.table_name.replace("_", "")
                index_no_underscore = text_index_name.replace("_", "")
                rank_function = f"{self.schema}.{table_no_underscore}_{index_no_underscore}Rank"

                # Build WHERE clause with document ID filter if provided
                where_clause = f"WHERE %ID %FIND search_index({text_index_name}, ?)"
                # First param for Rank function, second for FIND
                params = [query, query]

                if document_ids_filter:
                    # Add document ID filter
                    placeholders = ",".join("?" * len(document_ids_filter))
                    where_clause += f" AND JSON_VALUE(meta, '$.document_id') IN ({placeholders})"
                    params.extend(document_ids_filter)

                sql = f"""
                    SELECT TOP {top_k}
                        id,
                        text,
                        meta,
                        {rank_function}(%ID, ?) AS score
                    FROM {self.schema}.{self.table_name}
                    {where_clause}
                    ORDER BY score DESC
                """

                logger.debug(
                    "iFind search: query='%s', index='%s', rank='%s'",
                    query,
                    text_index_name,
                    rank_function,
                )

                try:
                    cursor.execute(sql, params)
                except Exception:  # pylint: disable=broad-exception-caught
                    # Fallback to query without Rank function if it fails
                    logger.warning(
                        "Rank function '%s' failed, using fixed score",
                        rank_function,
                        exc_info=True,
                    )
                    sql_fallback = f"""
                        SELECT TOP {top_k} id, text, meta, {self._FULL_TEXT_FALLBACK_SCORE} AS score
                        FROM {self.schema}.{self.table_name}
                        {where_clause}
                    """
                    # Skip first param (for Rank function)
                    cursor.execute(sql_fallback, params[1:])
            else:
                # Fallback to LIKE search (IRIS_TEXT_INDEX disabled)
                from libs.helper import (  # pylint: disable=import-outside-toplevel
                    escape_like_pattern,
                )

                escaped_query = escape_like_pattern(query)
                query_pattern = f"%{escaped_query}%"

                # Build WHERE clause with document ID filter if provided
                where_clause = "WHERE text LIKE ? ESCAPE '\\\\'"
                params = [query_pattern]

                if document_ids_filter:
                    placeholders = ",".join("?" * len(document_ids_filter))
                    where_clause += f" AND JSON_VALUE(meta, '$.document_id') IN ({placeholders})"
                    params.extend(document_ids_filter)

                sql = f"""
                    SELECT TOP {top_k} id, text, meta, {self._FULL_TEXT_FALLBACK_SCORE} AS score
                    FROM {self.schema}.{self.table_name}
                    {where_clause}
                    ORDER BY LENGTH(text) ASC
                """

                logger.debug(
                    "LIKE fallback (TEXT_INDEX disabled): query='%s'",
                    query_pattern,
                )
                cursor.execute(sql, params)

            docs = []
            for row in cursor.fetchall():
                # Expecting 4 columns: id, text, meta, score
                if len(row) >= 4:
                    text_content = row[1]
                    meta_str = row[2]
                    score_value = row[3]

                    metadata = parse_metadata_json(meta_str)
                    # Add score to metadata for hybrid search compatibility
                    score = float(score_value) if score_value is not None else 0.0
                    metadata["score"] = score

                    docs.append(Document(page_content=text_content, metadata=metadata))

            logger.info(
                "Full-text search completed: query='%s', results=%d/%d",
                query,
                len(docs),
                top_k,
            )

            if not docs:
                logger.warning("Full-text search for '%s' returned no results", query)

            return docs

    def delete(self) -> None:
        """Delete the entire collection (drop table - permanent)."""
        with self._get_cursor() as cursor:
            sql = f"DROP TABLE {self.schema}.{self.table_name}"
            cursor.execute(sql)

    def _create_collection(self, dimension: int) -> None:
        """Create table with VECTOR column and HNSW index.

        Uses Redis lock to prevent concurrent creation attempts across multiple
        API server instances (api, worker, worker_beat).
        """
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"

        with redis_client.lock(lock_name, timeout=20):  # pylint: disable=not-context-manager
            if redis_client.get(cache_key):
                return

            # Ensure schema exists (idempotent, cached after first call)
            self.pool.ensure_schema_exists(self.schema)

            with self._get_cursor() as cursor:
                # Create table with VECTOR column
                sql = f"""
                    CREATE TABLE {self.schema}.{self.table_name} (
                        id VARCHAR(255) PRIMARY KEY,
                        text CLOB,
                        meta CLOB,
                        embedding VECTOR(DOUBLE, {dimension})
                    )
                """
                logger.info("Creating table: %s.%s", self.schema, self.table_name)
                cursor.execute(sql)

                # Create HNSW index for vector similarity search
                index_name = f"idx_{self.table_name}_embedding"
                sql_index = (
                    f"CREATE INDEX {index_name} ON {self.schema}.{self.table_name} "
                    "(embedding) AS HNSW(Distance='Cosine')"
                )
                logger.info("Creating HNSW index: %s", index_name)
                cursor.execute(sql_index)
                logger.info("HNSW index created successfully: %s", index_name)

                # Create full-text search index if enabled
                logger.info(
                    "IRIS_TEXT_INDEX config value: %s (type: %s)",
                    self.config.IRIS_TEXT_INDEX,
                    type(self.config.IRIS_TEXT_INDEX),
                )
                if self.config.IRIS_TEXT_INDEX:
                    text_index_name = f"idx_{self.table_name}_text"
                    language = self.config.IRIS_TEXT_INDEX_LANGUAGE
                    # Fixed: Removed extra parentheses and corrected syntax
                    sql_text_index = f"""
                        CREATE INDEX {text_index_name} ON {self.schema}.{self.table_name} (text)
                        AS %iFind.Index.Basic
                        (LANGUAGE = '{language}', LOWER = 1, INDEXOPTION = 0)
                    """
                    logger.info(
                        "Creating text index: %s with language: %s",
                        text_index_name,
                        language,
                    )
                    logger.info("SQL for text index: %s", sql_text_index)
                    cursor.execute(sql_text_index)
                    logger.info("Text index created successfully: %s", text_index_name)
                else:
                    logger.warning("Text index creation skipped - IRIS_TEXT_INDEX is disabled")

            redis_client.set(cache_key, 1, ex=3600)


class IrisVectorFactory(AbstractVectorFactory):
    """Factory for creating IrisVector instances."""

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> IrisVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            index_struct_dict = self.gen_index_struct_dict(VectorType.IRIS, collection_name)
            dataset.index_struct = json.dumps(index_struct_dict)

        return IrisVector(
            collection_name=collection_name,
            config=IrisVectorConfig(
                IRIS_HOST=dify_config.IRIS_HOST,
                IRIS_SUPER_SERVER_PORT=dify_config.IRIS_SUPER_SERVER_PORT,
                IRIS_USER=dify_config.IRIS_USER,
                IRIS_PASSWORD=dify_config.IRIS_PASSWORD,
                IRIS_DATABASE=dify_config.IRIS_DATABASE,
                IRIS_SCHEMA=dify_config.IRIS_SCHEMA,
                IRIS_CONNECTION_URL=dify_config.IRIS_CONNECTION_URL,
                IRIS_MIN_CONNECTION=dify_config.IRIS_MIN_CONNECTION,
                IRIS_MAX_CONNECTION=dify_config.IRIS_MAX_CONNECTION,
                IRIS_TEXT_INDEX=dify_config.IRIS_TEXT_INDEX,
                IRIS_TEXT_INDEX_LANGUAGE=dify_config.IRIS_TEXT_INDEX_LANGUAGE,
            ),
        )

```

### api/core/rag/datasource/vdb/iris/__init__.py
```py

```

### api/core/rag/datasource/vdb/opengauss/__init__.py
```py

```

### api/core/rag/datasource/vdb/opengauss/opengauss.py
```py
import json
import uuid
from contextlib import contextmanager
from typing import Any

import psycopg2.extras
import psycopg2.pool
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset


class OpenGaussConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    min_connection: int
    max_connection: int
    enable_pq: bool = False  # Enable PQ acceleration

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config OPENGAUSS_HOST is required")
        if not values["port"]:
            raise ValueError("config OPENGAUSS_PORT is required")
        if not values["user"]:
            raise ValueError("config OPENGAUSS_USER is required")
        if not values["password"]:
            raise ValueError("config OPENGAUSS_PASSWORD is required")
        if not values["database"]:
            raise ValueError("config OPENGAUSS_DATABASE is required")
        if not values["min_connection"]:
            raise ValueError("config OPENGAUSS_MIN_CONNECTION is required")
        if not values["max_connection"]:
            raise ValueError("config OPENGAUSS_MAX_CONNECTION is required")
        if values["min_connection"] > values["max_connection"]:
            raise ValueError("config OPENGAUSS_MIN_CONNECTION should less than OPENGAUSS_MAX_CONNECTION")
        return values


SQL_CREATE_TABLE = """
CREATE TABLE IF NOT EXISTS {table_name} (
    id UUID PRIMARY KEY,
    text TEXT NOT NULL,
    meta JSONB NOT NULL,
    embedding vector({dimension}) NOT NULL
);
"""

SQL_CREATE_INDEX_PQ = """
CREATE INDEX IF NOT EXISTS embedding_{table_name}_pq_idx ON {table_name}
USING hnsw (embedding vector_cosine_ops) WITH (m = 16, ef_construction = 64, enable_pq=on, pq_m={pq_m});
"""

SQL_CREATE_INDEX = """
CREATE INDEX IF NOT EXISTS embedding_cosine_{table_name}_idx ON {table_name}
USING hnsw (embedding vector_cosine_ops) WITH (m = 16, ef_construction = 64);
"""


class OpenGauss(BaseVector):
    def __init__(self, collection_name: str, config: OpenGaussConfig):
        super().__init__(collection_name)
        self.pool = self._create_connection_pool(config)
        self.table_name = f"embedding_{collection_name}"
        self.pq_enabled = config.enable_pq

    def get_type(self) -> str:
        return VectorType.OPENGAUSS

    def _create_connection_pool(self, config: OpenGaussConfig):
        return psycopg2.pool.SimpleConnectionPool(
            config.min_connection,
            config.max_connection,
            host=config.host,
            port=config.port,
            user=config.user,
            password=config.password,
            database=config.database,
        )

    @contextmanager
    def _get_cursor(self):
        conn = self.pool.getconn()
        cur = conn.cursor()
        try:
            yield cur
        finally:
            cur.close()
            conn.commit()
            self.pool.putconn(conn)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        self.add_texts(texts, embeddings)
        self._create_index(dimension)

    def _create_index(self, dimension: int):
        index_cache_key = f"vector_index_{self._collection_name}"
        lock_name = f"{index_cache_key}_lock"
        with redis_client.lock(lock_name, timeout=60):
            index_exist_cache_key = f"vector_index_{self._collection_name}"
            if redis_client.get(index_exist_cache_key):
                return

            with self._get_cursor() as cur:
                if dimension <= 2000:
                    if self.pq_enabled:
                        cur.execute(SQL_CREATE_INDEX_PQ.format(table_name=self.table_name, pq_m=int(dimension / 4)))
                        cur.execute("SET hnsw_earlystop_threshold = 320")

                    if not self.pq_enabled:
                        cur.execute(SQL_CREATE_INDEX.format(table_name=self.table_name))
            redis_client.set(index_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        pks = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                pks.append(doc_id)
                values.append(
                    (
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata),
                        embeddings[i],
                    )
                )
        with self._get_cursor() as cur:
            psycopg2.extras.execute_values(
                cur, f"INSERT INTO {self.table_name} (id, text, meta, embedding) VALUES %s", values
            )
        return pks

    def text_exists(self, id: str) -> bool:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT id FROM {self.table_name} WHERE id = %s", (id,))
            return cur.fetchone() is not None

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT meta, text FROM {self.table_name} WHERE id IN %s", (tuple(ids),))
            docs = []
            for record in cur:
                docs.append(Document(page_content=record[1], metadata=record[0]))
        return docs

    def delete_by_ids(self, ids: list[str]):
        # Avoiding crashes caused by performing delete operations on empty lists in certain scenarios
        # Scenario 1: extract a document fails, resulting in a table not being created.
        # Then clicking the retry button triggers a delete operation on an empty list.
        if not ids:
            return
        with self._get_cursor() as cur:
            cur.execute(f"DELETE FROM {self.table_name} WHERE id IN %s", (tuple(ids),))

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_cursor() as cur:
            cur.execute(f"DELETE FROM {self.table_name} WHERE meta->>%s = %s", (key, value))

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search the nearest neighbors to a vector.

        :param query_vector: The input vector to search for similar items.
        :return: List of Documents that are nearest to the query vector.
        """
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        with self._get_cursor() as cur:
            cur.execute(
                f"SELECT meta, text, embedding <=> %s AS distance FROM {self.table_name}"
                f" ORDER BY distance LIMIT {top_k}",
                (json.dumps(query_vector),),
            )
            docs = []
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            for record in cur:
                metadata, text, distance = record
                score = 1 - distance
                metadata["score"] = score
                if score >= score_threshold:
                    docs.append(Document(page_content=text, metadata=metadata))
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 5)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        with self._get_cursor() as cur:
            cur.execute(
                f"""SELECT meta, text, ts_rank(to_tsvector(coalesce(text, '')), plainto_tsquery(%s)) AS score
                FROM {self.table_name}
                WHERE to_tsvector(text) @@ plainto_tsquery(%s)
                ORDER BY score DESC
                LIMIT {top_k}""",
                # f"'{query}'" is required in order to account for whitespace in query
                (f"'{query}'", f"'{query}'"),
            )

            docs = []

            for record in cur:
                metadata, text, score = record
                metadata["score"] = score
                docs.append(Document(page_content=text, metadata=metadata))

        return docs

    def delete(self):
        with self._get_cursor() as cur:
            cur.execute(f"DROP TABLE IF EXISTS {self.table_name}")

    def _create_collection(self, dimension: int):
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            with self._get_cursor() as cur:
                cur.execute(SQL_CREATE_TABLE.format(table_name=self.table_name, dimension=dimension))
            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class OpenGaussFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> OpenGauss:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.OPENGAUSS, collection_name))

        return OpenGauss(
            collection_name=collection_name,
            config=OpenGaussConfig(
                host=dify_config.OPENGAUSS_HOST or "localhost",
                port=dify_config.OPENGAUSS_PORT,
                user=dify_config.OPENGAUSS_USER or "postgres",
                password=dify_config.OPENGAUSS_PASSWORD or "",
                database=dify_config.OPENGAUSS_DATABASE or "dify",
                min_connection=dify_config.OPENGAUSS_MIN_CONNECTION,
                max_connection=dify_config.OPENGAUSS_MAX_CONNECTION,
                enable_pq=dify_config.OPENGAUSS_ENABLE_PQ or False,
            ),
        )

```

### api/core/rag/datasource/vdb/opensearch/opensearch_vector.py
```py
import json
import logging
from typing import Any
from uuid import uuid4

from opensearchpy import OpenSearch, Urllib3AWSV4SignerAuth, Urllib3HttpConnection, helpers
from opensearchpy.helpers import BulkIndexError
from pydantic import BaseModel, model_validator

from configs import dify_config
from configs.middleware.vdb.opensearch_config import AuthMethod
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class OpenSearchConfig(BaseModel):
    host: str
    port: int
    secure: bool = False  # use_ssl
    verify_certs: bool = True
    auth_method: AuthMethod = AuthMethod.BASIC
    user: str | None = None
    password: str | None = None
    aws_region: str | None = None
    aws_service: str | None = None

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values.get("host"):
            raise ValueError("config OPENSEARCH_HOST is required")
        if not values.get("port"):
            raise ValueError("config OPENSEARCH_PORT is required")
        if values.get("auth_method") == "aws_managed_iam":
            if not values.get("aws_region"):
                raise ValueError("config OPENSEARCH_AWS_REGION is required for AWS_MANAGED_IAM auth method")
            if not values.get("aws_service"):
                raise ValueError("config OPENSEARCH_AWS_SERVICE is required for AWS_MANAGED_IAM auth method")
        if not values.get("OPENSEARCH_SECURE") and values.get("OPENSEARCH_VERIFY_CERTS"):
            raise ValueError("verify_certs=True requires secure (HTTPS) connection")
        return values

    def create_aws_managed_iam_auth(self) -> Urllib3AWSV4SignerAuth:
        import boto3

        return Urllib3AWSV4SignerAuth(
            credentials=boto3.Session().get_credentials(),
            region=self.aws_region,
            service=self.aws_service,  # type: ignore[arg-type]
        )

    def to_opensearch_params(self) -> dict[str, Any]:
        params = {
            "hosts": [{"host": self.host, "port": self.port}],
            "use_ssl": self.secure,
            "verify_certs": self.verify_certs,
            "connection_class": Urllib3HttpConnection,
            "pool_maxsize": 20,
        }

        if self.auth_method == "basic":
            logger.info("Using basic authentication for OpenSearch Vector DB")

            params["http_auth"] = (self.user, self.password)
        elif self.auth_method == "aws_managed_iam":
            logger.info("Using AWS managed IAM role for OpenSearch Vector DB")

            params["http_auth"] = self.create_aws_managed_iam_auth()

        return params


class OpenSearchVector(BaseVector):
    def __init__(self, collection_name: str, config: OpenSearchConfig):
        super().__init__(collection_name)
        self._client_config = config
        self._client = OpenSearch(**config.to_opensearch_params())

    def get_type(self) -> str:
        return VectorType.OPENSEARCH

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        metadatas = [d.metadata if d.metadata is not None else {} for d in texts]
        self.create_collection(embeddings, metadatas)
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        actions = []
        for i in range(len(documents)):
            action = {
                "_op_type": "index",
                "_index": self._collection_name.lower(),
                "_source": {
                    Field.CONTENT_KEY: documents[i].page_content,
                    Field.VECTOR: embeddings[i],  # Make sure you pass an array here
                    Field.METADATA_KEY: documents[i].metadata,
                },
            }
            # See https://github.com/langchain-ai/langchainjs/issues/4346#issuecomment-1935123377
            if self._client_config.aws_service != "aoss":
                action["_id"] = uuid4().hex
            actions.append(action)

        helpers.bulk(
            client=self._client,
            actions=actions,
            timeout=30,
            max_retries=3,
        )

    def get_ids_by_metadata_field(self, key: str, value: str):
        query = {"query": {"term": {f"{Field.METADATA_KEY}.{key}": value}}}
        response = self._client.search(index=self._collection_name.lower(), body=query)
        if response["hits"]["hits"]:
            return [hit["_id"] for hit in response["hits"]["hits"]]
        else:
            return None

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self.delete_by_ids(ids)

    def delete_by_ids(self, ids: list[str]):
        index_name = self._collection_name.lower()
        if not self._client.indices.exists(index=index_name):
            logger.warning("Index %s does not exist", index_name)
            return

        # Obtaining All Actual Documents_ID
        actual_ids = []

        for doc_id in ids:
            es_ids = self.get_ids_by_metadata_field("doc_id", doc_id)
            if es_ids:
                actual_ids.extend(es_ids)
            else:
                logger.warning("Document with metadata doc_id %s not found for deletion", doc_id)

        if actual_ids:
            actions = [{"_op_type": "delete", "_index": index_name, "_id": es_id} for es_id in actual_ids]
            try:
                helpers.bulk(self._client, actions)
            except BulkIndexError as e:
                for error in e.errors:
                    delete_error = error.get("delete", {})
                    status = delete_error.get("status")
                    doc_id = delete_error.get("_id")

                    if status == 404:
                        logger.warning("Document not found for deletion: %s", doc_id)
                    else:
                        logger.exception("Error deleting document: %s", error)

    def delete(self):
        self._client.indices.delete(index=self._collection_name.lower(), ignore_unavailable=True)

    def text_exists(self, id: str) -> bool:
        try:
            self._client.get(index=self._collection_name.lower(), id=id)
            return True
        except:
            return False

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        # Make sure query_vector is a list
        if not isinstance(query_vector, list):
            raise ValueError("query_vector should be a list of floats")

        # Check whether query_vector is a floating-point number list
        if not all(isinstance(x, float) for x in query_vector):
            raise ValueError("All elements in query_vector should be floats")

        query = {
            "size": kwargs.get("top_k", 4),
            "query": {"knn": {Field.VECTOR: {Field.VECTOR: query_vector, "k": kwargs.get("top_k", 4)}}},
        }
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            query["query"] = {
                "script_score": {
                    "query": {"bool": {"filter": [{"terms": {Field.DOCUMENT_ID: document_ids_filter}}]}},
                    "script": {
                        "source": "knn_score",
                        "lang": "knn",
                        "params": {"field": Field.VECTOR, "query_value": query_vector, "space_type": "l2"},
                    },
                }
            }

        try:
            response = self._client.search(index=self._collection_name.lower(), body=query)
        except Exception:
            logger.exception("Error executing vector search, query: %s", query)
            raise

        docs = []
        for hit in response["hits"]["hits"]:
            metadata = hit["_source"].get(Field.METADATA_KEY, {})

            # Make sure metadata is a dictionary
            if metadata is None:
                metadata = {}

            metadata["score"] = hit["_score"]
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            if hit["_score"] >= score_threshold:
                doc = Document(page_content=hit["_source"].get(Field.CONTENT_KEY), metadata=metadata)
                docs.append(doc)

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        full_text_query = {"query": {"bool": {"must": [{"match": {Field.CONTENT_KEY.value: query}}]}}}
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            full_text_query["query"]["bool"]["filter"] = [{"terms": {"metadata.document_id": document_ids_filter}}]

        response = self._client.search(index=self._collection_name.lower(), body=full_text_query)

        docs = []
        for hit in response["hits"]["hits"]:
            metadata = hit["_source"].get(Field.METADATA_KEY)
            vector = hit["_source"].get(Field.VECTOR)
            page_content = hit["_source"].get(Field.CONTENT_KEY)
            doc = Document(page_content=page_content, vector=vector, metadata=metadata)
            docs.append(doc)

        return docs

    def create_collection(
        self, embeddings: list, metadatas: list[dict] | None = None, index_params: dict | None = None
    ):
        lock_name = f"vector_indexing_lock_{self._collection_name.lower()}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name.lower()}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name.lower())
                return

            if not self._client.indices.exists(index=self._collection_name.lower()):
                index_body = {
                    "settings": {"index": {"knn": True}},
                    "mappings": {
                        "properties": {
                            Field.CONTENT_KEY: {"type": "text"},
                            Field.VECTOR: {
                                "type": "knn_vector",
                                "dimension": len(embeddings[0]),  # Make sure the dimension is correct here
                                "method": {
                                    "name": "hnsw",
                                    "space_type": "l2",
                                    "engine": "faiss",
                                    "parameters": {"ef_construction": 64, "m": 8},
                                },
                            },
                            Field.METADATA_KEY: {
                                "type": "object",
                                "properties": {
                                    "doc_id": {"type": "keyword"},  # Map doc_id to keyword type
                                    "document_id": {"type": "keyword"},
                                },
                            },
                        }
                    },
                }

                logger.info("Creating OpenSearch index %s", self._collection_name.lower())
                self._client.indices.create(index=self._collection_name.lower(), body=index_body)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class OpenSearchVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> OpenSearchVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.OPENSEARCH, collection_name))

        open_search_config = OpenSearchConfig(
            host=dify_config.OPENSEARCH_HOST or "localhost",
            port=dify_config.OPENSEARCH_PORT,
            secure=dify_config.OPENSEARCH_SECURE,
            verify_certs=dify_config.OPENSEARCH_VERIFY_CERTS,
            auth_method=dify_config.OPENSEARCH_AUTH_METHOD,
            user=dify_config.OPENSEARCH_USER,
            password=dify_config.OPENSEARCH_PASSWORD,
            aws_region=dify_config.OPENSEARCH_AWS_REGION,
            aws_service=dify_config.OPENSEARCH_AWS_SERVICE,
        )

        return OpenSearchVector(collection_name=collection_name, config=open_search_config)

```

### api/core/rag/datasource/vdb/opensearch/__init__.py
```py

```

### api/core/rag/datasource/vdb/pgvector/__init__.py
```py

```

### api/core/rag/datasource/vdb/pgvector/pgvector.py
```py
import hashlib
import json
import logging
import uuid
from contextlib import contextmanager
from typing import Any

import psycopg2.errors
import psycopg2.extras
import psycopg2.pool
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class PGVectorConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    min_connection: int
    max_connection: int
    pg_bigm: bool = False

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config PGVECTOR_HOST is required")
        if not values["port"]:
            raise ValueError("config PGVECTOR_PORT is required")
        if not values["user"]:
            raise ValueError("config PGVECTOR_USER is required")
        if not values["password"]:
            raise ValueError("config PGVECTOR_PASSWORD is required")
        if not values["database"]:
            raise ValueError("config PGVECTOR_DATABASE is required")
        if not values["min_connection"]:
            raise ValueError("config PGVECTOR_MIN_CONNECTION is required")
        if not values["max_connection"]:
            raise ValueError("config PGVECTOR_MAX_CONNECTION is required")
        if values["min_connection"] > values["max_connection"]:
            raise ValueError("config PGVECTOR_MIN_CONNECTION should less than PGVECTOR_MAX_CONNECTION")
        return values


SQL_CREATE_TABLE = """
CREATE TABLE IF NOT EXISTS {table_name} (
    id UUID PRIMARY KEY,
    text TEXT NOT NULL,
    meta JSONB NOT NULL,
    embedding vector({dimension}) NOT NULL
) using heap;
"""

SQL_CREATE_INDEX = """
CREATE INDEX IF NOT EXISTS embedding_cosine_v1_idx_{index_hash} ON {table_name}
USING hnsw (embedding vector_cosine_ops) WITH (m = 16, ef_construction = 64);
"""

SQL_CREATE_INDEX_PG_BIGM = """
CREATE INDEX IF NOT EXISTS bigm_idx_{index_hash} ON {table_name}
USING gin (text gin_bigm_ops);
"""


class PGVector(BaseVector):
    def __init__(self, collection_name: str, config: PGVectorConfig):
        super().__init__(collection_name)
        self.pool = self._create_connection_pool(config)
        self.table_name = f"embedding_{collection_name}"
        self.index_hash = hashlib.md5(self.table_name.encode()).hexdigest()[:8]
        self.pg_bigm = config.pg_bigm

    def get_type(self) -> str:
        return VectorType.PGVECTOR

    def _create_connection_pool(self, config: PGVectorConfig):
        return psycopg2.pool.SimpleConnectionPool(
            config.min_connection,
            config.max_connection,
            host=config.host,
            port=config.port,
            user=config.user,
            password=config.password,
            database=config.database,
        )

    @contextmanager
    def _get_cursor(self):
        conn = self.pool.getconn()
        cur = conn.cursor()
        try:
            yield cur
        finally:
            cur.close()
            conn.commit()
            self.pool.putconn(conn)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        pks = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                pks.append(doc_id)
                values.append(
                    (
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata),
                        embeddings[i],
                    )
                )
        with self._get_cursor() as cur:
            psycopg2.extras.execute_values(
                cur, f"INSERT INTO {self.table_name} (id, text, meta, embedding) VALUES %s", values
            )
        return pks

    def text_exists(self, id: str) -> bool:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT id FROM {self.table_name} WHERE id = %s", (id,))
            return cur.fetchone() is not None

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT meta, text FROM {self.table_name} WHERE id IN %s", (tuple(ids),))
            docs = []
            for record in cur:
                docs.append(Document(page_content=record[1], metadata=record[0]))
        return docs

    def delete_by_ids(self, ids: list[str]):
        # Avoiding crashes caused by performing delete operations on empty lists in certain scenarios
        # Scenario 1: extract a document fails, resulting in a table not being created.
        # Then clicking the retry button triggers a delete operation on an empty list.
        if not ids:
            return
        with self._get_cursor() as cur:
            try:
                cur.execute(f"DELETE FROM {self.table_name} WHERE id IN %s", (tuple(ids),))
            except psycopg2.errors.UndefinedTable:
                # table not exists
                logger.warning("Table %s not found, skipping delete operation.", self.table_name)
                return
            except Exception as e:
                raise e

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_cursor() as cur:
            cur.execute(f"DELETE FROM {self.table_name} WHERE meta->>%s = %s", (key, value))

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search the nearest neighbors to a vector.

        :param query_vector: The input vector to search for similar items.
        :return: List of Documents that are nearest to the query vector.
        """
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause = f" WHERE meta->>'document_id' in ({document_ids}) "

        with self._get_cursor() as cur:
            cur.execute(
                f"SELECT meta, text, embedding <=> %s AS distance FROM {self.table_name}"
                f" {where_clause}"
                f" ORDER BY distance LIMIT {top_k}",
                (json.dumps(query_vector),),
            )
            docs = []
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            for record in cur:
                metadata, text, distance = record
                score = 1 - distance
                metadata["score"] = score
                if score >= score_threshold:
                    docs.append(Document(page_content=text, metadata=metadata))
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 5)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        with self._get_cursor() as cur:
            document_ids_filter = kwargs.get("document_ids_filter")
            where_clause = ""
            if document_ids_filter:
                document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
                where_clause = f" AND meta->>'document_id' in ({document_ids}) "
            if self.pg_bigm:
                cur.execute("SET pg_bigm.similarity_limit TO 0.000001")
                cur.execute(
                    f"""SELECT meta, text, bigm_similarity(unistr(%s), coalesce(text, '')) AS score
                    FROM {self.table_name}
                    WHERE text =%% unistr(%s)
                    {where_clause}
                    ORDER BY score DESC
                    LIMIT {top_k}""",
                    # f"'{query}'" is required in order to account for whitespace in query
                    (f"'{query}'", f"'{query}'"),
                )
            else:
                cur.execute(
                    f"""SELECT meta, text, ts_rank(to_tsvector(coalesce(text, '')), plainto_tsquery(%s)) AS score
                    FROM {self.table_name}
                    WHERE to_tsvector(text) @@ plainto_tsquery(%s)
                    {where_clause}
                    ORDER BY score DESC
                    LIMIT {top_k}""",
                    # f"'{query}'" is required in order to account for whitespace in query
                    (f"'{query}'", f"'{query}'"),
                )

            docs = []

            for record in cur:
                metadata, text, score = record
                metadata["score"] = score
                docs.append(Document(page_content=text, metadata=metadata))

        return docs

    def delete(self):
        with self._get_cursor() as cur:
            cur.execute(f"DROP TABLE IF EXISTS {self.table_name}")

    def _create_collection(self, dimension: int):
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            with self._get_cursor() as cur:
                cur.execute("SELECT 1 FROM pg_extension WHERE extname = 'vector'")
                if not cur.fetchone():
                    cur.execute("CREATE EXTENSION vector")

                cur.execute(SQL_CREATE_TABLE.format(table_name=self.table_name, dimension=dimension))
                # PG hnsw index only support 2000 dimension or less
                # ref: https://github.com/pgvector/pgvector?tab=readme-ov-file#indexing
                if dimension <= 2000:
                    cur.execute(SQL_CREATE_INDEX.format(table_name=self.table_name, index_hash=self.index_hash))
                if self.pg_bigm:
                    cur.execute(SQL_CREATE_INDEX_PG_BIGM.format(table_name=self.table_name, index_hash=self.index_hash))
            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class PGVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> PGVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.PGVECTOR, collection_name))

        return PGVector(
            collection_name=collection_name,
            config=PGVectorConfig(
                host=dify_config.PGVECTOR_HOST or "localhost",
                port=dify_config.PGVECTOR_PORT,
                user=dify_config.PGVECTOR_USER or "postgres",
                password=dify_config.PGVECTOR_PASSWORD or "",
                database=dify_config.PGVECTOR_DATABASE or "postgres",
                min_connection=dify_config.PGVECTOR_MIN_CONNECTION,
                max_connection=dify_config.PGVECTOR_MAX_CONNECTION,
                pg_bigm=dify_config.PGVECTOR_PG_BIGM,
            ),
        )

```

### api/core/rag/datasource/vdb/clickzetta/__init__.py
```py
# Clickzetta Vector Database Integration for Dify

```

### api/core/rag/datasource/vdb/clickzetta/clickzetta_vector.py
```py
from __future__ import annotations

import contextlib
import json
import logging
import queue
import re
import threading
import time
import uuid
from typing import TYPE_CHECKING, Any

import clickzetta  # type: ignore
from pydantic import BaseModel, model_validator

if TYPE_CHECKING:
    from clickzetta.connector.v0.connection import Connection  # type: ignore

from configs import dify_config
from core.rag.datasource.vdb.field import Field, parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from models.dataset import Dataset

logger = logging.getLogger(__name__)


# ClickZetta Lakehouse Vector Database Configuration


class ClickzettaConfig(BaseModel):
    """
    Configuration class for Clickzetta connection.
    """

    username: str
    password: str
    instance: str
    service: str = "api.clickzetta.com"
    workspace: str = "quick_start"
    vcluster: str = "default_ap"
    schema_name: str = "dify"  # Renamed to avoid shadowing BaseModel.schema
    # Advanced settings
    batch_size: int = 20  # Reduced batch size to avoid large SQL statements
    enable_inverted_index: bool = True  # Enable inverted index for full-text search
    analyzer_type: str = "chinese"  # Analyzer type for full-text search: keyword, english, chinese, unicode
    analyzer_mode: str = "smart"  # Analyzer mode: max_word, smart
    vector_distance_function: str = "cosine_distance"  # l2_distance or cosine_distance

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        """
        Validate the configuration values.
        """
        if not values.get("username"):
            raise ValueError("config CLICKZETTA_USERNAME is required")
        if not values.get("password"):
            raise ValueError("config CLICKZETTA_PASSWORD is required")
        if not values.get("instance"):
            raise ValueError("config CLICKZETTA_INSTANCE is required")
        if not values.get("service"):
            raise ValueError("config CLICKZETTA_SERVICE is required")
        if not values.get("workspace"):
            raise ValueError("config CLICKZETTA_WORKSPACE is required")
        if not values.get("vcluster"):
            raise ValueError("config CLICKZETTA_VCLUSTER is required")
        if not values.get("schema_name"):
            raise ValueError("config CLICKZETTA_SCHEMA is required")
        return values


class ClickzettaConnectionPool:
    """
    Global connection pool for ClickZetta connections.
    Manages connection reuse across ClickzettaVector instances.
    """

    _instance: ClickzettaConnectionPool | None = None
    _lock = threading.Lock()

    def __init__(self):
        self._pools: dict[str, list[tuple[Connection, float]]] = {}  # config_key -> [(connection, last_used_time)]
        self._pool_locks: dict[str, threading.Lock] = {}
        self._max_pool_size = 5  # Maximum connections per configuration
        self._connection_timeout = 300  # 5 minutes timeout
        self._cleanup_thread: threading.Thread | None = None
        self._shutdown = False
        self._start_cleanup_thread()

    @classmethod
    def get_instance(cls) -> ClickzettaConnectionPool:
        """Get singleton instance of connection pool."""
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = cls()
        return cls._instance

    def _get_config_key(self, config: ClickzettaConfig) -> str:
        """Generate unique key for connection configuration."""
        return (
            f"{config.username}:{config.instance}:{config.service}:"
            f"{config.workspace}:{config.vcluster}:{config.schema_name}"
        )

    def _create_connection(self, config: ClickzettaConfig) -> Connection:
        """Create a new ClickZetta connection."""
        max_retries = 3
        retry_delay = 1.0

        for attempt in range(max_retries):
            try:
                connection = clickzetta.connect(
                    username=config.username,
                    password=config.password,
                    instance=config.instance,
                    service=config.service,
                    workspace=config.workspace,
                    vcluster=config.vcluster,
                    schema=config.schema_name,
                )

                # Configure connection session settings
                self._configure_connection(connection)
                logger.debug("Created new ClickZetta connection (attempt %d/%d)", attempt + 1, max_retries)
                return connection
            except Exception:
                logger.exception("ClickZetta connection attempt %d/%d failed", attempt + 1, max_retries)
                if attempt < max_retries - 1:
                    time.sleep(retry_delay * (2**attempt))
                else:
                    raise

        raise RuntimeError(f"Failed to create ClickZetta connection after {max_retries} attempts")

    def _configure_connection(self, connection: Connection):
        """Configure connection session settings."""
        try:
            with connection.cursor() as cursor:
                # Temporarily suppress ClickZetta client logging to reduce noise
                clickzetta_logger = logging.getLogger("clickzetta")
                original_level = clickzetta_logger.level
                clickzetta_logger.setLevel(logging.WARNING)

                try:
                    # Use quote mode for string literal escaping
                    cursor.execute("SET cz.sql.string.literal.escape.mode = 'quote'")

                    # Apply performance optimization hints
                    performance_hints = [
                        # Vector index optimization
                        "SET cz.storage.parquet.vector.index.read.memory.cache = true",
                        "SET cz.storage.parquet.vector.index.read.local.cache = false",
                        # Query optimization
                        "SET cz.sql.table.scan.push.down.filter = true",
                        "SET cz.sql.table.scan.enable.ensure.filter = true",
                        "SET cz.storage.always.prefetch.internal = true",
                        "SET cz.optimizer.generate.columns.always.valid = true",
                        "SET cz.sql.index.prewhere.enabled = true",
                        # Storage optimization
                        "SET cz.storage.parquet.enable.io.prefetch = false",
                        "SET cz.optimizer.enable.mv.rewrite = false",
                        "SET cz.sql.dump.as.lz4 = true",
                        "SET cz.optimizer.limited.optimization.naive.query = true",
                        "SET cz.sql.table.scan.enable.push.down.log = false",
                        "SET cz.storage.use.file.format.local.stats = false",
                        "SET cz.storage.local.file.object.cache.level = all",
                        # Job execution optimization
                        "SET cz.sql.job.fast.mode = true",
                        "SET cz.storage.parquet.non.contiguous.read = true",
                        "SET cz.sql.compaction.after.commit = true",
                    ]

                    for hint in performance_hints:
                        cursor.execute(hint)
                finally:
                    # Restore original logging level
                    clickzetta_logger.setLevel(original_level)

        except Exception:
            logger.exception("Failed to configure connection, continuing with defaults")

    def _is_connection_valid(self, connection: Connection) -> bool:
        """Check if connection is still valid."""
        try:
            with connection.cursor() as cursor:
                cursor.execute("SELECT 1")
                return True
        except Exception:
            return False

    def get_connection(self, config: ClickzettaConfig) -> Connection:
        """Get a connection from the pool or create a new one."""
        config_key = self._get_config_key(config)

        # Ensure pool lock exists
        if config_key not in self._pool_locks:
            with self._lock:
                if config_key not in self._pool_locks:
                    self._pool_locks[config_key] = threading.Lock()
                    self._pools[config_key] = []

        with self._pool_locks[config_key]:
            pool = self._pools[config_key]
            current_time = time.time()

            # Try to reuse existing connection
            while pool:
                connection, last_used = pool.pop(0)

                # Check if connection is not expired and still valid
                if current_time - last_used < self._connection_timeout and self._is_connection_valid(connection):
                    logger.debug("Reusing ClickZetta connection from pool")
                    return connection
                else:
                    # Connection expired or invalid, close it
                    with contextlib.suppress(Exception):
                        connection.close()

            # No valid connection found, create new one
            return self._create_connection(config)

    def return_connection(self, config: ClickzettaConfig, connection: Connection):
        """Return a connection to the pool."""
        config_key = self._get_config_key(config)

        if config_key not in self._pool_locks:
            # Pool was cleaned up, just close the connection
            with contextlib.suppress(Exception):
                connection.close()
            return

        with self._pool_locks[config_key]:
            pool = self._pools[config_key]

            # Only return to pool if not at capacity and connection is valid
            if len(pool) < self._max_pool_size and self._is_connection_valid(connection):
                pool.append((connection, time.time()))
                logger.debug("Returned ClickZetta connection to pool")
            else:
                # Pool full or connection invalid, close it
                with contextlib.suppress(Exception):
                    connection.close()

    def _cleanup_expired_connections(self):
        """Clean up expired connections from all pools."""
        current_time = time.time()

        with self._lock:
            for config_key in list(self._pools.keys()):
                if config_key not in self._pool_locks:
                    continue

                with self._pool_locks[config_key]:
                    pool = self._pools[config_key]
                    valid_connections = []

                    for connection, last_used in pool:
                        if current_time - last_used < self._connection_timeout:
                            valid_connections.append((connection, last_used))
                        else:
                            with contextlib.suppress(Exception):
                                connection.close()

                    self._pools[config_key] = valid_connections

    def _start_cleanup_thread(self):
        """Start background thread for connection cleanup."""

        def cleanup_worker():
            while not self._shutdown:
                try:
                    time.sleep(60)  # Cleanup every minute
                    if not self._shutdown:
                        self._cleanup_expired_connections()
                except Exception:
                    logger.exception("Error in connection pool cleanup")

        self._cleanup_thread = threading.Thread(target=cleanup_worker, daemon=True)
        self._cleanup_thread.start()

    def shutdown(self):
        """Shutdown connection pool and close all connections."""
        self._shutdown = True

        with self._lock:
            for config_key in list(self._pools.keys()):
                if config_key not in self._pool_locks:
                    continue

                with self._pool_locks[config_key]:
                    pool = self._pools[config_key]
                    for connection, _ in pool:
                        with contextlib.suppress(Exception):
                            connection.close()
                    pool.clear()


class ClickzettaVector(BaseVector):
    """
    Clickzetta vector storage implementation.
    """

    # Class-level write queue and lock for serializing writes
    _write_queue: queue.Queue | None = None
    _write_thread: threading.Thread | None = None
    _write_lock = threading.Lock()
    _shutdown = False

    def __init__(self, collection_name: str, config: ClickzettaConfig):
        super().__init__(collection_name)
        self._config = config
        self._table_name = collection_name.replace("-", "_").lower()  # Ensure valid table name
        self._connection_pool = ClickzettaConnectionPool.get_instance()
        self._init_write_queue()

    def _get_connection(self) -> Connection:
        """Get a connection from the pool."""
        return self._connection_pool.get_connection(self._config)

    def _return_connection(self, connection: Connection):
        """Return a connection to the pool."""
        self._connection_pool.return_connection(self._config, connection)

    class ConnectionContext:
        """Context manager for borrowing and returning connections."""

        def __init__(self, vector_instance: ClickzettaVector):
            self.vector = vector_instance
            self.connection: Connection | None = None

        def __enter__(self) -> Connection:
            self.connection = self.vector._get_connection()
            return self.connection

        def __exit__(self, exc_type, exc_val, exc_tb):
            if self.connection:
                self.vector._return_connection(self.connection)

    def get_connection_context(self) -> ClickzettaVector.ConnectionContext:
        """Get a connection context manager."""
        return self.ConnectionContext(self)

    def _parse_metadata(self, raw_metadata: str, row_id: str):
        """
        Parse metadata from JSON string with proper error handling and fallback.

        Args:
            raw_metadata: Raw JSON string from database
            row_id: Row ID for fallback document_id

        Returns:
            Parsed metadata dict with guaranteed required fields
        """
        try:
            if raw_metadata:
                # First parse may yield a string (double-encoded JSON) so use json.loads
                first_pass = json.loads(raw_metadata)

                # Handle double-encoded JSON
                if isinstance(first_pass, str):
                    metadata = parse_metadata_json(first_pass)
                elif isinstance(first_pass, dict):
                    metadata = first_pass
                else:
                    metadata = {}
            else:
                metadata = {}
        except (json.JSONDecodeError, ValueError, TypeError):
            logger.exception("JSON parsing failed for metadata")
            # Fallback: extract document_id with regex
            doc_id_match = re.search(r'"document_id":\s*"([^"]+)"', raw_metadata or "")
            metadata = {"document_id": doc_id_match.group(1)} if doc_id_match else {}

        # Ensure required fields are set
        metadata["doc_id"] = row_id  # segment id

        # Ensure document_id exists (critical for Dify's format_retrieval_documents)
        if "document_id" not in metadata:
            metadata["document_id"] = row_id  # fallback to segment id

        return metadata

    @classmethod
    def _init_write_queue(cls):
        """Initialize the write queue and worker thread."""
        with cls._write_lock:
            if cls._write_queue is None:
                cls._write_queue = queue.Queue()
                cls._write_thread = threading.Thread(target=cls._write_worker, daemon=True)
                cls._write_thread.start()
                logger.info("Started Clickzetta write worker thread")

    @classmethod
    def _write_worker(cls):
        """Worker thread that processes write tasks sequentially."""
        while not cls._shutdown:
            try:
                # Get task from queue with timeout
                if cls._write_queue is not None:
                    task = cls._write_queue.get(timeout=1)
                    if task is None:  # Shutdown signal
                        break

                    # Execute the write task
                    func, args, kwargs, result_queue = task
                    try:
                        result = func(*args, **kwargs)
                        result_queue.put((True, result))
                    except (RuntimeError, ValueError, TypeError, ConnectionError) as e:
                        logger.exception("Write task failed")
                        result_queue.put((False, e))
                    finally:
                        cls._write_queue.task_done()
                else:
                    break
            except queue.Empty:
                continue
            except (RuntimeError, ValueError, TypeError, ConnectionError) as e:
                logger.exception("Write worker error")

    def _execute_write(self, func, *args, **kwargs):
        """Execute a write operation through the queue."""
        if ClickzettaVector._write_queue is None:
            raise RuntimeError("Write queue not initialized")

        result_queue: queue.Queue[tuple[bool, Any]] = queue.Queue()
        ClickzettaVector._write_queue.put((func, args, kwargs, result_queue))

        # Wait for result
        success, result = result_queue.get()
        if not success:
            raise result
        return result

    def get_type(self) -> str:
        """Return the vector database type."""
        return "clickzetta"

    def _ensure_connection(self) -> Connection:
        """Get a connection from the pool."""
        return self._get_connection()

    def _table_exists(self) -> bool:
        """Check if the table exists."""
        try:
            with self.get_connection_context() as connection:
                with connection.cursor() as cursor:
                    cursor.execute(f"DESC {self._config.schema_name}.{self._table_name}")
                    return True
        except Exception as e:
            error_message = str(e).lower()
            # Handle ClickZetta specific "table or view not found" errors
            if any(
                phrase in error_message
                for phrase in ["table or view not found", "czlh-42000", "semantic analysis exception"]
            ):
                logger.debug("Table %s.%s does not exist", self._config.schema_name, self._table_name)
                return False
            else:
                # For other connection/permission errors, log warning but return False to avoid blocking cleanup
                logger.exception(
                    "Table existence check failed for %s.%s, assuming it doesn't exist",
                    self._config.schema_name,
                    self._table_name,
                )
                return False

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        """Create the collection and add initial documents."""
        # Execute table creation through write queue to avoid concurrent conflicts
        self._execute_write(self._create_table_and_indexes, embeddings)

        # Add initial texts
        if texts:
            self.add_texts(texts, embeddings, **kwargs)

    def _create_table_and_indexes(self, embeddings: list[list[float]]):
        """Create table and indexes (executed in write worker thread)."""
        # Check if table already exists to avoid unnecessary index creation
        if self._table_exists():
            logger.info("Table %s.%s already exists, skipping creation", self._config.schema_name, self._table_name)
            return

        # Create table with vector and metadata columns
        dimension = len(embeddings[0]) if embeddings else 768

        create_table_sql = f"""
        CREATE TABLE IF NOT EXISTS {self._config.schema_name}.{self._table_name} (
            id STRING NOT NULL COMMENT 'Unique document identifier',
            {Field.CONTENT_KEY} STRING NOT NULL COMMENT 'Document text content for search and retrieval',
            {Field.METADATA_KEY} JSON COMMENT 'Document metadata including source, type, and other attributes',
            {Field.VECTOR} VECTOR(FLOAT, {dimension}) NOT NULL COMMENT
                'High-dimensional embedding vector for semantic similarity search',
            PRIMARY KEY (id)
        ) COMMENT 'Dify RAG knowledge base vector storage table for document embeddings and content'
        """

        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                cursor.execute(create_table_sql)
                logger.info("Created table %s.%s", self._config.schema_name, self._table_name)

                # Create vector index
                self._create_vector_index(cursor)

                # Create inverted index for full-text search if enabled
                if self._config.enable_inverted_index:
                    self._create_inverted_index(cursor)

    def _create_vector_index(self, cursor):
        """Create HNSW vector index for similarity search."""
        # Use a fixed index name based on table and column name
        index_name = f"idx_{self._table_name}_vector"

        # First check if an index already exists on this column
        try:
            cursor.execute(f"SHOW INDEX FROM {self._config.schema_name}.{self._table_name}")
            existing_indexes = cursor.fetchall()
            for idx in existing_indexes:
                # Check if vector index already exists on the embedding column
                if Field.VECTOR in str(idx).lower():
                    logger.info("Vector index already exists on column %s", Field.VECTOR)
                    return
        except (RuntimeError, ValueError) as e:
            logger.warning("Failed to check existing indexes: %s", e)

        index_sql = f"""
        CREATE VECTOR INDEX IF NOT EXISTS {index_name}
        ON TABLE {self._config.schema_name}.{self._table_name}({Field.VECTOR})
        PROPERTIES (
            "distance.function" = "{self._config.vector_distance_function}",
            "scalar.type" = "f32",
            "m" = "16",
            "ef.construction" = "128"
        )
        """
        try:
            cursor.execute(index_sql)
            logger.info("Created vector index: %s", index_name)
        except (RuntimeError, ValueError) as e:
            error_msg = str(e).lower()
            if "already exists" in error_msg or "already has index" in error_msg or "with the same type" in error_msg:
                logger.info("Vector index already exists: %s", e)
            else:
                logger.exception("Failed to create vector index")
                raise

    def _create_inverted_index(self, cursor):
        """Create inverted index for full-text search."""
        # Use a fixed index name based on table name to avoid duplicates
        index_name = f"idx_{self._table_name}_text"

        # Check if an inverted index already exists on this column
        try:
            cursor.execute(f"SHOW INDEX FROM {self._config.schema_name}.{self._table_name}")
            existing_indexes = cursor.fetchall()
            for idx in existing_indexes:
                idx_str = str(idx).lower()
                # More precise check: look for inverted index specifically on the content column
                if (
                    "inverted" in idx_str
                    and Field.CONTENT_KEY.lower() in idx_str
                    and (index_name.lower() in idx_str or f"idx_{self._table_name}_text" in idx_str)
                ):
                    logger.info("Inverted index already exists on column %s: %s", Field.CONTENT_KEY, idx)
                    return
        except (RuntimeError, ValueError) as e:
            logger.warning("Failed to check existing indexes: %s", e)

        index_sql = f"""
        CREATE INVERTED INDEX IF NOT EXISTS {index_name}
        ON TABLE {self._config.schema_name}.{self._table_name} ({Field.CONTENT_KEY})
        PROPERTIES (
            "analyzer" = "{self._config.analyzer_type}",
            "mode" = "{self._config.analyzer_mode}"
        )
        """
        try:
            cursor.execute(index_sql)
            logger.info("Created inverted index: %s", index_name)
        except (RuntimeError, ValueError) as e:
            error_msg = str(e).lower()
            # Handle ClickZetta specific error messages
            if (
                "already exists" in error_msg
                or "already has index" in error_msg
                or "with the same type" in error_msg
                or "cannot create inverted index" in error_msg
            ) and "already has index" in error_msg:
                logger.info("Inverted index already exists on column %s", Field.CONTENT_KEY)
                # Try to get the existing index name for logging
                try:
                    cursor.execute(f"SHOW INDEX FROM {self._config.schema_name}.{self._table_name}")
                    existing_indexes = cursor.fetchall()
                    for idx in existing_indexes:
                        if "inverted" in str(idx).lower() and Field.CONTENT_KEY.lower() in str(idx).lower():
                            logger.info("Found existing inverted index: %s", idx)
                            break
                except (RuntimeError, ValueError):
                    pass
            else:
                logger.warning("Failed to create inverted index: %s", e)
                # Continue without inverted index - full-text search will fall back to LIKE

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs) -> list[str]:
        """Add documents with embeddings to the collection."""
        if not documents:
            return []

        batch_size = self._config.batch_size
        total_batches = (len(documents) + batch_size - 1) // batch_size
        added_ids = []

        for i in range(0, len(documents), batch_size):
            batch_docs = documents[i : i + batch_size]
            batch_embeddings = embeddings[i : i + batch_size]
            batch_doc_ids = []
            for doc in batch_docs:
                metadata = doc.metadata if isinstance(doc.metadata, dict) else {}
                batch_doc_ids.append(self._safe_doc_id(metadata.get("doc_id", str(uuid.uuid4()))))
            added_ids.extend(batch_doc_ids)

            # Execute batch insert through write queue
            self._execute_write(
                self._insert_batch, batch_docs, batch_embeddings, batch_doc_ids, i, batch_size, total_batches
            )

        return added_ids

    def _insert_batch(
        self,
        batch_docs: list[Document],
        batch_embeddings: list[list[float]],
        batch_doc_ids: list[str],
        batch_index: int,
        batch_size: int,
        total_batches: int,
    ):
        """Insert a batch of documents using parameterized queries (executed in write worker thread)."""
        if not batch_docs or not batch_embeddings:
            logger.warning("Empty batch provided, skipping insertion")
            return

        if len(batch_docs) != len(batch_embeddings):
            logger.error("Mismatch between docs (%d) and embeddings (%d)", len(batch_docs), len(batch_embeddings))
            return

        # Prepare data for parameterized insertion
        data_rows = []
        vector_dimension = len(batch_embeddings[0]) if batch_embeddings and batch_embeddings[0] else 768

        for doc, embedding, doc_id in zip(batch_docs, batch_embeddings, batch_doc_ids):
            # Optimized: minimal checks for common case, fallback for edge cases
            metadata = doc.metadata if isinstance(doc.metadata, dict) else {}

            # Fast path for JSON serialization
            try:
                metadata_json = json.dumps(metadata, ensure_ascii=True)
            except (TypeError, ValueError):
                logger.warning("JSON serialization failed, using empty dict")
                metadata_json = "{}"

            content = doc.page_content or ""

            # According to ClickZetta docs, vector should be formatted as array string
            # for external systems: '[1.0, 2.0, 3.0]'
            vector_str = "[" + ",".join(map(str, embedding)) + "]"
            data_rows.append([doc_id, content, metadata_json, vector_str])

        # Check if we have any valid data to insert
        if not data_rows:
            logger.warning("No valid documents to insert in batch %d/%d", batch_index // batch_size + 1, total_batches)
            return

        # Use parameterized INSERT with executemany for better performance and security
        # Cast JSON and VECTOR in SQL, pass raw data as parameters
        columns = f"id, {Field.CONTENT_KEY}, {Field.METADATA_KEY}, {Field.VECTOR}"
        insert_sql = (
            f"INSERT INTO {self._config.schema_name}.{self._table_name} ({columns}) "
            f"VALUES (?, ?, CAST(? AS JSON), CAST(? AS VECTOR({vector_dimension})))"
        )

        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                try:
                    # Set session-level hints for batch insert operations
                    # Note: executemany doesn't support hints parameter, so we set them as session variables
                    # Temporarily suppress ClickZetta client logging to reduce noise
                    clickzetta_logger = logging.getLogger("clickzetta")
                    original_level = clickzetta_logger.level
                    clickzetta_logger.setLevel(logging.WARNING)

                    try:
                        cursor.execute("SET cz.sql.job.fast.mode = true")
                        cursor.execute("SET cz.sql.compaction.after.commit = true")
                        cursor.execute("SET cz.storage.always.prefetch.internal = true")
                    finally:
                        # Restore original logging level
                        clickzetta_logger.setLevel(original_level)

                    cursor.executemany(insert_sql, data_rows)
                    logger.info(
                        "Inserted batch %d/%d (%d valid docs using parameterized query with VECTOR(%d) cast)",
                        batch_index // batch_size + 1,
                        total_batches,
                        len(data_rows),
                        vector_dimension,
                    )
                except (RuntimeError, ValueError, TypeError, ConnectionError):
                    logger.exception("Parameterized SQL execution failed for %d documents", len(data_rows))
                    logger.exception("SQL template: %s", insert_sql)
                    logger.exception("Sample data row: %s", data_rows[0] if data_rows else "None")
                    raise

    def text_exists(self, id: str) -> bool:
        """Check if a document exists by ID."""
        # Check if table exists first
        if not self._table_exists():
            return False

        safe_id = self._safe_doc_id(id)
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    f"SELECT COUNT(*) FROM {self._config.schema_name}.{self._table_name} WHERE id = ?",
                    binding_params=[safe_id],
                )
                result = cursor.fetchone()
                return result[0] > 0 if result else False

    def delete_by_ids(self, ids: list[str]):
        """Delete documents by IDs."""
        if not ids:
            return

        # Check if table exists before attempting delete
        if not self._table_exists():
            logger.warning("Table %s.%s does not exist, skipping delete", self._config.schema_name, self._table_name)
            return

        # Execute delete through write queue
        self._execute_write(self._delete_by_ids_impl, ids)

    def _delete_by_ids_impl(self, ids: list[str]):
        """Implementation of delete by IDs (executed in write worker thread)."""
        safe_ids = [self._safe_doc_id(id) for id in ids]

        # Use parameterized query to prevent SQL injection
        placeholders = ",".join("?" for _ in safe_ids)
        sql = f"DELETE FROM {self._config.schema_name}.{self._table_name} WHERE id IN ({placeholders})"

        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                cursor.execute(sql, binding_params=safe_ids)

    def delete_by_metadata_field(self, key: str, value: str):
        """Delete documents by metadata field."""
        # Check if table exists before attempting delete
        if not self._table_exists():
            logger.warning("Table %s.%s does not exist, skipping delete", self._config.schema_name, self._table_name)
            return

        # Execute delete through write queue
        self._execute_write(self._delete_by_metadata_field_impl, key, value)

    def _delete_by_metadata_field_impl(self, key: str, value: str):
        """Implementation of delete by metadata field (executed in write worker thread)."""
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                # Using JSON path to filter with parameterized query
                # Note: JSON path requires literal key name, cannot be parameterized
                # Use json_extract_string function for ClickZetta compatibility
                sql = (
                    f"DELETE FROM {self._config.schema_name}.{self._table_name} "
                    f"WHERE json_extract_string({Field.METADATA_KEY}, '$.{key}') = ?"
                )
                cursor.execute(sql, binding_params=[value])

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """Search for documents by vector similarity."""
        # Check if table exists first
        if not self._table_exists():
            logger.warning(
                "Table %s.%s does not exist, returning empty results",
                self._config.schema_name,
                self._table_name,
            )
            return []

        top_k = kwargs.get("top_k", 10)
        score_threshold = kwargs.get("score_threshold", 0.0)
        document_ids_filter = kwargs.get("document_ids_filter")

        # Handle filter parameter from canvas (workflow)
        _ = kwargs.get("filter", {})

        # Build filter clause
        filter_clauses = []
        if document_ids_filter:
            safe_doc_ids = [str(id).replace("'", "''") for id in document_ids_filter]
            doc_ids_str = ",".join(f"'{id}'" for id in safe_doc_ids)
            # Use json_extract_string function for ClickZetta compatibility
            filter_clauses.append(f"json_extract_string({Field.METADATA_KEY}, '$.document_id') IN ({doc_ids_str})")

        # No need for dataset_id filter since each dataset has its own table

        # Add distance threshold based on distance function
        vector_dimension = len(query_vector)
        if self._config.vector_distance_function == "cosine_distance":
            # For cosine distance, smaller is better (0 = identical, 2 = opposite)
            distance_func = "COSINE_DISTANCE"
            if score_threshold > 0:
                query_vector_str = f"CAST('[{self._format_vector_simple(query_vector)}]' AS VECTOR({vector_dimension}))"
                filter_clauses.append(f"{distance_func}({Field.VECTOR}, {query_vector_str}) < {2 - score_threshold}")
        else:
            # For L2 distance, smaller is better
            distance_func = "L2_DISTANCE"
            if score_threshold > 0:
                query_vector_str = f"CAST('[{self._format_vector_simple(query_vector)}]' AS VECTOR({vector_dimension}))"
                filter_clauses.append(f"{distance_func}({Field.VECTOR}, {query_vector_str}) < {score_threshold}")

        where_clause = " AND ".join(filter_clauses) if filter_clauses else "1=1"

        # Execute vector search query
        query_vector_str = f"CAST('[{self._format_vector_simple(query_vector)}]' AS VECTOR({vector_dimension}))"
        search_sql = f"""
        SELECT id, {Field.CONTENT_KEY}, {Field.METADATA_KEY},
               {distance_func}({Field.VECTOR}, {query_vector_str}) AS distance
        FROM {self._config.schema_name}.{self._table_name}
        WHERE {where_clause}
        ORDER BY distance
        LIMIT {top_k}
        """

        documents = []
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                # Use hints parameter for vector search optimization
                search_hints = {
                    "hints": {
                        "sdk.job.timeout": 60,  # Increase timeout for vector search
                        "cz.sql.job.fast.mode": True,
                        "cz.storage.parquet.vector.index.read.memory.cache": True,
                    }
                }
                cursor.execute(search_sql, search_hints)
                results = cursor.fetchall()

                for row in results:
                    # Parse metadata using centralized method
                    metadata = self._parse_metadata(row[2], row[0])

                    # Add score based on distance
                    if self._config.vector_distance_function == "cosine_distance":
                        metadata["score"] = 1 - (row[3] / 2)
                    else:
                        metadata["score"] = 1 / (1 + row[3])

                    doc = Document(page_content=row[1], metadata=metadata)
                    documents.append(doc)

        return documents

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """Search for documents using full-text search with inverted index."""
        if not self._config.enable_inverted_index:
            logger.warning("Full-text search is not enabled. Enable inverted index in config.")
            return []

        # Check if table exists first
        if not self._table_exists():
            logger.warning(
                "Table %s.%s does not exist, returning empty results",
                self._config.schema_name,
                self._table_name,
            )
            return []

        top_k = kwargs.get("top_k", 10)
        document_ids_filter = kwargs.get("document_ids_filter")

        # Handle filter parameter from canvas (workflow)
        _ = kwargs.get("filter", {})

        # Build filter clause
        filter_clauses = []
        if document_ids_filter:
            safe_doc_ids = [str(id).replace("'", "''") for id in document_ids_filter]
            doc_ids_str = ",".join(f"'{id}'" for id in safe_doc_ids)
            # Use json_extract_string function for ClickZetta compatibility
            filter_clauses.append(f"json_extract_string({Field.METADATA_KEY}, '$.document_id') IN ({doc_ids_str})")

        # No need for dataset_id filter since each dataset has its own table

        # Use match_all function for full-text search
        # match_all requires all terms to be present
        # Use simple quote escaping for MATCH_ALL since it needs to be in the WHERE clause
        escaped_query = query.replace("'", "''")
        filter_clauses.append(f"MATCH_ALL({Field.CONTENT_KEY}, '{escaped_query}')")

        where_clause = " AND ".join(filter_clauses)

        # Execute full-text search query
        search_sql = f"""
        SELECT id, {Field.CONTENT_KEY}, {Field.METADATA_KEY}
        FROM {self._config.schema_name}.{self._table_name}
        WHERE {where_clause}
        LIMIT {top_k}
        """

        documents = []
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                try:
                    # Use hints parameter for full-text search optimization
                    fulltext_hints = {
                        "hints": {
                            "sdk.job.timeout": 30,  # Timeout for full-text search
                            "cz.sql.job.fast.mode": True,
                            "cz.sql.index.prewhere.enabled": True,
                        }
                    }
                    cursor.execute(search_sql, fulltext_hints)
                    results = cursor.fetchall()

                    for row in results:
                        # Parse metadata from JSON string (may be double-encoded)
                        try:
                            if row[2]:
                                # First parse may yield a string (double-encoded JSON)
                                first_pass = json.loads(row[2])

                                if isinstance(first_pass, str):
                                    metadata = parse_metadata_json(first_pass)
                                elif isinstance(first_pass, dict):
                                    metadata = first_pass
                                else:
                                    metadata = {}
                            else:
                                metadata = {}
                        except (json.JSONDecodeError, ValueError, TypeError):
                            logger.exception("JSON parsing failed")
                            # Fallback: extract document_id with regex

                            doc_id_match = re.search(r'"document_id":\s*"([^"]+)"', str(row[2] or ""))
                            metadata = {"document_id": doc_id_match.group(1)} if doc_id_match else {}

                        # Ensure required fields are set
                        metadata["doc_id"] = row[0]  # segment id

                        # Ensure document_id exists (critical for Dify's format_retrieval_documents)
                        if "document_id" not in metadata:
                            metadata["document_id"] = row[0]  # fallback to segment id

                        # Add a relevance score for full-text search
                        metadata["score"] = 1.0  # Clickzetta doesn't provide relevance scores
                        doc = Document(page_content=row[1], metadata=metadata)
                        documents.append(doc)
                except (RuntimeError, ValueError, TypeError, ConnectionError):
                    logger.exception("Full-text search failed")
                    # Fallback to LIKE search if full-text search fails
                    return self._search_by_like(query, **kwargs)

        return documents

    def _search_by_like(self, query: str, **kwargs: Any) -> list[Document]:
        """Fallback search using LIKE operator."""
        # Check if table exists first
        if not self._table_exists():
            logger.warning(
                "Table %s.%s does not exist, returning empty results",
                self._config.schema_name,
                self._table_name,
            )
            return []

        top_k = kwargs.get("top_k", 10)
        document_ids_filter = kwargs.get("document_ids_filter")

        # Handle filter parameter from canvas (workflow)
        _ = kwargs.get("filter", {})

        # Build filter clause
        filter_clauses = []
        if document_ids_filter:
            safe_doc_ids = [str(id).replace("'", "''") for id in document_ids_filter]
            doc_ids_str = ",".join(f"'{id}'" for id in safe_doc_ids)
            # Use json_extract_string function for ClickZetta compatibility
            filter_clauses.append(f"json_extract_string({Field.METADATA_KEY}, '$.document_id') IN ({doc_ids_str})")

        # No need for dataset_id filter since each dataset has its own table

        # Escape special characters for LIKE clause to prevent SQL injection
        from libs.helper import escape_like_pattern

        escaped_query = escape_like_pattern(query).replace("'", "''")
        filter_clauses.append(f"{Field.CONTENT_KEY} LIKE '%{escaped_query}%' ESCAPE '\\\\'")
        where_clause = " AND ".join(filter_clauses)

        search_sql = f"""
        SELECT id, {Field.CONTENT_KEY}, {Field.METADATA_KEY}
        FROM {self._config.schema_name}.{self._table_name}
        WHERE {where_clause}
        LIMIT {top_k}
        """

        documents = []
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                # Use hints parameter for LIKE search optimization
                like_hints = {
                    "hints": {
                        "sdk.job.timeout": 20,  # Timeout for LIKE search
                        "cz.sql.job.fast.mode": True,
                    }
                }
                cursor.execute(search_sql, like_hints)
                results = cursor.fetchall()

                for row in results:
                    # Parse metadata using centralized method
                    metadata = self._parse_metadata(row[2], row[0])

                    metadata["score"] = 0.5  # Lower score for LIKE search
                    doc = Document(page_content=row[1], metadata=metadata)
                    documents.append(doc)

        return documents

    def delete(self):
        """Delete the entire collection."""
        with self.get_connection_context() as connection:
            with connection.cursor() as cursor:
                cursor.execute(f"DROP TABLE IF EXISTS {self._config.schema_name}.{self._table_name}")

    def _format_vector_simple(self, vector: list[float]) -> str:
        """Simple vector formatting for SQL queries."""
        return ",".join(map(str, vector))

    def _safe_doc_id(self, doc_id: str) -> str:
        """Ensure doc_id is safe for SQL and doesn't contain special characters."""
        if not doc_id:
            return str(uuid.uuid4())
        # Remove or replace potentially problematic characters
        safe_id = str(doc_id)
        # Only allow alphanumeric, hyphens, underscores
        safe_id = "".join(c for c in safe_id if c.isalnum() or c in "-_")
        if not safe_id:  # If all characters were removed
            return str(uuid.uuid4())
        return safe_id[:255]  # Limit length


class ClickzettaVectorFactory(AbstractVectorFactory):
    """Factory for creating Clickzetta vector instances."""

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> BaseVector:
        """Initialize a Clickzetta vector instance."""
        # Get configuration from environment variables or dataset config
        config = ClickzettaConfig(
            username=dify_config.CLICKZETTA_USERNAME or "",
            password=dify_config.CLICKZETTA_PASSWORD or "",
            instance=dify_config.CLICKZETTA_INSTANCE or "",
            service=dify_config.CLICKZETTA_SERVICE or "api.clickzetta.com",
            workspace=dify_config.CLICKZETTA_WORKSPACE or "quick_start",
            vcluster=dify_config.CLICKZETTA_VCLUSTER or "default_ap",
            schema_name=dify_config.CLICKZETTA_SCHEMA or "dify",
            batch_size=dify_config.CLICKZETTA_BATCH_SIZE or 100,
            enable_inverted_index=dify_config.CLICKZETTA_ENABLE_INVERTED_INDEX or True,
            analyzer_type=dify_config.CLICKZETTA_ANALYZER_TYPE or "chinese",
            analyzer_mode=dify_config.CLICKZETTA_ANALYZER_MODE or "smart",
            vector_distance_function=dify_config.CLICKZETTA_VECTOR_DISTANCE_FUNCTION or "cosine_distance",
        )

        # Use dataset collection name as table name
        collection_name = Dataset.gen_collection_name_by_id(dataset.id).lower()

        return ClickzettaVector(collection_name=collection_name, config=config)

```

### api/core/rag/datasource/vdb/oracle/oraclevector.py
```py
import array
import json
import logging
import re
import uuid
from typing import Any

import jieba.posseg as pseg  # type: ignore
import numpy
import oracledb
from oracledb.connection import Connection
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)

oracledb.defaults.fetch_lobs = False


class OracleVectorConfig(BaseModel):
    user: str
    password: str
    dsn: str
    config_dir: str | None = None
    wallet_location: str | None = None
    wallet_password: str | None = None
    is_autonomous: bool = False

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["user"]:
            raise ValueError("config ORACLE_USER is required")
        if not values["password"]:
            raise ValueError("config ORACLE_PASSWORD is required")
        if not values["dsn"]:
            raise ValueError("config ORACLE_DSN is required")
        if values.get("is_autonomous", False):
            if not values.get("config_dir"):
                raise ValueError("config_dir is required for autonomous database")
            if not values.get("wallet_location"):
                raise ValueError("wallet_location is required for autonomous database")
            if not values.get("wallet_password"):
                raise ValueError("wallet_password is required for autonomous database")
        return values


SQL_CREATE_TABLE = """
CREATE TABLE IF NOT EXISTS {table_name} (
    id varchar2(100)
    ,text CLOB NOT NULL
    ,meta JSON
    ,embedding vector NOT NULL
)
"""
SQL_CREATE_INDEX = """
CREATE INDEX IF NOT EXISTS idx_docs_{table_name} ON {table_name}(text)
INDEXTYPE IS CTXSYS.CONTEXT PARAMETERS
('FILTER CTXSYS.NULL_FILTER SECTION GROUP CTXSYS.HTML_SECTION_GROUP LEXER world_lexer')
"""


class OracleVector(BaseVector):
    def __init__(self, collection_name: str, config: OracleVectorConfig):
        super().__init__(collection_name)
        self.pool = self._create_connection_pool(config)
        self.table_name = f"embedding_{collection_name}"
        self.config = config

    def get_type(self) -> str:
        return VectorType.ORACLE

    def numpy_converter_in(self, value):
        if value.dtype == numpy.float64:
            dtype = "d"
        elif value.dtype == numpy.float32:
            dtype = "f"
        else:
            dtype = "b"
        return array.array(dtype, value)

    def input_type_handler(self, cursor, value, arraysize):
        if isinstance(value, numpy.ndarray):
            return cursor.var(
                oracledb.DB_TYPE_VECTOR,
                arraysize=arraysize,
                inconverter=self.numpy_converter_in,
            )

    def numpy_converter_out(self, value):
        if value.typecode == "b":
            return numpy.array(value, copy=False, dtype=numpy.int8)
        elif value.typecode == "f":
            return numpy.array(value, copy=False, dtype=numpy.float32)
        else:
            return numpy.array(value, copy=False, dtype=numpy.float64)

    def output_type_handler(self, cursor, metadata):
        if metadata.type_code is oracledb.DB_TYPE_VECTOR:
            return cursor.var(
                metadata.type_code,
                arraysize=cursor.arraysize,
                outconverter=self.numpy_converter_out,
            )

    def _get_connection(self) -> Connection:
        if self.config.is_autonomous:
            connection = oracledb.connect(
                user=self.config.user,
                password=self.config.password,
                dsn=self.config.dsn,
                config_dir=self.config.config_dir,
                wallet_location=self.config.wallet_location,
                wallet_password=self.config.wallet_password,
            )
            return connection
        else:
            connection = oracledb.connect(user=self.config.user, password=self.config.password, dsn=self.config.dsn)
            return connection

    def _create_connection_pool(self, config: OracleVectorConfig):
        pool_params = {
            "user": config.user,
            "password": config.password,
            "dsn": config.dsn,
            "min": 1,
            "max": 5,
            "increment": 1,
        }
        if config.is_autonomous:
            pool_params.update(
                {
                    "config_dir": config.config_dir,
                    "wallet_location": config.wallet_location,
                    "wallet_password": config.wallet_password,
                }
            )
        return oracledb.create_pool(**pool_params)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        pks = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                pks.append(doc_id)
                values.append(
                    (
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata),
                        # array.array("f", embeddings[i]),
                        numpy.array(embeddings[i]),
                    )
                )
        with self._get_connection() as conn:
            conn.inputtypehandler = self.input_type_handler
            conn.outputtypehandler = self.output_type_handler
            # with conn.cursor() as cur:
            #    cur.executemany(
            #        f"INSERT INTO {self.table_name} (id, text, meta, embedding) VALUES (:1, :2, :3, :4)", values
            #    )
            # conn.commit()
            for value in values:
                with conn.cursor() as cur:
                    try:
                        cur.execute(
                            f"""INSERT INTO {self.table_name} (id, text, meta, embedding)
                        VALUES (:1, :2, :3, :4)""",
                            value,
                        )
                        conn.commit()
                    except Exception:
                        logger.exception("Failed to insert record %s into %s", value[0], self.table_name)
            conn.close()
        return pks

    def text_exists(self, id: str) -> bool:
        with self._get_connection() as conn:
            with conn.cursor() as cur:
                cur.execute(f"SELECT id FROM {self.table_name} WHERE id = :1", (id,))
                return cur.fetchone() is not None
            conn.close()

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        if not ids:
            return []
        with self._get_connection() as conn:
            with conn.cursor() as cur:
                placeholders = ", ".join(f":{i + 1}" for i in range(len(ids)))
                cur.execute(f"SELECT meta, text FROM {self.table_name} WHERE id IN ({placeholders})", ids)
                docs = []
                for record in cur:
                    docs.append(Document(page_content=record[1], metadata=record[0]))
            self.pool.release(connection=conn)
            conn.close()
        return docs

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        with self._get_connection() as conn:
            with conn.cursor() as cur:
                placeholders = ", ".join(f":{i + 1}" for i in range(len(ids)))
                cur.execute(f"DELETE FROM {self.table_name} WHERE id IN ({placeholders})", ids)
            conn.commit()
            conn.close()

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_connection() as conn:
            with conn.cursor() as cur:
                cur.execute(f"DELETE FROM {self.table_name} WHERE JSON_VALUE(meta, '$." + key + "') = :1", (value,))
            conn.commit()
            conn.close()

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search the nearest neighbors to a vector.

        :param query_vector: The input vector to search for similar items.
        :param top_k: The number of nearest neighbors to return, default is 5.
        :return: List of Documents that are nearest to the query vector.
        """
        # Validate and sanitize top_k to prevent SQL injection
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0 or top_k > 10000:
            top_k = 4  # Use default if invalid

        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        params = [numpy.array(query_vector)]

        if document_ids_filter:
            placeholders = ", ".join(f":{i + 2}" for i in range(len(document_ids_filter)))
            where_clause = f"WHERE JSON_VALUE(meta, '$.document_id') IN ({placeholders})"
            params.extend(document_ids_filter)

        with self._get_connection() as conn:
            conn.inputtypehandler = self.input_type_handler
            conn.outputtypehandler = self.output_type_handler
            with conn.cursor() as cur:
                cur.execute(
                    f"""SELECT meta, text, vector_distance(embedding,(select to_vector(:1) from dual),cosine)
                    AS distance FROM {self.table_name}
                    {where_clause} ORDER BY distance fetch first {top_k} rows only""",
                    params,
                )
                docs = []
                score_threshold = float(kwargs.get("score_threshold") or 0.0)
                for record in cur:
                    metadata, text, distance = record
                    score = 1 - distance
                    metadata["score"] = score
                    if score >= score_threshold:
                        docs.append(Document(page_content=text, metadata=metadata))
            conn.close()
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        # lazy import
        import nltk  # type: ignore
        from nltk.corpus import stopwords  # type: ignore

        # Validate and sanitize top_k to prevent SQL injection
        top_k = kwargs.get("top_k", 5)
        if not isinstance(top_k, int) or top_k <= 0 or top_k > 10000:
            top_k = 5  # Use default if invalid
        # just not implement fetch by score_threshold now, may be later
        if len(query) > 0:
            # Check which language the query is in
            zh_pattern = re.compile("[\u4e00-\u9fa5]+")
            match = zh_pattern.search(query)
            entities = []
            #  match: query condition maybe is a chinese sentence, so using Jieba split,else using nltk split
            if match:
                words = pseg.cut(query)
                current_entity = ""
                for word, pos in words:
                    # `nr`: Person, `ns`: Location, `nt`: Organization
                    if pos in {"nr", "Ng", "eng", "nz", "n", "ORG", "v"}:
                        current_entity += word
                    else:
                        if current_entity:
                            entities.append(current_entity)
                            current_entity = ""
                if current_entity:
                    entities.append(current_entity)
            else:
                try:
                    nltk.data.find("tokenizers/punkt")
                    nltk.data.find("corpora/stopwords")
                except LookupError:
                    raise LookupError("Unable to find the required NLTK data package: punkt and stopwords")
                e_str = re.sub(r"[^\w ]", "", query)
                all_tokens = nltk.word_tokenize(e_str)
                stop_words = stopwords.words("english")
                for token in all_tokens:
                    if token not in stop_words:
                        entities.append(token)
            with self._get_connection() as conn:
                with conn.cursor() as cur:
                    document_ids_filter = kwargs.get("document_ids_filter")
                    where_clause = ""
                    params: dict[str, Any] = {"kk": " ACCUM ".join(entities)}

                    if document_ids_filter:
                        placeholders = []
                        for i, doc_id in enumerate(document_ids_filter):
                            param_name = f"doc_id_{i}"
                            placeholders.append(f":{param_name}")
                            params[param_name] = doc_id
                        where_clause = f" AND JSON_VALUE(meta, '$.document_id') IN ({', '.join(placeholders)}) "

                    cur.execute(
                        f"""select meta, text, embedding FROM {self.table_name}
                    WHERE CONTAINS(text, :kk, 1) > 0  {where_clause}
                    order by score(1) desc fetch first {top_k} rows only""",
                        params,
                    )
                    docs = []
                    for record in cur:
                        metadata, text, embedding = record
                        docs.append(Document(page_content=text, vector=embedding, metadata=metadata))
                conn.close()
            return docs
        else:
            return [Document(page_content="", metadata={})]

    def delete(self):
        with self._get_connection() as conn:
            with conn.cursor() as cur:
                cur.execute(f"DROP TABLE IF EXISTS {self.table_name} cascade constraints")
            conn.commit()
            conn.close()

    def _create_collection(self, dimension: int):
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            with self._get_connection() as conn:
                with conn.cursor() as cur:
                    cur.execute(SQL_CREATE_TABLE.format(table_name=self.table_name))
                redis_client.set(collection_exist_cache_key, 1, ex=3600)
                with conn.cursor() as cur:
                    cur.execute(SQL_CREATE_INDEX.format(table_name=self.table_name))
                conn.commit()
                conn.close()


class OracleVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> OracleVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.ORACLE, collection_name))

        return OracleVector(
            collection_name=collection_name,
            config=OracleVectorConfig(
                user=dify_config.ORACLE_USER or "system",
                password=dify_config.ORACLE_PASSWORD or "oracle",
                dsn=dify_config.ORACLE_DSN or "oracle:1521/freepdb1",
                config_dir=dify_config.ORACLE_CONFIG_DIR,
                wallet_location=dify_config.ORACLE_WALLET_LOCATION,
                wallet_password=dify_config.ORACLE_WALLET_PASSWORD,
                is_autonomous=dify_config.ORACLE_IS_AUTONOMOUS,
            ),
        )

```

### api/core/rag/datasource/vdb/oracle/__init__.py
```py

```

### api/core/rag/datasource/vdb/vector_base.py
```py
from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any

from core.rag.models.document import Document


class BaseVector(ABC):
    def __init__(self, collection_name: str):
        self._collection_name = collection_name

    @abstractmethod
    def get_type(self) -> str:
        raise NotImplementedError

    @abstractmethod
    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs) -> list[str] | None:
        raise NotImplementedError

    @abstractmethod
    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs) -> list[str]:
        raise NotImplementedError

    @abstractmethod
    def text_exists(self, id: str) -> bool:
        raise NotImplementedError

    @abstractmethod
    def delete_by_ids(self, ids: list[str]) -> None:
        raise NotImplementedError

    def get_ids_by_metadata_field(self, key: str, value: str):
        raise NotImplementedError

    @abstractmethod
    def delete_by_metadata_field(self, key: str, value: str) -> None:
        raise NotImplementedError

    @abstractmethod
    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        raise NotImplementedError

    @abstractmethod
    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        raise NotImplementedError

    @abstractmethod
    def delete(self) -> None:
        raise NotImplementedError

    def _filter_duplicate_texts(self, texts: list[Document]) -> list[Document]:
        for text in texts.copy():
            if text.metadata and "doc_id" in text.metadata:
                doc_id = text.metadata["doc_id"]
                exists_duplicate_node = self.text_exists(doc_id)
                if exists_duplicate_node:
                    texts.remove(text)

        return texts

    def _get_uuids(self, texts: list[Document]) -> list[str]:
        return [text.metadata["doc_id"] for text in texts if text.metadata and "doc_id" in text.metadata]

    @property
    def collection_name(self):
        return self._collection_name

```

### api/core/rag/datasource/vdb/pyvastbase/__init__.py
```py

```

### api/core/rag/datasource/vdb/pyvastbase/vastbase_vector.py
```py
import json
import uuid
from contextlib import contextmanager
from typing import Any

import psycopg2.extras
import psycopg2.pool
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset


class VastbaseVectorConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    min_connection: int
    max_connection: int

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config VASTBASE_HOST is required")
        if not values["port"]:
            raise ValueError("config VASTBASE_PORT is required")
        if not values["user"]:
            raise ValueError("config VASTBASE_USER is required")
        if not values["password"]:
            raise ValueError("config VASTBASE_PASSWORD is required")
        if not values["database"]:
            raise ValueError("config VASTBASE_DATABASE is required")
        if not values["min_connection"]:
            raise ValueError("config VASTBASE_MIN_CONNECTION is required")
        if not values["max_connection"]:
            raise ValueError("config VASTBASE_MAX_CONNECTION is required")
        if values["min_connection"] > values["max_connection"]:
            raise ValueError("config VASTBASE_MIN_CONNECTION should less than VASTBASE_MAX_CONNECTION")
        return values


SQL_CREATE_TABLE = """
CREATE TABLE IF NOT EXISTS {table_name} (
    id UUID PRIMARY KEY,
    text TEXT NOT NULL,
    meta JSONB NOT NULL,
    embedding floatvector({dimension}) NOT NULL
);
"""

SQL_CREATE_INDEX = """
CREATE INDEX IF NOT EXISTS embedding_cosine_v1_idx ON {table_name}
USING hnsw (embedding floatvector_cosine_ops) WITH (m = 16, ef_construction = 64);
"""


class VastbaseVector(BaseVector):
    def __init__(self, collection_name: str, config: VastbaseVectorConfig):
        super().__init__(collection_name)
        self.pool = self._create_connection_pool(config)
        self.table_name = f"embedding_{collection_name}"

    def get_type(self) -> str:
        return VectorType.VASTBASE

    def _create_connection_pool(self, config: VastbaseVectorConfig):
        return psycopg2.pool.SimpleConnectionPool(
            config.min_connection,
            config.max_connection,
            host=config.host,
            port=config.port,
            user=config.user,
            password=config.password,
            database=config.database,
        )

    @contextmanager
    def _get_cursor(self):
        conn = self.pool.getconn()
        cur = conn.cursor()
        try:
            yield cur
        finally:
            cur.close()
            conn.commit()
            self.pool.putconn(conn)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        pks = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                pks.append(doc_id)
                values.append(
                    (
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata),
                        embeddings[i],
                    )
                )
        with self._get_cursor() as cur:
            psycopg2.extras.execute_values(
                cur, f"INSERT INTO {self.table_name} (id, text, meta, embedding) VALUES %s", values
            )
        return pks

    def text_exists(self, id: str) -> bool:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT id FROM {self.table_name} WHERE id = %s", (id,))
            return cur.fetchone() is not None

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT meta, text FROM {self.table_name} WHERE id IN %s", (tuple(ids),))
            docs = []
            for record in cur:
                docs.append(Document(page_content=record[1], metadata=record[0]))
        return docs

    def delete_by_ids(self, ids: list[str]):
        # Avoiding crashes caused by performing delete operations on empty lists in certain scenarios
        # Scenario 1: extract a document fails, resulting in a table not being created.
        # Then clicking the retry button triggers a delete operation on an empty list.
        if not ids:
            return
        with self._get_cursor() as cur:
            cur.execute(f"DELETE FROM {self.table_name} WHERE id IN %s", (tuple(ids),))

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_cursor() as cur:
            cur.execute(f"DELETE FROM {self.table_name} WHERE meta->>%s = %s", (key, value))

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search the nearest neighbors to a vector.

        :param query_vector: The input vector to search for similar items.
        :param top_k: The number of nearest neighbors to return, default is 5.
        :return: List of Documents that are nearest to the query vector.
        """
        top_k = kwargs.get("top_k", 4)

        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        with self._get_cursor() as cur:
            cur.execute(
                f"SELECT meta, text, embedding <=> %s AS distance FROM {self.table_name}"
                f" ORDER BY distance LIMIT {top_k}",
                (json.dumps(query_vector),),
            )
            docs = []
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            for record in cur:
                metadata, text, distance = record
                score = 1 - distance
                metadata["score"] = score
                if score >= score_threshold:
                    docs.append(Document(page_content=text, metadata=metadata))
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 5)

        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        with self._get_cursor() as cur:
            cur.execute(
                f"""SELECT meta, text, ts_rank(to_tsvector(coalesce(text, '')), plainto_tsquery(%s)) AS score
                FROM {self.table_name}
                WHERE to_tsvector(text) @@ plainto_tsquery(%s)
                ORDER BY score DESC
                LIMIT {top_k}""",
                # f"'{query}'" is required in order to account for whitespace in query
                (f"'{query}'", f"'{query}'"),
            )

            docs = []

            for record in cur:
                metadata, text, score = record
                metadata["score"] = score
                docs.append(Document(page_content=text, metadata=metadata))

        return docs

    def delete(self):
        with self._get_cursor() as cur:
            cur.execute(f"DROP TABLE IF EXISTS {self.table_name}")

    def _create_collection(self, dimension: int):
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            with self._get_cursor() as cur:
                cur.execute(SQL_CREATE_TABLE.format(table_name=self.table_name, dimension=dimension))
                # Vastbase supports vector dimensions in the range [1, 16,000]
                if dimension <= 16000:
                    cur.execute(SQL_CREATE_INDEX.format(table_name=self.table_name))
            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class VastbaseVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> VastbaseVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.VASTBASE, collection_name))

        return VastbaseVector(
            collection_name=collection_name,
            config=VastbaseVectorConfig(
                host=dify_config.VASTBASE_HOST or "localhost",
                port=dify_config.VASTBASE_PORT,
                user=dify_config.VASTBASE_USER or "dify",
                password=dify_config.VASTBASE_PASSWORD or "",
                database=dify_config.VASTBASE_DATABASE or "dify",
                min_connection=dify_config.VASTBASE_MIN_CONNECTION,
                max_connection=dify_config.VASTBASE_MAX_CONNECTION,
            ),
        )

```

### api/core/rag/datasource/vdb/__init__.py
```py

```

### api/core/rag/datasource/vdb/vector_factory.py
```py
import base64
import logging
import time
from abc import ABC, abstractmethod
from typing import Any

from graphon.model_runtime.entities.model_entities import ModelType
from sqlalchemy import select

from configs import dify_config
from core.model_manager import ModelManager
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.cached_embedding import CacheEmbedding
from core.rag.embedding.embedding_base import Embeddings
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from extensions.ext_storage import storage
from models.dataset import Dataset, Whitelist
from models.model import UploadFile

logger = logging.getLogger(__name__)


class AbstractVectorFactory(ABC):
    @abstractmethod
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> BaseVector:
        raise NotImplementedError

    @staticmethod
    def gen_index_struct_dict(vector_type: VectorType, collection_name: str):
        index_struct_dict = {"type": vector_type, "vector_store": {"class_prefix": collection_name}}
        return index_struct_dict


class Vector:
    def __init__(self, dataset: Dataset, attributes: list | None = None):
        if attributes is None:
            attributes = ["doc_id", "dataset_id", "document_id", "doc_hash", "doc_type"]
        self._dataset = dataset
        self._embeddings = self._get_embeddings()
        self._attributes = attributes
        self._vector_processor = self._init_vector()

    def _init_vector(self) -> BaseVector:
        vector_type = dify_config.VECTOR_STORE

        if self._dataset.index_struct_dict:
            vector_type = self._dataset.index_struct_dict["type"]
        else:
            if dify_config.VECTOR_STORE_WHITELIST_ENABLE:
                stmt = select(Whitelist).where(
                    Whitelist.tenant_id == self._dataset.tenant_id, Whitelist.category == "vector_db"
                )
                whitelist = db.session.scalars(stmt).one_or_none()
                if whitelist:
                    vector_type = VectorType.TIDB_ON_QDRANT

        if not vector_type:
            raise ValueError("Vector store must be specified.")

        vector_factory_cls = self.get_vector_factory(vector_type)
        return vector_factory_cls().init_vector(self._dataset, self._attributes, self._embeddings)

    @staticmethod
    def get_vector_factory(vector_type: str) -> type[AbstractVectorFactory]:
        match vector_type:
            case VectorType.CHROMA:
                from core.rag.datasource.vdb.chroma.chroma_vector import ChromaVectorFactory

                return ChromaVectorFactory
            case VectorType.MILVUS:
                from core.rag.datasource.vdb.milvus.milvus_vector import MilvusVectorFactory

                return MilvusVectorFactory
            case VectorType.ALIBABACLOUD_MYSQL:
                from core.rag.datasource.vdb.alibabacloud_mysql.alibabacloud_mysql_vector import (
                    AlibabaCloudMySQLVectorFactory,
                )

                return AlibabaCloudMySQLVectorFactory
            case VectorType.MYSCALE:
                from core.rag.datasource.vdb.myscale.myscale_vector import MyScaleVectorFactory

                return MyScaleVectorFactory
            case VectorType.PGVECTOR:
                from core.rag.datasource.vdb.pgvector.pgvector import PGVectorFactory

                return PGVectorFactory
            case VectorType.VASTBASE:
                from core.rag.datasource.vdb.pyvastbase.vastbase_vector import VastbaseVectorFactory

                return VastbaseVectorFactory
            case VectorType.PGVECTO_RS:
                from core.rag.datasource.vdb.pgvecto_rs.pgvecto_rs import PGVectoRSFactory

                return PGVectoRSFactory
            case VectorType.QDRANT:
                from core.rag.datasource.vdb.qdrant.qdrant_vector import QdrantVectorFactory

                return QdrantVectorFactory
            case VectorType.RELYT:
                from core.rag.datasource.vdb.relyt.relyt_vector import RelytVectorFactory

                return RelytVectorFactory
            case VectorType.ELASTICSEARCH:
                from core.rag.datasource.vdb.elasticsearch.elasticsearch_vector import ElasticSearchVectorFactory

                return ElasticSearchVectorFactory
            case VectorType.ELASTICSEARCH_JA:
                from core.rag.datasource.vdb.elasticsearch.elasticsearch_ja_vector import (
                    ElasticSearchJaVectorFactory,
                )

                return ElasticSearchJaVectorFactory
            case VectorType.TIDB_VECTOR:
                from core.rag.datasource.vdb.tidb_vector.tidb_vector import TiDBVectorFactory

                return TiDBVectorFactory
            case VectorType.WEAVIATE:
                from core.rag.datasource.vdb.weaviate.weaviate_vector import WeaviateVectorFactory

                return WeaviateVectorFactory
            case VectorType.TENCENT:
                from core.rag.datasource.vdb.tencent.tencent_vector import TencentVectorFactory

                return TencentVectorFactory
            case VectorType.ORACLE:
                from core.rag.datasource.vdb.oracle.oraclevector import OracleVectorFactory

                return OracleVectorFactory
            case VectorType.OPENSEARCH:
                from core.rag.datasource.vdb.opensearch.opensearch_vector import OpenSearchVectorFactory

                return OpenSearchVectorFactory
            case VectorType.ANALYTICDB:
                from core.rag.datasource.vdb.analyticdb.analyticdb_vector import AnalyticdbVectorFactory

                return AnalyticdbVectorFactory
            case VectorType.COUCHBASE:
                from core.rag.datasource.vdb.couchbase.couchbase_vector import CouchbaseVectorFactory

                return CouchbaseVectorFactory
            case VectorType.BAIDU:
                from core.rag.datasource.vdb.baidu.baidu_vector import BaiduVectorFactory

                return BaiduVectorFactory
            case VectorType.VIKINGDB:
                from core.rag.datasource.vdb.vikingdb.vikingdb_vector import VikingDBVectorFactory

                return VikingDBVectorFactory
            case VectorType.UPSTASH:
                from core.rag.datasource.vdb.upstash.upstash_vector import UpstashVectorFactory

                return UpstashVectorFactory
            case VectorType.TIDB_ON_QDRANT:
                from core.rag.datasource.vdb.tidb_on_qdrant.tidb_on_qdrant_vector import TidbOnQdrantVectorFactory

                return TidbOnQdrantVectorFactory
            case VectorType.LINDORM:
                from core.rag.datasource.vdb.lindorm.lindorm_vector import LindormVectorStoreFactory

                return LindormVectorStoreFactory
            case VectorType.OCEANBASE | VectorType.SEEKDB:
                from core.rag.datasource.vdb.oceanbase.oceanbase_vector import OceanBaseVectorFactory

                return OceanBaseVectorFactory
            case VectorType.OPENGAUSS:
                from core.rag.datasource.vdb.opengauss.opengauss import OpenGaussFactory

                return OpenGaussFactory
            case VectorType.TABLESTORE:
                from core.rag.datasource.vdb.tablestore.tablestore_vector import TableStoreVectorFactory

                return TableStoreVectorFactory
            case VectorType.HUAWEI_CLOUD:
                from core.rag.datasource.vdb.huawei.huawei_cloud_vector import HuaweiCloudVectorFactory

                return HuaweiCloudVectorFactory
            case VectorType.MATRIXONE:
                from core.rag.datasource.vdb.matrixone.matrixone_vector import MatrixoneVectorFactory

                return MatrixoneVectorFactory
            case VectorType.CLICKZETTA:
                from core.rag.datasource.vdb.clickzetta.clickzetta_vector import ClickzettaVectorFactory

                return ClickzettaVectorFactory
            case VectorType.IRIS:
                from core.rag.datasource.vdb.iris.iris_vector import IrisVectorFactory

                return IrisVectorFactory
            case VectorType.HOLOGRES:
                from core.rag.datasource.vdb.hologres.hologres_vector import HologresVectorFactory

                return HologresVectorFactory
            case _:
                raise ValueError(f"Vector store {vector_type} is not supported.")

    def create(self, texts: list | None = None, **kwargs):
        if texts:
            start = time.time()
            logger.info("start embedding %s texts %s", len(texts), start)
            batch_size = 1000
            total_batches = len(texts) + batch_size - 1
            for i in range(0, len(texts), batch_size):
                batch = texts[i : i + batch_size]
                batch_start = time.time()
                logger.info("Processing batch %s/%s (%s texts)", i // batch_size + 1, total_batches, len(batch))
                batch_embeddings = self._embeddings.embed_documents([document.page_content for document in batch])
                logger.info(
                    "Embedding batch %s/%s took %s s", i // batch_size + 1, total_batches, time.time() - batch_start
                )
                self._vector_processor.create(texts=batch, embeddings=batch_embeddings, **kwargs)
            logger.info("Embedding %s texts took %s s", len(texts), time.time() - start)

    def create_multimodal(self, file_documents: list | None = None, **kwargs):
        if file_documents:
            start = time.time()
            logger.info("start embedding %s files %s", len(file_documents), start)
            batch_size = 1000
            total_batches = len(file_documents) + batch_size - 1
            for i in range(0, len(file_documents), batch_size):
                batch = file_documents[i : i + batch_size]
                batch_start = time.time()
                logger.info("Processing batch %s/%s (%s files)", i // batch_size + 1, total_batches, len(batch))

                # Batch query all upload files to avoid N+1 queries
                attachment_ids = [doc.metadata["doc_id"] for doc in batch]
                stmt = select(UploadFile).where(UploadFile.id.in_(attachment_ids))
                upload_files = db.session.scalars(stmt).all()
                upload_file_map = {str(f.id): f for f in upload_files}

                file_base64_list = []
                real_batch = []
                for document in batch:
                    attachment_id = document.metadata["doc_id"]
                    doc_type = document.metadata["doc_type"]
                    upload_file = upload_file_map.get(attachment_id)
                    if upload_file:
                        blob = storage.load_once(upload_file.key)
                        file_base64_str = base64.b64encode(blob).decode()
                        file_base64_list.append(
                            {
                                "content": file_base64_str,
                                "content_type": doc_type,
                                "file_id": attachment_id,
                            }
                        )
                        real_batch.append(document)
                batch_embeddings = self._embeddings.embed_multimodal_documents(file_base64_list)
                logger.info(
                    "Embedding batch %s/%s took %s s", i // batch_size + 1, total_batches, time.time() - batch_start
                )
                self._vector_processor.create(texts=real_batch, embeddings=batch_embeddings, **kwargs)
            logger.info("Embedding %s files took %s s", len(file_documents), time.time() - start)

    def add_texts(self, documents: list[Document], **kwargs):
        if kwargs.get("duplicate_check", False):
            documents = self._filter_duplicate_texts(documents)

        embeddings = self._embeddings.embed_documents([document.page_content for document in documents])
        self._vector_processor.create(texts=documents, embeddings=embeddings, **kwargs)

    def text_exists(self, id: str) -> bool:
        return self._vector_processor.text_exists(id)

    def delete_by_ids(self, ids: list[str]):
        self._vector_processor.delete_by_ids(ids)

    def delete_by_metadata_field(self, key: str, value: str):
        self._vector_processor.delete_by_metadata_field(key, value)

    def search_by_vector(self, query: str, **kwargs: Any) -> list[Document]:
        query_vector = self._embeddings.embed_query(query)
        return self._vector_processor.search_by_vector(query_vector, **kwargs)

    def search_by_file(self, file_id: str, **kwargs: Any) -> list[Document]:
        upload_file: UploadFile | None = db.session.get(UploadFile, file_id)

        if not upload_file:
            return []
        blob = storage.load_once(upload_file.key)
        file_base64_str = base64.b64encode(blob).decode()
        multimodal_vector = self._embeddings.embed_multimodal_query(
            {
                "content": file_base64_str,
                "content_type": DocType.IMAGE,
                "file_id": file_id,
            }
        )
        return self._vector_processor.search_by_vector(multimodal_vector, **kwargs)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return self._vector_processor.search_by_full_text(query, **kwargs)

    def delete(self):
        self._vector_processor.delete()
        # delete collection redis cache
        if self._vector_processor.collection_name:
            collection_exist_cache_key = f"vector_indexing_{self._vector_processor.collection_name}"
            redis_client.delete(collection_exist_cache_key)

    def _get_embeddings(self) -> Embeddings:
        model_manager = ModelManager.for_tenant(tenant_id=self._dataset.tenant_id)

        embedding_model = model_manager.get_model_instance(
            tenant_id=self._dataset.tenant_id,
            provider=self._dataset.embedding_model_provider,
            model_type=ModelType.TEXT_EMBEDDING,
            model=self._dataset.embedding_model,
        )
        return CacheEmbedding(embedding_model)

    def _filter_duplicate_texts(self, texts: list[Document]) -> list[Document]:
        for text in texts.copy():
            if text.metadata is None:
                continue
            doc_id = text.metadata["doc_id"]
            if doc_id:
                exists_duplicate_node = self.text_exists(doc_id)
                if exists_duplicate_node:
                    texts.remove(text)

        return texts

    def __getattr__(self, name):
        if self._vector_processor is not None:
            method = getattr(self._vector_processor, name)
            if callable(method):
                return method

        raise AttributeError(f"'vector_processor' object has no attribute '{name}'")

```

### api/core/rag/datasource/vdb/relyt/__init__.py
```py

```

### api/core/rag/datasource/vdb/relyt/relyt_vector.py
```py
import json
import logging
import uuid
from typing import Any

from pydantic import BaseModel, model_validator
from sqlalchemy import Column, String, Table, create_engine, insert
from sqlalchemy import text as sql_text
from sqlalchemy.dialects.postgresql import JSON, TEXT
from sqlalchemy.orm import Session

from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from models.dataset import Dataset

try:
    from sqlalchemy.orm import declarative_base
except ImportError:
    from sqlalchemy.ext.declarative import declarative_base

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.models.document import Document
from extensions.ext_redis import redis_client

logger = logging.getLogger(__name__)

Base = declarative_base()  # type: Any


class RelytConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config RELYT_HOST is required")
        if not values["port"]:
            raise ValueError("config RELYT_PORT is required")
        if not values["user"]:
            raise ValueError("config RELYT_USER is required")
        if not values["password"]:
            raise ValueError("config RELYT_PASSWORD is required")
        if not values["database"]:
            raise ValueError("config RELYT_DATABASE is required")
        return values


class RelytVector(BaseVector):
    def __init__(self, collection_name: str, config: RelytConfig, group_id: str):
        super().__init__(collection_name)
        self.embedding_dimension = 1536
        self._client_config = config
        self._url = (
            f"postgresql+psycopg2://{config.user}:{config.password}@{config.host}:{config.port}/{config.database}"
        )
        self.client = create_engine(self._url)
        self._fields: list[str] = []
        self._group_id = group_id

    def get_type(self) -> str:
        return VectorType.RELYT

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self.create_collection(len(embeddings[0]))
        self.embedding_dimension = len(embeddings[0])
        self.add_texts(texts, embeddings)

    def create_collection(self, dimension: int):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            index_name = f"{self._collection_name}_embedding_index"
            with Session(self.client) as session:
                drop_statement = sql_text(f"""DROP TABLE IF EXISTS "{self._collection_name}"; """)
                session.execute(drop_statement)
                create_statement = sql_text(f"""
                    CREATE TABLE IF NOT EXISTS "{self._collection_name}" (
                        id TEXT PRIMARY KEY,
                        document TEXT NOT NULL,
                        metadata JSON NOT NULL,
                        embedding vector({dimension}) NOT NULL
                    ) using heap;
                """)
                session.execute(create_statement)
                index_statement = sql_text(f"""
                        CREATE INDEX {index_name}
                        ON "{self._collection_name}" USING vectors(embedding vector_l2_ops)
                        WITH (options = $$
                                optimizing.optimizing_threads = 30
                                segment.max_growing_segment_size = 2000
                                segment.max_sealed_segment_size = 30000000
                                [indexing.hnsw]
                                m=30
                                ef_construction=500
                                $$);
                    """)
                session.execute(index_statement)
                session.commit()
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        from pgvecto_rs.sqlalchemy import VECTOR  # type: ignore

        ids = [str(uuid.uuid1()) for _ in documents]
        metadatas = [d.metadata for d in documents if d.metadata is not None]
        for metadata in metadatas:
            metadata["group_id"] = self._group_id
        texts = [d.page_content for d in documents]

        # Define the table schema
        chunks_table = Table(
            self._collection_name,
            Base.metadata,
            Column("id", TEXT, primary_key=True),
            Column("embedding", VECTOR(len(embeddings[0]))),
            Column("document", String, nullable=True),
            Column("metadata", JSON, nullable=True),
            extend_existing=True,
        )

        chunks_table_data = []
        with self.client.connect() as conn, conn.begin():
            for document, metadata, chunk_id, embedding in zip(texts, metadatas, ids, embeddings):
                chunks_table_data.append(
                    {
                        "id": chunk_id,
                        "embedding": embedding,
                        "document": document,
                        "metadata": metadata,
                    }
                )

                # Execute the batch insert when the batch size is reached
                if len(chunks_table_data) == 500:
                    conn.execute(insert(chunks_table).values(chunks_table_data))
                    # Clear the chunks_table_data list for the next batch
                    chunks_table_data.clear()

            # Insert any remaining records that didn't make up a full batch
            if chunks_table_data:
                conn.execute(insert(chunks_table).values(chunks_table_data))

        return ids

    def get_ids_by_metadata_field(self, key: str, value: str):
        result = None
        with Session(self.client) as session:
            select_statement = sql_text(f"""SELECT id FROM "{self._collection_name}" WHERE metadata->>:key = :value""")
            result = session.execute(select_statement, {"key": key, "value": value}).fetchall()
        if result:
            return [item[0] for item in result]
        else:
            return None

    def delete_by_uuids(self, ids: list[str] | None = None):
        """Delete by vector IDs.

        Args:
            ids: List of ids to delete.
        """
        from pgvecto_rs.sqlalchemy import VECTOR

        if ids is None:
            raise ValueError("No ids provided to delete.")

        # Define the table schema
        chunks_table = Table(
            self._collection_name,
            Base.metadata,
            Column("id", TEXT, primary_key=True),
            Column("embedding", VECTOR(self.embedding_dimension)),
            Column("document", String, nullable=True),
            Column("metadata", JSON, nullable=True),
            extend_existing=True,
        )

        try:
            with self.client.connect() as conn, conn.begin():
                delete_condition = chunks_table.c.id.in_(ids)
                conn.execute(chunks_table.delete().where(delete_condition))
                return True
        except Exception:
            logger.exception("Delete operation failed for collection %s", self._collection_name)
            return False

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self.delete_by_uuids(ids)

    def delete_by_ids(self, ids: list[str]):
        with Session(self.client) as session:
            select_statement = sql_text(
                f"""SELECT id FROM "{self._collection_name}" WHERE metadata->>'doc_id' = ANY(:doc_ids)"""
            )
            result = session.execute(select_statement, {"doc_ids": ids}).fetchall()
        if result:
            ids = [item[0] for item in result]
            self.delete_by_uuids(ids)

    def delete(self):
        with Session(self.client) as session:
            session.execute(sql_text(f"""DROP TABLE IF EXISTS "{self._collection_name}";"""))
            session.commit()

    def text_exists(self, id: str) -> bool:
        with Session(self.client) as session:
            select_statement = sql_text(
                f"""SELECT id FROM "{self._collection_name}" WHERE metadata->>'doc_id' = :doc_id limit 1"""
            )
            result = session.execute(select_statement, {"doc_id": id}).fetchall()
        return len(result) > 0

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = kwargs.get("filter", {})
        if document_ids_filter:
            filter["document_id"] = document_ids_filter
        results = self.similarity_search_with_score_by_vector(
            k=int(kwargs.get("top_k", 4)), embedding=query_vector, filter=filter
        )

        # Organize results.
        docs = []
        for document, score in results:
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            if 1 - score >= score_threshold:
                docs.append(document)
        return docs

    def similarity_search_with_score_by_vector(
        self,
        embedding: list[float],
        k: int = 4,
        filter: dict | None = None,
    ) -> list[tuple[Document, float]]:
        # Add the filter if provided

        filter_condition = ""
        if filter is not None:
            conditions = [
                f"metadata->>'{key!r}' in ({', '.join(map(repr, value))})"
                if len(value) > 1
                else f"metadata->>'{key!r}' = {value[0]!r}"
                for key, value in filter.items()
            ]
            filter_condition = f"WHERE {' AND '.join(conditions)}"

        # Define the base query
        sql_query = f"""
            set vectors.enable_search_growing = on;
            set vectors.enable_search_write = on;
            SELECT document, metadata, embedding <-> :embedding as distance
            FROM "{self._collection_name}"
            {filter_condition}
            ORDER BY embedding <-> :embedding
            LIMIT :k
        """

        # Set up the query parameters
        embedding_str = ", ".join(format(x) for x in embedding)
        embedding_str = "[" + embedding_str + "]"
        params = {"embedding": embedding_str, "k": k}

        # Execute the query and fetch the results
        with self.client.connect() as conn:
            results = conn.execute(sql_text(sql_query), params).fetchall()

        documents_with_scores = [
            (
                Document(
                    page_content=result.document,
                    metadata=result.metadata,
                ),
                result.distance,
            )
            for result in results
        ]
        return documents_with_scores

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        # milvus/zilliz/relyt doesn't support bm25 search
        return []


class RelytVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> RelytVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.RELYT, collection_name))

        return RelytVector(
            collection_name=collection_name,
            config=RelytConfig(
                host=dify_config.RELYT_HOST or "localhost",
                port=dify_config.RELYT_PORT,
                user=dify_config.RELYT_USER or "",
                password=dify_config.RELYT_PASSWORD or "",
                database=dify_config.RELYT_DATABASE or "default",
            ),
            group_id=dataset.id,
        )

```

### api/core/rag/datasource/vdb/oceanbase/oceanbase_vector.py
```py
import json
import logging
import re
from typing import Any, Literal

from pydantic import BaseModel, model_validator
from pyobvector import VECTOR, ObVecClient, cosine_distance, inner_product, l2_distance  # type: ignore
from sqlalchemy import JSON, Column, String
from sqlalchemy.dialects.mysql import LONGTEXT
from sqlalchemy.exc import SQLAlchemyError

from configs import dify_config
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)

OCEANBASE_SUPPORTED_VECTOR_INDEX_TYPE = "HNSW"
_VALID_TABLE_NAME_RE = re.compile(r"^[a-zA-Z0-9_]+$")

_DISTANCE_FUNC_MAP = {
    "l2": l2_distance,
    "cosine": cosine_distance,
    "inner_product": inner_product,
}


class OceanBaseVectorConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    enable_hybrid_search: bool = False
    batch_size: int = 100
    metric_type: Literal["l2", "cosine", "inner_product"] = "l2"
    hnsw_m: int = 16
    hnsw_ef_construction: int = 256
    hnsw_ef_search: int = -1
    pool_size: int = 5
    max_overflow: int = 10
    hnsw_refresh_threshold: int = 1000

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config OCEANBASE_VECTOR_HOST is required")
        if not values["port"]:
            raise ValueError("config OCEANBASE_VECTOR_PORT is required")
        if not values["user"]:
            raise ValueError("config OCEANBASE_VECTOR_USER is required")
        if not values["database"]:
            raise ValueError("config OCEANBASE_VECTOR_DATABASE is required")
        return values


class OceanBaseVector(BaseVector):
    def __init__(self, collection_name: str, config: OceanBaseVectorConfig):
        if not _VALID_TABLE_NAME_RE.match(collection_name):
            raise ValueError(
                f"Invalid collection name '{collection_name}': "
                "only alphanumeric characters and underscores are allowed."
            )
        super().__init__(collection_name)
        self._config = config
        self._hnsw_ef_search = self._config.hnsw_ef_search
        self._client = ObVecClient(
            uri=f"{self._config.host}:{self._config.port}",
            user=self._config.user,
            password=self._config.password,
            db_name=self._config.database,
            pool_size=self._config.pool_size,
            max_overflow=self._config.max_overflow,
            pool_recycle=3600,
            pool_pre_ping=True,
        )
        self._fields: list[str] = []  # List of fields in the collection
        if self._client.check_table_exists(collection_name):
            self._load_collection_fields()
        self._hybrid_search_enabled = self._check_hybrid_search_support()  # Check if hybrid search is supported

    def get_type(self) -> str:
        return VectorType.OCEANBASE

    def _load_collection_fields(self):
        """
        Load collection fields from the database table.
        This method populates the _fields list with column names from the table.
        """
        try:
            if self._collection_name in self._client.metadata_obj.tables:
                table = self._client.metadata_obj.tables[self._collection_name]
                # Store all column names except 'id' (primary key)
                self._fields = [column.name for column in table.columns if column.name != "id"]
                logger.debug("Loaded fields for collection '%s': %s", self._collection_name, self._fields)
            else:
                logger.warning("Collection '%s' not found in metadata", self._collection_name)
        except Exception as e:
            logger.warning("Failed to load collection fields for '%s': %s", self._collection_name, str(e))

    def field_exists(self, field: str) -> bool:
        """
        Check if a field exists in the collection.

        :param field: Field name to check
        :return: True if field exists, False otherwise
        """
        return field in self._fields

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self._vec_dim = len(embeddings[0])
        self._create_collection()
        self.add_texts(texts, embeddings)

    def _create_collection(self):
        lock_name = "vector_indexing_lock_" + self._collection_name
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = "vector_indexing_" + self._collection_name
            if redis_client.get(collection_exist_cache_key):
                return

            if self._client.check_table_exists(self._collection_name):
                return

            self.delete()

            vals = []
            params = self._client.perform_raw_text_sql("SHOW PARAMETERS LIKE '%ob_vector_memory_limit_percentage%'")
            for row in params:
                val = int(row[6])
                vals.append(val)
            if len(vals) == 0:
                raise ValueError("ob_vector_memory_limit_percentage not found in parameters.")
            if any(val == 0 for val in vals):
                try:
                    self._client.perform_raw_text_sql("ALTER SYSTEM SET ob_vector_memory_limit_percentage = 30")
                except Exception as e:
                    raise Exception(
                        "Failed to set ob_vector_memory_limit_percentage. "
                        + "Maybe the database user has insufficient privilege.",
                        e,
                    )

            cols = [
                Column("id", String(36), primary_key=True, autoincrement=False),
                Column("vector", VECTOR(self._vec_dim)),
                Column("text", LONGTEXT),
                Column("metadata", JSON),
            ]
            vidx_params = self._client.prepare_index_params()
            vidx_params.add_index(
                field_name="vector",
                index_type=OCEANBASE_SUPPORTED_VECTOR_INDEX_TYPE,
                index_name="vector_index",
                metric_type=self._config.metric_type,
                params={"M": self._config.hnsw_m, "efConstruction": self._config.hnsw_ef_construction},
            )

            self._client.create_table_with_index_params(
                table_name=self._collection_name,
                columns=cols,
                vidxs=vidx_params,
            )
            logger.debug("DEBUG: Table '%s' created successfully", self._collection_name)

            if self._hybrid_search_enabled:
                # Get parser from config or use default ik parser
                parser_name = dify_config.OCEANBASE_FULLTEXT_PARSER or "ik"

                allowed_parsers = ["ngram", "beng", "space", "ngram2", "ik", "japanese_ftparser", "thai_ftparser"]
                if parser_name not in allowed_parsers:
                    raise ValueError(
                        f"Invalid OceanBase full-text parser: {parser_name}. "
                        f"Allowed values are: {', '.join(allowed_parsers)}"
                    )
                logger.debug("Hybrid search is enabled, parser_name='%s'", parser_name)
                logger.debug(
                    "About to create fulltext index for collection '%s' using parser '%s'",
                    self._collection_name,
                    parser_name,
                )
                try:
                    sql_command = f"""ALTER TABLE {self._collection_name}
                    ADD FULLTEXT INDEX fulltext_index_for_col_text (text) WITH PARSER {parser_name}"""
                    logger.debug("DEBUG: Executing SQL: %s", sql_command)
                    self._client.perform_raw_text_sql(sql_command)
                    logger.debug("DEBUG: Fulltext index created successfully for '%s'", self._collection_name)
                except Exception as e:
                    logger.exception("Exception occurred while creating fulltext index")
                    raise Exception(
                        "Failed to add fulltext index to the target table, your OceanBase version must be "
                        "4.3.5.1 or above to support fulltext index and vector index in the same table"
                    ) from e
            else:
                logger.debug("DEBUG: Hybrid search is NOT enabled for '%s'", self._collection_name)

            try:
                self._client.perform_raw_text_sql(
                    f"CREATE INDEX IF NOT EXISTS idx_metadata_doc_id ON `{self._collection_name}` "
                    f"((CAST(metadata->>'$.document_id' AS CHAR(64))))"
                )
            except SQLAlchemyError:
                logger.warning(
                    "Failed to create metadata functional index on '%s'; metadata queries may be slow without it.",
                    self._collection_name,
                )

            self._client.refresh_metadata([self._collection_name])
            self._load_collection_fields()
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def _check_hybrid_search_support(self) -> bool:
        """
        Check if the current OceanBase version supports hybrid search.
        Returns True if the version is >= 4.3.5.1, otherwise False.
        """
        if not self._config.enable_hybrid_search:
            return False

        try:
            from packaging import version

            # return OceanBase_CE 4.3.5.1 (r101000042025031818-bxxxx) (Built Mar 18 2025 18:13:36)
            result = self._client.perform_raw_text_sql("SELECT @@version_comment AS version")
            ob_full_version = result.fetchone()[0]
            ob_version = ob_full_version.split()[1]
            logger.debug("Current OceanBase version is %s", ob_version)
            return version.parse(ob_version) >= version.parse("4.3.5.1")
        except Exception as e:
            logger.warning("Failed to check OceanBase version: %s. Disabling hybrid search.", str(e))
            return False

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        ids = self._get_uuids(documents)
        batch_size = self._config.batch_size
        total = len(documents)

        all_data = [
            {
                "id": doc_id,
                "vector": emb,
                "text": doc.page_content,
                "metadata": doc.metadata,
            }
            for doc_id, doc, emb in zip(ids, documents, embeddings)
        ]

        for start in range(0, total, batch_size):
            batch = all_data[start : start + batch_size]
            try:
                self._client.insert(
                    table_name=self._collection_name,
                    data=batch,
                )
            except Exception as e:
                logger.exception(
                    "Failed to insert batch [%d:%d] into collection '%s'",
                    start,
                    start + len(batch),
                    self._collection_name,
                )
                raise Exception(
                    f"Failed to insert batch [{start}:{start + len(batch)}] into collection '{self._collection_name}'"
                ) from e

        if self._config.hnsw_refresh_threshold > 0 and total >= self._config.hnsw_refresh_threshold:
            try:
                self._client.refresh_index(
                    table_name=self._collection_name,
                    index_name="vector_index",
                )
            except SQLAlchemyError:
                logger.warning(
                    "Failed to refresh HNSW index after inserting %d documents into '%s'",
                    total,
                    self._collection_name,
                )

    def text_exists(self, id: str) -> bool:
        try:
            cur = self._client.get(table_name=self._collection_name, ids=id)
            return bool(cur.rowcount != 0)
        except Exception as e:
            logger.exception(
                "Failed to check if text exists with id '%s' in collection '%s'",
                id,
                self._collection_name,
            )
            raise Exception(f"Failed to check text existence for id '{id}'") from e

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        try:
            self._client.delete(table_name=self._collection_name, ids=ids)
            logger.debug("Deleted %d documents from collection '%s'", len(ids), self._collection_name)
        except Exception as e:
            logger.exception(
                "Failed to delete %d documents from collection '%s'",
                len(ids),
                self._collection_name,
            )
            raise Exception(f"Failed to delete documents from collection '{self._collection_name}'") from e

    def get_ids_by_metadata_field(self, key: str, value: str) -> list[str]:
        try:
            import re

            from sqlalchemy import text

            # Validate key to prevent injection in JSON path
            if not re.match(r"^[a-zA-Z0-9_.]+$", key):
                raise ValueError(f"Invalid characters in metadata key: {key}")

            # Use parameterized query to prevent SQL injection
            sql = text(f"SELECT id FROM `{self._collection_name}` WHERE metadata->>'$.{key}' = :value")

            with self._client.engine.connect() as conn:
                result = conn.execute(sql, {"value": value})
                ids = [row[0] for row in result]

            logger.debug(
                "Found %d documents with metadata field '%s'='%s' in collection '%s'",
                len(ids),
                key,
                value,
                self._collection_name,
            )
            return ids
        except Exception as e:
            logger.exception(
                "Failed to get IDs by metadata field '%s'='%s' in collection '%s'",
                key,
                value,
                self._collection_name,
            )
            raise Exception(f"Failed to query documents by metadata field '{key}'") from e

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self.delete_by_ids(ids)
        else:
            logger.debug("No documents found to delete with metadata field '%s'='%s'", key, value)

    def _process_search_results(
        self, results: list[tuple], score_threshold: float = 0.0, score_key: str = "score"
    ) -> list[Document]:
        """
        Common method to process search results

        :param results: Search results as list of tuples (text, metadata, score)
        :param score_threshold: Score threshold for filtering
        :param score_key: Key name for score in metadata
        :return: List of documents
        """
        docs = []
        for row in results:
            text, metadata_str, score = row[0], row[1], row[2]

            # Parse metadata JSON
            try:
                metadata = parse_metadata_json(metadata_str)
            except (ValueError, TypeError):
                logger.warning("Invalid JSON metadata: %s", metadata_str)
                metadata = {}

            # Add score to metadata
            metadata[score_key] = score

            # Filter by score threshold
            if score >= score_threshold:
                docs.append(Document(page_content=text, metadata=metadata))

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        if not self._hybrid_search_enabled:
            logger.warning(
                "Full-text search is disabled: set OCEANBASE_ENABLE_HYBRID_SEARCH=true (requires OceanBase >= 4.3.5.1)."
            )
            return []
        if not self.field_exists("text"):
            logger.warning(
                "Full-text search unavailable: collection '%s' missing 'text' field; "
                "recreate the collection after enabling OCEANBASE_ENABLE_HYBRID_SEARCH to add fulltext index.",
                self._collection_name,
            )
            return []

        try:
            top_k = kwargs.get("top_k", 5)
            if not isinstance(top_k, int) or top_k <= 0:
                raise ValueError("top_k must be a positive integer")

            score_threshold = float(kwargs.get("score_threshold") or 0.0)

            # Build parameterized query to prevent SQL injection
            from sqlalchemy import text

            document_ids_filter = kwargs.get("document_ids_filter")
            params = {"query": query}
            where_clause = ""

            if document_ids_filter:
                # Create parameterized placeholders for document IDs
                placeholders = ", ".join(f":doc_id_{i}" for i in range(len(document_ids_filter)))
                where_clause = f" AND metadata->>'$.document_id' IN ({placeholders})"
                # Add document IDs to parameters
                for i, doc_id in enumerate(document_ids_filter):
                    params[f"doc_id_{i}"] = doc_id

            full_sql = f"""SELECT text, metadata, MATCH (text) AGAINST (:query) AS score
            FROM {self._collection_name}
            WHERE MATCH (text) AGAINST (:query) > 0
            {where_clause}
            ORDER BY score DESC
            LIMIT {top_k}"""

            with self._client.engine.connect() as conn:
                with conn.begin():
                    result = conn.execute(text(full_sql), params)
                    rows = result.fetchall()

                    return self._process_search_results(rows, score_threshold=score_threshold)
        except Exception as e:
            logger.exception(
                "Failed to perform full-text search on collection '%s' with query '%s'",
                self._collection_name,
                query,
            )
            raise Exception(f"Full-text search failed for collection '{self._collection_name}'") from e

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        from sqlalchemy import text

        document_ids_filter = kwargs.get("document_ids_filter")
        _where_clause = None
        if document_ids_filter:
            # Validate document IDs to prevent SQL injection
            # Document IDs should be alphanumeric with hyphens and underscores
            import re

            for doc_id in document_ids_filter:
                if not isinstance(doc_id, str) or not re.match(r"^[a-zA-Z0-9_-]+$", doc_id):
                    raise ValueError(f"Invalid document ID format: {doc_id}")

            # Safe to use in query after validation
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause = f"metadata->>'$.document_id' in ({document_ids})"
            _where_clause = [text(where_clause)]
        ef_search = kwargs.get("ef_search", self._hnsw_ef_search)
        if ef_search != self._hnsw_ef_search:
            self._client.set_ob_hnsw_ef_search(ef_search)
            self._hnsw_ef_search = ef_search
        topk = kwargs.get("top_k", 10)
        try:
            score_threshold = float(val) if (val := kwargs.get("score_threshold")) is not None else 0.0
        except (ValueError, TypeError) as e:
            raise ValueError(f"Invalid score_threshold parameter: {e}") from e
        try:
            cur = self._client.ann_search(
                table_name=self._collection_name,
                vec_column_name="vector",
                vec_data=query_vector,
                topk=topk,
                distance_func=self._get_distance_func(),
                output_column_names=["text", "metadata"],
                with_dist=True,
                where_clause=_where_clause,
            )
        except Exception as e:
            logger.exception(
                "Failed to perform vector search on collection '%s'",
                self._collection_name,
            )
            raise Exception(f"Vector search failed for collection '{self._collection_name}'") from e

        results = []
        for _text, metadata_str, distance in cur:
            score = self._distance_to_score(distance)
            results.append((_text, metadata_str, score))

        return self._process_search_results(results, score_threshold=score_threshold)

    def _get_distance_func(self):
        func = _DISTANCE_FUNC_MAP.get(self._config.metric_type)
        if func is None:
            raise ValueError(
                f"Unsupported metric_type '{self._config.metric_type}'. Supported: {', '.join(_DISTANCE_FUNC_MAP)}"
            )
        return func

    def _distance_to_score(self, distance: float) -> float:
        metric = self._config.metric_type
        if metric == "l2":
            return 1.0 / (1.0 + distance)
        elif metric == "cosine":
            return 1.0 - distance
        elif metric == "inner_product":
            return -distance
        raise ValueError(f"Unsupported metric_type '{metric}'")

    def delete(self):
        try:
            self._client.drop_table_if_exist(self._collection_name)
            logger.debug("Dropped collection '%s'", self._collection_name)
        except Exception as e:
            logger.exception("Failed to delete collection '%s'", self._collection_name)
            raise Exception(f"Failed to delete collection '{self._collection_name}'") from e


class OceanBaseVectorFactory(AbstractVectorFactory):
    def init_vector(
        self,
        dataset: Dataset,
        attributes: list,
        embeddings: Embeddings,
    ) -> BaseVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.OCEANBASE, collection_name))
        return OceanBaseVector(
            collection_name,
            OceanBaseVectorConfig(
                host=dify_config.OCEANBASE_VECTOR_HOST or "",
                port=dify_config.OCEANBASE_VECTOR_PORT or 0,
                user=dify_config.OCEANBASE_VECTOR_USER or "",
                password=(dify_config.OCEANBASE_VECTOR_PASSWORD or ""),
                database=dify_config.OCEANBASE_VECTOR_DATABASE or "",
                enable_hybrid_search=dify_config.OCEANBASE_ENABLE_HYBRID_SEARCH or False,
                batch_size=dify_config.OCEANBASE_VECTOR_BATCH_SIZE,
                metric_type=dify_config.OCEANBASE_VECTOR_METRIC_TYPE,
                hnsw_m=dify_config.OCEANBASE_HNSW_M,
                hnsw_ef_construction=dify_config.OCEANBASE_HNSW_EF_CONSTRUCTION,
                hnsw_ef_search=dify_config.OCEANBASE_HNSW_EF_SEARCH,
                pool_size=dify_config.OCEANBASE_VECTOR_POOL_SIZE,
                max_overflow=dify_config.OCEANBASE_VECTOR_MAX_OVERFLOW,
                hnsw_refresh_threshold=dify_config.OCEANBASE_HNSW_REFRESH_THRESHOLD,
            ),
        )

```

### api/core/rag/datasource/vdb/oceanbase/__init__.py
```py

```

### api/core/rag/datasource/vdb/tidb_vector/__init__.py
```py

```

### api/core/rag/datasource/vdb/tidb_vector/tidb_vector.py
```py
import json
import logging
from typing import Any

import sqlalchemy
from pydantic import BaseModel, model_validator
from sqlalchemy import JSON, TEXT, Column, DateTime, String, Table, create_engine, insert
from sqlalchemy import text as sql_text
from sqlalchemy.orm import Session, declarative_base

from configs import dify_config
from core.rag.datasource.vdb.field import Field, parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class TiDBVectorConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    program_name: str

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config TIDB_VECTOR_HOST is required")
        if not values["port"]:
            raise ValueError("config TIDB_VECTOR_PORT is required")
        if not values["user"]:
            raise ValueError("config TIDB_VECTOR_USER is required")
        if not values["database"]:
            raise ValueError("config TIDB_VECTOR_DATABASE is required")
        if not values["program_name"]:
            raise ValueError("config APPLICATION_NAME is required")
        return values


class TiDBVector(BaseVector):
    def get_type(self) -> str:
        return VectorType.TIDB_VECTOR

    def _table(self, dim: int) -> Table:
        from tidb_vector.sqlalchemy import VectorType  # type: ignore

        return Table(
            self._collection_name,
            self._orm_base.metadata,
            Column(Field.PRIMARY_KEY, String(36), primary_key=True, nullable=False),
            Column(
                Field.VECTOR,
                VectorType(dim),
                nullable=False,
            ),
            Column(Field.TEXT_KEY, TEXT, nullable=False),
            Column("meta", JSON, nullable=False),
            Column("create_time", DateTime, server_default=sqlalchemy.text("CURRENT_TIMESTAMP")),
            Column(
                "update_time", DateTime, server_default=sqlalchemy.text("CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP")
            ),
            extend_existing=True,
        )

    def __init__(self, collection_name: str, config: TiDBVectorConfig, distance_func: str = "cosine"):
        super().__init__(collection_name)
        self._client_config = config
        self._url = (
            f"mysql+pymysql://{config.user}:{config.password}@{config.host}:{config.port}/{config.database}?"
            f"ssl_verify_cert=true&ssl_verify_identity=true&program_name={config.program_name}"
        )
        self._distance_func = distance_func.lower()
        self._engine = create_engine(self._url)
        self._orm_base = declarative_base()
        self._dimension = 1536

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        logger.info("create collection and add texts, collection_name: %s", self._collection_name)
        self._create_collection(len(embeddings[0]))
        self.add_texts(texts, embeddings)
        self._dimension = len(embeddings[0])
        pass

    def _create_collection(self, dimension: int):
        logger.info("_create_collection, collection_name %s", self._collection_name)
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            tidb_dist_func = self._get_distance_func()
            with Session(self._engine) as session:
                session.begin()
                create_statement = sql_text(f"""
                    CREATE TABLE IF NOT EXISTS {self._collection_name} (
                        id CHAR(36) PRIMARY KEY,
                        text TEXT NOT NULL,
                        meta JSON NOT NULL,
                        doc_id VARCHAR(64) AS (JSON_UNQUOTE(JSON_EXTRACT(meta, '$.doc_id'))) STORED,
                        document_id VARCHAR(64) AS (JSON_UNQUOTE(JSON_EXTRACT(meta, '$.document_id'))) STORED,
                        vector VECTOR<FLOAT>({dimension}) NOT NULL,
                        create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
                        update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                        KEY (doc_id),
                        KEY (document_id),
                        VECTOR INDEX idx_vector (({tidb_dist_func}(vector))) USING HNSW
                    );
                """)
                session.execute(create_statement)
                session.commit()
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        table = self._table(len(embeddings[0]))
        ids = self._get_uuids(documents)
        metas = [d.metadata for d in documents]
        texts = [d.page_content for d in documents]

        chunks_table_data = []
        with self._engine.connect() as conn, conn.begin():
            for id, text, meta, embedding in zip(ids, texts, metas, embeddings):
                chunks_table_data.append({"id": id, "vector": embedding, "text": text, "meta": meta})

                # Execute the batch insert when the batch size is reached
                if len(chunks_table_data) == 500:
                    conn.execute(insert(table).values(chunks_table_data))
                    # Clear the chunks_table_data list for the next batch
                    chunks_table_data.clear()

            # Insert any remaining records that didn't make up a full batch
            if chunks_table_data:
                conn.execute(insert(table).values(chunks_table_data))
        return ids

    def text_exists(self, id: str) -> bool:
        result = self.get_ids_by_metadata_field("doc_id", id)
        return bool(result)

    def delete_by_ids(self, ids: list[str]):
        with Session(self._engine) as session:
            ids_str = ",".join(f"'{doc_id}'" for doc_id in ids)
            select_statement = sql_text(
                f"""SELECT id FROM {self._collection_name} WHERE meta->>'$.doc_id' in ({ids_str}); """
            )
            result = session.execute(select_statement).fetchall()
        if result:
            ids = [item[0] for item in result]
            self._delete_by_ids(ids)

    def _delete_by_ids(self, ids: list[str]) -> bool:
        if ids is None:
            raise ValueError("No ids provided to delete.")
        table = self._table(self._dimension)
        try:
            with self._engine.connect() as conn, conn.begin():
                delete_condition = table.c.id.in_(ids)
                conn.execute(table.delete().where(delete_condition))
                return True
        except Exception:
            logger.exception("Delete operation failed for collection %s", self._collection_name)
            return False

    def get_ids_by_metadata_field(self, key: str, value: str):
        with Session(self._engine) as session:
            select_statement = sql_text(
                f"""SELECT id FROM {self._collection_name} WHERE meta->>'$.{key}' = '{value}'; """
            )
            result = session.execute(select_statement).fetchall()
        if result:
            return [item[0] for item in result]
        else:
            return None

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self._delete_by_ids(ids)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        distance = 1 - score_threshold

        query_vector_str = ", ".join(format(x) for x in query_vector)
        query_vector_str = "[" + query_vector_str + "]"
        logger.debug(
            "_collection_name: %s, score_threshold: %s, distance: %s", self._collection_name, score_threshold, distance
        )

        docs = []
        tidb_dist_func = self._get_distance_func()
        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause = f" WHERE meta->>'$.document_id' in ({document_ids}) "

        with Session(self._engine) as session:
            select_statement = sql_text(f"""
                SELECT meta, text, distance
                FROM (
                  SELECT
                    meta,
                    text,
                    {tidb_dist_func}(vector, :query_vector_str) AS distance
                  FROM {self._collection_name}
                  {where_clause}
                  ORDER BY distance ASC
                  LIMIT :top_k
                ) t
                WHERE distance <= :distance
                """)
            res = session.execute(
                select_statement,
                params={
                    "query_vector_str": query_vector_str,
                    "distance": distance,
                    "top_k": top_k,
                },
            )
            results = [(row[0], row[1], row[2]) for row in res]
            for meta, text, distance in results:
                metadata = parse_metadata_json(meta)
                metadata["score"] = 1 - distance
                docs.append(Document(page_content=text, metadata=metadata))
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        # tidb doesn't support bm25 search
        return []

    def delete(self):
        with Session(self._engine) as session:
            session.execute(sql_text(f"""DROP TABLE IF EXISTS {self._collection_name};"""))
            session.commit()

    def _get_distance_func(self) -> str:
        match self._distance_func:
            case "l2":
                tidb_dist_func = "VEC_L2_DISTANCE"
            case "cosine":
                tidb_dist_func = "VEC_COSINE_DISTANCE"
            case _:
                tidb_dist_func = "VEC_COSINE_DISTANCE"
        return tidb_dist_func


class TiDBVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> TiDBVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.TIDB_VECTOR, collection_name))

        return TiDBVector(
            collection_name=collection_name,
            config=TiDBVectorConfig(
                host=dify_config.TIDB_VECTOR_HOST or "",
                port=dify_config.TIDB_VECTOR_PORT or 0,
                user=dify_config.TIDB_VECTOR_USER or "",
                password=dify_config.TIDB_VECTOR_PASSWORD or "",
                database=dify_config.TIDB_VECTOR_DATABASE or "",
                program_name=dify_config.APPLICATION_NAME,
            ),
        )

```

### api/core/rag/datasource/vdb/tidb_on_qdrant/__init__.py
```py

```

### api/core/rag/datasource/vdb/tidb_on_qdrant/tidb_on_qdrant_vector.py
```py
import json
import os
import uuid
from collections.abc import Generator, Iterable, Sequence
from itertools import islice
from typing import TYPE_CHECKING, Any

import httpx
import qdrant_client
from flask import current_app
from httpx import DigestAuth
from pydantic import BaseModel
from qdrant_client.http import models as rest
from qdrant_client.http.models import (
    FilterSelector,
    HnswConfigDiff,
    PayloadSchemaType,
    TextIndexParams,
    TextIndexType,
    TokenizerType,
)
from qdrant_client.local.qdrant_local import QdrantLocal
from sqlalchemy import select

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.tidb_on_qdrant.tidb_service import TidbService
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from models.dataset import Dataset, TidbAuthBinding
from models.enums import TidbAuthBindingStatus

if TYPE_CHECKING:
    from qdrant_client import grpc  # noqa
    from qdrant_client.conversions import common_types
    from qdrant_client.http import models as rest

    type DictFilter = dict[str, str | int | bool | dict | list]
    type MetadataFilter = DictFilter | common_types.Filter


class TidbOnQdrantConfig(BaseModel):
    endpoint: str
    api_key: str | None = None
    timeout: float = 20
    root_path: str | None = None
    grpc_port: int = 6334
    prefer_grpc: bool = False
    replication_factor: int = 1

    def to_qdrant_params(self):
        if self.endpoint and self.endpoint.startswith("path:"):
            path = self.endpoint.replace("path:", "")
            if not os.path.isabs(path):
                if self.root_path:
                    path = os.path.join(self.root_path, path)
                else:
                    raise ValueError("root_path is required")

            return {"path": path}
        else:
            return {
                "url": self.endpoint,
                "api_key": self.api_key,
                "timeout": self.timeout,
                "verify": False,
                "grpc_port": self.grpc_port,
                "prefer_grpc": self.prefer_grpc,
            }


class TidbConfig(BaseModel):
    api_url: str
    public_key: str
    private_key: str


class TidbOnQdrantVector(BaseVector):
    def __init__(self, collection_name: str, group_id: str, config: TidbOnQdrantConfig, distance_func: str = "Cosine"):
        super().__init__(collection_name)
        self._client_config = config
        self._client = qdrant_client.QdrantClient(**self._client_config.to_qdrant_params())
        self._distance_func = distance_func.upper()
        self._group_id = group_id

    def get_type(self) -> str:
        return VectorType.TIDB_ON_QDRANT

    def to_index_struct(self):
        return {"type": self.get_type(), "vector_store": {"class_prefix": self._collection_name}}

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        if texts:
            # get embedding vector size
            vector_size = len(embeddings[0])
            # get collection name
            collection_name = self._collection_name
            # create collection
            self.create_collection(collection_name, vector_size)

            self.add_texts(texts, embeddings, **kwargs)

    def create_collection(self, collection_name: str, vector_size: int):
        lock_name = f"vector_indexing_lock_{collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            collection_name = collection_name or uuid.uuid4().hex
            all_collection_name = []
            collections_response = self._client.get_collections()
            collection_list = collections_response.collections
            for collection in collection_list:
                all_collection_name.append(collection.name)
            if collection_name not in all_collection_name:
                from qdrant_client.http import models as rest

                vectors_config = rest.VectorParams(
                    size=vector_size,
                    distance=rest.Distance[self._distance_func],
                )
                hnsw_config = HnswConfigDiff(
                    m=0,
                    payload_m=16,
                    ef_construct=100,
                    full_scan_threshold=10000,
                    max_indexing_threads=0,
                    on_disk=False,
                )
                self._client.create_collection(
                    collection_name=collection_name,
                    vectors_config=vectors_config,
                    hnsw_config=hnsw_config,
                    timeout=int(self._client_config.timeout),
                    replication_factor=self._client_config.replication_factor,
                )

                # create group_id payload index
                self._client.create_payload_index(
                    collection_name, Field.GROUP_KEY, field_schema=PayloadSchemaType.KEYWORD
                )
                # create doc_id payload index
                self._client.create_payload_index(collection_name, Field.DOC_ID, field_schema=PayloadSchemaType.KEYWORD)
                # create document_id payload index
                self._client.create_payload_index(
                    collection_name, Field.DOCUMENT_ID, field_schema=PayloadSchemaType.KEYWORD
                )
                # create full text index
                text_index_params = TextIndexParams(
                    type=TextIndexType.TEXT,
                    tokenizer=TokenizerType.MULTILINGUAL,
                    min_token_len=2,
                    max_token_len=20,
                    lowercase=True,
                )
                self._client.create_payload_index(collection_name, Field.CONTENT_KEY, field_schema=text_index_params)
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)
        texts = [d.page_content for d in documents]
        metadatas = [d.metadata for d in documents if d.metadata is not None]

        added_ids = []
        for batch_ids, points in self._generate_rest_batches(texts, embeddings, metadatas, uuids, 64, self._group_id):
            self._client.upsert(collection_name=self._collection_name, points=points)
            added_ids.extend(batch_ids)

        return added_ids

    def _generate_rest_batches(
        self,
        texts: Iterable[str],
        embeddings: list[list[float]],
        metadatas: list[dict] | None = None,
        ids: Sequence[str] | None = None,
        batch_size: int = 64,
        group_id: str | None = None,
    ) -> Generator[tuple[list[str], list[rest.PointStruct]], None, None]:
        from qdrant_client.http import models as rest

        texts_iterator = iter(texts)
        embeddings_iterator = iter(embeddings)
        metadatas_iterator = iter(metadatas or [])
        ids_iterator = iter(ids or [uuid.uuid4().hex for _ in iter(texts)])
        while batch_texts := list(islice(texts_iterator, batch_size)):
            # Take the corresponding metadata and id for each text in a batch
            batch_metadatas = list(islice(metadatas_iterator, batch_size)) or None
            batch_ids = list(islice(ids_iterator, batch_size))

            # Generate the embeddings for all the texts in a batch
            batch_embeddings = list(islice(embeddings_iterator, batch_size))

            points = [
                rest.PointStruct(
                    id=point_id,
                    vector=vector,
                    payload=payload,
                )
                for point_id, vector, payload in zip(
                    batch_ids,
                    batch_embeddings,
                    self._build_payloads(
                        batch_texts,
                        batch_metadatas,
                        Field.CONTENT_KEY,
                        Field.METADATA_KEY,
                        group_id or "",
                        Field.GROUP_KEY,
                    ),
                )
            ]

            yield batch_ids, points

    @classmethod
    def _build_payloads(
        cls,
        texts: Iterable[str],
        metadatas: list[dict] | None,
        content_payload_key: str,
        metadata_payload_key: str,
        group_id: str,
        group_payload_key: str,
    ) -> list[dict]:
        payloads = []
        for i, text in enumerate(texts):
            if text is None:
                raise ValueError(
                    "At least one of the texts is None. Please remove it before "
                    "calling .from_texts or .add_texts on Qdrant instance."
                )
            metadata = metadatas[i] if metadatas is not None else None
            payloads.append({content_payload_key: text, metadata_payload_key: metadata, group_payload_key: group_id})

        return payloads

    def delete_by_metadata_field(self, key: str, value: str):
        from qdrant_client.http import models
        from qdrant_client.http.exceptions import UnexpectedResponse

        try:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key=f"metadata.{key}",
                        match=models.MatchValue(value=value),
                    ),
                ],
            )

            self._reload_if_needed()

            self._client.delete(
                collection_name=self._collection_name,
                points_selector=FilterSelector(filter=filter),
            )
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def delete(self):
        from qdrant_client.http.exceptions import UnexpectedResponse

        try:
            self._client.delete_collection(collection_name=self._collection_name)
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def delete_by_ids(self, ids: list[str]):
        from qdrant_client.http import models
        from qdrant_client.http.exceptions import UnexpectedResponse

        if not ids:
            return

        try:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key="metadata.doc_id",
                        match=models.MatchAny(any=ids),
                    ),
                ],
            )
            self._client.delete(
                collection_name=self._collection_name,
                points_selector=FilterSelector(filter=filter),
            )
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def text_exists(self, id: str) -> bool:
        all_collection_name = []
        collections_response = self._client.get_collections()
        collection_list = collections_response.collections
        for collection in collection_list:
            all_collection_name.append(collection.name)
        if self._collection_name not in all_collection_name:
            return False
        response = self._client.retrieve(collection_name=self._collection_name, ids=[id])

        return len(response) > 0

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        from qdrant_client.http import models

        filter = None
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key="metadata.document_id",
                        match=models.MatchAny(any=document_ids_filter),
                    )
                ],
            )
        results = self._client.search(
            collection_name=self._collection_name,
            query_vector=query_vector,
            query_filter=filter,
            limit=kwargs.get("top_k", 4),
            with_payload=True,
            with_vectors=True,
            score_threshold=kwargs.get("score_threshold", 0.0),
        )
        docs = []
        for result in results:
            if result.payload is None:
                continue
            metadata = result.payload.get(Field.METADATA_KEY) or {}
            # duplicate check score threshold
            score_threshold = kwargs.get("score_threshold") or 0.0
            if result.score >= score_threshold:
                metadata["score"] = result.score
                doc = Document(
                    page_content=result.payload.get(Field.CONTENT_KEY, ""),
                    metadata=metadata,
                )
                docs.append(doc)
        # Sort the documents by score in descending order
        docs = sorted(docs, key=lambda x: x.metadata["score"] if x.metadata is not None else 0, reverse=True)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """Return docs most similar by bm25.
        Returns:
            List of documents most similar to the query text and distance for each.
        """
        from qdrant_client.http import models

        scroll_filter = None
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            scroll_filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key="metadata.document_id",
                        match=models.MatchAny(any=document_ids_filter),
                    )
                ]
            )
        response = self._client.scroll(
            collection_name=self._collection_name,
            scroll_filter=scroll_filter,
            limit=kwargs.get("top_k", 2),
            with_payload=True,
            with_vectors=True,
        )
        results = response[0]
        documents = []
        for result in results:
            if result:
                document = self._document_from_scored_point(result, Field.CONTENT_KEY, Field.METADATA_KEY)
                documents.append(document)

        return documents

    def _reload_if_needed(self):
        if isinstance(self._client, QdrantLocal):
            self._client._load()

    @classmethod
    def _document_from_scored_point(
        cls,
        scored_point: Any,
        content_payload_key: str,
        metadata_payload_key: str,
    ) -> Document:
        return Document(
            page_content=scored_point.payload.get(content_payload_key),
            vector=scored_point.vector,
            metadata=scored_point.payload.get(metadata_payload_key) or {},
        )


class TidbOnQdrantVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> TidbOnQdrantVector:
        stmt = select(TidbAuthBinding).where(TidbAuthBinding.tenant_id == dataset.tenant_id)
        tidb_auth_binding = db.session.scalars(stmt).one_or_none()
        if not tidb_auth_binding:
            with redis_client.lock("create_tidb_serverless_cluster_lock", timeout=900):
                stmt = select(TidbAuthBinding).where(TidbAuthBinding.tenant_id == dataset.tenant_id)
                tidb_auth_binding = db.session.scalars(stmt).one_or_none()
                if tidb_auth_binding:
                    TIDB_ON_QDRANT_API_KEY = f"{tidb_auth_binding.account}:{tidb_auth_binding.password}"

                else:
                    idle_tidb_auth_binding = db.session.scalar(
                        select(TidbAuthBinding)
                        .where(TidbAuthBinding.active == False, TidbAuthBinding.status == "ACTIVE")
                        .limit(1)
                    )
                    if idle_tidb_auth_binding:
                        idle_tidb_auth_binding.active = True
                        idle_tidb_auth_binding.tenant_id = dataset.tenant_id
                        db.session.commit()
                        TIDB_ON_QDRANT_API_KEY = f"{idle_tidb_auth_binding.account}:{idle_tidb_auth_binding.password}"
                    else:
                        new_cluster = TidbService.create_tidb_serverless_cluster(
                            dify_config.TIDB_PROJECT_ID or "",
                            dify_config.TIDB_API_URL or "",
                            dify_config.TIDB_IAM_API_URL or "",
                            dify_config.TIDB_PUBLIC_KEY or "",
                            dify_config.TIDB_PRIVATE_KEY or "",
                            dify_config.TIDB_REGION or "",
                        )
                        new_tidb_auth_binding = TidbAuthBinding(
                            cluster_id=new_cluster["cluster_id"],
                            cluster_name=new_cluster["cluster_name"],
                            account=new_cluster["account"],
                            password=new_cluster["password"],
                            tenant_id=dataset.tenant_id,
                            active=True,
                            status=TidbAuthBindingStatus.ACTIVE,
                        )
                        db.session.add(new_tidb_auth_binding)
                        db.session.commit()
                        TIDB_ON_QDRANT_API_KEY = f"{new_tidb_auth_binding.account}:{new_tidb_auth_binding.password}"
        else:
            TIDB_ON_QDRANT_API_KEY = f"{tidb_auth_binding.account}:{tidb_auth_binding.password}"

        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.TIDB_ON_QDRANT, collection_name))

        config = current_app.config

        return TidbOnQdrantVector(
            collection_name=collection_name,
            group_id=dataset.id,
            config=TidbOnQdrantConfig(
                endpoint=dify_config.TIDB_ON_QDRANT_URL or "",
                api_key=TIDB_ON_QDRANT_API_KEY,
                root_path=str(config.root_path),
                timeout=dify_config.TIDB_ON_QDRANT_CLIENT_TIMEOUT,
                grpc_port=dify_config.TIDB_ON_QDRANT_GRPC_PORT,
                prefer_grpc=dify_config.TIDB_ON_QDRANT_GRPC_ENABLED,
                replication_factor=dify_config.QDRANT_REPLICATION_FACTOR,
            ),
        )

    def create_tidb_serverless_cluster(self, tidb_config: TidbConfig, display_name: str, region: str):
        """
        Creates a new TiDB Serverless cluster.
        :param tidb_config: The configuration for the TiDB Cloud API.
        :param display_name: The user-friendly display name of the cluster (required).
        :param region: The region where the cluster will be created (required).

        :return: The response from the API.
        """
        region_object = {
            "name": region,
        }

        labels = {
            "tidb.cloud/project": "1372813089454548012",
        }
        cluster_data = {"displayName": display_name, "region": region_object, "labels": labels}

        response = httpx.post(
            f"{tidb_config.api_url}/clusters",
            json=cluster_data,
            auth=DigestAuth(tidb_config.public_key, tidb_config.private_key),
        )

        if response.status_code == 200:
            return response.json()
        else:
            response.raise_for_status()

    def change_tidb_serverless_root_password(self, tidb_config: TidbConfig, cluster_id: str, new_password: str):
        """
        Changes the root password of a specific TiDB Serverless cluster.

        :param tidb_config: The configuration for the TiDB Cloud API.
        :param cluster_id: The ID of the cluster for which the password is to be changed (required).
        :param new_password: The new password for the root user (required).
        :return: The response from the API.
        """

        body = {"password": new_password}

        response = httpx.put(
            f"{tidb_config.api_url}/clusters/{cluster_id}/password",
            json=body,
            auth=DigestAuth(tidb_config.public_key, tidb_config.private_key),
        )

        if response.status_code == 200:
            return response.json()
        else:
            response.raise_for_status()

```

### api/core/rag/datasource/vdb/tidb_on_qdrant/tidb_service.py
```py
import time
import uuid
from collections.abc import Sequence

import httpx
from httpx import DigestAuth

from configs import dify_config
from core.helper.http_client_pooling import get_pooled_http_client
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from models.dataset import TidbAuthBinding
from models.enums import TidbAuthBindingStatus

# Reuse a pooled HTTP client for all TiDB Cloud requests to minimize connection churn
_tidb_http_client: httpx.Client = get_pooled_http_client(
    "tidb:cloud",
    lambda: httpx.Client(limits=httpx.Limits(max_keepalive_connections=50, max_connections=100)),
)


class TidbService:
    @staticmethod
    def create_tidb_serverless_cluster(
        project_id: str, api_url: str, iam_url: str, public_key: str, private_key: str, region: str
    ):
        """
        Creates a new TiDB Serverless cluster.
        :param project_id: The project ID of the TiDB Cloud project (required).
        :param api_url: The URL of the TiDB Cloud API (required).
        :param iam_url: The URL of the TiDB Cloud IAM API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).
        :param region: The region where the cluster will be created (required).

        :return: The response from the API.
        """

        region_object = {
            "name": region,
        }

        labels = {
            "tidb.cloud/project": project_id,
        }

        spending_limit = {
            "monthly": dify_config.TIDB_SPEND_LIMIT,
        }
        password = str(uuid.uuid4()).replace("-", "")[:16]
        display_name = str(uuid.uuid4()).replace("-", "")[:16]
        cluster_data = {
            "displayName": display_name,
            "region": region_object,
            "labels": labels,
            "spendingLimit": spending_limit,
            "rootPassword": password,
        }

        response = _tidb_http_client.post(
            f"{api_url}/clusters", json=cluster_data, auth=DigestAuth(public_key, private_key)
        )

        if response.status_code == 200:
            response_data = response.json()
            cluster_id = response_data["clusterId"]
            retry_count = 0
            max_retries = 30
            while retry_count < max_retries:
                cluster_response = TidbService.get_tidb_serverless_cluster(api_url, public_key, private_key, cluster_id)
                if cluster_response["state"] == "ACTIVE":
                    user_prefix = cluster_response["userPrefix"]
                    return {
                        "cluster_id": cluster_id,
                        "cluster_name": display_name,
                        "account": f"{user_prefix}.root",
                        "password": password,
                    }
                time.sleep(30)  # wait 30 seconds before retrying
                retry_count += 1
        else:
            response.raise_for_status()

    @staticmethod
    def delete_tidb_serverless_cluster(api_url: str, public_key: str, private_key: str, cluster_id: str):
        """
        Deletes a specific TiDB Serverless cluster.

        :param api_url: The URL of the TiDB Cloud API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).
        :param cluster_id: The ID of the cluster to be deleted (required).
        :return: The response from the API.
        """

        response = _tidb_http_client.delete(
            f"{api_url}/clusters/{cluster_id}", auth=DigestAuth(public_key, private_key)
        )

        if response.status_code == 200:
            return response.json()
        else:
            response.raise_for_status()

    @staticmethod
    def get_tidb_serverless_cluster(api_url: str, public_key: str, private_key: str, cluster_id: str):
        """
        Deletes a specific TiDB Serverless cluster.

        :param api_url: The URL of the TiDB Cloud API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).
        :param cluster_id: The ID of the cluster to be deleted (required).
        :return: The response from the API.
        """

        response = _tidb_http_client.get(f"{api_url}/clusters/{cluster_id}", auth=DigestAuth(public_key, private_key))

        if response.status_code == 200:
            return response.json()
        else:
            response.raise_for_status()

    @staticmethod
    def change_tidb_serverless_root_password(
        api_url: str, public_key: str, private_key: str, cluster_id: str, account: str, new_password: str
    ):
        """
        Changes the root password of a specific TiDB Serverless cluster.

        :param api_url: The URL of the TiDB Cloud API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).
        :param cluster_id: The ID of the cluster for which the password is to be changed (required).+
        :param account: The account for which the password is to be changed (required).
        :param new_password: The new password for the root user (required).
        :return: The response from the API.
        """

        body = {"password": new_password, "builtinRole": "role_admin", "customRoles": []}

        response = _tidb_http_client.patch(
            f"{api_url}/clusters/{cluster_id}/sqlUsers/{account}",
            json=body,
            auth=DigestAuth(public_key, private_key),
        )

        if response.status_code == 200:
            return response.json()
        else:
            response.raise_for_status()

    @staticmethod
    def batch_update_tidb_serverless_cluster_status(
        tidb_serverless_list: Sequence[TidbAuthBinding],
        project_id: str,
        api_url: str,
        iam_url: str,
        public_key: str,
        private_key: str,
    ):
        """
        Update the status of a new TiDB Serverless cluster.
        :param tidb_serverless_list: The TiDB serverless list (required).
        :param project_id: The project ID of the TiDB Cloud project (required).
        :param api_url: The URL of the TiDB Cloud API (required).
        :param iam_url: The URL of the TiDB Cloud IAM API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).

        :return: The response from the API.
        """
        tidb_serverless_list_map = {item.cluster_id: item for item in tidb_serverless_list}
        cluster_ids = [item.cluster_id for item in tidb_serverless_list]
        params = {"clusterIds": cluster_ids, "view": "BASIC"}
        response = _tidb_http_client.get(
            f"{api_url}/clusters:batchGet", params=params, auth=DigestAuth(public_key, private_key)
        )

        if response.status_code == 200:
            response_data = response.json()
            for item in response_data["clusters"]:
                state = item["state"]
                userPrefix = item["userPrefix"]
                if state == "ACTIVE" and len(userPrefix) > 0:
                    cluster_info = tidb_serverless_list_map[item["clusterId"]]
                    cluster_info.status = TidbAuthBindingStatus.ACTIVE
                    cluster_info.account = f"{userPrefix}.root"
                    db.session.add(cluster_info)
            db.session.commit()
        else:
            response.raise_for_status()

    @staticmethod
    def batch_create_tidb_serverless_cluster(
        batch_size: int, project_id: str, api_url: str, iam_url: str, public_key: str, private_key: str, region: str
    ) -> list[dict]:
        """
        Creates a new TiDB Serverless cluster.
        :param batch_size: The batch size (required).
        :param project_id: The project ID of the TiDB Cloud project (required).
        :param api_url: The URL of the TiDB Cloud API (required).
        :param iam_url: The URL of the TiDB Cloud IAM API (required).
        :param public_key: The public key for the API (required).
        :param private_key: The private key for the API (required).
        :param region: The region where the cluster will be created (required).

        :return: The response from the API.
        """
        clusters = []
        for _ in range(batch_size):
            region_object = {
                "name": region,
            }

            labels = {
                "tidb.cloud/project": project_id,
            }

            spending_limit = {
                "monthly": dify_config.TIDB_SPEND_LIMIT,
            }
            password = str(uuid.uuid4()).replace("-", "")[:16]
            display_name = str(uuid.uuid4()).replace("-", "")
            cluster_data = {
                "cluster": {
                    "displayName": display_name,
                    "region": region_object,
                    "labels": labels,
                    "spendingLimit": spending_limit,
                    "rootPassword": password,
                }
            }
            cache_key = f"tidb_serverless_cluster_password:{display_name}"
            redis_client.setex(cache_key, 3600, password)
            clusters.append(cluster_data)

        request_body = {"requests": clusters}
        response = _tidb_http_client.post(
            f"{api_url}/clusters:batchCreate", json=request_body, auth=DigestAuth(public_key, private_key)
        )

        if response.status_code == 200:
            response_data = response.json()
            cluster_infos = []
            for item in response_data["clusters"]:
                cache_key = f"tidb_serverless_cluster_password:{item['displayName']}"
                cached_password = redis_client.get(cache_key)
                if not cached_password:
                    continue
                cluster_info = {
                    "cluster_id": item["clusterId"],
                    "cluster_name": item["displayName"],
                    "account": "root",
                    "password": cached_password.decode("utf-8"),
                }
                cluster_infos.append(cluster_info)
            return cluster_infos
        else:
            response.raise_for_status()
            return []

```

### api/core/rag/datasource/vdb/tablestore/tablestore_vector.py
```py
import json
import logging
import math
from collections.abc import Iterable
from typing import Any

import tablestore  # type: ignore
from pydantic import BaseModel, model_validator
from tablestore import BatchGetRowRequest, TableInBatchGetRowItem

from configs import dify_config
from core.rag.datasource.vdb.field import Field, parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models import Dataset

logger = logging.getLogger(__name__)


class TableStoreConfig(BaseModel):
    access_key_id: str | None = None
    access_key_secret: str | None = None
    instance_name: str | None = None
    endpoint: str | None = None
    normalize_full_text_bm25_score: bool | None = False

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["access_key_id"]:
            raise ValueError("config ACCESS_KEY_ID is required")
        if not values["access_key_secret"]:
            raise ValueError("config ACCESS_KEY_SECRET is required")
        if not values["instance_name"]:
            raise ValueError("config INSTANCE_NAME is required")
        if not values["endpoint"]:
            raise ValueError("config ENDPOINT is required")
        return values


class TableStoreVector(BaseVector):
    def __init__(self, collection_name: str, config: TableStoreConfig):
        super().__init__(collection_name)
        self._config = config
        self._tablestore_client = tablestore.OTSClient(
            config.endpoint,
            config.access_key_id,
            config.access_key_secret,
            config.instance_name,
        )
        self._normalize_full_text_bm25_score = config.normalize_full_text_bm25_score
        self._table_name = f"{collection_name}"
        self._index_name = f"{collection_name}_idx"
        self._tags_field = f"{Field.METADATA_KEY}_tags"

    def create_collection(self, embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        docs = []
        request = BatchGetRowRequest()
        columns_to_get = [Field.METADATA_KEY, Field.CONTENT_KEY]
        rows_to_get = [[("id", _id)] for _id in ids]
        request.add(TableInBatchGetRowItem(self._table_name, rows_to_get, columns_to_get, None, 1))

        result = self._tablestore_client.batch_get_row(request)
        table_result = result.get_result_by_table(self._table_name)
        for item in table_result:
            if item.is_ok and item.row:
                kv = {k: v for k, v, _ in item.row.attribute_columns}
                metadata = parse_metadata_json(kv[Field.METADATA_KEY])
                docs.append(Document(page_content=kv[Field.CONTENT_KEY], metadata=metadata))
        return docs

    def get_type(self) -> str:
        return VectorType.TABLESTORE

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        self.add_texts(documents=texts, embeddings=embeddings, **kwargs)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)

        for i in range(len(documents)):
            self._write_row(
                primary_key=uuids[i],
                attributes={
                    Field.CONTENT_KEY: documents[i].page_content,
                    Field.VECTOR: embeddings[i],
                    Field.METADATA_KEY: documents[i].metadata,
                },
            )
        return uuids

    def text_exists(self, id: str) -> bool:
        result = self._tablestore_client.get_row(
            table_name=self._table_name, primary_key=[("id", id)], columns_to_get=["id"]
        )
        assert isinstance(result, tuple | list)
        # Unpack the tuple result
        _, return_row, _ = result

        return return_row is not None

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        for id in ids:
            self._delete_row(id=id)

    def get_ids_by_metadata_field(self, key: str, value: str):
        return self._search_by_metadata(key, value)

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        self.delete_by_ids(ids)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        document_ids_filter = kwargs.get("document_ids_filter")
        filtered_list = None
        if document_ids_filter:
            filtered_list = ["document_id=" + item for item in document_ids_filter]
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._search_by_vector(query_vector, filtered_list, top_k, score_threshold)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        document_ids_filter = kwargs.get("document_ids_filter")
        filtered_list = None
        if document_ids_filter:
            filtered_list = ["document_id=" + item for item in document_ids_filter]
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._search_by_full_text(query, filtered_list, top_k, score_threshold)

    def delete(self):
        self._delete_table_if_exist()

    def _create_collection(self, dimension: int):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name)
                return

            self._create_table_if_not_exist()
            self._create_search_index_if_not_exist(dimension)
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def _create_table_if_not_exist(self):
        table_list = self._tablestore_client.list_table()
        if self._table_name in table_list:
            logger.info("Tablestore system table[%s] already exists", self._table_name)
            return None

        schema_of_primary_key = [("id", "STRING")]
        table_meta = tablestore.TableMeta(self._table_name, schema_of_primary_key)
        table_options = tablestore.TableOptions()
        reserved_throughput = tablestore.ReservedThroughput(tablestore.CapacityUnit(0, 0))
        self._tablestore_client.create_table(table_meta, table_options, reserved_throughput)
        logger.info("Tablestore create table[%s] successfully.", self._table_name)

    def _create_search_index_if_not_exist(self, dimension: int):
        search_index_list = self._tablestore_client.list_search_index(table_name=self._table_name)
        assert isinstance(search_index_list, Iterable)
        if self._index_name in [t[1] for t in search_index_list]:
            logger.info("Tablestore system index[%s] already exists", self._index_name)
            return None

        field_schemas = [
            tablestore.FieldSchema(
                Field.CONTENT_KEY,
                tablestore.FieldType.TEXT,
                analyzer=tablestore.AnalyzerType.MAXWORD,
                index=True,
                enable_sort_and_agg=False,
                store=False,
            ),
            tablestore.FieldSchema(
                Field.VECTOR,
                tablestore.FieldType.VECTOR,
                vector_options=tablestore.VectorOptions(
                    data_type=tablestore.VectorDataType.VD_FLOAT_32,
                    dimension=dimension,
                    metric_type=tablestore.VectorMetricType.VM_COSINE,
                ),
            ),
            tablestore.FieldSchema(
                Field.METADATA_KEY,
                tablestore.FieldType.KEYWORD,
                index=True,
                store=False,
            ),
            tablestore.FieldSchema(
                self._tags_field,
                tablestore.FieldType.KEYWORD,
                index=True,
                store=False,
                is_array=True,
            ),
        ]

        index_meta = tablestore.SearchIndexMeta(field_schemas)
        self._tablestore_client.create_search_index(self._table_name, self._index_name, index_meta)
        logger.info("Tablestore create system index[%s] successfully.", self._index_name)

    def _delete_table_if_exist(self):
        search_index_list = self._tablestore_client.list_search_index(table_name=self._table_name)
        assert isinstance(search_index_list, Iterable)
        for resp_tuple in search_index_list:
            self._tablestore_client.delete_search_index(resp_tuple[0], resp_tuple[1])
            logger.info("Tablestore delete index[%s] successfully.", self._index_name)

        self._tablestore_client.delete_table(self._table_name)
        logger.info("Tablestore delete system table[%s] successfully.", self._index_name)

    def _delete_search_index(self):
        self._tablestore_client.delete_search_index(self._table_name, self._index_name)
        logger.info("Tablestore delete index[%s] successfully.", self._index_name)

    def _write_row(self, primary_key: str, attributes: dict[str, Any]):
        pk = [("id", primary_key)]

        tags = []
        for key, value in attributes[Field.METADATA_KEY].items():
            tags.append(str(key) + "=" + str(value))

        attribute_columns = [
            (Field.CONTENT_KEY, attributes[Field.CONTENT_KEY]),
            (Field.VECTOR, json.dumps(attributes[Field.VECTOR])),
            (
                Field.METADATA_KEY,
                json.dumps(attributes[Field.METADATA_KEY]),
            ),
            (self._tags_field, json.dumps(tags)),
        ]
        row = tablestore.Row(pk, attribute_columns)
        self._tablestore_client.put_row(self._table_name, row)

    def _delete_row(self, id: str):
        primary_key = [("id", id)]
        row = tablestore.Row(primary_key)
        self._tablestore_client.delete_row(self._table_name, row, None)

    def _search_by_metadata(self, key: str, value: str) -> list[str]:
        query = tablestore.SearchQuery(
            tablestore.TermQuery(self._tags_field, str(key) + "=" + str(value)),
            limit=1000,
            get_total_count=False,
        )
        rows: list[str] = []
        next_token = None
        while True:
            if next_token is not None:
                query.next_token = next_token

            search_response = self._tablestore_client.search(
                table_name=self._table_name,
                index_name=self._index_name,
                search_query=query,
                columns_to_get=tablestore.ColumnsToGet(
                    column_names=[Field.PRIMARY_KEY], return_type=tablestore.ColumnReturnType.SPECIFIED
                ),
            )

            if search_response is not None:
                rows.extend([row[0][0][1] for row in list(search_response.rows)])

            if search_response is None or search_response.next_token == b"":
                break
            else:
                next_token = search_response.next_token

        return rows

    def _search_by_vector(
        self, query_vector: list[float], document_ids_filter: list[str] | None, top_k: int, score_threshold: float
    ) -> list[Document]:
        knn_vector_query = tablestore.KnnVectorQuery(
            field_name=Field.VECTOR,
            top_k=top_k,
            float32_query_vector=query_vector,
        )
        if document_ids_filter:
            knn_vector_query.filter = tablestore.TermsQuery(self._tags_field, document_ids_filter)

        sort = tablestore.Sort(sorters=[tablestore.ScoreSort(sort_order=tablestore.SortOrder.DESC)])
        search_query = tablestore.SearchQuery(knn_vector_query, limit=top_k, get_total_count=False, sort=sort)

        search_response = self._tablestore_client.search(
            table_name=self._table_name,
            index_name=self._index_name,
            search_query=search_query,
            columns_to_get=tablestore.ColumnsToGet(return_type=tablestore.ColumnReturnType.ALL_FROM_INDEX),
        )
        documents = []
        for search_hit in search_response.search_hits:
            if search_hit.score >= score_threshold:
                ots_column_map = {}
                for col in search_hit.row[1]:
                    ots_column_map[col[0]] = col[1]

                vector_str = ots_column_map.get(Field.VECTOR)
                metadata_str = ots_column_map.get(Field.METADATA_KEY)

                vector = json.loads(vector_str) if vector_str else None
                metadata = parse_metadata_json(metadata_str)

                metadata["score"] = search_hit.score

                documents.append(
                    Document(
                        page_content=ots_column_map.get(Field.CONTENT_KEY) or "",
                        vector=vector,
                        metadata=metadata,
                    )
                )
        documents = sorted(documents, key=lambda x: x.metadata["score"] if x.metadata else 0, reverse=True)
        return documents

    @staticmethod
    def _normalize_score_exp_decay(score: float, k: float = 0.15) -> float:
        """
        Args:
            score: BM25 search score.
            k: decay factor, the larger the k, the steeper the low score end
        """
        normalized_score = 1 - math.exp(-k * score)
        return max(0.0, min(1.0, normalized_score))

    def _search_by_full_text(
        self, query: str, document_ids_filter: list[str] | None, top_k: int, score_threshold: float
    ) -> list[Document]:
        bool_query = tablestore.BoolQuery(must_queries=[], filter_queries=[], should_queries=[], must_not_queries=[])
        bool_query.must_queries.append(tablestore.MatchQuery(text=query, field_name=Field.CONTENT_KEY))

        if document_ids_filter:
            bool_query.filter_queries.append(tablestore.TermsQuery(self._tags_field, document_ids_filter))

        search_query = tablestore.SearchQuery(
            query=bool_query,
            sort=tablestore.Sort(sorters=[tablestore.ScoreSort(sort_order=tablestore.SortOrder.DESC)]),
            limit=top_k,
        )
        search_response = self._tablestore_client.search(
            table_name=self._table_name,
            index_name=self._index_name,
            search_query=search_query,
            columns_to_get=tablestore.ColumnsToGet(return_type=tablestore.ColumnReturnType.ALL_FROM_INDEX),
        )

        documents = []
        for search_hit in search_response.search_hits:
            score = None
            if self._normalize_full_text_bm25_score:
                score = self._normalize_score_exp_decay(search_hit.score)

            # skip when score is below threshold and use normalize score
            if score and score <= score_threshold:
                continue

            ots_column_map = {}
            for col in search_hit.row[1]:
                ots_column_map[col[0]] = col[1]

            metadata_str = ots_column_map.get(Field.METADATA_KEY)
            metadata = parse_metadata_json(metadata_str)

            vector_str = ots_column_map.get(Field.VECTOR)
            vector = json.loads(vector_str) if vector_str else None

            if score:
                metadata["score"] = score

            documents.append(
                Document(
                    page_content=ots_column_map.get(Field.CONTENT_KEY) or "",
                    vector=vector,
                    metadata=metadata,
                )
            )
        if self._normalize_full_text_bm25_score:
            documents = sorted(documents, key=lambda x: x.metadata["score"] if x.metadata else 0, reverse=True)
        return documents


class TableStoreVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> TableStoreVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.TABLESTORE, collection_name))

        return TableStoreVector(
            collection_name=collection_name,
            config=TableStoreConfig(
                endpoint=dify_config.TABLESTORE_ENDPOINT,
                instance_name=dify_config.TABLESTORE_INSTANCE_NAME,
                access_key_id=dify_config.TABLESTORE_ACCESS_KEY_ID,
                access_key_secret=dify_config.TABLESTORE_ACCESS_KEY_SECRET,
                normalize_full_text_bm25_score=dify_config.TABLESTORE_NORMALIZE_FULLTEXT_BM25_SCORE,
            ),
        )

```

### api/core/rag/datasource/vdb/tablestore/__init__.py
```py

```

### api/core/rag/datasource/vdb/vikingdb/vikingdb_vector.py
```py
import json
from typing import Any

from pydantic import BaseModel
from volcengine.viking_db import (  # type: ignore
    Data,
    DistanceType,
    Field,
    FieldType,
    IndexType,
    QuantType,
    VectorIndexParams,
    VikingDBService,
)

from configs import dify_config
from core.rag.datasource.vdb.field import Field as vdb_Field
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset


class VikingDBConfig(BaseModel):
    access_key: str
    secret_key: str
    host: str
    region: str
    scheme: str
    connection_timeout: int
    socket_timeout: int
    index_type: str = str(IndexType.HNSW)
    distance: str = str(DistanceType.L2)
    quant: str = str(QuantType.Float)


class VikingDBVector(BaseVector):
    def __init__(self, collection_name: str, group_id: str, config: VikingDBConfig):
        super().__init__(collection_name)
        self._group_id = group_id
        self._client_config = config
        self._index_name = f"{self._collection_name}_idx"
        self._client = VikingDBService(
            host=config.host,
            region=config.region,
            scheme=config.scheme,
            connection_timeout=config.connection_timeout,
            socket_timeout=config.socket_timeout,
            ak=config.access_key,
            sk=config.secret_key,
        )

    def _has_collection(self) -> bool:
        try:
            self._client.get_collection(self._collection_name)
        except Exception:
            return False
        return True

    def _has_index(self) -> bool:
        try:
            self._client.get_index(self._collection_name, self._index_name)
        except Exception:
            return False
        return True

    def _create_collection(self, dimension: int):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            if not self._has_collection():
                fields = [
                    Field(field_name=vdb_Field.PRIMARY_KEY, field_type=FieldType.String, is_primary_key=True),
                    Field(field_name=vdb_Field.METADATA_KEY, field_type=FieldType.String),
                    Field(field_name=vdb_Field.GROUP_KEY, field_type=FieldType.String),
                    Field(field_name=vdb_Field.CONTENT_KEY, field_type=FieldType.Text),
                    Field(field_name=vdb_Field.VECTOR, field_type=FieldType.Vector, dim=dimension),
                ]

                self._client.create_collection(
                    collection_name=self._collection_name,
                    fields=fields,
                    description="Collection For Dify",
                )

            if not self._has_index():
                vector_index = VectorIndexParams(
                    distance=self._client_config.distance,
                    index_type=self._client_config.index_type,
                    quant=self._client_config.quant,
                )

                self._client.create_index(
                    collection_name=self._collection_name,
                    index_name=self._index_name,
                    vector_index=vector_index,
                    partition_by=vdb_Field.GROUP_KEY,
                    description="Index For Dify",
                )
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def get_type(self) -> str:
        return VectorType.VIKINGDB

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        self.add_texts(texts, embeddings, **kwargs)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        page_contents = [doc.page_content for doc in documents]
        metadatas = [doc.metadata for doc in documents]
        docs = []

        for i, page_content in enumerate(page_contents):
            metadata = {}
            if metadatas is not None:
                for key, val in (metadatas[i] or {}).items():
                    metadata[key] = val
            # FIXME: fix the type of metadata later
            doc = Data(
                {
                    vdb_Field.PRIMARY_KEY: metadatas[i]["doc_id"],  # type: ignore
                    vdb_Field.VECTOR: embeddings[i] if embeddings else None,
                    vdb_Field.CONTENT_KEY: page_content,
                    vdb_Field.METADATA_KEY: json.dumps(metadata),
                    vdb_Field.GROUP_KEY: self._group_id,
                }
            )
            docs.append(doc)

        self._client.get_collection(self._collection_name).upsert_data(docs)

    def text_exists(self, id: str) -> bool:
        docs = self._client.get_collection(self._collection_name).fetch_data(id)
        not_exists_str = "data does not exist"
        if docs is not None and not_exists_str not in docs.fields.get("message", ""):
            return True
        return False

    def delete_by_ids(self, ids: list[str]):
        self._client.get_collection(self._collection_name).delete_data(ids)

    def get_ids_by_metadata_field(self, key: str, value: str):
        # Note: Metadata field value is an dict, but vikingdb field
        # not support json type
        results = self._client.get_index(self._collection_name, self._index_name).search(
            filter={"op": "must", "field": vdb_Field.GROUP_KEY, "conds": [self._group_id]},
            # max value is 5000
            limit=5000,
        )

        if not results:
            return []

        ids = []
        for result in results:
            metadata = result.fields.get(vdb_Field.METADATA_KEY)
            if metadata is not None:
                metadata = parse_metadata_json(metadata)
                if metadata.get(key) == value:
                    ids.append(result.id)
        return ids

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        self.delete_by_ids(ids)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        results = self._client.get_index(self._collection_name, self._index_name).search_by_vector(
            query_vector, limit=kwargs.get("top_k", 4)
        )
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        docs = self._get_search_res(results, score_threshold)
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            docs = [doc for doc in docs if doc.metadata.get("document_id") in document_ids_filter]
        return docs

    def _get_search_res(self, results, score_threshold) -> list[Document]:
        if len(results) == 0:
            return []

        docs = []
        for result in results:
            metadata = parse_metadata_json(result.fields.get(vdb_Field.METADATA_KEY))
            if result.score >= score_threshold:
                metadata["score"] = result.score
                doc = Document(page_content=result.fields.get(vdb_Field.CONTENT_KEY), metadata=metadata)
                docs.append(doc)
        docs = sorted(docs, key=lambda x: x.metadata.get("score", 0) if x.metadata else 0, reverse=True)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return []

    def delete(self):
        if self._has_index():
            self._client.drop_index(self._collection_name, self._index_name)
        if self._has_collection():
            self._client.drop_collection(self._collection_name)


class VikingDBVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> VikingDBVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.VIKINGDB, collection_name))

        if dify_config.VIKINGDB_ACCESS_KEY is None:
            raise ValueError("VIKINGDB_ACCESS_KEY should not be None")
        if dify_config.VIKINGDB_SECRET_KEY is None:
            raise ValueError("VIKINGDB_SECRET_KEY should not be None")
        if dify_config.VIKINGDB_HOST is None:
            raise ValueError("VIKINGDB_HOST should not be None")
        if dify_config.VIKINGDB_REGION is None:
            raise ValueError("VIKINGDB_REGION should not be None")
        if dify_config.VIKINGDB_SCHEME is None:
            raise ValueError("VIKINGDB_SCHEME should not be None")
        return VikingDBVector(
            collection_name=collection_name,
            group_id=dataset.id,
            config=VikingDBConfig(
                access_key=dify_config.VIKINGDB_ACCESS_KEY,
                secret_key=dify_config.VIKINGDB_SECRET_KEY,
                host=dify_config.VIKINGDB_HOST,
                region=dify_config.VIKINGDB_REGION,
                scheme=dify_config.VIKINGDB_SCHEME,
                connection_timeout=dify_config.VIKINGDB_CONNECTION_TIMEOUT,
                socket_timeout=dify_config.VIKINGDB_SOCKET_TIMEOUT,
            ),
        )

```

### api/core/rag/datasource/vdb/vikingdb/__init__.py
```py

```

### api/core/rag/datasource/vdb/qdrant/__init__.py
```py

```

### api/core/rag/datasource/vdb/qdrant/qdrant_vector.py
```py
import json
import os
import uuid
from collections.abc import Generator, Iterable, Sequence
from itertools import islice
from typing import TYPE_CHECKING, Any

import qdrant_client
from flask import current_app
from pydantic import BaseModel
from qdrant_client.http import models as rest
from qdrant_client.http.models import (
    FilterSelector,
    HnswConfigDiff,
    PayloadSchemaType,
    TextIndexParams,
    TextIndexType,
    TokenizerType,
)
from qdrant_client.local.qdrant_local import QdrantLocal
from sqlalchemy import select

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from models.dataset import Dataset, DatasetCollectionBinding

if TYPE_CHECKING:
    from qdrant_client import grpc  # noqa
    from qdrant_client.conversions import common_types
    from qdrant_client.http import models as rest

    type DictFilter = dict[str, str | int | bool | dict | list]
    type MetadataFilter = DictFilter | common_types.Filter


class PathQdrantParams(BaseModel):
    path: str


class UrlQdrantParams(BaseModel):
    url: str
    api_key: str | None
    timeout: float
    verify: bool
    grpc_port: int
    prefer_grpc: bool


class QdrantConfig(BaseModel):
    endpoint: str
    api_key: str | None = None
    timeout: float = 20
    root_path: str | None = None
    grpc_port: int = 6334
    prefer_grpc: bool = False
    replication_factor: int = 1
    write_consistency_factor: int = 1

    def to_qdrant_params(self) -> PathQdrantParams | UrlQdrantParams:
        if self.endpoint and self.endpoint.startswith("path:"):
            path = self.endpoint.replace("path:", "")
            if not os.path.isabs(path):
                if not self.root_path:
                    raise ValueError("Root path is not set")
                path = os.path.join(self.root_path, path)

            return PathQdrantParams(path=path)
        else:
            return UrlQdrantParams(
                url=self.endpoint,
                api_key=self.api_key,
                timeout=self.timeout,
                verify=self.endpoint.startswith("https"),
                grpc_port=self.grpc_port,
                prefer_grpc=self.prefer_grpc,
            )


class QdrantVector(BaseVector):
    def __init__(self, collection_name: str, group_id: str, config: QdrantConfig, distance_func: str = "Cosine"):
        super().__init__(collection_name)
        self._client_config = config
        self._client = qdrant_client.QdrantClient(**self._client_config.to_qdrant_params().model_dump())
        self._distance_func = distance_func.upper()
        self._group_id = group_id

    def get_type(self) -> str:
        return VectorType.QDRANT

    def to_index_struct(self):
        return {"type": self.get_type(), "vector_store": {"class_prefix": self._collection_name}}

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        if texts:
            # get embedding vector size
            vector_size = len(embeddings[0])
            # get collection name
            collection_name = self._collection_name
            # create collection
            self.create_collection(collection_name, vector_size)

            self.add_texts(texts, embeddings, **kwargs)

    def create_collection(self, collection_name: str, vector_size: int):
        lock_name = f"vector_indexing_lock_{collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            collection_name = collection_name or uuid.uuid4().hex
            all_collection_name = []
            collections_response = self._client.get_collections()
            collection_list = collections_response.collections
            for collection in collection_list:
                all_collection_name.append(collection.name)
            if collection_name not in all_collection_name:
                from qdrant_client.http import models as rest

                vectors_config = rest.VectorParams(
                    size=vector_size,
                    distance=rest.Distance[self._distance_func],
                )
                hnsw_config = HnswConfigDiff(
                    m=0,
                    payload_m=16,
                    ef_construct=100,
                    full_scan_threshold=10000,
                    max_indexing_threads=0,
                    on_disk=False,
                )

                self._client.create_collection(
                    collection_name=collection_name,
                    vectors_config=vectors_config,
                    hnsw_config=hnsw_config,
                    timeout=int(self._client_config.timeout),
                    replication_factor=self._client_config.replication_factor,
                    write_consistency_factor=self._client_config.write_consistency_factor,
                )

                # create group_id payload index
                self._client.create_payload_index(
                    collection_name, Field.GROUP_KEY, field_schema=PayloadSchemaType.KEYWORD
                )
                # create doc_id payload index
                self._client.create_payload_index(collection_name, Field.DOC_ID, field_schema=PayloadSchemaType.KEYWORD)
                # create document_id payload index
                self._client.create_payload_index(
                    collection_name, Field.DOCUMENT_ID, field_schema=PayloadSchemaType.KEYWORD
                )
                # create full text index
                text_index_params = TextIndexParams(
                    type=TextIndexType.TEXT,
                    tokenizer=TokenizerType.MULTILINGUAL,
                    min_token_len=2,
                    max_token_len=20,
                    lowercase=True,
                )
                self._client.create_payload_index(collection_name, Field.CONTENT_KEY, field_schema=text_index_params)
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)
        texts = [d.page_content for d in documents]
        metadatas = [d.metadata for d in documents]
        added_ids = []
        # Filter out None values from metadatas list to match expected type
        filtered_metadatas = [m for m in metadatas if m is not None]
        for batch_ids, points in self._generate_rest_batches(
            texts, embeddings, filtered_metadatas, uuids, 64, self._group_id
        ):
            self._client.upsert(collection_name=self._collection_name, points=points)
            added_ids.extend(batch_ids)

        return added_ids

    def _generate_rest_batches(
        self,
        texts: Iterable[str],
        embeddings: list[list[float]],
        metadatas: list[dict] | None = None,
        ids: Sequence[str] | None = None,
        batch_size: int = 64,
        group_id: str | None = None,
    ) -> Generator[tuple[list[str], list[rest.PointStruct]], None, None]:
        from qdrant_client.http import models as rest

        texts_iterator = iter(texts)
        embeddings_iterator = iter(embeddings)
        metadatas_iterator = iter(metadatas or [])
        ids_iterator = iter(ids or [uuid.uuid4().hex for _ in iter(texts)])
        while batch_texts := list(islice(texts_iterator, batch_size)):
            # Take the corresponding metadata and id for each text in a batch
            batch_metadatas = list(islice(metadatas_iterator, batch_size)) or None
            batch_ids = list(islice(ids_iterator, batch_size))

            # Generate the embeddings for all the texts in a batch
            batch_embeddings = list(islice(embeddings_iterator, batch_size))

            points = [
                rest.PointStruct(
                    id=point_id,
                    vector=vector,
                    payload=payload,
                )
                for point_id, vector, payload in zip(
                    batch_ids,
                    batch_embeddings,
                    self._build_payloads(
                        batch_texts,
                        batch_metadatas,
                        Field.CONTENT_KEY,
                        Field.METADATA_KEY,
                        group_id or "",  # Ensure group_id is never None
                        Field.GROUP_KEY,
                    ),
                )
            ]

            yield batch_ids, points

    @classmethod
    def _build_payloads(
        cls,
        texts: Iterable[str],
        metadatas: list[dict] | None,
        content_payload_key: str,
        metadata_payload_key: str,
        group_id: str,
        group_payload_key: str,
    ) -> list[dict]:
        payloads = []
        for i, text in enumerate(texts):
            if text is None:
                raise ValueError(
                    "At least one of the texts is None. Please remove it before "
                    "calling .from_texts or .add_texts on Qdrant instance."
                )
            metadata = metadatas[i] if metadatas is not None else None
            payloads.append({content_payload_key: text, metadata_payload_key: metadata, group_payload_key: group_id})

        return payloads

    def delete_by_metadata_field(self, key: str, value: str):
        from qdrant_client.http import models
        from qdrant_client.http.exceptions import UnexpectedResponse

        try:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key=f"metadata.{key}",
                        match=models.MatchValue(value=value),
                    ),
                ],
            )

            self._reload_if_needed()

            self._client.delete(
                collection_name=self._collection_name,
                points_selector=FilterSelector(filter=filter),
            )
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def delete(self):
        from qdrant_client.http import models
        from qdrant_client.http.exceptions import UnexpectedResponse

        try:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key="group_id",
                        match=models.MatchValue(value=self._group_id),
                    ),
                ],
            )
            self._client.delete(
                collection_name=self._collection_name,
                points_selector=FilterSelector(filter=filter),
            )
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def delete_by_ids(self, ids: list[str]):
        from qdrant_client.http import models
        from qdrant_client.http.exceptions import UnexpectedResponse

        try:
            filter = models.Filter(
                must=[
                    models.FieldCondition(
                        key="metadata.doc_id",
                        match=models.MatchAny(any=ids),
                    ),
                ],
            )
            self._client.delete(
                collection_name=self._collection_name,
                points_selector=FilterSelector(filter=filter),
            )
        except UnexpectedResponse as e:
            # Collection does not exist, so return
            if e.status_code == 404:
                return
            # Some other error occurred, so re-raise the exception
            else:
                raise e

    def text_exists(self, id: str) -> bool:
        all_collection_name = []
        collections_response = self._client.get_collections()
        collection_list = collections_response.collections
        for collection in collection_list:
            all_collection_name.append(collection.name)
        if self._collection_name not in all_collection_name:
            return False
        response = self._client.retrieve(collection_name=self._collection_name, ids=[id])

        return len(response) > 0

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        from qdrant_client.http import models

        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        if score_threshold >= 1:
            # return empty list because some versions of qdrant may response with 400 bad request，
            # and at the same time, the score_threshold with value 1 may be valid for other vector stores
            return []

        filter = models.Filter(
            must=[
                models.FieldCondition(
                    key="group_id",
                    match=models.MatchValue(value=self._group_id),
                ),
            ],
        )
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            if filter.must:
                filter.must.append(
                    models.FieldCondition(
                        key="metadata.document_id",
                        match=models.MatchAny(any=document_ids_filter),
                    )
                )
        results = self._client.search(
            collection_name=self._collection_name,
            query_vector=query_vector,
            query_filter=filter,
            limit=kwargs.get("top_k", 4),
            with_payload=True,
            with_vectors=True,
            score_threshold=score_threshold,
        )
        docs = []
        for result in results:
            if result.payload is None:
                continue
            metadata = result.payload.get(Field.METADATA_KEY) or {}
            # duplicate check score threshold
            if result.score >= score_threshold:
                metadata["score"] = result.score
                doc = Document(
                    page_content=result.payload.get(Field.CONTENT_KEY, ""),
                    metadata=metadata,
                )
                docs.append(doc)
        # Sort the documents by score in descending order
        docs = sorted(docs, key=lambda x: x.metadata["score"] if x.metadata is not None else 0, reverse=True)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """Return docs most similar by full-text search.

        Searches each keyword separately and merges results to ensure documents
        matching ANY keyword are returned (OR logic). Results are capped at top_k.

        Args:
            query: Search query text. Multi-word queries are split into keywords,
                   with each keyword searched separately. Limited to 10 keywords.
            **kwargs: Additional search parameters (top_k, document_ids_filter)

        Returns:
            List of up to top_k unique documents matching any query keyword.
        """
        from qdrant_client.http import models

        # Build base must conditions (AND logic) for metadata filters
        base_must_conditions: list = [
            models.FieldCondition(
                key="group_id",
                match=models.MatchValue(value=self._group_id),
            ),
        ]

        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            base_must_conditions.append(
                models.FieldCondition(
                    key="metadata.document_id",
                    match=models.MatchAny(any=document_ids_filter),
                )
            )

        # Split query into keywords, deduplicate and limit to prevent DoS
        keywords = list(dict.fromkeys(kw.strip() for kw in query.strip().split() if kw.strip()))[:10]

        if not keywords:
            return []

        top_k = kwargs.get("top_k", 2)
        seen_ids: set[str | int] = set()
        documents: list[Document] = []

        # Search each keyword separately and merge results.
        # This ensures each keyword gets its own search, preventing one keyword's
        # results from completely overshadowing another's due to scroll ordering.
        for keyword in keywords:
            scroll_filter = models.Filter(
                must=[
                    *base_must_conditions,
                    models.FieldCondition(
                        key="page_content",
                        match=models.MatchText(text=keyword),
                    ),
                ]
            )

            response = self._client.scroll(
                collection_name=self._collection_name,
                scroll_filter=scroll_filter,
                limit=top_k,
                with_payload=True,
                with_vectors=True,
            )
            results = response[0]

            for result in results:
                if result and result.id not in seen_ids:
                    seen_ids.add(result.id)
                    document = self._document_from_scored_point(result, Field.CONTENT_KEY, Field.METADATA_KEY)
                    documents.append(document)
                    if len(documents) >= top_k:
                        return documents

        return documents

    def _reload_if_needed(self):
        if isinstance(self._client, QdrantLocal):
            self._client._load()

    @classmethod
    def _document_from_scored_point(
        cls,
        scored_point: Any,
        content_payload_key: str,
        metadata_payload_key: str,
    ) -> Document:
        return Document(
            page_content=scored_point.payload.get(content_payload_key),
            vector=scored_point.vector,
            metadata=scored_point.payload.get(metadata_payload_key) or {},
        )


class QdrantVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> QdrantVector:
        if dataset.collection_binding_id:
            stmt = select(DatasetCollectionBinding).where(DatasetCollectionBinding.id == dataset.collection_binding_id)
            dataset_collection_binding = db.session.scalars(stmt).one_or_none()
            if dataset_collection_binding:
                collection_name = dataset_collection_binding.collection_name
            else:
                raise ValueError("Dataset Collection Bindings does not exist!")
        else:
            if dataset.index_struct_dict:
                class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
                collection_name = class_prefix
            else:
                dataset_id = dataset.id
                collection_name = Dataset.gen_collection_name_by_id(dataset_id)

        if not dataset.index_struct_dict:
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.QDRANT, collection_name))

        return QdrantVector(
            collection_name=collection_name,
            group_id=dataset.id,
            config=QdrantConfig(
                endpoint=dify_config.QDRANT_URL or "",
                api_key=dify_config.QDRANT_API_KEY,
                root_path=str(current_app.config.root_path),
                timeout=dify_config.QDRANT_CLIENT_TIMEOUT,
                grpc_port=dify_config.QDRANT_GRPC_PORT,
                prefer_grpc=dify_config.QDRANT_GRPC_ENABLED,
                replication_factor=dify_config.QDRANT_REPLICATION_FACTOR,
            ),
        )

```

### api/core/rag/datasource/vdb/pgvecto_rs/__init__.py
```py

```

### api/core/rag/datasource/vdb/pgvecto_rs/pgvecto_rs.py
```py
import json
import logging
from typing import Any
from uuid import UUID, uuid4

from numpy import ndarray
from pgvecto_rs.sqlalchemy import VECTOR  # type: ignore
from pydantic import BaseModel, model_validator
from sqlalchemy import Float, create_engine, insert, select, text
from sqlalchemy import text as sql_text
from sqlalchemy.dialects import postgresql
from sqlalchemy.orm import Mapped, Session, mapped_column

from configs import dify_config
from core.rag.datasource.vdb.pgvecto_rs.collection import CollectionORM
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class PgvectoRSConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config PGVECTO_RS_HOST is required")
        if not values["port"]:
            raise ValueError("config PGVECTO_RS_PORT is required")
        if not values["user"]:
            raise ValueError("config PGVECTO_RS_USER is required")
        if not values["password"]:
            raise ValueError("config PGVECTO_RS_PASSWORD is required")
        if not values["database"]:
            raise ValueError("config PGVECTO_RS_DATABASE is required")
        return values


class PGVectoRS(BaseVector):
    def __init__(self, collection_name: str, config: PgvectoRSConfig, dim: int):
        super().__init__(collection_name)
        self._client_config = config
        self._url = (
            f"postgresql+psycopg2://{config.user}:{config.password}@{config.host}:{config.port}/{config.database}"
        )
        self._client = create_engine(self._url)
        with Session(self._client) as session:
            session.execute(text("CREATE EXTENSION IF NOT EXISTS vectors"))
            session.commit()
        self._fields: list[str] = []

        class _Table(CollectionORM):
            __tablename__ = collection_name
            __table_args__ = {"extend_existing": True}
            id: Mapped[UUID] = mapped_column(
                postgresql.UUID(as_uuid=True),
                primary_key=True,
            )
            text: Mapped[str]
            meta: Mapped[dict] = mapped_column(postgresql.JSONB)
            vector: Mapped[ndarray] = mapped_column(VECTOR(dim))

        self._table = _Table
        self._distance_op = "<=>"

    def get_type(self) -> str:
        return VectorType.PGVECTO_RS

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self.create_collection(len(embeddings[0]))
        self.add_texts(texts, embeddings)

    def create_collection(self, dimension: int):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            index_name = f"{self._collection_name}_embedding_index"
            with Session(self._client) as session:
                create_statement = sql_text(f"""
                    CREATE TABLE IF NOT EXISTS {self._collection_name} (
                        id UUID PRIMARY KEY,
                        text TEXT NOT NULL,
                        meta JSONB NOT NULL,
                        vector vector({dimension}) NOT NULL
                    ) using heap;
                """)
                session.execute(create_statement)
                index_statement = sql_text(f"""
                        CREATE INDEX IF NOT EXISTS {index_name}
                        ON {self._collection_name} USING vectors(vector vector_l2_ops)
                        WITH (options = $$
                                optimizing.optimizing_threads = 30
                                segment.max_growing_segment_size = 2000
                                segment.max_sealed_segment_size = 30000000
                                [indexing.hnsw]
                                m=30
                                ef_construction=500
                                $$);
                    """)
                session.execute(index_statement)
                session.commit()
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        pks = []
        with Session(self._client) as session:
            for document, embedding in zip(documents, embeddings):
                pk = uuid4()
                session.execute(
                    insert(self._table).values(
                        id=pk,
                        text=document.page_content,
                        meta=document.metadata,
                        vector=embedding,
                    ),
                )
                pks.append(pk)
            session.commit()

        return pks

    def get_ids_by_metadata_field(self, key: str, value: str):
        result = None
        with Session(self._client) as session:
            select_statement = sql_text(f"SELECT id FROM {self._collection_name} WHERE meta->>:key = :value")
            result = session.execute(select_statement, {"key": key, "value": value}).fetchall()
        if result:
            return [item[0] for item in result]
        else:
            return None

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            with Session(self._client) as session:
                select_statement = sql_text(f"DELETE FROM {self._collection_name} WHERE id = ANY(:ids)")
                session.execute(select_statement, {"ids": ids})
                session.commit()

    def delete_by_ids(self, ids: list[str]):
        with Session(self._client) as session:
            select_statement = sql_text(
                f"SELECT id FROM {self._collection_name} WHERE meta->>'doc_id' = ANY (:doc_ids); "
            )
            result = session.execute(select_statement, {"doc_ids": ids}).fetchall()
        if result:
            ids = [item[0] for item in result]
            if ids:
                with Session(self._client) as session:
                    select_statement = sql_text(f"DELETE FROM {self._collection_name} WHERE id = ANY(:ids)")
                    session.execute(select_statement, {"ids": ids})
                    session.commit()

    def delete(self):
        with Session(self._client) as session:
            session.execute(sql_text(f"DROP TABLE IF EXISTS {self._collection_name}"))
            session.commit()

    def text_exists(self, id: str) -> bool:
        with Session(self._client) as session:
            select_statement = sql_text(
                f"SELECT id FROM {self._collection_name} WHERE meta->>'doc_id' = :doc_id limit 1"
            )
            result = session.execute(select_statement, {"doc_id": id}).fetchall()
        return len(result) > 0

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        with Session(self._client) as session:
            stmt = (
                select(
                    self._table,
                    self._table.vector.op(self._distance_op, return_type=Float)(
                        query_vector,
                    ).label("distance"),
                )
                .limit(kwargs.get("top_k", 4))
                .order_by("distance")
            )
            document_ids_filter = kwargs.get("document_ids_filter")
            if document_ids_filter:
                stmt = stmt.where(self._table.meta["document_id"].in_(document_ids_filter))
            res = session.execute(stmt)
            results = [(row[0], row[1]) for row in res]

        # Organize results.
        docs = []
        for record, dis in results:
            metadata = record.meta
            score = 1 - dis
            metadata["score"] = score
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            if score >= score_threshold:
                doc = Document(page_content=record.text, metadata=metadata)
                docs.append(doc)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return []


class PGVectoRSFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> PGVectoRS:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.PGVECTO_RS, collection_name))
        dim = len(embeddings.embed_query("pgvecto_rs"))

        return PGVectoRS(
            collection_name=collection_name,
            config=PgvectoRSConfig(
                host=dify_config.PGVECTO_RS_HOST or "localhost",
                port=dify_config.PGVECTO_RS_PORT or 5432,
                user=dify_config.PGVECTO_RS_USER or "postgres",
                password=dify_config.PGVECTO_RS_PASSWORD or "",
                database=dify_config.PGVECTO_RS_DATABASE or "postgres",
            ),
            dim=dim,
        )

```

### api/core/rag/datasource/vdb/pgvecto_rs/collection.py
```py
from uuid import UUID

from numpy import ndarray
from sqlalchemy.orm import DeclarativeBase, Mapped


class CollectionORM(DeclarativeBase):
    __tablename__: str
    id: Mapped[UUID]
    text: Mapped[str]
    meta: Mapped[dict]
    vector: Mapped[ndarray]

```

### api/core/rag/datasource/vdb/myscale/__init__.py
```py

```

### api/core/rag/datasource/vdb/myscale/myscale_vector.py
```py
import json
import logging
import uuid
from enum import StrEnum
from typing import Any

from clickhouse_connect import get_client  # type: ignore[import-untyped]
from pydantic import BaseModel

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class MyScaleConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    fts_params: str


class SortOrder(StrEnum):
    ASC = "ASC"
    DESC = "DESC"


class MyScaleVector(BaseVector):
    def __init__(self, collection_name: str, config: MyScaleConfig, metric: str = "Cosine"):
        super().__init__(collection_name)
        self._config = config
        self._metric = metric
        self._vec_order = SortOrder.ASC if metric.upper() in {"COSINE", "L2"} else SortOrder.DESC
        self._client = get_client(
            host=config.host,
            port=config.port,
            username=config.user,
            password=config.password,
        )
        self._client.command("SET allow_experimental_object_type=1")

    def get_type(self) -> str:
        return VectorType.MYSCALE

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(documents=texts, embeddings=embeddings, **kwargs)

    def _create_collection(self, dimension: int):
        logger.info("create MyScale collection %s with dimension %s", self._collection_name, dimension)
        self._client.command(f"CREATE DATABASE IF NOT EXISTS {self._config.database}")
        fts_params = f"('{self._config.fts_params}')" if self._config.fts_params else ""
        sql = f"""
            CREATE TABLE IF NOT EXISTS {self._config.database}.{self._collection_name}(
                id String,
                text String,
                vector Array(Float32),
                metadata JSON,
                CONSTRAINT cons_vec_len CHECK length(vector) = {dimension},
                VECTOR INDEX vidx vector TYPE DEFAULT('metric_type = {self._metric}'),
                INDEX text_idx text TYPE fts{fts_params}
            ) ENGINE = MergeTree ORDER BY id
        """
        self._client.command(sql)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        ids = []
        columns = ["id", "text", "vector", "metadata"]
        values = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                row = (
                    doc_id,
                    self.escape_str(doc.page_content),
                    embeddings[i],
                    json.dumps(doc.metadata) if doc.metadata else {},
                )
                values.append(str(row))
                ids.append(doc_id)
        sql = f"""
            INSERT INTO {self._config.database}.{self._collection_name}
            ({",".join(columns)}) VALUES {",".join(values)}
        """
        self._client.command(sql)
        return ids

    @staticmethod
    def escape_str(value: Any) -> str:
        return "".join(" " if c in {"\\", "'"} else c for c in str(value))

    def text_exists(self, id: str) -> bool:
        results = self._client.query(f"SELECT id FROM {self._config.database}.{self._collection_name} WHERE id='{id}'")
        return results.row_count > 0

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        self._client.command(
            f"DELETE FROM {self._config.database}.{self._collection_name} WHERE id IN {str(tuple(ids))}"
        )

    def get_ids_by_metadata_field(self, key: str, value: str):
        rows = self._client.query(
            f"SELECT DISTINCT id FROM {self._config.database}.{self._collection_name} WHERE metadata.{key}='{value}'"
        ).result_rows
        return [row[0] for row in rows]

    def delete_by_metadata_field(self, key: str, value: str):
        self._client.command(
            f"DELETE FROM {self._config.database}.{self._collection_name} WHERE metadata.{key}='{value}'"
        )

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        return self._search(f"distance(vector, {str(query_vector)})", self._vec_order, **kwargs)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return self._search(f"TextSearch('enable_nlq=false')(text, '{query}')", SortOrder.DESC, **kwargs)

    def _search(self, dist: str, order: SortOrder, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        where_str = (
            f"WHERE dist < {1 - score_threshold}"
            if self._metric.upper() == "COSINE" and order == SortOrder.ASC and score_threshold > 0.0
            else ""
        )
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_str = f"{where_str} AND metadata['document_id'] in ({document_ids})"
        sql = f"""
            SELECT text, vector, metadata, {dist} as dist FROM {self._config.database}.{self._collection_name}
            {where_str} ORDER BY dist {order.value} LIMIT {top_k}
        """
        try:
            return [
                Document(
                    page_content=r["text"],
                    vector=r["vector"],
                    metadata=r["metadata"],
                )
                for r in self._client.query(sql).named_results()
            ]
        except Exception:
            logger.exception("Vector search operation failed")
            return []

    def delete(self):
        self._client.command(f"DROP TABLE IF EXISTS {self._config.database}.{self._collection_name}")


class MyScaleVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> MyScaleVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.MYSCALE, collection_name))

        return MyScaleVector(
            collection_name=collection_name,
            config=MyScaleConfig(
                host=dify_config.MYSCALE_HOST,
                port=dify_config.MYSCALE_PORT,
                user=dify_config.MYSCALE_USER,
                password=dify_config.MYSCALE_PASSWORD,
                database=dify_config.MYSCALE_DATABASE,
                fts_params=dify_config.MYSCALE_FTS_PARAMS,
            ),
        )

```

### api/core/rag/datasource/vdb/tencent/tencent_vector.py
```py
import json
import logging
import math
from typing import Any

from pydantic import BaseModel
from tcvdb_text.encoder import BM25Encoder  # type: ignore
from tcvectordb import RPCVectorDBClient, VectorDBException  # type: ignore
from tcvectordb.model import document, enum  # type: ignore
from tcvectordb.model import index as vdb_index  # type: ignore
from tcvectordb.model.document import AnnSearch, Filter, KeywordSearch, WeightedRerank  # type: ignore

from configs import dify_config
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class TencentConfig(BaseModel):
    url: str
    api_key: str | None = None
    timeout: float = 30
    username: str | None = None
    database: str | None = None
    index_type: str = "HNSW"
    metric_type: str = "IP"
    shard: int = 1
    replicas: int = 2
    max_upsert_batch_size: int = 128
    enable_hybrid_search: bool = False  # Flag to enable hybrid search

    def to_tencent_params(self):
        return {"url": self.url, "username": self.username, "key": self.api_key, "timeout": self.timeout}


bm25 = BM25Encoder.default("zh")


class TencentVector(BaseVector):
    field_id: str = "id"
    field_vector: str = "vector"
    field_text: str = "text"
    field_metadata: str = "metadata"

    def __init__(self, collection_name: str, config: TencentConfig):
        super().__init__(collection_name)
        self._client_config = config
        self._client = RPCVectorDBClient(**self._client_config.to_tencent_params())
        self._enable_hybrid_search = False
        self._dimension = 1024
        self._init_database()
        self._load_collection()

    def _load_collection(self):
        """
        Check if the collection supports hybrid search.
        """
        if self._client_config.enable_hybrid_search:
            self._enable_hybrid_search = True
            if self._has_collection():
                coll = self._client.describe_collection(
                    database_name=self._client_config.database, collection_name=self.collection_name
                )
                has_hybrid_search = False
                for idx in coll.indexes:
                    if idx.name == "sparse_vector":
                        has_hybrid_search = True
                    elif idx.name == "vector":
                        self._dimension = idx.dimension
                if not has_hybrid_search:
                    self._enable_hybrid_search = False

    def _init_database(self):
        return self._client.create_database_if_not_exists(database_name=self._client_config.database)

    def get_type(self) -> str:
        return VectorType.TENCENT

    def to_index_struct(self):
        return {"type": self.get_type(), "vector_store": {"class_prefix": self._collection_name}}

    def _has_collection(self) -> bool:
        return bool(
            self._client.exists_collection(
                database_name=self._client_config.database, collection_name=self.collection_name
            )
        )

    def _create_collection(self, dimension: int):
        self._dimension = dimension
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return

            if self._has_collection():
                return

            index_type = None
            for k, v in enum.IndexType.__members__.items():
                if k == self._client_config.index_type:
                    index_type = v
            if index_type is None:
                raise ValueError("unsupported index_type")
            metric_type = None
            for k, v in enum.MetricType.__members__.items():
                if k == self._client_config.metric_type:
                    metric_type = v
            if metric_type is None:
                raise ValueError("unsupported metric_type")
            params = vdb_index.HNSWParams(m=16, efconstruction=200)
            index_id = vdb_index.FilterIndex(self.field_id, enum.FieldType.String, enum.IndexType.PRIMARY_KEY)
            index_vector = vdb_index.VectorIndex(
                self.field_vector,
                dimension,
                index_type,
                metric_type,
                params,
            )
            index_metadate = vdb_index.FilterIndex(self.field_metadata, enum.FieldType.Json, enum.IndexType.FILTER)
            index_sparse_vector = vdb_index.SparseIndex(
                name="sparse_vector",
                field_type=enum.FieldType.SparseVector,
                index_type=enum.IndexType.SPARSE_INVERTED,
                metric_type=enum.MetricType.IP,
            )
            indexes = [index_id, index_vector, index_metadate]
            if self._enable_hybrid_search:
                indexes.append(index_sparse_vector)
            try:
                self._client.create_collection(
                    database_name=self._client_config.database,
                    collection_name=self._collection_name,
                    shard=self._client_config.shard,
                    replicas=self._client_config.replicas,
                    description="Collection for Dify",
                    indexes=indexes,
                )
            except VectorDBException as e:
                if "fieldType:json" not in e.message:
                    raise e
                # vdb version not support json, use string
                index_metadate = vdb_index.FilterIndex(
                    self.field_metadata, enum.FieldType.String, enum.IndexType.FILTER
                )
                indexes = [index_id, index_vector, index_metadate]
                if self._enable_hybrid_search:
                    indexes.append(index_sparse_vector)
                self._client.create_collection(
                    database_name=self._client_config.database,
                    collection_name=self._collection_name,
                    shard=self._client_config.shard,
                    replicas=self._client_config.replicas,
                    description="Collection for Dify",
                    indexes=indexes,
                )
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self._create_collection(len(embeddings[0]))
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        texts = [doc.page_content for doc in documents]
        metadatas = [doc.metadata for doc in documents]
        total_count = len(embeddings)
        batch_size = self._client_config.max_upsert_batch_size
        batch = math.ceil(total_count / batch_size)
        for j in range(batch):
            docs = []
            start_idx = j * batch_size
            end_idx = min(total_count, (j + 1) * batch_size)
            for i in range(start_idx, end_idx):
                if metadatas is None:
                    continue
                metadata = metadatas[i] or {}
                doc = document.Document(
                    id=metadata.get("doc_id"),
                    vector=embeddings[i],
                    text=texts[i],
                    metadata=metadata,
                )
                if self._enable_hybrid_search:
                    doc.__dict__["sparse_vector"] = bm25.encode_texts(texts[i])
                docs.append(doc)
            self._client.upsert(
                database_name=self._client_config.database,
                collection_name=self.collection_name,
                documents=docs,
                timeout=self._client_config.timeout,
            )

    def text_exists(self, id: str) -> bool:
        docs = self._client.query(
            database_name=self._client_config.database, collection_name=self.collection_name, document_ids=[id]
        )
        if docs and len(docs) > 0:
            return True
        return False

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return

        total_count = len(ids)
        batch_size = self._client_config.max_upsert_batch_size
        batch = math.ceil(total_count / batch_size)

        for j in range(batch):
            start_idx = j * batch_size
            end_idx = min(total_count, (j + 1) * batch_size)
            batch_ids = ids[start_idx:end_idx]

            self._client.delete(
                database_name=self._client_config.database, collection_name=self.collection_name, document_ids=batch_ids
            )

    def delete_by_metadata_field(self, key: str, value: str):
        self._client.delete(
            database_name=self._client_config.database,
            collection_name=self.collection_name,
            filter=Filter(Filter.In(f"metadata.{key}", [value])),
        )

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = None
        if document_ids_filter:
            filter = Filter(Filter.In("metadata.document_id", document_ids_filter))
        res = self._client.search(
            database_name=self._client_config.database,
            collection_name=self.collection_name,
            vectors=[query_vector],
            filter=filter,
            params=document.HNSWSearchParams(ef=kwargs.get("ef", 10)),
            retrieve_vector=False,
            limit=kwargs.get("top_k", 4),
            timeout=self._client_config.timeout,
        )
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._get_search_res(res, score_threshold)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = None
        if document_ids_filter:
            filter = Filter(Filter.In("metadata.document_id", document_ids_filter))
        if not self._enable_hybrid_search:
            return []
        res = self._client.hybrid_search(
            database_name=self._client_config.database,
            collection_name=self.collection_name,
            ann=[
                AnnSearch(
                    field_name="vector",
                    data=[0.0] * self._dimension,
                )
            ],
            match=[
                KeywordSearch(
                    field_name="sparse_vector",
                    data=bm25.encode_queries(query),
                ),
            ],
            rerank=WeightedRerank(
                field_list=["vector", "sparse_vector"],
                weight=[0, 1],
            ),
            retrieve_vector=False,
            limit=kwargs.get("top_k", 4),
            filter=filter,
        )
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        return self._get_search_res(res, score_threshold)

    def _get_search_res(self, res: list | None, score_threshold: float) -> list[Document]:
        docs: list[Document] = []
        if res is None or len(res) == 0:
            return docs

        for result in res[0]:
            raw_meta = result.get(self.field_metadata)
            # Compatible with version 1.1.3 and below: str means old driver.
            score = (1 - result.get("score", 0.0)) if isinstance(raw_meta, str) else result.get("score", 0.0)
            meta = parse_metadata_json(raw_meta)
            if score >= score_threshold:
                meta["score"] = score
                doc = Document(page_content=result.get(self.field_text), metadata=meta)
                docs.append(doc)
        return docs

    def delete(self):
        if self._has_collection():
            self._client.drop_collection(
                database_name=self._client_config.database, collection_name=self.collection_name
            )


class TencentVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> TencentVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.TENCENT, collection_name))

        return TencentVector(
            collection_name=collection_name,
            config=TencentConfig(
                url=dify_config.TENCENT_VECTOR_DB_URL or "",
                api_key=dify_config.TENCENT_VECTOR_DB_API_KEY,
                timeout=dify_config.TENCENT_VECTOR_DB_TIMEOUT,
                username=dify_config.TENCENT_VECTOR_DB_USERNAME,
                database=dify_config.TENCENT_VECTOR_DB_DATABASE,
                shard=dify_config.TENCENT_VECTOR_DB_SHARD,
                replicas=dify_config.TENCENT_VECTOR_DB_REPLICAS,
                enable_hybrid_search=dify_config.TENCENT_VECTOR_DB_ENABLE_HYBRID_SEARCH or False,
            ),
        )

```

### api/core/rag/datasource/vdb/tencent/__init__.py
```py

```

### api/core/rag/datasource/vdb/lindorm/__init__.py
```py

```

### api/core/rag/datasource/vdb/lindorm/lindorm_vector.py
```py
import json
import logging
import time
from typing import Any

from opensearchpy import OpenSearch, helpers
from opensearchpy.helpers import BulkIndexError
from pydantic import BaseModel, model_validator
from tenacity import retry, stop_after_attempt, wait_exponential

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
logging.getLogger("lindorm").setLevel(logging.WARN)

ROUTING_FIELD = "routing_field"
UGC_INDEX_PREFIX = "ugc_index"


class LindormVectorStoreConfig(BaseModel):
    hosts: str | None
    username: str | None = None
    password: str | None = None
    using_ugc: bool | None = False
    request_timeout: float | None = 1.0  # timeout units: s

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["hosts"]:
            raise ValueError("config URL is required")
        if not values["username"]:
            raise ValueError("config USERNAME is required")
        if not values["password"]:
            raise ValueError("config PASSWORD is required")
        return values

    def to_opensearch_params(self) -> dict[str, Any]:
        params: dict[str, Any] = {
            "hosts": self.hosts,
            "use_ssl": False,
            "pool_maxsize": 128,
            "timeout": 30,
        }
        if self.username and self.password:
            params["http_auth"] = (self.username, self.password)
        return params


class LindormVectorStore(BaseVector):
    def __init__(self, collection_name: str, config: LindormVectorStoreConfig, using_ugc: bool, **kwargs):
        self._routing: str | None = None
        if using_ugc:
            routing_value: str | None = kwargs.get("routing_value")
            if routing_value is None:
                raise ValueError("UGC index should init vector with valid 'routing_value' parameter value")
            self._routing = routing_value.lower()
        super().__init__(collection_name.lower())
        self._client_config = config
        self._client = OpenSearch(**config.to_opensearch_params())
        self._using_ugc = using_ugc
        self.kwargs = kwargs

    def get_type(self) -> str:
        return VectorType.LINDORM

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        metadatas = [d.metadata if d.metadata is not None else {} for d in texts]
        self.create_collection(embeddings, metadatas)
        self.add_texts(texts, embeddings)

    def refresh(self):
        self._client.indices.refresh(index=self._collection_name)

    def add_texts(
        self,
        documents: list[Document],
        embeddings: list[list[float]],
        batch_size: int = 64,
        timeout: int = 60,
        **kwargs,
    ):
        logger.info("Total documents to add: %s", len(documents))
        uuids = self._get_uuids(documents)

        total_docs = len(documents)
        num_batches = (total_docs + batch_size - 1) // batch_size

        @retry(
            stop=stop_after_attempt(3),
            wait=wait_exponential(multiplier=1, min=4, max=10),
        )
        def _bulk_with_retry(actions):
            try:
                response = self._client.bulk(actions, timeout=timeout)
                if response["errors"]:
                    error_items = [item for item in response["items"] if "error" in item["index"]]
                    error_msg = f"Bulk indexing had {len(error_items)} errors"
                    logger.exception(error_msg)
                    raise Exception(error_msg)
                return response
            except Exception:
                logger.exception("Bulk indexing error")
                raise

        for batch_num in range(num_batches):
            start_idx = batch_num * batch_size
            end_idx = min((batch_num + 1) * batch_size, total_docs)

            actions = []
            for i in range(start_idx, end_idx):
                action_header = {
                    "index": {
                        "_index": self.collection_name,
                        "_id": uuids[i],
                    }
                }
                action_values: dict[str, Any] = {
                    Field.CONTENT_KEY: documents[i].page_content,
                    Field.VECTOR: embeddings[i],
                    Field.METADATA_KEY: documents[i].metadata,
                }
                if self._using_ugc:
                    action_header["index"]["routing"] = self._routing
                    action_values[ROUTING_FIELD] = self._routing

                actions.append(action_header)
                actions.append(action_values)

            try:
                _bulk_with_retry(actions)
                # logger.info(f"Successfully processed batch {batch_num + 1}")
                # simple latency to avoid too many requests in a short time
                if batch_num < num_batches - 1:
                    time.sleep(0.5)

            except Exception:
                logger.exception("Failed to process batch %s", batch_num + 1)
                raise

    def get_ids_by_metadata_field(self, key: str, value: str):
        query: dict[str, Any] = {
            "query": {"bool": {"must": [{"term": {f"{Field.METADATA_KEY}.{key}.keyword": value}}]}}
        }
        if self._using_ugc:
            query["query"]["bool"]["must"].append({"term": {f"{ROUTING_FIELD}.keyword": self._routing}})
        response = self._client.search(index=self._collection_name, body=query)
        if response["hits"]["hits"]:
            return [hit["_id"] for hit in response["hits"]["hits"]]
        else:
            return None

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self.delete_by_ids(ids)

    def delete_by_ids(self, ids: list[str]):
        """Delete documents by their IDs in batch.

        Args:
            ids: List of document IDs to delete
        """
        if not ids:
            return

        params = {"routing": self._routing} if self._using_ugc else {}

        # 1. First check if collection exists
        if not self._client.indices.exists(index=self._collection_name):
            logger.warning("Collection %s does not exist", self._collection_name)
            return

        # 2. Batch process deletions
        actions = []
        for id in ids:
            if self._client.exists(index=self._collection_name, id=id, params=params):
                actions.append(
                    {
                        "_op_type": "delete",
                        "_index": self._collection_name,
                        "_id": id,
                        **params,  # Include routing if using UGC
                    }
                )
            else:
                logger.warning("DELETE BY ID: ID %s does not exist in the index.", id)

        # 3. Perform bulk deletion if there are valid documents to delete
        if actions:
            try:
                helpers.bulk(self._client, actions)
            except BulkIndexError as e:
                for error in e.errors:
                    delete_error = error.get("delete", {})
                    status = delete_error.get("status")
                    doc_id = delete_error.get("_id")

                    if status == 404:
                        logger.warning("Document not found for deletion: %s", doc_id)
                    else:
                        logger.exception("Error deleting document: %s", error)

    def delete(self):
        if self._using_ugc:
            routing_filter_query = {
                "query": {"bool": {"must": [{"term": {f"{ROUTING_FIELD}.keyword": self._routing}}]}}
            }
            self._client.delete_by_query(self._collection_name, body=routing_filter_query)
            self.refresh()
        else:
            if self._client.indices.exists(index=self._collection_name):
                self._client.indices.delete(index=self._collection_name, params={"timeout": 60})
                logger.info("Delete index success")
            else:
                logger.warning("Index '%s' does not exist. No deletion performed.", self._collection_name)

    def text_exists(self, id: str) -> bool:
        try:
            params: dict[str, Any] = {}
            if self._using_ugc:
                params["routing"] = self._routing
            self._client.get(index=self._collection_name, id=id, params=params)
            return True
        except:
            return False

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        if not isinstance(query_vector, list):
            raise ValueError("query_vector should be a list of floats")

        if not all(isinstance(x, float) for x in query_vector):
            raise ValueError("All elements in query_vector should be floats")

        filters = []
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            filters.append({"terms": {"metadata.document_id.keyword": document_ids_filter}})
        if self._using_ugc:
            filters.append({"term": {f"{ROUTING_FIELD}.keyword": self._routing}})

        top_k = kwargs.get("top_k", 5)
        search_query: dict[str, Any] = {
            "size": top_k,
            "_source": True,
            "query": {"knn": {Field.VECTOR: {"vector": query_vector, "k": top_k}}},
        }

        final_ext: dict[str, Any] = {"lvector": {}}
        if filters is not None and len(filters) > 0:
            # when using filter, transform filter from List[Dict] to Dict as valid format
            filter_dict = {"bool": {"must": filters}} if len(filters) > 1 else filters[0]
            search_query["query"]["knn"][Field.VECTOR]["filter"] = filter_dict  # filter should be Dict
            final_ext["lvector"]["filter_type"] = "pre_filter"

        if final_ext != {"lvector": {}}:
            search_query["ext"] = final_ext

        try:
            params = {"timeout": self._client_config.request_timeout}
            if self._using_ugc:
                params["routing"] = self._routing  # type: ignore
            response = self._client.search(index=self._collection_name, body=search_query, params=params)
        except Exception:
            logger.exception("Error executing vector search, query: %s", search_query)
            raise

        docs_and_scores = []
        for hit in response["hits"]["hits"]:
            docs_and_scores.append(
                (
                    Document(
                        page_content=hit["_source"][Field.CONTENT_KEY],
                        vector=hit["_source"][Field.VECTOR],
                        metadata=hit["_source"][Field.METADATA_KEY],
                    ),
                    hit["_score"],
                )
            )
        docs = []
        for doc, score in docs_and_scores:
            score_threshold = kwargs.get("score_threshold", 0.0) or 0.0
            if score >= score_threshold:
                if doc.metadata is not None:
                    doc.metadata["score"] = score
                docs.append(doc)

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        full_text_query = {"query": {"bool": {"must": [{"match": {Field.CONTENT_KEY.value: query}}]}}}
        filters = []
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            filters.append({"terms": {"metadata.document_id.keyword": document_ids_filter}})
        if self._using_ugc:
            filters.append({"term": {f"{ROUTING_FIELD}.keyword": self._routing}})
        if filters:
            full_text_query["query"]["bool"]["filter"] = filters

        try:
            params: dict[str, Any] = {"timeout": self._client_config.request_timeout}
            if self._using_ugc:
                params["routing"] = self._routing
            response = self._client.search(index=self._collection_name, body=full_text_query, params=params)
        except Exception:
            logger.exception("Error executing vector search, query: %s", full_text_query)
            raise

        docs = []
        for hit in response["hits"]["hits"]:
            metadata = hit["_source"].get(Field.METADATA_KEY)
            vector = hit["_source"].get(Field.VECTOR)
            page_content = hit["_source"].get(Field.CONTENT_KEY)
            doc = Document(page_content=page_content, vector=vector, metadata=metadata)
            docs.append(doc)

        return docs

    def create_collection(
        self, embeddings: list, metadatas: list[dict] | None = None, index_params: dict | None = None
    ):
        if not embeddings:
            raise ValueError(f"Embeddings list cannot be empty for collection create '{self._collection_name}'")
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name)
                return
            if not self._client.indices.exists(index=self._collection_name):
                index_body = {
                    "settings": {"index": {"knn": True, "knn_routing": self._using_ugc}},
                    "mappings": {
                        "properties": {
                            Field.CONTENT_KEY: {"type": "text"},
                            Field.VECTOR: {
                                "type": "knn_vector",
                                "dimension": len(embeddings[0]),  # Make sure the dimension is correct here
                                "method": {
                                    "name": index_params.get("index_type", "hnsw")
                                    if index_params
                                    else dify_config.LINDORM_INDEX_TYPE,
                                    "space_type": index_params.get("space_type", "l2")
                                    if index_params
                                    else dify_config.LINDORM_DISTANCE_TYPE,
                                    "engine": "lvector",
                                },
                            },
                        }
                    },
                }
                logger.info("Creating Lindorm Search index %s", self._collection_name)
                self._client.indices.create(index=self._collection_name, body=index_body)
            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class LindormVectorStoreFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> LindormVectorStore:
        lindorm_config = LindormVectorStoreConfig(
            hosts=dify_config.LINDORM_URL,
            username=dify_config.LINDORM_USERNAME,
            password=dify_config.LINDORM_PASSWORD,
            using_ugc=dify_config.LINDORM_USING_UGC,
            request_timeout=dify_config.LINDORM_QUERY_TIMEOUT,
        )
        using_ugc = dify_config.LINDORM_USING_UGC
        if using_ugc is None:
            raise ValueError("LINDORM_USING_UGC is not set")
        routing_value = None
        if dataset.index_struct:
            # if an existed record's index_struct_dict doesn't contain using_ugc field,
            # it actually stores in the normal index format
            stored_in_ugc: bool = dataset.index_struct_dict.get("using_ugc", False)
            using_ugc = stored_in_ugc
            if stored_in_ugc:
                dimension = dataset.index_struct_dict["dimension"]
                index_type = dataset.index_struct_dict["index_type"]
                distance_type = dataset.index_struct_dict["distance_type"]
                routing_value = dataset.index_struct_dict["vector_store"]["class_prefix"]
                index_name = f"{UGC_INDEX_PREFIX}_{dimension}_{index_type}_{distance_type}".lower()
            else:
                index_name = dataset.index_struct_dict["vector_store"]["class_prefix"].lower()
        else:
            embedding_vector = embeddings.embed_query("hello word")
            dimension = len(embedding_vector)
            class_prefix = Dataset.gen_collection_name_by_id(dataset.id)
            index_struct_dict = {
                "type": VectorType.LINDORM,
                "vector_store": {"class_prefix": class_prefix},
                "index_type": dify_config.LINDORM_INDEX_TYPE,
                "dimension": dimension,
                "distance_type": dify_config.LINDORM_DISTANCE_TYPE,
                "using_ugc": using_ugc,
            }
            dataset.index_struct = json.dumps(index_struct_dict)
            if using_ugc:
                index_type = dify_config.LINDORM_INDEX_TYPE
                distance_type = dify_config.LINDORM_DISTANCE_TYPE
                index_name = f"{UGC_INDEX_PREFIX}_{dimension}_{index_type}_{distance_type}".lower()
                routing_value = class_prefix.lower()
            else:
                index_name = class_prefix.lower()
        return LindormVectorStore(index_name, lindorm_config, routing_value=routing_value, using_ugc=using_ugc)

```

### api/core/rag/datasource/vdb/alibabacloud_mysql/__init__.py
```py

```

### api/core/rag/datasource/vdb/alibabacloud_mysql/alibabacloud_mysql_vector.py
```py
import hashlib
import json
import logging
import uuid
from contextlib import contextmanager
from typing import Any, Literal, cast

import mysql.connector
from mysql.connector import Error as MySQLError
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class AlibabaCloudMySQLVectorConfig(BaseModel):
    host: str
    port: int
    user: str
    password: str
    database: str
    max_connection: int
    charset: str = "utf8mb4"
    distance_function: Literal["cosine", "euclidean"] = "cosine"
    hnsw_m: int = 6

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values.get("host"):
            raise ValueError("config ALIBABACLOUD_MYSQL_HOST is required")
        if not values.get("port"):
            raise ValueError("config ALIBABACLOUD_MYSQL_PORT is required")
        if not values.get("user"):
            raise ValueError("config ALIBABACLOUD_MYSQL_USER is required")
        if values.get("password") is None:
            raise ValueError("config ALIBABACLOUD_MYSQL_PASSWORD is required")
        if not values.get("database"):
            raise ValueError("config ALIBABACLOUD_MYSQL_DATABASE is required")
        if not values.get("max_connection"):
            raise ValueError("config ALIBABACLOUD_MYSQL_MAX_CONNECTION is required")
        return values


SQL_CREATE_TABLE = """
CREATE TABLE IF NOT EXISTS {table_name} (
    id VARCHAR(36) PRIMARY KEY,
    text LONGTEXT NOT NULL,
    meta JSON NOT NULL,
    embedding VECTOR({dimension}) NOT NULL,
    VECTOR INDEX (embedding) M={hnsw_m} DISTANCE={distance_function}
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"""

SQL_CREATE_META_INDEX = """
CREATE INDEX idx_{index_hash}_meta ON {table_name}
    ((CAST(JSON_UNQUOTE(JSON_EXTRACT(meta, '$.document_id')) AS CHAR(36))));
"""

SQL_CREATE_FULLTEXT_INDEX = """
CREATE FULLTEXT INDEX idx_{index_hash}_text ON {table_name} (text) WITH PARSER ngram;
"""


class AlibabaCloudMySQLVector(BaseVector):
    def __init__(self, collection_name: str, config: AlibabaCloudMySQLVectorConfig):
        super().__init__(collection_name)
        self.pool = self._create_connection_pool(config)
        self.table_name = collection_name.lower()
        self.index_hash = hashlib.md5(self.table_name.encode()).hexdigest()[:8]
        self.distance_function = config.distance_function.lower()
        self.hnsw_m = config.hnsw_m
        self._check_vector_support()

    def get_type(self) -> str:
        return VectorType.ALIBABACLOUD_MYSQL

    def _create_connection_pool(self, config: AlibabaCloudMySQLVectorConfig):
        # Create connection pool using mysql-connector-python pooling
        pool_config: dict[str, Any] = {
            "host": config.host,
            "port": config.port,
            "user": config.user,
            "password": config.password,
            "database": config.database,
            "charset": config.charset,
            "autocommit": True,
            "pool_name": f"pool_{self.collection_name}",
            "pool_size": config.max_connection,
            "pool_reset_session": True,
        }
        return mysql.connector.pooling.MySQLConnectionPool(**pool_config)

    def _check_vector_support(self):
        """Check if the MySQL server supports vector operations."""
        try:
            with self._get_cursor() as cur:
                # Check MySQL version and vector support
                cur.execute("SELECT VERSION()")
                version = cur.fetchone()["VERSION()"]
                logger.debug("Connected to MySQL version: %s", version)
                # Try to execute a simple vector function to verify support
                cur.execute("SELECT VEC_FromText('[1,2,3]') IS NOT NULL as vector_support")
                result = cur.fetchone()
                if not result or not result.get("vector_support"):
                    raise ValueError(
                        "RDS MySQL Vector functions are not available."
                        " Please ensure you're using RDS MySQL 8.0.36+ with Vector support."
                    )

        except MySQLError as e:
            if "FUNCTION" in str(e) and "VEC_FromText" in str(e):
                raise ValueError(
                    "RDS MySQL Vector functions are not available."
                    " Please ensure you're using RDS MySQL 8.0.36+ with Vector support."
                ) from e
            raise e

    @contextmanager
    def _get_cursor(self):
        conn = self.pool.get_connection()
        cur = conn.cursor(dictionary=True)
        try:
            yield cur
        finally:
            cur.close()
            conn.close()

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self._create_collection(dimension)
        return self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        pks = []
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                pks.append(doc_id)
                # Convert embedding list to Aliyun MySQL vector format
                vector_str = "[" + ",".join(map(str, embeddings[i])) + "]"
                values.append(
                    (
                        doc_id,
                        doc.page_content,
                        json.dumps(doc.metadata),
                        vector_str,
                    )
                )

        with self._get_cursor() as cur:
            insert_sql = (
                f"INSERT INTO {self.table_name} (id, text, meta, embedding) VALUES (%s, %s, %s, VEC_FromText(%s))"
            )
            cur.executemany(insert_sql, values)
        return pks

    def text_exists(self, id: str) -> bool:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT id FROM {self.table_name} WHERE id = %s", (id,))
            return cur.fetchone() is not None

    def get_by_ids(self, ids: list[str]) -> list[Document]:
        if not ids:
            return []

        with self._get_cursor() as cur:
            placeholders = ",".join(["%s"] * len(ids))
            cur.execute(f"SELECT meta, text FROM {self.table_name} WHERE id IN ({placeholders})", ids)
            docs = []
            for record in cur:
                metadata = parse_metadata_json(record["meta"])
                docs.append(Document(page_content=record["text"], metadata=metadata))
        return docs

    def delete_by_ids(self, ids: list[str]):
        # Avoiding crashes caused by performing delete operations on empty lists
        if not ids:
            return

        with self._get_cursor() as cur:
            try:
                placeholders = ",".join(["%s"] * len(ids))
                cur.execute(f"DELETE FROM {self.table_name} WHERE id IN ({placeholders})", ids)
            except MySQLError as e:
                if e.errno == 1146:  # Table doesn't exist
                    logger.warning("Table %s not found, skipping delete operation.", self.table_name)
                    return
                else:
                    raise e

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_cursor() as cur:
            cur.execute(
                f"DELETE FROM {self.table_name} WHERE JSON_UNQUOTE(JSON_EXTRACT(meta, %s)) = %s", (f"$.{key}", value)
            )

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search the nearest neighbors to a vector using RDS MySQL vector distance functions.

        :param query_vector: The input vector to search for similar items.
        :return: List of Documents that are nearest to the query vector.
        """
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")

        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        params = []

        if document_ids_filter:
            placeholders = ",".join(["%s"] * len(document_ids_filter))
            where_clause = f" WHERE JSON_UNQUOTE(JSON_EXTRACT(meta, '$.document_id')) IN ({placeholders}) "
            params.extend(document_ids_filter)

        # Convert query vector to RDS MySQL vector format
        query_vector_str = "[" + ",".join(map(str, query_vector)) + "]"

        # Use RSD MySQL's native vector distance functions
        with self._get_cursor() as cur:
            # Choose distance function based on configuration
            distance_func = "VEC_DISTANCE_COSINE" if self.distance_function == "cosine" else "VEC_DISTANCE_EUCLIDEAN"

            # Note: RDS MySQL optimizer will use vector index when ORDER BY + LIMIT are present
            # Use column alias in ORDER BY to avoid calculating distance twice
            sql = f"""
            SELECT meta, text,
                   {distance_func}(embedding, VEC_FromText(%s)) AS distance
            FROM {self.table_name}
            {where_clause}
            ORDER BY distance
            LIMIT %s
            """
            query_params = [query_vector_str] + params + [top_k]

            cur.execute(sql, query_params)

            docs = []
            score_threshold = float(kwargs.get("score_threshold") or 0.0)

            for record in cur:
                try:
                    distance = float(record["distance"])
                    # Convert distance to similarity score
                    if self.distance_function == "cosine":
                        # For cosine distance: similarity = 1 - distance
                        similarity = 1.0 - distance
                    else:
                        # For euclidean distance: use inverse relationship
                        # similarity = 1 / (1 + distance)
                        similarity = 1.0 / (1.0 + distance)

                    metadata = parse_metadata_json(record["meta"])
                    metadata["score"] = similarity
                    metadata["distance"] = distance

                    if similarity >= score_threshold:
                        docs.append(Document(page_content=record["text"], metadata=metadata))
                except (ValueError, TypeError) as e:
                    logger.warning("Error processing search result: %s", e)
                    continue

            return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 5)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")

        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        params = []

        if document_ids_filter:
            placeholders = ",".join(["%s"] * len(document_ids_filter))
            where_clause = f" AND JSON_UNQUOTE(JSON_EXTRACT(meta, '$.document_id')) IN ({placeholders}) "
            params.extend(document_ids_filter)

        with self._get_cursor() as cur:
            # Build query parameters: query (twice for MATCH clauses), document_ids_filter (if any), top_k
            query_params = [query, query] + params + [top_k]
            cur.execute(
                f"""SELECT meta, text,
                    MATCH(text) AGAINST(%s IN NATURAL LANGUAGE MODE) AS score
                    FROM {self.table_name}
                    WHERE MATCH(text) AGAINST(%s IN NATURAL LANGUAGE MODE)
                    {where_clause}
                    ORDER BY score DESC
                    LIMIT %s""",
                query_params,
            )
            docs = []
            for record in cur:
                metadata = parse_metadata_json(record["meta"])
                metadata["score"] = float(record["score"])
                docs.append(Document(page_content=record["text"], metadata=metadata))
        return docs

    def delete(self):
        with self._get_cursor() as cur:
            cur.execute(f"DROP TABLE IF EXISTS {self.table_name}")

    def _create_collection(self, dimension: int):
        collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{collection_exist_cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            if redis_client.get(collection_exist_cache_key):
                return

            with self._get_cursor() as cur:
                # Create table with vector column and vector index
                cur.execute(
                    SQL_CREATE_TABLE.format(
                        table_name=self.table_name,
                        dimension=dimension,
                        distance_function=self.distance_function,
                        hnsw_m=self.hnsw_m,
                    )
                )
                # Create metadata index (check if exists first)
                try:
                    cur.execute(SQL_CREATE_META_INDEX.format(table_name=self.table_name, index_hash=self.index_hash))
                except MySQLError as e:
                    if e.errno != 1061:  # Duplicate key name
                        logger.warning("Could not create meta index: %s", e)

                # Create full-text index for text search
                try:
                    cur.execute(
                        SQL_CREATE_FULLTEXT_INDEX.format(table_name=self.table_name, index_hash=self.index_hash)
                    )
                except MySQLError as e:
                    if e.errno != 1061:  # Duplicate key name
                        logger.warning("Could not create fulltext index: %s", e)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class AlibabaCloudMySQLVectorFactory(AbstractVectorFactory):
    def _validate_distance_function(self, distance_function: str) -> Literal["cosine", "euclidean"]:
        """Validate and return the distance function as a proper Literal type."""
        if distance_function not in ["cosine", "euclidean"]:
            raise ValueError(f"Invalid distance function: {distance_function}. Must be 'cosine' or 'euclidean'")
        return cast(Literal["cosine", "euclidean"], distance_function)

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> AlibabaCloudMySQLVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(
                self.gen_index_struct_dict(VectorType.ALIBABACLOUD_MYSQL, collection_name)
            )
        return AlibabaCloudMySQLVector(
            collection_name=collection_name,
            config=AlibabaCloudMySQLVectorConfig(
                host=dify_config.ALIBABACLOUD_MYSQL_HOST or "localhost",
                port=dify_config.ALIBABACLOUD_MYSQL_PORT,
                user=dify_config.ALIBABACLOUD_MYSQL_USER or "root",
                password=dify_config.ALIBABACLOUD_MYSQL_PASSWORD or "",
                database=dify_config.ALIBABACLOUD_MYSQL_DATABASE or "dify",
                max_connection=dify_config.ALIBABACLOUD_MYSQL_MAX_CONNECTION,
                charset=dify_config.ALIBABACLOUD_MYSQL_CHARSET or "utf8mb4",
                distance_function=self._validate_distance_function(
                    dify_config.ALIBABACLOUD_MYSQL_DISTANCE_FUNCTION or "cosine"
                ),
                hnsw_m=dify_config.ALIBABACLOUD_MYSQL_HNSW_M or 6,
            ),
        )

```

### api/core/rag/datasource/vdb/weaviate/weaviate_vector.py
```py
"""
Weaviate vector database implementation for Dify's RAG system.

This module provides integration with Weaviate vector database for storing and retrieving
document embeddings used in retrieval-augmented generation workflows.
"""

import atexit
import datetime
import json
import logging
import threading
import uuid as _uuid
from typing import Any
from urllib.parse import urlparse

import weaviate
import weaviate.classes.config as wc
from pydantic import BaseModel, model_validator
from weaviate.classes.data import DataObject
from weaviate.classes.init import Auth
from weaviate.classes.query import Filter, MetadataQuery
from weaviate.exceptions import UnexpectedStatusCodeError

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)

_weaviate_client: weaviate.WeaviateClient | None = None
_weaviate_client_lock = threading.Lock()


def _shutdown_weaviate_client() -> None:
    """
    Best-effort shutdown hook to close the module-level Weaviate client.

    This is registered with atexit so that HTTP/gRPC resources are released
    when the Python interpreter exits.
    """
    global _weaviate_client

    # Ensure thread-safety when accessing the shared client instance
    with _weaviate_client_lock:
        client = _weaviate_client
        _weaviate_client = None

    if client is not None:
        try:
            client.close()
        except Exception:
            # Best-effort cleanup; log at debug level and ignore errors.
            logger.debug("Failed to close Weaviate client during shutdown", exc_info=True)


# Register the shutdown hook once per process.
atexit.register(_shutdown_weaviate_client)


class WeaviateConfig(BaseModel):
    """
    Configuration model for Weaviate connection settings.

    Attributes:
        endpoint: Weaviate server endpoint URL
        grpc_endpoint: Optional Weaviate gRPC server endpoint URL
        api_key: Optional API key for authentication
        batch_size: Number of objects to batch per insert operation
    """

    endpoint: str
    grpc_endpoint: str | None = None
    api_key: str | None = None
    batch_size: int = 100

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict) -> dict:
        """Validates that required configuration values are present."""
        if not values["endpoint"]:
            raise ValueError("config WEAVIATE_ENDPOINT is required")
        return values


class WeaviateVector(BaseVector):
    """
    Weaviate vector database implementation for document storage and retrieval.

    Handles creation, insertion, deletion, and querying of document embeddings
    in a Weaviate collection.
    """

    _DOCUMENT_ID_PROPERTY = "document_id"

    def __init__(self, collection_name: str, config: WeaviateConfig, attributes: list):
        """
        Initializes the Weaviate vector store.

        Args:
            collection_name: Name of the Weaviate collection
            config: Weaviate configuration settings
            attributes: List of metadata attributes to store
        """
        super().__init__(collection_name)
        self._client = self._init_client(config)
        self._attributes = attributes

    def _init_client(self, config: WeaviateConfig) -> weaviate.WeaviateClient:
        """
        Initializes and returns a connected Weaviate client.

        Configures both HTTP and gRPC connections with proper authentication.
        """
        global _weaviate_client
        if _weaviate_client and _weaviate_client.is_ready():
            return _weaviate_client

        with _weaviate_client_lock:
            if _weaviate_client and _weaviate_client.is_ready():
                return _weaviate_client

            p = urlparse(config.endpoint)
            host = p.hostname or config.endpoint.replace("https://", "").replace("http://", "")
            http_secure = p.scheme == "https"
            http_port = p.port or (443 if http_secure else 80)

            # Parse gRPC configuration
            if config.grpc_endpoint:
                # Urls without scheme won't be parsed correctly in some python versions,
                # see https://bugs.python.org/issue27657
                grpc_endpoint_with_scheme = (
                    config.grpc_endpoint if "://" in config.grpc_endpoint else f"grpc://{config.grpc_endpoint}"
                )
                grpc_p = urlparse(grpc_endpoint_with_scheme)
                grpc_host = grpc_p.hostname or "localhost"
                grpc_port = grpc_p.port or (443 if grpc_p.scheme == "grpcs" else 50051)
                grpc_secure = grpc_p.scheme == "grpcs"
            else:
                # Infer from HTTP endpoint as fallback
                grpc_host = host
                grpc_secure = http_secure
                grpc_port = 443 if grpc_secure else 50051

            client = weaviate.connect_to_custom(
                http_host=host,
                http_port=http_port,
                http_secure=http_secure,
                grpc_host=grpc_host,
                grpc_port=grpc_port,
                grpc_secure=grpc_secure,
                auth_credentials=Auth.api_key(config.api_key) if config.api_key else None,
                skip_init_checks=True,  # Skip PyPI version check to avoid unnecessary HTTP requests
            )

            if not client.is_ready():
                raise ConnectionError("Vector database is not ready")

            _weaviate_client = client
            return client

    def get_type(self) -> str:
        """Returns the vector database type identifier."""
        return VectorType.WEAVIATE

    def get_collection_name(self, dataset: Dataset) -> str:
        """
        Retrieves or generates the collection name for a dataset.

        Uses existing index structure if available, otherwise generates from dataset ID.
        """
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            if not class_prefix.endswith("_Node"):
                class_prefix += "_Node"
            return class_prefix

        dataset_id = dataset.id
        return Dataset.gen_collection_name_by_id(dataset_id)

    def to_index_struct(self) -> dict:
        """Returns the index structure dictionary for persistence."""
        return {"type": self.get_type(), "vector_store": {"class_prefix": self._collection_name}}

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        """
        Creates a new collection and adds initial documents with embeddings.
        """
        self._create_collection()
        self.add_texts(texts, embeddings)

    def _create_collection(self):
        """
        Creates the Weaviate collection with required schema if it doesn't exist.

        Uses Redis locking to prevent concurrent creation attempts.
        """
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(cache_key):
                return

            try:
                if not self._client.collections.exists(self._collection_name):
                    tokenization = (
                        wc.Tokenization(dify_config.WEAVIATE_TOKENIZATION)
                        if dify_config.WEAVIATE_TOKENIZATION
                        else wc.Tokenization.WORD
                    )
                    self._client.collections.create(
                        name=self._collection_name,
                        properties=[
                            wc.Property(
                                name=Field.TEXT_KEY.value,
                                data_type=wc.DataType.TEXT,
                                tokenization=tokenization,
                            ),
                            wc.Property(name="document_id", data_type=wc.DataType.TEXT),
                            wc.Property(name="doc_id", data_type=wc.DataType.TEXT),
                            wc.Property(name="doc_type", data_type=wc.DataType.TEXT),
                            wc.Property(name="chunk_index", data_type=wc.DataType.INT),
                        ],
                        vector_config=wc.Configure.Vectors.self_provided(),
                    )

                self._ensure_properties()
                redis_client.set(cache_key, 1, ex=3600)
            except Exception as e:
                logger.exception("Error creating collection %s", self._collection_name)
                raise

    def _ensure_properties(self) -> None:
        """
        Ensures all required properties exist in the collection schema.

        Adds missing properties if the collection exists but lacks them.
        """
        if not self._client.collections.exists(self._collection_name):
            return

        col = self._client.collections.use(self._collection_name)
        cfg = col.config.get()
        existing = {p.name for p in (cfg.properties or [])}

        to_add = []
        if "document_id" not in existing:
            to_add.append(wc.Property(name="document_id", data_type=wc.DataType.TEXT))
        if "doc_id" not in existing:
            to_add.append(wc.Property(name="doc_id", data_type=wc.DataType.TEXT))
        if "doc_type" not in existing:
            to_add.append(wc.Property(name="doc_type", data_type=wc.DataType.TEXT))
        if "chunk_index" not in existing:
            to_add.append(wc.Property(name="chunk_index", data_type=wc.DataType.INT))

        for prop in to_add:
            try:
                col.config.add_property(prop)
            except Exception as e:
                logger.warning("Could not add property %s: %s", prop.name, e)

    def _get_uuids(self, documents: list[Document]) -> list[str]:
        """
        Generates deterministic UUIDs for documents based on their content.

        Uses UUID5 with URL namespace to ensure consistent IDs for identical content.
        """
        URL_NAMESPACE = _uuid.UUID("6ba7b811-9dad-11d1-80b4-00c04fd430c8")

        uuids = []
        for doc in documents:
            uuid_val = _uuid.uuid5(URL_NAMESPACE, doc.page_content)
            uuids.append(str(uuid_val))

        return uuids

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        """
        Adds documents with their embeddings to the collection.

        Batches insertions for efficiency and returns the list of inserted object IDs.
        """
        uuids = self._get_uuids(documents)
        texts = [d.page_content for d in documents]
        metadatas = [d.metadata for d in documents]

        col = self._client.collections.use(self._collection_name)
        objs: list[DataObject] = []
        ids_out: list[str] = []

        for i, text in enumerate(texts):
            props: dict[str, Any] = {Field.TEXT_KEY.value: text}
            meta = metadatas[i] or {}
            for k, v in meta.items():
                props[k] = self._json_serializable(v)

            candidate = uuids[i] if uuids else None
            uid = candidate if (candidate and self._is_uuid(candidate)) else str(_uuid.uuid4())
            ids_out.append(uid)

            vec_payload = None
            if embeddings and i < len(embeddings) and embeddings[i]:
                vec_payload = {"default": embeddings[i]}

            objs.append(
                DataObject(
                    uuid=uid,
                    properties=props,  # type: ignore[arg-type]  # mypy incorrectly infers DataObject signature
                    vector=vec_payload,
                )
            )

        with col.batch.dynamic() as batch:
            for obj in objs:
                batch.add_object(properties=obj.properties, uuid=obj.uuid, vector=obj.vector)

        return ids_out

    def _is_uuid(self, val: str) -> bool:
        """Validates whether a string is a valid UUID format."""
        try:
            _uuid.UUID(str(val))
            return True
        except Exception:
            return False

    def delete_by_metadata_field(self, key: str, value: str) -> None:
        """Deletes all objects matching a specific metadata field value."""
        if not self._client.collections.exists(self._collection_name):
            return

        col = self._client.collections.use(self._collection_name)
        col.data.delete_many(where=Filter.by_property(key).equal(value))

    def delete(self):
        """Deletes the entire collection from Weaviate."""
        if self._client.collections.exists(self._collection_name):
            self._client.collections.delete(self._collection_name)

    def text_exists(self, id: str) -> bool:
        """Checks if a document with the given doc_id exists in the collection."""
        if not self._client.collections.exists(self._collection_name):
            return False

        col = self._client.collections.use(self._collection_name)
        res = col.query.fetch_objects(
            filters=Filter.by_property("doc_id").equal(id),
            limit=1,
            return_properties=["doc_id"],
        )

        return len(res.objects) > 0

    def delete_by_ids(self, ids: list[str]) -> None:
        """
        Deletes objects by their UUID identifiers.

        Silently ignores 404 errors for non-existent IDs.
        """
        if not self._client.collections.exists(self._collection_name):
            return

        col = self._client.collections.use(self._collection_name)

        for uid in ids:
            try:
                col.data.delete_by_id(uid)
            except UnexpectedStatusCodeError as e:
                if getattr(e, "status_code", None) != 404:
                    raise

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Performs vector similarity search using the provided query vector.

        Filters by document IDs if provided and applies score threshold.
        Returns documents sorted by relevance score.
        """
        if not self._client.collections.exists(self._collection_name):
            return []

        col = self._client.collections.use(self._collection_name)
        props = list({*self._attributes, self._DOCUMENT_ID_PROPERTY, Field.TEXT_KEY.value})

        where = None
        doc_ids = kwargs.get("document_ids_filter") or []
        if doc_ids:
            where = Filter.by_property(self._DOCUMENT_ID_PROPERTY).contains_any(doc_ids)

        top_k = int(kwargs.get("top_k", 4))
        score_threshold = float(kwargs.get("score_threshold") or 0.0)

        res = col.query.near_vector(
            near_vector=query_vector,
            limit=top_k,
            return_properties=props,
            return_metadata=MetadataQuery(distance=True),
            include_vector=False,
            filters=where,
            target_vector="default",
        )

        docs: list[Document] = []
        for obj in res.objects:
            properties = dict(obj.properties or {})
            text = properties.pop(Field.TEXT_KEY.value, "")
            if obj.metadata and obj.metadata.distance is not None:
                distance = obj.metadata.distance
            else:
                distance = 1.0
            score = 1.0 - distance

            if score > score_threshold:
                properties["score"] = score
                docs.append(Document(page_content=text, metadata=properties))

        docs.sort(key=lambda d: d.metadata.get("score", 0.0), reverse=True)
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """
        Performs BM25 full-text search on document content.

        Filters by document IDs if provided and returns matching documents with vectors.
        """
        if not self._client.collections.exists(self._collection_name):
            return []

        col = self._client.collections.use(self._collection_name)
        props = list({*self._attributes, Field.TEXT_KEY.value})

        where = None
        doc_ids = kwargs.get("document_ids_filter") or []
        if doc_ids:
            where = Filter.by_property(self._DOCUMENT_ID_PROPERTY).contains_any(doc_ids)

        top_k = int(kwargs.get("top_k", 4))

        res = col.query.bm25(
            query=query,
            query_properties=[Field.TEXT_KEY.value],
            limit=top_k,
            return_properties=props,
            include_vector=True,
            filters=where,
        )

        docs: list[Document] = []
        for obj in res.objects:
            properties = dict(obj.properties or {})
            text = properties.pop(Field.TEXT_KEY.value, "")

            vec = obj.vector
            if isinstance(vec, dict):
                vec = vec.get("default") or next(iter(vec.values()), None)

            docs.append(Document(page_content=text, vector=vec, metadata=properties))
        return docs

    def _json_serializable(self, value: Any) -> Any:
        """Converts values to JSON-serializable format, handling datetime objects."""
        if isinstance(value, datetime.datetime):
            return value.isoformat()
        return value


class WeaviateVectorFactory(AbstractVectorFactory):
    """Factory class for creating WeaviateVector instances."""

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> WeaviateVector:
        """
        Initializes a WeaviateVector instance for the given dataset.

        Uses existing collection name from dataset index structure or generates a new one.
        Updates dataset index structure if not already set.
        """
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.WEAVIATE, collection_name))
        return WeaviateVector(
            collection_name=collection_name,
            config=WeaviateConfig(
                endpoint=dify_config.WEAVIATE_ENDPOINT or "",
                grpc_endpoint=dify_config.WEAVIATE_GRPC_ENDPOINT or "",
                api_key=dify_config.WEAVIATE_API_KEY,
                batch_size=dify_config.WEAVIATE_BATCH_SIZE,
            ),
            attributes=attributes,
        )

```

### api/core/rag/datasource/vdb/weaviate/__init__.py
```py

```

### api/core/rag/datasource/vdb/elasticsearch/__init__.py
```py

```

### api/core/rag/datasource/vdb/elasticsearch/elasticsearch_vector.py
```py
import json
import logging
import math
from typing import Any, cast
from urllib.parse import urlparse

from elasticsearch import ConnectionError as ElasticsearchConnectionError
from elasticsearch import Elasticsearch
from flask import current_app
from packaging.version import parse as parse_version
from pydantic import BaseModel, model_validator

from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class ElasticSearchConfig(BaseModel):
    # Regular Elasticsearch config
    host: str | None = None
    port: int | None = None
    username: str | None = None
    password: str | None = None

    # Elastic Cloud specific config
    cloud_url: str | None = None  # Cloud URL for Elasticsearch Cloud
    api_key: str | None = None

    # Common config
    use_cloud: bool = False
    ca_certs: str | None = None
    verify_certs: bool = False
    request_timeout: int = 100000
    retry_on_timeout: bool = True
    max_retries: int = 10000

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        use_cloud = values.get("use_cloud", False)
        cloud_url = values.get("cloud_url")

        if use_cloud:
            # Cloud configuration validation - requires cloud_url and api_key
            if not cloud_url:
                raise ValueError("cloud_url is required for Elastic Cloud")

            api_key = values.get("api_key")
            if not api_key:
                raise ValueError("api_key is required for Elastic Cloud")

        else:
            # Regular Elasticsearch validation
            if not values.get("host"):
                raise ValueError("config HOST is required for regular Elasticsearch")
            if not values.get("port"):
                raise ValueError("config PORT is required for regular Elasticsearch")
            if not values.get("username"):
                raise ValueError("config USERNAME is required for regular Elasticsearch")
            if not values.get("password"):
                raise ValueError("config PASSWORD is required for regular Elasticsearch")

        return values


class ElasticSearchVector(BaseVector):
    def __init__(self, index_name: str, config: ElasticSearchConfig, attributes: list):
        super().__init__(index_name.lower())
        self._client = self._init_client(config)
        self._version = self._get_version()
        self._check_version()
        self._attributes = attributes

    def _init_client(self, config: ElasticSearchConfig) -> Elasticsearch:
        """
        Initialize Elasticsearch client for both regular Elasticsearch and Elastic Cloud.
        """
        try:
            # Check if using Elastic Cloud
            client_config: dict[str, Any]
            if config.use_cloud and config.cloud_url:
                client_config = {
                    "request_timeout": config.request_timeout,
                    "retry_on_timeout": config.retry_on_timeout,
                    "max_retries": config.max_retries,
                    "verify_certs": config.verify_certs,
                }

                # Parse cloud URL and configure hosts
                parsed_url = urlparse(config.cloud_url)
                host = f"{parsed_url.scheme}://{parsed_url.hostname}"
                if parsed_url.port:
                    host += f":{parsed_url.port}"

                client_config["hosts"] = [host]

                # API key authentication for cloud
                client_config["api_key"] = config.api_key

                # SSL settings
                if config.ca_certs:
                    client_config["ca_certs"] = config.ca_certs

            else:
                # Regular Elasticsearch configuration
                parsed_url = urlparse(config.host or "")
                if parsed_url.scheme in {"http", "https"}:
                    hosts = f"{config.host}:{config.port}"
                    use_https = parsed_url.scheme == "https"
                else:
                    hosts = f"http://{config.host}:{config.port}"
                    use_https = False

                client_config = {
                    "hosts": [hosts],
                    "basic_auth": (config.username, config.password),
                    "request_timeout": config.request_timeout,
                    "retry_on_timeout": config.retry_on_timeout,
                    "max_retries": config.max_retries,
                }

                # Only add SSL settings if using HTTPS
                if use_https:
                    client_config["verify_certs"] = config.verify_certs
                    if config.ca_certs:
                        client_config["ca_certs"] = config.ca_certs

            client = Elasticsearch(**client_config)

            # Test connection
            if not client.ping():
                raise ConnectionError("Failed to connect to Elasticsearch")

        except ElasticsearchConnectionError as e:
            raise ConnectionError(f"Vector database connection error: {str(e)}")
        except Exception as e:
            raise ConnectionError(f"Elasticsearch client initialization failed: {str(e)}")

        return client

    def _get_version(self) -> str:
        info = self._client.info()
        # remove any suffix like "-SNAPSHOT" from the version string
        return cast(str, info["version"]["number"]).split("-")[0]

    def _check_version(self):
        if parse_version(self._version) < parse_version("8.0.0"):
            raise ValueError("Elasticsearch vector database version must be greater than 8.0.0")

    def get_type(self) -> str:
        return VectorType.ELASTICSEARCH

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)
        for i in range(len(documents)):
            self._client.index(
                index=self._collection_name,
                id=uuids[i],
                document={
                    Field.CONTENT_KEY: documents[i].page_content,
                    Field.VECTOR: embeddings[i] or None,
                    Field.METADATA_KEY: documents[i].metadata or {},
                },
            )
        self._client.indices.refresh(index=self._collection_name)
        return uuids

    def text_exists(self, id: str) -> bool:
        return bool(self._client.exists(index=self._collection_name, id=id))

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        for id in ids:
            self._client.delete(index=self._collection_name, id=id)

    def delete_by_metadata_field(self, key: str, value: str):
        query_str = {"query": {"match": {f"metadata.{key}": f"{value}"}}}
        results = self._client.search(index=self._collection_name, body=query_str)
        ids = [hit["_id"] for hit in results["hits"]["hits"]]
        if ids:
            self.delete_by_ids(ids)

    def delete(self):
        self._client.indices.delete(index=self._collection_name)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        num_candidates = math.ceil(top_k * 1.5)
        knn = {"field": Field.VECTOR, "query_vector": query_vector, "k": top_k, "num_candidates": num_candidates}
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            knn["filter"] = {"terms": {"metadata.document_id": document_ids_filter}}

        results = self._client.search(index=self._collection_name, knn=knn, size=top_k)

        docs_and_scores = []
        for hit in results["hits"]["hits"]:
            docs_and_scores.append(
                (
                    Document(
                        page_content=hit["_source"][Field.CONTENT_KEY],
                        vector=hit["_source"][Field.VECTOR],
                        metadata=hit["_source"][Field.METADATA_KEY],
                    ),
                    hit["_score"],
                )
            )

        docs = []
        for doc, score in docs_and_scores:
            score_threshold = float(kwargs.get("score_threshold") or 0.0)
            if score >= score_threshold:
                if doc.metadata is not None:
                    doc.metadata["score"] = score
                    docs.append(doc)

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        query_str: dict[str, Any] = {"match": {Field.CONTENT_KEY: query}}
        document_ids_filter = kwargs.get("document_ids_filter")

        if document_ids_filter:
            query_str = {
                "bool": {
                    "must": {"match": {Field.CONTENT_KEY: query}},
                    "filter": {"terms": {"metadata.document_id": document_ids_filter}},
                }
            }

        results = self._client.search(index=self._collection_name, query=query_str, size=kwargs.get("top_k", 4))
        docs = []
        for hit in results["hits"]["hits"]:
            docs.append(
                Document(
                    page_content=hit["_source"][Field.CONTENT_KEY],
                    vector=hit["_source"][Field.VECTOR],
                    metadata=hit["_source"][Field.METADATA_KEY],
                )
            )

        return docs

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        metadatas = [d.metadata if d.metadata is not None else {} for d in texts]
        self.create_collection(embeddings, metadatas)
        self.add_texts(texts, embeddings, **kwargs)

    def create_collection(
        self,
        embeddings: list[list[float]],
        metadatas: list[dict[Any, Any]] | None = None,
        index_params: dict | None = None,
    ):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name)
                return

            if not self._client.indices.exists(index=self._collection_name):
                dim = len(embeddings[0])
                mappings = {
                    "properties": {
                        Field.CONTENT_KEY: {"type": "text"},
                        Field.VECTOR: {  # Make sure the dimension is correct here
                            "type": "dense_vector",
                            "dims": dim,
                            "index": True,
                            "similarity": "cosine",
                        },
                        Field.METADATA_KEY: {
                            "type": "object",
                            "properties": {
                                "doc_id": {"type": "keyword"},  # Map doc_id to keyword type
                                "document_id": {"type": "keyword"},  # Map doc_id to keyword type
                            },
                        },
                    }
                }

                self._client.indices.create(index=self._collection_name, mappings=mappings)
                logger.info("Created index %s with dimension %s", self._collection_name, dim)
            else:
                logger.info("Collection %s already exists.", self._collection_name)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class ElasticSearchVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> ElasticSearchVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.ELASTICSEARCH, collection_name))

        config = current_app.config

        # Check if ELASTICSEARCH_USE_CLOUD is explicitly set to false (boolean)
        use_cloud_env = config.get("ELASTICSEARCH_USE_CLOUD", False)

        if use_cloud_env is False:
            # Use regular Elasticsearch with config values
            config_dict = {
                "use_cloud": False,
                "host": config.get("ELASTICSEARCH_HOST", "elasticsearch"),
                "port": config.get("ELASTICSEARCH_PORT", 9200),
                "username": config.get("ELASTICSEARCH_USERNAME", "elastic"),
                "password": config.get("ELASTICSEARCH_PASSWORD", "elastic"),
            }
        else:
            # Check for cloud configuration
            cloud_url = config.get("ELASTICSEARCH_CLOUD_URL")
            if cloud_url:
                config_dict = {
                    "use_cloud": True,
                    "cloud_url": cloud_url,
                    "api_key": config.get("ELASTICSEARCH_API_KEY"),
                }
            else:
                # Fallback to regular Elasticsearch
                config_dict = {
                    "use_cloud": False,
                    "host": config.get("ELASTICSEARCH_HOST", "localhost"),
                    "port": config.get("ELASTICSEARCH_PORT", 9200),
                    "username": config.get("ELASTICSEARCH_USERNAME", "elastic"),
                    "password": config.get("ELASTICSEARCH_PASSWORD", ""),
                }

        # Common configuration
        config_dict.update(
            {
                "ca_certs": str(config.get("ELASTICSEARCH_CA_CERTS")) if config.get("ELASTICSEARCH_CA_CERTS") else None,
                "verify_certs": bool(config.get("ELASTICSEARCH_VERIFY_CERTS", False)),
                "request_timeout": int(config.get("ELASTICSEARCH_REQUEST_TIMEOUT", 100000)),
                "retry_on_timeout": bool(config.get("ELASTICSEARCH_RETRY_ON_TIMEOUT", True)),
                "max_retries": int(config.get("ELASTICSEARCH_MAX_RETRIES", 10000)),
            }
        )

        return ElasticSearchVector(
            index_name=collection_name,
            config=ElasticSearchConfig(**config_dict),
            attributes=[],
        )

```

### api/core/rag/datasource/vdb/elasticsearch/elasticsearch_ja_vector.py
```py
import json
import logging
from typing import Any

from flask import current_app

from core.rag.datasource.vdb.elasticsearch.elasticsearch_vector import (
    ElasticSearchConfig,
    ElasticSearchVector,
    ElasticSearchVectorFactory,
)
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class ElasticSearchJaVector(ElasticSearchVector):
    def create_collection(
        self,
        embeddings: list[list[float]],
        metadatas: list[dict[Any, Any]] | None = None,
        index_params: dict | None = None,
    ):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                logger.info("Collection %s already exists.", self._collection_name)
                return

            if not self._client.indices.exists(index=self._collection_name):
                dim = len(embeddings[0])
                settings = {
                    "analysis": {
                        "analyzer": {
                            "ja_analyzer": {
                                "type": "custom",
                                "char_filter": [
                                    "icu_normalizer",
                                    "kuromoji_iteration_mark",
                                ],
                                "tokenizer": "kuromoji_tokenizer",
                                "filter": [
                                    "kuromoji_baseform",
                                    "kuromoji_part_of_speech",
                                    "ja_stop",
                                    "kuromoji_number",
                                    "kuromoji_stemmer",
                                ],
                            }
                        }
                    }
                }
                mappings = {
                    "properties": {
                        Field.CONTENT_KEY: {
                            "type": "text",
                            "analyzer": "ja_analyzer",
                            "search_analyzer": "ja_analyzer",
                        },
                        Field.VECTOR: {  # Make sure the dimension is correct here
                            "type": "dense_vector",
                            "dims": dim,
                            "index": True,
                            "similarity": "cosine",
                        },
                        Field.METADATA_KEY: {
                            "type": "object",
                            "properties": {
                                "doc_id": {"type": "keyword"}  # Map doc_id to keyword type
                            },
                        },
                    }
                }
                self._client.indices.create(index=self._collection_name, settings=settings, mappings=mappings)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)


class ElasticSearchJaVectorFactory(ElasticSearchVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> ElasticSearchJaVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.ELASTICSEARCH, collection_name))

        config = current_app.config
        return ElasticSearchJaVector(
            index_name=collection_name,
            config=ElasticSearchConfig(
                host=config.get("ELASTICSEARCH_HOST", "localhost"),
                port=config.get("ELASTICSEARCH_PORT", 9200),
                username=config.get("ELASTICSEARCH_USERNAME", ""),
                password=config.get("ELASTICSEARCH_PASSWORD", ""),
            ),
            attributes=[],
        )

```

### api/core/rag/datasource/vdb/milvus/__init__.py
```py

```

### api/core/rag/datasource/vdb/milvus/milvus_vector.py
```py
import json
import logging
from typing import Any

from packaging import version
from pydantic import BaseModel, model_validator
from pymilvus import MilvusClient, MilvusException  # type: ignore
from pymilvus.milvus_client import IndexParams  # type: ignore

from configs import dify_config
from core.rag.datasource.vdb.field import Field
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class MilvusConfig(BaseModel):
    """
    Configuration class for Milvus connection.
    """

    uri: str  # Milvus server URI
    token: str | None = None  # Optional token for authentication
    user: str | None = None  # Username for authentication
    password: str | None = None  # Password for authentication
    batch_size: int = 100  # Batch size for operations
    database: str = "default"  # Database name
    enable_hybrid_search: bool = False  # Flag to enable hybrid search
    analyzer_params: str | None = None  # Analyzer params

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        """
        Validate the configuration values.
        Raises ValueError if required fields are missing.
        """
        if not values.get("uri"):
            raise ValueError("config MILVUS_URI is required")
        if not values.get("token"):
            if not values.get("user"):
                raise ValueError("config MILVUS_USER is required")
            if not values.get("password"):
                raise ValueError("config MILVUS_PASSWORD is required")
        return values

    def to_milvus_params(self):
        """
        Convert the configuration to a dictionary of Milvus connection parameters.
        """
        return {
            "uri": self.uri,
            "token": self.token,
            "user": self.user,
            "password": self.password,
            "db_name": self.database,
            "analyzer_params": self.analyzer_params,
        }


class MilvusVector(BaseVector):
    """
    Milvus vector storage implementation.
    """

    def __init__(self, collection_name: str, config: MilvusConfig):
        super().__init__(collection_name)
        self._client_config = config
        self._client = self._init_client(config)
        self._consistency_level = "Session"  # Consistency level for Milvus operations
        self._fields: list[str] = []  # List of fields in the collection
        if self._client.has_collection(collection_name):
            self._load_collection_fields()
        self._hybrid_search_enabled = self._check_hybrid_search_support()  # Check if hybrid search is supported

    def _load_collection_fields(self, fields: list[str] | None = None):
        if fields is None:
            # Load collection fields from remote server
            collection_info = self._client.describe_collection(self._collection_name)
            fields = [field["name"] for field in collection_info["fields"]]
        # Since primary field is auto-id, no need to track it
        self._fields = [f for f in fields if f != Field.PRIMARY_KEY]

    def _check_hybrid_search_support(self) -> bool:
        """
        Check if the current Milvus version supports hybrid search.
        Returns True if the version is >= 2.5.0, otherwise False.
        """
        if not self._client_config.enable_hybrid_search:
            return False

        try:
            milvus_version = self._client.get_server_version()
            # Check if it's Zilliz Cloud - it supports full-text search with Milvus 2.5 compatibility
            if "Zilliz Cloud" in milvus_version:
                return True
            # For standard Milvus installations, check version number
            return version.parse(milvus_version) >= version.parse("2.5.0")
        except Exception as e:
            logger.warning("Failed to check Milvus version: %s. Disabling hybrid search.", str(e))
            return False

    def get_type(self) -> str:
        """
        Get the type of vector storage (Milvus).
        """
        return VectorType.MILVUS

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        """
        Create a collection and add texts with embeddings.
        """
        index_params = {"metric_type": "IP", "index_type": "HNSW", "params": {"M": 8, "efConstruction": 64}}
        metadatas = [d.metadata if d.metadata is not None else {} for d in texts]
        self.create_collection(embeddings, metadatas, index_params)
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        """
        Add texts and their embeddings to the collection.
        """
        insert_dict_list = []
        for i in range(len(documents)):
            insert_dict = {
                # Do not need to insert the sparse_vector field separately, as the text_bm25_emb
                # function will automatically convert the native text into a sparse vector for us.
                Field.CONTENT_KEY: documents[i].page_content,
                Field.VECTOR: embeddings[i],
                Field.METADATA_KEY: documents[i].metadata,
            }
            insert_dict_list.append(insert_dict)
        # Total insert count
        total_count = len(insert_dict_list)
        pks: list[str] = []

        for i in range(0, total_count, 1000):
            # Insert into the collection.
            batch_insert_list = insert_dict_list[i : i + 1000]
            try:
                ids = self._client.insert(collection_name=self._collection_name, data=batch_insert_list)
                pks.extend(ids)
            except MilvusException as e:
                logger.exception("Failed to insert batch starting at entity: %s/%s", i, total_count)
                raise e
        return pks

    def get_ids_by_metadata_field(self, key: str, value: str):
        """
        Get document IDs by metadata field key and value.
        """
        result = self._client.query(
            collection_name=self._collection_name, filter=f'metadata["{key}"] == "{value}"', output_fields=["id"]
        )
        if result:
            return [item["id"] for item in result]
        else:
            return None

    def delete_by_metadata_field(self, key: str, value: str):
        """
        Delete documents by metadata field key and value.
        """
        if self._client.has_collection(self._collection_name):
            ids = self.get_ids_by_metadata_field(key, value)
            if ids:
                self._client.delete(collection_name=self._collection_name, pks=ids)

    def delete_by_ids(self, ids: list[str]):
        """
        Delete documents by their IDs.
        """
        if self._client.has_collection(self._collection_name):
            result = self._client.query(
                collection_name=self._collection_name, filter=f'metadata["doc_id"] in {ids}', output_fields=["id"]
            )
            if result:
                ids = [item["id"] for item in result]
                self._client.delete(collection_name=self._collection_name, pks=ids)

    def delete(self):
        """
        Delete the entire collection.
        """
        if self._client.has_collection(self._collection_name):
            self._client.drop_collection(self._collection_name, None)

    def text_exists(self, id: str) -> bool:
        """
        Check if a text with the given ID exists in the collection.
        """
        if not self._client.has_collection(self._collection_name):
            return False

        result = self._client.query(
            collection_name=self._collection_name, filter=f'metadata["doc_id"] == "{id}"', output_fields=["id"]
        )

        return len(result) > 0

    def field_exists(self, field: str) -> bool:
        """
        Check if a field exists in the collection.
        """
        return field in self._fields

    def _process_search_results(
        self, results: list[Any], output_fields: list[str], score_threshold: float = 0.0
    ) -> list[Document]:
        """
        Common method to process search results

        :param results: Search results
        :param output_fields: Fields to be output
        :param score_threshold: Score threshold for filtering
        :return: List of documents
        """
        docs = []
        for result in results[0]:
            metadata = result["entity"].get(output_fields[1], {})
            metadata["score"] = result["distance"]

            if result["distance"] > score_threshold:
                doc = Document(page_content=result["entity"].get(output_fields[0], ""), metadata=metadata)
                docs.append(doc)

        return docs

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        """
        Search for documents by vector similarity.
        """
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = ""
        if document_ids_filter:
            document_ids = ", ".join(f'"{id}"' for id in document_ids_filter)
            filter = f'metadata["document_id"] in [{document_ids}]'
        results = self._client.search(
            collection_name=self._collection_name,
            data=[query_vector],
            anns_field=Field.VECTOR,
            limit=kwargs.get("top_k", 4),
            output_fields=[Field.CONTENT_KEY, Field.METADATA_KEY],
            filter=filter,
        )

        return self._process_search_results(
            results,
            output_fields=[Field.CONTENT_KEY, Field.METADATA_KEY],
            score_threshold=float(kwargs.get("score_threshold") or 0.0),
        )

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        """
        Search for documents by full-text search (if hybrid search is enabled).
        """
        if not self._hybrid_search_enabled:
            logger.warning(
                "Full-text search is disabled: set MILVUS_ENABLE_HYBRID_SEARCH=true (requires Milvus >= 2.5.0)."
            )
            return []
        if not self.field_exists(Field.SPARSE_VECTOR):
            logger.warning(
                "Full-text search unavailable: collection missing 'sparse_vector' field; "
                "recreate the collection after enabling MILVUS_ENABLE_HYBRID_SEARCH to add BM25 sparse index."
            )
            return []
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            filter = f'metadata["document_id"] in [{document_ids}]'

        results = self._client.search(
            collection_name=self._collection_name,
            data=[query],
            anns_field=Field.SPARSE_VECTOR,
            limit=kwargs.get("top_k", 4),
            output_fields=[Field.CONTENT_KEY, Field.METADATA_KEY],
            filter=filter,
        )

        return self._process_search_results(
            results,
            output_fields=[Field.CONTENT_KEY, Field.METADATA_KEY],
            score_threshold=float(kwargs.get("score_threshold") or 0.0),
        )

    def create_collection(
        self, embeddings: list, metadatas: list[dict] | None = None, index_params: dict | None = None
    ):
        """
        Create a new collection in Milvus with the specified schema and index parameters.
        """
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            # Grab the existing collection if it exists
            if not self._client.has_collection(self._collection_name):
                from pymilvus import CollectionSchema, DataType, FieldSchema, Function, FunctionType  # type: ignore
                from pymilvus.orm.types import infer_dtype_bydata  # type: ignore

                # Determine embedding dim
                dim = len(embeddings[0])
                fields = []
                if metadatas:
                    fields.append(FieldSchema(Field.METADATA_KEY, DataType.JSON, max_length=65_535))

                # Create the text field, enable_analyzer will be set True to support milvus automatically
                # transfer text to sparse_vector, reference: https://milvus.io/docs/full-text-search.md
                content_field_kwargs: dict[str, Any] = {
                    "max_length": 65_535,
                    "enable_analyzer": self._hybrid_search_enabled,
                }
                if (
                    self._hybrid_search_enabled
                    and self._client_config.analyzer_params is not None
                    and self._client_config.analyzer_params.strip()
                ):
                    content_field_kwargs["analyzer_params"] = self._client_config.analyzer_params

                fields.append(FieldSchema(Field.CONTENT_KEY, DataType.VARCHAR, **content_field_kwargs))

                # Create the primary key field
                fields.append(FieldSchema(Field.PRIMARY_KEY, DataType.INT64, is_primary=True, auto_id=True))
                # Create the vector field, supports binary or float vectors
                fields.append(FieldSchema(Field.VECTOR, infer_dtype_bydata(embeddings[0]), dim=dim))
                # Create Sparse Vector Index for the collection
                if self._hybrid_search_enabled:
                    fields.append(FieldSchema(Field.SPARSE_VECTOR, DataType.SPARSE_FLOAT_VECTOR))

                schema = CollectionSchema(fields)

                # Create custom function to support text to sparse vector by BM25
                if self._hybrid_search_enabled:
                    bm25_function = Function(
                        name="text_bm25_emb",
                        input_field_names=[Field.CONTENT_KEY],
                        output_field_names=[Field.SPARSE_VECTOR],
                        function_type=FunctionType.BM25,
                    )
                    schema.add_function(bm25_function)

                self._load_collection_fields([f.name for f in schema.fields])

                # Create Index params for the collection
                index_params_obj = IndexParams()
                assert index_params is not None
                index_params_obj.add_index(field_name=Field.VECTOR, **index_params)

                # Create Sparse Vector Index for the collection
                if self._hybrid_search_enabled:
                    index_params_obj.add_index(
                        field_name=Field.SPARSE_VECTOR, index_type="AUTOINDEX", metric_type="BM25"
                    )

                # Create the collection
                self._client.create_collection(
                    collection_name=self._collection_name,
                    schema=schema,
                    index_params=index_params_obj,
                    consistency_level=self._consistency_level,
                )
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def _init_client(self, config: MilvusConfig) -> MilvusClient:
        """
        Initialize and return a Milvus client.
        """
        if config.token:
            client = MilvusClient(uri=config.uri, token=config.token, db_name=config.database)
        else:
            client = MilvusClient(
                uri=config.uri,
                user=config.user or "",
                password=config.password or "",
                db_name=config.database,
            )
        return client


class MilvusVectorFactory(AbstractVectorFactory):
    """
    Factory class for creating MilvusVector instances.
    """

    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> MilvusVector:
        """
        Initialize a MilvusVector instance for the given dataset.
        """
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.MILVUS, collection_name))

        return MilvusVector(
            collection_name=collection_name,
            config=MilvusConfig(
                uri=dify_config.MILVUS_URI or "",
                token=dify_config.MILVUS_TOKEN or "",
                user=dify_config.MILVUS_USER or "",
                password=dify_config.MILVUS_PASSWORD or "",
                database=dify_config.MILVUS_DATABASE or "",
                enable_hybrid_search=dify_config.MILVUS_ENABLE_HYBRID_SEARCH or False,
                analyzer_params=dify_config.MILVUS_ANALYZER_PARAMS or "",
            ),
        )

```

### api/core/rag/datasource/vdb/analyticdb/__init__.py
```py

```

### api/core/rag/datasource/vdb/analyticdb/analyticdb_vector.py
```py
import json
from typing import Any

from configs import dify_config
from core.rag.datasource.vdb.analyticdb.analyticdb_vector_openapi import (
    AnalyticdbVectorOpenAPI,
    AnalyticdbVectorOpenAPIConfig,
)
from core.rag.datasource.vdb.analyticdb.analyticdb_vector_sql import AnalyticdbVectorBySql, AnalyticdbVectorBySqlConfig
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from models.dataset import Dataset


class AnalyticdbVector(BaseVector):
    def __init__(
        self,
        collection_name: str,
        api_config: AnalyticdbVectorOpenAPIConfig | None,
        sql_config: AnalyticdbVectorBySqlConfig | None,
    ):
        super().__init__(collection_name)
        if api_config is not None:
            self.analyticdb_vector: AnalyticdbVectorOpenAPI | AnalyticdbVectorBySql = AnalyticdbVectorOpenAPI(
                collection_name, api_config
            )
        else:
            if sql_config is None:
                raise ValueError("Either api_config or sql_config must be provided")
            self.analyticdb_vector = AnalyticdbVectorBySql(collection_name, sql_config)

    def get_type(self) -> str:
        return VectorType.ANALYTICDB

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        dimension = len(embeddings[0])
        self.analyticdb_vector._create_collection_if_not_exists(dimension)
        self.analyticdb_vector.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        self.analyticdb_vector.add_texts(documents, embeddings)

    def text_exists(self, id: str) -> bool:
        return self.analyticdb_vector.text_exists(id)

    def delete_by_ids(self, ids: list[str]):
        self.analyticdb_vector.delete_by_ids(ids)

    def delete_by_metadata_field(self, key: str, value: str):
        self.analyticdb_vector.delete_by_metadata_field(key, value)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        return self.analyticdb_vector.search_by_vector(query_vector, **kwargs)

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return self.analyticdb_vector.search_by_full_text(query, **kwargs)

    def delete(self):
        self.analyticdb_vector.delete()


class AnalyticdbVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> AnalyticdbVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.ANALYTICDB, collection_name))

        if dify_config.ANALYTICDB_HOST is None:
            # implemented through OpenAPI
            apiConfig = AnalyticdbVectorOpenAPIConfig(
                access_key_id=dify_config.ANALYTICDB_KEY_ID or "",
                access_key_secret=dify_config.ANALYTICDB_KEY_SECRET or "",
                region_id=dify_config.ANALYTICDB_REGION_ID or "",
                instance_id=dify_config.ANALYTICDB_INSTANCE_ID or "",
                account=dify_config.ANALYTICDB_ACCOUNT or "",
                account_password=dify_config.ANALYTICDB_PASSWORD or "",
                namespace=dify_config.ANALYTICDB_NAMESPACE or "",
                namespace_password=dify_config.ANALYTICDB_NAMESPACE_PASSWORD,
            )
            sqlConfig = None
        else:
            # implemented through sql
            sqlConfig = AnalyticdbVectorBySqlConfig(
                host=dify_config.ANALYTICDB_HOST,
                port=dify_config.ANALYTICDB_PORT,
                account=dify_config.ANALYTICDB_ACCOUNT or "",
                account_password=dify_config.ANALYTICDB_PASSWORD or "",
                min_connection=dify_config.ANALYTICDB_MIN_CONNECTION,
                max_connection=dify_config.ANALYTICDB_MAX_CONNECTION,
                namespace=dify_config.ANALYTICDB_NAMESPACE or "",
            )
            apiConfig = None
        return AnalyticdbVector(
            collection_name,
            apiConfig,
            sqlConfig,
        )

```

### api/core/rag/datasource/vdb/analyticdb/analyticdb_vector_openapi.py
```py
import json
from typing import Any

from pydantic import BaseModel, model_validator

_import_err_msg = (
    "`alibabacloud_gpdb20160503` and `alibabacloud_tea_openapi` packages not found, "
    "please run `pip install alibabacloud_gpdb20160503 alibabacloud_tea_openapi`"
)

from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.models.document import Document
from extensions.ext_redis import redis_client


class AnalyticdbVectorOpenAPIConfig(BaseModel):
    access_key_id: str
    access_key_secret: str
    region_id: str
    instance_id: str
    account: str
    account_password: str
    namespace: str = "dify"
    namespace_password: str | None = None
    metrics: str = "cosine"
    read_timeout: int = 60000

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["access_key_id"]:
            raise ValueError("config ANALYTICDB_KEY_ID is required")
        if not values["access_key_secret"]:
            raise ValueError("config ANALYTICDB_KEY_SECRET is required")
        if not values["region_id"]:
            raise ValueError("config ANALYTICDB_REGION_ID is required")
        if not values["instance_id"]:
            raise ValueError("config ANALYTICDB_INSTANCE_ID is required")
        if not values["account"]:
            raise ValueError("config ANALYTICDB_ACCOUNT is required")
        if not values["account_password"]:
            raise ValueError("config ANALYTICDB_PASSWORD is required")
        if not values["namespace_password"]:
            raise ValueError("config ANALYTICDB_NAMESPACE_PASSWORD is required")
        return values

    def to_analyticdb_client_params(self):
        return {
            "access_key_id": self.access_key_id,
            "access_key_secret": self.access_key_secret,
            "region_id": self.region_id,
            "read_timeout": self.read_timeout,
        }


class AnalyticdbVectorOpenAPI:
    def __init__(self, collection_name: str, config: AnalyticdbVectorOpenAPIConfig):
        try:
            from alibabacloud_gpdb20160503.client import Client  # type: ignore
            from alibabacloud_tea_openapi import models as open_api_models  # type: ignore
        except:
            raise ImportError(_import_err_msg)
        self._collection_name = collection_name.lower()
        self.config = config
        self._client_config = open_api_models.Config(user_agent="dify", **config.to_analyticdb_client_params())
        self._client = Client(self._client_config)
        self._initialize()

    def _initialize(self):
        cache_key = f"vector_initialize_{self.config.instance_id}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            database_exist_cache_key = f"vector_initialize_{self.config.instance_id}"
            if redis_client.get(database_exist_cache_key):
                return
            self._initialize_vector_database()
            self._create_namespace_if_not_exists()
            redis_client.set(database_exist_cache_key, 1, ex=3600)

    def _initialize_vector_database(self):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models  # type: ignore

        request = gpdb_20160503_models.InitVectorDatabaseRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            manager_account=self.config.account,
            manager_account_password=self.config.account_password,
        )
        self._client.init_vector_database(request)

    def _create_namespace_if_not_exists(self):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models
        from Tea.exceptions import TeaException  # type: ignore

        try:
            request = gpdb_20160503_models.DescribeNamespaceRequest(
                dbinstance_id=self.config.instance_id,
                region_id=self.config.region_id,
                namespace=self.config.namespace,
                manager_account=self.config.account,
                manager_account_password=self.config.account_password,
            )
            self._client.describe_namespace(request)
        except TeaException as e:
            if e.statusCode == 404:
                request = gpdb_20160503_models.CreateNamespaceRequest(
                    dbinstance_id=self.config.instance_id,
                    region_id=self.config.region_id,
                    manager_account=self.config.account,
                    manager_account_password=self.config.account_password,
                    namespace=self.config.namespace,
                    namespace_password=self.config.namespace_password,
                )
                self._client.create_namespace(request)
            else:
                raise ValueError(f"failed to create namespace {self.config.namespace}: {e}")

    def _create_collection_if_not_exists(self, embedding_dimension: int):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models
        from Tea.exceptions import TeaException

        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            try:
                request = gpdb_20160503_models.DescribeCollectionRequest(
                    dbinstance_id=self.config.instance_id,
                    region_id=self.config.region_id,
                    namespace=self.config.namespace,
                    namespace_password=self.config.namespace_password,
                    collection=self._collection_name,
                )
                self._client.describe_collection(request)
            except TeaException as e:
                if e.statusCode == 404:
                    metadata = '{"ref_doc_id":"text","page_content":"text","metadata_":"jsonb"}'
                    full_text_retrieval_fields = "page_content"
                    request = gpdb_20160503_models.CreateCollectionRequest(
                        dbinstance_id=self.config.instance_id,
                        region_id=self.config.region_id,
                        manager_account=self.config.account,
                        manager_account_password=self.config.account_password,
                        namespace=self.config.namespace,
                        collection=self._collection_name,
                        dimension=embedding_dimension,
                        metrics=self.config.metrics,
                        metadata=metadata,
                        full_text_retrieval_fields=full_text_retrieval_fields,
                    )
                    self._client.create_collection(request)
                else:
                    raise ValueError(f"failed to create collection {self._collection_name}: {e}")
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        rows: list[gpdb_20160503_models.UpsertCollectionDataRequestRows] = []
        for doc, embedding in zip(documents, embeddings, strict=True):
            if doc.metadata is not None:
                metadata = {
                    "ref_doc_id": doc.metadata["doc_id"],
                    "page_content": doc.page_content,
                    "metadata_": json.dumps(doc.metadata),
                }
                rows.append(
                    gpdb_20160503_models.UpsertCollectionDataRequestRows(
                        vector=embedding,
                        metadata=metadata,
                    )
                )
        request = gpdb_20160503_models.UpsertCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            rows=rows,
        )
        self._client.upsert_collection_data(request)

    def text_exists(self, id: str) -> bool:
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        request = gpdb_20160503_models.QueryCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            metrics=self.config.metrics,
            include_values=True,
            vector=None,
            content=None,
            top_k=1,
            filter=f"ref_doc_id='{id}'",
        )
        response = self._client.query_collection_data(request)
        return len(response.body.matches.match) > 0

    def delete_by_ids(self, ids: list[str]):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        ids_str = ",".join(f"'{id}'" for id in ids)
        ids_str = f"({ids_str})"
        request = gpdb_20160503_models.DeleteCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            collection_data=None,
            collection_data_filter=f"ref_doc_id IN {ids_str}",
        )
        self._client.delete_collection_data(request)

    def delete_by_metadata_field(self, key: str, value: str):
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        request = gpdb_20160503_models.DeleteCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            collection_data=None,
            collection_data_filter=f"metadata_ ->> '{key}' = '{value}'",
        )
        self._client.delete_collection_data(request)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause += f"metadata_->>'document_id' IN ({document_ids})"

        score_threshold = kwargs.get("score_threshold") or 0.0
        request = gpdb_20160503_models.QueryCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            include_values=kwargs.pop("include_values", True),
            metrics=self.config.metrics,
            vector=query_vector,
            content=None,
            top_k=kwargs.get("top_k", 4),
            filter=where_clause,
        )
        response = self._client.query_collection_data(request)
        documents = []
        for match in response.body.matches.match:
            if match.score >= score_threshold:
                metadata = parse_metadata_json(match.metadata.get("metadata_"))
                metadata["score"] = match.score
                doc = Document(
                    page_content=match.metadata.get("page_content"),
                    vector=match.values.value,
                    metadata=metadata,
                )
                documents.append(doc)
        documents = sorted(documents, key=lambda x: x.metadata["score"] if x.metadata else 0, reverse=True)
        return documents

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause += f"metadata_->>'document_id' IN ({document_ids})"
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        request = gpdb_20160503_models.QueryCollectionDataRequest(
            dbinstance_id=self.config.instance_id,
            region_id=self.config.region_id,
            namespace=self.config.namespace,
            namespace_password=self.config.namespace_password,
            collection=self._collection_name,
            include_values=kwargs.pop("include_values", True),
            metrics=self.config.metrics,
            vector=None,
            content=query,
            top_k=kwargs.get("top_k", 4),
            filter=where_clause,
        )
        response = self._client.query_collection_data(request)
        documents = []
        for match in response.body.matches.match:
            if match.score >= score_threshold:
                metadata = parse_metadata_json(match.metadata.get("metadata_"))
                metadata["score"] = match.score
                doc = Document(
                    page_content=match.metadata.get("page_content"),
                    vector=match.values.value,
                    metadata=metadata,
                )
                documents.append(doc)
        documents = sorted(documents, key=lambda x: x.metadata["score"] if x.metadata else 0, reverse=True)
        return documents

    def delete(self):
        try:
            from alibabacloud_gpdb20160503 import models as gpdb_20160503_models

            request = gpdb_20160503_models.DeleteCollectionRequest(
                collection=self._collection_name,
                dbinstance_id=self.config.instance_id,
                namespace=self.config.namespace,
                namespace_password=self.config.namespace_password,
                region_id=self.config.region_id,
            )
            self._client.delete_collection(request)
        except Exception as e:
            raise e

```

### api/core/rag/datasource/vdb/analyticdb/analyticdb_vector_sql.py
```py
import json
import uuid
from contextlib import contextmanager
from typing import Any

import psycopg2.extras
import psycopg2.pool
from pydantic import BaseModel, model_validator

from core.rag.models.document import Document
from extensions.ext_redis import redis_client


class AnalyticdbVectorBySqlConfig(BaseModel):
    host: str
    port: int
    account: str
    account_password: str
    min_connection: int
    max_connection: int
    namespace: str = "dify"
    metrics: str = "cosine"

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config ANALYTICDB_HOST is required")
        if not values["port"]:
            raise ValueError("config ANALYTICDB_PORT is required")
        if not values["account"]:
            raise ValueError("config ANALYTICDB_ACCOUNT is required")
        if not values["account_password"]:
            raise ValueError("config ANALYTICDB_PASSWORD is required")
        if not values["min_connection"]:
            raise ValueError("config ANALYTICDB_MIN_CONNECTION is required")
        if not values["max_connection"]:
            raise ValueError("config ANALYTICDB_MAX_CONNECTION is required")
        if values["min_connection"] > values["max_connection"]:
            raise ValueError("config ANALYTICDB_MIN_CONNECTION should less than ANALYTICDB_MAX_CONNECTION")
        return values


class AnalyticdbVectorBySql:
    def __init__(self, collection_name: str, config: AnalyticdbVectorBySqlConfig):
        self._collection_name = collection_name.lower()
        self.databaseName = "knowledgebase"
        self.config = config
        self.table_name = f"{self.config.namespace}.{self._collection_name}"
        self.pool = None
        self._initialize()
        if not self.pool:
            self.pool = self._create_connection_pool()

    def _initialize(self):
        cache_key = f"vector_initialize_{self.config.host}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            database_exist_cache_key = f"vector_initialize_{self.config.host}"
            if redis_client.get(database_exist_cache_key):
                return
            self._initialize_vector_database()
            redis_client.set(database_exist_cache_key, 1, ex=3600)

    def _create_connection_pool(self):
        return psycopg2.pool.SimpleConnectionPool(
            self.config.min_connection,
            self.config.max_connection,
            host=self.config.host,
            port=self.config.port,
            user=self.config.account,
            password=self.config.account_password,
            database=self.databaseName,
        )

    @contextmanager
    def _get_cursor(self):
        assert self.pool is not None, "Connection pool is not initialized"
        conn = self.pool.getconn()
        cur = conn.cursor()
        try:
            yield cur
        finally:
            cur.close()
            conn.commit()
            self.pool.putconn(conn)

    def _initialize_vector_database(self):
        conn = psycopg2.connect(
            host=self.config.host,
            port=self.config.port,
            user=self.config.account,
            password=self.config.account_password,
            database="postgres",
        )
        conn.autocommit = True
        cur = conn.cursor()
        try:
            cur.execute(f"CREATE DATABASE {self.databaseName}")
        except Exception as e:
            if "already exists" not in str(e):
                raise e
        finally:
            cur.close()
            conn.close()
        self.pool = self._create_connection_pool()
        with self._get_cursor() as cur:
            conn = cur.connection
            try:
                cur.execute("CREATE EXTENSION IF NOT EXISTS zhparser;")
            except Exception as e:
                conn.rollback()
                raise RuntimeError(
                    "Failed to create zhparser extension. Please ensure it is available in your AnalyticDB."
                ) from e
            try:
                cur.execute("CREATE TEXT SEARCH CONFIGURATION zh_cn (PARSER = zhparser)")
                cur.execute("ALTER TEXT SEARCH CONFIGURATION zh_cn ADD MAPPING FOR n,v,a,i,e,l,x WITH simple")
            except Exception as e:
                conn.rollback()
                if "already exists" not in str(e):
                    raise e
            cur.execute(
                "CREATE OR REPLACE FUNCTION "
                "public.to_tsquery_from_text(txt text, lang regconfig DEFAULT 'english'::regconfig) "
                "RETURNS tsquery LANGUAGE sql IMMUTABLE STRICT AS $function$ "
                "SELECT to_tsquery(lang, COALESCE(string_agg(split_part(word, ':', 1), ' | '), '')) "
                "FROM (SELECT unnest(string_to_array(to_tsvector(lang, txt)::text, ' ')) AS word) "
                "AS words_only;$function$"
            )
            cur.execute(f"CREATE SCHEMA IF NOT EXISTS {self.config.namespace}")

    def _create_collection_if_not_exists(self, embedding_dimension: int):
        cache_key = f"vector_indexing_{self._collection_name}"
        lock_name = f"{cache_key}_lock"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            with self._get_cursor() as cur:
                cur.execute(
                    f"CREATE TABLE IF NOT EXISTS {self.table_name}("
                    f"id text PRIMARY KEY,"
                    f"vector real[], ref_doc_id text, page_content text, metadata_ jsonb, "
                    f"to_tsvector TSVECTOR"
                    f") WITH (fillfactor=70) DISTRIBUTED BY (id);"
                )
                if embedding_dimension is not None:
                    index_name = f"{self._collection_name}_embedding_idx"
                    try:
                        cur.execute(f"ALTER TABLE {self.table_name} ALTER COLUMN vector SET STORAGE PLAIN")
                        cur.execute(
                            f"CREATE INDEX {index_name} ON {self.table_name} USING ann(vector) "
                            f"WITH(dim='{embedding_dimension}', distancemeasure='{self.config.metrics}', "
                            f"pq_enable=0, external_storage=0)"
                        )
                        cur.execute(f"CREATE INDEX ON {self.table_name} USING gin(to_tsvector)")
                    except Exception as e:
                        if "already exists" not in str(e):
                            raise e
            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        values = []
        id_prefix = str(uuid.uuid4()) + "_"
        sql = f"""
                INSERT INTO {self.table_name}
                (id, ref_doc_id, vector, page_content, metadata_, to_tsvector)
                VALUES (%s, %s, %s, %s, %s, to_tsvector('zh_cn',  %s));
            """
        for i, doc in enumerate(documents):
            if doc.metadata is not None:
                values.append(
                    (
                        id_prefix + str(i),
                        doc.metadata.get("doc_id", str(uuid.uuid4())),
                        embeddings[i],
                        doc.page_content,
                        json.dumps(doc.metadata),
                        doc.page_content,
                    )
                )
        with self._get_cursor() as cur:
            psycopg2.extras.execute_batch(cur, sql, values)

    def text_exists(self, id: str) -> bool:
        with self._get_cursor() as cur:
            cur.execute(f"SELECT id FROM {self.table_name} WHERE ref_doc_id = %s", (id,))
            return cur.fetchone() is not None

    def delete_by_ids(self, ids: list[str]):
        if not ids:
            return
        with self._get_cursor() as cur:
            try:
                cur.execute(f"DELETE FROM {self.table_name} WHERE ref_doc_id = ANY(%s)", (ids,))
            except Exception as e:
                if "does not exist" not in str(e):
                    raise e

    def delete_by_metadata_field(self, key: str, value: str):
        with self._get_cursor() as cur:
            try:
                cur.execute(f"DELETE FROM {self.table_name} WHERE metadata_->>%s = %s", (key, value))
            except Exception as e:
                if "does not exist" not in str(e):
                    raise e

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = "WHERE 1=1"
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause += f"AND metadata_->>'document_id' IN ({document_ids})"
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        with self._get_cursor() as cur:
            query_vector_str = json.dumps(query_vector)
            query_vector_str = "{" + query_vector_str[1:-1] + "}"
            cur.execute(
                f"SELECT t.id AS id, t.vector AS vector, (1.0 - t.score) AS score, "
                f"t.page_content as page_content, t.metadata_ AS metadata_ "
                f"FROM (SELECT id, vector, page_content, metadata_, vector <=> %s AS score "
                f"FROM {self.table_name} {where_clause} ORDER BY score LIMIT {top_k} ) t",
                (query_vector_str,),
            )
            documents = []
            for record in cur:
                _, vector, score, page_content, metadata = record
                if score >= score_threshold:
                    metadata["score"] = score
                    doc = Document(
                        page_content=page_content,
                        vector=vector,
                        metadata=metadata,
                    )
                    documents.append(doc)
        return documents

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        if not isinstance(top_k, int) or top_k <= 0:
            raise ValueError("top_k must be a positive integer")
        document_ids_filter = kwargs.get("document_ids_filter")
        where_clause = ""
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            where_clause += f"AND metadata_->>'document_id' IN ({document_ids})"
        with self._get_cursor() as cur:
            cur.execute(
                f"""SELECT id, vector, page_content, metadata_,
                ts_rank(to_tsvector, to_tsquery_from_text(%s, 'zh_cn'), 32) AS score
                FROM {self.table_name}
                WHERE to_tsvector@@to_tsquery_from_text(%s, 'zh_cn') {where_clause}
                ORDER BY score DESC, id DESC
                LIMIT {top_k}""",
                (f"'{query}'", f"'{query}'"),
            )
            documents = []
            for record in cur:
                _, vector, page_content, metadata, score = record
                metadata["score"] = score
                doc = Document(
                    page_content=page_content,
                    vector=vector,
                    metadata=metadata,
                )
                documents.append(doc)
        return documents

    def delete(self):
        with self._get_cursor() as cur:
            cur.execute(f"DROP TABLE IF EXISTS {self.table_name}")

```

### api/core/rag/datasource/vdb/matrixone/matrixone_vector.py
```py
import json
import logging
import uuid
from collections.abc import Callable
from functools import wraps
from typing import Any, Concatenate

from mo_vector.client import MoVectorClient  # type: ignore
from pydantic import BaseModel, model_validator

from configs import dify_config
from core.rag.datasource.vdb.field import parse_metadata_json
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


def ensure_client[T: MatrixoneVector, **P, R](
    func: Callable[Concatenate[T, P], R],
) -> Callable[Concatenate[T, P], R]:
    @wraps(func)
    def wrapper(self: T, *args: P.args, **kwargs: P.kwargs) -> R:
        if self.client is None:
            self.client = self._get_client(None, False)
        return func(self, *args, **kwargs)

    return wrapper


class MatrixoneConfig(BaseModel):
    host: str = "localhost"
    port: int = 6001
    user: str = "dump"
    password: str = "111"
    database: str = "dify"
    metric: str = "l2"

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["host"]:
            raise ValueError("config host is required")
        if not values["port"]:
            raise ValueError("config port is required")
        if not values["user"]:
            raise ValueError("config user is required")
        if not values["password"]:
            raise ValueError("config password is required")
        if not values["database"]:
            raise ValueError("config database is required")
        return values


class MatrixoneVector(BaseVector):
    """
    Matrixone vector storage implementation.
    """

    def __init__(self, collection_name: str, config: MatrixoneConfig):
        super().__init__(collection_name)
        self.config = config
        self.collection_name = collection_name.lower()
        self.client = None

    @property
    def collection_name(self):
        return self._collection_name

    @collection_name.setter
    def collection_name(self, value):
        self._collection_name = value

    def get_type(self) -> str:
        return VectorType.MATRIXONE

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        if self.client is None:
            self.client = self._get_client(len(embeddings[0]), True)
        return self.add_texts(texts, embeddings)

    def _get_client(self, dimension: int | None = None, create_table: bool = False) -> MoVectorClient:
        """
        Create a new client for the collection.

        The collection will be created if it doesn't exist.
        """
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            client = MoVectorClient(
                connection_string=f"mysql+pymysql://{self.config.user}:{self.config.password}@{self.config.host}:{self.config.port}/{self.config.database}",
                table_name=self.collection_name,
                vector_dimension=dimension,
                create_table=create_table,
            )
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return client
            try:
                client.create_full_text_index()
                redis_client.set(collection_exist_cache_key, 1, ex=3600)
            except Exception:
                logger.exception("Failed to create full text index")
            return client

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        if self.client is None:
            self.client = self._get_client(len(embeddings[0]), True)
        assert self.client is not None
        ids = []
        for doc in documents:
            if doc.metadata is not None:
                doc_id = doc.metadata.get("doc_id", str(uuid.uuid4()))
                ids.append(doc_id)
        self.client.insert(
            texts=[doc.page_content for doc in documents],
            embeddings=embeddings,
            metadatas=[doc.metadata for doc in documents],
            ids=ids,
        )
        return ids

    @ensure_client
    def text_exists(self, id: str) -> bool:
        assert self.client is not None
        result = self.client.get(ids=[id])
        return len(result) > 0

    @ensure_client
    def delete_by_ids(self, ids: list[str]):
        assert self.client is not None
        if not ids:
            return
        self.client.delete(ids=ids)

    @ensure_client
    def get_ids_by_metadata_field(self, key: str, value: str):
        assert self.client is not None
        results = self.client.query_by_metadata(filter={key: value})
        return [result.id for result in results]

    @ensure_client
    def delete_by_metadata_field(self, key: str, value: str):
        assert self.client is not None
        self.client.delete(filter={key: value})

    @ensure_client
    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        assert self.client is not None
        top_k = kwargs.get("top_k", 5)
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = None
        if document_ids_filter:
            filter = {"document_id": {"$in": document_ids_filter}}

        results = self.client.query(
            query_vector=query_vector,
            k=top_k,
            filter=filter,
        )

        docs = []
        # TODO: add the score threshold to the query
        for result in results:
            metadata = result.metadata
            docs.append(
                Document(
                    page_content=result.document,
                    metadata=metadata,
                )
            )
        return docs

    @ensure_client
    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        assert self.client is not None
        top_k = kwargs.get("top_k", 5)
        document_ids_filter = kwargs.get("document_ids_filter")
        filter = None
        if document_ids_filter:
            filter = {"document_id": {"$in": document_ids_filter}}
        score_threshold = float(kwargs.get("score_threshold", 0.0))

        results = self.client.full_text_query(
            keywords=[query],
            k=top_k,
            filter=filter,
        )

        docs = []
        for result in results:
            metadata = parse_metadata_json(result.metadata)
            score = 1 - result.distance
            if score >= score_threshold:
                metadata["score"] = score
                docs.append(
                    Document(
                        page_content=result.document,
                        metadata=metadata,
                    )
                )
        return docs

    @ensure_client
    def delete(self):
        assert self.client is not None
        self.client.delete()


class MatrixoneVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> MatrixoneVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.MATRIXONE, collection_name))

        config = MatrixoneConfig(
            host=dify_config.MATRIXONE_HOST or "localhost",
            port=dify_config.MATRIXONE_PORT or 6001,
            user=dify_config.MATRIXONE_USER or "dump",
            password=dify_config.MATRIXONE_PASSWORD or "111",
            database=dify_config.MATRIXONE_DATABASE or "dify",
            metric=dify_config.MATRIXONE_METRIC or "l2",
        )
        return MatrixoneVector(collection_name=collection_name, config=config)

```

### api/core/rag/datasource/vdb/matrixone/__init__.py
```py

```

### api/core/rag/datasource/vdb/vector_type.py
```py
from enum import StrEnum


class VectorType(StrEnum):
    ALIBABACLOUD_MYSQL = "alibabacloud_mysql"
    ANALYTICDB = "analyticdb"
    CHROMA = "chroma"
    MILVUS = "milvus"
    MYSCALE = "myscale"
    PGVECTOR = "pgvector"
    VASTBASE = "vastbase"
    PGVECTO_RS = "pgvecto-rs"

    QDRANT = "qdrant"
    RELYT = "relyt"
    TIDB_VECTOR = "tidb_vector"
    WEAVIATE = "weaviate"
    OPENSEARCH = "opensearch"
    TENCENT = "tencent"
    ORACLE = "oracle"
    ELASTICSEARCH = "elasticsearch"
    ELASTICSEARCH_JA = "elasticsearch-ja"
    LINDORM = "lindorm"
    COUCHBASE = "couchbase"
    BAIDU = "baidu"
    VIKINGDB = "vikingdb"
    UPSTASH = "upstash"
    TIDB_ON_QDRANT = "tidb_on_qdrant"
    OCEANBASE = "oceanbase"
    SEEKDB = "seekdb"
    OPENGAUSS = "opengauss"
    TABLESTORE = "tablestore"
    HUAWEI_CLOUD = "huawei_cloud"
    MATRIXONE = "matrixone"
    CLICKZETTA = "clickzetta"
    IRIS = "iris"
    HOLOGRES = "hologres"

```

### api/core/rag/datasource/vdb/upstash/upstash_vector.py
```py
import json
from typing import Any
from uuid import uuid4

from pydantic import BaseModel, model_validator
from upstash_vector import Index, Vector

from configs import dify_config
from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from models.dataset import Dataset


class UpstashVectorConfig(BaseModel):
    url: str
    token: str

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values["url"]:
            raise ValueError("Upstash URL is required")
        if not values["token"]:
            raise ValueError("Upstash Token is required")
        return values


class UpstashVector(BaseVector):
    def __init__(self, collection_name: str, config: UpstashVectorConfig):
        super().__init__(collection_name)
        self._table_name = collection_name
        self.index = Index(url=config.url, token=config.token)

    def _get_index_dimension(self) -> int:
        index_info = self.index.info()
        if index_info and index_info.dimension:
            return index_info.dimension
        else:
            return 1536

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        self.add_texts(texts, embeddings)

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        vectors = [
            Vector(
                id=str(uuid4()),
                vector=embedding,
                metadata=doc.metadata,
                data=doc.page_content,
            )
            for doc, embedding in zip(documents, embeddings)
        ]
        self.index.upsert(vectors=vectors)

    def text_exists(self, id: str) -> bool:
        response = self.get_ids_by_metadata_field("doc_id", id)
        return len(response) > 0

    def delete_by_ids(self, ids: list[str]):
        item_ids = []
        for doc_id in ids:
            ids = self.get_ids_by_metadata_field("doc_id", doc_id)
            if ids:
                item_ids += ids
        self._delete_by_ids(ids=item_ids)

    def _delete_by_ids(self, ids: list[str]):
        if ids:
            self.index.delete(ids=ids)

    def get_ids_by_metadata_field(self, key: str, value: str) -> list[str]:
        query_result = self.index.query(
            vector=[1.001 * i for i in range(self._get_index_dimension())],
            include_metadata=True,
            top_k=1000,
            filter=f"{key} = '{value}'",
        )
        return [result.id for result in query_result]

    def delete_by_metadata_field(self, key: str, value: str):
        ids = self.get_ids_by_metadata_field(key, value)
        if ids:
            self._delete_by_ids(ids)

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        document_ids_filter = kwargs.get("document_ids_filter")
        if document_ids_filter:
            document_ids = ", ".join(f"'{id}'" for id in document_ids_filter)
            filter = f"document_id in ({document_ids})"
        else:
            filter = ""
        result = self.index.query(
            vector=query_vector,
            top_k=top_k,
            include_metadata=True,
            include_data=True,
            include_vectors=False,
            filter=filter,
        )
        docs = []
        score_threshold = float(kwargs.get("score_threshold") or 0.0)
        for record in result:
            metadata = record.metadata
            text = record.data
            score = record.score
            if metadata is not None and text is not None:
                metadata["score"] = score
                if score >= score_threshold:
                    docs.append(Document(page_content=text, metadata=metadata))
        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        return []

    def delete(self):
        self.index.reset()

    def get_type(self) -> str:
        return VectorType.UPSTASH


class UpstashVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> UpstashVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix.lower()
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id).lower()
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.UPSTASH, collection_name))

        return UpstashVector(
            collection_name=collection_name,
            config=UpstashVectorConfig(
                url=dify_config.UPSTASH_VECTOR_URL or "",
                token=dify_config.UPSTASH_VECTOR_TOKEN or "",
            ),
        )

```

### api/core/rag/datasource/vdb/upstash/__init__.py
```py

```

### api/core/rag/datasource/vdb/couchbase/couchbase_vector.py
```py
import json
import logging
import time
import uuid
from datetime import timedelta
from typing import Any

from couchbase import search  # type: ignore
from couchbase.auth import PasswordAuthenticator  # type: ignore
from couchbase.cluster import Cluster  # type: ignore
from couchbase.management.search import SearchIndex  # type: ignore

# needed for options -- cluster, timeout, SQL++ (N1QL) query, etc.
from couchbase.options import ClusterOptions, SearchOptions  # type: ignore
from couchbase.vector_search import VectorQuery, VectorSearch  # type: ignore
from flask import current_app
from pydantic import BaseModel, model_validator

from core.rag.datasource.vdb.vector_base import BaseVector
from core.rag.datasource.vdb.vector_factory import AbstractVectorFactory
from core.rag.datasource.vdb.vector_type import VectorType
from core.rag.embedding.embedding_base import Embeddings
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset

logger = logging.getLogger(__name__)


class CouchbaseConfig(BaseModel):
    connection_string: str
    user: str
    password: str
    bucket_name: str
    scope_name: str

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        if not values.get("connection_string"):
            raise ValueError("config COUCHBASE_CONNECTION_STRING is required")
        if not values.get("user"):
            raise ValueError("config COUCHBASE_USER is required")
        if not values.get("password"):
            raise ValueError("config COUCHBASE_PASSWORD is required")
        if not values.get("bucket_name"):
            raise ValueError("config COUCHBASE_PASSWORD is required")
        if not values.get("scope_name"):
            raise ValueError("config COUCHBASE_SCOPE_NAME is required")
        return values


class CouchbaseVector(BaseVector):
    def __init__(self, collection_name: str, config: CouchbaseConfig):
        super().__init__(collection_name)
        self._client_config = config

        """Connect to couchbase"""

        auth = PasswordAuthenticator(config.user, config.password)
        options = ClusterOptions(auth)
        self._cluster = Cluster(config.connection_string, options)
        self._bucket = self._cluster.bucket(config.bucket_name)
        self._scope = self._bucket.scope(config.scope_name)
        self._bucket_name = config.bucket_name
        self._scope_name = config.scope_name

        # Wait until the cluster is ready for use.
        self._cluster.wait_until_ready(timedelta(seconds=5))

    def create(self, texts: list[Document], embeddings: list[list[float]], **kwargs):
        index_id = str(uuid.uuid4()).replace("-", "")
        self._create_collection(uuid=index_id, vector_length=len(embeddings[0]))
        self.add_texts(texts, embeddings)

    def _create_collection(self, vector_length: int, uuid: str):
        lock_name = f"vector_indexing_lock_{self._collection_name}"
        with redis_client.lock(lock_name, timeout=20):
            collection_exist_cache_key = f"vector_indexing_{self._collection_name}"
            if redis_client.get(collection_exist_cache_key):
                return
            if self._collection_exists(self._collection_name):
                return
            manager = self._bucket.collections()
            manager.create_collection(self._client_config.scope_name, self._collection_name)

            index_manager = self._scope.search_indexes()

            index_definition = json.loads("""
{
    "type": "fulltext-index",
    "name": "Embeddings._default.Vector_Search",
    "uuid": "26d4db528e78b716",
    "sourceType": "gocbcore",
    "sourceName": "Embeddings",
    "sourceUUID": "2242e4a25b4decd6650c9c7b3afa1dbf",
    "planParams": {
      "maxPartitionsPerPIndex": 1024,
      "indexPartitions": 1
    },
    "params": {
      "doc_config": {
        "docid_prefix_delim": "",
        "docid_regexp": "",
        "mode": "scope.collection.type_field",
        "type_field": "type"
      },
      "mapping": {
        "analysis": { },
        "default_analyzer": "standard",
        "default_datetime_parser": "dateTimeOptional",
        "default_field": "_all",
        "default_mapping": {
          "dynamic": true,
          "enabled": true
        },
        "default_type": "_default",
        "docvalues_dynamic": false,
        "index_dynamic": true,
        "store_dynamic": true,
        "type_field": "_type",
        "types": {
          "collection_name": {
            "dynamic": true,
            "enabled": true,
            "properties": {
              "embedding": {
                "dynamic": false,
                "enabled": true,
                "fields": [
                  {
                    "dims": 1536,
                    "index": true,
                    "name": "embedding",
                    "similarity": "dot_product",
                    "type": "vector",
                    "vector_index_optimized_for": "recall"
                  }
                ]
              },
              "metadata": {
                "dynamic": true,
                "enabled": true
              },
              "text": {
                "dynamic": false,
                "enabled": true,
                "fields": [
                  {
                    "index": true,
                    "name": "text",
                    "store": true,
                    "type": "text"
                  }
                ]
              }
            }
          }
        }
      },
      "store": {
        "indexType": "scorch",
        "segmentVersion": 16
      }
    },
    "sourceParams": { }
  }
""")
            index_definition["name"] = self._collection_name + "_search"
            index_definition["uuid"] = uuid
            index_definition["params"]["mapping"]["types"]["collection_name"]["properties"]["embedding"]["fields"][0][
                "dims"
            ] = vector_length
            index_definition["params"]["mapping"]["types"][self._scope_name + "." + self._collection_name] = (
                index_definition["params"]["mapping"]["types"].pop("collection_name")
            )
            time.sleep(2)
            index_manager.upsert_index(
                SearchIndex(
                    index_definition["name"],
                    params=index_definition["params"],
                    source_name=self._bucket_name,
                ),
            )
            time.sleep(1)

            redis_client.set(collection_exist_cache_key, 1, ex=3600)

    def _collection_exists(self, name: str):
        scope_collection_map: dict[str, Any] = {}

        # Get a list of all scopes in the bucket
        for scope in self._bucket.collections().get_all_scopes():
            scope_collection_map[scope.name] = []

            # Get a list of all the collections in the scope
            for collection in scope.collections:
                scope_collection_map[scope.name].append(collection.name)

        # Check if the collection exists in the scope
        return self._collection_name in scope_collection_map[self._scope_name]

    def get_type(self) -> str:
        return VectorType.COUCHBASE

    def add_texts(self, documents: list[Document], embeddings: list[list[float]], **kwargs):
        uuids = self._get_uuids(documents)
        texts = [d.page_content for d in documents]
        metadatas = [d.metadata for d in documents]

        doc_ids = []

        documents_to_insert = [
            {"text": text, "embedding": vector, "metadata": metadata}
            for _, text, vector, metadata in zip(uuids, texts, embeddings, metadatas)
        ]
        for doc, id in zip(documents_to_insert, uuids):
            _ = self._scope.collection(self._collection_name).upsert(id, doc)

        doc_ids.extend(uuids)

        return doc_ids

    def text_exists(self, id: str) -> bool:
        # Use a parameterized query for safety and correctness
        query = f"""
                SELECT COUNT(1) AS count FROM
                `{self._client_config.bucket_name}`.{self._client_config.scope_name}.{self._collection_name}
                WHERE META().id = $doc_id
                """
        # Pass the id as a parameter to the query
        result = self._cluster.query(query, named_parameters={"doc_id": id}).execute()
        for row in result:
            return bool(row["count"] > 0)
        return False  # Return False if no rows are returned

    def delete_by_ids(self, ids: list[str]):
        query = f"""
            DELETE FROM `{self._bucket_name}`.{self._client_config.scope_name}.{self._collection_name}
            WHERE META().id IN $doc_ids;
            """
        try:
            self._cluster.query(query, named_parameters={"doc_ids": ids}).execute()
        except Exception:
            logger.exception("Failed to delete documents, ids: %s", ids)

    def delete_by_document_id(self, document_id: str):
        query = f"""
                DELETE FROM
                `{self._client_config.bucket_name}`.{self._client_config.scope_name}.{self._collection_name}
                WHERE META().id = $doc_id;
                """
        self._cluster.query(query, named_parameters={"doc_id": document_id}).execute()

    # def get_ids_by_metadata_field(self, key: str, value: str):
    #     query = f"""
    #         SELECT id FROM
    #         `{self._client_config.bucket_name}`.{self._client_config.scope_name}.{self._collection_name}
    #         WHERE `metadata.{key}` = $value;
    #         """
    #     result = self._cluster.query(query, named_parameters={'value':value})
    #     return [row['id'] for row in result.rows()]

    def delete_by_metadata_field(self, key: str, value: str):
        query = f"""
            DELETE FROM `{self._client_config.bucket_name}`.{self._client_config.scope_name}.{self._collection_name}
            WHERE metadata.{key} = $value;
            """
        self._cluster.query(query, named_parameters={"value": value}).execute()

    def search_by_vector(self, query_vector: list[float], **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 5)
        score_threshold = kwargs.get("score_threshold") or 0.0

        search_req = search.SearchRequest.create(
            VectorSearch.from_vector_query(
                VectorQuery(
                    "embedding",
                    query_vector,
                    top_k,
                )
            )
        )
        try:
            search_iter = self._scope.search(
                self._collection_name + "_search",
                search_req,
                SearchOptions(limit=top_k, collections=[self._collection_name], fields=["*"]),
            )

            docs = []
            # Parse the results
            for row in search_iter.rows():
                text = row.fields.pop("text")
                metadata = self._format_metadata(row.fields)
                score = row.score
                metadata["score"] = score
                doc = Document(page_content=text, metadata=metadata)
                if score >= score_threshold:
                    docs.append(doc)
        except Exception as e:
            raise ValueError(f"Search failed with error: {e}")

        return docs

    def search_by_full_text(self, query: str, **kwargs: Any) -> list[Document]:
        top_k = kwargs.get("top_k", 4)
        try:
            CBrequest = search.SearchRequest.create(search.QueryStringQuery("text:" + query))
            search_iter = self._scope.search(
                self._collection_name + "_search", CBrequest, SearchOptions(limit=top_k, fields=["*"])
            )

            docs = []
            for row in search_iter.rows():
                text = row.fields.pop("text")
                metadata = self._format_metadata(row.fields)
                score = row.score
                metadata["score"] = score
                doc = Document(page_content=text, metadata=metadata)
                docs.append(doc)

        except Exception as e:
            raise ValueError(f"Search failed with error: {e}")

        return docs

    def delete(self):
        manager = self._bucket.collections()
        scopes = manager.get_all_scopes()

        for scope in scopes:
            for collection in scope.collections:
                if collection.name == self._collection_name:
                    manager.drop_collection("_default", self._collection_name)

    def _format_metadata(self, row_fields: dict[str, Any]) -> dict[str, Any]:
        """Helper method to format the metadata from the Couchbase Search API.
        Args:
            row_fields (Dict[str, Any]): The fields to format.

        Returns:
            Dict[str, Any]: The formatted metadata.
        """
        metadata = {}
        for key, value in row_fields.items():
            # Couchbase Search returns the metadata key with a prefix
            # `metadata.` We remove it to get the original metadata key
            if key.startswith("metadata"):
                new_key = key.split("metadata" + ".")[-1]
                metadata[new_key] = value
            else:
                metadata[key] = value

        return metadata


class CouchbaseVectorFactory(AbstractVectorFactory):
    def init_vector(self, dataset: Dataset, attributes: list, embeddings: Embeddings) -> CouchbaseVector:
        if dataset.index_struct_dict:
            class_prefix: str = dataset.index_struct_dict["vector_store"]["class_prefix"]
            collection_name = class_prefix
        else:
            dataset_id = dataset.id
            collection_name = Dataset.gen_collection_name_by_id(dataset_id)
            dataset.index_struct = json.dumps(self.gen_index_struct_dict(VectorType.COUCHBASE, collection_name))

        config = current_app.config
        return CouchbaseVector(
            collection_name=collection_name,
            config=CouchbaseConfig(
                connection_string=config.get("COUCHBASE_CONNECTION_STRING", ""),
                user=config.get("COUCHBASE_USER", ""),
                password=config.get("COUCHBASE_PASSWORD", ""),
                bucket_name=config.get("COUCHBASE_BUCKET_NAME", ""),
                scope_name=config.get("COUCHBASE_SCOPE_NAME", ""),
            ),
        )

```

### api/core/rag/datasource/vdb/couchbase/__init__.py
```py

```

### api/core/rag/datasource/keyword/keyword_base.py
```py
from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any

from core.rag.models.document import Document
from models.dataset import Dataset


class BaseKeyword(ABC):
    def __init__(self, dataset: Dataset):
        self.dataset = dataset

    @abstractmethod
    def create(self, texts: list[Document], **kwargs) -> BaseKeyword:
        raise NotImplementedError

    @abstractmethod
    def add_texts(self, texts: list[Document], **kwargs):
        raise NotImplementedError

    @abstractmethod
    def text_exists(self, id: str) -> bool:
        raise NotImplementedError

    @abstractmethod
    def delete_by_ids(self, ids: list[str]):
        raise NotImplementedError

    @abstractmethod
    def delete(self):
        raise NotImplementedError

    @abstractmethod
    def search(self, query: str, **kwargs: Any) -> list[Document]:
        raise NotImplementedError

    def _filter_duplicate_texts(self, texts: list[Document]) -> list[Document]:
        for text in texts.copy():
            if text.metadata is None:
                continue
            doc_id = text.metadata["doc_id"]
            exists_duplicate_node = self.text_exists(doc_id)
            if exists_duplicate_node:
                texts.remove(text)

        return texts

    def _get_uuids(self, texts: list[Document]) -> list[str]:
        return [text.metadata["doc_id"] for text in texts if text.metadata]

```

### api/core/rag/datasource/keyword/__init__.py
```py

```

### api/core/rag/datasource/keyword/jieba/jieba_keyword_table_handler.py
```py
import re
from operator import itemgetter
from typing import cast


class JiebaKeywordTableHandler:
    def __init__(self):
        from core.rag.datasource.keyword.jieba.stopwords import STOPWORDS

        tfidf = self._load_tfidf_extractor()
        tfidf.stop_words = STOPWORDS  # type: ignore[attr-defined]
        self._tfidf = tfidf

    def _load_tfidf_extractor(self):
        """
        Load jieba TFIDF extractor with fallback strategy.

        Loading Flow:
        ┌─────────────────────────────────────────────────────────────────────┐
        │                      jieba.analyse.default_tfidf                    │
        │                              exists?                                │
        └─────────────────────────────────────────────────────────────────────┘
                           │                              │
                          YES                            NO
                           │                              │
                           ▼                              ▼
                ┌──────────────────┐       ┌──────────────────────────────────┐
                │  Return default  │       │   jieba.analyse.TFIDF exists?    │
                │      TFIDF       │       └──────────────────────────────────┘
                └──────────────────┘                │                │
                                                   YES              NO
                                                    │                │
                                                    │                ▼
                                                    │   ┌────────────────────────────┐
                                                    │   │  Try import from          │
                                                    │   │  jieba.analyse.tfidf.TFIDF │
                                                    │   └────────────────────────────┘
                                                    │          │            │
                                                    │        SUCCESS      FAILED
                                                    │          │            │
                                                    ▼          ▼            ▼
                                        ┌────────────────────────┐    ┌─────────────────┐
                                        │  Instantiate TFIDF()   │    │  Build fallback │
                                        │  & cache to default    │    │  _SimpleTFIDF   │
                                        └────────────────────────┘    └─────────────────┘
        """
        import jieba.analyse  # type: ignore

        tfidf = getattr(jieba.analyse, "default_tfidf", None)
        if tfidf is not None:
            return tfidf

        tfidf_class = getattr(jieba.analyse, "TFIDF", None)
        if tfidf_class is None:
            try:
                from jieba.analyse.tfidf import TFIDF  # type: ignore

                tfidf_class = TFIDF
            except Exception:
                tfidf_class = None

        if tfidf_class is not None:
            tfidf = tfidf_class()
            jieba.analyse.default_tfidf = tfidf  # type: ignore[attr-defined]
            return tfidf

        return self._build_fallback_tfidf()

    @staticmethod
    def _build_fallback_tfidf():
        """Fallback lightweight TFIDF for environments missing jieba's TFIDF."""
        import jieba  # type: ignore

        from core.rag.datasource.keyword.jieba.stopwords import STOPWORDS

        class _SimpleTFIDF:
            def __init__(self):
                self.stop_words = STOPWORDS
                self._lcut = getattr(jieba, "lcut", None)

            def extract_tags(self, sentence: str, top_k: int | None = 20, **kwargs):
                # Basic frequency-based keyword extraction as a fallback when TF-IDF is unavailable.
                top_k = kwargs.pop("topK", top_k)
                cut = getattr(jieba, "cut", None)
                if self._lcut:
                    tokens = self._lcut(sentence)
                elif callable(cut):
                    tokens = list(cut(sentence))
                else:
                    tokens = re.findall(r"\w+", sentence)

                words = [w for w in tokens if w and w not in self.stop_words]
                freq: dict[str, int] = {}
                for w in words:
                    freq[w] = freq.get(w, 0) + 1

                sorted_words = sorted(freq.items(), key=itemgetter(1), reverse=True)
                if top_k is not None:
                    sorted_words = sorted_words[:top_k]

                return [item[0] for item in sorted_words]

        return _SimpleTFIDF()

    def extract_keywords(self, text: str, max_keywords_per_chunk: int | None = 10) -> set[str]:
        """Extract keywords with JIEBA tfidf."""
        keywords = self._tfidf.extract_tags(
            sentence=text,
            topK=max_keywords_per_chunk,
        )
        # jieba.analyse.extract_tags returns list[Any] when withFlag is False by default.
        keywords = cast(list[str], keywords)

        return set(self._expand_tokens_with_subtokens(set(keywords)))

    def _expand_tokens_with_subtokens(self, tokens: set[str]) -> set[str]:
        """Get subtokens from a list of tokens., filtering for stopwords."""
        from core.rag.datasource.keyword.jieba.stopwords import STOPWORDS

        results = set()
        for token in tokens:
            results.add(token)
            sub_tokens = re.findall(r"\w+", token)
            if len(sub_tokens) > 1:
                results.update({w for w in sub_tokens if w not in STOPWORDS})

        return results

```

### api/core/rag/datasource/keyword/jieba/__init__.py
```py

```

### api/core/rag/datasource/keyword/jieba/jieba.py
```py
from collections import defaultdict
from typing import Any, TypedDict

import orjson
from pydantic import BaseModel
from sqlalchemy import select

from configs import dify_config
from core.rag.datasource.keyword.jieba.jieba_keyword_table_handler import JiebaKeywordTableHandler
from core.rag.datasource.keyword.keyword_base import BaseKeyword
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from extensions.ext_storage import storage
from models.dataset import Dataset, DatasetKeywordTable, DocumentSegment


class PreSegmentData(TypedDict):
    segment: DocumentSegment
    keywords: list[str]


class KeywordTableConfig(BaseModel):
    max_keywords_per_chunk: int = 10


class Jieba(BaseKeyword):
    def __init__(self, dataset: Dataset):
        super().__init__(dataset)
        self._config = KeywordTableConfig()

    def create(self, texts: list[Document], **kwargs) -> BaseKeyword:
        lock_name = f"keyword_indexing_lock_{self.dataset.id}"
        with redis_client.lock(lock_name, timeout=600):
            keyword_table_handler = JiebaKeywordTableHandler()
            keyword_table = self._get_dataset_keyword_table()
            keyword_number = self.dataset.keyword_number or self._config.max_keywords_per_chunk

            for text in texts:
                keywords = keyword_table_handler.extract_keywords(text.page_content, keyword_number)
                if text.metadata is not None:
                    self._update_segment_keywords(self.dataset.id, text.metadata["doc_id"], list(keywords))
                    keyword_table = self._add_text_to_keyword_table(
                        keyword_table or {}, text.metadata["doc_id"], list(keywords)
                    )

            self._save_dataset_keyword_table(keyword_table)

            return self

    def add_texts(self, texts: list[Document], **kwargs):
        lock_name = f"keyword_indexing_lock_{self.dataset.id}"
        with redis_client.lock(lock_name, timeout=600):
            keyword_table_handler = JiebaKeywordTableHandler()

            keyword_table = self._get_dataset_keyword_table()
            keywords_list = kwargs.get("keywords_list")
            keyword_number = self.dataset.keyword_number or self._config.max_keywords_per_chunk
            for i in range(len(texts)):
                text = texts[i]
                if keywords_list:
                    keywords = keywords_list[i]
                    if not keywords:
                        keywords = keyword_table_handler.extract_keywords(text.page_content, keyword_number)
                else:
                    keywords = keyword_table_handler.extract_keywords(text.page_content, keyword_number)
                if text.metadata is not None:
                    self._update_segment_keywords(self.dataset.id, text.metadata["doc_id"], list(keywords))
                    keyword_table = self._add_text_to_keyword_table(
                        keyword_table or {}, text.metadata["doc_id"], list(keywords)
                    )

            self._save_dataset_keyword_table(keyword_table)

    def text_exists(self, id: str) -> bool:
        keyword_table = self._get_dataset_keyword_table()
        if keyword_table is None:
            return False
        return id in set.union(*keyword_table.values())

    def delete_by_ids(self, ids: list[str]):
        lock_name = f"keyword_indexing_lock_{self.dataset.id}"
        with redis_client.lock(lock_name, timeout=600):
            keyword_table = self._get_dataset_keyword_table()
            if keyword_table is not None:
                keyword_table = self._delete_ids_from_keyword_table(keyword_table, ids)

            self._save_dataset_keyword_table(keyword_table)

    def search(self, query: str, **kwargs: Any) -> list[Document]:
        keyword_table = self._get_dataset_keyword_table()

        k = kwargs.get("top_k", 4)
        document_ids_filter = kwargs.get("document_ids_filter")
        sorted_chunk_indices = self._retrieve_ids_by_query(keyword_table or {}, query, k)

        documents = []

        segment_query_stmt = select(DocumentSegment).where(
            DocumentSegment.dataset_id == self.dataset.id, DocumentSegment.index_node_id.in_(sorted_chunk_indices)
        )
        if document_ids_filter:
            segment_query_stmt = segment_query_stmt.where(DocumentSegment.document_id.in_(document_ids_filter))

        segments = db.session.scalars(segment_query_stmt).all()
        segment_map = {segment.index_node_id: segment for segment in segments}
        for chunk_index in sorted_chunk_indices:
            segment = segment_map.get(chunk_index)

            if segment:
                documents.append(
                    Document(
                        page_content=segment.content,
                        metadata={
                            "doc_id": chunk_index,
                            "doc_hash": segment.index_node_hash,
                            "document_id": segment.document_id,
                            "dataset_id": segment.dataset_id,
                        },
                    )
                )

        return documents

    def delete(self):
        lock_name = f"keyword_indexing_lock_{self.dataset.id}"
        with redis_client.lock(lock_name, timeout=600):
            dataset_keyword_table = self.dataset.dataset_keyword_table
            if dataset_keyword_table:
                db.session.delete(dataset_keyword_table)
                db.session.commit()
                if dataset_keyword_table.data_source_type != "database":
                    file_key = "keyword_files/" + self.dataset.tenant_id + "/" + self.dataset.id + ".txt"
                    storage.delete(file_key)

    def _save_dataset_keyword_table(self, keyword_table: dict[str, set[str]] | None):
        keyword_table_dict = {
            "__type__": "keyword_table",
            "__data__": {"index_id": self.dataset.id, "summary": None, "table": keyword_table},
        }
        dataset_keyword_table = self.dataset.dataset_keyword_table
        keyword_data_source_type = dataset_keyword_table.data_source_type
        if keyword_data_source_type == "database":
            dataset_keyword_table.keyword_table = dumps_with_sets(keyword_table_dict)
            db.session.commit()
        else:
            file_key = "keyword_files/" + self.dataset.tenant_id + "/" + self.dataset.id + ".txt"
            if storage.exists(file_key):
                storage.delete(file_key)
            storage.save(file_key, dumps_with_sets(keyword_table_dict).encode("utf-8"))

    def _get_dataset_keyword_table(self) -> dict[str, set[str]] | None:
        dataset_keyword_table = self.dataset.dataset_keyword_table
        if dataset_keyword_table:
            keyword_table_dict = dataset_keyword_table.keyword_table_dict
            if keyword_table_dict:
                return dict(keyword_table_dict["__data__"]["table"])
        else:
            keyword_data_source_type = dify_config.KEYWORD_DATA_SOURCE_TYPE
            dataset_keyword_table = DatasetKeywordTable(
                dataset_id=self.dataset.id,
                keyword_table="",
                data_source_type=keyword_data_source_type,
            )
            if keyword_data_source_type == "database":
                dataset_keyword_table.keyword_table = dumps_with_sets(
                    {
                        "__type__": "keyword_table",
                        "__data__": {"index_id": self.dataset.id, "summary": None, "table": {}},
                    }
                )
            db.session.add(dataset_keyword_table)
            db.session.commit()

        return {}

    def _add_text_to_keyword_table(
        self, keyword_table: dict[str, set[str]], id: str, keywords: list[str]
    ) -> dict[str, set[str]]:
        for keyword in keywords:
            if keyword not in keyword_table:
                keyword_table[keyword] = set()
            keyword_table[keyword].add(id)
        return keyword_table

    def _delete_ids_from_keyword_table(self, keyword_table: dict[str, set[str]], ids: list[str]) -> dict[str, set[str]]:
        # get set of ids that correspond to node
        node_idxs_to_delete = set(ids)

        # delete node_idxs from keyword to node idxs mapping
        keywords_to_delete = set()
        for keyword, node_idxs in keyword_table.items():
            if node_idxs_to_delete.intersection(node_idxs):
                keyword_table[keyword] = node_idxs.difference(node_idxs_to_delete)
                if not keyword_table[keyword]:
                    keywords_to_delete.add(keyword)

        for keyword in keywords_to_delete:
            del keyword_table[keyword]

        return keyword_table

    def _retrieve_ids_by_query(self, keyword_table: dict[str, set[str]], query: str, k: int = 4) -> list[str]:
        keyword_table_handler = JiebaKeywordTableHandler()
        keywords = keyword_table_handler.extract_keywords(query)

        # go through text chunks in order of most matching keywords
        chunk_indices_count: dict[str, int] = defaultdict(int)
        keywords_list = [keyword for keyword in keywords if keyword in set(keyword_table.keys())]
        for keyword in keywords_list:
            for node_id in keyword_table[keyword]:
                chunk_indices_count[node_id] += 1

        sorted_chunk_indices = sorted(
            chunk_indices_count.keys(),
            key=lambda x: chunk_indices_count[x],
            reverse=True,
        )

        return sorted_chunk_indices[:k]

    def _update_segment_keywords(self, dataset_id: str, node_id: str, keywords: list[str]):
        stmt = select(DocumentSegment).where(
            DocumentSegment.dataset_id == dataset_id, DocumentSegment.index_node_id == node_id
        )
        document_segment = db.session.scalar(stmt)
        if document_segment:
            document_segment.keywords = keywords
            db.session.add(document_segment)
            db.session.commit()

    def create_segment_keywords(self, node_id: str, keywords: list[str]):
        keyword_table = self._get_dataset_keyword_table()
        self._update_segment_keywords(self.dataset.id, node_id, keywords)
        keyword_table = self._add_text_to_keyword_table(keyword_table or {}, node_id, keywords)
        self._save_dataset_keyword_table(keyword_table)

    def multi_create_segment_keywords(self, pre_segment_data_list: list[PreSegmentData]):
        keyword_table_handler = JiebaKeywordTableHandler()
        keyword_table = self._get_dataset_keyword_table()
        for pre_segment_data in pre_segment_data_list:
            segment = pre_segment_data["segment"]
            if pre_segment_data["keywords"]:
                segment.keywords = pre_segment_data["keywords"]
                keyword_table = self._add_text_to_keyword_table(
                    keyword_table or {}, segment.index_node_id, pre_segment_data["keywords"]
                )
            else:
                keyword_number = self.dataset.keyword_number or self._config.max_keywords_per_chunk

                keywords = keyword_table_handler.extract_keywords(segment.content, keyword_number)
                segment.keywords = list(keywords)
                keyword_table = self._add_text_to_keyword_table(
                    keyword_table or {}, segment.index_node_id, list(keywords)
                )
        self._save_dataset_keyword_table(keyword_table)

    def update_segment_keywords_index(self, node_id: str, keywords: list[str]):
        keyword_table = self._get_dataset_keyword_table()
        keyword_table = self._add_text_to_keyword_table(keyword_table or {}, node_id, keywords)
        self._save_dataset_keyword_table(keyword_table)


def set_orjson_default(obj: Any):
    """Default function for orjson serialization of set types"""
    if isinstance(obj, set):
        return list(obj)
    raise TypeError(f"Object of type {type(obj).__name__} is not JSON serializable")


def dumps_with_sets(obj: Any) -> str:
    """JSON dumps with set support using orjson"""
    return orjson.dumps(obj, default=set_orjson_default).decode("utf-8")

```

### api/core/rag/datasource/keyword/jieba/stopwords.py
```py
STOPWORDS: frozenset[str] = frozenset(
    (
        "during",
        "when",
        "but",
        "then",
        "further",
        "isn",
        "mustn't",
        "until",
        "own",
        "i",
        "couldn",
        "y",
        "only",
        "you've",
        "ours",
        "who",
        "where",
        "ourselves",
        "has",
        "to",
        "was",
        "didn't",
        "themselves",
        "if",
        "against",
        "through",
        "her",
        "an",
        "your",
        "can",
        "those",
        "didn",
        "about",
        "aren't",
        "shan't",
        "be",
        "not",
        "these",
        "again",
        "so",
        "t",
        "theirs",
        "weren",
        "won't",
        "won",
        "itself",
        "just",
        "same",
        "while",
        "why",
        "doesn",
        "aren",
        "him",
        "haven",
        "for",
        "you'll",
        "that",
        "we",
        "am",
        "d",
        "by",
        "having",
        "wasn't",
        "than",
        "weren't",
        "out",
        "from",
        "now",
        "their",
        "too",
        "hadn",
        "o",
        "needn",
        "most",
        "it",
        "under",
        "needn't",
        "any",
        "some",
        "few",
        "ll",
        "hers",
        "which",
        "m",
        "you're",
        "off",
        "other",
        "had",
        "she",
        "you'd",
        "do",
        "you",
        "does",
        "s",
        "will",
        "each",
        "wouldn't",
        "hasn't",
        "such",
        "more",
        "whom",
        "she's",
        "my",
        "yours",
        "yourself",
        "of",
        "on",
        "very",
        "hadn't",
        "with",
        "yourselves",
        "been",
        "ma",
        "them",
        "mightn't",
        "shan",
        "mustn",
        "they",
        "what",
        "both",
        "that'll",
        "how",
        "is",
        "he",
        "because",
        "down",
        "haven't",
        "are",
        "no",
        "it's",
        "our",
        "being",
        "the",
        "or",
        "above",
        "myself",
        "once",
        "don't",
        "doesn't",
        "as",
        "nor",
        "here",
        "herself",
        "hasn",
        "mightn",
        "have",
        "its",
        "all",
        "were",
        "ain",
        "this",
        "at",
        "after",
        "over",
        "shouldn't",
        "into",
        "before",
        "don",
        "wouldn",
        "re",
        "couldn't",
        "wasn",
        "in",
        "should",
        "there",
        "himself",
        "isn't",
        "should've",
        "doing",
        "ve",
        "shouldn",
        "a",
        "did",
        "and",
        "his",
        "between",
        "me",
        "up",
        "below",
        "人民",
        "末##末",
        "啊",
        "阿",
        "哎",
        "哎呀",
        "哎哟",
        "唉",
        "俺",
        "俺们",
        "按",
        "按照",
        "吧",
        "吧哒",
        "把",
        "罢了",
        "被",
        "本",
        "本着",
        "比",
        "比方",
        "比如",
        "鄙人",
        "彼",
        "彼此",
        "边",
        "别",
        "别的",
        "别说",
        "并",
        "并且",
        "不比",
        "不成",
        "不单",
        "不但",
        "不独",
        "不管",
        "不光",
        "不过",
        "不仅",
        "不拘",
        "不论",
        "不怕",
        "不然",
        "不如",
        "不特",
        "不惟",
        "不问",
        "不只",
        "朝",
        "朝着",
        "趁",
        "趁着",
        "乘",
        "冲",
        "除",
        "除此之外",
        "除非",
        "除了",
        "此",
        "此间",
        "此外",
        "从",
        "从而",
        "打",
        "待",
        "但",
        "但是",
        "当",
        "当着",
        "到",
        "得",
        "的",
        "的话",
        "等",
        "等等",
        "地",
        "第",
        "叮咚",
        "对",
        "对于",
        "多",
        "多少",
        "而",
        "而况",
        "而且",
        "而是",
        "而外",
        "而言",
        "而已",
        "尔后",
        "反过来",
        "反过来说",
        "反之",
        "非但",
        "非徒",
        "否则",
        "嘎",
        "嘎登",
        "该",
        "赶",
        "个",
        "各",
        "各个",
        "各位",
        "各种",
        "各自",
        "给",
        "根据",
        "跟",
        "故",
        "故此",
        "固然",
        "关于",
        "管",
        "归",
        "果然",
        "果真",
        "过",
        "哈",
        "哈哈",
        "呵",
        "和",
        "何",
        "何处",
        "何况",
        "何时",
        "嘿",
        "哼",
        "哼唷",
        "呼哧",
        "乎",
        "哗",
        "还是",
        "还有",
        "换句话说",
        "换言之",
        "或",
        "或是",
        "或者",
        "极了",
        "及",
        "及其",
        "及至",
        "即",
        "即便",
        "即或",
        "即令",
        "即若",
        "即使",
        "几",
        "几时",
        "己",
        "既",
        "既然",
        "既是",
        "继而",
        "加之",
        "假如",
        "假若",
        "假使",
        "鉴于",
        "将",
        "较",
        "较之",
        "叫",
        "接着",
        "结果",
        "借",
        "紧接着",
        "进而",
        "尽",
        "尽管",
        "经",
        "经过",
        "就",
        "就是",
        "就是说",
        "据",
        "具体地说",
        "具体说来",
        "开始",
        "开外",
        "靠",
        "咳",
        "可",
        "可见",
        "可是",
        "可以",
        "况且",
        "啦",
        "来",
        "来着",
        "离",
        "例如",
        "哩",
        "连",
        "连同",
        "两者",
        "了",
        "临",
        "另",
        "另外",
        "另一方面",
        "论",
        "嘛",
        "吗",
        "慢说",
        "漫说",
        "冒",
        "么",
        "每",
        "每当",
        "们",
        "莫若",
        "某",
        "某个",
        "某些",
        "拿",
        "哪",
        "哪边",
        "哪儿",
        "哪个",
        "哪里",
        "哪年",
        "哪怕",
        "哪天",
        "哪些",
        "哪样",
        "那",
        "那边",
        "那儿",
        "那个",
        "那会儿",
        "那里",
        "那么",
        "那么些",
        "那么样",
        "那时",
        "那些",
        "那样",
        "乃",
        "乃至",
        "呢",
        "能",
        "你",
        "你们",
        "您",
        "宁",
        "宁可",
        "宁肯",
        "宁愿",
        "哦",
        "呕",
        "啪达",
        "旁人",
        "呸",
        "凭",
        "凭借",
        "其",
        "其次",
        "其二",
        "其他",
        "其它",
        "其一",
        "其余",
        "其中",
        "起",
        "起见",
        "岂但",
        "恰恰相反",
        "前后",
        "前者",
        "且",
        "然而",
        "然后",
        "然则",
        "让",
        "人家",
        "任",
        "任何",
        "任凭",
        "如",
        "如此",
        "如果",
        "如何",
        "如其",
        "如若",
        "如上所述",
        "若",
        "若非",
        "若是",
        "啥",
        "上下",
        "尚且",
        "设若",
        "设使",
        "甚而",
        "甚么",
        "甚至",
        "省得",
        "时候",
        "什么",
        "什么样",
        "使得",
        "是",
        "是的",
        "首先",
        "谁",
        "谁知",
        "顺",
        "顺着",
        "似的",
        "虽",
        "虽然",
        "虽说",
        "虽则",
        "随",
        "随着",
        "所",
        "所以",
        "他",
        "他们",
        "他人",
        "它",
        "它们",
        "她",
        "她们",
        "倘",
        "倘或",
        "倘然",
        "倘若",
        "倘使",
        "腾",
        "替",
        "通过",
        "同",
        "同时",
        "哇",
        "万一",
        "往",
        "望",
        "为",
        "为何",
        "为了",
        "为什么",
        "为着",
        "喂",
        "嗡嗡",
        "我",
        "我们",
        "呜",
        "呜呼",
        "乌乎",
        "无论",
        "无宁",
        "毋宁",
        "嘻",
        "吓",
        "相对而言",
        "像",
        "向",
        "向着",
        "嘘",
        "呀",
        "焉",
        "沿",
        "沿着",
        "要",
        "要不",
        "要不然",
        "要不是",
        "要么",
        "要是",
        "也",
        "也罢",
        "也好",
        "一",
        "一般",
        "一旦",
        "一方面",
        "一来",
        "一切",
        "一样",
        "一则",
        "依",
        "依照",
        "矣",
        "以",
        "以便",
        "以及",
        "以免",
        "以至",
        "以至于",
        "以致",
        "抑或",
        "因",
        "因此",
        "因而",
        "因为",
        "哟",
        "用",
        "由",
        "由此可见",
        "由于",
        "有",
        "有的",
        "有关",
        "有些",
        "又",
        "于",
        "于是",
        "于是乎",
        "与",
        "与此同时",
        "与否",
        "与其",
        "越是",
        "云云",
        "哉",
        "再说",
        "再者",
        "在",
        "在下",
        "咱",
        "咱们",
        "则",
        "怎",
        "怎么",
        "怎么办",
        "怎么样",
        "怎样",
        "咋",
        "照",
        "照着",
        "者",
        "这",
        "这边",
        "这儿",
        "这个",
        "这会儿",
        "这就是说",
        "这里",
        "这么",
        "这么点儿",
        "这么些",
        "这么样",
        "这时",
        "这些",
        "这样",
        "正如",
        "吱",
        "之",
        "之类",
        "之所以",
        "之一",
        "只是",
        "只限",
        "只要",
        "只有",
        "至",
        "至于",
        "诸位",
        "着",
        "着呢",
        "自",
        "自从",
        "自个儿",
        "自各儿",
        "自己",
        "自家",
        "自身",
        "综上所述",
        "总的来看",
        "总的来说",
        "总的说来",
        "总而言之",
        "总之",
        "纵",
        "纵令",
        "纵然",
        "纵使",
        "遵照",
        "作为",
        "兮",
        "呃",
        "呗",
        "咚",
        "咦",
        "喏",
        "啐",
        "喔唷",
        "嗬",
        "嗯",
        "嗳",
        "~",
        "!",
        ".",
        ":",
        '"',
        "'",
        "(",
        ")",
        "*",
        "A",
        "白",
        "社会主义",
        "--",
        "..",
        ">>",
        " [",
        " ]",
        "",
        "<",
        ">",
        "/",
        "\\",
        "|",
        "-",
        "_",
        "+",
        "=",
        "&",
        "^",
        "%",
        "#",
        "@",
        "`",
        ";",
        "$",
        "（",
        "）",
        "——",
        "—",
        "￥",
        "·",
        "...",
        "‘",
        "’",
        "〉",
        "〈",
        "…",
        " ",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "二",
        "三",
        "四",
        "五",
        "六",
        "七",
        "八",
        "九",
        "零",
        "＞",
        "＜",
        "＠",
        "＃",
        "＄",
        "％",
        "︿",
        "＆",
        "＊",
        "＋",
        "～",
        "｜",
        "［",
        "］",
        "｛",
        "｝",
        "啊哈",
        "啊呀",
        "啊哟",
        "挨次",
        "挨个",
        "挨家挨户",
        "挨门挨户",
        "挨门逐户",
        "挨着",
        "按理",
        "按期",
        "按时",
        "按说",
        "暗地里",
        "暗中",
        "暗自",
        "昂然",
        "八成",
        "白白",
        "半",
        "梆",
        "保管",
        "保险",
        "饱",
        "背地里",
        "背靠背",
        "倍感",
        "倍加",
        "本人",
        "本身",
        "甭",
        "比起",
        "比如说",
        "比照",
        "毕竟",
        "必",
        "必定",
        "必将",
        "必须",
        "便",
        "别人",
        "并非",
        "并肩",
        "并没",
        "并没有",
        "并排",
        "并无",
        "勃然",
        "不",
        "不必",
        "不常",
        "不大",
        "不但...而且",
        "不得",
        "不得不",
        "不得了",
        "不得已",
        "不迭",
        "不定",
        "不对",
        "不妨",
        "不管怎样",
        "不会",
        "不仅...而且",
        "不仅仅",
        "不仅仅是",
        "不经意",
        "不可开交",
        "不可抗拒",
        "不力",
        "不了",
        "不料",
        "不满",
        "不免",
        "不能不",
        "不起",
        "不巧",
        "不然的话",
        "不日",
        "不少",
        "不胜",
        "不时",
        "不是",
        "不同",
        "不能",
        "不要",
        "不外",
        "不外乎",
        "不下",
        "不限",
        "不消",
        "不已",
        "不亦乐乎",
        "不由得",
        "不再",
        "不择手段",
        "不怎么",
        "不曾",
        "不知不觉",
        "不止",
        "不止一次",
        "不至于",
        "才",
        "才能",
        "策略地",
        "差不多",
        "差一点",
        "常",
        "常常",
        "常言道",
        "常言说",
        "常言说得好",
        "长此下去",
        "长话短说",
        "长期以来",
        "长线",
        "敞开儿",
        "彻夜",
        "陈年",
        "趁便",
        "趁机",
        "趁热",
        "趁势",
        "趁早",
        "成年",
        "成年累月",
        "成心",
        "乘机",
        "乘胜",
        "乘势",
        "乘隙",
        "乘虚",
        "诚然",
        "迟早",
        "充分",
        "充其极",
        "充其量",
        "抽冷子",
        "臭",
        "初",
        "出",
        "出来",
        "出去",
        "除此",
        "除此而外",
        "除此以外",
        "除开",
        "除去",
        "除却",
        "除外",
        "处处",
        "川流不息",
        "传",
        "传说",
        "传闻",
        "串行",
        "纯",
        "纯粹",
        "此后",
        "此中",
        "次第",
        "匆匆",
        "从不",
        "从此",
        "从此以后",
        "从古到今",
        "从古至今",
        "从今以后",
        "从宽",
        "从来",
        "从轻",
        "从速",
        "从头",
        "从未",
        "从无到有",
        "从小",
        "从新",
        "从严",
        "从优",
        "从早到晚",
        "从中",
        "从重",
        "凑巧",
        "粗",
        "存心",
        "达旦",
        "打从",
        "打开天窗说亮话",
        "大",
        "大不了",
        "大大",
        "大抵",
        "大都",
        "大多",
        "大凡",
        "大概",
        "大家",
        "大举",
        "大略",
        "大面儿上",
        "大事",
        "大体",
        "大体上",
        "大约",
        "大张旗鼓",
        "大致",
        "呆呆地",
        "带",
        "殆",
        "待到",
        "单",
        "单纯",
        "单单",
        "但愿",
        "弹指之间",
        "当场",
        "当儿",
        "当即",
        "当口儿",
        "当然",
        "当庭",
        "当头",
        "当下",
        "当真",
        "当中",
        "倒不如",
        "倒不如说",
        "倒是",
        "到处",
        "到底",
        "到了儿",
        "到目前为止",
        "到头",
        "到头来",
        "得起",
        "得天独厚",
        "的确",
        "等到",
        "叮当",
        "顶多",
        "定",
        "动不动",
        "动辄",
        "陡然",
        "都",
        "独",
        "独自",
        "断然",
        "顿时",
        "多次",
        "多多",
        "多多少少",
        "多多益善",
        "多亏",
        "多年来",
        "多年前",
        "而后",
        "而论",
        "而又",
        "尔等",
        "二话不说",
        "二话没说",
        "反倒",
        "反倒是",
        "反而",
        "反手",
        "反之亦然",
        "反之则",
        "方",
        "方才",
        "方能",
        "放量",
        "非常",
        "非得",
        "分期",
        "分期分批",
        "分头",
        "奋勇",
        "愤然",
        "风雨无阻",
        "逢",
        "弗",
        "甫",
        "嘎嘎",
        "该当",
        "概",
        "赶快",
        "赶早不赶晚",
        "敢",
        "敢情",
        "敢于",
        "刚",
        "刚才",
        "刚好",
        "刚巧",
        "高低",
        "格外",
        "隔日",
        "隔夜",
        "个人",
        "各式",
        "更",
        "更加",
        "更进一步",
        "更为",
        "公然",
        "共",
        "共总",
        "够瞧的",
        "姑且",
        "古来",
        "故而",
        "故意",
        "固",
        "怪",
        "怪不得",
        "惯常",
        "光",
        "光是",
        "归根到底",
        "归根结底",
        "过于",
        "毫不",
        "毫无",
        "毫无保留地",
        "毫无例外",
        "好在",
        "何必",
        "何尝",
        "何妨",
        "何苦",
        "何乐而不为",
        "何须",
        "何止",
        "很",
        "很多",
        "很少",
        "轰然",
        "后来",
        "呼啦",
        "忽地",
        "忽然",
        "互",
        "互相",
        "哗啦",
        "话说",
        "还",
        "恍然",
        "会",
        "豁然",
        "活",
        "伙同",
        "或多或少",
        "或许",
        "基本",
        "基本上",
        "基于",
        "极",
        "极大",
        "极度",
        "极端",
        "极力",
        "极其",
        "极为",
        "急匆匆",
        "即将",
        "即刻",
        "即是说",
        "几度",
        "几番",
        "几乎",
        "几经",
        "既...又",
        "继之",
        "加上",
        "加以",
        "间或",
        "简而言之",
        "简言之",
        "简直",
        "见",
        "将才",
        "将近",
        "将要",
        "交口",
        "较比",
        "较为",
        "接连不断",
        "接下来",
        "皆可",
        "截然",
        "截至",
        "藉以",
        "借此",
        "借以",
        "届时",
        "仅",
        "仅仅",
        "谨",
        "进来",
        "进去",
        "近",
        "近几年来",
        "近来",
        "近年来",
        "尽管如此",
        "尽可能",
        "尽快",
        "尽量",
        "尽然",
        "尽如人意",
        "尽心竭力",
        "尽心尽力",
        "尽早",
        "精光",
        "经常",
        "竟",
        "竟然",
        "究竟",
        "就此",
        "就地",
        "就算",
        "居然",
        "局外",
        "举凡",
        "据称",
        "据此",
        "据实",
        "据说",
        "据我所知",
        "据悉",
        "具体来说",
        "决不",
        "决非",
        "绝",
        "绝不",
        "绝顶",
        "绝对",
        "绝非",
        "均",
        "喀",
        "看",
        "看来",
        "看起来",
        "看上去",
        "看样子",
        "可好",
        "可能",
        "恐怕",
        "快",
        "快要",
        "来不及",
        "来得及",
        "来讲",
        "来看",
        "拦腰",
        "牢牢",
        "老",
        "老大",
        "老老实实",
        "老是",
        "累次",
        "累年",
        "理当",
        "理该",
        "理应",
        "历",
        "立",
        "立地",
        "立刻",
        "立马",
        "立时",
        "联袂",
        "连连",
        "连日",
        "连日来",
        "连声",
        "连袂",
        "临到",
        "另方面",
        "另行",
        "另一个",
        "路经",
        "屡",
        "屡次",
        "屡次三番",
        "屡屡",
        "缕缕",
        "率尔",
        "率然",
        "略",
        "略加",
        "略微",
        "略为",
        "论说",
        "马上",
        "蛮",
        "满",
        "没",
        "没有",
        "每逢",
        "每每",
        "每时每刻",
        "猛然",
        "猛然间",
        "莫",
        "莫不",
        "莫非",
        "莫如",
        "默默地",
        "默然",
        "呐",
        "那末",
        "奈",
        "难道",
        "难得",
        "难怪",
        "难说",
        "内",
        "年复一年",
        "凝神",
        "偶而",
        "偶尔",
        "怕",
        "砰",
        "碰巧",
        "譬如",
        "偏偏",
        "乒",
        "平素",
        "颇",
        "迫于",
        "扑通",
        "其后",
        "其实",
        "奇",
        "齐",
        "起初",
        "起来",
        "起首",
        "起头",
        "起先",
        "岂",
        "岂非",
        "岂止",
        "迄",
        "恰逢",
        "恰好",
        "恰恰",
        "恰巧",
        "恰如",
        "恰似",
        "千",
        "千万",
        "千万千万",
        "切",
        "切不可",
        "切莫",
        "切切",
        "切勿",
        "窃",
        "亲口",
        "亲身",
        "亲手",
        "亲眼",
        "亲自",
        "顷",
        "顷刻",
        "顷刻间",
        "顷刻之间",
        "请勿",
        "穷年累月",
        "取道",
        "去",
        "权时",
        "全都",
        "全力",
        "全年",
        "全然",
        "全身心",
        "然",
        "人人",
        "仍",
        "仍旧",
        "仍然",
        "日复一日",
        "日见",
        "日渐",
        "日益",
        "日臻",
        "如常",
        "如此等等",
        "如次",
        "如今",
        "如期",
        "如前所述",
        "如上",
        "如下",
        "汝",
        "三番两次",
        "三番五次",
        "三天两头",
        "瑟瑟",
        "沙沙",
        "上",
        "上来",
        "上去",
        "一个",
        "月",
        "日",
        "\n",
    )
)

```

### api/core/rag/datasource/keyword/keyword_type.py
```py
from enum import StrEnum


class KeyWordType(StrEnum):
    JIEBA = "jieba"

```

### api/core/rag/datasource/keyword/keyword_factory.py
```py
from typing import Any

from configs import dify_config
from core.rag.datasource.keyword.keyword_base import BaseKeyword
from core.rag.datasource.keyword.keyword_type import KeyWordType
from core.rag.models.document import Document
from models.dataset import Dataset


class Keyword:
    def __init__(self, dataset: Dataset):
        self._dataset = dataset
        self._keyword_processor = self._init_keyword()

    def _init_keyword(self) -> BaseKeyword:
        keyword_type = dify_config.KEYWORD_STORE
        keyword_factory = self.get_keyword_factory(keyword_type)
        return keyword_factory(self._dataset)

    @staticmethod
    def get_keyword_factory(keyword_type: str) -> type[BaseKeyword]:
        match keyword_type:
            case KeyWordType.JIEBA:
                from core.rag.datasource.keyword.jieba.jieba import Jieba

                return Jieba
            case _:
                raise ValueError(f"Keyword store {keyword_type} is not supported.")

    def create(self, texts: list[Document], **kwargs):
        self._keyword_processor.create(texts, **kwargs)

    def add_texts(self, texts: list[Document], **kwargs):
        self._keyword_processor.add_texts(texts, **kwargs)

    def text_exists(self, id: str) -> bool:
        return self._keyword_processor.text_exists(id)

    def delete_by_ids(self, ids: list[str]):
        self._keyword_processor.delete_by_ids(ids)

    def delete(self):
        self._keyword_processor.delete()

    def search(self, query: str, **kwargs: Any) -> list[Document]:
        return self._keyword_processor.search(query, **kwargs)

    def __getattr__(self, name):
        if self._keyword_processor is not None:
            method = getattr(self._keyword_processor, name)
            if callable(method):
                return method

        raise AttributeError(f"'Keyword' object has no attribute '{name}'")

```

### api/core/rag/datasource/retrieval_service.py
```py
import concurrent.futures
import logging
from concurrent.futures import ThreadPoolExecutor
from typing import Any, NotRequired, TypedDict

from flask import Flask, current_app
from graphon.model_runtime.entities.model_entities import ModelType
from sqlalchemy import select
from sqlalchemy.orm import Session, load_only

from configs import dify_config
from core.db.session_factory import session_factory
from core.model_manager import ModelManager
from core.rag.data_post_processor.data_post_processor import DataPostProcessor, RerankingModelDict, WeightsDict
from core.rag.datasource.keyword.keyword_factory import Keyword
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.embedding.retrieval import AttachmentInfoDict, RetrievalChildChunk, RetrievalSegments
from core.rag.entities.metadata_entities import MetadataCondition
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.constant.query_type import QueryType
from core.rag.models.document import Document
from core.rag.rerank.rerank_type import RerankMode
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.tools.signature import sign_upload_file
from extensions.ext_database import db
from models.dataset import (
    ChildChunk,
    Dataset,
    DocumentSegment,
    DocumentSegmentSummary,
    SegmentAttachmentBinding,
)
from models.dataset import Document as DatasetDocument
from models.model import UploadFile
from services.external_knowledge_service import ExternalDatasetService


class SegmentAttachmentResult(TypedDict):
    attachment_info: AttachmentInfoDict
    segment_id: str


class SegmentAttachmentInfoResult(TypedDict):
    attachment_id: str
    attachment_info: AttachmentInfoDict
    segment_id: str


class ChildChunkDetail(TypedDict):
    id: str
    content: str
    position: int
    score: float


class SegmentChildMapDetail(TypedDict):
    max_score: float
    child_chunks: list[ChildChunkDetail]


class SegmentRecord(TypedDict):
    segment: DocumentSegment
    score: NotRequired[float]
    child_chunks: NotRequired[list[ChildChunkDetail]]
    files: NotRequired[list[AttachmentInfoDict]]


class DefaultRetrievalModelDict(TypedDict):
    search_method: RetrievalMethod
    reranking_enable: bool
    reranking_model: RerankingModelDict
    reranking_mode: NotRequired[str]
    weights: NotRequired[WeightsDict | None]
    score_threshold: NotRequired[float]
    top_k: int
    score_threshold_enabled: bool


default_retrieval_model: DefaultRetrievalModelDict = {
    "search_method": RetrievalMethod.SEMANTIC_SEARCH,
    "reranking_enable": False,
    "reranking_model": {"reranking_provider_name": "", "reranking_model_name": ""},
    "top_k": 4,
    "score_threshold_enabled": False,
}

logger = logging.getLogger(__name__)


class RetrievalService:
    # Cache precompiled regular expressions to avoid repeated compilation
    @classmethod
    def retrieve(
        cls,
        retrieval_method: RetrievalMethod,
        dataset_id: str,
        query: str,
        top_k: int = 4,
        score_threshold: float | None = 0.0,
        reranking_model: RerankingModelDict | None = None,
        reranking_mode: str = "reranking_model",
        weights: WeightsDict | None = None,
        document_ids_filter: list[str] | None = None,
        attachment_ids: list[str] | None = None,
    ):
        if not query and not attachment_ids:
            return []
        dataset = cls._get_dataset(dataset_id)
        if not dataset:
            return []

        all_documents: list[Document] = []
        exceptions: list[str] = []

        # Optimize multithreading with thread pools
        with ThreadPoolExecutor(max_workers=dify_config.RETRIEVAL_SERVICE_EXECUTORS) as executor:  # type: ignore
            futures = []
            retrieval_service = RetrievalService()
            if query:
                futures.append(
                    executor.submit(
                        retrieval_service._retrieve,
                        flask_app=current_app._get_current_object(),  # type: ignore
                        retrieval_method=retrieval_method,
                        dataset=dataset,
                        query=query,
                        top_k=top_k,
                        score_threshold=score_threshold,
                        reranking_model=reranking_model,
                        reranking_mode=reranking_mode,
                        weights=weights,
                        document_ids_filter=document_ids_filter,
                        attachment_id=None,
                        all_documents=all_documents,
                        exceptions=exceptions,
                    )
                )
            if attachment_ids:
                for attachment_id in attachment_ids:
                    futures.append(
                        executor.submit(
                            retrieval_service._retrieve,
                            flask_app=current_app._get_current_object(),  # type: ignore
                            retrieval_method=retrieval_method,
                            dataset=dataset,
                            query=None,
                            top_k=top_k,
                            score_threshold=score_threshold,
                            reranking_model=reranking_model,
                            reranking_mode=reranking_mode,
                            weights=weights,
                            document_ids_filter=document_ids_filter,
                            attachment_id=attachment_id,
                            all_documents=all_documents,
                            exceptions=exceptions,
                        )
                    )

            if futures:
                for future in concurrent.futures.as_completed(futures, timeout=3600):
                    if exceptions:
                        for f in futures:
                            f.cancel()
                        break

        if exceptions:
            raise ValueError(";\n".join(exceptions))

        return all_documents

    @classmethod
    def external_retrieve(
        cls,
        dataset_id: str,
        query: str,
        external_retrieval_model: dict | None = None,
        metadata_filtering_conditions: dict | None = None,
    ):
        stmt = select(Dataset).where(Dataset.id == dataset_id)
        dataset = db.session.scalar(stmt)
        if not dataset:
            return []
        metadata_condition = (
            MetadataCondition.model_validate(metadata_filtering_conditions) if metadata_filtering_conditions else None
        )
        all_documents = ExternalDatasetService.fetch_external_knowledge_retrieval(
            dataset.tenant_id,
            dataset_id,
            query,
            external_retrieval_model or {},
            metadata_condition=metadata_condition,
        )
        return all_documents

    @classmethod
    def _deduplicate_documents(cls, documents: list[Document]) -> list[Document]:
        """Deduplicate documents in O(n) while preserving first-seen order.

        Rules:
        - For provider == "dify" and metadata["doc_id"] exists: keep the doc with the highest
          metadata["score"] among duplicates; if a later duplicate has no score, ignore it.
        - For non-dify documents (or dify without doc_id): deduplicate by content key
          (provider, page_content), keeping the first occurrence.
        """
        if not documents:
            return documents

        # Map of dedup key -> chosen Document
        chosen: dict[tuple, Document] = {}
        # Preserve the order of first appearance of each dedup key
        order: list[tuple] = []

        for doc in documents:
            is_dify = doc.provider == "dify"
            doc_id = (doc.metadata or {}).get("doc_id") if is_dify else None

            if is_dify and doc_id:
                key = ("dify", doc_id)
                if key not in chosen:
                    chosen[key] = doc
                    order.append(key)
                else:
                    # Only replace if the new one has a score and it's strictly higher
                    if "score" in doc.metadata:
                        new_score = float(doc.metadata.get("score", 0.0))
                        old_score = float(chosen[key].metadata.get("score", 0.0)) if chosen[key].metadata else 0.0
                        if new_score > old_score:
                            chosen[key] = doc
            else:
                # Content-based dedup for non-dify or dify without doc_id
                content_key = (doc.provider or "dify", doc.page_content)
                if content_key not in chosen:
                    chosen[content_key] = doc
                    order.append(content_key)
                # If duplicate content appears, we keep the first occurrence (no score comparison)

        return [chosen[k] for k in order]

    @classmethod
    def _get_dataset(cls, dataset_id: str) -> Dataset | None:
        with Session(db.engine) as session:
            return session.scalar(select(Dataset).where(Dataset.id == dataset_id).limit(1))

    @classmethod
    def keyword_search(
        cls,
        flask_app: Flask,
        dataset_id: str,
        query: str,
        top_k: int,
        all_documents: list[Document],
        exceptions: list[str],
        document_ids_filter: list[str] | None = None,
    ):
        with flask_app.app_context():
            try:
                dataset = cls._get_dataset(dataset_id)
                if not dataset:
                    raise ValueError("dataset not found")

                keyword = Keyword(dataset=dataset)

                documents = keyword.search(
                    cls.escape_query_for_search(query), top_k=top_k, document_ids_filter=document_ids_filter
                )
                all_documents.extend(documents)
            except Exception as e:
                logger.error(e, exc_info=True)
                exceptions.append(str(e))

    @classmethod
    def embedding_search(
        cls,
        flask_app: Flask,
        dataset_id: str,
        query: str,
        top_k: int,
        score_threshold: float | None,
        reranking_model: RerankingModelDict | None,
        all_documents: list[Document],
        retrieval_method: RetrievalMethod,
        exceptions: list[str],
        document_ids_filter: list[str] | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ):
        with flask_app.app_context():
            try:
                dataset = cls._get_dataset(dataset_id)
                if not dataset:
                    raise ValueError("dataset not found")

                vector = Vector(dataset=dataset)
                documents = []
                if query_type == QueryType.TEXT_QUERY:
                    documents.extend(
                        vector.search_by_vector(
                            query,
                            search_type="similarity_score_threshold",
                            top_k=top_k,
                            score_threshold=score_threshold,
                            filter={"group_id": [dataset.id]},
                            document_ids_filter=document_ids_filter,
                        )
                    )
                if query_type == QueryType.IMAGE_QUERY:
                    if not dataset.is_multimodal:
                        return
                    documents.extend(
                        vector.search_by_file(
                            file_id=query,
                            top_k=top_k,
                            score_threshold=score_threshold,
                            filter={"group_id": [dataset.id]},
                            document_ids_filter=document_ids_filter,
                        )
                    )

                if documents:
                    if (
                        reranking_model
                        and reranking_model["reranking_model_name"]
                        and reranking_model["reranking_provider_name"]
                        and retrieval_method == RetrievalMethod.SEMANTIC_SEARCH
                    ):
                        data_post_processor = DataPostProcessor(
                            str(dataset.tenant_id), str(RerankMode.RERANKING_MODEL), reranking_model, None, False
                        )
                        if dataset.is_multimodal:
                            model_manager = ModelManager.for_tenant(tenant_id=dataset.tenant_id)
                            is_support_vision = model_manager.check_model_support_vision(
                                tenant_id=dataset.tenant_id,
                                provider=reranking_model["reranking_provider_name"],
                                model=reranking_model["reranking_model_name"],
                                model_type=ModelType.RERANK,
                            )
                            if is_support_vision:
                                all_documents.extend(
                                    data_post_processor.invoke(
                                        query=query,
                                        documents=documents,
                                        score_threshold=score_threshold,
                                        top_n=len(documents),
                                        query_type=query_type,
                                    )
                                )
                            else:
                                # not effective, return original documents
                                all_documents.extend(documents)
                        else:
                            all_documents.extend(
                                data_post_processor.invoke(
                                    query=query,
                                    documents=documents,
                                    score_threshold=score_threshold,
                                    top_n=len(documents),
                                    query_type=query_type,
                                )
                            )
                    else:
                        all_documents.extend(documents)
            except Exception as e:
                logger.error(e, exc_info=True)
                exceptions.append(str(e))

    @classmethod
    def full_text_index_search(
        cls,
        flask_app: Flask,
        dataset_id: str,
        query: str,
        top_k: int,
        score_threshold: float | None,
        reranking_model: RerankingModelDict | None,
        all_documents: list[Document],
        retrieval_method: str,
        exceptions: list[str],
        document_ids_filter: list[str] | None = None,
    ):
        with flask_app.app_context():
            try:
                dataset = cls._get_dataset(dataset_id)
                if not dataset:
                    raise ValueError("dataset not found")

                vector_processor = Vector(dataset=dataset)

                documents = vector_processor.search_by_full_text(
                    cls.escape_query_for_search(query), top_k=top_k, document_ids_filter=document_ids_filter
                )
                if documents:
                    if (
                        reranking_model
                        and reranking_model["reranking_model_name"]
                        and reranking_model["reranking_provider_name"]
                        and retrieval_method == RetrievalMethod.FULL_TEXT_SEARCH
                    ):
                        data_post_processor = DataPostProcessor(
                            str(dataset.tenant_id), str(RerankMode.RERANKING_MODEL), reranking_model, None, False
                        )
                        all_documents.extend(
                            data_post_processor.invoke(
                                query=query,
                                documents=documents,
                                score_threshold=score_threshold,
                                top_n=len(documents),
                            )
                        )
                    else:
                        all_documents.extend(documents)
            except Exception as e:
                logger.error(e, exc_info=True)
                exceptions.append(str(e))

    @staticmethod
    def escape_query_for_search(query: str) -> str:
        return query.replace('"', '\\"')

    @classmethod
    def format_retrieval_documents(cls, documents: list[Document]) -> list[RetrievalSegments]:
        """Format retrieval documents with optimized batch processing"""
        if not documents:
            return []

        try:
            # Collect document IDs
            document_ids = {doc.metadata.get("document_id") for doc in documents if "document_id" in doc.metadata}
            if not document_ids:
                return []

            # Batch query dataset documents
            dataset_documents = {
                doc.id: doc
                for doc in db.session.scalars(
                    select(DatasetDocument)
                    .where(DatasetDocument.id.in_(document_ids))
                    .options(load_only(DatasetDocument.id, DatasetDocument.doc_form, DatasetDocument.dataset_id))
                ).all()
            }

            valid_dataset_documents = {}
            image_doc_ids: list[Any] = []
            child_index_node_ids = []
            index_node_ids = []
            doc_to_document_map = {}
            summary_segment_ids = set()  # Track segments retrieved via summary
            summary_score_map: dict[str, float] = {}  # Map original_chunk_id to summary score

            # First pass: collect all document IDs and identify summary documents
            for document in documents:
                document_id = document.metadata.get("document_id")
                if document_id not in dataset_documents:
                    continue

                dataset_document = dataset_documents[document_id]
                if not dataset_document:
                    continue
                valid_dataset_documents[document_id] = dataset_document

                doc_id = document.metadata.get("doc_id") or ""
                doc_to_document_map[doc_id] = document

                # Check if this is a summary document
                is_summary = document.metadata.get("is_summary", False)
                if is_summary:
                    # For summary documents, find the original chunk via original_chunk_id
                    original_chunk_id = document.metadata.get("original_chunk_id")
                    if original_chunk_id:
                        summary_segment_ids.add(original_chunk_id)
                        # Save summary's score for later use
                        summary_score = document.metadata.get("score")
                        if summary_score is not None:
                            try:
                                summary_score_float = float(summary_score)
                                # If the same segment has multiple summary hits, take the highest score
                                if original_chunk_id not in summary_score_map:
                                    summary_score_map[original_chunk_id] = summary_score_float
                                else:
                                    summary_score_map[original_chunk_id] = max(
                                        summary_score_map[original_chunk_id], summary_score_float
                                    )
                            except (ValueError, TypeError):
                                # Skip invalid score values
                                pass
                    continue  # Skip adding to other lists for summary documents

                if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                    if document.metadata.get("doc_type") == DocType.IMAGE:
                        image_doc_ids.append(doc_id)
                    else:
                        child_index_node_ids.append(doc_id)
                else:
                    if document.metadata.get("doc_type") == DocType.IMAGE:
                        image_doc_ids.append(doc_id)
                    else:
                        index_node_ids.append(doc_id)

            image_doc_ids = [i for i in image_doc_ids if i]
            child_index_node_ids = [i for i in child_index_node_ids if i]
            index_node_ids = [i for i in index_node_ids if i]

            segment_ids: list[str] = []
            index_node_segments: list[DocumentSegment] = []
            segments: list[DocumentSegment] = []
            attachment_map: dict[str, list[AttachmentInfoDict]] = {}
            child_chunk_map: dict[str, list[ChildChunk]] = {}
            doc_segment_map: dict[str, list[str]] = {}
            segment_summary_map: dict[str, str] = {}  # Map segment_id to summary content

            with session_factory.create_session() as session:
                attachments = cls.get_segment_attachment_infos(image_doc_ids, session)

                for attachment in attachments:
                    segment_ids.append(attachment["segment_id"])
                    if attachment["segment_id"] in attachment_map:
                        attachment_map[attachment["segment_id"]].append(attachment["attachment_info"])
                    else:
                        attachment_map[attachment["segment_id"]] = [attachment["attachment_info"]]
                    if attachment["segment_id"] in doc_segment_map:
                        doc_segment_map[attachment["segment_id"]].append(attachment["attachment_id"])
                    else:
                        doc_segment_map[attachment["segment_id"]] = [attachment["attachment_id"]]

                child_chunk_stmt = select(ChildChunk).where(ChildChunk.index_node_id.in_(child_index_node_ids))
                child_index_nodes = session.execute(child_chunk_stmt).scalars().all()

                for i in child_index_nodes:
                    segment_ids.append(i.segment_id)
                    if i.segment_id in child_chunk_map:
                        child_chunk_map[i.segment_id].append(i)
                    else:
                        child_chunk_map[i.segment_id] = [i]
                    if i.segment_id in doc_segment_map:
                        doc_segment_map[i.segment_id].append(i.index_node_id)
                    else:
                        doc_segment_map[i.segment_id] = [i.index_node_id]

                if index_node_ids:
                    document_segment_stmt = select(DocumentSegment).where(
                        DocumentSegment.enabled == True,
                        DocumentSegment.status == "completed",
                        DocumentSegment.index_node_id.in_(index_node_ids),
                    )
                    index_node_segments = session.execute(document_segment_stmt).scalars().all()  # type: ignore
                    for index_node_segment in index_node_segments:
                        doc_segment_map[index_node_segment.id] = [index_node_segment.index_node_id]

                if segment_ids:
                    document_segment_stmt = select(DocumentSegment).where(
                        DocumentSegment.enabled == True,
                        DocumentSegment.status == "completed",
                        DocumentSegment.id.in_(segment_ids),
                    )
                    segments = session.execute(document_segment_stmt).scalars().all()  # type: ignore

                if index_node_segments:
                    segments.extend(index_node_segments)

                # Handle summary documents: query segments by original_chunk_id
                if summary_segment_ids:
                    summary_segment_ids_list = list(summary_segment_ids)
                    summary_segment_stmt = select(DocumentSegment).where(
                        DocumentSegment.enabled == True,
                        DocumentSegment.status == "completed",
                        DocumentSegment.id.in_(summary_segment_ids_list),
                    )
                    summary_segments = session.execute(summary_segment_stmt).scalars().all()  # type: ignore
                    segments.extend(summary_segments)
                    # Add summary segment IDs to segment_ids for summary query
                    for seg in summary_segments:
                        if seg.id not in segment_ids:
                            segment_ids.append(seg.id)

                # Batch query summaries for segments retrieved via summary (only enabled summaries)
                if summary_segment_ids:
                    summaries = session.scalars(
                        select(DocumentSegmentSummary).where(
                            DocumentSegmentSummary.chunk_id.in_(list(summary_segment_ids)),
                            DocumentSegmentSummary.status == "completed",
                            DocumentSegmentSummary.enabled.is_(True),  # Only retrieve enabled summaries
                        )
                    ).all()
                    for summary in summaries:
                        if summary.summary_content:
                            segment_summary_map[summary.chunk_id] = summary.summary_content

            include_segment_ids = set()
            segment_child_map: dict[str, SegmentChildMapDetail] = {}
            records: list[SegmentRecord] = []

            for segment in segments:
                child_chunks: list[ChildChunk] = child_chunk_map.get(segment.id, [])
                attachment_infos: list[AttachmentInfoDict] = attachment_map.get(segment.id, [])
                ds_dataset_document: DatasetDocument | None = valid_dataset_documents.get(segment.document_id)

                if ds_dataset_document and ds_dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                    if segment.id not in include_segment_ids:
                        include_segment_ids.add(segment.id)
                        # Check if this segment was retrieved via summary
                        # Use summary score as base score if available, otherwise 0.0
                        max_score = summary_score_map.get(segment.id, 0.0)

                        if child_chunks or attachment_infos:
                            child_chunk_details: list[ChildChunkDetail] = []
                            for child_chunk in child_chunks:
                                child_document: Document | None = doc_to_document_map.get(child_chunk.index_node_id)
                                if child_document:
                                    child_score = child_document.metadata.get("score", 0.0)
                                else:
                                    child_score = 0.0
                                child_chunk_detail: ChildChunkDetail = {
                                    "id": child_chunk.id,
                                    "content": child_chunk.content,
                                    "position": child_chunk.position,
                                    "score": child_score,
                                }
                                child_chunk_details.append(child_chunk_detail)
                                max_score = max(max_score, child_score)
                            for attachment_info in attachment_infos:
                                file_document = doc_to_document_map.get(attachment_info["id"])
                                if file_document:
                                    max_score = max(max_score, file_document.metadata.get("score", 0.0))

                            map_detail: SegmentChildMapDetail = {
                                "max_score": max_score,
                                "child_chunks": child_chunk_details,
                            }
                            segment_child_map[segment.id] = map_detail
                        else:
                            # No child chunks or attachments, use summary score if available
                            summary_score = summary_score_map.get(segment.id)
                            if summary_score is not None:
                                segment_child_map[segment.id] = {
                                    "max_score": summary_score,
                                    "child_chunks": [],
                                }
                        record: SegmentRecord = {
                            "segment": segment,
                        }
                        records.append(record)
                else:
                    if segment.id not in include_segment_ids:
                        include_segment_ids.add(segment.id)

                        # Check if this segment was retrieved via summary
                        # Use summary score if available (summary retrieval takes priority)
                        max_score = summary_score_map.get(segment.id, 0.0)

                        # If not retrieved via summary, use original segment's score
                        if segment.id not in summary_score_map:
                            segment_document = doc_to_document_map.get(segment.index_node_id)
                            if segment_document:
                                max_score = max(max_score, segment_document.metadata.get("score", 0.0))

                        # Also consider attachment scores
                        for attachment_info in attachment_infos:
                            file_doc = doc_to_document_map.get(attachment_info["id"])
                            if file_doc:
                                max_score = max(max_score, file_doc.metadata.get("score", 0.0))

                        another_record: SegmentRecord = {
                            "segment": segment,
                            "score": max_score,
                        }
                        records.append(another_record)

            # Add child chunks information to records
            for record in records:
                if record["segment"].id in segment_child_map:
                    record["child_chunks"] = segment_child_map[record["segment"].id]["child_chunks"]
                    record["score"] = segment_child_map[record["segment"].id]["max_score"]
                if record["segment"].id in attachment_map:
                    record["files"] = attachment_map[record["segment"].id]

            result: list[RetrievalSegments] = []
            for record in records:
                # Extract segment
                segment = record["segment"]

                # Extract child_chunks, ensuring it's a list or None
                raw_child_chunks = record.get("child_chunks")
                child_chunks_list: list[RetrievalChildChunk] | None = None
                if isinstance(raw_child_chunks, list):
                    # Sort by score descending
                    sorted_chunks = sorted(raw_child_chunks, key=lambda x: x.get("score", 0.0), reverse=True)
                    child_chunks_list = [
                        RetrievalChildChunk(
                            id=chunk["id"],
                            content=chunk["content"],
                            score=chunk.get("score", 0.0),
                            position=chunk["position"],
                        )
                        for chunk in sorted_chunks
                    ]

                # Extract files, ensuring it's a list or None
                files = record.get("files")
                if not isinstance(files, list):
                    files = None

                # Extract score, ensuring it's a float or None
                score_value = record.get("score")
                score = (
                    float(score_value)
                    if score_value is not None and isinstance(score_value, int | float | str)
                    else None
                )

                # Extract summary if this segment was retrieved via summary
                summary_content = segment_summary_map.get(segment.id)

                # Create RetrievalSegments object
                retrieval_segment = RetrievalSegments(
                    segment=segment,
                    child_chunks=child_chunks_list,
                    score=score,
                    files=files,
                    summary=summary_content,
                )
                result.append(retrieval_segment)

            return sorted(result, key=lambda x: x.score if x.score is not None else 0.0, reverse=True)
        except Exception as e:
            db.session.rollback()
            raise e

    def _retrieve(
        self,
        flask_app: Flask,
        retrieval_method: RetrievalMethod,
        dataset: Dataset,
        all_documents: list[Document],
        exceptions: list[str],
        query: str | None = None,
        top_k: int = 4,
        score_threshold: float | None = 0.0,
        reranking_model: RerankingModelDict | None = None,
        reranking_mode: str = "reranking_model",
        weights: WeightsDict | None = None,
        document_ids_filter: list[str] | None = None,
        attachment_id: str | None = None,
    ):
        if not query and not attachment_id:
            return
        with flask_app.app_context():
            all_documents_item: list[Document] = []
            # Optimize multithreading with thread pools
            with ThreadPoolExecutor(max_workers=dify_config.RETRIEVAL_SERVICE_EXECUTORS) as executor:  # type: ignore
                futures = []
                if retrieval_method == RetrievalMethod.KEYWORD_SEARCH and query:
                    futures.append(
                        executor.submit(
                            self.keyword_search,
                            flask_app=current_app._get_current_object(),  # type: ignore
                            dataset_id=dataset.id,
                            query=query,
                            top_k=top_k,
                            all_documents=all_documents_item,
                            exceptions=exceptions,
                            document_ids_filter=document_ids_filter,
                        )
                    )
                if RetrievalMethod.is_support_semantic_search(retrieval_method):
                    if query:
                        futures.append(
                            executor.submit(
                                self.embedding_search,
                                flask_app=current_app._get_current_object(),  # type: ignore
                                dataset_id=dataset.id,
                                query=query,
                                top_k=top_k,
                                score_threshold=score_threshold,
                                reranking_model=reranking_model,
                                all_documents=all_documents_item,
                                retrieval_method=retrieval_method,
                                exceptions=exceptions,
                                document_ids_filter=document_ids_filter,
                                query_type=QueryType.TEXT_QUERY,
                            )
                        )
                    if attachment_id:
                        futures.append(
                            executor.submit(
                                self.embedding_search,
                                flask_app=current_app._get_current_object(),  # type: ignore
                                dataset_id=dataset.id,
                                query=attachment_id,
                                top_k=top_k,
                                score_threshold=score_threshold,
                                reranking_model=reranking_model,
                                all_documents=all_documents_item,
                                retrieval_method=retrieval_method,
                                exceptions=exceptions,
                                document_ids_filter=document_ids_filter,
                                query_type=QueryType.IMAGE_QUERY,
                            )
                        )
                if RetrievalMethod.is_support_fulltext_search(retrieval_method) and query:
                    futures.append(
                        executor.submit(
                            self.full_text_index_search,
                            flask_app=current_app._get_current_object(),  # type: ignore
                            dataset_id=dataset.id,
                            query=query,
                            top_k=top_k,
                            score_threshold=score_threshold,
                            reranking_model=reranking_model,
                            all_documents=all_documents_item,
                            retrieval_method=retrieval_method,
                            exceptions=exceptions,
                            document_ids_filter=document_ids_filter,
                        )
                    )
                # Use as_completed for early error propagation - cancel remaining futures on first error
                if futures:
                    for future in concurrent.futures.as_completed(futures, timeout=300):
                        if future.exception():
                            # Cancel remaining futures to avoid unnecessary waiting
                            for f in futures:
                                f.cancel()
                            break

            if exceptions:
                raise ValueError(";\n".join(exceptions))

            # Deduplicate documents for hybrid search to avoid duplicate chunks
            if retrieval_method == RetrievalMethod.HYBRID_SEARCH:
                if attachment_id and reranking_mode == RerankMode.WEIGHTED_SCORE:
                    all_documents.extend(all_documents_item)
                all_documents_item = self._deduplicate_documents(all_documents_item)
                data_post_processor = DataPostProcessor(
                    str(dataset.tenant_id), reranking_mode, reranking_model, weights, False
                )

                query = query or attachment_id
                if not query:
                    return
                all_documents_item = data_post_processor.invoke(
                    query=query,
                    documents=all_documents_item,
                    score_threshold=score_threshold,
                    top_n=top_k,
                    query_type=QueryType.TEXT_QUERY if query else QueryType.IMAGE_QUERY,
                )

            all_documents.extend(all_documents_item)

    @classmethod
    def get_segment_attachment_info(
        cls, dataset_id: str, tenant_id: str, attachment_id: str, session: Session
    ) -> SegmentAttachmentResult | None:
        upload_file = session.scalar(select(UploadFile).where(UploadFile.id == attachment_id).limit(1))
        if upload_file:
            attachment_binding = session.scalar(
                select(SegmentAttachmentBinding)
                .where(SegmentAttachmentBinding.attachment_id == upload_file.id)
                .limit(1)
            )
            if attachment_binding:
                attachment_info: AttachmentInfoDict = {
                    "id": upload_file.id,
                    "name": upload_file.name,
                    "extension": "." + upload_file.extension,
                    "mime_type": upload_file.mime_type,
                    "source_url": sign_upload_file(upload_file.id, upload_file.extension),
                    "size": upload_file.size,
                }
                return {"attachment_info": attachment_info, "segment_id": attachment_binding.segment_id}
        return None

    @classmethod
    def get_segment_attachment_infos(
        cls, attachment_ids: list[str], session: Session
    ) -> list[SegmentAttachmentInfoResult]:
        attachment_infos: list[SegmentAttachmentInfoResult] = []
        upload_files = session.scalars(select(UploadFile).where(UploadFile.id.in_(attachment_ids))).all()
        if upload_files:
            upload_file_ids = [upload_file.id for upload_file in upload_files]
            attachment_bindings = session.scalars(
                select(SegmentAttachmentBinding).where(SegmentAttachmentBinding.attachment_id.in_(upload_file_ids))
            ).all()
            attachment_binding_map = {binding.attachment_id: binding for binding in attachment_bindings}

            if attachment_bindings:
                for upload_file in upload_files:
                    attachment_binding = attachment_binding_map.get(upload_file.id)
                    info: AttachmentInfoDict = {
                        "id": upload_file.id,
                        "name": upload_file.name,
                        "extension": "." + upload_file.extension,
                        "mime_type": upload_file.mime_type,
                        "source_url": sign_upload_file(upload_file.id, upload_file.extension),
                        "size": upload_file.size,
                    }
                    if attachment_binding:
                        attachment_infos.append(
                            {
                                "attachment_id": attachment_binding.attachment_id,
                                "attachment_info": info,
                                "segment_id": attachment_binding.segment_id,
                            }
                        )
        return attachment_infos

```

### api/core/rag/pipeline/queue.py
```py
from __future__ import annotations

import json
from collections.abc import Sequence
from typing import Any

from pydantic import BaseModel, ValidationError

from extensions.ext_redis import redis_client

_DEFAULT_TASK_TTL = 60 * 60  # 1 hour


class TaskWrapper(BaseModel):
    data: Any

    def serialize(self) -> str:
        return self.model_dump_json()

    @classmethod
    def deserialize(cls, serialized_data: str) -> TaskWrapper:
        return cls.model_validate_json(serialized_data)


class TenantIsolatedTaskQueue:
    """
    Simple queue for tenant isolated tasks, used for rag related tenant tasks isolation.
    It uses Redis list to store tasks, and Redis key to store task waiting flag.
    Support tasks that can be serialized by json.
    """

    def __init__(self, tenant_id: str, unique_key: str):
        self._tenant_id = tenant_id
        self._unique_key = unique_key
        self._queue = f"tenant_self_{unique_key}_task_queue:{tenant_id}"
        self._task_key = f"tenant_{unique_key}_task:{tenant_id}"

    def get_task_key(self):
        return redis_client.get(self._task_key)

    def set_task_waiting_time(self, ttl: int = _DEFAULT_TASK_TTL):
        redis_client.setex(self._task_key, ttl, 1)

    def delete_task_key(self):
        redis_client.delete(self._task_key)

    def push_tasks(self, tasks: Sequence[Any]):
        serialized_tasks = []
        for task in tasks:
            # Store str list directly, maintaining full compatibility for pipeline scenarios
            if isinstance(task, str):
                serialized_tasks.append(task)
            else:
                # Use TaskWrapper to do JSON serialization for non-string tasks
                wrapper = TaskWrapper(data=task)
                serialized_data = wrapper.serialize()
                serialized_tasks.append(serialized_data)

        if not serialized_tasks:
            return

        redis_client.lpush(self._queue, *serialized_tasks)

    def pull_tasks(self, count: int = 1) -> Sequence[Any]:
        if count <= 0:
            return []

        tasks = []
        for _ in range(count):
            serialized_task = redis_client.rpop(self._queue)
            if not serialized_task:
                break

            if isinstance(serialized_task, bytes):
                serialized_task = serialized_task.decode("utf-8")

            try:
                wrapper = TaskWrapper.deserialize(serialized_task)
                tasks.append(wrapper.data)
            except (json.JSONDecodeError, ValidationError, TypeError, ValueError):
                # Fall back to raw string for legacy format or invalid JSON
                tasks.append(serialized_task)

        return tasks

```

### api/core/rag/pipeline/__init__.py
```py

```

### api/core/rag/index_processor/index_processor_factory.py
```py
"""Abstract interface for document loader implementations."""

from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_base import BaseIndexProcessor
from core.rag.index_processor.processor.paragraph_index_processor import ParagraphIndexProcessor
from core.rag.index_processor.processor.parent_child_index_processor import ParentChildIndexProcessor
from core.rag.index_processor.processor.qa_index_processor import QAIndexProcessor


class IndexProcessorFactory:
    """IndexProcessorInit."""

    def __init__(self, index_type: str | None):
        self._index_type = index_type

    def init_index_processor(self) -> BaseIndexProcessor:
        """Init index processor."""

        if not self._index_type:
            raise ValueError("Index type must be specified.")

        if self._index_type == IndexStructureType.PARAGRAPH_INDEX:
            return ParagraphIndexProcessor()
        elif self._index_type == IndexStructureType.QA_INDEX:
            return QAIndexProcessor()
        elif self._index_type == IndexStructureType.PARENT_CHILD_INDEX:
            return ParentChildIndexProcessor()
        else:
            raise ValueError(f"Index type {self._index_type} is not supported.")

```

### api/core/rag/index_processor/processor/paragraph_index_processor.py
```py
"""Paragraph index processor."""

import logging
import re
import uuid
from collections.abc import Mapping
from typing import Any, cast

logger = logging.getLogger(__name__)

from graphon.file import File, FileTransferMethod, FileType, file_manager
from graphon.model_runtime.entities.llm_entities import LLMResult, LLMUsage
from graphon.model_runtime.entities.message_entities import (
    ImagePromptMessageContent,
    PromptMessage,
    PromptMessageContentUnionTypes,
    TextPromptMessageContent,
    UserPromptMessage,
)
from graphon.model_runtime.entities.model_entities import ModelFeature, ModelType
from sqlalchemy import select

from core.app.file_access import DatabaseFileAccessController
from core.app.llm import deduct_llm_quota
from core.entities.knowledge_entities import PreviewDetail
from core.llm_generator.prompts import DEFAULT_GENERATOR_SUMMARY_PROMPT
from core.model_manager import ModelInstance
from core.plugin.impl.model_runtime_factory import create_plugin_provider_manager
from core.rag.cleaner.clean_processor import CleanProcessor
from core.rag.data_post_processor.data_post_processor import RerankingModelDict
from core.rag.datasource.keyword.keyword_factory import Keyword
from core.rag.datasource.retrieval_service import RetrievalService
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.docstore.dataset_docstore import DatasetDocumentStore
from core.rag.extractor.entity.extract_setting import ExtractSetting
from core.rag.extractor.extract_processor import ExtractProcessor
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from core.rag.index_processor.index_processor_base import BaseIndexProcessor, SummaryIndexSettingDict
from core.rag.models.document import AttachmentDocument, Document, MultimodalGeneralStructureChunk
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.tools.utils.text_processing_utils import remove_leading_symbols
from core.workflow.file_reference import build_file_reference
from extensions.ext_database import db
from factories.file_factory import build_from_mapping
from libs import helper
from models import UploadFile
from models.account import Account
from models.dataset import Dataset, DatasetProcessRule, DocumentSegment, SegmentAttachmentBinding
from models.dataset import Document as DatasetDocument
from services.account_service import AccountService
from services.entities.knowledge_entities.knowledge_entities import Rule
from services.summary_index_service import SummaryIndexService

_file_access_controller = DatabaseFileAccessController()


class ParagraphIndexProcessor(BaseIndexProcessor):
    def extract(self, extract_setting: ExtractSetting, **kwargs) -> list[Document]:
        text_docs = ExtractProcessor.extract(
            extract_setting=extract_setting,
            is_automatic=(
                kwargs.get("process_rule_mode") == "automatic" or kwargs.get("process_rule_mode") == "hierarchical"
            ),
        )

        return text_docs

    def transform(self, documents: list[Document], current_user: Account | None = None, **kwargs) -> list[Document]:
        process_rule = kwargs.get("process_rule")
        if not process_rule:
            raise ValueError("No process rule found.")
        if process_rule.get("mode") == "automatic":
            automatic_rule = DatasetProcessRule.AUTOMATIC_RULES
            rules = Rule.model_validate(automatic_rule)
        else:
            if not process_rule.get("rules"):
                raise ValueError("No rules found in process rule.")
            rules = Rule.model_validate(process_rule.get("rules"))
        # Split the text documents into nodes.
        if not rules.segmentation:
            raise ValueError("No segmentation found in rules.")
        splitter = self._get_splitter(
            processing_rule_mode=process_rule.get("mode"),
            max_tokens=rules.segmentation.max_tokens,
            chunk_overlap=rules.segmentation.chunk_overlap,
            separator=rules.segmentation.separator,
            embedding_model_instance=kwargs.get("embedding_model_instance"),
        )
        all_documents = []
        for document in documents:
            # document clean
            document_text = CleanProcessor.clean(document.page_content, kwargs.get("process_rule", {}))
            document.page_content = document_text
            # parse document to nodes
            document_nodes = splitter.split_documents([document])
            split_documents = []
            for document_node in document_nodes:
                if document_node.page_content.strip():
                    doc_id = str(uuid.uuid4())
                    hash = helper.generate_text_hash(document_node.page_content)
                    if document_node.metadata is not None:
                        document_node.metadata["doc_id"] = doc_id
                        document_node.metadata["doc_hash"] = hash
                    multimodal_documents = (
                        self._get_content_files(document_node, current_user) if document_node.metadata else None
                    )
                    if multimodal_documents:
                        document_node.attachments = multimodal_documents
                    # delete Splitter character
                    page_content = remove_leading_symbols(document_node.page_content).strip()
                    if len(page_content) > 0:
                        document_node.page_content = page_content
                        split_documents.append(document_node)
            all_documents.extend(split_documents)
        return all_documents

    def load(
        self,
        dataset: Dataset,
        documents: list[Document],
        multimodal_documents: list[AttachmentDocument] | None = None,
        with_keywords: bool = True,
        **kwargs,
    ) -> None:
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            vector = Vector(dataset)
            vector.create(documents)
            if multimodal_documents and dataset.is_multimodal:
                vector.create_multimodal(multimodal_documents)
            with_keywords = False
        if with_keywords:
            keywords_list = kwargs.get("keywords_list")
            keyword = Keyword(dataset)
            if keywords_list and len(keywords_list) > 0:
                keyword.add_texts(documents, keywords_list=keywords_list)
            else:
                keyword.add_texts(documents)

    def clean(self, dataset: Dataset, node_ids: list[str] | None, with_keywords: bool = True, **kwargs) -> None:
        # Note: Summary indexes are now disabled (not deleted) when segments are disabled.
        # This method is called for actual deletion scenarios (e.g., when segment is deleted).
        # For disable operations, disable_summaries_for_segments is called directly in the task.
        # Only delete summaries if explicitly requested (e.g., when segment is actually deleted)
        delete_summaries = kwargs.get("delete_summaries", False)
        if delete_summaries:
            if node_ids:
                # Find segments by index_node_id
                segments = db.session.scalars(
                    select(DocumentSegment).where(
                        DocumentSegment.dataset_id == dataset.id,
                        DocumentSegment.index_node_id.in_(node_ids),
                    )
                ).all()
                segment_ids = [segment.id for segment in segments]
                if segment_ids:
                    SummaryIndexService.delete_summaries_for_segments(dataset, segment_ids)
            else:
                # Delete all summaries for the dataset
                SummaryIndexService.delete_summaries_for_segments(dataset, None)

        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            vector = Vector(dataset)
            if node_ids:
                vector.delete_by_ids(node_ids)
            else:
                vector.delete()
            with_keywords = False
        if with_keywords:
            keyword = Keyword(dataset)
            if node_ids:
                keyword.delete_by_ids(node_ids)
            else:
                keyword.delete()

    def retrieve(
        self,
        retrieval_method: RetrievalMethod,
        query: str,
        dataset: Dataset,
        top_k: int,
        score_threshold: float,
        reranking_model: RerankingModelDict,
    ) -> list[Document]:
        # Set search parameters.
        results = RetrievalService.retrieve(
            retrieval_method=retrieval_method,
            dataset_id=dataset.id,
            query=query,
            top_k=top_k,
            score_threshold=score_threshold,
            reranking_model=reranking_model,
        )
        # Organize results.
        docs = []
        for result in results:
            metadata = result.metadata
            metadata["score"] = result.score
            if result.score >= score_threshold:
                doc = Document(page_content=result.page_content, metadata=metadata)
                docs.append(doc)
        return docs

    def index(self, dataset: Dataset, document: DatasetDocument, chunks: Any) -> None:
        documents: list[Any] = []
        all_multimodal_documents: list[Any] = []
        if isinstance(chunks, list):
            for content in chunks:
                metadata = {
                    "dataset_id": dataset.id,
                    "document_id": document.id,
                    "doc_id": str(uuid.uuid4()),
                    "doc_hash": helper.generate_text_hash(content),
                }
                doc = Document(page_content=content, metadata=metadata)
                attachments = self._get_content_files(doc)
                if attachments:
                    doc.attachments = attachments
                    all_multimodal_documents.extend(attachments)
                documents.append(doc)
        else:
            multimodal_general_structure = MultimodalGeneralStructureChunk.model_validate(chunks)
            for general_chunk in multimodal_general_structure.general_chunks:
                metadata = {
                    "dataset_id": dataset.id,
                    "document_id": document.id,
                    "doc_id": str(uuid.uuid4()),
                    "doc_hash": helper.generate_text_hash(general_chunk.content),
                }
                doc = Document(page_content=general_chunk.content, metadata=metadata)
                if general_chunk.files:
                    attachments = []
                    for file in general_chunk.files:
                        file_metadata = {
                            "doc_id": file.id,
                            "doc_hash": "",
                            "document_id": document.id,
                            "dataset_id": dataset.id,
                            "doc_type": DocType.IMAGE,
                        }
                        file_document = AttachmentDocument(
                            page_content=file.filename or "image_file", metadata=file_metadata
                        )
                        attachments.append(file_document)
                        all_multimodal_documents.append(file_document)
                    doc.attachments = attachments
                else:
                    account = AccountService.load_user(document.created_by)
                    if not account:
                        raise ValueError("Invalid account")
                    doc.attachments = self._get_content_files(doc, current_user=account)
                    if doc.attachments:
                        all_multimodal_documents.extend(doc.attachments)
                documents.append(doc)
        if documents:
            # save node to document segment
            doc_store = DatasetDocumentStore(dataset=dataset, user_id=document.created_by, document_id=document.id)
            # add document segments
            doc_store.add_documents(docs=documents, save_child=False)
            if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                vector = Vector(dataset)
                vector.create(documents)
                if all_multimodal_documents and dataset.is_multimodal:
                    vector.create_multimodal(all_multimodal_documents)
            elif dataset.indexing_technique == IndexTechniqueType.ECONOMY:
                keyword = Keyword(dataset)
                keyword.add_texts(documents)

    def format_preview(self, chunks: Any) -> Mapping[str, Any]:
        if isinstance(chunks, list):
            preview = []
            for content in chunks:
                preview.append({"content": content})
            return {
                "chunk_structure": IndexStructureType.PARAGRAPH_INDEX,
                "preview": preview,
                "total_segments": len(chunks),
            }
        else:
            raise ValueError("Chunks is not a list")

    def generate_summary_preview(
        self,
        tenant_id: str,
        preview_texts: list[PreviewDetail],
        summary_index_setting: SummaryIndexSettingDict,
        doc_language: str | None = None,
    ) -> list[PreviewDetail]:
        """
        For each segment, concurrently call generate_summary to generate a summary
        and write it to the summary attribute of PreviewDetail.
        In preview mode (indexing-estimate), if any summary generation fails, the method will raise an exception.
        """
        import concurrent.futures

        from flask import current_app

        # Capture Flask app context for worker threads
        flask_app = None
        try:
            flask_app = current_app._get_current_object()  # type: ignore
        except RuntimeError:
            logger.warning("No Flask application context available, summary generation may fail")

        def process(preview: PreviewDetail) -> None:
            """Generate summary for a single preview item."""
            if flask_app:
                # Ensure Flask app context in worker thread
                with flask_app.app_context():
                    summary, _ = self.generate_summary(
                        tenant_id, preview.content, summary_index_setting, document_language=doc_language
                    )
                    preview.summary = summary
            else:
                # Fallback: try without app context (may fail)
                summary, _ = self.generate_summary(
                    tenant_id, preview.content, summary_index_setting, document_language=doc_language
                )
                preview.summary = summary

        # Generate summaries concurrently using ThreadPoolExecutor
        # Set a reasonable timeout to prevent hanging (60 seconds per chunk, max 5 minutes total)
        timeout_seconds = min(300, 60 * len(preview_texts))
        errors: list[Exception] = []

        with concurrent.futures.ThreadPoolExecutor(max_workers=min(10, len(preview_texts))) as executor:
            futures = [executor.submit(process, preview) for preview in preview_texts]
            # Wait for all tasks to complete with timeout
            done, not_done = concurrent.futures.wait(futures, timeout=timeout_seconds)

            # Cancel tasks that didn't complete in time
            if not_done:
                timeout_error_msg = (
                    f"Summary generation timeout: {len(not_done)} chunks did not complete within {timeout_seconds}s"
                )
                logger.warning("%s. Cancelling remaining tasks...", timeout_error_msg)
                # In preview mode, timeout is also an error
                errors.append(TimeoutError(timeout_error_msg))
                for future in not_done:
                    future.cancel()
                # Wait a bit for cancellation to take effect
                concurrent.futures.wait(not_done, timeout=5)

            # Collect exceptions from completed futures
            for future in done:
                try:
                    future.result()  # This will raise any exception that occurred
                except Exception as e:
                    logger.exception("Error in summary generation future")
                    errors.append(e)

        # In preview mode (indexing-estimate), if there are any errors, fail the request
        if errors:
            error_messages = [str(e) for e in errors]
            error_summary = (
                f"Failed to generate summaries for {len(errors)} chunk(s). "
                f"Errors: {'; '.join(error_messages[:3])}"  # Show first 3 errors
            )
            if len(errors) > 3:
                error_summary += f" (and {len(errors) - 3} more)"
            logger.error("Summary generation failed in preview mode: %s", error_summary)
            raise ValueError(error_summary)

        return preview_texts

    @staticmethod
    def generate_summary(
        tenant_id: str,
        text: str,
        summary_index_setting: SummaryIndexSettingDict | None = None,
        segment_id: str | None = None,
        document_language: str | None = None,
    ) -> tuple[str, LLMUsage]:
        """
        Generate summary for the given text using ModelInstance.invoke_llm and the default or custom summary prompt,
        and supports vision models by including images from the segment attachments or text content.

        Args:
            tenant_id: Tenant ID
            text: Text content to summarize
            summary_index_setting: Summary index configuration
            segment_id: Optional segment ID to fetch attachments from SegmentAttachmentBinding table
            document_language: Optional document language (e.g., "Chinese", "English")
                to ensure summary is generated in the correct language

        Returns:
            Tuple of (summary_content, llm_usage) where llm_usage is LLMUsage object
        """
        if not summary_index_setting or not summary_index_setting.get("enable"):
            raise ValueError("summary_index_setting is required and must be enabled to generate summary.")

        model_name = summary_index_setting.get("model_name")
        model_provider_name = summary_index_setting.get("model_provider_name")
        summary_prompt = summary_index_setting.get("summary_prompt")

        if not model_name or not model_provider_name:
            raise ValueError("model_name and model_provider_name are required in summary_index_setting")

        # Import default summary prompt
        is_default_prompt = False
        if not summary_prompt:
            summary_prompt = DEFAULT_GENERATOR_SUMMARY_PROMPT
            is_default_prompt = True

        # Format prompt with document language only for default prompt
        # Custom prompts are used as-is to avoid interfering with user-defined templates
        # If document_language is provided, use it; otherwise, use "the same language as the input content"
        # This is especially important for image-only chunks where text is empty or minimal
        if is_default_prompt:
            language_for_prompt = document_language or "the same language as the input content"
            try:
                summary_prompt = summary_prompt.format(language=language_for_prompt)
            except KeyError:
                # If default prompt doesn't have {language} placeholder, use it as-is
                pass

        provider_manager = create_plugin_provider_manager(tenant_id=tenant_id)
        provider_model_bundle = provider_manager.get_provider_model_bundle(
            tenant_id, model_provider_name, ModelType.LLM
        )
        model_instance = ModelInstance(provider_model_bundle, model_name)

        # Get model schema to check if vision is supported
        model_schema = model_instance.model_type_instance.get_model_schema(model_name, model_instance.credentials)
        supports_vision = model_schema and model_schema.features and ModelFeature.VISION in model_schema.features

        # Extract images if model supports vision
        image_files = []
        if supports_vision:
            # First, try to get images from SegmentAttachmentBinding (preferred method)
            if segment_id:
                image_files = ParagraphIndexProcessor._extract_images_from_segment_attachments(tenant_id, segment_id)

            # If no images from attachments, fall back to extracting from text
            if not image_files:
                image_files = ParagraphIndexProcessor._extract_images_from_text(tenant_id, text)

        # Build prompt messages
        prompt_messages = []

        if image_files:
            # If we have images, create a UserPromptMessage with both text and images
            prompt_message_contents: list[PromptMessageContentUnionTypes] = []

            # Add images first
            for file in image_files:
                try:
                    file_content = file_manager.to_prompt_message_content(
                        file, image_detail_config=ImagePromptMessageContent.DETAIL.LOW
                    )
                    prompt_message_contents.append(file_content)
                except Exception as e:
                    logger.warning("Failed to convert image file to prompt message content: %s", str(e))
                    continue

            # Add text content
            if prompt_message_contents:  # Only add text if we successfully added images
                prompt_message_contents.append(TextPromptMessageContent(data=f"{summary_prompt}\n{text}"))
                prompt_messages.append(UserPromptMessage(content=prompt_message_contents))
            else:
                # If image conversion failed, fall back to text-only
                prompt = f"{summary_prompt}\n{text}"
                prompt_messages.append(UserPromptMessage(content=prompt))
        else:
            # No images, use simple text prompt
            prompt = f"{summary_prompt}\n{text}"
            prompt_messages.append(UserPromptMessage(content=prompt))

        result = model_instance.invoke_llm(
            prompt_messages=cast(list[PromptMessage], prompt_messages), model_parameters={}, stream=False
        )

        # Type assertion: when stream=False, invoke_llm returns LLMResult, not Generator
        if not isinstance(result, LLMResult):
            raise ValueError("Expected LLMResult when stream=False")

        summary_content = result.message.get_text_content()
        usage = result.usage

        # Deduct quota for summary generation (same as workflow nodes)
        try:
            deduct_llm_quota(tenant_id=tenant_id, model_instance=model_instance, usage=usage)
        except Exception as e:
            # Log but don't fail summary generation if quota deduction fails
            logger.warning("Failed to deduct quota for summary generation: %s", str(e))

        return summary_content, usage

    @staticmethod
    def _extract_images_from_text(tenant_id: str, text: str) -> list[File]:
        """
        Extract images from markdown text and convert them to File objects.

        Args:
            tenant_id: Tenant ID
            text: Text content that may contain markdown image links

        Returns:
            List of File objects representing images found in the text
        """
        # Extract markdown images using regex pattern
        pattern = r"!\[.*?\]\((.*?)\)"
        images = re.findall(pattern, text)

        if not images:
            return []

        upload_file_id_list = []

        for image in images:
            # For data before v0.10.0
            pattern = r"/files/([a-f0-9\-]+)/image-preview(?:\?.*?)?"
            match = re.search(pattern, image)
            if match:
                upload_file_id = match.group(1)
                upload_file_id_list.append(upload_file_id)
                continue

            # For data after v0.10.0
            pattern = r"/files/([a-f0-9\-]+)/file-preview(?:\?.*?)?"
            match = re.search(pattern, image)
            if match:
                upload_file_id = match.group(1)
                upload_file_id_list.append(upload_file_id)
                continue

            # For tools directory - direct file formats (e.g., .png, .jpg, etc.)
            pattern = r"/files/tools/([a-f0-9\-]+)\.([a-zA-Z0-9]+)(?:\?[^\s\)\"\']*)?"
            match = re.search(pattern, image)
            if match:
                # Tool files are handled differently, skip for now
                continue

        if not upload_file_id_list:
            return []

        # Get unique IDs for database query
        unique_upload_file_ids = list(set(upload_file_id_list))
        upload_files = db.session.scalars(
            select(UploadFile).where(UploadFile.id.in_(unique_upload_file_ids), UploadFile.tenant_id == tenant_id)
        ).all()

        # Create File objects from UploadFile records
        file_objects = []
        for upload_file in upload_files:
            # Only process image files
            if not upload_file.mime_type or "image" not in upload_file.mime_type:
                continue

            mapping = {
                "upload_file_id": upload_file.id,
                "transfer_method": FileTransferMethod.LOCAL_FILE.value,
                "type": FileType.IMAGE.value,
            }

            try:
                file_obj = build_from_mapping(
                    mapping=mapping,
                    tenant_id=tenant_id,
                    access_controller=_file_access_controller,
                )
                file_objects.append(file_obj)
            except Exception as e:
                logger.warning("Failed to create File object from UploadFile %s: %s", upload_file.id, str(e))
                continue

        return file_objects

    @staticmethod
    def _extract_images_from_segment_attachments(tenant_id: str, segment_id: str) -> list[File]:
        """
        Extract images from SegmentAttachmentBinding table (preferred method).
        This matches how DatasetRetrieval gets segment attachments.

        Args:
            tenant_id: Tenant ID
            segment_id: Segment ID to fetch attachments for

        Returns:
            List of File objects representing images found in segment attachments
        """
        from sqlalchemy import select

        # Query attachments from SegmentAttachmentBinding table
        attachments_with_bindings = db.session.execute(
            select(SegmentAttachmentBinding, UploadFile)
            .join(UploadFile, UploadFile.id == SegmentAttachmentBinding.attachment_id)
            .where(
                SegmentAttachmentBinding.segment_id == segment_id,
                SegmentAttachmentBinding.tenant_id == tenant_id,
            )
        ).all()

        if not attachments_with_bindings:
            return []

        file_objects = []
        for _, upload_file in attachments_with_bindings:
            # Only process image files
            if not upload_file.mime_type or "image" not in upload_file.mime_type:
                continue

            try:
                # Create File object directly (similar to DatasetRetrieval)
                file_obj = File(
                    id=upload_file.id,
                    filename=upload_file.name,
                    extension="." + upload_file.extension,
                    mime_type=upload_file.mime_type,
                    type=FileType.IMAGE,
                    transfer_method=FileTransferMethod.LOCAL_FILE,
                    remote_url=upload_file.source_url,
                    reference=build_file_reference(
                        record_id=str(upload_file.id),
                    ),
                    size=upload_file.size,
                    storage_key=upload_file.key,
                )
                file_objects.append(file_obj)
            except Exception as e:
                logger.warning("Failed to create File object from UploadFile %s: %s", upload_file.id, str(e))
                continue

        return file_objects

```

### api/core/rag/index_processor/processor/parent_child_index_processor.py
```py
"""Paragraph index processor."""

import json
import logging
import uuid
from collections.abc import Mapping
from typing import Any

from sqlalchemy import delete, select

from configs import dify_config
from core.db.session_factory import session_factory
from core.entities.knowledge_entities import PreviewDetail
from core.model_manager import ModelInstance
from core.rag.cleaner.clean_processor import CleanProcessor
from core.rag.data_post_processor.data_post_processor import RerankingModelDict
from core.rag.datasource.retrieval_service import RetrievalService
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.docstore.dataset_docstore import DatasetDocumentStore
from core.rag.extractor.entity.extract_setting import ExtractSetting
from core.rag.extractor.extract_processor import ExtractProcessor
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from core.rag.index_processor.index_processor_base import BaseIndexProcessor, SummaryIndexSettingDict
from core.rag.models.document import AttachmentDocument, ChildDocument, Document, ParentChildStructureChunk
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from extensions.ext_database import db
from libs import helper
from models import Account
from models.dataset import ChildChunk, Dataset, DatasetProcessRule, DocumentSegment
from models.dataset import Document as DatasetDocument
from services.account_service import AccountService
from services.entities.knowledge_entities.knowledge_entities import ParentMode, Rule
from services.summary_index_service import SummaryIndexService

logger = logging.getLogger(__name__)


class ParentChildIndexProcessor(BaseIndexProcessor):
    def extract(self, extract_setting: ExtractSetting, **kwargs) -> list[Document]:
        text_docs = ExtractProcessor.extract(
            extract_setting=extract_setting,
            is_automatic=(
                kwargs.get("process_rule_mode") == "automatic" or kwargs.get("process_rule_mode") == "hierarchical"
            ),
        )

        return text_docs

    def transform(self, documents: list[Document], current_user: Account | None = None, **kwargs) -> list[Document]:
        process_rule = kwargs.get("process_rule")
        if not process_rule:
            raise ValueError("No process rule found.")
        if not process_rule.get("rules"):
            raise ValueError("No rules found in process rule.")
        rules = Rule.model_validate(process_rule.get("rules"))
        all_documents: list[Document] = []
        if rules.parent_mode == ParentMode.PARAGRAPH:
            # Split the text documents into nodes.
            if not rules.segmentation:
                raise ValueError("No segmentation found in rules.")
            splitter = self._get_splitter(
                processing_rule_mode=process_rule.get("mode"),
                max_tokens=rules.segmentation.max_tokens,
                chunk_overlap=rules.segmentation.chunk_overlap,
                separator=rules.segmentation.separator,
                embedding_model_instance=kwargs.get("embedding_model_instance"),
            )
            for document in documents:
                if kwargs.get("preview") and len(all_documents) >= 10:
                    return all_documents
                # document clean
                document_text = CleanProcessor.clean(document.page_content, process_rule)
                document.page_content = document_text
                # parse document to nodes
                document_nodes = splitter.split_documents([document])
                split_documents = []
                for document_node in document_nodes:
                    if document_node.page_content.strip():
                        doc_id = str(uuid.uuid4())
                        hash = helper.generate_text_hash(document_node.page_content)
                        document_node.metadata["doc_id"] = doc_id
                        document_node.metadata["doc_hash"] = hash
                        # delete Splitter character
                        page_content = document_node.page_content
                        if page_content.startswith(".") or page_content.startswith("。"):
                            page_content = page_content[1:].strip()
                        else:
                            page_content = page_content
                        if len(page_content) > 0:
                            document_node.page_content = page_content
                            multimodel_documents = self._get_content_files(document_node, current_user)
                            if multimodel_documents:
                                document_node.attachments = multimodel_documents
                            # parse document to child nodes
                            child_nodes = self._split_child_nodes(
                                document_node, rules, process_rule.get("mode"), kwargs.get("embedding_model_instance")
                            )
                            document_node.children = child_nodes
                            split_documents.append(document_node)
                all_documents.extend(split_documents)
        elif rules.parent_mode == ParentMode.FULL_DOC:
            page_content = "\n".join([document.page_content for document in documents])
            document = Document(page_content=page_content, metadata=documents[0].metadata)
            multimodel_documents = self._get_content_files(document)
            if multimodel_documents:
                document.attachments = multimodel_documents
            # parse document to child nodes
            child_nodes = self._split_child_nodes(
                document, rules, process_rule.get("mode"), kwargs.get("embedding_model_instance")
            )
            if kwargs.get("preview"):
                if len(child_nodes) > dify_config.CHILD_CHUNKS_PREVIEW_NUMBER:
                    child_nodes = child_nodes[: dify_config.CHILD_CHUNKS_PREVIEW_NUMBER]

            document.children = child_nodes
            doc_id = str(uuid.uuid4())
            hash = helper.generate_text_hash(document.page_content)
            document.metadata["doc_id"] = doc_id
            document.metadata["doc_hash"] = hash
            all_documents.append(document)

        return all_documents

    def load(
        self,
        dataset: Dataset,
        documents: list[Document],
        multimodal_documents: list[AttachmentDocument] | None = None,
        with_keywords: bool = True,
        **kwargs,
    ) -> None:
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            vector = Vector(dataset)
            for document in documents:
                child_documents = document.children
                if child_documents:
                    formatted_child_documents = [
                        Document.model_validate(child_document.model_dump()) for child_document in child_documents
                    ]
                    vector.create(formatted_child_documents)
            if multimodal_documents and dataset.is_multimodal:
                vector.create_multimodal(multimodal_documents)

    def clean(self, dataset: Dataset, node_ids: list[str] | None, with_keywords: bool = True, **kwargs) -> None:
        # node_ids is segment's node_ids
        # Note: Summary indexes are now disabled (not deleted) when segments are disabled.
        # This method is called for actual deletion scenarios (e.g., when segment is deleted).
        # For disable operations, disable_summaries_for_segments is called directly in the task.
        # Only delete summaries if explicitly requested (e.g., when segment is actually deleted)
        delete_summaries = kwargs.get("delete_summaries", False)
        if delete_summaries:
            if node_ids:
                # Find segments by index_node_id
                with session_factory.create_session() as session:
                    segments = (
                        session.query(DocumentSegment)
                        .filter(
                            DocumentSegment.dataset_id == dataset.id,
                            DocumentSegment.index_node_id.in_(node_ids),
                        )
                        .all()
                    )
                    segment_ids = [segment.id for segment in segments]
                    if segment_ids:
                        SummaryIndexService.delete_summaries_for_segments(dataset, segment_ids)
            else:
                # Delete all summaries for the dataset
                SummaryIndexService.delete_summaries_for_segments(dataset, None)

        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            delete_child_chunks = kwargs.get("delete_child_chunks") or False
            precomputed_child_node_ids = kwargs.get("precomputed_child_node_ids")
            vector = Vector(dataset)

            if node_ids:
                # Use precomputed child_node_ids if available (to avoid race conditions)
                if precomputed_child_node_ids is not None:
                    child_node_ids = precomputed_child_node_ids
                else:
                    # Fallback to original query (may fail if segments are already deleted)
                    rows = db.session.execute(
                        select(ChildChunk.index_node_id)
                        .join(DocumentSegment, ChildChunk.segment_id == DocumentSegment.id)
                        .where(
                            DocumentSegment.dataset_id == dataset.id,
                            DocumentSegment.index_node_id.in_(node_ids),
                            ChildChunk.dataset_id == dataset.id,
                        )
                    ).all()
                    child_node_ids = [row[0] for row in rows if row[0]]

                # Delete from vector index
                if child_node_ids:
                    vector.delete_by_ids(child_node_ids)

                # Delete from database
                if delete_child_chunks and child_node_ids:
                    db.session.execute(
                        delete(ChildChunk).where(
                            ChildChunk.dataset_id == dataset.id, ChildChunk.index_node_id.in_(child_node_ids)
                        )
                    )
                    db.session.commit()
            else:
                vector.delete()

                if delete_child_chunks:
                    # Use existing compound index: (tenant_id, dataset_id, ...)
                    db.session.execute(
                        delete(ChildChunk).where(
                            ChildChunk.tenant_id == dataset.tenant_id, ChildChunk.dataset_id == dataset.id
                        )
                    )
                    db.session.commit()

    def retrieve(
        self,
        retrieval_method: RetrievalMethod,
        query: str,
        dataset: Dataset,
        top_k: int,
        score_threshold: float,
        reranking_model: RerankingModelDict,
    ) -> list[Document]:
        # Set search parameters.
        results = RetrievalService.retrieve(
            retrieval_method=retrieval_method,
            dataset_id=dataset.id,
            query=query,
            top_k=top_k,
            score_threshold=score_threshold,
            reranking_model=reranking_model,
        )
        # Organize results.
        docs = []
        for result in results:
            metadata = result.metadata
            metadata["score"] = result.score
            if result.score >= score_threshold:
                doc = Document(page_content=result.page_content, metadata=metadata)
                docs.append(doc)
        return docs

    def _split_child_nodes(
        self,
        document_node: Document,
        rules: Rule,
        process_rule_mode: str,
        embedding_model_instance: ModelInstance | None,
    ) -> list[ChildDocument]:
        if not rules.subchunk_segmentation:
            raise ValueError("No subchunk segmentation found in rules.")
        child_splitter = self._get_splitter(
            processing_rule_mode=process_rule_mode,
            max_tokens=rules.subchunk_segmentation.max_tokens,
            chunk_overlap=rules.subchunk_segmentation.chunk_overlap,
            separator=rules.subchunk_segmentation.separator,
            embedding_model_instance=embedding_model_instance,
        )
        # parse document to child nodes
        child_nodes = []
        child_documents = child_splitter.split_documents([document_node])
        for child_document_node in child_documents:
            if child_document_node.page_content.strip():
                doc_id = str(uuid.uuid4())
                hash = helper.generate_text_hash(child_document_node.page_content)
                child_document = ChildDocument(
                    page_content=child_document_node.page_content, metadata=document_node.metadata
                )
                child_document.metadata["doc_id"] = doc_id
                child_document.metadata["doc_hash"] = hash
                child_page_content = child_document.page_content
                if child_page_content.startswith(".") or child_page_content.startswith("。"):
                    child_page_content = child_page_content[1:].strip()
                if len(child_page_content) > 0:
                    child_document.page_content = child_page_content
                    child_nodes.append(child_document)
        return child_nodes

    def index(self, dataset: Dataset, document: DatasetDocument, chunks: Any) -> None:
        parent_childs = ParentChildStructureChunk.model_validate(chunks)
        documents = []
        for parent_child in parent_childs.parent_child_chunks:
            metadata = {
                "dataset_id": dataset.id,
                "document_id": document.id,
                "doc_id": str(uuid.uuid4()),
                "doc_hash": helper.generate_text_hash(parent_child.parent_content),
            }
            child_documents = []
            for child in parent_child.child_contents:
                child_metadata = {
                    "dataset_id": dataset.id,
                    "document_id": document.id,
                    "doc_id": str(uuid.uuid4()),
                    "doc_hash": helper.generate_text_hash(child),
                }
                child_documents.append(ChildDocument(page_content=child, metadata=child_metadata))
            doc = Document(page_content=parent_child.parent_content, metadata=metadata, children=child_documents)
            if parent_child.files and len(parent_child.files) > 0:
                attachments = []
                for file in parent_child.files:
                    file_metadata = {
                        "doc_id": file.id,
                        "doc_hash": "",
                        "document_id": document.id,
                        "dataset_id": dataset.id,
                        "doc_type": DocType.IMAGE,
                    }
                    file_document = AttachmentDocument(page_content=file.filename or "", metadata=file_metadata)
                    attachments.append(file_document)
                doc.attachments = attachments
            else:
                account = AccountService.load_user(document.created_by)
                if not account:
                    raise ValueError("Invalid account")
                doc.attachments = self._get_content_files(doc, current_user=account)
            documents.append(doc)
        if documents:
            # update document parent mode
            dataset_process_rule = DatasetProcessRule(
                dataset_id=dataset.id,
                mode="hierarchical",
                rules=json.dumps(
                    {
                        "parent_mode": parent_childs.parent_mode,
                    }
                ),
                created_by=document.created_by,
            )
            db.session.add(dataset_process_rule)
            db.session.flush()
            document.dataset_process_rule_id = dataset_process_rule.id
            db.session.commit()
            # save node to document segment
            doc_store = DatasetDocumentStore(dataset=dataset, user_id=document.created_by, document_id=document.id)
            # add document segments
            doc_store.add_documents(docs=documents, save_child=True)
            if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                all_child_documents = []
                all_multimodal_documents = []
                for doc in documents:
                    if doc.children:
                        all_child_documents.extend(doc.children)
                    if doc.attachments:
                        all_multimodal_documents.extend(doc.attachments)
                vector = Vector(dataset)
                if all_child_documents:
                    vector.create(all_child_documents)
                if all_multimodal_documents and dataset.is_multimodal:
                    vector.create_multimodal(all_multimodal_documents)

    def format_preview(self, chunks: Any) -> Mapping[str, Any]:
        parent_childs = ParentChildStructureChunk.model_validate(chunks)
        preview = []
        for parent_child in parent_childs.parent_child_chunks:
            preview.append({"content": parent_child.parent_content, "child_chunks": parent_child.child_contents})
        return {
            "chunk_structure": IndexStructureType.PARENT_CHILD_INDEX,
            "parent_mode": parent_childs.parent_mode,
            "preview": preview,
            "total_segments": len(parent_childs.parent_child_chunks),
        }

    def generate_summary_preview(
        self,
        tenant_id: str,
        preview_texts: list[PreviewDetail],
        summary_index_setting: SummaryIndexSettingDict,
        doc_language: str | None = None,
    ) -> list[PreviewDetail]:
        """
        For each parent chunk in preview_texts, concurrently call generate_summary to generate a summary
        and write it to the summary attribute of PreviewDetail.
        In preview mode (indexing-estimate), if any summary generation fails, the method will raise an exception.

        Note: For parent-child structure, we only generate summaries for parent chunks.
        """
        import concurrent.futures

        from flask import current_app

        # Capture Flask app context for worker threads
        flask_app = None
        try:
            flask_app = current_app._get_current_object()  # type: ignore
        except RuntimeError:
            logger.warning("No Flask application context available, summary generation may fail")

        def process(preview: PreviewDetail) -> None:
            """Generate summary for a single preview item (parent chunk)."""
            from core.rag.index_processor.processor.paragraph_index_processor import ParagraphIndexProcessor

            if flask_app:
                # Ensure Flask app context in worker thread
                with flask_app.app_context():
                    summary, _ = ParagraphIndexProcessor.generate_summary(
                        tenant_id=tenant_id,
                        text=preview.content,
                        summary_index_setting=summary_index_setting,
                        document_language=doc_language,
                    )
                    preview.summary = summary
            else:
                # Fallback: try without app context (may fail)
                summary, _ = ParagraphIndexProcessor.generate_summary(
                    tenant_id=tenant_id,
                    text=preview.content,
                    summary_index_setting=summary_index_setting,
                    document_language=doc_language,
                )
                preview.summary = summary

        # Generate summaries concurrently using ThreadPoolExecutor
        # Set a reasonable timeout to prevent hanging (60 seconds per chunk, max 5 minutes total)
        timeout_seconds = min(300, 60 * len(preview_texts))
        errors: list[Exception] = []

        with concurrent.futures.ThreadPoolExecutor(max_workers=min(10, len(preview_texts))) as executor:
            futures = [executor.submit(process, preview) for preview in preview_texts]
            # Wait for all tasks to complete with timeout
            done, not_done = concurrent.futures.wait(futures, timeout=timeout_seconds)

            # Cancel tasks that didn't complete in time
            if not_done:
                timeout_error_msg = (
                    f"Summary generation timeout: {len(not_done)} chunks did not complete within {timeout_seconds}s"
                )
                logger.warning("%s. Cancelling remaining tasks...", timeout_error_msg)
                # In preview mode, timeout is also an error
                errors.append(TimeoutError(timeout_error_msg))
                for future in not_done:
                    future.cancel()
                # Wait a bit for cancellation to take effect
                concurrent.futures.wait(not_done, timeout=5)

            # Collect exceptions from completed futures
            for future in done:
                try:
                    future.result()  # This will raise any exception that occurred
                except Exception as e:
                    logger.exception("Error in summary generation future")
                    errors.append(e)

        # In preview mode (indexing-estimate), if there are any errors, fail the request
        if errors:
            error_messages = [str(e) for e in errors]
            error_summary = (
                f"Failed to generate summaries for {len(errors)} chunk(s). "
                f"Errors: {'; '.join(error_messages[:3])}"  # Show first 3 errors
            )
            if len(errors) > 3:
                error_summary += f" (and {len(errors) - 3} more)"
            logger.error("Summary generation failed in preview mode: %s", error_summary)
            raise ValueError(error_summary)

        return preview_texts

```

### api/core/rag/index_processor/processor/__init__.py
```py

```

### api/core/rag/index_processor/processor/qa_index_processor.py
```py
"""Paragraph index processor."""

import logging
import re
import threading
import uuid
from collections.abc import Mapping
from typing import Any

import pandas as pd
from flask import Flask, current_app
from werkzeug.datastructures import FileStorage

from core.db.session_factory import session_factory
from core.entities.knowledge_entities import PreviewDetail
from core.llm_generator.llm_generator import LLMGenerator
from core.rag.cleaner.clean_processor import CleanProcessor
from core.rag.data_post_processor.data_post_processor import RerankingModelDict
from core.rag.datasource.retrieval_service import RetrievalService
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.docstore.dataset_docstore import DatasetDocumentStore
from core.rag.extractor.entity.extract_setting import ExtractSetting
from core.rag.extractor.extract_processor import ExtractProcessor
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from core.rag.index_processor.index_processor_base import BaseIndexProcessor, SummaryIndexSettingDict
from core.rag.models.document import AttachmentDocument, Document, QAStructureChunk
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.tools.utils.text_processing_utils import remove_leading_symbols
from libs import helper
from models.account import Account
from models.dataset import Dataset, DocumentSegment
from models.dataset import Document as DatasetDocument
from services.entities.knowledge_entities.knowledge_entities import Rule
from services.summary_index_service import SummaryIndexService

logger = logging.getLogger(__name__)


class QAIndexProcessor(BaseIndexProcessor):
    def extract(self, extract_setting: ExtractSetting, **kwargs) -> list[Document]:
        text_docs = ExtractProcessor.extract(
            extract_setting=extract_setting,
            is_automatic=(
                kwargs.get("process_rule_mode") == "automatic" or kwargs.get("process_rule_mode") == "hierarchical"
            ),
        )
        return text_docs

    def transform(self, documents: list[Document], current_user: Account | None = None, **kwargs) -> list[Document]:
        preview = kwargs.get("preview")
        process_rule = kwargs.get("process_rule")
        if not process_rule:
            raise ValueError("No process rule found.")
        if not process_rule.get("rules"):
            raise ValueError("No rules found in process rule.")
        rules = Rule.model_validate(process_rule.get("rules"))
        splitter = self._get_splitter(
            processing_rule_mode=process_rule.get("mode"),
            max_tokens=rules.segmentation.max_tokens if rules.segmentation else 0,
            chunk_overlap=rules.segmentation.chunk_overlap if rules.segmentation else 0,
            separator=rules.segmentation.separator if rules.segmentation else "",
            embedding_model_instance=kwargs.get("embedding_model_instance"),
        )

        # Split the text documents into nodes.
        all_documents: list[Document] = []
        all_qa_documents: list[Document] = []
        for document in documents:
            # document clean
            document_text = CleanProcessor.clean(document.page_content, kwargs.get("process_rule") or {})
            document.page_content = document_text

            # parse document to nodes
            document_nodes = splitter.split_documents([document])
            split_documents = []
            for document_node in document_nodes:
                if document_node.page_content.strip():
                    doc_id = str(uuid.uuid4())
                    hash = helper.generate_text_hash(document_node.page_content)
                    if document_node.metadata is not None:
                        document_node.metadata["doc_id"] = doc_id
                        document_node.metadata["doc_hash"] = hash
                    # delete Splitter character
                    page_content = document_node.page_content
                    document_node.page_content = remove_leading_symbols(page_content)
                    split_documents.append(document_node)
            all_documents.extend(split_documents)
        if preview:
            self._format_qa_document(
                current_app._get_current_object(),  # type: ignore
                kwargs.get("tenant_id"),  # type: ignore
                all_documents[0],
                all_qa_documents,
                kwargs.get("doc_language", "English"),
            )
        else:
            for i in range(0, len(all_documents), 10):
                threads = []
                sub_documents = all_documents[i : i + 10]
                for doc in sub_documents:
                    document_format_thread = threading.Thread(
                        target=self._format_qa_document,
                        kwargs={
                            "flask_app": current_app._get_current_object(),  # type: ignore
                            "tenant_id": kwargs.get("tenant_id"),  # type: ignore
                            "document_node": doc,
                            "all_qa_documents": all_qa_documents,
                            "document_language": kwargs.get("doc_language", "English"),
                        },
                    )
                    threads.append(document_format_thread)
                    document_format_thread.start()
                for thread in threads:
                    thread.join()
        return all_qa_documents

    def format_by_template(self, file: FileStorage, **kwargs) -> list[Document]:
        # check file type
        if not file.filename or not file.filename.lower().endswith(".csv"):
            raise ValueError("Invalid file type. Only CSV files are allowed")

        try:
            # Skip the first row
            df = pd.read_csv(file)  # type: ignore
            text_docs = []
            for _, row in df.iterrows():
                data = Document(page_content=row.iloc[0], metadata={"answer": row.iloc[1]})
                text_docs.append(data)
            if len(text_docs) == 0:
                raise ValueError("The CSV file is empty.")

        except Exception as e:
            raise ValueError(str(e))
        return text_docs

    def load(
        self,
        dataset: Dataset,
        documents: list[Document],
        multimodal_documents: list[AttachmentDocument] | None = None,
        with_keywords: bool = True,
        **kwargs,
    ) -> None:
        if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            vector = Vector(dataset)
            vector.create(documents)
            if multimodal_documents and dataset.is_multimodal:
                vector.create_multimodal(multimodal_documents)

    def clean(self, dataset: Dataset, node_ids: list[str] | None, with_keywords: bool = True, **kwargs) -> None:
        # Note: Summary indexes are now disabled (not deleted) when segments are disabled.
        # This method is called for actual deletion scenarios (e.g., when segment is deleted).
        # For disable operations, disable_summaries_for_segments is called directly in the task.
        # Note: qa_model doesn't generate summaries, but we clean them for completeness
        # Only delete summaries if explicitly requested (e.g., when segment is actually deleted)
        delete_summaries = kwargs.get("delete_summaries", False)
        if delete_summaries:
            if node_ids:
                # Find segments by index_node_id
                with session_factory.create_session() as session:
                    segments = (
                        session.query(DocumentSegment)
                        .filter(
                            DocumentSegment.dataset_id == dataset.id,
                            DocumentSegment.index_node_id.in_(node_ids),
                        )
                        .all()
                    )
                    segment_ids = [segment.id for segment in segments]
                    if segment_ids:
                        SummaryIndexService.delete_summaries_for_segments(dataset, segment_ids)
            else:
                # Delete all summaries for the dataset
                SummaryIndexService.delete_summaries_for_segments(dataset, None)

        vector = Vector(dataset)
        if node_ids:
            vector.delete_by_ids(node_ids)
        else:
            vector.delete()

    def retrieve(
        self,
        retrieval_method: RetrievalMethod,
        query: str,
        dataset: Dataset,
        top_k: int,
        score_threshold: float,
        reranking_model: RerankingModelDict,
    ):
        # Set search parameters.
        results = RetrievalService.retrieve(
            retrieval_method=retrieval_method,
            dataset_id=dataset.id,
            query=query,
            top_k=top_k,
            score_threshold=score_threshold,
            reranking_model=reranking_model,
        )
        # Organize results.
        docs = []
        for result in results:
            metadata = result.metadata
            metadata["score"] = result.score
            if result.score >= score_threshold:
                doc = Document(page_content=result.page_content, metadata=metadata)
                docs.append(doc)
        return docs

    def index(self, dataset: Dataset, document: DatasetDocument, chunks: Any) -> None:
        qa_chunks = QAStructureChunk.model_validate(chunks)
        documents = []
        for qa_chunk in qa_chunks.qa_chunks:
            metadata = {
                "dataset_id": dataset.id,
                "document_id": document.id,
                "doc_id": str(uuid.uuid4()),
                "doc_hash": helper.generate_text_hash(qa_chunk.question),
                "answer": qa_chunk.answer,
            }
            doc = Document(page_content=qa_chunk.question, metadata=metadata)
            documents.append(doc)
        if documents:
            # save node to document segment
            doc_store = DatasetDocumentStore(dataset=dataset, user_id=document.created_by, document_id=document.id)
            doc_store.add_documents(docs=documents, save_child=False)
            if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                vector = Vector(dataset)
                vector.create(documents)
            else:
                raise ValueError("Indexing technique must be high quality.")

    def format_preview(self, chunks: Any) -> Mapping[str, Any]:
        qa_chunks = QAStructureChunk.model_validate(chunks)
        preview = []
        for qa_chunk in qa_chunks.qa_chunks:
            preview.append({"question": qa_chunk.question, "answer": qa_chunk.answer})
        return {
            "chunk_structure": IndexStructureType.QA_INDEX,
            "qa_preview": preview,
            "total_segments": len(qa_chunks.qa_chunks),
        }

    def generate_summary_preview(
        self,
        tenant_id: str,
        preview_texts: list[PreviewDetail],
        summary_index_setting: SummaryIndexSettingDict,
        doc_language: str | None = None,
    ) -> list[PreviewDetail]:
        """
        QA model doesn't generate summaries, so this method returns preview_texts unchanged.

        Note: QA model uses question-answer pairs, which don't require summary generation.
        """
        # QA model doesn't generate summaries, return as-is
        return preview_texts

    def _format_qa_document(self, flask_app: Flask, tenant_id: str, document_node, all_qa_documents, document_language):
        format_documents = []
        if document_node.page_content is None or not document_node.page_content.strip():
            return
        with flask_app.app_context():
            try:
                # qa model document
                response = LLMGenerator.generate_qa_document(tenant_id, document_node.page_content, document_language)
                document_qa_list = self._format_split_text(response)
                qa_documents = []
                for result in document_qa_list:
                    qa_document = Document(page_content=result["question"], metadata=document_node.metadata.copy())
                    if qa_document.metadata is not None:
                        doc_id = str(uuid.uuid4())
                        hash = helper.generate_text_hash(result["question"])
                        qa_document.metadata["answer"] = result["answer"]
                        qa_document.metadata["doc_id"] = doc_id
                        qa_document.metadata["doc_hash"] = hash
                    qa_documents.append(qa_document)
                format_documents.extend(qa_documents)
            except Exception:
                logger.exception("Failed to format qa document")

            all_qa_documents.extend(format_documents)

    def _format_split_text(self, text):
        regex = r"Q\d+:\s*(.*?)\s*A\d+:\s*([\s\S]*?)(?=Q\d+:|$)"
        matches = re.findall(regex, text, re.UNICODE)

        return [{"question": q, "answer": re.sub(r"\n\s*", "\n", a.strip())} for q, a in matches if q and a]

```

### api/core/rag/index_processor/__init__.py
```py

```

### api/core/rag/index_processor/constant/index_type.py
```py
from enum import StrEnum


class IndexStructureType(StrEnum):
    PARAGRAPH_INDEX = "text_model"
    QA_INDEX = "qa_model"
    PARENT_CHILD_INDEX = "hierarchical_model"


class IndexTechniqueType(StrEnum):
    ECONOMY = "economy"
    HIGH_QUALITY = "high_quality"

```

### api/core/rag/index_processor/constant/query_type.py
```py
from enum import StrEnum


class QueryType(StrEnum):
    TEXT_QUERY = "text_query"
    IMAGE_QUERY = "image_query"

```

### api/core/rag/index_processor/constant/built_in_field.py
```py
from enum import StrEnum, auto


class BuiltInField(StrEnum):
    document_name = auto()
    uploader = auto()
    upload_date = auto()
    last_update_date = auto()
    source = auto()


class MetadataDataSource(StrEnum):
    upload_file = "file_upload"
    website_crawl = "website"
    notion_import = "notion"
    local_file = "file_upload"
    online_document = "online_document"
    online_drive = "online_drive"

```

### api/core/rag/index_processor/constant/doc_type.py
```py
from enum import StrEnum


class DocType(StrEnum):
    TEXT = "text"
    IMAGE = "image"

```

### api/core/rag/index_processor/constant/__init__.py
```py

```

### api/core/rag/index_processor/index_processor.py
```py
import concurrent.futures
import datetime
import logging
import time
from collections.abc import Mapping
from typing import Any

from flask import current_app
from sqlalchemy import delete, func, select

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.index_processor.index_processor_base import SummaryIndexSettingDict
from core.workflow.nodes.knowledge_index.exc import KnowledgeIndexNodeError
from core.workflow.nodes.knowledge_index.protocols import Preview, PreviewItem, QaPreview
from models.dataset import Dataset, Document, DocumentSegment

from .index_processor_factory import IndexProcessorFactory
from .processor.paragraph_index_processor import ParagraphIndexProcessor

logger = logging.getLogger(__name__)


class IndexProcessor:
    def format_preview(self, chunk_structure: str, chunks: Any) -> Preview:
        index_processor = IndexProcessorFactory(chunk_structure).init_index_processor()
        preview = index_processor.format_preview(chunks)
        data = Preview(
            chunk_structure=preview["chunk_structure"],
            total_segments=preview["total_segments"],
            preview=[],
            parent_mode=None,
            qa_preview=[],
        )
        if "parent_mode" in preview:
            data.parent_mode = preview["parent_mode"]

        # Different index processors return different preview shapes:
        # - paragraph/parent-child processors: {"preview": [...]}
        # - QA processor: {"qa_preview": [...]} (no "preview" key)
        for item in preview.get("preview", []):
            if "content" in item and "child_chunks" in item:
                data.preview.append(
                    PreviewItem(content=item["content"], child_chunks=item["child_chunks"], summary=None)
                )
            elif "question" in item and "answer" in item:
                data.qa_preview.append(QaPreview(question=item["question"], answer=item["answer"]))
            elif "content" in item:
                data.preview.append(PreviewItem(content=item["content"], child_chunks=None, summary=None))

        for item in preview.get("qa_preview", []):
            if "question" in item and "answer" in item:
                data.qa_preview.append(QaPreview(question=item["question"], answer=item["answer"]))
        return data

    def index_and_clean(
        self,
        dataset_id: str,
        document_id: str,
        original_document_id: str,
        chunks: Mapping[str, Any],
        batch: Any,
        summary_index_setting: SummaryIndexSettingDict | None = None,
    ):
        with session_factory.create_session() as session:
            document = session.query(Document).filter_by(id=document_id).first()
            if not document:
                raise KnowledgeIndexNodeError(f"Document {document_id} not found.")

            dataset = session.query(Dataset).filter_by(id=dataset_id).first()
            if not dataset:
                raise KnowledgeIndexNodeError(f"Dataset {dataset_id} not found.")

            dataset_name_value = dataset.name
            document_name_value = document.name
            created_at_value = document.created_at
            if summary_index_setting is None:
                summary_index_setting = dataset.summary_index_setting
            index_node_ids = []

            index_processor = IndexProcessorFactory(dataset.chunk_structure).init_index_processor()
            if original_document_id:
                segments = session.scalars(
                    select(DocumentSegment).where(DocumentSegment.document_id == original_document_id)
                ).all()
                if segments:
                    index_node_ids = [segment.index_node_id for segment in segments]

        indexing_start_at = time.perf_counter()
        # delete from vector index
        if index_node_ids:
            index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)

        with session_factory.create_session() as session, session.begin():
            if index_node_ids:
                segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.document_id == original_document_id)
                session.execute(segment_delete_stmt)

        index_processor.index(dataset, document, chunks)
        indexing_end_at = time.perf_counter()

        with session_factory.create_session() as session, session.begin():
            document.indexing_latency = indexing_end_at - indexing_start_at
            document.indexing_status = "completed"
            document.completed_at = datetime.datetime.now(datetime.UTC).replace(tzinfo=None)
            document.word_count = (
                session.query(func.sum(DocumentSegment.word_count))
                .where(
                    DocumentSegment.document_id == document_id,
                    DocumentSegment.dataset_id == dataset_id,
                )
                .scalar()
            ) or 0
            # Update need_summary based on dataset's summary_index_setting
            if summary_index_setting and summary_index_setting.get("enable") is True:
                document.need_summary = True
            else:
                document.need_summary = False
            session.add(document)
            # update document segment status
            session.query(DocumentSegment).where(
                DocumentSegment.document_id == document_id,
                DocumentSegment.dataset_id == dataset_id,
            ).update(
                {
                    DocumentSegment.status: "completed",
                    DocumentSegment.enabled: True,
                    DocumentSegment.completed_at: datetime.datetime.now(datetime.UTC).replace(tzinfo=None),
                }
            )

        return {
            "dataset_id": dataset_id,
            "dataset_name": dataset_name_value,
            "batch": batch,
            "document_id": document_id,
            "document_name": document_name_value,
            "created_at": created_at_value.timestamp(),
            "display_status": "completed",
        }

    def get_preview_output(
        self,
        chunks: Any,
        dataset_id: str,
        document_id: str,
        chunk_structure: str,
        summary_index_setting: SummaryIndexSettingDict | None,
    ) -> Preview:
        doc_language = None
        with session_factory.create_session() as session:
            if document_id:
                document = session.query(Document).filter_by(id=document_id).first()
            else:
                document = None

            dataset = session.query(Dataset).filter_by(id=dataset_id).first()
            if not dataset:
                raise KnowledgeIndexNodeError(f"Dataset {dataset_id} not found.")

            if summary_index_setting is None:
                summary_index_setting = dataset.summary_index_setting

            if document:
                doc_language = document.doc_language
            indexing_technique = dataset.indexing_technique
            tenant_id = dataset.tenant_id

        preview_output = self.format_preview(chunk_structure, chunks)
        if indexing_technique != IndexTechniqueType.HIGH_QUALITY:
            return preview_output

        if not summary_index_setting or not summary_index_setting.get("enable"):
            return preview_output

        if preview_output.preview is not None:
            chunk_count = len(preview_output.preview)
            logger.info(
                "Generating summaries for %s chunks in preview mode (dataset: %s)",
                chunk_count,
                dataset_id,
            )

            flask_app = None
            try:
                flask_app = current_app._get_current_object()  # type: ignore
            except RuntimeError:
                logger.warning("No Flask application context available, summary generation may fail")

            def generate_summary_for_chunk(preview_item: PreviewItem) -> None:
                """Generate summary for a single chunk."""
                if flask_app:
                    with flask_app.app_context():
                        if preview_item.content is not None:
                            # Set Flask application context in worker thread
                            summary, _ = ParagraphIndexProcessor.generate_summary(
                                tenant_id=tenant_id,
                                text=preview_item.content,
                                summary_index_setting=summary_index_setting,
                                document_language=doc_language,
                            )
                            if summary:
                                preview_item.summary = summary

                        else:
                            summary, _ = ParagraphIndexProcessor.generate_summary(
                                tenant_id=tenant_id,
                                text=preview_item.content if preview_item.content is not None else "",
                                summary_index_setting=summary_index_setting,
                                document_language=doc_language,
                            )
                            if summary:
                                preview_item.summary = summary

            # Generate summaries concurrently using ThreadPoolExecutor
            # Set a reasonable timeout to prevent hanging (60 seconds per chunk, max 5 minutes total)
            timeout_seconds = min(300, 60 * len(preview_output.preview))
            errors: list[Exception] = []

            with concurrent.futures.ThreadPoolExecutor(max_workers=min(10, len(preview_output.preview))) as executor:
                futures = [
                    executor.submit(generate_summary_for_chunk, preview_item) for preview_item in preview_output.preview
                ]
                # Wait for all tasks to complete with timeout
                done, not_done = concurrent.futures.wait(futures, timeout=timeout_seconds)

                # Cancel tasks that didn't complete in time
                if not_done:
                    timeout_error_msg = (
                        f"Summary generation timeout: {len(not_done)} chunks did not complete within {timeout_seconds}s"
                    )
                    logger.warning("%s. Cancelling remaining tasks...", timeout_error_msg)
                    # In preview mode, timeout is also an error
                    errors.append(TimeoutError(timeout_error_msg))
                    for future in not_done:
                        future.cancel()
                    # Wait a bit for cancellation to take effect
                    concurrent.futures.wait(not_done, timeout=5)

                # Collect exceptions from completed futures
                for future in done:
                    try:
                        future.result()  # This will raise any exception that occurred
                    except Exception as e:
                        logger.exception("Error in summary generation future")
                        errors.append(e)

            # In preview mode, if there are any errors, fail the request
            if errors:
                error_messages = [str(e) for e in errors]
                error_summary = (
                    f"Failed to generate summaries for {len(errors)} chunk(s). "
                    f"Errors: {'; '.join(error_messages[:3])}"  # Show first 3 errors
                )
                if len(errors) > 3:
                    error_summary += f" (and {len(errors) - 3} more)"
                logger.error("Summary generation failed in preview mode: %s", error_summary)
                raise KnowledgeIndexNodeError(error_summary)

            completed_count = sum(1 for item in preview_output.preview if item.summary is not None)
            logger.info(
                "Completed summary generation for preview chunks: %s/%s succeeded",
                completed_count,
                len(preview_output.preview),
            )
        return preview_output

```

### api/core/rag/index_processor/index_processor_base.py
```py
"""Abstract interface for document loader implementations."""

import cgi
import logging
import mimetypes
import os
import re
from abc import ABC, abstractmethod
from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, NotRequired, TypedDict
from urllib.parse import unquote, urlparse

import httpx
from sqlalchemy import select

from configs import dify_config
from core.entities.knowledge_entities import PreviewDetail
from core.helper import ssrf_proxy
from core.rag.data_post_processor.data_post_processor import RerankingModelDict
from core.rag.extractor.entity.extract_setting import ExtractSetting
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.models.document import AttachmentDocument, Document
from core.rag.retrieval.retrieval_methods import RetrievalMethod
from core.rag.splitter.fixed_text_splitter import (
    EnhanceRecursiveCharacterTextSplitter,
    FixedRecursiveCharacterTextSplitter,
)
from core.rag.splitter.text_splitter import TextSplitter
from extensions.ext_database import db
from extensions.ext_storage import storage
from models import Account, ToolFile
from models.dataset import Dataset, DatasetProcessRule
from models.dataset import Document as DatasetDocument
from models.model import UploadFile

if TYPE_CHECKING:
    from core.model_manager import ModelInstance


class SummaryIndexSettingDict(TypedDict):
    enable: bool
    model_name: NotRequired[str]
    model_provider_name: NotRequired[str]
    summary_prompt: NotRequired[str]


class BaseIndexProcessor(ABC):
    """Interface for extract files."""

    @abstractmethod
    def extract(self, extract_setting: ExtractSetting, **kwargs) -> list[Document]:
        raise NotImplementedError

    @abstractmethod
    def transform(self, documents: list[Document], current_user: Account | None = None, **kwargs) -> list[Document]:
        raise NotImplementedError

    @abstractmethod
    def generate_summary_preview(
        self,
        tenant_id: str,
        preview_texts: list[PreviewDetail],
        summary_index_setting: SummaryIndexSettingDict,
        doc_language: str | None = None,
    ) -> list[PreviewDetail]:
        """
        For each segment in preview_texts, generate a summary using LLM and attach it to the segment.
        The summary can be stored in a new attribute, e.g., summary.
        This method should be implemented by subclasses.

        Args:
            tenant_id: Tenant ID
            preview_texts: List of preview details to generate summaries for
            summary_index_setting: Summary index configuration
            doc_language: Optional document language to ensure summary is generated in the correct language
        """
        raise NotImplementedError

    @abstractmethod
    def load(
        self,
        dataset: Dataset,
        documents: list[Document],
        multimodal_documents: list[AttachmentDocument] | None = None,
        with_keywords: bool = True,
        **kwargs,
    ) -> None:
        raise NotImplementedError

    @abstractmethod
    def clean(self, dataset: Dataset, node_ids: list[str] | None, with_keywords: bool = True, **kwargs) -> None:
        raise NotImplementedError

    @abstractmethod
    def index(self, dataset: Dataset, document: DatasetDocument, chunks: Any) -> None:
        raise NotImplementedError

    @abstractmethod
    def format_preview(self, chunks: Any) -> Mapping[str, Any]:
        raise NotImplementedError

    @abstractmethod
    def retrieve(
        self,
        retrieval_method: RetrievalMethod,
        query: str,
        dataset: Dataset,
        top_k: int,
        score_threshold: float,
        reranking_model: RerankingModelDict,
    ) -> list[Document]:
        raise NotImplementedError

    def _get_splitter(
        self,
        processing_rule_mode: str,
        max_tokens: int,
        chunk_overlap: int,
        separator: str,
        embedding_model_instance: "ModelInstance | None",
    ) -> TextSplitter:
        """
        Get the NodeParser object according to the processing rule.
        """
        character_splitter: TextSplitter
        if processing_rule_mode in ["custom", "hierarchical"]:
            # The user-defined segmentation rule
            max_segmentation_tokens_length = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
            if max_tokens < 50 or max_tokens > max_segmentation_tokens_length:
                raise ValueError(f"Custom segment length should be between 50 and {max_segmentation_tokens_length}.")

            if separator:
                separator = separator.replace("\\n", "\n")

            character_splitter = FixedRecursiveCharacterTextSplitter.from_encoder(
                chunk_size=max_tokens,
                chunk_overlap=chunk_overlap,
                fixed_separator=separator,
                separators=["\n\n", "。", ". ", " ", ""],
                embedding_model_instance=embedding_model_instance,
            )
        else:
            # Automatic segmentation
            character_splitter = EnhanceRecursiveCharacterTextSplitter.from_encoder(
                chunk_size=DatasetProcessRule.AUTOMATIC_RULES["segmentation"]["max_tokens"],
                chunk_overlap=DatasetProcessRule.AUTOMATIC_RULES["segmentation"]["chunk_overlap"],
                separators=["\n\n", "。", ". ", " ", ""],
                embedding_model_instance=embedding_model_instance,
            )

        return character_splitter

    def _get_content_files(self, document: Document, current_user: Account | None = None) -> list[AttachmentDocument]:
        """
        Get the content files from the document.
        """
        multi_model_documents: list[AttachmentDocument] = []
        text = document.page_content
        images = self._extract_markdown_images(text)
        if not images:
            return multi_model_documents
        upload_file_id_list = []

        for image in images:
            # Collect all upload_file_ids including duplicates to preserve occurrence count

            # For data before v0.10.0
            pattern = r"/files/([a-f0-9\-]+)/image-preview(?:\?.*?)?"
            match = re.search(pattern, image)
            if match:
                upload_file_id = match.group(1)
                upload_file_id_list.append(upload_file_id)
                continue

            # For data after v0.10.0
            pattern = r"/files/([a-f0-9\-]+)/file-preview(?:\?.*?)?"
            match = re.search(pattern, image)
            if match:
                upload_file_id = match.group(1)
                upload_file_id_list.append(upload_file_id)
                continue

            # For tools directory - direct file formats (e.g., .png, .jpg, etc.)
            # Match URL including any query parameters up to common URL boundaries (space, parenthesis, quotes)
            pattern = r"/files/tools/([a-f0-9\-]+)\.([a-zA-Z0-9]+)(?:\?[^\s\)\"\']*)?"
            match = re.search(pattern, image)
            if match:
                if current_user:
                    tool_file_id = match.group(1)
                    upload_file_id = self._download_tool_file(tool_file_id, current_user)
                    if upload_file_id:
                        upload_file_id_list.append(upload_file_id)
                continue
            if current_user:
                upload_file_id = self._download_image(image.split(" ")[0], current_user)
                if upload_file_id:
                    upload_file_id_list.append(upload_file_id)

        if not upload_file_id_list:
            return multi_model_documents

        # Get unique IDs for database query
        unique_upload_file_ids = list(set(upload_file_id_list))
        upload_files = db.session.scalars(select(UploadFile).where(UploadFile.id.in_(unique_upload_file_ids))).all()

        # Create a mapping from ID to UploadFile for quick lookup
        upload_file_map = {upload_file.id: upload_file for upload_file in upload_files}

        # Create a Document for each occurrence (including duplicates)
        for upload_file_id in upload_file_id_list:
            upload_file = upload_file_map.get(upload_file_id)
            if upload_file:
                multi_model_documents.append(
                    AttachmentDocument(
                        page_content=upload_file.name,
                        metadata={
                            "doc_id": upload_file.id,
                            "doc_hash": "",
                            "document_id": document.metadata.get("document_id"),
                            "dataset_id": document.metadata.get("dataset_id"),
                            "doc_type": DocType.IMAGE,
                        },
                    )
                )
        return multi_model_documents

    def _extract_markdown_images(self, text: str) -> list[str]:
        """
        Extract the markdown images from the text.
        """
        pattern = r"!\[.*?\]\((.*?)\)"
        return re.findall(pattern, text)

    def _download_image(self, image_url: str, current_user: Account) -> str | None:
        """
        Download the image from the URL.
        Image size must not exceed 2MB.
        """
        from services.file_service import FileService

        MAX_IMAGE_SIZE = dify_config.ATTACHMENT_IMAGE_FILE_SIZE_LIMIT * 1024 * 1024
        DOWNLOAD_TIMEOUT = dify_config.ATTACHMENT_IMAGE_DOWNLOAD_TIMEOUT

        try:
            # Download with timeout
            response = ssrf_proxy.get(image_url, timeout=DOWNLOAD_TIMEOUT)
            response.raise_for_status()

            # Check Content-Length header if available
            content_length = response.headers.get("Content-Length")
            if content_length and int(content_length) > MAX_IMAGE_SIZE:
                logging.warning("Image from %s exceeds 2MB limit (size: %s bytes)", image_url, content_length)
                return None

            filename = None

            content_disposition = response.headers.get("content-disposition")
            if content_disposition:
                _, params = cgi.parse_header(content_disposition)
                if "filename" in params:
                    filename = params["filename"]
                    filename = unquote(filename)

            if not filename:
                parsed_url = urlparse(image_url)
                # Decode percent-encoded characters in the URL path.
                path = unquote(parsed_url.path)
                filename = os.path.basename(path)

            if not filename:
                filename = "downloaded_image_file"

            name, current_ext = os.path.splitext(filename)

            content_type = response.headers.get("content-type", "").split(";")[0].strip()

            real_ext = mimetypes.guess_extension(content_type)

            if not current_ext and real_ext or current_ext in [".php", ".jsp", ".asp", ".html"] and real_ext:
                filename = f"{name}{real_ext}"
            # Download content with size limit
            blob = b""
            for chunk in response.iter_bytes(chunk_size=8192):
                blob += chunk
                if len(blob) > MAX_IMAGE_SIZE:
                    logging.warning("Image from %s exceeds 2MB limit during download", image_url)
                    return None

            if not blob:
                logging.warning("Image from %s is empty", image_url)
                return None

            upload_file = FileService(db.engine).upload_file(
                filename=filename,
                content=blob,
                mimetype=content_type,
                user=current_user,
            )
            return upload_file.id
        except httpx.TimeoutException:
            logging.warning("Timeout downloading image from %s after %s seconds", image_url, DOWNLOAD_TIMEOUT)
            return None
        except httpx.RequestError as e:
            logging.warning("Error downloading image from %s: %s", image_url, str(e))
            return None
        except Exception:
            logging.warning("Unexpected error downloading image from %s", image_url, exc_info=True)
            return None

    def _download_tool_file(self, tool_file_id: str, current_user: Account) -> str | None:
        """
        Download the tool file from the ID.
        """
        from services.file_service import FileService

        tool_file = db.session.get(ToolFile, tool_file_id)
        if not tool_file:
            return None
        blob = storage.load_once(tool_file.file_key)
        upload_file = FileService(db.engine).upload_file(
            filename=tool_file.name,
            content=blob,
            mimetype=tool_file.mimetype,
            user=current_user,
        )
        return upload_file.id

```

### api/core/rag/summary_index/__init__.py
```py

```

### api/core/rag/summary_index/summary_index.py
```py
import concurrent.futures
import logging

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.index_processor.index_processor_base import SummaryIndexSettingDict
from models.dataset import Dataset, Document, DocumentSegment, DocumentSegmentSummary
from services.summary_index_service import SummaryIndexService
from tasks.generate_summary_index_task import generate_summary_index_task

logger = logging.getLogger(__name__)


class SummaryIndex:
    def generate_and_vectorize_summary(
        self,
        dataset_id: str,
        document_id: str,
        is_preview: bool,
        summary_index_setting: SummaryIndexSettingDict | None = None,
    ) -> None:
        if is_preview:
            with session_factory.create_session() as session:
                dataset = session.query(Dataset).filter_by(id=dataset_id).first()
                if not dataset or dataset.indexing_technique != IndexTechniqueType.HIGH_QUALITY:
                    return

                if summary_index_setting is None:
                    summary_index_setting = dataset.summary_index_setting

                if not summary_index_setting or not summary_index_setting.get("enable"):
                    return

                if not document_id:
                    return

                document = session.query(Document).filter_by(id=document_id).first()
                # Skip qa_model documents
                if document is None or document.doc_form == "qa_model":
                    return

                query = session.query(DocumentSegment).filter_by(
                    dataset_id=dataset_id,
                    document_id=document_id,
                    status="completed",
                    enabled=True,
                )
                segments = query.all()
                segment_ids = [segment.id for segment in segments]

                if not segment_ids:
                    return

                existing_summaries = (
                    session.query(DocumentSegmentSummary)
                    .filter(
                        DocumentSegmentSummary.chunk_id.in_(segment_ids),
                        DocumentSegmentSummary.dataset_id == dataset_id,
                        DocumentSegmentSummary.status == "completed",
                    )
                    .all()
                )
                completed_summary_segment_ids = {i.chunk_id for i in existing_summaries}
                # Preview mode should process segments that are MISSING completed summaries
                pending_segment_ids = [sid for sid in segment_ids if sid not in completed_summary_segment_ids]

            # If all segments already have completed summaries, nothing to do in preview mode
            if not pending_segment_ids:
                return

            max_workers = min(10, len(pending_segment_ids))

            def process_segment(segment_id: str) -> None:
                """Process a single segment in a thread with a fresh DB session."""
                with session_factory.create_session() as session:
                    segment = session.query(DocumentSegment).filter_by(id=segment_id).first()
                    if segment is None:
                        return
                    try:
                        SummaryIndexService.generate_and_vectorize_summary(segment, dataset, summary_index_setting)
                    except Exception:
                        logger.exception(
                            "Failed to generate summary for segment %s",
                            segment_id,
                        )
                        # Continue processing other segments

            with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
                futures = [executor.submit(process_segment, segment_id) for segment_id in pending_segment_ids]
                concurrent.futures.wait(futures)
        else:
            generate_summary_index_task.delay(dataset_id, document_id, None)

```

### api/core/rag/data_post_processor/__init__.py
```py

```

### api/core/rag/data_post_processor/data_post_processor.py
```py
from typing import TypedDict

from graphon.model_runtime.entities.model_entities import ModelType
from graphon.model_runtime.errors.invoke import InvokeAuthorizationError

from core.model_manager import ModelInstance, ModelManager
from core.rag.data_post_processor.reorder import ReorderRunner
from core.rag.index_processor.constant.query_type import QueryType
from core.rag.models.document import Document
from core.rag.rerank.entity.weight import KeywordSetting, VectorSetting, Weights
from core.rag.rerank.rerank_base import BaseRerankRunner
from core.rag.rerank.rerank_factory import RerankRunnerFactory
from core.rag.rerank.rerank_type import RerankMode


class RerankingModelDict(TypedDict):
    reranking_provider_name: str
    reranking_model_name: str


class VectorSettingDict(TypedDict):
    vector_weight: float
    embedding_provider_name: str
    embedding_model_name: str


class KeywordSettingDict(TypedDict):
    keyword_weight: float


class WeightsDict(TypedDict):
    vector_setting: VectorSettingDict
    keyword_setting: KeywordSettingDict


class DataPostProcessor:
    """Interface for data post-processing document."""

    def __init__(
        self,
        tenant_id: str,
        reranking_mode: str,
        reranking_model: RerankingModelDict | None = None,
        weights: WeightsDict | None = None,
        reorder_enabled: bool = False,
    ):
        self.rerank_runner = self._get_rerank_runner(reranking_mode, tenant_id, reranking_model, weights)
        self.reorder_runner = self._get_reorder_runner(reorder_enabled)

    def invoke(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ) -> list[Document]:
        if self.rerank_runner:
            documents = self.rerank_runner.run(query, documents, score_threshold, top_n, query_type)

        if self.reorder_runner:
            documents = self.reorder_runner.run(documents)

        return documents

    def _get_rerank_runner(
        self,
        reranking_mode: str,
        tenant_id: str,
        reranking_model: RerankingModelDict | None = None,
        weights: WeightsDict | None = None,
    ) -> BaseRerankRunner | None:
        if reranking_mode == RerankMode.WEIGHTED_SCORE and weights:
            runner = RerankRunnerFactory.create_rerank_runner(
                runner_type=reranking_mode,
                tenant_id=tenant_id,
                weights=Weights(
                    vector_setting=VectorSetting(
                        vector_weight=weights["vector_setting"]["vector_weight"],
                        embedding_provider_name=weights["vector_setting"]["embedding_provider_name"],
                        embedding_model_name=weights["vector_setting"]["embedding_model_name"],
                    ),
                    keyword_setting=KeywordSetting(
                        keyword_weight=weights["keyword_setting"]["keyword_weight"],
                    ),
                ),
            )
            return runner
        elif reranking_mode == RerankMode.RERANKING_MODEL:
            rerank_model_instance = self._get_rerank_model_instance(tenant_id, reranking_model)
            if rerank_model_instance is None:
                return None
            runner = RerankRunnerFactory.create_rerank_runner(
                runner_type=reranking_mode, rerank_model_instance=rerank_model_instance
            )
            return runner
        return None

    def _get_reorder_runner(self, reorder_enabled) -> ReorderRunner | None:
        if reorder_enabled:
            return ReorderRunner()
        return None

    def _get_rerank_model_instance(
        self, tenant_id: str, reranking_model: RerankingModelDict | None
    ) -> ModelInstance | None:
        if reranking_model:
            try:
                model_manager = ModelManager.for_tenant(tenant_id=tenant_id)
                reranking_provider_name = reranking_model.get("reranking_provider_name")
                reranking_model_name = reranking_model.get("reranking_model_name")
                if not reranking_provider_name or not reranking_model_name:
                    return None
                rerank_model_instance = model_manager.get_model_instance(
                    tenant_id=tenant_id,
                    provider=reranking_provider_name,
                    model_type=ModelType.RERANK,
                    model=reranking_model_name,
                )
                return rerank_model_instance
            except InvokeAuthorizationError:
                return None
        return None

```

### api/core/rag/data_post_processor/reorder.py
```py
from core.rag.models.document import Document


class ReorderRunner:
    def run(self, documents: list[Document]) -> list[Document]:
        # Retrieve elements from odd indices (0, 2, 4, etc.) of the documents list
        odd_elements = documents[::2]

        # Retrieve elements from even indices (1, 3, 5, etc.) of the documents list
        even_elements = documents[1::2]

        # Reverse the list of elements from even indices
        even_elements_reversed = even_elements[::-1]

        new_documents = odd_elements + even_elements_reversed

        return new_documents

```

### api/core/rag/embedding/__init__.py
```py

```

### api/core/rag/embedding/cached_embedding.py
```py
import base64
import logging
import pickle
from typing import Any, cast

import numpy as np
from graphon.model_runtime.entities.model_entities import ModelPropertyKey
from graphon.model_runtime.model_providers.__base.text_embedding_model import TextEmbeddingModel
from sqlalchemy import select
from sqlalchemy.exc import IntegrityError

from configs import dify_config
from core.entities.embedding_type import EmbeddingInputType
from core.model_manager import ModelInstance
from core.rag.embedding.embedding_base import Embeddings
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from libs import helper
from models.dataset import Embedding

logger = logging.getLogger(__name__)


class CacheEmbedding(Embeddings):
    def __init__(self, model_instance: ModelInstance):
        self._model_instance = model_instance

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed search docs in batches of 10."""
        # use doc embedding cache or store if not exists
        text_embeddings: list[Any] = [None for _ in range(len(texts))]
        embedding_queue_indices = []
        for i, text in enumerate(texts):
            hash = helper.generate_text_hash(text)
            embedding = db.session.scalar(
                select(Embedding)
                .where(
                    Embedding.model_name == self._model_instance.model_name,
                    Embedding.hash == hash,
                    Embedding.provider_name == self._model_instance.provider,
                )
                .limit(1)
            )
            if embedding:
                text_embeddings[i] = embedding.get_embedding()
            else:
                embedding_queue_indices.append(i)

        # NOTE: avoid closing the shared scoped session here; downstream code may still have pending work

        if embedding_queue_indices:
            embedding_queue_texts = [texts[i] for i in embedding_queue_indices]
            embedding_queue_embeddings = []
            try:
                model_type_instance = cast(TextEmbeddingModel, self._model_instance.model_type_instance)
                model_schema = model_type_instance.get_model_schema(
                    self._model_instance.model_name, self._model_instance.credentials
                )
                max_chunks = (
                    model_schema.model_properties[ModelPropertyKey.MAX_CHUNKS]
                    if model_schema and ModelPropertyKey.MAX_CHUNKS in model_schema.model_properties
                    else 1
                )
                for i in range(0, len(embedding_queue_texts), max_chunks):
                    batch_texts = embedding_queue_texts[i : i + max_chunks]

                    embedding_result = self._model_instance.invoke_text_embedding(
                        texts=batch_texts, input_type=EmbeddingInputType.DOCUMENT
                    )

                    for vector in embedding_result.embeddings:
                        try:
                            # FIXME: type ignore for numpy here
                            normalized_embedding = (vector / np.linalg.norm(vector)).tolist()  # type: ignore
                            # stackoverflow best way: https://stackoverflow.com/questions/20319813/how-to-check-list-containing-nan
                            if np.isnan(normalized_embedding).any():
                                # for issue #11827  float values are not json compliant
                                logger.warning("Normalized embedding is nan: %s", normalized_embedding)
                                continue
                            embedding_queue_embeddings.append(normalized_embedding)
                        except IntegrityError:
                            db.session.rollback()
                        except Exception:
                            logger.exception("Failed transform embedding")
                cache_embeddings = []
                try:
                    for i, n_embedding in zip(embedding_queue_indices, embedding_queue_embeddings):
                        text_embeddings[i] = n_embedding
                        hash = helper.generate_text_hash(texts[i])
                        if hash not in cache_embeddings:
                            embedding_cache = Embedding(
                                model_name=self._model_instance.model_name,
                                hash=hash,
                                provider_name=self._model_instance.provider,
                                embedding=pickle.dumps(n_embedding, protocol=pickle.HIGHEST_PROTOCOL),
                            )
                            db.session.add(embedding_cache)
                            cache_embeddings.append(hash)
                    db.session.commit()
                except IntegrityError:
                    db.session.rollback()
            except Exception as ex:
                db.session.rollback()
                logger.exception("Failed to embed documents")
                raise ex

        return text_embeddings

    def embed_multimodal_documents(self, multimodel_documents: list[dict]) -> list[list[float]]:
        """Embed file documents."""
        # use doc embedding cache or store if not exists
        multimodel_embeddings: list[Any] = [None for _ in range(len(multimodel_documents))]
        embedding_queue_indices = []
        for i, multimodel_document in enumerate(multimodel_documents):
            file_id = multimodel_document["file_id"]
            embedding = db.session.scalar(
                select(Embedding)
                .where(
                    Embedding.model_name == self._model_instance.model_name,
                    Embedding.hash == file_id,
                    Embedding.provider_name == self._model_instance.provider,
                )
                .limit(1)
            )
            if embedding:
                multimodel_embeddings[i] = embedding.get_embedding()
            else:
                embedding_queue_indices.append(i)

        # NOTE: avoid closing the shared scoped session here; downstream code may still have pending work

        if embedding_queue_indices:
            embedding_queue_multimodel_documents = [multimodel_documents[i] for i in embedding_queue_indices]
            embedding_queue_embeddings = []
            try:
                model_type_instance = cast(TextEmbeddingModel, self._model_instance.model_type_instance)
                model_schema = model_type_instance.get_model_schema(
                    self._model_instance.model_name, self._model_instance.credentials
                )
                max_chunks = (
                    model_schema.model_properties[ModelPropertyKey.MAX_CHUNKS]
                    if model_schema and ModelPropertyKey.MAX_CHUNKS in model_schema.model_properties
                    else 1
                )
                for i in range(0, len(embedding_queue_multimodel_documents), max_chunks):
                    batch_multimodel_documents = embedding_queue_multimodel_documents[i : i + max_chunks]

                    embedding_result = self._model_instance.invoke_multimodal_embedding(
                        multimodel_documents=batch_multimodel_documents,
                        input_type=EmbeddingInputType.DOCUMENT,
                    )

                    for vector in embedding_result.embeddings:
                        try:
                            # FIXME: type ignore for numpy here
                            normalized_embedding = (vector / np.linalg.norm(vector)).tolist()  # type: ignore
                            # stackoverflow best way: https://stackoverflow.com/questions/20319813/how-to-check-list-containing-nan
                            if np.isnan(normalized_embedding).any():
                                # for issue #11827  float values are not json compliant
                                logger.warning("Normalized embedding is nan: %s", normalized_embedding)
                                continue
                            embedding_queue_embeddings.append(normalized_embedding)
                        except IntegrityError:
                            db.session.rollback()
                        except Exception:
                            logger.exception("Failed transform embedding")
                cache_embeddings = []
                try:
                    for i, n_embedding in zip(embedding_queue_indices, embedding_queue_embeddings):
                        multimodel_embeddings[i] = n_embedding
                        file_id = multimodel_documents[i]["file_id"]
                        if file_id not in cache_embeddings:
                            embedding_cache = Embedding(
                                model_name=self._model_instance.model_name,
                                hash=file_id,
                                provider_name=self._model_instance.provider,
                                embedding=pickle.dumps(n_embedding, protocol=pickle.HIGHEST_PROTOCOL),
                            )
                            embedding_cache.set_embedding(n_embedding)
                            db.session.add(embedding_cache)
                            cache_embeddings.append(file_id)
                    db.session.commit()
                except IntegrityError:
                    db.session.rollback()
            except Exception as ex:
                db.session.rollback()
                logger.exception("Failed to embed documents")
                raise ex

        return multimodel_embeddings

    def embed_query(self, text: str) -> list[float]:
        """Embed query text."""
        # use doc embedding cache or store if not exists
        hash = helper.generate_text_hash(text)
        embedding_cache_key = f"{self._model_instance.provider}_{self._model_instance.model_name}_{hash}"
        embedding = redis_client.get(embedding_cache_key)
        if embedding:
            redis_client.expire(embedding_cache_key, 600)
            decoded_embedding = np.frombuffer(base64.b64decode(embedding), dtype="float")
            return [float(x) for x in decoded_embedding]
        try:
            embedding_result = self._model_instance.invoke_text_embedding(
                texts=[text], input_type=EmbeddingInputType.QUERY
            )

            embedding_results = embedding_result.embeddings[0]
            # FIXME: type ignore for numpy here
            embedding_results = (embedding_results / np.linalg.norm(embedding_results)).tolist()  # type: ignore
            if np.isnan(embedding_results).any():
                raise ValueError("Normalized embedding is nan please try again")
        except Exception as ex:
            if dify_config.DEBUG:
                logger.exception("Failed to embed query text '%s...(%s chars)'", text[:10], len(text))
            raise ex

        try:
            # encode embedding to base64
            embedding_vector = np.array(embedding_results)
            vector_bytes = embedding_vector.tobytes()
            # Transform to Base64
            encoded_vector = base64.b64encode(vector_bytes)
            # Transform to string
            encoded_str = encoded_vector.decode("utf-8")
            redis_client.setex(embedding_cache_key, 600, encoded_str)
        except Exception as ex:
            if dify_config.DEBUG:
                logger.exception(
                    "Failed to add embedding to redis for the text '%s...(%s chars)'", text[:10], len(text)
                )
            raise ex

        return embedding_results  # type: ignore

    def embed_multimodal_query(self, multimodel_document: dict) -> list[float]:
        """Embed multimodal documents."""
        # use doc embedding cache or store if not exists
        file_id = multimodel_document["file_id"]
        embedding_cache_key = f"{self._model_instance.provider}_{self._model_instance.model_name}_{file_id}"
        embedding = redis_client.get(embedding_cache_key)
        if embedding:
            redis_client.expire(embedding_cache_key, 600)
            decoded_embedding = np.frombuffer(base64.b64decode(embedding), dtype="float")
            return [float(x) for x in decoded_embedding]
        try:
            embedding_result = self._model_instance.invoke_multimodal_embedding(
                multimodel_documents=[multimodel_document], input_type=EmbeddingInputType.QUERY
            )

            embedding_results = embedding_result.embeddings[0]
            # FIXME: type ignore for numpy here
            embedding_results = (embedding_results / np.linalg.norm(embedding_results)).tolist()  # type: ignore
            if np.isnan(embedding_results).any():
                raise ValueError("Normalized embedding is nan please try again")
        except Exception as ex:
            if dify_config.DEBUG:
                logger.exception("Failed to embed multimodal document '%s'", multimodel_document["file_id"])
            raise ex

        try:
            # encode embedding to base64
            embedding_vector = np.array(embedding_results)
            vector_bytes = embedding_vector.tobytes()
            # Transform to Base64
            encoded_vector = base64.b64encode(vector_bytes)
            # Transform to string
            encoded_str = encoded_vector.decode("utf-8")
            redis_client.setex(embedding_cache_key, 600, encoded_str)
        except Exception as ex:
            if dify_config.DEBUG:
                logger.exception(
                    "Failed to add embedding to redis for the multimodal document '%s'", multimodel_document["file_id"]
                )
            raise ex

        return embedding_results  # type: ignore

```

### api/core/rag/embedding/retrieval.py
```py
from typing import TypedDict

from pydantic import BaseModel

from models.dataset import DocumentSegment


class AttachmentInfoDict(TypedDict):
    id: str
    name: str
    extension: str
    mime_type: str
    source_url: str
    size: int


class RetrievalChildChunk(BaseModel):
    """Retrieval segments."""

    id: str
    content: str
    score: float
    position: int


class RetrievalSegments(BaseModel):
    """Retrieval segments."""

    model_config = {"arbitrary_types_allowed": True}
    segment: DocumentSegment
    child_chunks: list[RetrievalChildChunk] | None = None
    score: float | None = None
    files: list[AttachmentInfoDict] | None = None
    summary: str | None = None  # Summary content if retrieved via summary index

```

### api/core/rag/embedding/embedding_base.py
```py
from abc import ABC, abstractmethod


class Embeddings(ABC):
    """Interface for embedding models."""

    @abstractmethod
    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed search docs."""
        raise NotImplementedError

    @abstractmethod
    def embed_multimodal_documents(self, multimodel_documents: list[dict]) -> list[list[float]]:
        """Embed file documents."""
        raise NotImplementedError

    @abstractmethod
    def embed_query(self, text: str) -> list[float]:
        """Embed query text."""
        raise NotImplementedError

    @abstractmethod
    def embed_multimodal_query(self, multimodel_document: dict) -> list[float]:
        """Embed multimodal query."""
        raise NotImplementedError

    async def aembed_documents(self, texts: list[str]) -> list[list[float]]:
        """Asynchronous Embed search docs."""
        raise NotImplementedError

    async def aembed_query(self, text: str) -> list[float]:
        """Asynchronous Embed query text."""
        raise NotImplementedError

```

### api/core/rag/splitter/text_splitter.py
```py
from __future__ import annotations

import copy
import logging
import re
from abc import ABC, abstractmethod
from collections.abc import Callable, Collection, Iterable, Sequence, Set
from dataclasses import dataclass
from typing import Any, Literal

from core.rag.models.document import BaseDocumentTransformer, Document

logger = logging.getLogger(__name__)


def _split_text_with_regex(text: str, separator: str, keep_separator: bool) -> list[str]:
    # Now that we have the separator, split the text
    if separator:
        if keep_separator:
            # The parentheses in the pattern keep the delimiters in the result.
            _splits = re.split(f"({re.escape(separator)})", text)
            splits = [_splits[i - 1] + _splits[i] for i in range(1, len(_splits), 2)]
            if len(_splits) % 2 != 0:
                splits += _splits[-1:]
        else:
            splits = re.split(separator, text)
    else:
        splits = list(text)
    return [s for s in splits if (s not in {"", "\n"})]


class TextSplitter(BaseDocumentTransformer, ABC):
    """Interface for splitting text into chunks."""

    def __init__(
        self,
        chunk_size: int = 4000,
        chunk_overlap: int = 200,
        length_function: Callable[[list[str]], list[int]] = lambda x: [len(x) for x in x],
        keep_separator: bool = False,
        add_start_index: bool = False,
    ):
        """Create a new TextSplitter.

        Args:
            chunk_size: Maximum size of chunks to return
            chunk_overlap: Overlap in characters between chunks
            length_function: Function that measures the length of given chunks
            keep_separator: Whether to keep the separator in the chunks
            add_start_index: If `True`, includes chunk's start index in metadata
        """
        if chunk_overlap > chunk_size:
            raise ValueError(
                f"Got a larger chunk overlap ({chunk_overlap}) than chunk size ({chunk_size}), should be smaller."
            )
        self._chunk_size = chunk_size
        self._chunk_overlap = chunk_overlap
        self._length_function = length_function
        self._keep_separator = keep_separator
        self._add_start_index = add_start_index

    @abstractmethod
    def split_text(self, text: str) -> list[str]:
        """Split text into multiple components."""

    def create_documents(self, texts: list[str], metadatas: list[dict] | None = None) -> list[Document]:
        """Create documents from a list of texts."""
        _metadatas = metadatas or [{}] * len(texts)
        documents = []
        for i, text in enumerate(texts):
            index = -1
            for chunk in self.split_text(text):
                metadata = copy.deepcopy(_metadatas[i])
                if self._add_start_index:
                    index = text.find(chunk, index + 1)
                    metadata["start_index"] = index
                new_doc = Document(page_content=chunk, metadata=metadata)
                documents.append(new_doc)
        return documents

    def split_documents(self, documents: Iterable[Document]) -> list[Document]:
        """Split documents."""
        texts, metadatas = [], []
        for doc in documents:
            texts.append(doc.page_content)
            metadatas.append(doc.metadata or {})
        return self.create_documents(texts, metadatas=metadatas)

    def _join_docs(self, docs: list[str], separator: str) -> str | None:
        text = separator.join(docs)
        text = text.strip()
        if text == "":
            return None
        else:
            return text

    def _merge_splits(self, splits: Iterable[str], separator: str, lengths: list[int]) -> list[str]:
        # We now want to combine these smaller pieces into medium size
        # chunks to send to the LLM.
        separator_len = self._length_function([separator])[0]

        docs = []
        current_doc: list[str] = []
        total = 0
        for d, _len in zip(splits, lengths):
            if total + _len + (separator_len if len(current_doc) > 0 else 0) > self._chunk_size:
                if total > self._chunk_size:
                    logger.warning(
                        "Created a chunk of size %s, which is longer than the specified %s", total, self._chunk_size
                    )
                if len(current_doc) > 0:
                    doc = self._join_docs(current_doc, separator)
                    if doc is not None:
                        docs.append(doc)
                    # Keep on popping if:
                    # - we have a larger chunk than in the chunk overlap
                    # - or if we still have any chunks and the length is long
                    while total > self._chunk_overlap or (
                        total + _len + (separator_len if len(current_doc) > 0 else 0) > self._chunk_size and total > 0
                    ):
                        total -= self._length_function([current_doc[0]])[0] + (
                            separator_len if len(current_doc) > 1 else 0
                        )
                        current_doc = current_doc[1:]
            current_doc.append(d)
            total += _len + (separator_len if len(current_doc) > 1 else 0)
        doc = self._join_docs(current_doc, separator)
        if doc is not None:
            docs.append(doc)
        return docs

    @classmethod
    def from_huggingface_tokenizer(cls, tokenizer: Any, **kwargs: Any) -> TextSplitter:
        """Text splitter that uses HuggingFace tokenizer to count length."""
        try:
            from transformers import PreTrainedTokenizerBase

            if not isinstance(tokenizer, PreTrainedTokenizerBase):
                raise ValueError("Tokenizer received was not an instance of PreTrainedTokenizerBase")

            def _huggingface_tokenizer_length(text: str) -> int:
                return len(tokenizer.encode(text))

        except ImportError:
            raise ValueError(
                "Could not import transformers python package. Please install it with `pip install transformers`."
            )
        return cls(length_function=lambda x: [_huggingface_tokenizer_length(text) for text in x], **kwargs)

    def transform_documents(self, documents: Sequence[Document], **kwargs: Any) -> Sequence[Document]:
        """Transform sequence of documents by splitting them."""
        return self.split_documents(list(documents))

    async def atransform_documents(self, documents: Sequence[Document], **kwargs: Any) -> Sequence[Document]:
        """Asynchronously transform a sequence of documents by splitting them."""
        raise NotImplementedError


# @dataclass(frozen=True, kw_only=True, slots=True)
@dataclass(frozen=True)
class Tokenizer:
    chunk_overlap: int
    tokens_per_chunk: int
    decode: Callable[[list[int]], str]
    encode: Callable[[str], list[int]]


def split_text_on_tokens(*, text: str, tokenizer: Tokenizer) -> list[str]:
    """Split incoming text and return chunks using tokenizer."""
    splits: list[str] = []
    input_ids = tokenizer.encode(text)
    start_idx = 0
    cur_idx = min(start_idx + tokenizer.tokens_per_chunk, len(input_ids))
    chunk_ids = input_ids[start_idx:cur_idx]
    while start_idx < len(input_ids):
        splits.append(tokenizer.decode(chunk_ids))
        start_idx += tokenizer.tokens_per_chunk - tokenizer.chunk_overlap
        cur_idx = min(start_idx + tokenizer.tokens_per_chunk, len(input_ids))
        chunk_ids = input_ids[start_idx:cur_idx]
    return splits


class TokenTextSplitter(TextSplitter):
    """Splitting text to tokens using model tokenizer."""

    def __init__(
        self,
        encoding_name: str = "gpt2",
        model_name: str | None = None,
        allowed_special: Literal["all"] | Set[str] = set(),
        disallowed_special: Literal["all"] | Collection[str] = "all",
        **kwargs: Any,
    ):
        """Create a new TextSplitter."""
        super().__init__(**kwargs)
        try:
            import tiktoken
        except ImportError:
            raise ImportError(
                "Could not import tiktoken python package. "
                "This is needed in order to for TokenTextSplitter. "
                "Please install it with `pip install tiktoken`."
            )

        if model_name is not None:
            enc = tiktoken.encoding_for_model(model_name)
        else:
            enc = tiktoken.get_encoding(encoding_name)
        self._tokenizer = enc
        self._allowed_special = allowed_special
        self._disallowed_special = disallowed_special

    def split_text(self, text: str) -> list[str]:
        def _encode(_text: str) -> list[int]:
            return self._tokenizer.encode(
                _text,
                allowed_special=self._allowed_special,
                disallowed_special=self._disallowed_special,
            )

        tokenizer = Tokenizer(
            chunk_overlap=self._chunk_overlap,
            tokens_per_chunk=self._chunk_size,
            decode=self._tokenizer.decode,
            encode=_encode,
        )

        return split_text_on_tokens(text=text, tokenizer=tokenizer)


class RecursiveCharacterTextSplitter(TextSplitter):
    """Splitting text by recursively look at characters.

    Recursively tries to split by different characters to find one
    that works.
    """

    def __init__(
        self,
        separators: list[str] | None = None,
        keep_separator: bool = True,
        **kwargs: Any,
    ):
        """Create a new TextSplitter."""
        super().__init__(keep_separator=keep_separator, **kwargs)
        self._separators = separators or ["\n\n", "\n", " ", ""]

    def _split_text(self, text: str, separators: list[str]) -> list[str]:
        final_chunks = []
        separator = separators[-1]
        new_separators = []

        for i, _s in enumerate(separators):
            if _s == "":
                separator = _s
                break
            if re.search(_s, text):
                separator = _s
                new_separators = separators[i + 1 :]
                break

        splits = _split_text_with_regex(text, separator, self._keep_separator)
        _good_splits = []
        _good_splits_lengths = []  # cache the lengths of the splits
        _separator = "" if self._keep_separator else separator
        s_lens = self._length_function(splits)
        for s, s_len in zip(splits, s_lens):
            if s_len < self._chunk_size:
                _good_splits.append(s)
                _good_splits_lengths.append(s_len)
            else:
                if _good_splits:
                    merged_text = self._merge_splits(_good_splits, _separator, _good_splits_lengths)
                    final_chunks.extend(merged_text)
                    _good_splits = []
                    _good_splits_lengths = []
                if not new_separators:
                    final_chunks.append(s)
                else:
                    other_info = self._split_text(s, new_separators)
                    final_chunks.extend(other_info)

        if _good_splits:
            merged_text = self._merge_splits(_good_splits, _separator, _good_splits_lengths)
            final_chunks.extend(merged_text)

        return final_chunks

    def split_text(self, text: str) -> list[str]:
        return self._split_text(text, self._separators)

```

### api/core/rag/splitter/__init__.py
```py

```

### api/core/rag/splitter/fixed_text_splitter.py
```py
"""Functionality for splitting text."""

from __future__ import annotations

import codecs
import re
from collections.abc import Collection
from typing import Any, Literal

from graphon.model_runtime.model_providers.__base.tokenizers.gpt2_tokenizer import GPT2Tokenizer

from core.model_manager import ModelInstance
from core.rag.splitter.text_splitter import RecursiveCharacterTextSplitter


class EnhanceRecursiveCharacterTextSplitter(RecursiveCharacterTextSplitter):
    """
    This class is used to implement from_gpt2_encoder, to prevent using of tiktoken
    """

    @classmethod
    def from_encoder[T: EnhanceRecursiveCharacterTextSplitter](
        cls: type[T],
        embedding_model_instance: ModelInstance | None,
        allowed_special: Literal["all"] | set[str] = set(),
        disallowed_special: Literal["all"] | Collection[str] = "all",
        **kwargs: Any,
    ) -> T:
        def _token_encoder(texts: list[str]) -> list[int]:
            if not texts:
                return []

            if embedding_model_instance:
                return embedding_model_instance.get_text_embedding_num_tokens(texts=texts)
            else:
                return [GPT2Tokenizer.get_num_tokens(text) for text in texts]

        def _character_encoder(texts: list[str]) -> list[int]:
            if not texts:
                return []

            return [len(text) for text in texts]

        return cls(length_function=_character_encoder, **kwargs)


class FixedRecursiveCharacterTextSplitter(EnhanceRecursiveCharacterTextSplitter):
    def __init__(self, fixed_separator: str = "\n\n", separators: list[str] | None = None, **kwargs: Any):
        """Create a new TextSplitter."""
        super().__init__(**kwargs)
        self._fixed_separator = codecs.decode(fixed_separator, "unicode_escape")
        self._separators = separators or ["\n\n", "\n", "。", ". ", " ", ""]

    def split_text(self, text: str) -> list[str]:
        """Split incoming text and return chunks."""
        if self._fixed_separator:
            chunks = text.split(self._fixed_separator)
        else:
            chunks = [text]

        final_chunks = []
        chunks_lengths = self._length_function(chunks)
        for chunk, chunk_length in zip(chunks, chunks_lengths):
            if chunk_length > self._chunk_size:
                final_chunks.extend(self.recursive_split_text(chunk))
            else:
                final_chunks.append(chunk)

        return final_chunks

    def recursive_split_text(self, text: str) -> list[str]:
        """Split incoming text and return chunks."""

        final_chunks = []
        separator = self._separators[-1]
        new_separators = []

        for i, _s in enumerate(self._separators):
            if _s == "":
                separator = _s
                break
            if _s in text:
                separator = _s
                new_separators = self._separators[i + 1 :]
                break

        # Now that we have the separator, split the text
        if separator:
            if separator == " ":
                splits = re.split(r" +", text)
            else:
                splits = text.split(separator)
                if self._keep_separator:
                    splits = [s + separator for s in splits[:-1]] + splits[-1:]
        else:
            splits = list(text)
        if separator == "\n":
            splits = [s for s in splits if s != ""]
        else:
            splits = [s for s in splits if (s not in {"", "\n"})]
        _good_splits = []
        _good_splits_lengths = []  # cache the lengths of the splits
        _separator = "" if self._keep_separator else separator
        s_lens = self._length_function(splits)
        if separator != "":
            for s, s_len in zip(splits, s_lens):
                if s_len < self._chunk_size:
                    _good_splits.append(s)
                    _good_splits_lengths.append(s_len)
                else:
                    if _good_splits:
                        merged_text = self._merge_splits(_good_splits, _separator, _good_splits_lengths)
                        final_chunks.extend(merged_text)
                        _good_splits = []
                        _good_splits_lengths = []
                    if not new_separators:
                        final_chunks.append(s)
                    else:
                        other_info = self._split_text(s, new_separators)
                        final_chunks.extend(other_info)

            if _good_splits:
                merged_text = self._merge_splits(_good_splits, _separator, _good_splits_lengths)
                final_chunks.extend(merged_text)
        else:
            current_part = ""
            current_length = 0
            overlap_part = ""
            overlap_part_length = 0
            for s, s_len in zip(splits, s_lens):
                if current_length + s_len <= self._chunk_size - self._chunk_overlap:
                    current_part += s
                    current_length += s_len
                elif current_length + s_len <= self._chunk_size:
                    current_part += s
                    current_length += s_len
                    overlap_part += s
                    overlap_part_length += s_len
                else:
                    final_chunks.append(current_part)
                    current_part = overlap_part + s
                    current_length = s_len + overlap_part_length
                    overlap_part = ""
                    overlap_part_length = 0
            if current_part:
                final_chunks.append(current_part)

        return final_chunks

```

### api/core/rag/__init__.py
```py

```

### api/core/rag/extractor/watercrawl/provider.py
```py
from collections.abc import Generator
from datetime import datetime
from typing import Any, TypedDict

from core.rag.extractor.watercrawl.client import PageOptions, SpiderOptions, WaterCrawlAPIClient


class WatercrawlDocumentData(TypedDict):
    title: str | None
    description: str | None
    source_url: str | None
    markdown: str | None


class CrawlJobResponse(TypedDict):
    status: str
    job_id: str | None


class WatercrawlCrawlStatusResponse(TypedDict):
    status: str
    job_id: str | None
    total: int
    current: int
    data: list[WatercrawlDocumentData]
    time_consuming: float


class WaterCrawlProvider:
    def __init__(self, api_key, base_url: str | None = None):
        self.client = WaterCrawlAPIClient(api_key, base_url)

    def crawl_url(self, url: str, options: dict[str, Any] | None = None) -> CrawlJobResponse:
        options = options or {}
        spider_options: SpiderOptions = {
            "max_depth": 1,
            "page_limit": 1,
            "allowed_domains": [],
            "exclude_paths": [],
            "include_paths": [],
        }
        if options.get("crawl_sub_pages", True):
            spider_options["page_limit"] = options.get("limit", 1)
            spider_options["max_depth"] = options.get("max_depth", 1)
            spider_options["include_paths"] = options.get("includes", "").split(",") if options.get("includes") else []
            spider_options["exclude_paths"] = options.get("excludes", "").split(",") if options.get("excludes") else []

        wait_time = options.get("wait_time", 1000)
        page_options: PageOptions = {
            "exclude_tags": options.get("exclude_tags", "").split(",") if options.get("exclude_tags") else [],
            "include_tags": options.get("include_tags", "").split(",") if options.get("include_tags") else [],
            "wait_time": max(1000, wait_time),  # minimum wait time is 1 second
            "include_html": False,
            "only_main_content": options.get("only_main_content", True),
            "include_links": False,
            "timeout": 15000,
            "accept_cookies_selector": "#cookies-accept",
            "locale": "en-US",
            "actions": [],
        }
        result = self.client.create_crawl_request(url=url, spider_options=spider_options, page_options=page_options)

        return {"status": "active", "job_id": result.get("uuid")}

    def get_crawl_status(self, crawl_request_id: str) -> WatercrawlCrawlStatusResponse:
        response = self.client.get_crawl_request(crawl_request_id)
        data: list[WatercrawlDocumentData] = []
        if response["status"] in ["new", "running"]:
            status = "active"
        else:
            status = "completed"
            data = list(self._get_results(crawl_request_id))

        time_str = response.get("duration")
        time_consuming: float = 0
        if time_str:
            time_obj = datetime.strptime(time_str, "%H:%M:%S.%f")
            time_consuming = (
                time_obj.hour * 3600 + time_obj.minute * 60 + time_obj.second + time_obj.microsecond / 1_000_000
            )

        return {
            "status": status,
            "job_id": response.get("uuid"),
            "total": response.get("options", {}).get("spider_options", {}).get("page_limit", 1),
            "current": response.get("number_of_documents", 0),
            "data": data,
            "time_consuming": time_consuming,
        }

    def get_crawl_url_data(self, job_id: str, url: str) -> WatercrawlDocumentData | None:
        if not job_id:
            return self.scrape_url(url)

        for result in self._get_results(
            job_id,
            {
                # filter by url
                "url": url
            },
        ):
            return result

        return None

    def scrape_url(self, url: str) -> WatercrawlDocumentData:
        response = self.client.scrape_url(url=url, sync=True, prefetched=True)
        return self._structure_data(response)

    def _structure_data(self, result_object: dict[str, Any]) -> WatercrawlDocumentData:
        if isinstance(result_object.get("result", {}), str):
            raise ValueError("Invalid result object. Expected a dictionary.")

        metadata = result_object.get("result", {}).get("metadata", {})
        return {
            "title": metadata.get("og:title") or metadata.get("title"),
            "description": metadata.get("description"),
            "source_url": result_object.get("url"),
            "markdown": result_object.get("result", {}).get("markdown"),
        }

    def _get_results(
        self, crawl_request_id: str, query_params: dict | None = None
    ) -> Generator[WatercrawlDocumentData, None, None]:
        page = 0
        page_size = 100

        query_params = query_params or {}
        query_params.update({"prefetched": "true"})
        while True:
            page += 1
            response = self.client.get_crawl_request_results(crawl_request_id, page, page_size, query_params)
            if not response["results"]:
                break

            for result in response["results"]:
                yield self._structure_data(result)

            if response["next"] is None:
                break

```

### api/core/rag/extractor/watercrawl/client.py
```py
import json
from collections.abc import Generator
from typing import Any, TypedDict
from urllib.parse import urljoin

import httpx
from httpx import Response

from core.rag.extractor.watercrawl.exceptions import (
    WaterCrawlAuthenticationError,
    WaterCrawlBadRequestError,
    WaterCrawlPermissionError,
)


class SpiderOptions(TypedDict):
    max_depth: int
    page_limit: int
    allowed_domains: list[str]
    exclude_paths: list[str]
    include_paths: list[str]


class PageOptions(TypedDict):
    exclude_tags: list[str]
    include_tags: list[str]
    wait_time: int
    include_html: bool
    only_main_content: bool
    include_links: bool
    timeout: int
    accept_cookies_selector: str
    locale: str
    actions: list[Any]


class BaseAPIClient:
    def __init__(self, api_key, base_url):
        self.api_key = api_key
        self.base_url = base_url
        self.session = self.init_session()

    def init_session(self):
        headers = {
            "X-API-Key": self.api_key,
            "Content-Type": "application/json",
            "Accept": "application/json",
            "User-Agent": "WaterCrawl-Plugin",
            "Accept-Language": "en-US",
        }
        return httpx.Client(headers=headers, timeout=None)

    def _request(
        self,
        method: str,
        endpoint: str,
        query_params: dict | None = None,
        data: dict | None = None,
        **kwargs,
    ) -> Response:
        stream = kwargs.pop("stream", False)
        url = urljoin(self.base_url, endpoint)
        if stream:
            request = self.session.build_request(method, url, params=query_params, json=data)
            return self.session.send(request, stream=True, **kwargs)

        return self.session.request(method, url, params=query_params, json=data, **kwargs)

    def _get(self, endpoint: str, query_params: dict | None = None, **kwargs):
        return self._request("GET", endpoint, query_params=query_params, **kwargs)

    def _post(self, endpoint: str, query_params: dict | None = None, data: dict | None = None, **kwargs):
        return self._request("POST", endpoint, query_params=query_params, data=data, **kwargs)

    def _put(self, endpoint: str, query_params: dict | None = None, data: dict | None = None, **kwargs):
        return self._request("PUT", endpoint, query_params=query_params, data=data, **kwargs)

    def _delete(self, endpoint: str, query_params: dict | None = None, **kwargs):
        return self._request("DELETE", endpoint, query_params=query_params, **kwargs)

    def _patch(self, endpoint: str, query_params: dict | None = None, data: dict | None = None, **kwargs):
        return self._request("PATCH", endpoint, query_params=query_params, data=data, **kwargs)


class WaterCrawlAPIClient(BaseAPIClient):
    def __init__(self, api_key, base_url: str | None = "https://app.watercrawl.dev/"):
        super().__init__(api_key, base_url)

    def process_eventstream(self, response: Response, download: bool = False) -> Generator:
        try:
            for raw_line in response.iter_lines():
                line = raw_line.decode("utf-8") if isinstance(raw_line, bytes) else raw_line
                if line.startswith("data:"):
                    line = line[5:].strip()
                    data = json.loads(line)
                    if data["type"] == "result" and download:
                        data["data"] = self.download_result(data["data"])
                    yield data
        finally:
            response.close()

    def process_response(self, response: Response) -> dict | bytes | list | None | Generator:
        if response.status_code == 401:
            raise WaterCrawlAuthenticationError(response)

        if response.status_code == 403:
            raise WaterCrawlPermissionError(response)

        if 400 <= response.status_code < 500:
            raise WaterCrawlBadRequestError(response)

        response.raise_for_status()
        if response.status_code == 204:
            return None
        if response.headers.get("Content-Type") == "application/json":
            return response.json() or {}

        if response.headers.get("Content-Type") == "application/octet-stream":
            return response.content

        if response.headers.get("Content-Type") == "text/event-stream":
            return self.process_eventstream(response)

        raise Exception(f"Unknown response type: {response.headers.get('Content-Type')}")

    def get_crawl_requests_list(self, page: int | None = None, page_size: int | None = None):
        query_params = {"page": page or 1, "page_size": page_size or 10}
        return self.process_response(
            self._get(
                "/api/v1/core/crawl-requests/",
                query_params=query_params,
            )
        )

    def get_crawl_request(self, item_id: str):
        return self.process_response(
            self._get(
                f"/api/v1/core/crawl-requests/{item_id}/",
            )
        )

    def create_crawl_request(
        self,
        url: list | str | None = None,
        spider_options: SpiderOptions | None = None,
        page_options: PageOptions | None = None,
        plugin_options: dict[str, Any] | None = None,
    ):
        data = {
            # 'urls': url if isinstance(url, list) else [url],
            "url": url,
            "options": {
                "spider_options": spider_options or {},
                "page_options": page_options or {},
                "plugin_options": plugin_options or {},
            },
        }
        return self.process_response(
            self._post(
                "/api/v1/core/crawl-requests/",
                data=data,
            )
        )

    def stop_crawl_request(self, item_id: str):
        return self.process_response(
            self._delete(
                f"/api/v1/core/crawl-requests/{item_id}/",
            )
        )

    def download_crawl_request(self, item_id: str):
        return self.process_response(
            self._get(
                f"/api/v1/core/crawl-requests/{item_id}/download/",
            )
        )

    def monitor_crawl_request(self, item_id: str, prefetched=False) -> Generator:
        query_params = {"prefetched": str(prefetched).lower()}
        generator = self.process_response(
            self._get(f"/api/v1/core/crawl-requests/{item_id}/status/", stream=True, query_params=query_params),
        )
        if not isinstance(generator, Generator):
            raise ValueError("Generator expected")
        yield from generator

    def get_crawl_request_results(
        self, item_id: str, page: int = 1, page_size: int = 25, query_params: dict | None = None
    ):
        query_params = query_params or {}
        query_params.update({"page": page or 1, "page_size": page_size or 25})
        return self.process_response(
            self._get(f"/api/v1/core/crawl-requests/{item_id}/results/", query_params=query_params)
        )

    def scrape_url(
        self,
        url: str,
        page_options: PageOptions | None = None,
        plugin_options: dict[str, Any] | None = None,
        sync: bool = True,
        prefetched: bool = True,
    ):
        response_result = self.create_crawl_request(url=url, page_options=page_options, plugin_options=plugin_options)
        if not sync:
            return response_result

        for event_data in self.monitor_crawl_request(response_result["uuid"], prefetched):
            if event_data["type"] == "result":
                return event_data["data"]

    def download_result(self, result_object: dict):
        response = httpx.get(result_object["result"], timeout=None)
        try:
            response.raise_for_status()
            result_object["result"] = response.json()
        finally:
            response.close()
        return result_object

```

### api/core/rag/extractor/watercrawl/extractor.py
```py
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from services.website_service import WebsiteService


class WaterCrawlWebExtractor(BaseExtractor):
    """
    Crawl and scrape websites and return content in clean llm-ready markdown.


    Args:
        url: The URL to scrape.
        api_key: The API key for WaterCrawl.
        base_url: The base URL for the Firecrawl API. Defaults to 'https://app.firecrawl.dev'.
        mode: The mode of operation. Defaults to 'scrape'. Options are 'crawl', 'scrape' and 'crawl_return_urls'.
        only_main_content: Only return the main content of the page excluding headers, navs, footers, etc.
    """

    def __init__(
        self,
        url: str,
        job_id: str,
        tenant_id: str,
        mode: str = "crawl",
        only_main_content: bool = True,
    ):
        """Initialize with url, api_key, base_url and mode."""
        self._url = url
        self.job_id = job_id
        self.tenant_id = tenant_id
        self.mode = mode
        self.only_main_content = only_main_content

    def extract(self) -> list[Document]:
        """Extract content from the URL."""
        documents = []
        if self.mode == "crawl":
            crawl_data = WebsiteService.get_crawl_url_data(self.job_id, "watercrawl", self._url, self.tenant_id)
            if crawl_data is None:
                return []
            document = Document(
                page_content=crawl_data.get("markdown", ""),
                metadata={
                    "source_url": crawl_data.get("source_url"),
                    "description": crawl_data.get("description"),
                    "title": crawl_data.get("title"),
                },
            )
            documents.append(document)
        elif self.mode == "scrape":
            scrape_data = WebsiteService.get_scrape_url_data(
                "watercrawl", self._url, self.tenant_id, self.only_main_content
            )

            document = Document(
                page_content=scrape_data.get("markdown", ""),
                metadata={
                    "source_url": scrape_data.get("source_url"),
                    "description": scrape_data.get("description"),
                    "title": scrape_data.get("title"),
                },
            )
            documents.append(document)
        return documents

```

### api/core/rag/extractor/watercrawl/exceptions.py
```py
import json


class WaterCrawlError(Exception):
    pass


class WaterCrawlBadRequestError(WaterCrawlError):
    def __init__(self, response):
        self.status_code = response.status_code
        self.response = response
        data = response.json()
        self.message = data.get("message", "Unknown error occurred")
        self.errors = data.get("errors", {})
        super().__init__(self.message)

    @property
    def flat_errors(self):
        return json.dumps(self.errors)

    def __str__(self):
        return f"WaterCrawlBadRequestError: {self.message} \n {self.flat_errors}"


class WaterCrawlPermissionError(WaterCrawlBadRequestError):
    def __str__(self):
        return f"You are exceeding your WaterCrawl API limits. {self.message}"


class WaterCrawlAuthenticationError(WaterCrawlBadRequestError):
    def __str__(self):
        return "WaterCrawl API key is invalid or expired. Please check your API key and try again."

```

### api/core/rag/extractor/csv_extractor.py
```py
"""Abstract interface for document loader implementations."""

import csv

import pandas as pd

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.extractor.helpers import detect_file_encodings
from core.rag.models.document import Document


class CSVExtractor(BaseExtractor):
    """Load CSV files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(
        self,
        file_path: str,
        encoding: str | None = None,
        autodetect_encoding: bool = False,
        source_column: str | None = None,
        csv_args: dict | None = None,
    ):
        """Initialize with file path."""
        self._file_path = file_path
        self._encoding = encoding
        self._autodetect_encoding = autodetect_encoding
        self.source_column = source_column
        self.csv_args = csv_args or {}

    def extract(self) -> list[Document]:
        """Load data into document objects."""
        docs = []
        try:
            with open(self._file_path, newline="", encoding=self._encoding) as csvfile:
                docs = self._read_from_file(csvfile)
        except UnicodeDecodeError as e:
            if self._autodetect_encoding:
                detected_encodings = detect_file_encodings(self._file_path)
                for encoding in detected_encodings:
                    try:
                        with open(self._file_path, newline="", encoding=encoding.encoding) as csvfile:
                            docs = self._read_from_file(csvfile)
                        break
                    except UnicodeDecodeError:
                        continue
            else:
                raise RuntimeError(f"Error loading {self._file_path}") from e

        return docs

    def _read_from_file(self, csvfile) -> list[Document]:
        docs = []
        try:
            # load csv file into pandas dataframe
            df = pd.read_csv(csvfile, on_bad_lines="skip", **self.csv_args)

            # check source column exists
            if self.source_column and self.source_column not in df.columns:
                raise ValueError(f"Source column '{self.source_column}' not found in CSV file.")

            # create document objects

            for i, row in df.iterrows():
                content = ";".join(f"{col.strip()}: {str(row[col]).strip()}" for col in df.columns)
                source = row[self.source_column] if self.source_column else ""
                metadata = {"source": source, "row": i}
                doc = Document(page_content=content, metadata=metadata)
                docs.append(doc)
        except csv.Error as e:
            raise e

        return docs

```

### api/core/rag/extractor/extractor_base.py
```py
"""Abstract interface for document loader implementations."""

from abc import ABC, abstractmethod


class BaseExtractor(ABC):
    """Interface for extract files."""

    @abstractmethod
    def extract(self):
        raise NotImplementedError

```

### api/core/rag/extractor/entity/datasource_type.py
```py
from enum import StrEnum


class DatasourceType(StrEnum):
    FILE = "upload_file"
    NOTION = "notion_import"
    WEBSITE = "website_crawl"

```

### api/core/rag/extractor/entity/extract_setting.py
```py
from pydantic import BaseModel, ConfigDict

from models.dataset import Document
from models.model import UploadFile


class NotionInfo(BaseModel):
    """
    Notion import info.
    """

    credential_id: str | None = None
    notion_workspace_id: str | None = ""
    notion_obj_id: str
    notion_page_type: str
    document: Document | None = None
    tenant_id: str
    model_config = ConfigDict(arbitrary_types_allowed=True)


class WebsiteInfo(BaseModel):
    """
    website import info.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    provider: str
    job_id: str
    url: str
    mode: str
    tenant_id: str
    only_main_content: bool = False


class ExtractSetting(BaseModel):
    """
    Model class for provider response.
    """

    datasource_type: str
    upload_file: UploadFile | None = None
    notion_info: NotionInfo | None = None
    website_info: WebsiteInfo | None = None
    document_model: str | None = None
    model_config = ConfigDict(arbitrary_types_allowed=True)

```

### api/core/rag/extractor/firecrawl/firecrawl_app.py
```py
import json
import time
from typing import Any, NotRequired, TypedDict, cast

import httpx

from extensions.ext_storage import storage


class FirecrawlDocumentData(TypedDict):
    title: str | None
    description: str | None
    source_url: str | None
    markdown: str | None


class CrawlStatusResponse(TypedDict):
    status: str
    total: int | None
    current: int | None
    data: list[FirecrawlDocumentData]


class MapResponse(TypedDict):
    success: bool
    links: list[str]


class SearchResponse(TypedDict):
    success: bool
    data: list[dict[str, Any]]
    warning: NotRequired[str]


class FirecrawlApp:
    def __init__(self, api_key=None, base_url=None):
        self.api_key = api_key
        self.base_url = base_url or "https://api.firecrawl.dev"
        if self.api_key is None and self.base_url == "https://api.firecrawl.dev":
            raise ValueError("No API key provided")

    def scrape_url(self, url, params=None) -> FirecrawlDocumentData:
        # Documentation: https://docs.firecrawl.dev/api-reference/endpoint/scrape
        headers = self._prepare_headers()
        json_data = {
            "url": url,
            "formats": ["markdown"],
            "onlyMainContent": True,
            "timeout": 30000,
        }
        if params:
            json_data.update(params)
        response = self._post_request(self._build_url("v2/scrape"), json_data, headers)
        if response.status_code == 200:
            response_data = response.json()
            data = response_data["data"]
            return self._extract_common_fields(data)
        elif response.status_code in {402, 409, 500, 429, 408}:
            self._handle_error(response, "scrape URL")
        raise Exception(f"Failed to scrape URL. Status code: {response.status_code}")

    def crawl_url(self, url, params=None) -> str:
        # Documentation: https://docs.firecrawl.dev/api-reference/endpoint/crawl-post
        headers = self._prepare_headers()
        json_data = {"url": url}
        if params:
            json_data.update(params)
        response = self._post_request(self._build_url("v2/crawl"), json_data, headers)
        if response.status_code == 200:
            # There's also another two fields in the response: "success" (bool) and "url" (str)
            job_id = response.json().get("id")
            return cast(str, job_id)
        else:
            self._handle_error(response, "start crawl job")
            return ""  # unreachable

    def map(self, url: str, params: dict[str, Any] | None = None) -> MapResponse:
        # Documentation: https://docs.firecrawl.dev/api-reference/endpoint/map
        headers = self._prepare_headers()
        json_data: dict[str, Any] = {"url": url, "integration": "dify"}
        if params:
            # Pass through provided params, including optional "sitemap": "only" | "include" | "skip"
            json_data.update(params)
        response = self._post_request(self._build_url("v2/map"), json_data, headers)
        if response.status_code == 200:
            return cast(MapResponse, response.json())
        elif response.status_code in {402, 409, 500, 429, 408}:
            self._handle_error(response, "start map job")
        raise Exception(f"Failed to start map job. Status code: {response.status_code}")

    def check_crawl_status(self, job_id) -> CrawlStatusResponse:
        headers = self._prepare_headers()
        response = self._get_request(self._build_url(f"v2/crawl/{job_id}"), headers)
        if response.status_code == 200:
            crawl_status_response = response.json()
            if crawl_status_response.get("status") == "completed":
                # Normalize to avoid None bypassing the zero-guard when the API returns null.
                total = crawl_status_response.get("total") or 0
                if total <= 0:
                    raise Exception("Failed to check crawl status. Error: No page found")
                url_data_list = self._collect_all_crawl_pages(crawl_status_response, headers)
                if url_data_list:
                    file_key = "website_files/" + job_id + ".txt"
                    try:
                        if storage.exists(file_key):
                            storage.delete(file_key)
                        storage.save(file_key, json.dumps(url_data_list).encode("utf-8"))
                    except Exception as e:
                        raise Exception(f"Error saving crawl data: {e}")
                return self._format_crawl_status_response("completed", crawl_status_response, url_data_list)
            else:
                return self._format_crawl_status_response(
                    crawl_status_response.get("status"), crawl_status_response, []
                )
        self._handle_error(response, "check crawl status")
        raise RuntimeError("unreachable: _handle_error always raises")

    def _collect_all_crawl_pages(
        self, first_page: dict[str, Any], headers: dict[str, str]
    ) -> list[FirecrawlDocumentData]:
        """Collect all crawl result pages by following pagination links.

        Raises an exception if any paginated request fails, to avoid returning
        partial data that is inconsistent with the reported total.

        The number of pages processed is capped at ``total`` (the
        server-reported page count) to guard against infinite loops caused by
        a misbehaving server that keeps returning a ``next`` URL.
        """
        total: int = first_page.get("total") or 0
        url_data_list: list[FirecrawlDocumentData] = []
        current_page = first_page
        pages_processed = 0
        while True:
            for item in current_page.get("data", []):
                if isinstance(item, dict) and "metadata" in item and "markdown" in item:
                    url_data_list.append(self._extract_common_fields(item))
            next_url: str | None = current_page.get("next")
            pages_processed += 1
            if not next_url or pages_processed >= total:
                break
            response = self._get_request(next_url, headers)
            if response.status_code != 200:
                self._handle_error(response, "fetch next crawl page")
            current_page = response.json()
        return url_data_list

    def _format_crawl_status_response(
        self,
        status: str,
        crawl_status_response: dict[str, Any],
        url_data_list: list[FirecrawlDocumentData],
    ) -> CrawlStatusResponse:
        return {
            "status": status,
            "total": crawl_status_response.get("total"),
            "current": crawl_status_response.get("completed"),
            "data": url_data_list,
        }

    def _extract_common_fields(self, item: dict[str, Any]) -> FirecrawlDocumentData:
        return {
            "title": item.get("metadata", {}).get("title"),
            "description": item.get("metadata", {}).get("description"),
            "source_url": item.get("metadata", {}).get("sourceURL"),
            "markdown": item.get("markdown"),
        }

    def _prepare_headers(self) -> dict[str, str]:
        return {"Content-Type": "application/json", "Authorization": f"Bearer {self.api_key}"}

    def _build_url(self, path: str) -> str:
        # ensure exactly one slash between base and path, regardless of user-provided base_url
        return f"{self.base_url.rstrip('/')}/{path.lstrip('/')}"

    def _post_request(self, url, data, headers, retries=3, backoff_factor=0.5) -> httpx.Response:
        for attempt in range(retries):
            response = httpx.post(url, headers=headers, json=data)
            if response.status_code == 502:
                time.sleep(backoff_factor * (2**attempt))
            else:
                return response
        return response

    def _get_request(self, url, headers, retries=3, backoff_factor=0.5) -> httpx.Response:
        for attempt in range(retries):
            response = httpx.get(url, headers=headers)
            if response.status_code == 502:
                time.sleep(backoff_factor * (2**attempt))
            else:
                return response
        return response

    def _handle_error(self, response, action):
        try:
            payload = response.json()
            error_message = payload.get("error") or payload.get("message") or response.text or "Unknown error occurred"
        except json.JSONDecodeError:
            error_message = response.text or "Unknown error occurred"
        raise Exception(f"Failed to {action}. Status code: {response.status_code}. Error: {error_message}")  # type: ignore[return]

    def search(self, query: str, params: dict[str, Any] | None = None) -> SearchResponse:
        # Documentation: https://docs.firecrawl.dev/api-reference/endpoint/search
        headers = self._prepare_headers()
        json_data: dict[str, Any] = {
            "query": query,
            "limit": 5,
            "lang": "en",
            "country": "us",
            "timeout": 60000,
            "ignoreInvalidURLs": True,
            "scrapeOptions": {},
            "sources": [
                {"type": "web"},
            ],
            "integration": "dify",
        }
        if params:
            json_data.update(params)
        response = self._post_request(self._build_url("v2/search"), json_data, headers)
        if response.status_code == 200:
            response_data: SearchResponse = response.json()
            if not response_data.get("success"):
                raise Exception(f"Search failed. Error: {response_data.get('warning', 'Unknown error')}")
            return response_data
        elif response.status_code in {402, 409, 500, 429, 408}:
            self._handle_error(response, "perform search")
        raise Exception(f"Failed to perform search. Status code: {response.status_code}")

```

### api/core/rag/extractor/firecrawl/firecrawl_web_extractor.py
```py
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from services.website_service import WebsiteService


class FirecrawlWebExtractor(BaseExtractor):
    """
    Crawl and scrape websites and return content in clean llm-ready markdown.

    Args:
        url: The URL to scrape.
        job_id: The crawl job id.
        tenant_id: The tenant id.
        mode: The mode of operation. Defaults to 'scrape'. Options are 'crawl', 'scrape' and 'crawl_return_urls'.
        only_main_content: Only return the main content of the page excluding headers, navs, footers, etc.
    """

    def __init__(
        self,
        url: str,
        job_id: str,
        tenant_id: str,
        mode: str = "crawl",
        only_main_content: bool = True,
    ):
        """Initialize with url, api_key, base_url and mode."""
        self._url = url
        self.job_id = job_id
        self.tenant_id = tenant_id
        self.mode = mode
        self.only_main_content = only_main_content

    def extract(self) -> list[Document]:
        """Extract content from the URL."""
        documents = []
        if self.mode == "crawl":
            crawl_data = WebsiteService.get_crawl_url_data(self.job_id, "firecrawl", self._url, self.tenant_id)
            if crawl_data is None:
                return []
            document = Document(
                page_content=crawl_data.get("markdown", ""),
                metadata={
                    "source_url": crawl_data.get("source_url"),
                    "description": crawl_data.get("description"),
                    "title": crawl_data.get("title"),
                },
            )
            documents.append(document)
        elif self.mode == "scrape":
            scrape_data = WebsiteService.get_scrape_url_data(
                "firecrawl", self._url, self.tenant_id, self.only_main_content
            )

            document = Document(
                page_content=scrape_data.get("markdown", ""),
                metadata={
                    "source_url": scrape_data.get("source_url"),
                    "description": scrape_data.get("description"),
                    "title": scrape_data.get("title"),
                },
            )
            documents.append(document)
        return documents

```

### api/core/rag/extractor/text_extractor.py
```py
"""Abstract interface for document loader implementations."""

from pathlib import Path

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.extractor.helpers import detect_file_encodings
from core.rag.models.document import Document


class TextExtractor(BaseExtractor):
    """Load text files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, encoding: str | None = None, autodetect_encoding: bool = False):
        """Initialize with file path."""
        self._file_path = file_path
        self._encoding = encoding
        self._autodetect_encoding = autodetect_encoding

    def extract(self) -> list[Document]:
        """Load from file path."""
        text = ""
        try:
            text = Path(self._file_path).read_text(encoding=self._encoding)
        except UnicodeDecodeError as e:
            if self._autodetect_encoding:
                detected_encodings = detect_file_encodings(self._file_path)
                for encoding in detected_encodings:
                    try:
                        text = Path(self._file_path).read_text(encoding=encoding.encoding)
                        break
                    except UnicodeDecodeError:
                        continue
                else:
                    raise RuntimeError(
                        f"Decode failed: {self._file_path}, all detected encodings failed. Original error: {e}"
                    )
            else:
                raise RuntimeError(f"Decode failed: {self._file_path}, specified encoding failed. Original error: {e}")
        except Exception as e:
            raise RuntimeError(f"Error loading {self._file_path}") from e

        metadata = {"source": self._file_path}
        return [Document(page_content=text, metadata=metadata)]

```

### api/core/rag/extractor/word_extractor.py
```py
"""Word (.docx) document extractor used for RAG ingestion.

Supports local file paths and remote URLs (downloaded via `core.helper.ssrf_proxy`).
"""

import logging
import mimetypes
import os
import re
import tempfile
import uuid
from urllib.parse import urlparse

from docx import Document as DocxDocument
from docx.oxml.ns import qn
from docx.text.run import Run

from configs import dify_config
from core.helper import ssrf_proxy
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_storage import storage
from extensions.storage.storage_type import StorageType
from libs.datetime_utils import naive_utc_now
from models.enums import CreatorUserRole
from models.model import UploadFile

logger = logging.getLogger(__name__)


class WordExtractor(BaseExtractor):
    """Load docx files.

    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, tenant_id: str, user_id: str):
        """Initialize with file path."""
        self.file_path = file_path
        self.tenant_id = tenant_id
        self.user_id = user_id

        if "~" in self.file_path:
            self.file_path = os.path.expanduser(self.file_path)

        # If the file is a web path, download it to a temporary file, and use that
        if not os.path.isfile(self.file_path) and self._is_valid_url(self.file_path):
            response = ssrf_proxy.get(self.file_path)

            if response.status_code != 200:
                response.close()
                raise ValueError(f"Check the url of your file; returned status code {response.status_code}")

            self.web_path = self.file_path
            # TODO: use a better way to handle the file
            self.temp_file = tempfile.NamedTemporaryFile()  # noqa SIM115
            try:
                self.temp_file.write(response.content)
                self.temp_file.flush()
            finally:
                response.close()
            self.file_path = self.temp_file.name
        elif not os.path.isfile(self.file_path):
            raise ValueError(f"File path {self.file_path} is not a valid file or url")

    def __del__(self):
        if hasattr(self, "temp_file"):
            self.temp_file.close()

    def extract(self) -> list[Document]:
        """Load given path as single page."""
        content = self.parse_docx(self.file_path)
        return [
            Document(
                page_content=content,
                metadata={"source": self.file_path},
            )
        ]

    @staticmethod
    def _is_valid_url(url: str) -> bool:
        """Check if the url is valid."""
        parsed = urlparse(url)
        return bool(parsed.netloc) and bool(parsed.scheme)

    def _extract_images_from_docx(self, doc):
        image_count = 0
        image_map = {}
        base_url = dify_config.INTERNAL_FILES_URL or dify_config.FILES_URL

        for r_id, rel in doc.part.rels.items():
            if "image" in rel.target_ref:
                image_count += 1
                if rel.is_external:
                    url = rel.target_ref
                    if not self._is_valid_url(url):
                        continue
                    try:
                        response = ssrf_proxy.get(url)
                    except Exception as e:
                        logger.warning("Failed to download image from URL: %s: %s", url, str(e))
                        continue
                    if response.status_code == 200:
                        image_ext = mimetypes.guess_extension(response.headers.get("Content-Type", ""))
                        if image_ext is None:
                            continue
                        file_uuid = str(uuid.uuid4())
                        file_key = "image_files/" + self.tenant_id + "/" + file_uuid + image_ext
                        mime_type, _ = mimetypes.guess_type(file_key)
                        storage.save(file_key, response.content)
                        # save file to db
                        upload_file = UploadFile(
                            tenant_id=self.tenant_id,
                            storage_type=StorageType(dify_config.STORAGE_TYPE),
                            key=file_key,
                            name=file_key,
                            size=0,
                            extension=str(image_ext),
                            mime_type=mime_type or "",
                            created_by=self.user_id,
                            created_by_role=CreatorUserRole.ACCOUNT,
                            created_at=naive_utc_now(),
                            used=True,
                            used_by=self.user_id,
                            used_at=naive_utc_now(),
                        )
                        db.session.add(upload_file)
                        image_map[r_id] = f"![image]({base_url}/files/{upload_file.id}/file-preview)"
                else:
                    image_ext = rel.target_ref.split(".")[-1]
                    if image_ext is None:
                        continue
                    # user uuid as file name
                    file_uuid = str(uuid.uuid4())
                    file_key = "image_files/" + self.tenant_id + "/" + file_uuid + "." + image_ext
                    mime_type, _ = mimetypes.guess_type(file_key)

                    storage.save(file_key, rel.target_part.blob)
                    # save file to db
                    upload_file = UploadFile(
                        tenant_id=self.tenant_id,
                        storage_type=StorageType(dify_config.STORAGE_TYPE),
                        key=file_key,
                        name=file_key,
                        size=0,
                        extension=str(image_ext),
                        mime_type=mime_type or "",
                        created_by=self.user_id,
                        created_by_role=CreatorUserRole.ACCOUNT,
                        created_at=naive_utc_now(),
                        used=True,
                        used_by=self.user_id,
                        used_at=naive_utc_now(),
                    )
                    db.session.add(upload_file)
                    image_map[rel.target_part] = f"![image]({base_url}/files/{upload_file.id}/file-preview)"
        db.session.commit()
        return image_map

    def _table_to_markdown(self, table, image_map):
        markdown = []
        # calculate the total number of columns
        total_cols = max(len(row.cells) for row in table.rows)

        header_row = table.rows[0]
        headers = self._parse_row(header_row, image_map, total_cols)
        markdown.append("| " + " | ".join(headers) + " |")
        markdown.append("| " + " | ".join(["---"] * total_cols) + " |")

        for row in table.rows[1:]:
            row_cells = self._parse_row(row, image_map, total_cols)
            markdown.append("| " + " | ".join(row_cells) + " |")
        return "\n".join(markdown)

    def _parse_row(self, row, image_map, total_cols):
        # Initialize a row, all of which are empty by default
        row_cells = [""] * total_cols
        col_index = 0
        while col_index < len(row.cells):
            # make sure the col_index is not out of range
            while col_index < len(row.cells) and row_cells[col_index] != "":
                col_index += 1
            # if col_index is out of range the loop is jumped
            if col_index >= len(row.cells):
                break
            # get the correct cell
            cell = row.cells[col_index]
            cell_content = self._parse_cell(cell, image_map).strip()
            cell_colspan = cell.grid_span or 1
            for i in range(cell_colspan):
                if col_index + i < total_cols:
                    row_cells[col_index + i] = cell_content if i == 0 else ""
            col_index += cell_colspan
        return row_cells

    def _parse_cell(self, cell, image_map):
        cell_content = []
        for paragraph in cell.paragraphs:
            parsed_paragraph = self._parse_cell_paragraph(paragraph, image_map)
            if parsed_paragraph:
                cell_content.append(parsed_paragraph)
        unique_content = list(dict.fromkeys(cell_content))
        return " ".join(unique_content)

    def _parse_cell_paragraph(self, paragraph, image_map):
        paragraph_content: list[str] = []

        for child in paragraph._element:
            tag = child.tag
            if tag == qn("w:hyperlink"):
                # Note: w:hyperlink elements may also use w:anchor for internal bookmarks.
                # This extractor intentionally only converts external links (HTTP/mailto, etc.)
                # that are backed by a relationship id (r:id) with rel.is_external == True.
                # Hyperlinks without such an external rel (including anchor-only bookmarks)
                # are left as plain text link_text.
                r_id = child.get(qn("r:id"))
                link_text_parts: list[str] = []
                for run_elem in child.findall(qn("w:r")):
                    run = Run(run_elem, paragraph)
                    if run.text:
                        link_text_parts.append(run.text)
                link_text = "".join(link_text_parts).strip()
                if r_id:
                    try:
                        rel = paragraph.part.rels.get(r_id)
                        if rel:
                            target_ref = getattr(rel, "target_ref", None)
                            if target_ref:
                                parsed_target = urlparse(str(target_ref))
                                if rel.is_external or parsed_target.scheme in ("http", "https", "mailto"):
                                    display_text = link_text or str(target_ref)
                                    link_text = f"[{display_text}]({target_ref})"
                    except Exception:
                        logger.exception("Failed to resolve URL for hyperlink with r:id: %s", r_id)
                if link_text:
                    paragraph_content.append(link_text)

            elif tag == qn("w:r"):
                run = Run(child, paragraph)
                if run.element.xpath(".//a:blip"):
                    for blip in run.element.xpath(".//a:blip"):
                        image_id = blip.get(
                            "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}embed"
                        )
                        if not image_id:
                            continue
                        rel = paragraph.part.rels.get(image_id)
                        if rel is None:
                            continue
                        if rel.is_external:
                            if image_id in image_map:
                                paragraph_content.append(image_map[image_id])
                        else:
                            image_part = rel.target_part
                            if image_part in image_map:
                                paragraph_content.append(image_map[image_part])
                else:
                    if run.text:
                        paragraph_content.append(run.text)

        return "".join(paragraph_content).strip()

    def parse_docx(self, docx_path):
        doc = DocxDocument(docx_path)

        content = []

        image_map = self._extract_images_from_docx(doc)

        def parse_paragraph(paragraph):
            def append_image_link(image_id, has_drawing, target_buffer):
                """Helper to append image link from image_map based on relationship type."""
                rel = doc.part.rels[image_id]
                if rel.is_external:
                    if image_id in image_map and not has_drawing:
                        target_buffer.append(image_map[image_id])
                else:
                    image_part = rel.target_part
                    if image_part in image_map and not has_drawing:
                        target_buffer.append(image_map[image_part])

            def process_run(run, target_buffer):
                # Helper to extract text and embedded images from a run element and append them to target_buffer
                if hasattr(run.element, "tag") and isinstance(run.element.tag, str) and run.element.tag.endswith("r"):
                    # Process drawing type images
                    drawing_elements = run.element.findall(
                        ".//{http://schemas.openxmlformats.org/wordprocessingml/2006/main}drawing"
                    )
                    has_drawing = False
                    for drawing in drawing_elements:
                        blip_elements = drawing.findall(
                            ".//{http://schemas.openxmlformats.org/drawingml/2006/main}blip"
                        )
                        for blip in blip_elements:
                            embed_id = blip.get(
                                "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}embed"
                            )
                            if embed_id:
                                rel = doc.part.rels.get(embed_id)
                                if rel is not None and rel.is_external:
                                    # External image: use embed_id as key
                                    if embed_id in image_map:
                                        has_drawing = True
                                        target_buffer.append(image_map[embed_id])
                                else:
                                    # Internal image: use target_part as key
                                    image_part = doc.part.related_parts.get(embed_id)
                                    if image_part in image_map:
                                        has_drawing = True
                                        target_buffer.append(image_map[image_part])
                    # Process pict type images
                    shape_elements = run.element.findall(
                        ".//{http://schemas.openxmlformats.org/wordprocessingml/2006/main}pict"
                    )
                    for shape in shape_elements:
                        # Find image data in VML
                        shape_image = shape.find(
                            ".//{http://schemas.openxmlformats.org/wordprocessingml/2006/main}binData"
                        )
                        if shape_image is not None and shape_image.text:
                            image_id = shape_image.get(
                                "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}id"
                            )
                            if image_id and image_id in doc.part.rels:
                                append_image_link(image_id, has_drawing, target_buffer)
                        # Find imagedata element in VML
                        image_data = shape.find(".//{urn:schemas-microsoft-com:vml}imagedata")
                        if image_data is not None:
                            image_id = image_data.get("id") or image_data.get(
                                "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}id"
                            )
                            if image_id and image_id in doc.part.rels:
                                append_image_link(image_id, has_drawing, target_buffer)
                if run.text.strip():
                    target_buffer.append(run.text.strip())

            def process_hyperlink(hyperlink_elem, target_buffer):
                # Helper to extract text from a hyperlink element and append it to target_buffer
                r_id = hyperlink_elem.get(qn("r:id"))

                # Extract text from runs inside the hyperlink
                link_text_parts = []
                for run_elem in hyperlink_elem.findall(qn("w:r")):
                    run = Run(run_elem, paragraph)
                    # Hyperlink text may be split across multiple runs (e.g., with different formatting),
                    # so collect all run texts first
                    if run.text:
                        link_text_parts.append(run.text)

                link_text = "".join(link_text_parts).strip()

                # Resolve URL
                if r_id:
                    try:
                        rel = doc.part.rels.get(r_id)
                        if rel and rel.is_external:
                            link_text = f"[{link_text or rel.target_ref}]({rel.target_ref})"
                    except Exception:
                        logger.exception("Failed to resolve URL for hyperlink with r:id: %s", r_id)

                if link_text:
                    target_buffer.append(link_text)

            paragraph_content = []
            # State for legacy HYPERLINK fields
            hyperlink_field_url = None
            hyperlink_field_text_parts: list[str] = []
            is_collecting_field_text = False
            # Iterate through paragraph elements in document order
            for child in paragraph._element:
                tag = child.tag
                if tag == qn("w:r"):
                    # Regular run
                    run = Run(child, paragraph)

                    # Check for fldChar (begin/end/separate) and instrText for legacy hyperlinks
                    fld_chars = child.findall(qn("w:fldChar"))
                    instr_texts = child.findall(qn("w:instrText"))

                    # Handle Fields
                    if fld_chars or instr_texts:
                        # Process instrText to find HYPERLINK "url"
                        for instr in instr_texts:
                            if instr.text and "HYPERLINK" in instr.text:
                                # Quick regex to extract URL
                                match = re.search(r'HYPERLINK\s+"([^"]+)"', instr.text, re.IGNORECASE)
                                if match:
                                    hyperlink_field_url = match.group(1)

                        # Process fldChar
                        for fld_char in fld_chars:
                            fld_char_type = fld_char.get(qn("w:fldCharType"))
                            if fld_char_type == "begin":
                                # Start of a field: reset legacy link state
                                hyperlink_field_url = None
                                hyperlink_field_text_parts = []
                                is_collecting_field_text = False
                            elif fld_char_type == "separate":
                                # Separator: if we found a URL, start collecting visible text
                                if hyperlink_field_url:
                                    is_collecting_field_text = True
                            elif fld_char_type == "end":
                                # End of field
                                if is_collecting_field_text and hyperlink_field_url:
                                    # Create markdown link and append to main content
                                    display_text = "".join(hyperlink_field_text_parts).strip()
                                    if display_text:
                                        link_md = f"[{display_text}]({hyperlink_field_url})"
                                        paragraph_content.append(link_md)
                                # Reset state
                                hyperlink_field_url = None
                                hyperlink_field_text_parts = []
                                is_collecting_field_text = False

                    # Decide where to append content
                    target_buffer = hyperlink_field_text_parts if is_collecting_field_text else paragraph_content
                    process_run(run, target_buffer)
                elif tag == qn("w:hyperlink"):
                    process_hyperlink(child, paragraph_content)
            return "".join(paragraph_content) if paragraph_content else ""

        paragraphs = doc.paragraphs.copy()
        tables = doc.tables.copy()
        for element in doc.element.body:
            if hasattr(element, "tag"):
                if isinstance(element.tag, str) and element.tag.endswith("p"):  # paragraph
                    para = paragraphs.pop(0)
                    parsed_paragraph = parse_paragraph(para)
                    if parsed_paragraph.strip():
                        content.append(parsed_paragraph)
                    else:
                        content.append("\n")
                elif isinstance(element.tag, str) and element.tag.endswith("tbl"):  # table
                    table = tables.pop(0)
                    content.append(self._table_to_markdown(table, image_map))
        return "\n".join(content)

```

### api/core/rag/extractor/markdown_extractor.py
```py
"""Abstract interface for document loader implementations."""

import re
from pathlib import Path

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.extractor.helpers import detect_file_encodings
from core.rag.models.document import Document


class MarkdownExtractor(BaseExtractor):
    """Load Markdown files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(
        self,
        file_path: str,
        remove_hyperlinks: bool = False,
        remove_images: bool = False,
        encoding: str | None = None,
        autodetect_encoding: bool = True,
    ):
        """Initialize with file path."""
        self._file_path = file_path
        self._remove_hyperlinks = remove_hyperlinks
        self._remove_images = remove_images
        self._encoding = encoding
        self._autodetect_encoding = autodetect_encoding

    def extract(self) -> list[Document]:
        """Load from file path."""
        tups = self.parse_tups(self._file_path)
        documents = []
        for header, value in tups:
            value = value.strip()
            if header is None:
                documents.append(Document(page_content=value))
            else:
                documents.append(Document(page_content=f"\n\n{header}\n{value}"))

        return documents

    def markdown_to_tups(self, markdown_text: str) -> list[tuple[str | None, str]]:
        """Convert a markdown file to a dictionary.

        The keys are the headers and the values are the text under each header.

        """
        markdown_tups: list[tuple[str | None, str]] = []
        lines = markdown_text.split("\n")

        current_header = None
        current_text = ""
        code_block_flag = False

        for line in lines:
            if line.startswith("```"):
                code_block_flag = not code_block_flag
                current_text += line + "\n"
                continue
            if code_block_flag:
                current_text += line + "\n"
                continue
            header_match = re.match(r"^#+\s", line)
            if header_match:
                markdown_tups.append((current_header, current_text))
                current_header = line
                current_text = ""
            else:
                current_text += line + "\n"
        markdown_tups.append((current_header, current_text))

        markdown_tups = [
            (re.sub(r"#", "", key).strip() if key else None, re.sub(r"<.*?>", "", value))
            for key, value in markdown_tups
        ]

        return markdown_tups

    def remove_images(self, content: str) -> str:
        """Get a dictionary of a markdown file from its path."""
        pattern = r"!{1}\[\[(.*)\]\]"
        content = re.sub(pattern, "", content)
        return content

    def remove_hyperlinks(self, content: str) -> str:
        """Get a dictionary of a markdown file from its path."""
        pattern = r"\[(.*?)\]\((.*?)\)"
        content = re.sub(pattern, r"\1", content)
        return content

    def parse_tups(self, filepath: str) -> list[tuple[str | None, str]]:
        """Parse file into tuples."""
        content = ""
        try:
            content = Path(filepath).read_text(encoding=self._encoding)
        except UnicodeDecodeError as e:
            if self._autodetect_encoding:
                detected_encodings = detect_file_encodings(filepath)
                for encoding in detected_encodings:
                    try:
                        content = Path(filepath).read_text(encoding=encoding.encoding)
                        break
                    except UnicodeDecodeError:
                        continue
            else:
                raise RuntimeError(f"Error loading {filepath}") from e
        except Exception as e:
            raise RuntimeError(f"Error loading {filepath}") from e

        if self._remove_hyperlinks:
            content = self.remove_hyperlinks(content)

        if self._remove_images:
            content = self.remove_images(content)

        return self.markdown_to_tups(content)

```

### api/core/rag/extractor/html_extractor.py
```py
"""Abstract interface for document loader implementations."""

from bs4 import BeautifulSoup

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document


class HtmlExtractor(BaseExtractor):
    """
    Load html files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str):
        """Initialize with file path."""
        self._file_path = file_path

    def extract(self) -> list[Document]:
        return [Document(page_content=self._load_as_text())]

    def _load_as_text(self) -> str:
        text: str = ""
        with open(self._file_path, "rb") as fp:
            soup = BeautifulSoup(fp, "html.parser")
            text = soup.get_text()
            text = text.strip() if text else ""

        return text

```

### api/core/rag/extractor/jina_reader_extractor.py
```py
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from services.website_service import WebsiteService


class JinaReaderWebExtractor(BaseExtractor):
    """
    Crawl and scrape websites and return content in clean llm-ready markdown.
    """

    def __init__(
        self,
        url: str,
        job_id: str,
        tenant_id: str,
        mode: str = "crawl",
        only_main_content: bool = False,
    ):
        """Initialize with url, api_key, base_url and mode."""
        self._url = url
        self.job_id = job_id
        self.tenant_id = tenant_id
        self.mode = mode
        self.only_main_content = only_main_content

    def extract(self) -> list[Document]:
        """Extract content from the URL."""
        documents = []
        if self.mode == "crawl":
            crawl_data = WebsiteService.get_crawl_url_data(self.job_id, "jinareader", self._url, self.tenant_id)
            if crawl_data is None:
                return []
            document = Document(
                page_content=crawl_data.get("content", ""),
                metadata={
                    "source_url": crawl_data.get("url"),
                    "description": crawl_data.get("description"),
                    "title": crawl_data.get("title"),
                },
            )
            documents.append(document)
        return documents

```

### api/core/rag/extractor/notion_extractor.py
```py
import json
import logging
import operator
from typing import Any, cast

import httpx
from sqlalchemy import update

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from extensions.ext_database import db
from models.dataset import Document as DocumentModel
from services.datasource_provider_service import DatasourceProviderService

logger = logging.getLogger(__name__)

BLOCK_CHILD_URL_TMPL = "https://api.notion.com/v1/blocks/{block_id}/children"
DATABASE_URL_TMPL = "https://api.notion.com/v1/databases/{database_id}/query"
SEARCH_URL = "https://api.notion.com/v1/search"

RETRIEVE_PAGE_URL_TMPL = "https://api.notion.com/v1/pages/{page_id}"
RETRIEVE_DATABASE_URL_TMPL = "https://api.notion.com/v1/databases/{database_id}"
# if user want split by headings, use the corresponding splitter
HEADING_SPLITTER = {
    "heading_1": "# ",
    "heading_2": "## ",
    "heading_3": "### ",
}


class NotionExtractor(BaseExtractor):
    def __init__(
        self,
        notion_workspace_id: str,
        notion_obj_id: str,
        notion_page_type: str,
        tenant_id: str,
        document_model: DocumentModel | None = None,
        notion_access_token: str | None = None,
        credential_id: str | None = None,
    ):
        self._notion_access_token = None
        self._document_model = document_model
        self._notion_workspace_id = notion_workspace_id
        self._notion_obj_id = notion_obj_id
        self._notion_page_type = notion_page_type
        self._credential_id = credential_id
        if notion_access_token:
            self._notion_access_token = notion_access_token
        else:
            try:
                self._notion_access_token = self._get_access_token(tenant_id, self._credential_id)
            except Exception as e:
                logger.warning(
                    (
                        "Failed to get Notion access token from datasource credentials: %s, "
                        "falling back to environment variable NOTION_INTEGRATION_TOKEN"
                    ),
                    e,
                )
                integration_token = dify_config.NOTION_INTEGRATION_TOKEN
                if integration_token is None:
                    raise ValueError(
                        "Must specify `integration_token` or set environment variable `NOTION_INTEGRATION_TOKEN`."
                    ) from e

                self._notion_access_token = integration_token

    def extract(self) -> list[Document]:
        self.update_last_edited_time(self._document_model)

        text_docs = self._load_data_as_documents(self._notion_obj_id, self._notion_page_type)

        return text_docs

    def _load_data_as_documents(self, notion_obj_id: str, notion_page_type: str) -> list[Document]:
        docs = []
        if notion_page_type == "database":
            # get all the pages in the database
            page_text_documents = self._get_notion_database_data(notion_obj_id)
            docs.extend(page_text_documents)
        elif notion_page_type == "page":
            page_text_list = self._get_notion_block_data(notion_obj_id)
            docs.append(Document(page_content="\n".join(page_text_list)))
        else:
            raise ValueError("notion page type not supported")

        return docs

    def _get_notion_database_data(self, database_id: str, query_dict: dict[str, Any] = {}) -> list[Document]:
        """Get all the pages from a Notion database."""
        assert self._notion_access_token is not None, "Notion access token is required"

        database_content = []
        next_cursor = None
        has_more = True

        while has_more:
            current_query = query_dict.copy()
            if next_cursor:
                current_query["start_cursor"] = next_cursor

            res = httpx.post(
                DATABASE_URL_TMPL.format(database_id=database_id),
                headers={
                    "Authorization": "Bearer " + self._notion_access_token,
                    "Content-Type": "application/json",
                    "Notion-Version": "2022-06-28",
                },
                json=current_query,
            )

            response_data = res.json()

            if "results" not in response_data or response_data["results"] is None:
                break

            for result in response_data["results"]:
                properties = result["properties"]
                data = {}
                value: Any
                for property_name, property_value in properties.items():
                    type = property_value["type"]
                    if type == "multi_select":
                        value = []
                        multi_select_list = property_value[type]
                        for multi_select in multi_select_list:
                            value.append(multi_select["name"])
                    elif type in {"rich_text", "title"}:
                        if len(property_value[type]) > 0:
                            value = property_value[type][0]["plain_text"]
                        else:
                            value = ""
                    elif type in {"select", "status"}:
                        if property_value[type]:
                            value = property_value[type]["name"]
                        else:
                            value = ""
                    else:
                        value = property_value[type]
                    data[property_name] = value
                row_dict = {k: v for k, v in data.items() if v}
                row_content = ""
                for key, value in sorted(row_dict.items(), key=operator.itemgetter(0)):
                    if isinstance(value, dict):
                        value_dict = {k: v for k, v in value.items() if v}
                        value_content = "".join(f"{k}:{v} " for k, v in value_dict.items())
                        row_content = row_content + f"{key}:{value_content}\n"
                    else:
                        row_content = row_content + f"{key}:{value}\n"
                if "url" in result:
                    row_content = row_content + f"Row Page URL:{result.get('url', '')}\n"
                database_content.append(row_content)

            has_more = response_data.get("has_more", False)
            next_cursor = response_data.get("next_cursor")

        if not database_content:
            return []

        return [Document(page_content="\n".join(database_content))]

    def _get_notion_block_data(self, page_id: str) -> list[str]:
        assert self._notion_access_token is not None, "Notion access token is required"
        result_lines_arr = []
        start_cursor = None
        block_url = BLOCK_CHILD_URL_TMPL.format(block_id=page_id)
        while True:
            query_dict: dict[str, Any] = {} if not start_cursor else {"start_cursor": start_cursor}
            try:
                res = httpx.request(
                    "GET",
                    block_url,
                    headers={
                        "Authorization": "Bearer " + self._notion_access_token,
                        "Content-Type": "application/json",
                        "Notion-Version": "2022-06-28",
                    },
                    params=query_dict,
                )
                if res.status_code != 200:
                    raise ValueError(f"Error fetching Notion block data: {res.text}")
                data = res.json()
            except httpx.HTTPError as e:
                raise ValueError("Error fetching Notion block data") from e
            if "results" not in data or not isinstance(data["results"], list):
                raise ValueError("Error fetching Notion block data")
            for result in data["results"]:
                result_type = result["type"]
                result_obj = result[result_type]
                cur_result_text_arr = []
                if result_type == "table":
                    result_block_id = result["id"]
                    text = self._read_table_rows(result_block_id)
                    text += "\n\n"
                    result_lines_arr.append(text)
                else:
                    if "rich_text" in result_obj:
                        for rich_text in result_obj["rich_text"]:
                            # skip if doesn't have text object
                            if "text" in rich_text:
                                text = rich_text["text"]["content"]
                                cur_result_text_arr.append(text)

                    result_block_id = result["id"]
                    has_children = result["has_children"]
                    block_type = result["type"]
                    if has_children and block_type != "child_page":
                        children_text = self._read_block(result_block_id, num_tabs=1)
                        cur_result_text_arr.append(children_text)

                    cur_result_text = "\n".join(cur_result_text_arr)
                    if result_type in HEADING_SPLITTER:
                        result_lines_arr.append(f"{HEADING_SPLITTER[result_type]}{cur_result_text}")
                    else:
                        result_lines_arr.append(cur_result_text + "\n\n")

            if data["next_cursor"] is None:
                break
            else:
                start_cursor = data["next_cursor"]
        return result_lines_arr

    def _read_block(self, block_id: str, num_tabs: int = 0) -> str:
        """Read a block."""
        assert self._notion_access_token is not None, "Notion access token is required"
        result_lines_arr = []
        start_cursor = None
        block_url = BLOCK_CHILD_URL_TMPL.format(block_id=block_id)
        while True:
            query_dict: dict[str, Any] = {} if not start_cursor else {"start_cursor": start_cursor}

            res = httpx.request(
                "GET",
                block_url,
                headers={
                    "Authorization": "Bearer " + self._notion_access_token,
                    "Content-Type": "application/json",
                    "Notion-Version": "2022-06-28",
                },
                params=query_dict,
            )
            data = res.json()
            if "results" not in data or data["results"] is None:
                break
            for result in data["results"]:
                result_type = result["type"]
                result_obj = result[result_type]
                cur_result_text_arr = []
                if result_type == "table":
                    result_block_id = result["id"]
                    text = self._read_table_rows(result_block_id)
                    result_lines_arr.append(text)
                else:
                    if "rich_text" in result_obj:
                        for rich_text in result_obj["rich_text"]:
                            # skip if doesn't have text object
                            if "text" in rich_text:
                                text = rich_text["text"]["content"]
                                prefix = "\t" * num_tabs
                                cur_result_text_arr.append(prefix + text)
                    result_block_id = result["id"]
                    has_children = result["has_children"]
                    block_type = result["type"]
                    if has_children and block_type != "child_page":
                        children_text = self._read_block(result_block_id, num_tabs=num_tabs + 1)
                        cur_result_text_arr.append(children_text)

                    cur_result_text = "\n".join(cur_result_text_arr)
                    if result_type in HEADING_SPLITTER:
                        result_lines_arr.append(f"{HEADING_SPLITTER[result_type]}{cur_result_text}")
                    else:
                        result_lines_arr.append(cur_result_text + "\n\n")

            if data["next_cursor"] is None:
                break
            else:
                start_cursor = data["next_cursor"]

        result_lines = "\n".join(result_lines_arr)
        return result_lines

    def _read_table_rows(self, block_id: str) -> str:
        """Read table rows."""
        assert self._notion_access_token is not None, "Notion access token is required"
        done = False
        result_lines_arr = []
        start_cursor = None
        block_url = BLOCK_CHILD_URL_TMPL.format(block_id=block_id)
        while not done:
            query_dict: dict[str, Any] = {} if not start_cursor else {"start_cursor": start_cursor}

            res = httpx.request(
                "GET",
                block_url,
                headers={
                    "Authorization": "Bearer " + self._notion_access_token,
                    "Content-Type": "application/json",
                    "Notion-Version": "2022-06-28",
                },
                params=query_dict,
            )
            data = res.json()
            # get table headers text
            table_header_cell_texts = []
            table_header_cells = data["results"][0]["table_row"]["cells"]
            for table_header_cell in table_header_cells:
                if table_header_cell:
                    for table_header_cell_text in table_header_cell:
                        text = table_header_cell_text["text"]["content"]
                        table_header_cell_texts.append(text)
                else:
                    table_header_cell_texts.append("")
            # Initialize Markdown table with headers
            markdown_table = "| " + " | ".join(table_header_cell_texts) + " |\n"
            markdown_table += "| " + " | ".join(["---"] * len(table_header_cell_texts)) + " |\n"

            # Process data to format each row in Markdown table format
            results = data["results"]
            for i in range(len(results) - 1):
                column_texts = []
                table_column_cells = data["results"][i + 1]["table_row"]["cells"]
                for j in range(len(table_column_cells)):
                    if table_column_cells[j]:
                        for table_column_cell_text in table_column_cells[j]:
                            column_text = table_column_cell_text["text"]["content"]
                            column_texts.append(column_text)
                # Add row to Markdown table
                markdown_table += "| " + " | ".join(column_texts) + " |\n"
            result_lines_arr.append(markdown_table)
            if data["next_cursor"] is None:
                done = True
                break
            else:
                start_cursor = data["next_cursor"]

        result_lines = "\n".join(result_lines_arr)
        return result_lines

    def update_last_edited_time(self, document_model: DocumentModel | None):
        if not document_model:
            return

        last_edited_time = self.get_notion_last_edited_time()
        data_source_info = document_model.data_source_info_dict
        if data_source_info:
            data_source_info["last_edited_time"] = last_edited_time

        db.session.execute(
            update(DocumentModel)
            .where(DocumentModel.id == document_model.id)
            .values(data_source_info=json.dumps(data_source_info))
        )
        db.session.commit()

    def get_notion_last_edited_time(self) -> str:
        assert self._notion_access_token is not None, "Notion access token is required"
        obj_id = self._notion_obj_id
        page_type = self._notion_page_type
        if page_type == "database":
            retrieve_page_url = RETRIEVE_DATABASE_URL_TMPL.format(database_id=obj_id)
        else:
            retrieve_page_url = RETRIEVE_PAGE_URL_TMPL.format(page_id=obj_id)

        query_dict: dict[str, Any] = {}

        res = httpx.request(
            "GET",
            retrieve_page_url,
            headers={
                "Authorization": "Bearer " + self._notion_access_token,
                "Content-Type": "application/json",
                "Notion-Version": "2022-06-28",
            },
            json=query_dict,
        )

        data = res.json()
        return cast(str, data["last_edited_time"])

    @classmethod
    def _get_access_token(cls, tenant_id: str, credential_id: str | None) -> str:
        # get credential from tenant_id and credential_id
        if not credential_id:
            raise Exception(f"No credential id found for tenant {tenant_id}")
        datasource_provider_service = DatasourceProviderService()
        credential = datasource_provider_service.get_datasource_credentials(
            tenant_id=tenant_id,
            credential_id=credential_id,
            provider="notion_datasource",
            plugin_id="langgenius/notion_datasource",
        )
        if not credential:
            raise Exception(f"No notion credential found for tenant {tenant_id} and credential {credential_id}")

        return cast(str, credential["integration_secret"])

```

### api/core/rag/extractor/pdf_extractor.py
```py
"""Abstract interface for document loader implementations."""

import contextlib
import io
import logging
import uuid
from collections.abc import Iterator

import pypdfium2
import pypdfium2.raw as pdfium_c

from configs import dify_config
from core.rag.extractor.blob.blob import Blob
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document
from extensions.ext_database import db
from extensions.ext_storage import storage
from extensions.storage.storage_type import StorageType
from libs.datetime_utils import naive_utc_now
from models.enums import CreatorUserRole
from models.model import UploadFile

logger = logging.getLogger(__name__)


class PdfExtractor(BaseExtractor):
    """
    PdfExtractor is used to extract text and images from PDF files.

    Args:
        file_path: Path to the PDF file.
        tenant_id: Workspace ID.
        user_id: ID of the user performing the extraction.
        file_cache_key: Optional cache key for the extracted text.
    """

    # Magic bytes for image format detection: (magic_bytes, extension, mime_type)
    IMAGE_FORMATS: tuple[tuple[bytes, str, str], ...] = (
        (b"\xff\xd8\xff", "jpg", "image/jpeg"),
        (b"\x89PNG\r\n\x1a\n", "png", "image/png"),
        (b"\x00\x00\x00\x0c\x6a\x50\x20\x20\x0d\x0a\x87\x0a", "jp2", "image/jp2"),
        (b"GIF8", "gif", "image/gif"),
        (b"BM", "bmp", "image/bmp"),
        (b"II*\x00", "tiff", "image/tiff"),
        (b"MM\x00*", "tiff", "image/tiff"),
        (b"II+\x00", "tiff", "image/tiff"),
        (b"MM\x00+", "tiff", "image/tiff"),
    )
    MAX_MAGIC_LEN = max(len(m) for m, _, _ in IMAGE_FORMATS)

    def __init__(self, file_path: str, tenant_id: str, user_id: str, file_cache_key: str | None = None):
        """Initialize PdfExtractor."""
        self._file_path = file_path
        self._tenant_id = tenant_id
        self._user_id = user_id
        self._file_cache_key = file_cache_key

    def extract(self) -> list[Document]:
        plaintext_file_exists = False
        if self._file_cache_key:
            with contextlib.suppress(FileNotFoundError):
                text = storage.load(self._file_cache_key).decode("utf-8")
                plaintext_file_exists = True
                return [Document(page_content=text)]
        documents = list(self.load())
        text_list = []
        for document in documents:
            text_list.append(document.page_content)
        text = "\n\n".join(text_list)

        # save plaintext file for caching
        if not plaintext_file_exists and self._file_cache_key:
            storage.save(self._file_cache_key, text.encode("utf-8"))

        return documents

    def load(
        self,
    ) -> Iterator[Document]:
        """Lazy load given path as pages."""
        blob = Blob.from_path(self._file_path)
        yield from self.parse(blob)

    def parse(self, blob: Blob) -> Iterator[Document]:
        """Lazily parse the blob."""

        with blob.as_bytes_io() as file_path:
            pdf_reader = pypdfium2.PdfDocument(file_path, autoclose=True)
            try:
                for page_number, page in enumerate(pdf_reader):
                    text_page = page.get_textpage()
                    content = text_page.get_text_range()
                    text_page.close()

                    image_content = self._extract_images(page)
                    if image_content:
                        content += "\n" + image_content

                    page.close()
                    metadata = {"source": blob.source, "page": page_number}
                    yield Document(page_content=content, metadata=metadata)
            finally:
                pdf_reader.close()

    def _extract_images(self, page) -> str:
        """
        Extract images from a PDF page, save them to storage and database,
        and return markdown image links.

        Args:
            page: pypdfium2 page object.

        Returns:
            Markdown string containing links to the extracted images.
        """
        image_content = []
        upload_files = []
        base_url = dify_config.INTERNAL_FILES_URL or dify_config.FILES_URL

        try:
            image_objects = page.get_objects(filter=(pdfium_c.FPDF_PAGEOBJ_IMAGE,))
            for obj in image_objects:
                try:
                    # Extract image bytes
                    img_byte_arr = io.BytesIO()
                    # Extract DCTDecode (JPEG) and JPXDecode (JPEG 2000) images directly
                    # Fallback to png for other formats
                    obj.extract(img_byte_arr, fb_format="png")
                    img_bytes = img_byte_arr.getvalue()

                    if not img_bytes:
                        continue

                    header = img_bytes[: self.MAX_MAGIC_LEN]
                    image_ext = None
                    mime_type = None
                    for magic, ext, mime in self.IMAGE_FORMATS:
                        if header.startswith(magic):
                            image_ext = ext
                            mime_type = mime
                            break

                    if not image_ext or not mime_type:
                        continue

                    file_uuid = str(uuid.uuid4())
                    file_key = "image_files/" + self._tenant_id + "/" + file_uuid + "." + image_ext

                    storage.save(file_key, img_bytes)

                    # save file to db
                    upload_file = UploadFile(
                        tenant_id=self._tenant_id,
                        storage_type=StorageType(dify_config.STORAGE_TYPE),
                        key=file_key,
                        name=file_key,
                        size=len(img_bytes),
                        extension=image_ext,
                        mime_type=mime_type,
                        created_by=self._user_id,
                        created_by_role=CreatorUserRole.ACCOUNT,
                        created_at=naive_utc_now(),
                        used=True,
                        used_by=self._user_id,
                        used_at=naive_utc_now(),
                    )
                    upload_files.append(upload_file)
                    image_content.append(f"![image]({base_url}/files/{upload_file.id}/file-preview)")
                except Exception as e:
                    logger.warning("Failed to extract image from PDF: %s", e)
                    continue
        except Exception as e:
            logger.warning("Failed to get objects from PDF page: %s", e)
        if upload_files:
            db.session.add_all(upload_files)
            db.session.commit()
        return "\n".join(image_content)

```

### api/core/rag/extractor/extract_processor.py
```py
import re
import tempfile
from pathlib import Path
from typing import Union
from urllib.parse import unquote

from configs import dify_config
from core.helper import ssrf_proxy
from core.rag.extractor.csv_extractor import CSVExtractor
from core.rag.extractor.entity.datasource_type import DatasourceType
from core.rag.extractor.entity.extract_setting import ExtractSetting
from core.rag.extractor.excel_extractor import ExcelExtractor
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.extractor.firecrawl.firecrawl_web_extractor import FirecrawlWebExtractor
from core.rag.extractor.html_extractor import HtmlExtractor
from core.rag.extractor.jina_reader_extractor import JinaReaderWebExtractor
from core.rag.extractor.markdown_extractor import MarkdownExtractor
from core.rag.extractor.notion_extractor import NotionExtractor
from core.rag.extractor.pdf_extractor import PdfExtractor
from core.rag.extractor.text_extractor import TextExtractor
from core.rag.extractor.unstructured.unstructured_doc_extractor import UnstructuredWordExtractor
from core.rag.extractor.unstructured.unstructured_eml_extractor import UnstructuredEmailExtractor
from core.rag.extractor.unstructured.unstructured_epub_extractor import UnstructuredEpubExtractor
from core.rag.extractor.unstructured.unstructured_markdown_extractor import UnstructuredMarkdownExtractor
from core.rag.extractor.unstructured.unstructured_msg_extractor import UnstructuredMsgExtractor
from core.rag.extractor.unstructured.unstructured_ppt_extractor import UnstructuredPPTExtractor
from core.rag.extractor.unstructured.unstructured_pptx_extractor import UnstructuredPPTXExtractor
from core.rag.extractor.unstructured.unstructured_xml_extractor import UnstructuredXmlExtractor
from core.rag.extractor.watercrawl.extractor import WaterCrawlWebExtractor
from core.rag.extractor.word_extractor import WordExtractor
from core.rag.models.document import Document
from extensions.ext_storage import storage
from models.model import UploadFile

SUPPORT_URL_CONTENT_TYPES = ["application/pdf", "text/plain", "application/json"]
USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124"
    " Safari/537.36"
)


class ExtractProcessor:
    @classmethod
    def load_from_upload_file(
        cls, upload_file: UploadFile, return_text: bool = False, is_automatic: bool = False
    ) -> Union[list[Document], str]:
        extract_setting = ExtractSetting(
            datasource_type=DatasourceType.FILE, upload_file=upload_file, document_model="text_model"
        )
        if return_text:
            delimiter = "\n"
            return delimiter.join([document.page_content for document in cls.extract(extract_setting, is_automatic)])
        else:
            return cls.extract(extract_setting, is_automatic)

    @classmethod
    def load_from_url(cls, url: str, return_text: bool = False) -> Union[list[Document], str]:
        response = ssrf_proxy.get(url, headers={"User-Agent": USER_AGENT})

        with tempfile.TemporaryDirectory() as temp_dir:
            suffix = Path(url).suffix
            if not suffix and suffix != ".":
                # get content-type
                if response.headers.get("Content-Type"):
                    suffix = "." + response.headers.get("Content-Type").split("/")[-1]
                else:
                    content_disposition = response.headers.get("Content-Disposition")
                    filename_match = re.search(r'filename="([^"]+)"', content_disposition)
                    if filename_match:
                        filename = unquote(filename_match.group(1))
                        match = re.search(r"\.(\w+)$", filename)
                        if match:
                            suffix = "." + match.group(1)
                        else:
                            suffix = ""
            # https://stackoverflow.com/questions/26541416/generate-temporary-file-names-without-creating-actual-file-in-python#comment90414256_26541521
            # Generate a temporary filename under the created temp_dir and ensure the directory exists
            file_path = f"{temp_dir}/{next(tempfile._get_candidate_names())}{suffix}"  # type: ignore
            Path(file_path).write_bytes(response.content)
            extract_setting = ExtractSetting(datasource_type=DatasourceType.FILE, document_model="text_model")
            if return_text:
                delimiter = "\n"
                return delimiter.join(
                    [
                        document.page_content
                        for document in cls.extract(extract_setting=extract_setting, file_path=file_path)
                    ]
                )
            else:
                return cls.extract(extract_setting=extract_setting, file_path=file_path)

    @classmethod
    def extract(
        cls, extract_setting: ExtractSetting, is_automatic: bool = False, file_path: str | None = None
    ) -> list[Document]:
        if extract_setting.datasource_type == DatasourceType.FILE:
            with tempfile.TemporaryDirectory() as temp_dir:
                if not file_path:
                    assert extract_setting.upload_file is not None, "upload_file is required"
                    upload_file: UploadFile = extract_setting.upload_file
                    suffix = Path(upload_file.key).suffix
                    # FIXME mypy: Cannot determine type of 'tempfile._get_candidate_names' better not use it here
                    file_path = f"{temp_dir}/{next(tempfile._get_candidate_names())}{suffix}"  # type: ignore
                    storage.download(upload_file.key, file_path)
                input_file = Path(file_path)
                file_extension = input_file.suffix.lower()
                etl_type = dify_config.ETL_TYPE
                extractor: BaseExtractor | None = None
                if etl_type == "Unstructured":
                    unstructured_api_url = dify_config.UNSTRUCTURED_API_URL or ""
                    unstructured_api_key = dify_config.UNSTRUCTURED_API_KEY or ""

                    if file_extension in {".xlsx", ".xls"}:
                        extractor = ExcelExtractor(file_path)
                    elif file_extension == ".pdf":
                        extractor = PdfExtractor(file_path, upload_file.tenant_id, upload_file.created_by)
                    elif file_extension in {".md", ".markdown", ".mdx"}:
                        extractor = (
                            UnstructuredMarkdownExtractor(file_path, unstructured_api_url, unstructured_api_key)
                            if is_automatic
                            else MarkdownExtractor(file_path, autodetect_encoding=True)
                        )
                    elif file_extension in {".htm", ".html"}:
                        extractor = HtmlExtractor(file_path)
                    elif file_extension == ".docx":
                        extractor = WordExtractor(file_path, upload_file.tenant_id, upload_file.created_by)
                    elif file_extension == ".doc":
                        extractor = UnstructuredWordExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    elif file_extension == ".csv":
                        extractor = CSVExtractor(file_path, autodetect_encoding=True)
                    elif file_extension == ".msg":
                        extractor = UnstructuredMsgExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    elif file_extension == ".eml":
                        extractor = UnstructuredEmailExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    elif file_extension == ".ppt":
                        extractor = UnstructuredPPTExtractor(file_path, unstructured_api_url, unstructured_api_key)
                        # You must first specify the API key
                        # because unstructured_api_key is necessary to parse .ppt documents
                    elif file_extension == ".pptx":
                        extractor = UnstructuredPPTXExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    elif file_extension == ".xml":
                        extractor = UnstructuredXmlExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    elif file_extension == ".epub":
                        extractor = UnstructuredEpubExtractor(file_path, unstructured_api_url, unstructured_api_key)
                    else:
                        # txt
                        extractor = TextExtractor(file_path, autodetect_encoding=True)
                else:
                    if file_extension in {".xlsx", ".xls"}:
                        extractor = ExcelExtractor(file_path)
                    elif file_extension == ".pdf":
                        extractor = PdfExtractor(file_path, upload_file.tenant_id, upload_file.created_by)
                    elif file_extension in {".md", ".markdown", ".mdx"}:
                        extractor = MarkdownExtractor(file_path, autodetect_encoding=True)
                    elif file_extension in {".htm", ".html"}:
                        extractor = HtmlExtractor(file_path)
                    elif file_extension == ".docx":
                        extractor = WordExtractor(file_path, upload_file.tenant_id, upload_file.created_by)
                    elif file_extension == ".csv":
                        extractor = CSVExtractor(file_path, autodetect_encoding=True)
                    elif file_extension == ".epub":
                        extractor = UnstructuredEpubExtractor(file_path)
                    else:
                        # txt
                        extractor = TextExtractor(file_path, autodetect_encoding=True)
                return extractor.extract()
        elif extract_setting.datasource_type == DatasourceType.NOTION:
            assert extract_setting.notion_info is not None, "notion_info is required"
            extractor = NotionExtractor(
                notion_workspace_id=extract_setting.notion_info.notion_workspace_id or "",
                notion_obj_id=extract_setting.notion_info.notion_obj_id,
                notion_page_type=extract_setting.notion_info.notion_page_type,
                document_model=extract_setting.notion_info.document,
                tenant_id=extract_setting.notion_info.tenant_id,
                credential_id=extract_setting.notion_info.credential_id,
            )
            return extractor.extract()
        elif extract_setting.datasource_type == DatasourceType.WEBSITE:
            assert extract_setting.website_info is not None, "website_info is required"
            if extract_setting.website_info.provider == "firecrawl":
                extractor = FirecrawlWebExtractor(
                    url=extract_setting.website_info.url,
                    job_id=extract_setting.website_info.job_id,
                    tenant_id=extract_setting.website_info.tenant_id,
                    mode=extract_setting.website_info.mode,
                    only_main_content=extract_setting.website_info.only_main_content,
                )
                return extractor.extract()
            elif extract_setting.website_info.provider == "watercrawl":
                extractor = WaterCrawlWebExtractor(
                    url=extract_setting.website_info.url,
                    job_id=extract_setting.website_info.job_id,
                    tenant_id=extract_setting.website_info.tenant_id,
                    mode=extract_setting.website_info.mode,
                    only_main_content=extract_setting.website_info.only_main_content,
                )
                return extractor.extract()
            elif extract_setting.website_info.provider == "jinareader":
                extractor = JinaReaderWebExtractor(
                    url=extract_setting.website_info.url,
                    job_id=extract_setting.website_info.job_id,
                    tenant_id=extract_setting.website_info.tenant_id,
                    mode=extract_setting.website_info.mode,
                    only_main_content=extract_setting.website_info.only_main_content,
                )
                return extractor.extract()
            else:
                raise ValueError(f"Unsupported website provider: {extract_setting.website_info.provider}")
        else:
            raise ValueError(f"Unsupported datasource type: {extract_setting.datasource_type}")

```

### api/core/rag/extractor/unstructured/unstructured_xml_extractor.py
```py
import logging

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredXmlExtractor(BaseExtractor):
    """Load xml files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.xml import partition_xml

            elements = partition_xml(filename=self._file_path, xml_keep_tags=True)

        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))

        return documents

```

### api/core/rag/extractor/unstructured/unstructured_markdown_extractor.py
```py
import logging

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredMarkdownExtractor(BaseExtractor):
    """Load md files.


    Args:
        file_path: Path to the file to load.

    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.md import partition_md

            elements = partition_md(filename=self._file_path)
        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))

        return documents

```

### api/core/rag/extractor/unstructured/unstructured_msg_extractor.py
```py
import logging

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredMsgExtractor(BaseExtractor):
    """Load msg files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.msg import partition_msg

            elements = partition_msg(filename=self._file_path)
        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))

        return documents

```

### api/core/rag/extractor/unstructured/unstructured_ppt_extractor.py
```py
import logging

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredPPTExtractor(BaseExtractor):
    """Load ppt files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            raise NotImplementedError("Unstructured API Url is not configured")
        text_by_page: dict[int, str] = {}
        for element in elements:
            page = element.metadata.page_number
            if page is None:
                continue
            text = element.text
            if page in text_by_page:
                text_by_page[page] += "\n" + text
            else:
                text_by_page[page] = text

        combined_texts = list(text_by_page.values())
        documents = []
        for combined_text in combined_texts:
            text = combined_text.strip()
            documents.append(Document(page_content=text))
        return documents

```

### api/core/rag/extractor/unstructured/unstructured_doc_extractor.py
```py
import logging
import os

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredWordExtractor(BaseExtractor):
    """Loader that uses unstructured to load word documents."""

    def __init__(self, file_path: str, api_url: str, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        from unstructured.__version__ import __version__ as __unstructured_version__
        from unstructured.file_utils.filetype import FileType, detect_filetype

        unstructured_version = tuple(int(x) for x in __unstructured_version__.split("."))
        # check the file extension
        try:
            import magic  # noqa: F401

            is_doc = detect_filetype(self._file_path) == FileType.DOC
        except ImportError:
            _, extension = os.path.splitext(str(self._file_path))
            is_doc = extension == ".doc"

        if is_doc and unstructured_version < (0, 4, 11):
            raise ValueError(
                f"You are on unstructured version {__unstructured_version__}. "
                "Partitioning .doc files is only supported in unstructured>=0.4.11. "
                "Please upgrade the unstructured package and try again."
            )

        if is_doc:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)

        else:
            from unstructured.partition.docx import partition_docx

            elements = partition_docx(filename=self._file_path)

        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))
        return documents

```

### api/core/rag/extractor/unstructured/unstructured_pptx_extractor.py
```py
import logging

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredPPTXExtractor(BaseExtractor):
    """Load pptx files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.pptx import partition_pptx

            elements = partition_pptx(filename=self._file_path)
        text_by_page: dict[int, str] = {}
        for element in elements:
            page = element.metadata.page_number
            text = element.text
            if page is not None:
                if page in text_by_page:
                    text_by_page[page] += "\n" + text
                else:
                    text_by_page[page] = text

        combined_texts = list(text_by_page.values())
        documents = []
        for combined_text in combined_texts:
            text = combined_text.strip()
            documents.append(Document(page_content=text))

        return documents

```

### api/core/rag/extractor/unstructured/unstructured_epub_extractor.py
```py
import logging

import pypandoc  # type: ignore

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredEpubExtractor(BaseExtractor):
    """Load epub files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(
        self,
        file_path: str,
        api_url: str | None = None,
        api_key: str = "",
    ):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.epub import partition_epub

            pypandoc.download_pandoc()
            elements = partition_epub(filename=self._file_path, xml_keep_tags=True)

        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))

        return documents

```

### api/core/rag/extractor/unstructured/unstructured_eml_extractor.py
```py
import base64
import contextlib
import logging

from bs4 import BeautifulSoup

from configs import dify_config
from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document

logger = logging.getLogger(__name__)


class UnstructuredEmailExtractor(BaseExtractor):
    """Load eml files.
    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, api_url: str | None = None, api_key: str = ""):
        """Initialize with file path."""
        self._file_path = file_path
        self._api_url = api_url
        self._api_key = api_key

    def extract(self) -> list[Document]:
        if self._api_url:
            from unstructured.partition.api import partition_via_api

            elements = partition_via_api(filename=self._file_path, api_url=self._api_url, api_key=self._api_key)
        else:
            from unstructured.partition.email import partition_email

            elements = partition_email(filename=self._file_path)

        # noinspection PyBroadException
        with contextlib.suppress(Exception):
            for element in elements:
                element_text = element.text.strip()

                padding_needed = 4 - len(element_text) % 4
                element_text += "=" * padding_needed

                element_decode = base64.b64decode(element_text)
                soup = BeautifulSoup(element_decode.decode("utf-8"), "html.parser")
                element.text = soup.get_text()

        from unstructured.chunking.title import chunk_by_title

        max_characters = dify_config.INDEXING_MAX_SEGMENTATION_TOKENS_LENGTH
        chunks = chunk_by_title(elements, max_characters=max_characters, combine_text_under_n_chars=max_characters)
        documents = []
        for chunk in chunks:
            text = chunk.text.strip()
            documents.append(Document(page_content=text))
        return documents

```

### api/core/rag/extractor/blob/blob.py
```py
"""Schema for Blobs and Blob Loaders.

The goal is to facilitate decoupling of content loading from content parsing code.

In addition, content loading code should provide a lazy loading interface by default.
"""

from __future__ import annotations

import contextlib
import mimetypes
from collections.abc import Generator, Mapping
from io import BufferedReader, BytesIO
from pathlib import Path, PurePath
from typing import Any

from pydantic import BaseModel, ConfigDict, model_validator

type PathLike = str | PurePath


class Blob(BaseModel):
    """A blob is used to represent raw data by either reference or value.

    Provides an interface to materialize the blob in different representations, and
    help to decouple the development of data loaders from the downstream parsing of
    the raw data.

    Inspired by: https://developer.mozilla.org/en-US/docs/Web/API/Blob
    """

    data: bytes | str | None = None  # Raw data
    mimetype: str | None = None  # Not to be confused with a file extension
    encoding: str = "utf-8"  # Use utf-8 as default encoding, if decoding to string
    # Location where the original content was found
    # Represent location on the local file system
    # Useful for situations where downstream code assumes it must work with file paths
    # rather than in-memory content.
    path: PathLike | None = None
    model_config = ConfigDict(arbitrary_types_allowed=True, frozen=True)

    @property
    def source(self) -> str | None:
        """The source location of the blob as string if known otherwise none."""
        return str(self.path) if self.path else None

    @model_validator(mode="before")
    @classmethod
    def check_blob_is_valid(cls, values: Mapping[str, Any]) -> Mapping[str, Any]:
        """Verify that either data or path is provided."""
        if "data" not in values and "path" not in values:
            raise ValueError("Either data or path must be provided")
        return values

    def as_string(self) -> str:
        """Read data as a string."""
        if self.data is None and self.path:
            return Path(str(self.path)).read_text(encoding=self.encoding)
        elif isinstance(self.data, bytes):
            return self.data.decode(self.encoding)
        elif isinstance(self.data, str):
            return self.data
        else:
            raise ValueError(f"Unable to get string for blob {self}")

    def as_bytes(self) -> bytes:
        """Read data as bytes."""
        if isinstance(self.data, bytes):
            return self.data
        elif isinstance(self.data, str):
            return self.data.encode(self.encoding)
        elif self.data is None and self.path:
            return Path(str(self.path)).read_bytes()
        else:
            raise ValueError(f"Unable to get bytes for blob {self}")

    @contextlib.contextmanager
    def as_bytes_io(self) -> Generator[BytesIO | BufferedReader, None, None]:
        """Read data as a byte stream."""
        if isinstance(self.data, bytes):
            yield BytesIO(self.data)
        elif self.data is None and self.path:
            with open(str(self.path), "rb") as f:
                yield f
        else:
            raise NotImplementedError(f"Unable to convert blob {self}")

    @classmethod
    def from_path(
        cls,
        path: PathLike,
        *,
        encoding: str = "utf-8",
        mime_type: str | None = None,
        guess_type: bool = True,
    ) -> Blob:
        """Load the blob from a path like object.

        Args:
            path: path like object to file to be read
            encoding: Encoding to use if decoding the bytes into a string
            mime_type: if provided, will be set as the mime-type of the data
            guess_type: If True, the mimetype will be guessed from the file extension,
                        if a mime-type was not provided

        Returns:
            Blob instance
        """
        if mime_type is None and guess_type:
            _mimetype = mimetypes.guess_type(path)[0]
        else:
            _mimetype = mime_type
        # We do not load the data immediately, instead we treat the blob as a
        # reference to the underlying data.
        return cls(data=None, mimetype=_mimetype, encoding=encoding, path=path)

    @classmethod
    def from_data(
        cls,
        data: str | bytes,
        *,
        encoding: str = "utf-8",
        mime_type: str | None = None,
        path: str | None = None,
    ) -> Blob:
        """Initialize the blob from in-memory data.

        Args:
            data: the in-memory data associated with the blob
            encoding: Encoding to use if decoding the bytes into a string
            mime_type: if provided, will be set as the mime-type of the data
            path: if provided, will be set as the source from which the data came

        Returns:
            Blob instance
        """
        return cls(data=data, mimetype=mime_type, encoding=encoding, path=path)

    def __repr__(self) -> str:
        """Define the blob representation."""
        str_repr = f"Blob {id(self)}"
        if self.source:
            str_repr += f" {self.source}"
        return str_repr

```

### api/core/rag/extractor/helpers.py
```py
"""Document loader helpers."""

import concurrent.futures
from typing import NamedTuple

import charset_normalizer


class FileEncoding(NamedTuple):
    """A file encoding as the NamedTuple."""

    encoding: str | None
    """The encoding of the file."""
    confidence: float
    """The confidence of the encoding."""
    language: str | None
    """The language of the file."""


def detect_file_encodings(file_path: str, timeout: int = 5, sample_size: int = 1024 * 1024) -> list[FileEncoding]:
    """Try to detect the file encoding.

    Returns a list of `FileEncoding` tuples with the detected encodings ordered
    by confidence.

    Args:
        file_path: The path to the file to detect the encoding for.
        timeout: The timeout in seconds for the encoding detection.
        sample_size: The number of bytes to read for encoding detection. Default is 1MB.
                    For large files, reading only a sample is sufficient and prevents timeout.
    """

    def read_and_detect(filename: str):
        rst = charset_normalizer.from_path(filename)
        best = rst.best()
        if best is None:
            return []
        file_encoding = FileEncoding(encoding=best.encoding, confidence=best.coherence, language=best.language)
        return [file_encoding]

    with concurrent.futures.ThreadPoolExecutor() as executor:
        future = executor.submit(read_and_detect, file_path)
        try:
            encodings = future.result(timeout=timeout)
        except concurrent.futures.TimeoutError:
            raise TimeoutError(f"Timeout reached while detecting encoding for {file_path}")

    if all(encoding.encoding is None for encoding in encodings):
        raise RuntimeError(f"Could not detect encoding for {file_path}")
    return [enc for enc in encodings if enc.encoding is not None]

```

### api/core/rag/extractor/excel_extractor.py
```py
"""Abstract interface for document loader implementations."""

import os
from typing import TypedDict

import pandas as pd
from openpyxl import load_workbook

from core.rag.extractor.extractor_base import BaseExtractor
from core.rag.models.document import Document


class Candidate(TypedDict):
    idx: int
    count: int
    map: dict[int, str]


class ExcelExtractor(BaseExtractor):
    """Load Excel files.


    Args:
        file_path: Path to the file to load.
    """

    def __init__(self, file_path: str, encoding: str | None = None, autodetect_encoding: bool = False):
        """Initialize with file path."""
        self._file_path = file_path
        self._encoding = encoding
        self._autodetect_encoding = autodetect_encoding

    def extract(self) -> list[Document]:
        """Load from Excel file in xls or xlsx format using Pandas and openpyxl."""
        documents = []
        file_extension = os.path.splitext(self._file_path)[-1].lower()

        if file_extension == ".xlsx":
            wb = load_workbook(self._file_path, read_only=True, data_only=True)
            try:
                for sheet_name in wb.sheetnames:
                    sheet = wb[sheet_name]
                    header_row_idx, column_map, max_col_idx = self._find_header_and_columns(sheet)
                    if not column_map:
                        continue
                    start_row = header_row_idx + 1
                    for row in sheet.iter_rows(min_row=start_row, max_col=max_col_idx, values_only=False):
                        if all(cell.value is None for cell in row):
                            continue
                        page_content = []
                        for col_idx, cell in enumerate(row):
                            value = cell.value
                            if col_idx in column_map:
                                col_name = column_map[col_idx]
                                if hasattr(cell, "hyperlink") and cell.hyperlink:
                                    target = getattr(cell.hyperlink, "target", None)
                                    if target:
                                        value = f"[{value}]({target})"
                                if value is None:
                                    value = ""
                                elif not isinstance(value, str):
                                    value = str(value)
                                value = value.strip().replace('"', '\\"')
                                page_content.append(f'"{col_name}":"{value}"')
                        if page_content:
                            documents.append(
                                Document(page_content=";".join(page_content), metadata={"source": self._file_path})
                            )
            finally:
                wb.close()

        elif file_extension == ".xls":
            excel_file = pd.ExcelFile(self._file_path, engine="xlrd")
            for excel_sheet_name in excel_file.sheet_names:
                df = excel_file.parse(sheet_name=excel_sheet_name)
                df.dropna(how="all", inplace=True)

                for _, series_row in df.iterrows():
                    page_content = []
                    for k, v in series_row.items():
                        if pd.notna(v):
                            page_content.append(f'"{k}":"{v}"')
                    documents.append(
                        Document(page_content=";".join(page_content), metadata={"source": self._file_path})
                    )
        else:
            raise ValueError(f"Unsupported file extension: {file_extension}")

        return documents

    def _find_header_and_columns(self, sheet, scan_rows=10) -> tuple[int, dict[int, str], int]:
        """
        Scan first N rows to find the most likely header row.
        Returns:
            header_row_idx: 1-based index of the header row
            column_map: Dict mapping 0-based column index to column name
            max_col_idx: 1-based index of the last valid column (for iter_rows boundary)
        """
        # Store potential candidates: (row_index, non_empty_count, column_map)
        candidates: list[Candidate] = []

        # Limit scan to avoid performance issues on huge files
        # We iterate manually to control the read scope
        for current_row_idx, row in enumerate(sheet.iter_rows(min_row=1, max_row=scan_rows, values_only=True), start=1):
            # Filter out empty cells and build a temp map for this row
            # col_idx is 0-based
            row_map = {}
            for col_idx, cell_value in enumerate(row):
                if cell_value is not None and str(cell_value).strip():
                    row_map[col_idx] = str(cell_value).strip().replace('"', '\\"')

            if not row_map:
                continue

            non_empty_count = len(row_map)

            # Header selection heuristic (implemented):
            # - Prefer the first row with at least 2 non-empty columns.
            # - Fallback: choose the row with the most non-empty columns
            #   (tie-breaker: smaller row index).
            candidates.append({"idx": current_row_idx, "count": non_empty_count, "map": row_map})

        if not candidates:
            return 0, {}, 0

        # Choose the best candidate header row.

        best_candidate: Candidate | None = None

        # Strategy: prefer the first row with >= 2 non-empty columns; otherwise fallback.

        for cand in candidates:
            if cand["count"] >= 2:
                best_candidate = cand
                break

        # Fallback: if no row has >= 2 columns, or all have 1, just take the one with max columns
        if not best_candidate:
            # Sort by count desc, then index asc
            candidates.sort(key=lambda x: (-x["count"], x["idx"]))
            best_candidate = candidates[0]

        # Determine max_col_idx (1-based for openpyxl)
        # It is the index of the last valid column in our map + 1
        max_col_idx = max(best_candidate["map"].keys()) + 1

        return best_candidate["idx"], best_candidate["map"], max_col_idx

```

### api/core/rag/models/__init__.py
```py

```

### api/core/rag/models/document.py
```py
from abc import ABC, abstractmethod
from collections.abc import Sequence
from typing import Any

from graphon.file import File
from pydantic import BaseModel, Field


class ChildDocument(BaseModel):
    """Class for storing a piece of text and associated metadata."""

    page_content: str

    vector: list[float] | None = None

    """Arbitrary metadata about the page content (e.g., source, relationships to other
        documents, etc.).
    """
    metadata: dict[str, Any] = Field(default_factory=dict)


class AttachmentDocument(BaseModel):
    """Class for storing a piece of text and associated metadata."""

    page_content: str

    provider: str | None = "dify"

    vector: list[float] | None = None

    metadata: dict[str, Any] = Field(default_factory=dict)


class Document(BaseModel):
    """Class for storing a piece of text and associated metadata."""

    page_content: str

    vector: list[float] | None = None

    """Arbitrary metadata about the page content (e.g., source, relationships to other
        documents, etc.).
    """
    metadata: dict[str, Any] = Field(default_factory=dict)

    provider: str | None = "dify"

    children: list[ChildDocument] | None = None

    attachments: list[AttachmentDocument] | None = None


class GeneralChunk(BaseModel):
    """
    General Chunk.
    """

    content: str
    files: list[File] | None = None


class MultimodalGeneralStructureChunk(BaseModel):
    """
    Multimodal General Structure Chunk.
    """

    general_chunks: list[GeneralChunk]


class GeneralStructureChunk(BaseModel):
    """
    General Structure Chunk.
    """

    general_chunks: list[str]


class ParentChildChunk(BaseModel):
    """
    Parent Child Chunk.
    """

    parent_content: str
    child_contents: list[str]
    files: list[File] | None = None


class ParentChildStructureChunk(BaseModel):
    """
    Parent Child Structure Chunk.
    """

    parent_child_chunks: list[ParentChildChunk]
    parent_mode: str = "paragraph"


class QAChunk(BaseModel):
    """
    QA Chunk.
    """

    question: str
    answer: str


class QAStructureChunk(BaseModel):
    """
    QAStructureChunk.
    """

    qa_chunks: list[QAChunk]


class BaseDocumentTransformer(ABC):
    """Abstract base class for document transformation systems.

    A document transformation system takes a sequence of Documents and returns a
    sequence of transformed Documents.

    Example:
        .. code-block:: python

            class EmbeddingsRedundantFilter(BaseDocumentTransformer, BaseModel):
                model_config = ConfigDict(arbitrary_types_allowed=True)

                embeddings: Embeddings
                similarity_fn: Callable = cosine_similarity
                similarity_threshold: float = 0.95

                def transform_documents(
                    self, documents: Sequence[Document], **kwargs: Any
                ) -> Sequence[Document]:
                    stateful_documents = get_stateful_documents(documents)
                    embedded_documents = _get_embeddings_from_stateful_docs(
                        self.embeddings, stateful_documents
                    )
                    included_idxs = _filter_similar_embeddings(
                        embedded_documents, self.similarity_fn, self.similarity_threshold
                    )
                    return [stateful_documents[i] for i in sorted(included_idxs)]

                async def atransform_documents(
                    self, documents: Sequence[Document], **kwargs: Any
                ) -> Sequence[Document]:
                    raise NotImplementedError

    """

    @abstractmethod
    def transform_documents(self, documents: Sequence[Document], **kwargs: Any) -> Sequence[Document]:
        """Transform a list of documents.

        Args:
            documents: A sequence of Documents to be transformed.

        Returns:
            A list of transformed Documents.
        """

    @abstractmethod
    async def atransform_documents(self, documents: Sequence[Document], **kwargs: Any) -> Sequence[Document]:
        """Asynchronously transform a list of documents.

        Args:
            documents: A sequence of Documents to be transformed.

        Returns:
            A list of transformed Documents.
        """

```

### api/core/rag/retrieval/output_parser/react_output.py
```py
from __future__ import annotations

from dataclasses import dataclass
from typing import NamedTuple, Union


@dataclass
class ReactAction:
    """A full description of an action for an ReactAction to execute."""

    tool: str
    """The name of the Tool to execute."""
    tool_input: Union[str, dict]
    """The input to pass in to the Tool."""
    log: str
    """Additional information to log about the action."""


class ReactFinish(NamedTuple):
    """The final return value of an ReactFinish."""

    return_values: dict
    """Dictionary of return values."""
    log: str
    """Additional information to log about the return value"""

```

### api/core/rag/retrieval/output_parser/__init__.py
```py

```

### api/core/rag/retrieval/output_parser/structured_chat.py
```py
import json
import re
from typing import Union

from core.rag.retrieval.output_parser.react_output import ReactAction, ReactFinish


class StructuredChatOutputParser:
    def parse(self, text: str) -> Union[ReactAction, ReactFinish]:
        try:
            action_match = re.search(r"```(\w*)\n?({.*?)```", text, re.DOTALL)
            if action_match is not None:
                response = json.loads(action_match.group(2).strip(), strict=False)
                if isinstance(response, list):
                    response = response[0]
                if response["action"] == "Final Answer":
                    return ReactFinish({"output": response["action_input"]}, text)
                else:
                    return ReactAction(response["action"], response.get("action_input", {}), text)
            else:
                return ReactFinish({"output": text}, text)
        except Exception:
            raise ValueError(f"Could not parse LLM output: {text}")

```

### api/core/rag/retrieval/__init__.py
```py

```

### api/core/rag/retrieval/retrieval_methods.py
```py
from enum import StrEnum


class RetrievalMethod(StrEnum):
    SEMANTIC_SEARCH = "semantic_search"
    FULL_TEXT_SEARCH = "full_text_search"
    HYBRID_SEARCH = "hybrid_search"
    KEYWORD_SEARCH = "keyword_search"

    @staticmethod
    def is_support_semantic_search(retrieval_method: str) -> bool:
        return retrieval_method in {RetrievalMethod.SEMANTIC_SEARCH, RetrievalMethod.HYBRID_SEARCH}

    @staticmethod
    def is_support_fulltext_search(retrieval_method: str) -> bool:
        return retrieval_method in {RetrievalMethod.FULL_TEXT_SEARCH, RetrievalMethod.HYBRID_SEARCH}

```

### api/core/rag/retrieval/template_prompts.py
```py
METADATA_FILTER_SYSTEM_PROMPT = """
    ### Job Description',
    You are a text metadata extract engine that extract text's metadata based on user input and set the metadata value
    ### Task
    Your task is to ONLY extract the metadatas that exist in the input text from the provided metadata list and Use the following operators ["contains", "not contains", "start with", "end with", "is", "is not", "empty", "not empty", "=", "≠", ">", "<", "≥", "≤", "before", "after"] to express logical relationships, then return result in JSON format with the key "metadata_fields" and value "metadata_field_value" and comparison operator "comparison_operator".
    ### Format
    The input text is in the variable input_text. Metadata are specified as a list in the variable metadata_fields.
    ### Constraint
    DO NOT include anything other than the JSON array in your response.
"""  # noqa: E501

METADATA_FILTER_USER_PROMPT_1 = """
    { "input_text": "I want to know which company’s email address test@example.com is?",
    "metadata_fields": ["filename", "email", "phone", "address"]
    }
"""

METADATA_FILTER_ASSISTANT_PROMPT_1 = """
```json
    {"metadata_map": [
        {"metadata_field_name": "email", "metadata_field_value": "test@example.com", "comparison_operator": "="}
    ]
    }
```
"""

METADATA_FILTER_USER_PROMPT_2 = """
    {"input_text": "What are the movies with a score of more than 9 in 2024?",
    "metadata_fields": ["name", "year", "rating", "country"]}
"""

METADATA_FILTER_ASSISTANT_PROMPT_2 = """
```json
    {"metadata_map": [
        {"metadata_field_name": "year", "metadata_field_value": "2024", "comparison_operator": "="},
        {"metadata_field_name": "rating", "metadata_field_value": "9", "comparison_operator": ">"},
    ]}
```
"""

METADATA_FILTER_USER_PROMPT_3 = """
    '{{"input_text": "{input_text}",',
    '"metadata_fields": {metadata_fields}}}'
"""

METADATA_FILTER_COMPLETION_PROMPT = """
### Job Description
You are a text metadata extract engine that extract text's metadata based on user input and set the metadata value
### Task
# Your task is to ONLY extract the metadatas that exist in the input text from the provided metadata list and Use the following operators ["=", "!=", ">", "<", ">=", "<="] to express logical relationships, then return result in JSON format with the key "metadata_fields" and value "metadata_field_value" and comparison operator "comparison_operator".
### Format
The input text is in the variable input_text. Metadata are specified as a list in the variable metadata_fields.
### Constraint
DO NOT include anything other than the JSON array in your response.
### Example
Here is the chat example between human and assistant, inside <example></example> XML tags.
<example>
User:{{"input_text": ["I want to know which company’s email address test@example.com is?"], "metadata_fields": ["filename", "email", "phone", "address"]}}
Assistant:{{"metadata_map": [{{"metadata_field_name": "email", "metadata_field_value": "test@example.com", "comparison_operator": "="}}]}}
User:{{"input_text": "What are the movies with a score of more than 9 in 2024?", "metadata_fields": ["name", "year", "rating", "country"]}}
Assistant:{{"metadata_map": [{{"metadata_field_name": "year", "metadata_field_value": "2024", "comparison_operator": "="}, {{"metadata_field_name": "rating", "metadata_field_value": "9", "comparison_operator": ">"}}]}}
</example>
### User Input
{{"input_text" : "{input_text}", "metadata_fields" : {metadata_fields}}}
### Assistant Output
"""  # noqa: E501

```

### api/core/rag/retrieval/router/multi_dataset_function_call_router.py
```py
from typing import Union

from graphon.model_runtime.entities.llm_entities import LLMResult, LLMUsage
from graphon.model_runtime.entities.message_entities import PromptMessageTool, SystemPromptMessage, UserPromptMessage

from core.app.entities.app_invoke_entities import ModelConfigWithCredentialsEntity
from core.model_manager import ModelInstance


class FunctionCallMultiDatasetRouter:
    def invoke(
        self,
        query: str,
        dataset_tools: list[PromptMessageTool],
        model_config: ModelConfigWithCredentialsEntity,
        model_instance: ModelInstance,
    ) -> tuple[Union[str, None], LLMUsage]:
        """Given input, decided what to do.
        Returns:
            Action specifying what tool to use.
        """
        if len(dataset_tools) == 0:
            return None, LLMUsage.empty_usage()
        elif len(dataset_tools) == 1:
            return dataset_tools[0].name, LLMUsage.empty_usage()

        try:
            prompt_messages = [
                SystemPromptMessage(content="You are a helpful AI assistant."),
                UserPromptMessage(content=query),
            ]
            result: LLMResult = model_instance.invoke_llm(
                prompt_messages=prompt_messages,
                tools=dataset_tools,
                stream=False,
                model_parameters={"temperature": 0.2, "top_p": 0.3, "max_tokens": 1500},
            )
            usage = result.usage or LLMUsage.empty_usage()
            if result.message.tool_calls:
                # get retrieval model config
                return result.message.tool_calls[0].function.name, usage
            return None, usage
        except Exception:
            return None, LLMUsage.empty_usage()

```

### api/core/rag/retrieval/router/multi_dataset_react_route.py
```py
from collections.abc import Generator, Sequence
from typing import Union

from graphon.model_runtime.entities.llm_entities import LLMResult, LLMUsage
from graphon.model_runtime.entities.message_entities import PromptMessage, PromptMessageRole, PromptMessageTool
from graphon.model_runtime.entities.model_entities import ModelType

from core.app.entities.app_invoke_entities import ModelConfigWithCredentialsEntity
from core.app.llm import deduct_llm_quota
from core.model_manager import ModelInstance, ModelManager
from core.prompt.advanced_prompt_transform import AdvancedPromptTransform
from core.prompt.entities.advanced_prompt_entities import ChatModelMessage, CompletionModelPromptTemplate
from core.rag.retrieval.output_parser.react_output import ReactAction
from core.rag.retrieval.output_parser.structured_chat import StructuredChatOutputParser

PREFIX = """Respond to the human as helpfully and accurately as possible. You have access to the following tools:"""

SUFFIX = """Begin! Reminder to ALWAYS respond with a valid json blob of a single action. Use tools if necessary. Respond directly if appropriate. Format is Action:```$JSON_BLOB```then Observation:.
Thought:"""  # noqa: E501

FORMAT_INSTRUCTIONS = """Use a json blob to specify a tool by providing an action key (tool name) and an action_input key (tool input).
The nouns in the format of "Thought", "Action", "Action Input", "Final Answer" must be expressed in English.
Valid "action" values: "Final Answer" or {tool_names}

Provide only ONE action per $JSON_BLOB, as shown:

```
{{
  "action": $TOOL_NAME,
  "action_input": $INPUT
}}
```

Follow this format:

Question: input question to answer
Thought: consider previous and subsequent steps
Action:
```
$JSON_BLOB
```
Observation: action result
... (repeat Thought/Action/Observation N times)
Thought: I know what to respond
Action:
```
{{
  "action": "Final Answer",
  "action_input": "Final response to human"
}}
```"""  # noqa: E501


class ReactMultiDatasetRouter:
    def invoke(
        self,
        query: str,
        dataset_tools: list[PromptMessageTool],
        model_config: ModelConfigWithCredentialsEntity,
        model_instance: ModelInstance,
        user_id: str,
        tenant_id: str,
    ) -> tuple[Union[str, None], LLMUsage]:
        """Given input, decided what to do.
        Returns:
            Action specifying what tool to use.
        """
        if len(dataset_tools) == 0:
            return None, LLMUsage.empty_usage()
        elif len(dataset_tools) == 1:
            return dataset_tools[0].name, LLMUsage.empty_usage()

        try:
            return self._react_invoke(
                query=query,
                model_config=model_config,
                model_instance=model_instance,
                tools=dataset_tools,
                user_id=user_id,
                tenant_id=tenant_id,
            )
        except Exception:
            return None, LLMUsage.empty_usage()

    def _react_invoke(
        self,
        query: str,
        model_config: ModelConfigWithCredentialsEntity,
        model_instance: ModelInstance,
        tools: Sequence[PromptMessageTool],
        user_id: str,
        tenant_id: str,
        prefix: str = PREFIX,
        suffix: str = SUFFIX,
        format_instructions: str = FORMAT_INSTRUCTIONS,
    ) -> tuple[Union[str, None], LLMUsage]:
        prompt: Union[list[ChatModelMessage], CompletionModelPromptTemplate]
        if model_config.mode == "chat":
            prompt = self.create_chat_prompt(
                query=query,
                tools=tools,
                prefix=prefix,
                suffix=suffix,
                format_instructions=format_instructions,
            )
        else:
            prompt = self.create_completion_prompt(
                tools=tools,
                prefix=prefix,
                format_instructions=format_instructions,
            )
        stop = ["Observation:"]
        # handle invoke result
        prompt_transform = AdvancedPromptTransform()
        prompt_messages = prompt_transform.get_prompt(
            prompt_template=prompt,
            inputs={},
            query="",
            files=[],
            context="",
            memory_config=None,
            memory=None,
            model_config=model_config,
            model_instance=model_instance,
        )
        result_text, usage = self._invoke_llm(
            completion_param=model_config.parameters,
            model_instance=model_instance,
            prompt_messages=prompt_messages,
            stop=stop,
            user_id=user_id,
            tenant_id=tenant_id,
        )
        output_parser = StructuredChatOutputParser()
        react_decision = output_parser.parse(result_text)
        if isinstance(react_decision, ReactAction):
            return react_decision.tool, usage
        return None, usage

    def _invoke_llm(
        self,
        completion_param: dict,
        model_instance: ModelInstance,
        prompt_messages: list[PromptMessage],
        stop: list[str],
        user_id: str,
        tenant_id: str,
    ) -> tuple[str, LLMUsage]:
        """
        Invoke large language model
        :param model_instance: model instance
        :param prompt_messages: prompt messages
        :param stop: stop
        :return:
        """
        bound_model_instance = ModelManager.for_tenant(tenant_id=tenant_id, user_id=user_id).get_model_instance(
            tenant_id=tenant_id,
            provider=model_instance.provider,
            model_type=ModelType.LLM,
            model=model_instance.model_name,
        )
        invoke_result: Generator[LLMResult, None, None] = bound_model_instance.invoke_llm(
            prompt_messages=prompt_messages,
            model_parameters=completion_param,
            stop=stop,
            stream=True,
        )

        # handle invoke result
        text, usage = self._handle_invoke_result(invoke_result=invoke_result)

        # deduct quota
        deduct_llm_quota(tenant_id=tenant_id, model_instance=bound_model_instance, usage=usage)

        return text, usage

    def _handle_invoke_result(self, invoke_result: Generator) -> tuple[str, LLMUsage]:
        """
        Handle invoke result
        :param invoke_result: invoke result
        :return:
        """
        model = None
        prompt_messages: list[PromptMessage] = []
        full_text = ""
        usage = None
        for result in invoke_result:
            text = result.delta.message.content
            full_text += text

            if not model:
                model = result.model

            if not prompt_messages:
                prompt_messages = result.prompt_messages

            if not usage and result.delta.usage:
                usage = result.delta.usage

        if not usage:
            usage = LLMUsage.empty_usage()

        return full_text, usage

    def create_chat_prompt(
        self,
        query: str,
        tools: Sequence[PromptMessageTool],
        prefix: str = PREFIX,
        suffix: str = SUFFIX,
        format_instructions: str = FORMAT_INSTRUCTIONS,
    ) -> list[ChatModelMessage]:
        tool_strings = []
        for tool in tools:
            tool_strings.append(
                f"{tool.name}: {tool.description}, args: {{'query': {{'title': 'Query',"
                f" 'description': 'Query for the dataset to be used to retrieve the dataset.', 'type': 'string'}}}}"
            )
        formatted_tools = "\n".join(tool_strings)
        unique_tool_names = {tool.name for tool in tools}
        tool_names = ", ".join('"' + name + '"' for name in unique_tool_names)
        format_instructions = format_instructions.format(tool_names=tool_names)
        template = "\n\n".join([prefix, formatted_tools, format_instructions, suffix])
        prompt_messages = []
        system_prompt_messages = ChatModelMessage(role=PromptMessageRole.SYSTEM, text=template)
        prompt_messages.append(system_prompt_messages)
        user_prompt_message = ChatModelMessage(role=PromptMessageRole.USER, text=query)
        prompt_messages.append(user_prompt_message)
        return prompt_messages

    def create_completion_prompt(
        self,
        tools: Sequence[PromptMessageTool],
        prefix: str = PREFIX,
        format_instructions: str = FORMAT_INSTRUCTIONS,
    ) -> CompletionModelPromptTemplate:
        """Create prompt in the style of the zero shot agent.

        Args:
            tools: List of tools the agent will have access to, used to format the
                prompt.
            prefix: String to put before the list of tools.
            format_instructions: The format instruction prompt.
        Returns:
            A PromptTemplate with the template assembled from the pieces here.
        """
        suffix = """Begin! Reminder to ALWAYS respond with a valid json blob of a single action. Use tools if necessary. Respond directly if appropriate. Format is Action:```$JSON_BLOB```then Observation:.
Question: {input}
Thought: {agent_scratchpad}
"""  # noqa: E501

        tool_strings = "\n".join([f"{tool.name}: {tool.description}" for tool in tools])
        tool_names = ", ".join([tool.name for tool in tools])
        format_instructions = format_instructions.format(tool_names=tool_names)
        template = "\n\n".join([prefix, tool_strings, format_instructions, suffix])
        return CompletionModelPromptTemplate(text=template)

```

### api/core/rag/docstore/__init__.py
```py

```

### api/core/rag/docstore/dataset_docstore.py
```py
from __future__ import annotations

from collections.abc import Sequence
from typing import Any

from graphon.model_runtime.entities.model_entities import ModelType
from sqlalchemy import delete, func, select

from core.model_manager import ModelManager
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.models.document import AttachmentDocument, Document
from extensions.ext_database import db
from models.dataset import ChildChunk, Dataset, DocumentSegment, SegmentAttachmentBinding


class DatasetDocumentStore:
    def __init__(
        self,
        dataset: Dataset,
        user_id: str,
        document_id: str | None = None,
    ):
        self._dataset = dataset
        self._user_id = user_id
        self._document_id = document_id

    @classmethod
    def from_dict(cls, config_dict: dict[str, Any]) -> DatasetDocumentStore:
        return cls(**config_dict)

    def to_dict(self) -> dict[str, Any]:
        """Serialize to dict."""
        return {
            "dataset_id": self._dataset.id,
        }

    @property
    def dataset_id(self):
        return self._dataset.id

    @property
    def user_id(self):
        return self._user_id

    @property
    def docs(self) -> dict[str, Document]:
        stmt = select(DocumentSegment).where(DocumentSegment.dataset_id == self._dataset.id)
        document_segments = db.session.scalars(stmt).all()

        output = {}
        for document_segment in document_segments:
            doc_id = document_segment.index_node_id
            output[doc_id] = Document(
                page_content=document_segment.content,
                metadata={
                    "doc_id": document_segment.index_node_id,
                    "doc_hash": document_segment.index_node_hash,
                    "document_id": document_segment.document_id,
                    "dataset_id": document_segment.dataset_id,
                },
            )

        return output

    def add_documents(self, docs: Sequence[Document], allow_update: bool = True, save_child: bool = False):
        max_position = db.session.scalar(
            select(func.max(DocumentSegment.position)).where(DocumentSegment.document_id == self._document_id)
        )

        if max_position is None:
            max_position = 0
        embedding_model = None
        if self._dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
            model_manager = ModelManager.for_tenant(tenant_id=self._dataset.tenant_id)
            embedding_model = model_manager.get_model_instance(
                tenant_id=self._dataset.tenant_id,
                provider=self._dataset.embedding_model_provider,
                model_type=ModelType.TEXT_EMBEDDING,
                model=self._dataset.embedding_model,
            )

        if embedding_model:
            page_content_list = [doc.page_content for doc in docs]
            tokens_list = embedding_model.get_text_embedding_num_tokens(page_content_list)
        else:
            tokens_list = [0] * len(docs)

        for doc, tokens in zip(docs, tokens_list):
            if not isinstance(doc, Document):
                raise ValueError("doc must be a Document")

            if doc.metadata is None:
                raise ValueError("doc.metadata must be a dict")

            segment_document = self.get_document_segment(doc_id=doc.metadata["doc_id"])

            # NOTE: doc could already exist in the store, but we overwrite it
            if not allow_update and segment_document:
                raise ValueError(
                    f"doc_id {doc.metadata['doc_id']} already exists. Set allow_update to True to overwrite."
                )

            if not segment_document:
                max_position += 1

                segment_document = DocumentSegment(
                    tenant_id=self._dataset.tenant_id,
                    dataset_id=self._dataset.id,
                    document_id=self._document_id,
                    index_node_id=doc.metadata["doc_id"],
                    index_node_hash=doc.metadata["doc_hash"],
                    position=max_position,
                    content=doc.page_content,
                    word_count=len(doc.page_content),
                    tokens=tokens,
                    enabled=False,
                    created_by=self._user_id,
                )
                if doc.metadata.get("answer"):
                    segment_document.answer = doc.metadata.pop("answer", "")

                db.session.add(segment_document)
                db.session.flush()
                self.add_multimodel_documents_binding(
                    segment_id=segment_document.id, multimodel_documents=doc.attachments
                )
                if save_child:
                    if doc.children:
                        for position, child in enumerate(doc.children, start=1):
                            child_segment = ChildChunk(
                                tenant_id=self._dataset.tenant_id,
                                dataset_id=self._dataset.id,
                                document_id=self._document_id,
                                segment_id=segment_document.id,
                                position=position,
                                index_node_id=child.metadata.get("doc_id"),
                                index_node_hash=child.metadata.get("doc_hash"),
                                content=child.page_content,
                                word_count=len(child.page_content),
                                type="automatic",
                                created_by=self._user_id,
                            )
                            db.session.add(child_segment)
            else:
                segment_document.content = doc.page_content
                if doc.metadata.get("answer"):
                    segment_document.answer = doc.metadata.pop("answer", "")
                segment_document.index_node_hash = doc.metadata.get("doc_hash")
                segment_document.word_count = len(doc.page_content)
                segment_document.tokens = tokens
                self.add_multimodel_documents_binding(
                    segment_id=segment_document.id, multimodel_documents=doc.attachments
                )
                if save_child and doc.children:
                    # delete the existing child chunks
                    db.session.execute(
                        delete(ChildChunk).where(
                            ChildChunk.tenant_id == self._dataset.tenant_id,
                            ChildChunk.dataset_id == self._dataset.id,
                            ChildChunk.document_id == self._document_id,
                            ChildChunk.segment_id == segment_document.id,
                        )
                    )
                    # add new child chunks
                    for position, child in enumerate(doc.children, start=1):
                        child_segment = ChildChunk(
                            tenant_id=self._dataset.tenant_id,
                            dataset_id=self._dataset.id,
                            document_id=self._document_id,
                            segment_id=segment_document.id,
                            position=position,
                            index_node_id=child.metadata.get("doc_id"),
                            index_node_hash=child.metadata.get("doc_hash"),
                            content=child.page_content,
                            word_count=len(child.page_content),
                            type="automatic",
                            created_by=self._user_id,
                        )
                        db.session.add(child_segment)

            db.session.commit()

    def document_exists(self, doc_id: str) -> bool:
        """Check if document exists."""
        result = self.get_document_segment(doc_id)
        return result is not None

    def get_document(self, doc_id: str, raise_error: bool = True) -> Document | None:
        document_segment = self.get_document_segment(doc_id)

        if document_segment is None:
            if raise_error:
                raise ValueError(f"doc_id {doc_id} not found.")
            else:
                return None

        return Document(
            page_content=document_segment.content,
            metadata={
                "doc_id": document_segment.index_node_id,
                "doc_hash": document_segment.index_node_hash,
                "document_id": document_segment.document_id,
                "dataset_id": document_segment.dataset_id,
            },
        )

    def delete_document(self, doc_id: str, raise_error: bool = True):
        document_segment = self.get_document_segment(doc_id)

        if document_segment is None:
            if raise_error:
                raise ValueError(f"doc_id {doc_id} not found.")
            else:
                return None

        db.session.delete(document_segment)
        db.session.commit()

    def set_document_hash(self, doc_id: str, doc_hash: str):
        """Set the hash for a given doc_id."""
        document_segment = self.get_document_segment(doc_id)

        if document_segment is None:
            return None

        document_segment.index_node_hash = doc_hash
        db.session.commit()

    def get_document_hash(self, doc_id: str) -> str | None:
        """Get the stored hash for a document, if it exists."""
        document_segment = self.get_document_segment(doc_id)

        if document_segment is None:
            return None
        data: str | None = document_segment.index_node_hash
        return data

    def get_document_segment(self, doc_id: str) -> DocumentSegment | None:
        stmt = select(DocumentSegment).where(
            DocumentSegment.dataset_id == self._dataset.id, DocumentSegment.index_node_id == doc_id
        )
        document_segment = db.session.scalar(stmt)

        return document_segment

    def add_multimodel_documents_binding(self, segment_id: str, multimodel_documents: list[AttachmentDocument] | None):
        if multimodel_documents:
            for multimodel_document in multimodel_documents:
                binding = SegmentAttachmentBinding(
                    tenant_id=self._dataset.tenant_id,
                    dataset_id=self._dataset.id,
                    document_id=self._document_id,
                    segment_id=segment_id,
                    attachment_id=multimodel_document.metadata["doc_id"],
                )
                db.session.add(binding)

```

### api/core/rag/rerank/weight_rerank.py
```py
import math
from collections import Counter

import numpy as np
from graphon.model_runtime.entities.model_entities import ModelType

from core.model_manager import ModelManager
from core.rag.datasource.keyword.jieba.jieba_keyword_table_handler import JiebaKeywordTableHandler
from core.rag.embedding.cached_embedding import CacheEmbedding
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.query_type import QueryType
from core.rag.models.document import Document
from core.rag.rerank.entity.weight import VectorSetting, Weights
from core.rag.rerank.rerank_base import BaseRerankRunner


class WeightRerankRunner(BaseRerankRunner):
    def __init__(self, tenant_id: str, weights: Weights):
        self.tenant_id = tenant_id
        self.weights = weights

    def run(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ) -> list[Document]:
        """
        Run rerank model
        :param query: search query
        :param documents: documents for reranking
        :param score_threshold: score threshold
        :param top_n: top n

        :return:
        """
        unique_documents = []
        doc_ids = set()
        for document in documents:
            if (
                document.provider == "dify"
                and document.metadata is not None
                and document.metadata["doc_id"] not in doc_ids
            ):
                # weight rerank only support text documents
                if not document.metadata.get("doc_type") or document.metadata.get("doc_type") == DocType.TEXT:
                    doc_ids.add(document.metadata["doc_id"])
                    unique_documents.append(document)
            else:
                if document not in unique_documents:
                    unique_documents.append(document)

        documents = unique_documents

        query_scores = self._calculate_keyword_score(query, documents)
        query_vector_scores = self._calculate_cosine(self.tenant_id, query, documents, self.weights.vector_setting)

        rerank_documents = []
        for document, query_score, query_vector_score in zip(documents, query_scores, query_vector_scores):
            score = (
                self.weights.vector_setting.vector_weight * query_vector_score
                + self.weights.keyword_setting.keyword_weight * query_score
            )
            if score_threshold and score < score_threshold:
                continue
            if document.metadata is not None:
                document.metadata["score"] = score
                rerank_documents.append(document)

        rerank_documents.sort(key=lambda x: x.metadata["score"] if x.metadata else 0, reverse=True)
        return rerank_documents[:top_n] if top_n else rerank_documents

    def _calculate_keyword_score(self, query: str, documents: list[Document]) -> list[float]:
        """
        Calculate BM25 scores
        :param query: search query
        :param documents: documents for reranking

        :return:
        """
        keyword_table_handler = JiebaKeywordTableHandler()
        query_keywords = keyword_table_handler.extract_keywords(query, None)
        documents_keywords = []
        for document in documents:
            # get the document keywords
            document_keywords = keyword_table_handler.extract_keywords(document.page_content, None)
            if document.metadata is not None:
                document.metadata["keywords"] = document_keywords
                documents_keywords.append(document_keywords)

        # Counter query keywords(TF)
        query_keyword_counts = Counter(query_keywords)

        # total documents
        total_documents = len(documents)

        # calculate all documents' keywords IDF
        all_keywords = set()
        for document_keywords in documents_keywords:
            all_keywords.update(document_keywords)

        keyword_idf = {}
        for keyword in all_keywords:
            # calculate include query keywords' documents
            doc_count_containing_keyword = sum(1 for doc_keywords in documents_keywords if keyword in doc_keywords)
            # IDF
            keyword_idf[keyword] = math.log((1 + total_documents) / (1 + doc_count_containing_keyword)) + 1

        query_tfidf = {}

        for keyword, count in query_keyword_counts.items():
            tf = count
            idf = keyword_idf.get(keyword, 0)
            query_tfidf[keyword] = tf * idf

        # calculate all documents' TF-IDF
        documents_tfidf = []
        for document_keywords in documents_keywords:
            document_keyword_counts = Counter(document_keywords)
            document_tfidf = {}
            for keyword, count in document_keyword_counts.items():
                tf = count
                idf = keyword_idf.get(keyword, 0)
                document_tfidf[keyword] = tf * idf
            documents_tfidf.append(document_tfidf)

        def cosine_similarity(vec1, vec2):
            intersection = set(vec1.keys()) & set(vec2.keys())
            numerator = sum(vec1[x] * vec2[x] for x in intersection)

            sum1 = sum(vec1[x] ** 2 for x in vec1)
            sum2 = sum(vec2[x] ** 2 for x in vec2)
            denominator = math.sqrt(sum1) * math.sqrt(sum2)

            if not denominator:
                return 0.0
            else:
                return float(numerator) / denominator

        similarities = []
        for document_tfidf in documents_tfidf:
            similarity = cosine_similarity(query_tfidf, document_tfidf)
            similarities.append(similarity)

        # for idx, similarity in enumerate(similarities):
        #     print(f"Document {idx + 1} similarity: {similarity}")

        return similarities

    def _calculate_cosine(
        self, tenant_id: str, query: str, documents: list[Document], vector_setting: VectorSetting
    ) -> list[float]:
        """
        Calculate Cosine scores
        :param query: search query
        :param documents: documents for reranking

        :return:
        """
        query_vector_scores = []

        model_manager = ModelManager.for_tenant(tenant_id=tenant_id)

        embedding_model = model_manager.get_model_instance(
            tenant_id=tenant_id,
            provider=vector_setting.embedding_provider_name,
            model_type=ModelType.TEXT_EMBEDDING,
            model=vector_setting.embedding_model_name,
        )
        cache_embedding = CacheEmbedding(embedding_model)
        query_vector = cache_embedding.embed_query(query)
        for document in documents:
            # calculate cosine similarity
            if document.metadata and "score" in document.metadata:
                query_vector_scores.append(document.metadata["score"])
            else:
                # transform to NumPy
                vec1 = np.array(query_vector)
                vec2 = np.array(document.vector)

                # calculate dot product
                dot_product = np.dot(vec1, vec2)

                # calculate norm
                norm_vec1 = np.linalg.norm(vec1)
                norm_vec2 = np.linalg.norm(vec2)

                # calculate cosine similarity
                cosine_sim = dot_product / (norm_vec1 * norm_vec2)
                query_vector_scores.append(cosine_sim)

        return query_vector_scores

```

### api/core/rag/rerank/rerank_base.py
```py
from abc import ABC, abstractmethod

from core.rag.index_processor.constant.query_type import QueryType
from core.rag.models.document import Document


class BaseRerankRunner(ABC):
    @abstractmethod
    def run(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ) -> list[Document]:
        """
        Run rerank model
        :param query: search query
        :param documents: documents for reranking
        :param score_threshold: score threshold
        :param top_n: top n
        :return:
        """
        raise NotImplementedError

```

### api/core/rag/rerank/entity/weight.py
```py
from pydantic import BaseModel


class VectorSetting(BaseModel):
    vector_weight: float

    embedding_provider_name: str

    embedding_model_name: str


class KeywordSetting(BaseModel):
    keyword_weight: float


class Weights(BaseModel):
    """Model for weighted rerank."""

    vector_setting: VectorSetting

    keyword_setting: KeywordSetting

```

### api/core/rag/rerank/rerank_factory.py
```py
from core.rag.rerank.rerank_base import BaseRerankRunner
from core.rag.rerank.rerank_model import RerankModelRunner
from core.rag.rerank.rerank_type import RerankMode
from core.rag.rerank.weight_rerank import WeightRerankRunner


class RerankRunnerFactory:
    @staticmethod
    def create_rerank_runner(runner_type: str, *args, **kwargs) -> BaseRerankRunner:
        match runner_type:
            case RerankMode.RERANKING_MODEL:
                return RerankModelRunner(*args, **kwargs)
            case RerankMode.WEIGHTED_SCORE:
                return WeightRerankRunner(*args, **kwargs)
            case _:
                raise ValueError(f"Unknown runner type: {runner_type}")

```

### api/core/rag/rerank/__init__.py
```py

```

### api/core/rag/rerank/rerank_type.py
```py
from enum import StrEnum


class RerankMode(StrEnum):
    RERANKING_MODEL = "reranking_model"
    WEIGHTED_SCORE = "weighted_score"

```

### api/core/rag/rerank/rerank_model.py
```py
import base64

from graphon.model_runtime.entities.model_entities import ModelType
from graphon.model_runtime.entities.rerank_entities import RerankResult

from core.model_manager import ModelInstance, ModelManager
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.query_type import QueryType
from core.rag.models.document import Document
from core.rag.rerank.rerank_base import BaseRerankRunner
from extensions.ext_database import db
from extensions.ext_storage import storage
from models.model import UploadFile


class RerankModelRunner(BaseRerankRunner):
    def __init__(self, rerank_model_instance: ModelInstance):
        self.rerank_model_instance = rerank_model_instance

    def run(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ) -> list[Document]:
        """
        Run rerank model
        :param query: search query
        :param documents: documents for reranking
        :param score_threshold: score threshold
        :param top_n: top n
        :return:
        """
        model_manager = ModelManager.for_tenant(
            tenant_id=self.rerank_model_instance.provider_model_bundle.configuration.tenant_id
        )
        is_support_vision = model_manager.check_model_support_vision(
            tenant_id=self.rerank_model_instance.provider_model_bundle.configuration.tenant_id,
            provider=self.rerank_model_instance.provider,
            model=self.rerank_model_instance.model_name,
            model_type=ModelType.RERANK,
        )
        if not is_support_vision:
            if query_type == QueryType.TEXT_QUERY:
                rerank_result, unique_documents = self.fetch_text_rerank(query, documents, score_threshold, top_n)
            else:
                return documents
        else:
            rerank_result, unique_documents = self.fetch_multimodal_rerank(
                query, documents, score_threshold, top_n, query_type
            )

        rerank_documents = []
        for result in rerank_result.docs:
            if score_threshold is None or result.score >= score_threshold:
                # format document
                rerank_document = Document(
                    page_content=result.text,
                    metadata=unique_documents[result.index].metadata,
                    provider=unique_documents[result.index].provider,
                )
                if rerank_document.metadata is not None:
                    rerank_document.metadata["score"] = result.score
                    rerank_documents.append(rerank_document)

        rerank_documents.sort(key=lambda x: x.metadata.get("score", 0.0), reverse=True)
        return rerank_documents[:top_n] if top_n else rerank_documents

    def fetch_text_rerank(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
    ) -> tuple[RerankResult, list[Document]]:
        """
        Fetch text rerank
        :param query: search query
        :param documents: documents for reranking
        :param score_threshold: score threshold
        :param top_n: top n
        :return:
        """
        docs = []
        doc_ids = set()
        unique_documents = []
        for document in documents:
            if (
                document.provider == "dify"
                and document.metadata is not None
                and document.metadata["doc_id"] not in doc_ids
            ):
                if not document.metadata.get("doc_type") or document.metadata.get("doc_type") == DocType.TEXT:
                    doc_ids.add(document.metadata["doc_id"])
                    docs.append(document.page_content)
                    unique_documents.append(document)
            elif document.provider == "external":
                if document not in unique_documents:
                    docs.append(document.page_content)
                    unique_documents.append(document)

        rerank_result = self.rerank_model_instance.invoke_rerank(
            query=query, docs=docs, score_threshold=score_threshold, top_n=top_n
        )
        return rerank_result, unique_documents

    def fetch_multimodal_rerank(
        self,
        query: str,
        documents: list[Document],
        score_threshold: float | None = None,
        top_n: int | None = None,
        query_type: QueryType = QueryType.TEXT_QUERY,
    ) -> tuple[RerankResult, list[Document]]:
        """
        Fetch multimodal rerank
        :param query: search query
        :param documents: documents for reranking
        :param score_threshold: score threshold
        :param top_n: top n
        :param query_type: query type
        :return: rerank result
        """
        docs = []
        doc_ids = set()
        unique_documents = []
        for document in documents:
            if (
                document.provider == "dify"
                and document.metadata is not None
                and document.metadata["doc_id"] not in doc_ids
            ):
                if document.metadata.get("doc_type") == DocType.IMAGE:
                    # Query file info within db.session context to ensure thread-safe access
                    upload_file = db.session.get(UploadFile, document.metadata["doc_id"])
                    if upload_file:
                        blob = storage.load_once(upload_file.key)
                        document_file_base64 = base64.b64encode(blob).decode()
                        document_file_dict = {
                            "content": document_file_base64,
                            "content_type": document.metadata["doc_type"],
                        }
                        docs.append(document_file_dict)
                else:
                    document_text_dict = {
                        "content": document.page_content,
                        "content_type": document.metadata.get("doc_type") or DocType.TEXT,
                    }
                    docs.append(document_text_dict)
                doc_ids.add(document.metadata["doc_id"])
                unique_documents.append(document)
            elif document.provider == "external":
                if document not in unique_documents:
                    docs.append(
                        {
                            "content": document.page_content,
                            "content_type": document.metadata.get("doc_type") or DocType.TEXT,
                        }
                    )
                    unique_documents.append(document)

        documents = unique_documents
        if query_type == QueryType.TEXT_QUERY:
            rerank_result, unique_documents = self.fetch_text_rerank(query, documents, score_threshold, top_n)
            return rerank_result, unique_documents
        elif query_type == QueryType.IMAGE_QUERY:
            # Query file info within db.session context to ensure thread-safe access
            upload_file = db.session.get(UploadFile, query)
            if upload_file:
                blob = storage.load_once(upload_file.key)
                file_query = base64.b64encode(blob).decode()
                file_query_dict = {
                    "content": file_query,
                    "content_type": DocType.IMAGE,
                }
                rerank_result = self.rerank_model_instance.invoke_multimodal_rerank(
                    query=file_query_dict, docs=docs, score_threshold=score_threshold, top_n=top_n
                )
                return rerank_result, unique_documents
            else:
                raise ValueError(f"Upload file not found for query: {query}")

        else:
            raise ValueError(f"Query type {query_type} is not supported")

```

### api/core/rag/cleaner/cleaner_base.py
```py
"""Abstract interface for document cleaner implementations."""

from abc import ABC, abstractmethod


class BaseCleaner(ABC):
    """Interface for clean chunk content."""

    @abstractmethod
    def clean(self, content: str):
        raise NotImplementedError

```

### api/core/rag/cleaner/clean_processor.py
```py
import re
from typing import Any


class CleanProcessor:
    @classmethod
    def clean(cls, text: str, process_rule: dict[str, Any] | None) -> str:
        # default clean
        # remove invalid symbol
        text = re.sub(r"<\|", "<", text)
        text = re.sub(r"\|>", ">", text)
        text = re.sub(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F\xEF\xBF\xBE]", "", text)
        # Unicode  U+FFFE
        text = re.sub("\ufffe", "", text)

        rules = process_rule["rules"] if process_rule else {}
        if "pre_processing_rules" in rules:
            pre_processing_rules = rules["pre_processing_rules"]
            for pre_processing_rule in pre_processing_rules:
                if pre_processing_rule["id"] == "remove_extra_spaces" and pre_processing_rule["enabled"] is True:
                    # Remove extra spaces
                    pattern = r"\n{3,}"
                    text = re.sub(pattern, "\n\n", text)
                    pattern = r"[\t\f\r\x20\u00a0\u1680\u180e\u2000-\u200a\u202f\u205f\u3000]{2,}"
                    text = re.sub(pattern, " ", text)
                elif pre_processing_rule["id"] == "remove_urls_emails" and pre_processing_rule["enabled"] is True:
                    # Remove email
                    pattern = r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)"
                    text = re.sub(pattern, "", text)

                    # Remove URL but keep Markdown image URLs and link URLs
                    # Replace the ENTIRE markdown link/image with a single placeholder to protect
                    # the link text (which might also be a URL) from being removed
                    markdown_link_pattern = r"\[([^\]]*)\]\((https?://[^)]+)\)"
                    markdown_image_pattern = r"!\[.*?\]\((https?://[^)]+)\)"
                    placeholders: list[tuple[str, str, str]] = []  # (type, text, url)

                    def replace_markdown_with_placeholder(match, placeholders=placeholders):
                        link_type = "link"
                        link_text = match.group(1)
                        url = match.group(2)
                        placeholder = f"__MARKDOWN_PLACEHOLDER_{len(placeholders)}__"
                        placeholders.append((link_type, link_text, url))
                        return placeholder

                    def replace_image_with_placeholder(match, placeholders=placeholders):
                        link_type = "image"
                        url = match.group(1)
                        placeholder = f"__MARKDOWN_PLACEHOLDER_{len(placeholders)}__"
                        placeholders.append((link_type, "image", url))
                        return placeholder

                    # Protect markdown links first
                    text = re.sub(markdown_link_pattern, replace_markdown_with_placeholder, text)
                    # Then protect markdown images
                    text = re.sub(markdown_image_pattern, replace_image_with_placeholder, text)

                    # Now remove all remaining URLs
                    url_pattern = r"https?://\S+"
                    text = re.sub(url_pattern, "", text)

                    # Restore the Markdown links and images
                    for i, (link_type, text_or_alt, url) in enumerate(placeholders):
                        placeholder = f"__MARKDOWN_PLACEHOLDER_{i}__"
                        if link_type == "link":
                            text = text.replace(placeholder, f"[{text_or_alt}]({url})")
                        else:  # image
                            text = text.replace(placeholder, f"![{text_or_alt}]({url})")
        return text

    def filter_string(self, text):
        return text

```

### api/core/rag/entities/event.py
```py
from collections.abc import Mapping
from enum import StrEnum
from typing import Any

from pydantic import BaseModel, Field


class DatasourceStreamEvent(StrEnum):
    """
    Datasource Stream event
    """

    PROCESSING = "datasource_processing"
    COMPLETED = "datasource_completed"
    ERROR = "datasource_error"


class BaseDatasourceEvent(BaseModel):
    pass


class DatasourceErrorEvent(BaseDatasourceEvent):
    event: DatasourceStreamEvent = DatasourceStreamEvent.ERROR
    error: str = Field(..., description="error message")


class DatasourceCompletedEvent(BaseDatasourceEvent):
    event: DatasourceStreamEvent = DatasourceStreamEvent.COMPLETED
    data: Mapping[str, Any] | list = Field(..., description="result")
    total: int | None = Field(default=0, description="total")
    completed: int | None = Field(default=0, description="completed")
    time_consuming: float | None = Field(default=0.0, description="time consuming")


class DatasourceProcessingEvent(BaseDatasourceEvent):
    event: DatasourceStreamEvent = DatasourceStreamEvent.PROCESSING
    total: int | None = Field(..., description="total")
    completed: int | None = Field(..., description="completed")

```

### api/core/rag/entities/citation_metadata.py
```py
from typing import Any

from pydantic import BaseModel


class RetrievalSourceMetadata(BaseModel):
    position: int | None = None
    dataset_id: str | None = None
    dataset_name: str | None = None
    document_id: str | None = None
    document_name: str | None = None
    data_source_type: str | None = None
    segment_id: str | None = None
    retriever_from: str | None = None
    score: float | None = None
    hit_count: int | None = None
    word_count: int | None = None
    segment_position: int | None = None
    index_node_hash: str | None = None
    content: str | None = None
    page: int | None = None
    doc_metadata: dict[str, Any] | None = None
    title: str | None = None
    files: list[dict[str, Any]] | None = None
    summary: str | None = None

```

### api/core/rag/entities/context_entities.py
```py
from pydantic import BaseModel


class DocumentContext(BaseModel):
    """
    Model class for document context.
    """

    content: str
    score: float | None = None

```

### api/core/rag/entities/metadata_entities.py
```py
from collections.abc import Sequence
from typing import Literal

from pydantic import BaseModel, Field

SupportedComparisonOperator = Literal[
    # for string or array
    "contains",
    "not contains",
    "start with",
    "end with",
    "is",
    "is not",
    "empty",
    "not empty",
    "in",
    "not in",
    # for number
    "=",
    "≠",
    ">",
    "<",
    "≥",
    "≤",
    # for time
    "before",
    "after",
]


class Condition(BaseModel):
    """
    Condition detail
    """

    name: str
    comparison_operator: SupportedComparisonOperator
    value: str | Sequence[str] | None | int | float = None


class MetadataCondition(BaseModel):
    """
    Metadata Condition.
    """

    logical_operator: Literal["and", "or"] | None = "and"
    conditions: list[Condition] | None = Field(default=None, deprecated=True)

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-013.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
