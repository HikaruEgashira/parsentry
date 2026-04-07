You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-022
- **Kind**: public_api
- **Identifier**: Node.js SDK Client
- **Description**: Official Node.js SDK for Dify API. Risk of API key exposure in client-side code, insecure default HTTP configuration, and insufficient input validation before API calls.
- **Locations**: sdks/nodejs-client/src/client/, sdks/nodejs-client/src/http/

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

### sdks/nodejs-client/src/client/validation.ts
```ts
import { ValidationError } from "../errors/dify-error";
import { isRecord } from "../internal/type-guards";

const MAX_STRING_LENGTH = 10000;
const MAX_LIST_LENGTH = 1000;
const MAX_DICT_LENGTH = 100;

export function ensureNonEmptyString(
  value: unknown,
  name: string
): asserts value is string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new ValidationError(`${name} must be a non-empty string`);
  }
  if (value.length > MAX_STRING_LENGTH) {
    throw new ValidationError(
      `${name} exceeds maximum length of ${MAX_STRING_LENGTH} characters`
    );
  }
}

/**
 * Validates optional string fields that must be non-empty when provided.
 * Use this for fields like `name` that are optional but should not be empty strings.
 *
 * For filter parameters that accept empty strings (e.g., `keyword: ""`),
 * use `validateParams` which allows empty strings for optional params.
 */
export function ensureOptionalString(value: unknown, name: string): void {
  if (value === undefined || value === null) {
    return;
  }
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new ValidationError(`${name} must be a non-empty string when set`);
  }
  if (value.length > MAX_STRING_LENGTH) {
    throw new ValidationError(
      `${name} exceeds maximum length of ${MAX_STRING_LENGTH} characters`
    );
  }
}

export function ensureOptionalInt(value: unknown, name: string): void {
  if (value === undefined || value === null) {
    return;
  }
  if (!Number.isInteger(value)) {
    throw new ValidationError(`${name} must be an integer when set`);
  }
}

export function ensureOptionalBoolean(value: unknown, name: string): void {
  if (value === undefined || value === null) {
    return;
  }
  if (typeof value !== "boolean") {
    throw new ValidationError(`${name} must be a boolean when set`);
  }
}

export function ensureStringArray(value: unknown, name: string): void {
  if (!Array.isArray(value) || value.length === 0) {
    throw new ValidationError(`${name} must be a non-empty string array`);
  }
  if (value.length > MAX_LIST_LENGTH) {
    throw new ValidationError(
      `${name} exceeds maximum size of ${MAX_LIST_LENGTH} items`
    );
  }
  value.forEach((item) => {
    if (typeof item !== "string" || item.trim().length === 0) {
      throw new ValidationError(`${name} must contain non-empty strings`);
    }
  });
}

export function ensureOptionalStringArray(value: unknown, name: string): void {
  if (value === undefined || value === null) {
    return;
  }
  ensureStringArray(value, name);
}

export function ensureRating(value: unknown): void {
  if (value === undefined || value === null) {
    return;
  }
  if (value !== "like" && value !== "dislike") {
    throw new ValidationError("rating must be either 'like' or 'dislike'");
  }
}

export function validateParams(params: Record<string, unknown>): void {
  Object.entries(params).forEach(([key, value]) => {
    if (value === undefined || value === null) {
      return;
    }

    // Only check max length for strings; empty strings are allowed for optional params
    // Required fields are validated at method level via ensureNonEmptyString
    if (typeof value === "string") {
      if (value.length > MAX_STRING_LENGTH) {
        throw new ValidationError(
          `Parameter '${key}' exceeds maximum length of ${MAX_STRING_LENGTH} characters`
        );
      }
    } else if (Array.isArray(value)) {
      if (value.length > MAX_LIST_LENGTH) {
        throw new ValidationError(
          `Parameter '${key}' exceeds maximum size of ${MAX_LIST_LENGTH} items`
        );
      }
    } else if (isRecord(value)) {
      if (Object.keys(value).length > MAX_DICT_LENGTH) {
        throw new ValidationError(
          `Parameter '${key}' exceeds maximum size of ${MAX_DICT_LENGTH} items`
        );
      }
    }

    if (key === "user" && typeof value !== "string") {
      throw new ValidationError(`Parameter '${key}' must be a string`);
    }
    if (
      (key === "page" || key === "limit" || key === "page_size") &&
      !Number.isInteger(value)
    ) {
      throw new ValidationError(`Parameter '${key}' must be an integer`);
    }
    if (key === "files" && !Array.isArray(value) && typeof value !== "object") {
      throw new ValidationError(`Parameter '${key}' must be a list or dict`);
    }
    if (key === "rating" && value !== "like" && value !== "dislike") {
      throw new ValidationError(`Parameter '${key}' must be 'like' or 'dislike'`);
    }
  });
}

```

### sdks/nodejs-client/src/client/base.ts
```ts
import type {
  BinaryStream,
  DifyClientConfig,
  DifyResponse,
  JsonObject,
  MessageFeedbackRequest,
  QueryParams,
  RequestMethod,
  SuccessResponse,
  TextToAudioRequest,
} from "../types/common";
import type { HttpRequestBody } from "../http/client";
import { HttpClient } from "../http/client";
import { ensureNonEmptyString, ensureRating } from "./validation";
import { FileUploadError, ValidationError } from "../errors/dify-error";
import type { SdkFormData } from "../http/form-data";
import { isFormData } from "../http/form-data";

const toConfig = (
  init: string | DifyClientConfig,
  baseUrl?: string
): DifyClientConfig => {
  if (typeof init === "string") {
    return {
      apiKey: init,
      baseUrl,
    };
  }
  return init;
};

const appendUserToFormData = (form: SdkFormData, user: string): void => {
  form.append("user", user);
};

export class DifyClient {
  protected http: HttpClient;

  constructor(config: string | DifyClientConfig | HttpClient, baseUrl?: string) {
    if (config instanceof HttpClient) {
      this.http = config;
    } else {
      this.http = new HttpClient(toConfig(config, baseUrl));
    }
  }

  updateApiKey(apiKey: string): void {
    ensureNonEmptyString(apiKey, "apiKey");
    this.http.updateApiKey(apiKey);
  }

  getHttpClient(): HttpClient {
    return this.http;
  }

  sendRequest(
    method: RequestMethod,
    endpoint: string,
    data: HttpRequestBody = null,
    params: QueryParams | null = null,
    stream = false,
    headerParams: Record<string, string> = {}
  ): ReturnType<HttpClient["requestRaw"]> {
    return this.http.requestRaw({
      method,
      path: endpoint,
      data,
      query: params ?? undefined,
      headers: headerParams,
      responseType: stream ? "stream" : "json",
    });
  }

  getRoot(): Promise<DifyResponse<JsonObject>> {
    return this.http.request({
      method: "GET",
      path: "/",
    });
  }

  getApplicationParameters(user?: string): Promise<DifyResponse<JsonObject>> {
    if (user) {
      ensureNonEmptyString(user, "user");
    }
    return this.http.request({
      method: "GET",
      path: "/parameters",
      query: user ? { user } : undefined,
    });
  }

  async getParameters(user?: string): Promise<DifyResponse<JsonObject>> {
    return this.getApplicationParameters(user);
  }

  getMeta(user?: string): Promise<DifyResponse<JsonObject>> {
    if (user) {
      ensureNonEmptyString(user, "user");
    }
    return this.http.request({
      method: "GET",
      path: "/meta",
      query: user ? { user } : undefined,
    });
  }

  messageFeedback(
    request: MessageFeedbackRequest
  ): Promise<DifyResponse<SuccessResponse>>;
  messageFeedback(
    messageId: string,
    rating: "like" | "dislike" | null,
    user: string,
    content?: string
  ): Promise<DifyResponse<SuccessResponse>>;
  messageFeedback(
    messageIdOrRequest: string | MessageFeedbackRequest,
    rating?: "like" | "dislike" | null,
    user?: string,
    content?: string
  ): Promise<DifyResponse<SuccessResponse>> {
    let messageId: string;
    const payload: JsonObject = {};

    if (typeof messageIdOrRequest === "string") {
      messageId = messageIdOrRequest;
      ensureNonEmptyString(messageId, "messageId");
      ensureNonEmptyString(user, "user");
      payload.user = user;
      if (rating !== undefined && rating !== null) {
        ensureRating(rating);
        payload.rating = rating;
      }
      if (content !== undefined) {
        payload.content = content;
      }
    } else {
      const request = messageIdOrRequest;
      messageId = request.messageId;
      ensureNonEmptyString(messageId, "messageId");
      ensureNonEmptyString(request.user, "user");
      payload.user = request.user;
      if (request.rating !== undefined && request.rating !== null) {
        ensureRating(request.rating);
        payload.rating = request.rating;
      }
      if (request.content !== undefined) {
        payload.content = request.content;
      }
    }

    return this.http.request({
      method: "POST",
      path: `/messages/${messageId}/feedbacks`,
      data: payload,
    });
  }

  getInfo(user?: string): Promise<DifyResponse<JsonObject>> {
    if (user) {
      ensureNonEmptyString(user, "user");
    }
    return this.http.request({
      method: "GET",
      path: "/info",
      query: user ? { user } : undefined,
    });
  }

  getSite(user?: string): Promise<DifyResponse<JsonObject>> {
    if (user) {
      ensureNonEmptyString(user, "user");
    }
    return this.http.request({
      method: "GET",
      path: "/site",
      query: user ? { user } : undefined,
    });
  }

  fileUpload(form: unknown, user: string): Promise<DifyResponse<JsonObject>> {
    if (!isFormData(form)) {
      throw new FileUploadError("FormData is required for file uploads");
    }
    ensureNonEmptyString(user, "user");
    appendUserToFormData(form, user);
    return this.http.request({
      method: "POST",
      path: "/files/upload",
      data: form,
    });
  }

  filePreview(
    fileId: string,
    user: string,
    asAttachment?: boolean
  ): Promise<DifyResponse<Buffer>> {
    ensureNonEmptyString(fileId, "fileId");
    ensureNonEmptyString(user, "user");
    return this.http.request<Buffer, "bytes">({
      method: "GET",
      path: `/files/${fileId}/preview`,
      query: {
        user,
        as_attachment: asAttachment ? "true" : undefined,
      },
      responseType: "bytes",
    });
  }

  audioToText(form: unknown, user: string): Promise<DifyResponse<JsonObject>> {
    if (!isFormData(form)) {
      throw new FileUploadError("FormData is required for audio uploads");
    }
    ensureNonEmptyString(user, "user");
    appendUserToFormData(form, user);
    return this.http.request({
      method: "POST",
      path: "/audio-to-text",
      data: form,
    });
  }

  textToAudio(
    request: TextToAudioRequest
  ): Promise<DifyResponse<Buffer> | BinaryStream>;
  textToAudio(
    text: string,
    user: string,
    streaming?: boolean,
    voice?: string
  ): Promise<DifyResponse<Buffer> | BinaryStream>;
  textToAudio(
    textOrRequest: string | TextToAudioRequest,
    user?: string,
    streaming = false,
    voice?: string
  ): Promise<DifyResponse<Buffer> | BinaryStream> {
    let payload: TextToAudioRequest;

    if (typeof textOrRequest === "string") {
      ensureNonEmptyString(textOrRequest, "text");
      ensureNonEmptyString(user, "user");
      payload = {
        text: textOrRequest,
        user,
        streaming,
      };
      if (voice) {
        payload.voice = voice;
      }
    } else {
      payload = { ...textOrRequest };
      ensureNonEmptyString(payload.user, "user");
      if (payload.text !== undefined && payload.text !== null) {
        ensureNonEmptyString(payload.text, "text");
      }
      if (payload.message_id !== undefined && payload.message_id !== null) {
        ensureNonEmptyString(payload.message_id, "messageId");
      }
      if (!payload.text && !payload.message_id) {
        throw new ValidationError("text or message_id is required");
      }
      payload.streaming = payload.streaming ?? false;
    }

    if (payload.streaming) {
      return this.http.requestBinaryStream({
        method: "POST",
        path: "/text-to-audio",
        data: payload,
      });
    }

    return this.http.request<Buffer, "bytes">({
      method: "POST",
      path: "/text-to-audio",
      data: payload,
      responseType: "bytes",
    });
  }
}

```

