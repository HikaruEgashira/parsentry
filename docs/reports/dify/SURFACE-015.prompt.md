You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-015
- **Kind**: iac_resource
- **Identifier**: Docker Compose Infrastructure (SSRF Proxy, Nginx, Database)
- **Description**: Infrastructure configuration including SSRF proxy (Squid/similar), Nginx reverse proxy, and database services. Risk of SSRF proxy bypass, misconfigured network policies, exposed database ports, and secrets in environment files.
- **Locations**: docker/docker-compose.yaml, docker/docker-compose.middleware.yaml, docker/nginx/conf.d/, docker/ssrf_proxy/, docker/middleware.env.example

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

### docker/docker-compose.middleware.yaml
```yaml
services:
  # The postgres database.
  db_postgres:
    image: postgres:15-alpine
    profiles:
      - ""
      - postgresql
    restart: always
    env_file:
      - ./middleware.env
    environment:
      POSTGRES_PASSWORD: ${DB_PASSWORD:-difyai123456}
      POSTGRES_DB: ${DB_DATABASE:-dify}
      PGDATA: ${PGDATA:-/var/lib/postgresql/data/pgdata}
    command: >
      postgres -c 'max_connections=${POSTGRES_MAX_CONNECTIONS:-100}'
               -c 'shared_buffers=${POSTGRES_SHARED_BUFFERS:-128MB}'
               -c 'work_mem=${POSTGRES_WORK_MEM:-4MB}'
               -c 'maintenance_work_mem=${POSTGRES_MAINTENANCE_WORK_MEM:-64MB}'
               -c 'effective_cache_size=${POSTGRES_EFFECTIVE_CACHE_SIZE:-4096MB}'
               -c 'statement_timeout=${POSTGRES_STATEMENT_TIMEOUT:-0}'
               -c 'idle_in_transaction_session_timeout=${POSTGRES_IDLE_IN_TRANSACTION_SESSION_TIMEOUT:-0}'
    volumes:
      - ${PGDATA_HOST_VOLUME:-./volumes/db/data}:/var/lib/postgresql/data
    ports:
      - "${EXPOSE_POSTGRES_PORT:-5432}:5432"
    healthcheck:
      test:
        [
          "CMD",
          "pg_isready",
          "-h",
          "db_postgres",
          "-U",
          "${DB_USERNAME:-postgres}",
          "-d",
          "${DB_DATABASE:-dify}",
        ]
      interval: 1s
      timeout: 3s
      retries: 30

  db_mysql:
    image: mysql:8.0
    profiles:
      - mysql
    restart: always
    env_file:
      - ./middleware.env
    environment:
      MYSQL_ROOT_PASSWORD: ${DB_PASSWORD:-difyai123456}
      MYSQL_DATABASE: ${DB_DATABASE:-dify}
    command: >
      --max_connections=1000
      --innodb_buffer_pool_size=${MYSQL_INNODB_BUFFER_POOL_SIZE:-512M}
      --innodb_log_file_size=${MYSQL_INNODB_LOG_FILE_SIZE:-128M}
      --innodb_flush_log_at_trx_commit=${MYSQL_INNODB_FLUSH_LOG_AT_TRX_COMMIT:-2}
    volumes:
      - ${MYSQL_HOST_VOLUME:-./volumes/mysql/data}:/var/lib/mysql
    ports:
      - "${EXPOSE_MYSQL_PORT:-3306}:3306"
    healthcheck:
      test:
        [
          "CMD",
          "mysqladmin",
          "ping",
          "-u",
          "root",
          "-p${DB_PASSWORD:-difyai123456}",
        ]
      interval: 1s
      timeout: 3s
      retries: 30

  # The redis cache.
  redis:
    image: redis:6-alpine
    restart: always
    env_file:
      - ./middleware.env
    environment:
      REDISCLI_AUTH: ${REDIS_PASSWORD:-difyai123456}
    volumes:
      # Mount the redis data directory to the container.
      - ${REDIS_HOST_VOLUME:-./volumes/redis/data}:/data
    # Set the redis password when startup redis server.
    command: redis-server --requirepass ${REDIS_PASSWORD:-difyai123456}
    ports:
      - "${EXPOSE_REDIS_PORT:-6379}:6379"
    healthcheck:
      test:
        [
          "CMD-SHELL",
          "redis-cli -a ${REDIS_PASSWORD:-difyai123456} ping | grep -q PONG",
        ]

  # The DifySandbox
  sandbox:
    image: langgenius/dify-sandbox:0.2.14
    restart: always
    env_file:
      - ./middleware.env
    environment:
      # The DifySandbox configurations
      # Make sure you are changing this key for your deployment with a strong key.
      # You can generate a strong key using `openssl rand -base64 42`.
      API_KEY: ${SANDBOX_API_KEY:-dify-sandbox}
      GIN_MODE: ${SANDBOX_GIN_MODE:-release}
      WORKER_TIMEOUT: ${SANDBOX_WORKER_TIMEOUT:-15}
      ENABLE_NETWORK: ${SANDBOX_ENABLE_NETWORK:-true}
      HTTP_PROXY: ${SANDBOX_HTTP_PROXY:-http://ssrf_proxy:3128}
      HTTPS_PROXY: ${SANDBOX_HTTPS_PROXY:-http://ssrf_proxy:3128}
      SANDBOX_PORT: ${SANDBOX_PORT:-8194}
      PIP_MIRROR_URL: ${PIP_MIRROR_URL:-}
    volumes:
      - ./volumes/sandbox/dependencies:/dependencies
      - ./volumes/sandbox/conf:/conf
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8194/health"]
    networks:
      - ssrf_proxy_network

  # plugin daemon
  plugin_daemon:
    image: langgenius/dify-plugin-daemon:0.5.3-local
    restart: always
    env_file:
      - ./middleware.env
    extra_hosts:
      - "host.docker.internal:host-gateway"
    environment:
      # Use the shared environment variables.
      LOG_OUTPUT_FORMAT: ${LOG_OUTPUT_FORMAT:-text}
      DB_DATABASE: ${DB_PLUGIN_DATABASE:-dify_plugin}
      REDIS_HOST: ${REDIS_HOST:-redis}
      REDIS_PORT: ${REDIS_PORT:-6379}
      REDIS_PASSWORD: ${REDIS_PASSWORD:-difyai123456}
      SERVER_PORT: ${PLUGIN_DAEMON_PORT:-5002}
      SERVER_KEY: ${PLUGIN_DAEMON_KEY:-lYkiYYT6owG+71oLerGzA7GXCgOT++6ovaezWAjpCjf+Sjc3ZtU+qUEi}
      MAX_PLUGIN_PACKAGE_SIZE: ${PLUGIN_MAX_PACKAGE_SIZE:-52428800}
      PPROF_ENABLED: ${PLUGIN_PPROF_ENABLED:-false}
      DIFY_INNER_API_URL: ${PLUGIN_DIFY_INNER_API_URL:-http://host.docker.internal:5001}
      DIFY_INNER_API_KEY: ${PLUGIN_DIFY_INNER_API_KEY:-QaHbTe77CtuXmsfyhR7+vRjI/+XbV1AaFy691iy+kGDv2Jvy0/eAh8Y1}
      PLUGIN_REMOTE_INSTALLING_HOST: ${PLUGIN_DEBUGGING_HOST:-0.0.0.0}
      PLUGIN_REMOTE_INSTALLING_PORT: ${PLUGIN_DEBUGGING_PORT:-5003}
      PLUGIN_WORKING_PATH: ${PLUGIN_WORKING_PATH:-/app/storage/cwd}
      PYTHON_ENV_INIT_TIMEOUT: ${PLUGIN_PYTHON_ENV_INIT_TIMEOUT:-120}
      PLUGIN_MAX_EXECUTION_TIMEOUT: ${PLUGIN_MAX_EXECUTION_TIMEOUT:-600}
      PIP_MIRROR_URL: ${PIP_MIRROR_URL:-}
      PLUGIN_STORAGE_TYPE: ${PLUGIN_STORAGE_TYPE:-local}
      PLUGIN_STORAGE_LOCAL_ROOT: ${PLUGIN_STORAGE_LOCAL_ROOT:-/app/storage}
      PLUGIN_INSTALLED_PATH: ${PLUGIN_INSTALLED_PATH:-plugin}
      PLUGIN_PACKAGE_CACHE_PATH: ${PLUGIN_PACKAGE_CACHE_PATH:-plugin_packages}
      PLUGIN_MEDIA_CACHE_PATH: ${PLUGIN_MEDIA_CACHE_PATH:-assets}
      PLUGIN_STORAGE_OSS_BUCKET: ${PLUGIN_STORAGE_OSS_BUCKET:-}
      S3_USE_AWS: ${PLUGIN_S3_USE_AWS:-false}
      S3_USE_AWS_MANAGED_IAM: ${PLUGIN_S3_USE_AWS_MANAGED_IAM:-false}
      S3_ENDPOINT: ${PLUGIN_S3_ENDPOINT:-}
      S3_USE_PATH_STYLE: ${PLUGIN_S3_USE_PATH_STYLE:-false}
      AWS_ACCESS_KEY: ${PLUGIN_AWS_ACCESS_KEY:-}
      AWS_SECRET_KEY: ${PLUGIN_AWS_SECRET_KEY:-}
      AWS_REGION: ${PLUGIN_AWS_REGION:-}
      AZURE_BLOB_STORAGE_CONNECTION_STRING: ${PLUGIN_AZURE_BLOB_STORAGE_CONNECTION_STRING:-}
      AZURE_BLOB_STORAGE_CONTAINER_NAME: ${PLUGIN_AZURE_BLOB_STORAGE_CONTAINER_NAME:-}
      TENCENT_COS_SECRET_KEY: ${PLUGIN_TENCENT_COS_SECRET_KEY:-}
      TENCENT_COS_SECRET_ID: ${PLUGIN_TENCENT_COS_SECRET_ID:-}
      TENCENT_COS_REGION: ${PLUGIN_TENCENT_COS_REGION:-}
      ALIYUN_OSS_REGION: ${PLUGIN_ALIYUN_OSS_REGION:-}
      ALIYUN_OSS_ENDPOINT: ${PLUGIN_ALIYUN_OSS_ENDPOINT:-}
      ALIYUN_OSS_ACCESS_KEY_ID: ${PLUGIN_ALIYUN_OSS_ACCESS_KEY_ID:-}
      ALIYUN_OSS_ACCESS_KEY_SECRET: ${PLUGIN_ALIYUN_OSS_ACCESS_KEY_SECRET:-}
      ALIYUN_OSS_AUTH_VERSION: ${PLUGIN_ALIYUN_OSS_AUTH_VERSION:-v4}
      ALIYUN_OSS_PATH: ${PLUGIN_ALIYUN_OSS_PATH:-}
      VOLCENGINE_TOS_ENDPOINT: ${PLUGIN_VOLCENGINE_TOS_ENDPOINT:-}
      VOLCENGINE_TOS_ACCESS_KEY: ${PLUGIN_VOLCENGINE_TOS_ACCESS_KEY:-}
      VOLCENGINE_TOS_SECRET_KEY: ${PLUGIN_VOLCENGINE_TOS_SECRET_KEY:-}
      VOLCENGINE_TOS_REGION: ${PLUGIN_VOLCENGINE_TOS_REGION:-}
      THIRD_PARTY_SIGNATURE_VERIFICATION_ENABLED: true
      THIRD_PARTY_SIGNATURE_VERIFICATION_PUBLIC_KEYS: /app/keys/publickey.pem
      FORCE_VERIFYING_SIGNATURE: false
    ports:
      - "${EXPOSE_PLUGIN_DAEMON_PORT:-5002}:${PLUGIN_DAEMON_PORT:-5002}"
      - "${EXPOSE_PLUGIN_DEBUGGING_PORT:-5003}:${PLUGIN_DEBUGGING_PORT:-5003}"
    volumes:
      - ./volumes/plugin_daemon:/app/storage

  # ssrf_proxy server
  # for more information, please refer to
  # https://docs.dify.ai/learn-more/faq/install-faq#18-why-is-ssrf-proxy-needed%3F
  ssrf_proxy:
    image: ubuntu/squid:latest
    restart: always
    volumes:
      - ./ssrf_proxy/squid.conf.template:/etc/squid/squid.conf.template
      - ./ssrf_proxy/docker-entrypoint.sh:/docker-entrypoint-mount.sh
    entrypoint:
      [
        "sh",
        "-c",
        "cp /docker-entrypoint-mount.sh /docker-entrypoint.sh && sed -i 's/\r$$//' /docker-entrypoint.sh && chmod +x /docker-entrypoint.sh && /docker-entrypoint.sh",
      ]
    env_file:
      - ./middleware.env
    environment:
      # pls clearly modify the squid env vars to fit your network environment.
      HTTP_PORT: ${SSRF_HTTP_PORT:-3128}
      COREDUMP_DIR: ${SSRF_COREDUMP_DIR:-/var/spool/squid}
      REVERSE_PROXY_PORT: ${SSRF_REVERSE_PROXY_PORT:-8194}
      SANDBOX_HOST: ${SSRF_SANDBOX_HOST:-sandbox}
      SANDBOX_PORT: ${SANDBOX_PORT:-8194}
    ports:
      - "${EXPOSE_SSRF_PROXY_PORT:-3128}:${SSRF_HTTP_PORT:-3128}"
      - "${EXPOSE_SANDBOX_PORT:-8194}:${SANDBOX_PORT:-8194}"
    networks:
      - ssrf_proxy_network
      - default

  # The Weaviate vector store.
  weaviate:
    image: semitechnologies/weaviate:1.27.0
    profiles:
      - ""
      - weaviate
    restart: always
    volumes:
      # Mount the Weaviate data directory to the container.
      - ${WEAVIATE_HOST_VOLUME:-./volumes/weaviate}:/var/lib/weaviate
    env_file:
      - ./middleware.env
    environment:
      # The Weaviate configurations
      # You can refer to the [Weaviate](https://weaviate.io/developers/weaviate/config-refs/env-vars) documentation for more information.
      PERSISTENCE_DATA_PATH: ${WEAVIATE_PERSISTENCE_DATA_PATH:-/var/lib/weaviate}
      QUERY_DEFAULTS_LIMIT: ${WEAVIATE_QUERY_DEFAULTS_LIMIT:-25}
      AUTHENTICATION_ANONYMOUS_ACCESS_ENABLED: ${WEAVIATE_AUTHENTICATION_ANONYMOUS_ACCESS_ENABLED:-false}
      DEFAULT_VECTORIZER_MODULE: ${WEAVIATE_DEFAULT_VECTORIZER_MODULE:-none}
      CLUSTER_HOSTNAME: ${WEAVIATE_CLUSTER_HOSTNAME:-node1}
      AUTHENTICATION_APIKEY_ENABLED: ${WEAVIATE_AUTHENTICATION_APIKEY_ENABLED:-true}
      AUTHENTICATION_APIKEY_ALLOWED_KEYS: ${WEAVIATE_AUTHENTICATION_APIKEY_ALLOWED_KEYS:-WVF5YThaHlkYwhGUSmCRgsX3tD5ngdN8pkih}
      AUTHENTICATION_APIKEY_USERS: ${WEAVIATE_AUTHENTICATION_APIKEY_USERS:-hello@dify.ai}
      AUTHORIZATION_ADMINLIST_ENABLED: ${WEAVIATE_AUTHORIZATION_ADMINLIST_ENABLED:-true}
      AUTHORIZATION_ADMINLIST_USERS: ${WEAVIATE_AUTHORIZATION_ADMINLIST_USERS:-hello@dify.ai}
      DISABLE_TELEMETRY: ${WEAVIATE_DISABLE_TELEMETRY:-false}
    ports:
      - "${EXPOSE_WEAVIATE_PORT:-8080}:8080"
      - "${EXPOSE_WEAVIATE_GRPC_PORT:-50051}:50051"

networks:
  # create a network between sandbox, api and ssrf_proxy, and can not access outside.
  ssrf_proxy_network:
    driver: bridge
    internal: true

```