### sdks/nodejs-client/src/client/base.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ValidationError } from "../errors/dify-error";
import { DifyClient } from "./base";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("DifyClient base", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("getRoot calls root endpoint", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.getRoot();

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/",
    });
  });

  it("getApplicationParameters includes optional user", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.getApplicationParameters();
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/parameters",
      query: undefined,
    });

    await dify.getApplicationParameters("user-1");
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/parameters",
      query: { user: "user-1" },
    });
  });

  it("getMeta includes optional user", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.getMeta("user-1");
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/meta",
      query: { user: "user-1" },
    });
  });

  it("getInfo and getSite support optional user", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.getInfo();
    await dify.getSite("user");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/info",
      query: undefined,
    });
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/site",
      query: { user: "user" },
    });
  });

  it("messageFeedback builds payload from request object", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.messageFeedback({
      messageId: "msg",
      user: "user",
      rating: "like",
      content: "good",
    });

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/messages/msg/feedbacks",
      data: { user: "user", rating: "like", content: "good" },
    });
  });

  it("fileUpload appends user to form data", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);
    const form = { append: vi.fn(), getHeaders: () => ({}) };

    await dify.fileUpload(form, "user");

    expect(form.append).toHaveBeenCalledWith("user", "user");
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/files/upload",
      data: form,
    });
  });

  it("filePreview uses bytes response", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.filePreview("file", "user", true);

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/files/file/preview",
      query: { user: "user", as_attachment: "true" },
      responseType: "bytes",
    });
  });

  it("audioToText appends user and sends form", async () => {
    const { client, request } = createHttpClientWithSpies();
    const dify = new DifyClient(client);
    const form = { append: vi.fn(), getHeaders: () => ({}) };

    await dify.audioToText(form, "user");

    expect(form.append).toHaveBeenCalledWith("user", "user");
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/audio-to-text",
      data: form,
    });
  });

  it("textToAudio supports streaming and message id", async () => {
    const { client, request, requestBinaryStream } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    await dify.textToAudio({
      user: "user",
      message_id: "msg",
      streaming: true,
    });

    expect(requestBinaryStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/text-to-audio",
      data: {
        user: "user",
        message_id: "msg",
        streaming: true,
      },
    });

    await dify.textToAudio("hello", "user", false, "voice");
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/text-to-audio",
      data: {
        text: "hello",
        user: "user",
        streaming: false,
        voice: "voice",
      },
      responseType: "bytes",
    });
  });

  it("textToAudio requires text or message id", () => {
    const { client } = createHttpClientWithSpies();
    const dify = new DifyClient(client);

    expect(() => dify.textToAudio({ user: "user" })).toThrow(ValidationError);
  });
});

```

### sdks/nodejs-client/src/client/workspace.ts
```ts
import { DifyClient } from "./base";
import type { WorkspaceModelType, WorkspaceModelsResponse } from "../types/workspace";
import type { DifyResponse } from "../types/common";
import { ensureNonEmptyString } from "./validation";

export class WorkspaceClient extends DifyClient {
  async getModelsByType(
    modelType: WorkspaceModelType
  ): Promise<DifyResponse<WorkspaceModelsResponse>> {
    ensureNonEmptyString(modelType, "modelType");
    return this.http.request({
      method: "GET",
      path: `/workspaces/current/models/model-types/${modelType}`,
    });
  }
}

```

### sdks/nodejs-client/src/client/workflow.ts
```ts
import { DifyClient } from "./base";
import type { WorkflowRunRequest, WorkflowRunResponse } from "../types/workflow";
import type {
  DifyResponse,
  DifyStream,
  JsonObject,
  QueryParams,
  SuccessResponse,
} from "../types/common";
import {
  ensureNonEmptyString,
  ensureOptionalInt,
  ensureOptionalString,
} from "./validation";

export class WorkflowClient extends DifyClient {
  run(
    request: WorkflowRunRequest
  ): Promise<DifyResponse<WorkflowRunResponse> | DifyStream<WorkflowRunResponse>>;
  run(
    inputs: JsonObject,
    user: string,
    stream?: boolean
  ): Promise<DifyResponse<WorkflowRunResponse> | DifyStream<WorkflowRunResponse>>;
  run(
    inputOrRequest: WorkflowRunRequest | JsonObject,
    user?: string,
    stream = false
  ): Promise<DifyResponse<WorkflowRunResponse> | DifyStream<WorkflowRunResponse>> {
    let payload: WorkflowRunRequest;
    let shouldStream = stream;

    if (user === undefined && "user" in (inputOrRequest as WorkflowRunRequest)) {
      payload = inputOrRequest as WorkflowRunRequest;
      shouldStream = payload.response_mode === "streaming";
    } else {
      ensureNonEmptyString(user, "user");
      payload = {
        inputs: inputOrRequest,
        user,
        response_mode: stream ? "streaming" : "blocking",
      };
    }

    ensureNonEmptyString(payload.user, "user");

    if (shouldStream) {
      return this.http.requestStream<WorkflowRunResponse>({
        method: "POST",
        path: "/workflows/run",
        data: payload,
      });
    }

    return this.http.request<WorkflowRunResponse>({
      method: "POST",
      path: "/workflows/run",
      data: payload,
    });
  }

  runById(
    workflowId: string,
    request: WorkflowRunRequest
  ): Promise<DifyResponse<WorkflowRunResponse> | DifyStream<WorkflowRunResponse>> {
    ensureNonEmptyString(workflowId, "workflowId");
    ensureNonEmptyString(request.user, "user");
    if (request.response_mode === "streaming") {
      return this.http.requestStream<WorkflowRunResponse>({
        method: "POST",
        path: `/workflows/${workflowId}/run`,
        data: request,
      });
    }
    return this.http.request<WorkflowRunResponse>({
      method: "POST",
      path: `/workflows/${workflowId}/run`,
      data: request,
    });
  }

  getRun(workflowRunId: string): Promise<DifyResponse<WorkflowRunResponse>> {
    ensureNonEmptyString(workflowRunId, "workflowRunId");
    return this.http.request({
      method: "GET",
      path: `/workflows/run/${workflowRunId}`,
    });
  }

  stop(
    taskId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    ensureNonEmptyString(taskId, "taskId");
    ensureNonEmptyString(user, "user");
    return this.http.request<SuccessResponse>({
      method: "POST",
      path: `/workflows/tasks/${taskId}/stop`,
      data: { user },
    });
  }

  /**
   * Get workflow execution logs with filtering options.
   *
   * Note: The backend API filters by `createdByEndUserSessionId` (end user session ID)
   * or `createdByAccount` (account ID), not by a generic `user` parameter.
   */
  getLogs(options?: {
    keyword?: string;
    status?: string;
    createdAtBefore?: string;
    createdAtAfter?: string;
    createdByEndUserSessionId?: string;
    createdByAccount?: string;
    page?: number;
    limit?: number;
    startTime?: string;
    endTime?: string;
  }): Promise<DifyResponse<JsonObject>> {
    if (options?.keyword) {
      ensureOptionalString(options.keyword, "keyword");
    }
    if (options?.status) {
      ensureOptionalString(options.status, "status");
    }
    if (options?.createdAtBefore) {
      ensureOptionalString(options.createdAtBefore, "createdAtBefore");
    }
    if (options?.createdAtAfter) {
      ensureOptionalString(options.createdAtAfter, "createdAtAfter");
    }
    if (options?.createdByEndUserSessionId) {
      ensureOptionalString(
        options.createdByEndUserSessionId,
        "createdByEndUserSessionId"
      );
    }
    if (options?.createdByAccount) {
      ensureOptionalString(options.createdByAccount, "createdByAccount");
    }
    if (options?.startTime) {
      ensureOptionalString(options.startTime, "startTime");
    }
    if (options?.endTime) {
      ensureOptionalString(options.endTime, "endTime");
    }
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");

    const createdAtAfter = options?.createdAtAfter ?? options?.startTime;
    const createdAtBefore = options?.createdAtBefore ?? options?.endTime;

    const query: QueryParams = {
      keyword: options?.keyword,
      status: options?.status,
      created_at__before: createdAtBefore,
      created_at__after: createdAtAfter,
      created_by_end_user_session_id: options?.createdByEndUserSessionId,
      created_by_account: options?.createdByAccount,
      page: options?.page,
      limit: options?.limit,
    };

    return this.http.request({
      method: "GET",
      path: "/workflows/logs",
      query,
    });
  }
}

```

### sdks/nodejs-client/src/client/completion.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { CompletionClient } from "./completion";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("CompletionClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("creates completion messages in blocking mode", async () => {
    const { client, request } = createHttpClientWithSpies();
    const completion = new CompletionClient(client);

    await completion.createCompletionMessage({ input: "x" }, "user", false);

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/completion-messages",
      data: {
        inputs: { input: "x" },
        user: "user",
        files: undefined,
        response_mode: "blocking",
      },
    });
  });

  it("creates completion messages in streaming mode", async () => {
    const { client, requestStream } = createHttpClientWithSpies();
    const completion = new CompletionClient(client);

    await completion.createCompletionMessage({
      inputs: { input: "x" },
      user: "user",
      response_mode: "streaming",
    });

    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/completion-messages",
      data: {
        inputs: { input: "x" },
        user: "user",
        response_mode: "streaming",
      },
    });
  });

  it("stops completion messages", async () => {
    const { client, request } = createHttpClientWithSpies();
    const completion = new CompletionClient(client);

    await completion.stopCompletionMessage("task", "user");
    await completion.stop("task", "user");

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/completion-messages/task/stop",
      data: { user: "user" },
    });
  });

  it("supports deprecated runWorkflow", async () => {
    const { client, request, requestStream } = createHttpClientWithSpies();
    const completion = new CompletionClient(client);
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

    await completion.runWorkflow({ input: "x" }, "user", false);
    await completion.runWorkflow({ input: "x" }, "user", true);

    expect(warn).toHaveBeenCalled();
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/run",
      data: { inputs: { input: "x" }, user: "user", response_mode: "blocking" },
    });
    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/run",
      data: { inputs: { input: "x" }, user: "user", response_mode: "streaming" },
    });
  });
});

```

### sdks/nodejs-client/src/client/chat.ts
```ts
import { DifyClient } from "./base";
import type {
  ChatMessageRequest,
  ChatMessageResponse,
  ConversationSortBy,
} from "../types/chat";
import type {
  AnnotationCreateRequest,
  AnnotationListOptions,
  AnnotationReplyActionRequest,
  AnnotationResponse,
} from "../types/annotation";
import type {
  DifyResponse,
  DifyStream,
  JsonObject,
  JsonValue,
  QueryParams,
  SuccessResponse,
  SuggestedQuestionsResponse,
} from "../types/common";
import {
  ensureNonEmptyString,
  ensureOptionalInt,
  ensureOptionalString,
} from "./validation";

export class ChatClient extends DifyClient {
  createChatMessage(
    request: ChatMessageRequest
  ): Promise<DifyResponse<ChatMessageResponse> | DifyStream<ChatMessageResponse>>;
  createChatMessage(
    inputs: JsonObject,
    query: string,
    user: string,
    stream?: boolean,
    conversationId?: string | null,
    files?: ChatMessageRequest["files"]
  ): Promise<DifyResponse<ChatMessageResponse> | DifyStream<ChatMessageResponse>>;
  createChatMessage(
    inputOrRequest: ChatMessageRequest | JsonObject,
    query?: string,
    user?: string,
    stream = false,
    conversationId?: string | null,
    files?: ChatMessageRequest["files"]
  ): Promise<DifyResponse<ChatMessageResponse> | DifyStream<ChatMessageResponse>> {
    let payload: ChatMessageRequest;
    let shouldStream = stream;

    if (query === undefined && "user" in (inputOrRequest as ChatMessageRequest)) {
      payload = inputOrRequest as ChatMessageRequest;
      shouldStream = payload.response_mode === "streaming";
    } else {
      ensureNonEmptyString(query, "query");
      ensureNonEmptyString(user, "user");
        payload = {
        inputs: inputOrRequest,
        query,
        user,
        response_mode: stream ? "streaming" : "blocking",
        files,
      };
      if (conversationId) {
        payload.conversation_id = conversationId;
      }
    }

    ensureNonEmptyString(payload.user, "user");
    ensureNonEmptyString(payload.query, "query");

    if (shouldStream) {
      return this.http.requestStream<ChatMessageResponse>({
        method: "POST",
        path: "/chat-messages",
        data: payload,
      });
    }

    return this.http.request<ChatMessageResponse>({
      method: "POST",
      path: "/chat-messages",
      data: payload,
    });
  }

  stopChatMessage(
    taskId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    ensureNonEmptyString(taskId, "taskId");
    ensureNonEmptyString(user, "user");
    return this.http.request<SuccessResponse>({
      method: "POST",
      path: `/chat-messages/${taskId}/stop`,
      data: { user },
    });
  }

  stopMessage(
    taskId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    return this.stopChatMessage(taskId, user);
  }

  getSuggested(
    messageId: string,
    user: string
  ): Promise<DifyResponse<SuggestedQuestionsResponse>> {
    ensureNonEmptyString(messageId, "messageId");
    ensureNonEmptyString(user, "user");
    return this.http.request<SuggestedQuestionsResponse>({
      method: "GET",
      path: `/messages/${messageId}/suggested`,
      query: { user },
    });
  }

  // Note: messageFeedback is inherited from DifyClient

  getAppFeedbacks(
    page?: number,
    limit?: number
  ): Promise<DifyResponse<JsonObject>> {
    ensureOptionalInt(page, "page");
    ensureOptionalInt(limit, "limit");
    return this.http.request({
      method: "GET",
      path: "/app/feedbacks",
      query: {
        page,
        limit,
      },
    });
  }

  getConversations(
    user: string,
    lastId?: string | null,
    limit?: number | null,
    sortBy?: ConversationSortBy | null
  ): Promise<DifyResponse<JsonObject>> {
    ensureNonEmptyString(user, "user");
    ensureOptionalString(lastId, "lastId");
    ensureOptionalInt(limit, "limit");

    const params: QueryParams = { user };
    if (lastId) {
      params.last_id = lastId;
    }
    if (limit) {
      params.limit = limit;
    }
    if (sortBy) {
      params.sort_by = sortBy;
    }

    return this.http.request({
      method: "GET",
      path: "/conversations",
      query: params,
    });
  }

  getConversationMessages(
    user: string,
    conversationId: string,
    firstId?: string | null,
    limit?: number | null
  ): Promise<DifyResponse<JsonObject>> {
    ensureNonEmptyString(user, "user");
    ensureNonEmptyString(conversationId, "conversationId");
    ensureOptionalString(firstId, "firstId");
    ensureOptionalInt(limit, "limit");

    const params: QueryParams = { user };
    params.conversation_id = conversationId;
    if (firstId) {
      params.first_id = firstId;
    }
    if (limit) {
      params.limit = limit;
    }

    return this.http.request({
      method: "GET",
      path: "/messages",
      query: params,
    });
  }

  renameConversation(
    conversationId: string,
    name: string,
    user: string,
    autoGenerate?: boolean
  ): Promise<DifyResponse<JsonObject>>;
  renameConversation(
    conversationId: string,
    user: string,
    options?: { name?: string | null; autoGenerate?: boolean }
  ): Promise<DifyResponse<JsonObject>>;
  renameConversation(
    conversationId: string,
    nameOrUser: string,
    userOrOptions?: string | { name?: string | null; autoGenerate?: boolean },
    autoGenerate?: boolean
  ): Promise<DifyResponse<JsonObject>> {
    ensureNonEmptyString(conversationId, "conversationId");

    let name: string | null | undefined;
    let user: string;
    let resolvedAutoGenerate: boolean;

    if (typeof userOrOptions === "string" || userOrOptions === undefined) {
      name = nameOrUser;
      user = userOrOptions ?? "";
      resolvedAutoGenerate = autoGenerate ?? false;
    } else {
      user = nameOrUser;
      name = userOrOptions.name;
      resolvedAutoGenerate = userOrOptions.autoGenerate ?? false;
    }

    ensureNonEmptyString(user, "user");
    if (!resolvedAutoGenerate) {
      ensureNonEmptyString(name, "name");
    }

    const payload: JsonObject = {
      user,
      auto_generate: resolvedAutoGenerate,
    };
    if (typeof name === "string" && name.trim().length > 0) {
      payload.name = name;
    }

    return this.http.request({
      method: "POST",
      path: `/conversations/${conversationId}/name`,
      data: payload,
    });
  }

  deleteConversation(
    conversationId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    ensureNonEmptyString(conversationId, "conversationId");
    ensureNonEmptyString(user, "user");
    return this.http.request({
      method: "DELETE",
      path: `/conversations/${conversationId}`,
      data: { user },
    });
  }

  getConversationVariables(
    conversationId: string,
    user: string,
    lastId?: string | null,
    limit?: number | null,
    variableName?: string | null
  ): Promise<DifyResponse<JsonObject>> {
    ensureNonEmptyString(conversationId, "conversationId");
    ensureNonEmptyString(user, "user");
    ensureOptionalString(lastId, "lastId");
    ensureOptionalInt(limit, "limit");
    ensureOptionalString(variableName, "variableName");

    return this.http.request({
      method: "GET",
      path: `/conversations/${conversationId}/variables`,
      query: {
        user,
        last_id: lastId ?? undefined,
        limit: limit ?? undefined,
        variable_name: variableName ?? undefined,
      },
    });
  }

  updateConversationVariable(
    conversationId: string,
    variableId: string,
    user: string,
    value: JsonValue
  ): Promise<DifyResponse<JsonObject>> {
    ensureNonEmptyString(conversationId, "conversationId");
    ensureNonEmptyString(variableId, "variableId");
    ensureNonEmptyString(user, "user");
    return this.http.request({
      method: "PUT",
      path: `/conversations/${conversationId}/variables/${variableId}`,
      data: {
        user,
        value,
      },
    });
  }

  annotationReplyAction(
    action: "enable" | "disable",
    request: AnnotationReplyActionRequest
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureNonEmptyString(action, "action");
    ensureNonEmptyString(request.embedding_provider_name, "embedding_provider_name");
    ensureNonEmptyString(request.embedding_model_name, "embedding_model_name");
    return this.http.request({
      method: "POST",
      path: `/apps/annotation-reply/${action}`,
      data: request,
    });
  }

  getAnnotationReplyStatus(
    action: "enable" | "disable",
    jobId: string
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureNonEmptyString(action, "action");
    ensureNonEmptyString(jobId, "jobId");
    return this.http.request({
      method: "GET",
      path: `/apps/annotation-reply/${action}/status/${jobId}`,
    });
  }

  listAnnotations(
    options?: AnnotationListOptions
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");
    ensureOptionalString(options?.keyword, "keyword");
    return this.http.request({
      method: "GET",
      path: "/apps/annotations",
      query: {
        page: options?.page,
        limit: options?.limit,
        keyword: options?.keyword ?? undefined,
      },
    });
  }

  createAnnotation(
    request: AnnotationCreateRequest
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureNonEmptyString(request.question, "question");
    ensureNonEmptyString(request.answer, "answer");
    return this.http.request({
      method: "POST",
      path: "/apps/annotations",
      data: request,
    });
  }

  updateAnnotation(
    annotationId: string,
    request: AnnotationCreateRequest
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureNonEmptyString(annotationId, "annotationId");
    ensureNonEmptyString(request.question, "question");
    ensureNonEmptyString(request.answer, "answer");
    return this.http.request({
      method: "PUT",
      path: `/apps/annotations/${annotationId}`,
      data: request,
    });
  }

  deleteAnnotation(
    annotationId: string
  ): Promise<DifyResponse<AnnotationResponse>> {
    ensureNonEmptyString(annotationId, "annotationId");
    return this.http.request({
      method: "DELETE",
      path: `/apps/annotations/${annotationId}`,
    });
  }

  // Note: audioToText is inherited from DifyClient
}

```

### sdks/nodejs-client/src/client/workspace.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { WorkspaceClient } from "./workspace";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("WorkspaceClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("gets models by type", async () => {
    const { client, request } = createHttpClientWithSpies();
    const workspace = new WorkspaceClient(client);

    await workspace.getModelsByType("llm");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/workspaces/current/models/model-types/llm",
    });
  });
});

```

### sdks/nodejs-client/src/client/knowledge-base.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { FileUploadError, ValidationError } from "../errors/dify-error";
import { KnowledgeBaseClient } from "./knowledge-base";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("KnowledgeBaseClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("handles dataset and tag operations", async () => {
    const { client, request } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);

    await kb.listDatasets({
      page: 1,
      limit: 2,
      keyword: "k",
      includeAll: true,
      tagIds: ["t1"],
    });
    await kb.createDataset({ name: "dataset" });
    await kb.getDataset("ds");
    await kb.updateDataset("ds", { name: "new" });
    await kb.deleteDataset("ds");
    await kb.updateDocumentStatus("ds", "enable", ["doc1"]);

    await kb.listTags();
    await kb.createTag({ name: "tag" });
    await kb.updateTag({ tag_id: "tag", name: "name" });
    await kb.deleteTag({ tag_id: "tag" });
    await kb.bindTags({ tag_ids: ["tag"], target_id: "doc" });
    await kb.unbindTags({ tag_id: "tag", target_id: "doc" });
    await kb.getDatasetTags("ds");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/datasets",
      query: {
        page: 1,
        limit: 2,
        keyword: "k",
        include_all: true,
        tag_ids: ["t1"],
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets",
      data: { name: "dataset" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "PATCH",
      path: "/datasets/ds",
      data: { name: "new" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "PATCH",
      path: "/datasets/ds/documents/status/enable",
      data: { document_ids: ["doc1"] },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/tags/binding",
      data: { tag_ids: ["tag"], target_id: "doc" },
    });
  });

  it("handles document operations", async () => {
    const { client, request } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);
    const form = { append: vi.fn(), getHeaders: () => ({}) };

    await kb.createDocumentByText("ds", { name: "doc", text: "text" });
    await kb.updateDocumentByText("ds", "doc", { name: "doc2" });
    await kb.createDocumentByFile("ds", form);
    await kb.updateDocumentByFile("ds", "doc", form);
    await kb.listDocuments("ds", { page: 1, limit: 20, keyword: "k" });
    await kb.getDocument("ds", "doc", { metadata: "all" });
    await kb.deleteDocument("ds", "doc");
    await kb.getDocumentIndexingStatus("ds", "batch");

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/document/create_by_text",
      data: { name: "doc", text: "text" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/documents/doc/update_by_text",
      data: { name: "doc2" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/document/create_by_file",
      data: form,
    });
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/datasets/ds/documents",
      query: { page: 1, limit: 20, keyword: "k", status: undefined },
    });
  });

  it("handles segments and child chunks", async () => {
    const { client, request } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);

    await kb.createSegments("ds", "doc", { segments: [{ content: "x" }] });
    await kb.listSegments("ds", "doc", { page: 1, limit: 10, keyword: "k" });
    await kb.getSegment("ds", "doc", "seg");
    await kb.updateSegment("ds", "doc", "seg", {
      segment: { content: "y" },
    });
    await kb.deleteSegment("ds", "doc", "seg");

    await kb.createChildChunk("ds", "doc", "seg", { content: "c" });
    await kb.listChildChunks("ds", "doc", "seg", { page: 1, limit: 10 });
    await kb.updateChildChunk("ds", "doc", "seg", "child", {
      content: "c2",
    });
    await kb.deleteChildChunk("ds", "doc", "seg", "child");

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/documents/doc/segments",
      data: { segments: [{ content: "x" }] },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/documents/doc/segments/seg",
      data: { segment: { content: "y" } },
    });
    expect(request).toHaveBeenCalledWith({
      method: "PATCH",
      path: "/datasets/ds/documents/doc/segments/seg/child_chunks/child",
      data: { content: "c2" },
    });
  });

  it("handles metadata and retrieval", async () => {
    const { client, request } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);

    await kb.listMetadata("ds");
    await kb.createMetadata("ds", { name: "m", type: "string" });
    await kb.updateMetadata("ds", "mid", { name: "m2" });
    await kb.deleteMetadata("ds", "mid");
    await kb.listBuiltInMetadata("ds");
    await kb.updateBuiltInMetadata("ds", "enable");
    await kb.updateDocumentsMetadata("ds", {
      operation_data: [
        { document_id: "doc", metadata_list: [{ id: "m", name: "n" }] },
      ],
    });
    await kb.hitTesting("ds", { query: "q" });
    await kb.retrieve("ds", { query: "q" });

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/datasets/ds/metadata",
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/metadata",
      data: { name: "m", type: "string" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/hit-testing",
      data: { query: "q" },
    });
  });

  it("handles pipeline operations", async () => {
    const { client, request, requestStream } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);
    const form = { append: vi.fn(), getHeaders: () => ({}) };

    await kb.listDatasourcePlugins("ds", { isPublished: true });
    await kb.runDatasourceNode("ds", "node", {
      inputs: { input: "x" },
      datasource_type: "custom",
      is_published: true,
    });
    await kb.runPipeline("ds", {
      inputs: { input: "x" },
      datasource_type: "custom",
      datasource_info_list: [],
      start_node_id: "start",
      is_published: true,
      response_mode: "streaming",
    });
    await kb.runPipeline("ds", {
      inputs: { input: "x" },
      datasource_type: "custom",
      datasource_info_list: [],
      start_node_id: "start",
      is_published: true,
      response_mode: "blocking",
    });
    await kb.uploadPipelineFile(form);

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/datasets/ds/pipeline/datasource-plugins",
      query: { is_published: true },
    });
    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/pipeline/datasource/nodes/node/run",
      data: {
        inputs: { input: "x" },
        datasource_type: "custom",
        is_published: true,
      },
    });
    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/pipeline/run",
      data: {
        inputs: { input: "x" },
        datasource_type: "custom",
        datasource_info_list: [],
        start_node_id: "start",
        is_published: true,
        response_mode: "streaming",
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/ds/pipeline/run",
      data: {
        inputs: { input: "x" },
        datasource_type: "custom",
        datasource_info_list: [],
        start_node_id: "start",
        is_published: true,
        response_mode: "blocking",
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/datasets/pipeline/file-upload",
      data: form,
    });
  });

  it("validates form-data and optional array filters", async () => {
    const { client } = createHttpClientWithSpies();
    const kb = new KnowledgeBaseClient(client);

    await expect(kb.createDocumentByFile("ds", {})).rejects.toBeInstanceOf(
      FileUploadError
    );
    await expect(
      kb.listSegments("ds", "doc", { status: ["ok", 1] as unknown as string[] })
    ).rejects.toBeInstanceOf(ValidationError);
    await expect(
      kb.hitTesting("ds", {
        query: "q",
        attachment_ids: ["att-1", 2] as unknown as string[],
      })
    ).rejects.toBeInstanceOf(ValidationError);
  });
});

```