### docker/ssrf_proxy/docker-entrypoint.sh
```sh
#!/bin/bash

# Modified based on Squid OCI image entrypoint

# This entrypoint aims to forward the squid logs to stdout to assist users of
# common container related tooling (e.g., kubernetes, docker-compose, etc) to
# access the service logs.

# Moreover, it invokes the squid binary, leaving all the desired parameters to
# be provided by the "command" passed to the spawned container. If no command
# is provided by the user, the default behavior (as per the CMD statement in
# the Dockerfile) will be to use Ubuntu's default configuration [1] and run
# squid with the "-NYC" options to mimic the behavior of the Ubuntu provided
# systemd unit.

# [1] The default configuration is changed in the Dockerfile to allow local
# network connections. See the Dockerfile for further information.

echo "[ENTRYPOINT] re-create snakeoil self-signed certificate removed in the build process"
if [ ! -f /etc/ssl/private/ssl-cert-snakeoil.key ]; then
    /usr/sbin/make-ssl-cert generate-default-snakeoil --force-overwrite > /dev/null 2>&1
fi

tail -F /var/log/squid/access.log 2>/dev/null &
tail -F /var/log/squid/error.log 2>/dev/null &
tail -F /var/log/squid/store.log 2>/dev/null &
tail -F /var/log/squid/cache.log 2>/dev/null &

# Replace environment variables in the template and output to the squid.conf
echo "[ENTRYPOINT] replacing environment variables in the template"
awk '{
    while(match($0, /\${[A-Za-z_][A-Za-z_0-9]*}/)) {
        var = substr($0, RSTART+2, RLENGTH-3)
        val = ENVIRON[var]
        $0 = substr($0, 1, RSTART-1) val substr($0, RSTART+RLENGTH)
    }
    print
}' /etc/squid/squid.conf.template > /etc/squid/squid.conf

/usr/sbin/squid -Nz
echo "[ENTRYPOINT] starting squid"
/usr/sbin/squid -f /etc/squid/squid.conf -NYC 1

```