### sdks/nodejs-client/src/client/chat.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ValidationError } from "../errors/dify-error";
import { ChatClient } from "./chat";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("ChatClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("creates chat messages in blocking mode", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.createChatMessage({ input: "x" }, "hello", "user", false, null);

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/chat-messages",
      data: {
        inputs: { input: "x" },
        query: "hello",
        user: "user",
        response_mode: "blocking",
        files: undefined,
      },
    });
  });

  it("creates chat messages in streaming mode", async () => {
    const { client, requestStream } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.createChatMessage({
      inputs: { input: "x" },
      query: "hello",
      user: "user",
      response_mode: "streaming",
    });

    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/chat-messages",
      data: {
        inputs: { input: "x" },
        query: "hello",
        user: "user",
        response_mode: "streaming",
      },
    });
  });

  it("stops chat messages", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.stopChatMessage("task", "user");
    await chat.stopMessage("task", "user");

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/chat-messages/task/stop",
      data: { user: "user" },
    });
  });

  it("gets suggested questions", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.getSuggested("msg", "user");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/messages/msg/suggested",
      query: { user: "user" },
    });
  });

  it("submits message feedback", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.messageFeedback("msg", "like", "user", "good");
    await chat.messageFeedback({
      messageId: "msg",
      user: "user",
      rating: "dislike",
    });

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/messages/msg/feedbacks",
      data: { user: "user", rating: "like", content: "good" },
    });
  });

  it("lists app feedbacks", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.getAppFeedbacks(2, 5);

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/app/feedbacks",
      query: { page: 2, limit: 5 },
    });
  });

  it("lists conversations and messages", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.getConversations("user", "last", 10, "-updated_at");
    await chat.getConversationMessages("user", "conv", "first", 5);

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/conversations",
      query: {
        user: "user",
        last_id: "last",
        limit: 10,
        sort_by: "-updated_at",
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/messages",
      query: {
        user: "user",
        conversation_id: "conv",
        first_id: "first",
        limit: 5,
      },
    });
  });

  it("renames conversations with optional auto-generate", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.renameConversation("conv", "name", "user", false);
    await chat.renameConversation("conv", "user", { autoGenerate: true });

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/conversations/conv/name",
      data: { user: "user", auto_generate: false, name: "name" },
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/conversations/conv/name",
      data: { user: "user", auto_generate: true },
    });
  });

  it("requires name when autoGenerate is false", () => {
    const { client } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    expect(() => chat.renameConversation("conv", "", "user", false)).toThrow(
      ValidationError
    );
  });

  it("deletes conversations", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.deleteConversation("conv", "user");

    expect(request).toHaveBeenCalledWith({
      method: "DELETE",
      path: "/conversations/conv",
      data: { user: "user" },
    });
  });

  it("manages conversation variables", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.getConversationVariables("conv", "user", "last", 10, "name");
    await chat.updateConversationVariable("conv", "var", "user", "value");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/conversations/conv/variables",
      query: {
        user: "user",
        last_id: "last",
        limit: 10,
        variable_name: "name",
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "PUT",
      path: "/conversations/conv/variables/var",
      data: { user: "user", value: "value" },
    });
  });

  it("handles annotation APIs", async () => {
    const { client, request } = createHttpClientWithSpies();
    const chat = new ChatClient(client);

    await chat.annotationReplyAction("enable", {
      score_threshold: 0.5,
      embedding_provider_name: "prov",
      embedding_model_name: "model",
    });
    await chat.getAnnotationReplyStatus("enable", "job");
    await chat.listAnnotations({ page: 1, limit: 10, keyword: "k" });
    await chat.createAnnotation({ question: "q", answer: "a" });
    await chat.updateAnnotation("id", { question: "q", answer: "a" });
    await chat.deleteAnnotation("id");

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/apps/annotation-reply/enable",
      data: {
        score_threshold: 0.5,
        embedding_provider_name: "prov",
        embedding_model_name: "model",
      },
    });
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/apps/annotation-reply/enable/status/job",
    });
    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/apps/annotations",
      query: { page: 1, limit: 10, keyword: "k" },
    });
  });
});

```

### sdks/nodejs-client/src/client/knowledge-base.ts
```ts
import { DifyClient } from "./base";
import type {
  DatasetCreateRequest,
  DatasetListOptions,
  DatasetTagBindingRequest,
  DatasetTagCreateRequest,
  DatasetTagDeleteRequest,
  DatasetTagUnbindingRequest,
  DatasetTagUpdateRequest,
  DatasetUpdateRequest,
  DocumentGetOptions,
  DocumentListOptions,
  DocumentStatusAction,
  DocumentTextCreateRequest,
  DocumentTextUpdateRequest,
  SegmentCreateRequest,
  SegmentListOptions,
  SegmentUpdateRequest,
  ChildChunkCreateRequest,
  ChildChunkListOptions,
  ChildChunkUpdateRequest,
  MetadataCreateRequest,
  MetadataOperationRequest,
  MetadataUpdateRequest,
  HitTestingRequest,
  DatasourcePluginListOptions,
  DatasourceNodeRunRequest,
  PipelineRunRequest,
  KnowledgeBaseResponse,
  PipelineStreamEvent,
} from "../types/knowledge-base";
import type { DifyResponse, DifyStream, QueryParams } from "../types/common";
import {
  ensureNonEmptyString,
  ensureOptionalBoolean,
  ensureOptionalInt,
  ensureOptionalString,
  ensureStringArray,
} from "./validation";
import { FileUploadError, ValidationError } from "../errors/dify-error";
import type { SdkFormData } from "../http/form-data";
import { isFormData } from "../http/form-data";

function ensureFormData(
  form: unknown,
  context: string
): asserts form is SdkFormData {
  if (!isFormData(form)) {
    throw new FileUploadError(`${context} requires FormData`);
  }
}

const ensureNonEmptyArray = (value: unknown, name: string): void => {
  if (!Array.isArray(value) || value.length === 0) {
    throw new ValidationError(`${name} must be a non-empty array`);
  }
};

export class KnowledgeBaseClient extends DifyClient {
  async listDatasets(
    options?: DatasetListOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");
    ensureOptionalString(options?.keyword, "keyword");
    ensureOptionalBoolean(options?.includeAll, "includeAll");

    const query: QueryParams = {
      page: options?.page,
      limit: options?.limit,
      keyword: options?.keyword ?? undefined,
      include_all: options?.includeAll ?? undefined,
    };

    if (options?.tagIds && options.tagIds.length > 0) {
      ensureStringArray(options.tagIds, "tagIds");
      query.tag_ids = options.tagIds;
    }

    return this.http.request({
      method: "GET",
      path: "/datasets",
      query,
    });
  }

  async createDataset(
    request: DatasetCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(request.name, "name");
    return this.http.request({
      method: "POST",
      path: "/datasets",
      data: request,
    });
  }

  async getDataset(datasetId: string): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}`,
    });
  }

  async updateDataset(
    datasetId: string,
    request: DatasetUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    if (request.name !== undefined && request.name !== null) {
      ensureNonEmptyString(request.name, "name");
    }
    return this.http.request({
      method: "PATCH",
      path: `/datasets/${datasetId}`,
      data: request,
    });
  }

  async deleteDataset(datasetId: string): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "DELETE",
      path: `/datasets/${datasetId}`,
    });
  }

  async updateDocumentStatus(
    datasetId: string,
    action: DocumentStatusAction,
    documentIds: string[]
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(action, "action");
    ensureStringArray(documentIds, "documentIds");
    return this.http.request({
      method: "PATCH",
      path: `/datasets/${datasetId}/documents/status/${action}`,
      data: {
        document_ids: documentIds,
      },
    });
  }

  async listTags(): Promise<DifyResponse<KnowledgeBaseResponse>> {
    return this.http.request({
      method: "GET",
      path: "/datasets/tags",
    });
  }

  async createTag(
    request: DatasetTagCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(request.name, "name");
    return this.http.request({
      method: "POST",
      path: "/datasets/tags",
      data: request,
    });
  }

  async updateTag(
    request: DatasetTagUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(request.tag_id, "tag_id");
    ensureNonEmptyString(request.name, "name");
    return this.http.request({
      method: "PATCH",
      path: "/datasets/tags",
      data: request,
    });
  }

  async deleteTag(
    request: DatasetTagDeleteRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(request.tag_id, "tag_id");
    return this.http.request({
      method: "DELETE",
      path: "/datasets/tags",
      data: request,
    });
  }

  async bindTags(
    request: DatasetTagBindingRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureStringArray(request.tag_ids, "tag_ids");
    ensureNonEmptyString(request.target_id, "target_id");
    return this.http.request({
      method: "POST",
      path: "/datasets/tags/binding",
      data: request,
    });
  }

  async unbindTags(
    request: DatasetTagUnbindingRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(request.tag_id, "tag_id");
    ensureNonEmptyString(request.target_id, "target_id");
    return this.http.request({
      method: "POST",
      path: "/datasets/tags/unbinding",
      data: request,
    });
  }

  async getDatasetTags(
    datasetId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/tags`,
    });
  }

  async createDocumentByText(
    datasetId: string,
    request: DocumentTextCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(request.name, "name");
    ensureNonEmptyString(request.text, "text");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/document/create_by_text`,
      data: request,
    });
  }

  async updateDocumentByText(
    datasetId: string,
    documentId: string,
    request: DocumentTextUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    if (request.name !== undefined && request.name !== null) {
      ensureNonEmptyString(request.name, "name");
    }
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/${documentId}/update_by_text`,
      data: request,
    });
  }

  async createDocumentByFile(
    datasetId: string,
    form: unknown
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureFormData(form, "createDocumentByFile");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/document/create_by_file`,
      data: form,
    });
  }

  async updateDocumentByFile(
    datasetId: string,
    documentId: string,
    form: unknown
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureFormData(form, "updateDocumentByFile");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/${documentId}/update_by_file`,
      data: form,
    });
  }

  async listDocuments(
    datasetId: string,
    options?: DocumentListOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");
    ensureOptionalString(options?.keyword, "keyword");
    ensureOptionalString(options?.status, "status");

    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents`,
      query: {
        page: options?.page,
        limit: options?.limit,
        keyword: options?.keyword ?? undefined,
        status: options?.status ?? undefined,
      },
    });
  }

  async getDocument(
    datasetId: string,
    documentId: string,
    options?: DocumentGetOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    if (options?.metadata) {
      const allowed = new Set(["all", "only", "without"]);
      if (!allowed.has(options.metadata)) {
        throw new ValidationError("metadata must be one of all, only, without");
      }
    }
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents/${documentId}`,
      query: {
        metadata: options?.metadata ?? undefined,
      },
    });
  }

  async deleteDocument(
    datasetId: string,
    documentId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    return this.http.request({
      method: "DELETE",
      path: `/datasets/${datasetId}/documents/${documentId}`,
    });
  }

  async getDocumentIndexingStatus(
    datasetId: string,
    batch: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(batch, "batch");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents/${batch}/indexing-status`,
    });
  }

  async createSegments(
    datasetId: string,
    documentId: string,
    request: SegmentCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyArray(request.segments, "segments");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/${documentId}/segments`,
      data: request,
    });
  }

  async listSegments(
    datasetId: string,
    documentId: string,
    options?: SegmentListOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");
    ensureOptionalString(options?.keyword, "keyword");
    if (options?.status && options.status.length > 0) {
      ensureStringArray(options.status, "status");
    }

    const query: QueryParams = {
      page: options?.page,
      limit: options?.limit,
      keyword: options?.keyword ?? undefined,
    };
    if (options?.status && options.status.length > 0) {
      query.status = options.status;
    }

    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents/${documentId}/segments`,
      query,
    });
  }

  async getSegment(
    datasetId: string,
    documentId: string,
    segmentId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}`,
    });
  }

  async updateSegment(
    datasetId: string,
    documentId: string,
    segmentId: string,
    request: SegmentUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}`,
      data: request,
    });
  }

  async deleteSegment(
    datasetId: string,
    documentId: string,
    segmentId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    return this.http.request({
      method: "DELETE",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}`,
    });
  }

  async createChildChunk(
    datasetId: string,
    documentId: string,
    segmentId: string,
    request: ChildChunkCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    ensureNonEmptyString(request.content, "content");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}/child_chunks`,
      data: request,
    });
  }

  async listChildChunks(
    datasetId: string,
    documentId: string,
    segmentId: string,
    options?: ChildChunkListOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    ensureOptionalInt(options?.page, "page");
    ensureOptionalInt(options?.limit, "limit");
    ensureOptionalString(options?.keyword, "keyword");

    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}/child_chunks`,
      query: {
        page: options?.page,
        limit: options?.limit,
        keyword: options?.keyword ?? undefined,
      },
    });
  }

  async updateChildChunk(
    datasetId: string,
    documentId: string,
    segmentId: string,
    childChunkId: string,
    request: ChildChunkUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    ensureNonEmptyString(childChunkId, "childChunkId");
    ensureNonEmptyString(request.content, "content");
    return this.http.request({
      method: "PATCH",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}/child_chunks/${childChunkId}`,
      data: request,
    });
  }

  async deleteChildChunk(
    datasetId: string,
    documentId: string,
    segmentId: string,
    childChunkId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(documentId, "documentId");
    ensureNonEmptyString(segmentId, "segmentId");
    ensureNonEmptyString(childChunkId, "childChunkId");
    return this.http.request({
      method: "DELETE",
      path: `/datasets/${datasetId}/documents/${documentId}/segments/${segmentId}/child_chunks/${childChunkId}`,
    });
  }

  async listMetadata(
    datasetId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/metadata`,
    });
  }

  async createMetadata(
    datasetId: string,
    request: MetadataCreateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(request.name, "name");
    ensureNonEmptyString(request.type, "type");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/metadata`,
      data: request,
    });
  }

  async updateMetadata(
    datasetId: string,
    metadataId: string,
    request: MetadataUpdateRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(metadataId, "metadataId");
    ensureNonEmptyString(request.name, "name");
    return this.http.request({
      method: "PATCH",
      path: `/datasets/${datasetId}/metadata/${metadataId}`,
      data: request,
    });
  }

  async deleteMetadata(
    datasetId: string,
    metadataId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(metadataId, "metadataId");
    return this.http.request({
      method: "DELETE",
      path: `/datasets/${datasetId}/metadata/${metadataId}`,
    });
  }

  async listBuiltInMetadata(
    datasetId: string
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/metadata/built-in`,
    });
  }

  async updateBuiltInMetadata(
    datasetId: string,
    action: "enable" | "disable"
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(action, "action");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/metadata/built-in/${action}`,
    });
  }

  async updateDocumentsMetadata(
    datasetId: string,
    request: MetadataOperationRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyArray(request.operation_data, "operation_data");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/documents/metadata`,
      data: request,
    });
  }

  async hitTesting(
    datasetId: string,
    request: HitTestingRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    if (request.query !== undefined && request.query !== null) {
      ensureOptionalString(request.query, "query");
    }
    if (request.attachment_ids && request.attachment_ids.length > 0) {
      ensureStringArray(request.attachment_ids, "attachment_ids");
    }
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/hit-testing`,
      data: request,
    });
  }

  async retrieve(
    datasetId: string,
    request: HitTestingRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    return this.http.request({
      method: "POST",
      path: `/datasets/${datasetId}/retrieve`,
      data: request,
    });
  }

  async listDatasourcePlugins(
    datasetId: string,
    options?: DatasourcePluginListOptions
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureOptionalBoolean(options?.isPublished, "isPublished");
    return this.http.request({
      method: "GET",
      path: `/datasets/${datasetId}/pipeline/datasource-plugins`,
      query: {
        is_published: options?.isPublished ?? undefined,
      },
    });
  }

  async runDatasourceNode(
    datasetId: string,
    nodeId: string,
    request: DatasourceNodeRunRequest
  ): Promise<DifyStream<PipelineStreamEvent>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(nodeId, "nodeId");
    ensureNonEmptyString(request.datasource_type, "datasource_type");
    return this.http.requestStream<PipelineStreamEvent>({
      method: "POST",
      path: `/datasets/${datasetId}/pipeline/datasource/nodes/${nodeId}/run`,
      data: request,
    });
  }

  async runPipeline(
    datasetId: string,
    request: PipelineRunRequest
  ): Promise<DifyResponse<KnowledgeBaseResponse> | DifyStream<PipelineStreamEvent>> {
    ensureNonEmptyString(datasetId, "datasetId");
    ensureNonEmptyString(request.datasource_type, "datasource_type");
    ensureNonEmptyString(request.start_node_id, "start_node_id");
    const shouldStream = request.response_mode === "streaming";
    if (shouldStream) {
      return this.http.requestStream<PipelineStreamEvent>({
        method: "POST",
        path: `/datasets/${datasetId}/pipeline/run`,
        data: request,
      });
    }
    return this.http.request<KnowledgeBaseResponse>({
      method: "POST",
      path: `/datasets/${datasetId}/pipeline/run`,
      data: request,
    });
  }

  async uploadPipelineFile(
    form: unknown
  ): Promise<DifyResponse<KnowledgeBaseResponse>> {
    ensureFormData(form, "uploadPipelineFile");
    return this.http.request({
      method: "POST",
      path: "/datasets/pipeline/file-upload",
      data: form,
    });
  }
}

```

### sdks/nodejs-client/src/client/workflow.test.ts
```ts
import { beforeEach, describe, expect, it, vi } from "vitest";
import { WorkflowClient } from "./workflow";
import { createHttpClientWithSpies } from "../../tests/test-utils";

describe("WorkflowClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("runs workflows with blocking and streaming modes", async () => {
    const { client, request, requestStream } = createHttpClientWithSpies();
    const workflow = new WorkflowClient(client);

    await workflow.run({ inputs: { input: "x" }, user: "user" });
    await workflow.run({ input: "x" }, "user", true);

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/run",
      data: {
        inputs: { input: "x" },
        user: "user",
      },
    });
    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/run",
      data: {
        inputs: { input: "x" },
        user: "user",
        response_mode: "streaming",
      },
    });
  });

  it("runs workflow by id", async () => {
    const { client, request, requestStream } = createHttpClientWithSpies();
    const workflow = new WorkflowClient(client);

    await workflow.runById("wf", {
      inputs: { input: "x" },
      user: "user",
      response_mode: "blocking",
    });
    await workflow.runById("wf", {
      inputs: { input: "x" },
      user: "user",
      response_mode: "streaming",
    });

    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/wf/run",
      data: {
        inputs: { input: "x" },
        user: "user",
        response_mode: "blocking",
      },
    });
    expect(requestStream).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/wf/run",
      data: {
        inputs: { input: "x" },
        user: "user",
        response_mode: "streaming",
      },
    });
  });

  it("gets run details and stops workflow", async () => {
    const { client, request } = createHttpClientWithSpies();
    const workflow = new WorkflowClient(client);

    await workflow.getRun("run");
    await workflow.stop("task", "user");

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/workflows/run/run",
    });
    expect(request).toHaveBeenCalledWith({
      method: "POST",
      path: "/workflows/tasks/task/stop",
      data: { user: "user" },
    });
  });

  it("fetches workflow logs", async () => {
    const { client, request } = createHttpClientWithSpies();
    const workflow = new WorkflowClient(client);

    await workflow.getLogs({
      keyword: "k",
      status: "succeeded",
      startTime: "2024-01-01",
      endTime: "2024-01-02",
      createdByEndUserSessionId: "session-123",
      page: 1,
      limit: 20,
    });

    expect(request).toHaveBeenCalledWith({
      method: "GET",
      path: "/workflows/logs",
      query: {
        keyword: "k",
        status: "succeeded",
        created_at__before: "2024-01-02",
        created_at__after: "2024-01-01",
        created_by_end_user_session_id: "session-123",
        created_by_account: undefined,
        page: 1,
        limit: 20,
      },
    });
  });
});

```

### sdks/nodejs-client/src/client/completion.ts
```ts
import { DifyClient } from "./base";
import type { CompletionRequest, CompletionResponse } from "../types/completion";
import type {
  DifyResponse,
  DifyStream,
  JsonObject,
  SuccessResponse,
} from "../types/common";
import { ensureNonEmptyString } from "./validation";

const warned = new Set<string>();
const warnOnce = (message: string): void => {
  if (warned.has(message)) {
    return;
  }
  warned.add(message);
  console.warn(message);
};

export class CompletionClient extends DifyClient {
  createCompletionMessage(
    request: CompletionRequest
  ): Promise<DifyResponse<CompletionResponse> | DifyStream<CompletionResponse>>;
  createCompletionMessage(
    inputs: JsonObject,
    user: string,
    stream?: boolean,
    files?: CompletionRequest["files"]
  ): Promise<DifyResponse<CompletionResponse> | DifyStream<CompletionResponse>>;
  createCompletionMessage(
    inputOrRequest: CompletionRequest | JsonObject,
    user?: string,
    stream = false,
    files?: CompletionRequest["files"]
  ): Promise<DifyResponse<CompletionResponse> | DifyStream<CompletionResponse>> {
    let payload: CompletionRequest;
    let shouldStream = stream;

    if (user === undefined && "user" in (inputOrRequest as CompletionRequest)) {
      payload = inputOrRequest as CompletionRequest;
      shouldStream = payload.response_mode === "streaming";
    } else {
      ensureNonEmptyString(user, "user");
      payload = {
        inputs: inputOrRequest,
        user,
        files,
        response_mode: stream ? "streaming" : "blocking",
      };
    }

    ensureNonEmptyString(payload.user, "user");

    if (shouldStream) {
      return this.http.requestStream<CompletionResponse>({
        method: "POST",
        path: "/completion-messages",
        data: payload,
      });
    }

    return this.http.request<CompletionResponse>({
      method: "POST",
      path: "/completion-messages",
      data: payload,
    });
  }

  stopCompletionMessage(
    taskId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    ensureNonEmptyString(taskId, "taskId");
    ensureNonEmptyString(user, "user");
    return this.http.request<SuccessResponse>({
      method: "POST",
      path: `/completion-messages/${taskId}/stop`,
      data: { user },
    });
  }

  stop(
    taskId: string,
    user: string
  ): Promise<DifyResponse<SuccessResponse>> {
    return this.stopCompletionMessage(taskId, user);
  }

  runWorkflow(
    inputs: JsonObject,
    user: string,
    stream = false
  ): Promise<DifyResponse<JsonObject> | DifyStream<JsonObject>> {
    warnOnce(
      "CompletionClient.runWorkflow is deprecated. Use WorkflowClient.run instead."
    );
    ensureNonEmptyString(user, "user");
    const payload = {
      inputs,
      user,
      response_mode: stream ? "streaming" : "blocking",
    };
    if (stream) {
      return this.http.requestStream<JsonObject>({
        method: "POST",
        path: "/workflows/run",
        data: payload,
      });
    }
    return this.http.request<JsonObject>({
      method: "POST",
      path: "/workflows/run",
      data: payload,
    });
  }
}

```

### sdks/nodejs-client/src/client/validation.test.ts
```ts
import { describe, expect, it } from "vitest";
import {
  ensureNonEmptyString,
  ensureOptionalBoolean,
  ensureOptionalInt,
  ensureOptionalString,
  ensureOptionalStringArray,
  ensureRating,
  ensureStringArray,
  validateParams,
} from "./validation";

const makeLongString = (length: number) => "a".repeat(length);

describe("validation utilities", () => {
  it("ensureNonEmptyString throws on empty or whitespace", () => {
    expect(() => ensureNonEmptyString("", "name")).toThrow();
    expect(() => ensureNonEmptyString("   ", "name")).toThrow();
  });

  it("ensureNonEmptyString throws on overly long strings", () => {
    expect(() => ensureNonEmptyString(makeLongString(10001), "name")).toThrow();
  });

  it("ensureOptionalString ignores undefined and validates when set", () => {
    expect(() => ensureOptionalString(undefined, "opt")).not.toThrow();
    expect(() => ensureOptionalString("", "opt")).toThrow();
  });

  it("ensureOptionalString throws on overly long strings", () => {
    expect(() => ensureOptionalString(makeLongString(10001), "opt")).toThrow();
  });

  it("ensureOptionalInt validates integer", () => {
    expect(() => ensureOptionalInt(undefined, "limit")).not.toThrow();
    expect(() => ensureOptionalInt(1.2, "limit")).toThrow();
  });

  it("ensureOptionalBoolean validates boolean", () => {
    expect(() => ensureOptionalBoolean(undefined, "flag")).not.toThrow();
    expect(() => ensureOptionalBoolean("yes", "flag")).toThrow();
  });

  it("ensureStringArray enforces size and content", () => {
    expect(() => ensureStringArray([], "items")).toThrow();
    expect(() => ensureStringArray([""], "items")).toThrow();
    expect(() =>
      ensureStringArray(Array.from({ length: 1001 }, () => "a"), "items")
    ).toThrow();
    expect(() => ensureStringArray(["ok"], "items")).not.toThrow();
  });

  it("ensureOptionalStringArray ignores undefined", () => {
    expect(() => ensureOptionalStringArray(undefined, "tags")).not.toThrow();
  });

  it("ensureOptionalStringArray validates when set", () => {
    expect(() => ensureOptionalStringArray(["valid"], "tags")).not.toThrow();
    expect(() => ensureOptionalStringArray([], "tags")).toThrow();
    expect(() => ensureOptionalStringArray([""], "tags")).toThrow();
  });

  it("ensureRating validates allowed values", () => {
    expect(() => ensureRating(undefined)).not.toThrow();
    expect(() => ensureRating("like")).not.toThrow();
    expect(() => ensureRating("bad")).toThrow();
  });

  it("validateParams enforces generic rules", () => {
    expect(() => validateParams({ user: 123 })).toThrow();
    expect(() => validateParams({ rating: "bad" })).toThrow();
    expect(() => validateParams({ page: 1.1 })).toThrow();
    expect(() => validateParams({ files: "bad" })).toThrow();
    expect(() => validateParams({ keyword: "" })).not.toThrow();
    expect(() => validateParams({ name: makeLongString(10001) })).toThrow();
    expect(() =>
      validateParams({ items: Array.from({ length: 1001 }, () => "a") })
    ).toThrow();
    expect(() =>
      validateParams({
        data: Object.fromEntries(
          Array.from({ length: 101 }, (_, i) => [String(i), i])
        ),
      })
    ).toThrow();
    expect(() => validateParams({ user: "u", page: 1 })).not.toThrow();
  });
});

```

### sdks/nodejs-client/src/http/retry.test.ts
```ts
import { describe, expect, it } from "vitest";
import { getRetryDelayMs, shouldRetry } from "./retry";
import { NetworkError, RateLimitError, TimeoutError } from "../errors/dify-error";

const withMockedRandom = (value: number, fn: () => void): void => {
  const original = Math.random;
  Math.random = () => value;
  try {
    fn();
  } finally {
    Math.random = original;
  }
};

describe("retry helpers", () => {
  it("getRetryDelayMs honors retry-after header", () => {
    expect(getRetryDelayMs(1, 1, 3)).toBe(3000);
  });

  it("getRetryDelayMs uses exponential backoff with jitter", () => {
    withMockedRandom(0, () => {
      expect(getRetryDelayMs(1, 1)).toBe(1000);
      expect(getRetryDelayMs(2, 1)).toBe(2000);
      expect(getRetryDelayMs(3, 1)).toBe(4000);
    });
  });

  it("shouldRetry respects max retries", () => {
    expect(shouldRetry(new TimeoutError("timeout"), 3, 3)).toBe(false);
  });

  it("shouldRetry retries on network, timeout, and rate limit", () => {
    expect(shouldRetry(new TimeoutError("timeout"), 0, 3)).toBe(true);
    expect(shouldRetry(new NetworkError("network"), 0, 3)).toBe(true);
    expect(shouldRetry(new RateLimitError("limit"), 0, 3)).toBe(true);
    expect(shouldRetry(new Error("other"), 0, 3)).toBe(false);
  });
});

```

### sdks/nodejs-client/src/http/form-data.ts
```ts
import type { Headers } from "../types/common";

type FormDataAppendValue = Blob | string;

export type WebFormData = FormData;

export type LegacyNodeFormData = {
  append: (name: string, value: FormDataAppendValue, fileName?: string) => void;
  getHeaders: () => Headers;
  constructor?: { name?: string };
};

export type SdkFormData = WebFormData | LegacyNodeFormData;

export const isFormData = (value: unknown): value is SdkFormData => {
  if (!value || typeof value !== "object") {
    return false;
  }
  if (typeof FormData !== "undefined" && value instanceof FormData) {
    return true;
  }
  const candidate = value as Partial<LegacyNodeFormData>;
  if (typeof candidate.append !== "function") {
    return false;
  }
  if (typeof candidate.getHeaders === "function") {
    return true;
  }
  return candidate.constructor?.name === "FormData";
};

export const getFormDataHeaders = (form: SdkFormData): Headers => {
  if ("getHeaders" in form && typeof form.getHeaders === "function") {
    return form.getHeaders();
  }
  return {};
};

```

### sdks/nodejs-client/src/http/retry.ts
```ts
import { RateLimitError, NetworkError, TimeoutError } from "../errors/dify-error";

export const sleep = (ms: number): Promise<void> =>
  new Promise((resolve) => {
    setTimeout(resolve, ms);
  });

export const getRetryDelayMs = (
  attempt: number,
  retryDelaySeconds: number,
  retryAfterSeconds?: number
): number => {
  if (retryAfterSeconds && retryAfterSeconds > 0) {
    return retryAfterSeconds * 1000;
  }
  const base = retryDelaySeconds * 1000;
  const exponential = base * Math.pow(2, Math.max(0, attempt - 1));
  const jitter = Math.random() * base;
  return exponential + jitter;
};

export const shouldRetry = (
  error: unknown,
  attempt: number,
  maxRetries: number
): boolean => {
  if (attempt >= maxRetries) {
    return false;
  }
  if (error instanceof TimeoutError) {
    return true;
  }
  if (error instanceof NetworkError) {
    return true;
  }
  if (error instanceof RateLimitError) {
    return true;
  }
  return false;
};

```

### sdks/nodejs-client/src/http/sse.ts
```ts
import type { Readable } from "node:stream";
import { StringDecoder } from "node:string_decoder";
import type {
  BinaryStream,
  DifyStream,
  Headers,
  JsonValue,
  StreamEvent,
} from "../types/common";
import { isRecord } from "../internal/type-guards";

const toBufferChunk = (chunk: unknown): Buffer => {
  if (Buffer.isBuffer(chunk)) {
    return chunk;
  }
  if (chunk instanceof Uint8Array) {
    return Buffer.from(chunk);
  }
  return Buffer.from(String(chunk));
};

const readLines = async function* (stream: Readable): AsyncIterable<string> {
  const decoder = new StringDecoder("utf8");
  let buffered = "";
  for await (const chunk of stream) {
    buffered += decoder.write(toBufferChunk(chunk));
    let index = buffered.indexOf("\n");
    while (index >= 0) {
      let line = buffered.slice(0, index);
      buffered = buffered.slice(index + 1);
      if (line.endsWith("\r")) {
        line = line.slice(0, -1);
      }
      yield line;
      index = buffered.indexOf("\n");
    }
  }
  buffered += decoder.end();
  if (buffered) {
    yield buffered;
  }
};

const parseMaybeJson = (value: string): JsonValue | string | null => {
  if (!value) {
    return null;
  }
  try {
    return JSON.parse(value) as JsonValue;
  } catch {
    return value;
  }
};

export const parseSseStream = async function* <T>(
  stream: Readable
): AsyncIterable<StreamEvent<T>> {
  let eventName: string | undefined;
  const dataLines: string[] = [];

  const emitEvent = function* (): Iterable<StreamEvent<T>> {
    if (!eventName && dataLines.length === 0) {
      return;
    }
    const raw = dataLines.join("\n");
    const parsed = parseMaybeJson(raw) as T | string | null;
    yield {
      event: eventName,
      data: parsed,
      raw,
    };
    eventName = undefined;
    dataLines.length = 0;
  };

  for await (const line of readLines(stream)) {
    if (!line) {
      yield* emitEvent();
      continue;
    }
    if (line.startsWith(":")) {
      continue;
    }
    if (line.startsWith("event:")) {
      eventName = line.slice("event:".length).trim();
      continue;
    }
    if (line.startsWith("data:")) {
      dataLines.push(line.slice("data:".length).trimStart());
      continue;
    }
  }

  yield* emitEvent();
};

const extractTextFromEvent = (data: unknown): string => {
  if (typeof data === "string") {
    return data;
  }
  if (!isRecord(data)) {
    return "";
  }
  if (typeof data.answer === "string") {
    return data.answer;
  }
  if (typeof data.text === "string") {
    return data.text;
  }
  if (typeof data.delta === "string") {
    return data.delta;
  }
  return "";
};

export const createSseStream = <T>(
  stream: Readable,
  meta: { status: number; headers: Headers; requestId?: string }
): DifyStream<T> => {
  const iterator = parseSseStream<T>(stream)[Symbol.asyncIterator]();
  const iterable = {
    [Symbol.asyncIterator]: () => iterator,
    data: stream,
    status: meta.status,
    headers: meta.headers,
    requestId: meta.requestId,
    toReadable: () => stream,
    toText: async () => {
      let text = "";
      for await (const event of iterable) {
        text += extractTextFromEvent(event.data);
      }
      return text;
    },
  } satisfies DifyStream<T>;

  return iterable;
};

export const createBinaryStream = (
  stream: Readable,
  meta: { status: number; headers: Headers; requestId?: string }
): BinaryStream => ({
  data: stream,
  status: meta.status,
  headers: meta.headers,
  requestId: meta.requestId,
  toReadable: () => stream,
});

```

### sdks/nodejs-client/src/http/client.ts
```ts
import { Readable } from "node:stream";
import {
  DEFAULT_BASE_URL,
  DEFAULT_MAX_RETRIES,
  DEFAULT_RETRY_DELAY_SECONDS,
  DEFAULT_TIMEOUT_SECONDS,
} from "../types/common";
import type {
  BinaryStream,
  DifyClientConfig,
  DifyResponse,
  DifyStream,
  Headers,
  JsonValue,
  QueryParams,
  RequestMethod,
} from "../types/common";
import {
  APIError,
  AuthenticationError,
  DifyError,
  FileUploadError,
  NetworkError,
  RateLimitError,
  TimeoutError,
  ValidationError,
} from "../errors/dify-error";
import type { SdkFormData } from "./form-data";
import { getFormDataHeaders, isFormData } from "./form-data";
import { createBinaryStream, createSseStream } from "./sse";
import { getRetryDelayMs, shouldRetry, sleep } from "./retry";
import { validateParams } from "../client/validation";
import { hasStringProperty, isRecord } from "../internal/type-guards";

const DEFAULT_USER_AGENT = "dify-client-node";

export type HttpResponseType = "json" | "bytes" | "stream" | "arraybuffer";

export type HttpRequestBody =
  | JsonValue
  | Readable
  | SdkFormData
  | URLSearchParams
  | ArrayBuffer
  | ArrayBufferView
  | Blob
  | string
  | null;

export type ResponseDataFor<TResponseType extends HttpResponseType> =
  TResponseType extends "stream"
    ? Readable
    : TResponseType extends "bytes" | "arraybuffer"
      ? Buffer
      : JsonValue | string | null;

export type RawHttpResponse<TData = unknown> = {
  data: TData;
  status: number;
  headers: Headers;
  requestId?: string;
  url: string;
};

export type RequestOptions<TResponseType extends HttpResponseType = "json"> = {
  method: RequestMethod;
  path: string;
  query?: QueryParams;
  data?: HttpRequestBody;
  headers?: Headers;
  responseType?: TResponseType;
};

export type HttpClientSettings = Required<
  Omit<DifyClientConfig, "apiKey">
> & {
  apiKey: string;
};

type FetchRequestInit = RequestInit & {
  duplex?: "half";
};

type PreparedRequestBody = {
  body?: BodyInit | null;
  headers: Headers;
  duplex?: "half";
  replayable: boolean;
};

type TimeoutContext = {
  cleanup: () => void;
  reason: Error;
  signal: AbortSignal;
};

const normalizeSettings = (config: DifyClientConfig): HttpClientSettings => ({
  apiKey: config.apiKey,
  baseUrl: config.baseUrl ?? DEFAULT_BASE_URL,
  timeout: config.timeout ?? DEFAULT_TIMEOUT_SECONDS,
  maxRetries: config.maxRetries ?? DEFAULT_MAX_RETRIES,
  retryDelay: config.retryDelay ?? DEFAULT_RETRY_DELAY_SECONDS,
  enableLogging: config.enableLogging ?? false,
});

const normalizeHeaders = (headers: globalThis.Headers): Headers => {
  const result: Headers = {};
  headers.forEach((value, key) => {
    result[key.toLowerCase()] = value;
  });
  return result;
};

const resolveRequestId = (headers: Headers): string | undefined =>
  headers["x-request-id"] ?? headers["x-requestid"];

const buildRequestUrl = (
  baseUrl: string,
  path: string,
  query?: QueryParams
): string => {
  const trimmed = baseUrl.replace(/\/+$/, "");
  const url = new URL(`${trimmed}${path}`);
  const queryString = buildQueryString(query);
  if (queryString) {
    url.search = queryString;
  }
  return url.toString();
};

const buildQueryString = (params?: QueryParams): string => {
  if (!params) {
    return "";
  }
  const searchParams = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value === undefined || value === null) {
      return;
    }
    if (Array.isArray(value)) {
      value.forEach((item) => {
        searchParams.append(key, String(item));
      });
      return;
    }
    searchParams.append(key, String(value));
  });
  return searchParams.toString();
};

const parseRetryAfterSeconds = (headerValue?: string): number | undefined => {
  if (!headerValue) {
    return undefined;
  }
  const asNumber = Number.parseInt(headerValue, 10);
  if (!Number.isNaN(asNumber)) {
    return asNumber;
  }
  const asDate = Date.parse(headerValue);
  if (!Number.isNaN(asDate)) {
    const diff = asDate - Date.now();
    return diff > 0 ? Math.ceil(diff / 1000) : 0;
  }
  return undefined;
};

const isPipeableStream = (value: unknown): value is { pipe: (destination: unknown) => unknown } => {
  if (!value || typeof value !== "object") {
    return false;
  }
  return typeof (value as { pipe?: unknown }).pipe === "function";
};

const toNodeReadable = (value: unknown): Readable | null => {
  if (value instanceof Readable) {
    return value;
  }
  if (!isPipeableStream(value)) {
    return null;
  }
  const readable = new Readable({
    read() {},
  });
  return readable.wrap(value as NodeJS.ReadableStream);
};

const isBinaryBody = (
  value: unknown
): value is ArrayBuffer | ArrayBufferView | Blob => {
  if (value instanceof Blob) {
    return true;
  }
  if (value instanceof ArrayBuffer) {
    return true;
  }
  return ArrayBuffer.isView(value);
};

const isJsonBody = (value: unknown): value is Exclude<JsonValue, string> =>
  value === null ||
  typeof value === "boolean" ||
  typeof value === "number" ||
  Array.isArray(value) ||
  isRecord(value);

const isUploadLikeRequest = (path: string): boolean => {
  const normalizedPath = path.toLowerCase();
  return (
    normalizedPath.includes("upload") ||
    normalizedPath.includes("/files/") ||
    normalizedPath.includes("audio-to-text") ||
    normalizedPath.includes("create_by_file") ||
    normalizedPath.includes("update_by_file")
  );
};

const resolveErrorMessage = (status: number, responseBody: unknown): string => {
  if (typeof responseBody === "string" && responseBody.trim().length > 0) {
    return responseBody;
  }
  if (hasStringProperty(responseBody, "message")) {
    const message = responseBody.message.trim();
    if (message.length > 0) {
      return message;
    }
  }
  return `Request failed with status code ${status}`;
};

const parseJsonLikeText = (
  value: string,
  contentType?: string | null
): JsonValue | string | null => {
  if (value.length === 0) {
    return null;
  }
  const shouldParseJson =
    contentType?.includes("application/json") === true ||
    contentType?.includes("+json") === true;
  if (!shouldParseJson) {
    try {
      return JSON.parse(value) as JsonValue;
    } catch {
      return value;
    }
  }
  return JSON.parse(value) as JsonValue;
};

const prepareRequestBody = (
  method: RequestMethod,
  data: HttpRequestBody | undefined
): PreparedRequestBody => {
  if (method === "GET" || data === undefined) {
    return {
      body: undefined,
      headers: {},
      replayable: true,
    };
  }

  if (isFormData(data)) {
    if ("getHeaders" in data && typeof data.getHeaders === "function") {
      const readable = toNodeReadable(data);
      if (!readable) {
        throw new FileUploadError(
          "Legacy FormData must be a readable stream when used with fetch"
        );
      }
      return {
        body: Readable.toWeb(readable) as BodyInit,
        headers: getFormDataHeaders(data),
        duplex: "half",
        replayable: false,
      };
    }
    return {
      body: data as BodyInit,
      headers: getFormDataHeaders(data),
      replayable: true,
    };
  }

  if (typeof data === "string") {
    return {
      body: data,
      headers: {},
      replayable: true,
    };
  }

  const readable = toNodeReadable(data);
  if (readable) {
    return {
      body: Readable.toWeb(readable) as BodyInit,
      headers: {},
      duplex: "half",
      replayable: false,
    };
  }

  if (data instanceof URLSearchParams || isBinaryBody(data)) {
    const body =
      ArrayBuffer.isView(data) && !(data instanceof Uint8Array)
        ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength)
        : data;
    return {
      body: body as BodyInit,
      headers: {},
      replayable: true,
    };
  }

  if (isJsonBody(data)) {
    return {
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
      replayable: true,
    };
  }

  throw new ValidationError("Unsupported request body type");
};

const createTimeoutContext = (timeoutMs: number): TimeoutContext => {
  const controller = new AbortController();
  const reason = new Error("Request timed out");
  const timer = setTimeout(() => {
    controller.abort(reason);
  }, timeoutMs);
  return {
    signal: controller.signal,
    reason,
    cleanup: () => {
      clearTimeout(timer);
    },
  };
};

const parseResponseBody = async <TResponseType extends HttpResponseType>(
  response: Response,
  responseType: TResponseType
): Promise<ResponseDataFor<TResponseType>> => {
  if (responseType === "stream") {
    if (!response.body) {
      throw new NetworkError("Response body is empty");
    }
    return Readable.fromWeb(
      response.body as unknown as Parameters<typeof Readable.fromWeb>[0]
    ) as ResponseDataFor<TResponseType>;
  }

  if (responseType === "bytes" || responseType === "arraybuffer") {
    const bytes = Buffer.from(await response.arrayBuffer());
    return bytes as ResponseDataFor<TResponseType>;
  }

  if (response.status === 204 || response.status === 205 || response.status === 304) {
    return null as ResponseDataFor<TResponseType>;
  }

  const text = await response.text();
  try {
    return parseJsonLikeText(
      text,
      response.headers.get("content-type")
    ) as ResponseDataFor<TResponseType>;
  } catch (error) {
    if (!response.ok && error instanceof SyntaxError) {
      return text as ResponseDataFor<TResponseType>;
    }
    throw error;
  }
};

const mapHttpError = (
  response: RawHttpResponse,
  path: string
): DifyError => {
  const status = response.status;
  const responseBody = response.data;
  const message = resolveErrorMessage(status, responseBody);

  if (status === 401) {
    return new AuthenticationError(message, {
      statusCode: status,
      responseBody,
      requestId: response.requestId,
    });
  }

  if (status === 429) {
    const retryAfter = parseRetryAfterSeconds(response.headers["retry-after"]);
    return new RateLimitError(message, {
      statusCode: status,
      responseBody,
      requestId: response.requestId,
      retryAfter,
    });
  }

  if (status === 422) {
    return new ValidationError(message, {
      statusCode: status,
      responseBody,
      requestId: response.requestId,
    });
  }

  if (status === 400 && isUploadLikeRequest(path)) {
    return new FileUploadError(message, {
      statusCode: status,
      responseBody,
      requestId: response.requestId,
    });
  }

  return new APIError(message, {
    statusCode: status,
    responseBody,
    requestId: response.requestId,
  });
};

const mapTransportError = (
  error: unknown,
  timeoutContext: TimeoutContext
): DifyError => {
  if (error instanceof DifyError) {
    return error;
  }

  if (
    timeoutContext.signal.aborted &&
    timeoutContext.signal.reason === timeoutContext.reason
  ) {
    return new TimeoutError("Request timed out", { cause: error });
  }

  if (error instanceof Error) {
    if (error.name === "AbortError" || error.name === "TimeoutError") {
      return new TimeoutError("Request timed out", { cause: error });
    }
    return new NetworkError(error.message, { cause: error });
  }

  return new NetworkError("Unexpected network error", { cause: error });
};

export class HttpClient {
  private settings: HttpClientSettings;

  constructor(config: DifyClientConfig) {
    this.settings = normalizeSettings(config);
  }

  updateApiKey(apiKey: string): void {
    this.settings.apiKey = apiKey;
  }

  getSettings(): HttpClientSettings {
    return { ...this.settings };
  }

  async request<
    T,
    TResponseType extends HttpResponseType = "json",
  >(options: RequestOptions<TResponseType>): Promise<DifyResponse<T>> {
    const response = await this.requestRaw(options);
    return {
      data: response.data as T,
      status: response.status,
      headers: response.headers,
      requestId: response.requestId,
    };
  }

  async requestStream<T>(options: RequestOptions): Promise<DifyStream<T>> {
    const response = await this.requestRaw({
      ...options,
      responseType: "stream",
    });
    return createSseStream<T>(response.data, {
      status: response.status,
      headers: response.headers,
      requestId: response.requestId,
    });
  }

  async requestBinaryStream(options: RequestOptions): Promise<BinaryStream> {
    const response = await this.requestRaw({
      ...options,
      responseType: "stream",
    });
    return createBinaryStream(response.data, {
      status: response.status,
      headers: response.headers,
      requestId: response.requestId,
    });
  }

  async requestRaw<TResponseType extends HttpResponseType = "json">(
    options: RequestOptions<TResponseType>
  ): Promise<RawHttpResponse<ResponseDataFor<TResponseType>>> {
    const responseType = options.responseType ?? "json";
    const { method, path, query, data, headers } = options;
    const { apiKey, enableLogging, maxRetries, retryDelay, timeout } = this.settings;

    if (query) {
      validateParams(query as Record<string, unknown>);
    }

    if (isRecord(data) && !Array.isArray(data) && !isFormData(data) && !isPipeableStream(data)) {
      validateParams(data);
    }

    const url = buildRequestUrl(this.settings.baseUrl, path, query);

    if (enableLogging) {
      console.info(`dify-client-node request ${method} ${url}`);
    }

    let attempt = 0;
    while (true) {
      const preparedBody = prepareRequestBody(method, data);
      const requestHeaders: Headers = {
        Authorization: `Bearer ${apiKey}`,
        ...preparedBody.headers,
        ...headers,
      };

      if (
        typeof process !== "undefined" &&
        !!process.versions?.node &&
        !requestHeaders["User-Agent"] &&
        !requestHeaders["user-agent"]
      ) {
        requestHeaders["User-Agent"] = DEFAULT_USER_AGENT;
      }

      const timeoutContext = createTimeoutContext(timeout * 1000);
      const requestInit: FetchRequestInit = {
        method,
        headers: requestHeaders,
        body: preparedBody.body,
        signal: timeoutContext.signal,
      };

      if (preparedBody.duplex) {
        requestInit.duplex = preparedBody.duplex;
      }

      try {
        const fetchResponse = await fetch(url, requestInit);
        const responseHeaders = normalizeHeaders(fetchResponse.headers);
        const parsedBody =
          (await parseResponseBody(fetchResponse, responseType)) as ResponseDataFor<TResponseType>;
        const response: RawHttpResponse<ResponseDataFor<TResponseType>> = {
          data: parsedBody,
          status: fetchResponse.status,
          headers: responseHeaders,
          requestId: resolveRequestId(responseHeaders),
          url,
        };

        if (!fetchResponse.ok) {
          throw mapHttpError(response, path);
        }

        if (enableLogging) {
          console.info(
            `dify-client-node response ${response.status} ${method} ${url}`
          );
        }

        return response;
      } catch (error) {
        const mapped = mapTransportError(error, timeoutContext);
        const shouldRetryRequest =
          preparedBody.replayable && shouldRetry(mapped, attempt, maxRetries);
        if (!shouldRetryRequest) {
          throw mapped;
        }
        const retryAfterSeconds =
          mapped instanceof RateLimitError ? mapped.retryAfter : undefined;
        const delay = getRetryDelayMs(attempt + 1, retryDelay, retryAfterSeconds);
        if (enableLogging) {
          console.info(
            `dify-client-node retry ${attempt + 1} in ${delay}ms for ${method} ${url}`
          );
        }
        attempt += 1;
        await sleep(delay);
      } finally {
        timeoutContext.cleanup();
      }
    }
  }
}

```

### sdks/nodejs-client/src/http/form-data.test.ts
```ts
import { describe, expect, it, vi } from "vitest";
import { getFormDataHeaders, isFormData } from "./form-data";

describe("form-data helpers", () => {
  it("detects form-data like objects", () => {
    const formLike = {
      append: () => {},
      getHeaders: () => ({ "content-type": "multipart/form-data" }),
    };
    expect(isFormData(formLike)).toBe(true);
    expect(isFormData({})).toBe(false);
  });

  it("detects native FormData", () => {
    const form = new FormData();
    form.append("field", "value");
    expect(isFormData(form)).toBe(true);
  });

  it("returns headers from form-data", () => {
    const formLike = {
      append: vi.fn(),
      getHeaders: () => ({ "content-type": "multipart/form-data" }),
    };
    expect(getFormDataHeaders(formLike)).toEqual({
      "content-type": "multipart/form-data",
    });
  });
});

```

### sdks/nodejs-client/src/http/sse.test.ts
```ts
import { Readable } from "node:stream";
import { describe, expect, it } from "vitest";
import { createBinaryStream, createSseStream, parseSseStream } from "./sse";

describe("sse parsing", () => {
  it("parses event and data lines", async () => {
    const stream = Readable.from([
      "event: message\n",
      'data: {"answer":"hi"}\n',
      "\n",
    ]);
    const events: Array<{ event?: string; data: unknown; raw: string }> = [];
    for await (const event of parseSseStream(stream)) {
      events.push(event);
    }
    expect(events).toHaveLength(1);
    expect(events[0].event).toBe("message");
    expect(events[0].data).toEqual({ answer: "hi" });
  });

  it("handles multi-line data payloads", async () => {
    const stream = Readable.from(["data: line1\n", "data: line2\n", "\n"]);
    const events: Array<{ event?: string; data: unknown; raw: string }> = [];
    for await (const event of parseSseStream(stream)) {
      events.push(event);
    }
    expect(events[0].raw).toBe("line1\nline2");
    expect(events[0].data).toBe("line1\nline2");
  });

  it("ignores comments and flushes the last event without a trailing separator", async () => {
    const stream = Readable.from([
      Buffer.from(": keep-alive\n"),
      Uint8Array.from(Buffer.from('event: message\ndata: {"delta":"hi"}\n')),
    ]);
    const events: Array<{ event?: string; data: unknown; raw: string }> = [];
    for await (const event of parseSseStream(stream)) {
      events.push(event);
    }
    expect(events).toEqual([
      {
        event: "message",
        data: { delta: "hi" },
        raw: '{"delta":"hi"}',
      },
    ]);
  });

  it("createSseStream exposes toText", async () => {
    const stream = Readable.from([
      'data: {"answer":"hello"}\n\n',
      'data: {"delta":" world"}\n\n',
    ]);
    const sseStream = createSseStream(stream, {
      status: 200,
      headers: {},
      requestId: "req",
    });
    const text = await sseStream.toText();
    expect(text).toBe("hello world");
  });

  it("toText extracts text from string data", async () => {
    const stream = Readable.from(["data: plain text\n\n"]);
    const sseStream = createSseStream(stream, { status: 200, headers: {} });
    const text = await sseStream.toText();
    expect(text).toBe("plain text");
  });

  it("toText extracts text field from object", async () => {
    const stream = Readable.from(['data: {"text":"hello"}\n\n']);
    const sseStream = createSseStream(stream, { status: 200, headers: {} });
    const text = await sseStream.toText();
    expect(text).toBe("hello");
  });

  it("toText returns empty for invalid data", async () => {
    const stream = Readable.from(["data: null\n\n", "data: 123\n\n"]);
    const sseStream = createSseStream(stream, { status: 200, headers: {} });
    const text = await sseStream.toText();
    expect(text).toBe("");
  });

  it("createBinaryStream exposes metadata", () => {
    const stream = Readable.from(["chunk"]);
    const binary = createBinaryStream(stream, {
      status: 200,
      headers: { "content-type": "audio/mpeg" },
      requestId: "req",
    });
    expect(binary.status).toBe(200);
    expect(binary.headers["content-type"]).toBe("audio/mpeg");
    expect(binary.toReadable()).toBe(stream);
  });
});

```

### sdks/nodejs-client/src/http/client.test.ts
```ts
import { Readable, Stream } from "node:stream";
import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  APIError,
  AuthenticationError,
  FileUploadError,
  NetworkError,
  RateLimitError,
  TimeoutError,
  ValidationError,
} from "../errors/dify-error";
import { HttpClient } from "./client";

const stubFetch = (): ReturnType<typeof vi.fn> => {
  const fetchMock = vi.fn();
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
};

const getFetchCall = (
  fetchMock: ReturnType<typeof vi.fn>,
  index = 0
): [string, RequestInit | undefined] => {
  const call = fetchMock.mock.calls[index];
  if (!call) {
    throw new Error(`Missing fetch call at index ${index}`);
  }
  return call as [string, RequestInit | undefined];
};

const toHeaderRecord = (headers: HeadersInit | undefined): Record<string, string> =>
  Object.fromEntries(new Headers(headers).entries());

const jsonResponse = (
  body: unknown,
  init: ResponseInit = {}
): Response =>
  new Response(JSON.stringify(body), {
    ...init,
    headers: {
      "content-type": "application/json",
      ...(init.headers ?? {}),
    },
  });

const textResponse = (body: string, init: ResponseInit = {}): Response =>
  new Response(body, {
    ...init,
    headers: {
      ...(init.headers ?? {}),
    },
  });

describe("HttpClient", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it("builds requests with auth headers and JSON content type", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      jsonResponse({ ok: true }, { status: 200, headers: { "x-request-id": "req" } })
    );

    const client = new HttpClient({ apiKey: "test" });
    const response = await client.request({
      method: "POST",
      path: "/chat-messages",
      data: { user: "u" },
    });

    expect(response.requestId).toBe("req");
    expect(fetchMock).toHaveBeenCalledTimes(1);
    const [url, init] = getFetchCall(fetchMock);
    expect(url).toBe("https://api.dify.ai/v1/chat-messages");
    expect(toHeaderRecord(init?.headers)).toMatchObject({
      authorization: "Bearer test",
      "content-type": "application/json",
      "user-agent": "dify-client-node",
    });
    expect(init?.body).toBe(JSON.stringify({ user: "u" }));
  });

  it("serializes array query params", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));

    const client = new HttpClient({ apiKey: "test" });
    await client.requestRaw({
      method: "GET",
      path: "/datasets",
      query: { tag_ids: ["a", "b"], limit: 2 },
    });

    const [url] = getFetchCall(fetchMock);
    expect(new URL(url).searchParams.toString()).toBe(
      "tag_ids=a&tag_ids=b&limit=2"
    );
  });

  it("returns SSE stream helpers", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      new Response('data: {"text":"hi"}\n\n', {
        status: 200,
        headers: { "x-request-id": "req" },
      })
    );

    const client = new HttpClient({ apiKey: "test" });
    const stream = await client.requestStream({
      method: "POST",
      path: "/chat-messages",
      data: { user: "u" },
    });

    expect(stream.status).toBe(200);
    expect(stream.requestId).toBe("req");
    await expect(stream.toText()).resolves.toBe("hi");
  });

  it("returns binary stream helpers", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      new Response("chunk", {
        status: 200,
        headers: { "x-request-id": "req" },
      })
    );

    const client = new HttpClient({ apiKey: "test" });
    const stream = await client.requestBinaryStream({
      method: "POST",
      path: "/text-to-audio",
      data: { user: "u", text: "hi" },
    });

    expect(stream.status).toBe(200);
    expect(stream.requestId).toBe("req");
    expect(stream.data).toBeInstanceOf(Readable);
  });

  it("respects form-data headers", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));

    const client = new HttpClient({ apiKey: "test" });
    const form = new FormData();
    form.append("file", new Blob(["abc"]), "file.txt");

    await client.requestRaw({
      method: "POST",
      path: "/files/upload",
      data: form,
    });

    const [, init] = getFetchCall(fetchMock);
    expect(toHeaderRecord(init?.headers)).toMatchObject({
      authorization: "Bearer test",
    });
    expect(toHeaderRecord(init?.headers)["content-type"]).toBeUndefined();
  });

  it("sends legacy form-data as a readable request body", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));

    const client = new HttpClient({ apiKey: "test" });
    const legacyForm = Object.assign(Readable.from(["chunk"]), {
      append: vi.fn(),
      getHeaders: () => ({
        "content-type": "multipart/form-data; boundary=test",
      }),
    });

    await client.requestRaw({
      method: "POST",
      path: "/files/upload",
      data: legacyForm,
    });

    const [, init] = getFetchCall(fetchMock);
    expect(toHeaderRecord(init?.headers)).toMatchObject({
      authorization: "Bearer test",
      "content-type": "multipart/form-data; boundary=test",
    });
    expect((init as RequestInit & { duplex?: string } | undefined)?.duplex).toBe(
      "half"
    );
    expect(init?.body).not.toBe(legacyForm);
  });

  it("rejects legacy form-data objects that are not readable streams", async () => {
    const fetchMock = stubFetch();
    const client = new HttpClient({ apiKey: "test" });
    const legacyForm = {
      append: vi.fn(),
      getHeaders: () => ({
        "content-type": "multipart/form-data; boundary=test",
      }),
    };

    await expect(
      client.requestRaw({
        method: "POST",
        path: "/files/upload",
        data: legacyForm,
      })
    ).rejects.toBeInstanceOf(FileUploadError);

    expect(fetchMock).not.toHaveBeenCalled();
  });

  it("accepts legacy pipeable streams that are not Readable instances", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));
    const client = new HttpClient({ apiKey: "test" });

    const legacyStream = new Stream() as Stream &
      NodeJS.ReadableStream & {
        append: ReturnType<typeof vi.fn>;
        getHeaders: () => Record<string, string>;
      };
    legacyStream.readable = true;
    legacyStream.pause = () => legacyStream;
    legacyStream.resume = () => legacyStream;
    legacyStream.append = vi.fn();
    legacyStream.getHeaders = () => ({
      "content-type": "multipart/form-data; boundary=test",
    });
    queueMicrotask(() => {
      legacyStream.emit("data", Buffer.from("chunk"));
      legacyStream.emit("end");
    });

    await client.requestRaw({
      method: "POST",
      path: "/files/upload",
      data: legacyStream as unknown as FormData,
    });

    const [, init] = getFetchCall(fetchMock);
    expect((init as RequestInit & { duplex?: string } | undefined)?.duplex).toBe(
      "half"
    );
  });

  it("returns buffers for byte responses", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      new Response(Uint8Array.from([1, 2, 3]), {
        status: 200,
        headers: { "content-type": "application/octet-stream" },
      })
    );

    const client = new HttpClient({ apiKey: "test" });
    const response = await client.request<Buffer, "bytes">({
      method: "GET",
      path: "/files/file-1/preview",
      responseType: "bytes",
    });

    expect(Buffer.isBuffer(response.data)).toBe(true);
    expect(Array.from(response.data.values())).toEqual([1, 2, 3]);
  });

  it("keeps arraybuffer as a backward-compatible binary alias", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      new Response(Uint8Array.from([4, 5, 6]), {
        status: 200,
        headers: { "content-type": "application/octet-stream" },
      })
    );

    const client = new HttpClient({ apiKey: "test" });
    const response = await client.request<Buffer, "arraybuffer">({
      method: "GET",
      path: "/files/file-1/preview",
      responseType: "arraybuffer",
    });

    expect(Buffer.isBuffer(response.data)).toBe(true);
    expect(Array.from(response.data.values())).toEqual([4, 5, 6]);
  });

  it("returns null for empty no-content responses", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));

    const client = new HttpClient({ apiKey: "test" });
    const response = await client.requestRaw({
      method: "GET",
      path: "/meta",
    });

    expect(response.data).toBeNull();
  });

  it("maps 401 and 429 errors", async () => {
    const fetchMock = stubFetch();
    fetchMock
      .mockResolvedValueOnce(
        jsonResponse({ message: "unauthorized" }, { status: 401 })
      )
      .mockResolvedValueOnce(
        jsonResponse({ message: "rate" }, { status: 429, headers: { "retry-after": "2" } })
      );
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toBeInstanceOf(AuthenticationError);

    const error = await client
      .requestRaw({ method: "GET", path: "/meta" })
      .catch((err: unknown) => err);
    expect(error).toBeInstanceOf(RateLimitError);
    expect((error as RateLimitError).retryAfter).toBe(2);
  });

  it("maps validation and upload errors", async () => {
    const fetchMock = stubFetch();
    fetchMock
      .mockResolvedValueOnce(jsonResponse({ message: "invalid" }, { status: 422 }))
      .mockResolvedValueOnce(jsonResponse({ message: "bad upload" }, { status: 400 }));
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "POST", path: "/chat-messages", data: { user: "u" } })
    ).rejects.toBeInstanceOf(ValidationError);

    await expect(
      client.requestRaw({ method: "POST", path: "/files/upload", data: { user: "u" } })
    ).rejects.toBeInstanceOf(FileUploadError);
  });

  it("maps timeout and network errors", async () => {
    const fetchMock = stubFetch();
    fetchMock
      .mockRejectedValueOnce(Object.assign(new Error("timeout"), { name: "AbortError" }))
      .mockRejectedValueOnce(new Error("network"));
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toBeInstanceOf(TimeoutError);

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toBeInstanceOf(NetworkError);
  });

  it("maps unknown transport failures to NetworkError", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockRejectedValueOnce("boom");
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toMatchObject({
      name: "NetworkError",
      message: "Unexpected network error",
    });
  });

  it("retries on timeout errors", async () => {
    const fetchMock = stubFetch();
    fetchMock
      .mockRejectedValueOnce(Object.assign(new Error("timeout"), { name: "AbortError" }))
      .mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));
    const client = new HttpClient({ apiKey: "test", maxRetries: 1, retryDelay: 0 });

    await client.requestRaw({ method: "GET", path: "/meta" });
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });

  it("does not retry non-replayable readable request bodies", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockRejectedValueOnce(new Error("network"));
    const client = new HttpClient({ apiKey: "test", maxRetries: 2, retryDelay: 0 });

    await expect(
      client.requestRaw({
        method: "POST",
        path: "/chat-messages",
        data: Readable.from(["chunk"]),
      })
    ).rejects.toBeInstanceOf(NetworkError);

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const [, init] = getFetchCall(fetchMock);
    expect((init as RequestInit & { duplex?: string } | undefined)?.duplex).toBe(
      "half"
    );
  });

  it("validates query parameters before request", async () => {
    const fetchMock = stubFetch();
    const client = new HttpClient({ apiKey: "test" });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta", query: { user: 1 } })
    ).rejects.toBeInstanceOf(ValidationError);
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it("returns APIError for other http failures", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse({ message: "server" }, { status: 500 }));
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toBeInstanceOf(APIError);
  });

  it("uses plain text bodies when json parsing is not possible", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      textResponse("plain text", {
        status: 200,
        headers: { "content-type": "text/plain" },
      })
    );
    const client = new HttpClient({ apiKey: "test" });

    const response = await client.requestRaw({
      method: "GET",
      path: "/info",
    });

    expect(response.data).toBe("plain text");
  });

  it("keeps invalid json error bodies as API errors", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(
      textResponse("{invalid", {
        status: 500,
        headers: { "content-type": "application/json", "x-request-id": "req-500" },
      })
    );
    const client = new HttpClient({ apiKey: "test", maxRetries: 0 });

    await expect(
      client.requestRaw({ method: "GET", path: "/meta" })
    ).rejects.toMatchObject({
      name: "APIError",
      statusCode: 500,
      requestId: "req-500",
      responseBody: "{invalid",
    });
  });

  it("sends raw string bodies without additional json encoding", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));
    const client = new HttpClient({ apiKey: "test" });

    await client.requestRaw({
      method: "POST",
      path: "/meta",
      data: '{"pre":"serialized"}',
      headers: { "Content-Type": "application/custom+json" },
    });

    const [, init] = getFetchCall(fetchMock);
    expect(init?.body).toBe('{"pre":"serialized"}');
    expect(toHeaderRecord(init?.headers)).toMatchObject({
      "content-type": "application/custom+json",
    });
  });

  it("preserves explicit user-agent headers", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse({ ok: true }, { status: 200 }));
    const client = new HttpClient({ apiKey: "test" });

    await client.requestRaw({
      method: "GET",
      path: "/meta",
      headers: { "User-Agent": "custom-agent" },
    });

    const [, init] = getFetchCall(fetchMock);
    expect(toHeaderRecord(init?.headers)).toMatchObject({
      "user-agent": "custom-agent",
    });
  });

  it("logs requests and responses when enableLogging is true", async () => {
    const fetchMock = stubFetch();
    fetchMock.mockResolvedValueOnce(jsonResponse({ ok: true }, { status: 200 }));
    const consoleInfo = vi.spyOn(console, "info").mockImplementation(() => {});

    const client = new HttpClient({ apiKey: "test", enableLogging: true });
    await client.requestRaw({ method: "GET", path: "/meta" });

    expect(consoleInfo).toHaveBeenCalledWith(
      expect.stringContaining("dify-client-node response 200 GET")
    );
  });

  it("logs retry attempts when enableLogging is true", async () => {
    const fetchMock = stubFetch();
    fetchMock
      .mockRejectedValueOnce(Object.assign(new Error("timeout"), { name: "AbortError" }))
      .mockResolvedValueOnce(jsonResponse("ok", { status: 200 }));
    const consoleInfo = vi.spyOn(console, "info").mockImplementation(() => {});

    const client = new HttpClient({
      apiKey: "test",
      maxRetries: 1,
      retryDelay: 0,
      enableLogging: true,
    });

    await client.requestRaw({ method: "GET", path: "/meta" });

    expect(consoleInfo).toHaveBeenCalledWith(
      expect.stringContaining("dify-client-node retry")
    );
  });
});

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-022.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