### docker/middleware.env.example
```example
# ------------------------------
# Environment Variables for db Service
# ------------------------------
# Database Configuration
# Database type, supported values are `postgresql` and `mysql`
DB_TYPE=postgresql
# For MySQL, only `root` user is supported for now
DB_USERNAME=postgres
DB_PASSWORD=difyai123456
DB_HOST=db_postgres
DB_PORT=5432
DB_DATABASE=dify

# PostgreSQL Configuration
# postgres data directory
PGDATA=/var/lib/postgresql/data/pgdata
PGDATA_HOST_VOLUME=./volumes/db/data

# Maximum number of connections to the database
# Default is 100
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-connection.html#GUC-MAX-CONNECTIONS
POSTGRES_MAX_CONNECTIONS=100

# Sets the amount of shared memory used for postgres's shared buffers.
# Default is 128MB
# Recommended value: 25% of available memory
# Reference: https://www.postgresql.org/docs/current/runtime-config-resource.html#GUC-SHARED-BUFFERS
POSTGRES_SHARED_BUFFERS=128MB

# Sets the amount of memory used by each database worker for working space.
# Default is 4MB
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-resource.html#GUC-WORK-MEM
POSTGRES_WORK_MEM=4MB

# Sets the amount of memory reserved for maintenance activities.
# Default is 64MB
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-resource.html#GUC-MAINTENANCE-WORK-MEM
POSTGRES_MAINTENANCE_WORK_MEM=64MB

# Sets the planner's assumption about the effective cache size.
# Default is 4096MB
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-query.html#GUC-EFFECTIVE-CACHE-SIZE
POSTGRES_EFFECTIVE_CACHE_SIZE=4096MB

# Sets the maximum allowed duration of any statement before termination.
# Default is 0 (no timeout).
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-STATEMENT-TIMEOUT
# A value of 0 prevents the server from timing out statements.
POSTGRES_STATEMENT_TIMEOUT=0

# Sets the maximum allowed duration of any idle in-transaction session before termination.
# Default is 0 (no timeout).
#
# Reference: https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-IDLE-IN-TRANSACTION-SESSION-TIMEOUT
# A value of 0 prevents the server from terminating idle sessions.
POSTGRES_IDLE_IN_TRANSACTION_SESSION_TIMEOUT=0

# MySQL Configuration
# MySQL data directory host volume
MYSQL_HOST_VOLUME=./volumes/mysql/data

# MySQL Performance Configuration
# Maximum number of connections to MySQL
# Default is 1000
MYSQL_MAX_CONNECTIONS=1000

# InnoDB buffer pool size
# Default is 512M
# Recommended value: 70-80% of available memory for dedicated MySQL server
# Reference: https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_size
MYSQL_INNODB_BUFFER_POOL_SIZE=512M

# InnoDB log file size
# Default is 128M
# Reference: https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_file_size
MYSQL_INNODB_LOG_FILE_SIZE=128M

# InnoDB flush log at transaction commit
# Default is 2 (flush to OS cache, sync every second)
# Options: 0 (no flush), 1 (flush and sync), 2 (flush to OS cache)
# Reference: https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_log_at_trx_commit
MYSQL_INNODB_FLUSH_LOG_AT_TRX_COMMIT=2

# -----------------------------
# Environment Variables for redis Service
# -----------------------------
REDIS_HOST_VOLUME=./volumes/redis/data
REDIS_PASSWORD=difyai123456
# Optional: limit total Redis connections used by API/Worker (unset for default)
# Align with API's REDIS_MAX_CONNECTIONS in configs
REDIS_MAX_CONNECTIONS=

# ------------------------------
# Environment Variables for sandbox Service
# ------------------------------
SANDBOX_API_KEY=dify-sandbox
SANDBOX_GIN_MODE=release
SANDBOX_WORKER_TIMEOUT=15
SANDBOX_ENABLE_NETWORK=true
SANDBOX_HTTP_PROXY=http://ssrf_proxy:3128
SANDBOX_HTTPS_PROXY=http://ssrf_proxy:3128
SANDBOX_PORT=8194

# ------------------------------
# Environment Variables for ssrf_proxy Service
# ------------------------------
SSRF_HTTP_PORT=3128
SSRF_COREDUMP_DIR=/var/spool/squid
SSRF_REVERSE_PROXY_PORT=8194
SSRF_SANDBOX_HOST=sandbox

# ------------------------------
# Environment Variables for weaviate Service
# ------------------------------
WEAVIATE_QUERY_DEFAULTS_LIMIT=25
WEAVIATE_AUTHENTICATION_ANONYMOUS_ACCESS_ENABLED=true
WEAVIATE_DEFAULT_VECTORIZER_MODULE=none
WEAVIATE_CLUSTER_HOSTNAME=node1
WEAVIATE_AUTHENTICATION_APIKEY_ENABLED=true
WEAVIATE_AUTHENTICATION_APIKEY_ALLOWED_KEYS=WVF5YThaHlkYwhGUSmCRgsX3tD5ngdN8pkih
WEAVIATE_AUTHENTICATION_APIKEY_USERS=hello@dify.ai
WEAVIATE_AUTHORIZATION_ADMINLIST_ENABLED=true
WEAVIATE_AUTHORIZATION_ADMINLIST_USERS=hello@dify.ai
WEAVIATE_DISABLE_TELEMETRY=false
WEAVIATE_HOST_VOLUME=./volumes/weaviate

# ------------------------------
# Docker Compose profile configuration
# ------------------------------
# Loaded automatically when running `docker compose --env-file middleware.env ...`.
# Controls which DB/vector services start, so no extra `--profile` flag is needed.
COMPOSE_PROFILES=${DB_TYPE:-postgresql},weaviate

# ------------------------------
# Docker Compose Service Expose Host Port Configurations
# ------------------------------
EXPOSE_POSTGRES_PORT=5432
EXPOSE_MYSQL_PORT=3306
EXPOSE_REDIS_PORT=6379
EXPOSE_SANDBOX_PORT=8194
EXPOSE_SSRF_PROXY_PORT=3128
EXPOSE_WEAVIATE_PORT=8080

# ------------------------------
# Plugin Daemon Configuration
# ------------------------------

DB_PLUGIN_DATABASE=dify_plugin
EXPOSE_PLUGIN_DAEMON_PORT=5002
PLUGIN_DAEMON_PORT=5002
PLUGIN_DAEMON_KEY=lYkiYYT6owG+71oLerGzA7GXCgOT++6ovaezWAjpCjf+Sjc3ZtU+qUEi
PLUGIN_DAEMON_URL=http://host.docker.internal:5002
PLUGIN_MAX_PACKAGE_SIZE=52428800
PLUGIN_PPROF_ENABLED=false
PLUGIN_WORKING_PATH=/app/storage/cwd

ENDPOINT_URL_TEMPLATE=http://localhost:5002/e/{hook_id}

PLUGIN_DEBUGGING_PORT=5003
PLUGIN_DEBUGGING_HOST=0.0.0.0
EXPOSE_PLUGIN_DEBUGGING_HOST=localhost
EXPOSE_PLUGIN_DEBUGGING_PORT=5003

PLUGIN_DIFY_INNER_API_KEY=QaHbTe77CtuXmsfyhR7+vRjI/+XbV1AaFy691iy+kGDv2Jvy0/eAh8Y1
PLUGIN_DIFY_INNER_API_URL=http://host.docker.internal:5001

MARKETPLACE_ENABLED=true
MARKETPLACE_API_URL=https://marketplace.dify.ai

FORCE_VERIFYING_SIGNATURE=true

PLUGIN_PYTHON_ENV_INIT_TIMEOUT=120
PLUGIN_MAX_EXECUTION_TIMEOUT=600
# PIP_MIRROR_URL=https://pypi.tuna.tsinghua.edu.cn/simple
PIP_MIRROR_URL=

# https://github.com/langgenius/dify-plugin-daemon/blob/main/.env.example
# Plugin storage type, local aws_s3 tencent_cos azure_blob
PLUGIN_STORAGE_TYPE=local
PLUGIN_STORAGE_LOCAL_ROOT=/app/storage
PLUGIN_WORKING_PATH=/app/storage/cwd
PLUGIN_INSTALLED_PATH=plugin
PLUGIN_PACKAGE_CACHE_PATH=plugin_packages
PLUGIN_MEDIA_CACHE_PATH=assets
# Plugin oss bucket
PLUGIN_STORAGE_OSS_BUCKET=
# Plugin oss s3 credentials
PLUGIN_S3_USE_AWS_MANAGED_IAM=false
PLUGIN_S3_USE_AWS=false
PLUGIN_S3_ENDPOINT=
PLUGIN_S3_USE_PATH_STYLE=false
PLUGIN_AWS_ACCESS_KEY=
PLUGIN_AWS_SECRET_KEY=
PLUGIN_AWS_REGION=
# Plugin oss azure blob
PLUGIN_AZURE_BLOB_STORAGE_CONTAINER_NAME=
PLUGIN_AZURE_BLOB_STORAGE_CONNECTION_STRING=
# Plugin oss tencent cos
PLUGIN_TENCENT_COS_SECRET_KEY=
PLUGIN_TENCENT_COS_SECRET_ID=
PLUGIN_TENCENT_COS_REGION=
# Plugin oss aliyun oss
PLUGIN_ALIYUN_OSS_REGION=
PLUGIN_ALIYUN_OSS_ENDPOINT=
PLUGIN_ALIYUN_OSS_ACCESS_KEY_ID=
PLUGIN_ALIYUN_OSS_ACCESS_KEY_SECRET=
PLUGIN_ALIYUN_OSS_AUTH_VERSION=v4
PLUGIN_ALIYUN_OSS_PATH=
# Plugin oss volcengine tos
PLUGIN_VOLCENGINE_TOS_ENDPOINT=
PLUGIN_VOLCENGINE_TOS_ACCESS_KEY=
PLUGIN_VOLCENGINE_TOS_SECRET_KEY=
PLUGIN_VOLCENGINE_TOS_REGION=

# ------------------------------
# Environment Variables for Aliyun SLS (Simple Log Service)
# ------------------------------
# Aliyun SLS Access Key ID
ALIYUN_SLS_ACCESS_KEY_ID=
# Aliyun SLS Access Key Secret
ALIYUN_SLS_ACCESS_KEY_SECRET=
# Aliyun SLS Endpoint (e.g., cn-hangzhou.log.aliyuncs.com)
ALIYUN_SLS_ENDPOINT=
# Aliyun SLS Region (e.g., cn-hangzhou)
ALIYUN_SLS_REGION=
# Aliyun SLS Project Name
ALIYUN_SLS_PROJECT_NAME=
# Aliyun SLS Logstore TTL (default: 365 days， 3650 for permanent storage)
ALIYUN_SLS_LOGSTORE_TTL=365
# Enable dual-write to both LogStore and SQL database (default: true)
LOGSTORE_DUAL_WRITE_ENABLED=true
# Enable dual-read fallback to SQL database when LogStore returns no results (default: true)
# Useful for migration scenarios where historical data exists only in SQL database
LOGSTORE_DUAL_READ_ENABLED=true
# Control flag for whether to write the `graph` field to LogStore.
# If LOGSTORE_ENABLE_PUT_GRAPH_FIELD is "true", write the full `graph` field;
# otherwise write an empty {} instead. Defaults to writing the `graph` field.
LOGSTORE_ENABLE_PUT_GRAPH_FIELD=true
```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-015.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
