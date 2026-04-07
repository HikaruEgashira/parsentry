You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-003
- **Kind**: endpoint
- **Identifier**: Web/Public Chat and Completion Endpoints (/api/chat-messages, /api/completion-messages)
- **Description**: Public-facing endpoints for shared apps accessible without full authentication. Risk of IDOR via share tokens, prompt injection, and abuse of shared app resources.
- **Locations**: api/controllers/web/, web/app/(shareLayout)/chat/, web/app/(shareLayout)/chatbot/, web/app/(shareLayout)/completion/

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

### api/controllers/web/files.py
```py
from flask import request

import services
from controllers.common.errors import (
    FilenameNotExistsError,
    FileTooLargeError,
    NoFileUploadedError,
    TooManyFilesError,
    UnsupportedFileTypeError,
)
from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.wraps import WebApiResource
from extensions.ext_database import db
from fields.file_fields import FileResponse
from services.file_service import FileService

register_schema_models(web_ns, FileResponse)


@web_ns.route("/files/upload")
class FileApi(WebApiResource):
    @web_ns.doc("upload_file")
    @web_ns.doc(description="Upload a file for use in web applications")
    @web_ns.doc(
        responses={
            201: "File uploaded successfully",
            400: "Bad request - invalid file or parameters",
            413: "File too large",
            415: "Unsupported file type",
        }
    )
    @web_ns.response(201, "File uploaded successfully", web_ns.models[FileResponse.__name__])
    def post(self, app_model, end_user):
        """Upload a file for use in web applications.

        Accepts file uploads for use within web applications, supporting
        multiple file types with automatic validation and storage.

        Args:
            app_model: The associated application model
            end_user: The end user uploading the file

        Form Parameters:
            file: The file to upload (required)
            source: Optional source type (datasets or None)

        Returns:
            dict: File information including ID, URL, and metadata
            int: HTTP status code 201 for success

        Raises:
            NoFileUploadedError: No file provided in request
            TooManyFilesError: Multiple files provided (only one allowed)
            FilenameNotExistsError: File has no filename
            FileTooLargeError: File exceeds size limit
            UnsupportedFileTypeError: File type not supported
        """
        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()

        file = request.files["file"]
        if not file.filename:
            raise FilenameNotExistsError

        source = request.form.get("source")
        if source not in ("datasets", None):
            source = None

        try:
            upload_file = FileService(db.engine).upload_file(
                filename=file.filename,
                content=file.read(),
                mimetype=file.mimetype,
                user=end_user,
                source="datasets" if source == "datasets" else None,
            )
        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        response = FileResponse.model_validate(upload_file, from_attributes=True)
        return response.model_dump(mode="json"), 201

```

### api/controllers/web/error.py
```py
from libs.exception import BaseHTTPException


class AppUnavailableError(BaseHTTPException):
    error_code = "app_unavailable"
    description = "App unavailable, please check your app configurations."
    code = 400


class NotCompletionAppError(BaseHTTPException):
    error_code = "not_completion_app"
    description = "Please check if your Completion app mode matches the right API route."
    code = 400


class NotChatAppError(BaseHTTPException):
    error_code = "not_chat_app"
    description = "Please check if your app mode matches the right API route."
    code = 400


class NotWorkflowAppError(BaseHTTPException):
    error_code = "not_workflow_app"
    description = "Please check if your Workflow app mode matches the right API route."
    code = 400


class ConversationCompletedError(BaseHTTPException):
    error_code = "conversation_completed"
    description = "The conversation has ended. Please start a new conversation."
    code = 400


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
        "Your quota for Dify Hosted OpenAI has been exhausted. "
        "Please go to Settings -> Model Provider to complete your own provider credentials."
    )
    code = 400


class ProviderModelCurrentlyNotSupportError(BaseHTTPException):
    error_code = "model_currently_not_support"
    description = "Dify Hosted OpenAI trial currently not support the GPT-4 model."
    code = 400


class CompletionRequestError(BaseHTTPException):
    error_code = "completion_request_error"
    description = "Completion request failed."
    code = 400


class AppMoreLikeThisDisabledError(BaseHTTPException):
    error_code = "app_more_like_this_disabled"
    description = "The 'More like this' feature is disabled. Please refresh your page."
    code = 403


class AppSuggestedQuestionsAfterAnswerDisabledError(BaseHTTPException):
    error_code = "app_suggested_questions_after_answer_disabled"
    description = "The 'Suggested Questions After Answer' feature is disabled. Please refresh your page."
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


class WebAppAuthRequiredError(BaseHTTPException):
    error_code = "web_sso_auth_required"
    description = "Web app authentication required."
    code = 401


class WebAppAuthAccessDeniedError(BaseHTTPException):
    error_code = "web_app_access_denied"
    description = "You do not have permission to access this web app."
    code = 401


class InvokeRateLimitError(BaseHTTPException):
    """Raised when the Invoke returns rate limit error."""

    error_code = "rate_limit_error"
    description = "Rate Limit Error"
    code = 429


class WebFormRateLimitExceededError(BaseHTTPException):
    error_code = "web_form_rate_limit_exceeded"
    description = "Too many form requests. Please try again later."
    code = 429


class NotFoundError(BaseHTTPException):
    error_code = "not_found"
    code = 404


class InvalidArgumentError(BaseHTTPException):
    error_code = "invalid_param"
    code = 400

```

### api/controllers/web/completion.py
```py
import logging
from typing import Any, Literal

from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, field_validator
from werkzeug.exceptions import InternalServerError, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.error import (
    AppUnavailableError,
    CompletionRequestError,
    ConversationCompletedError,
    NotChatAppError,
    NotCompletionAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from controllers.web.wraps import WebApiResource
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from libs import helper
from libs.helper import uuid_value
from models.model import AppMode
from services.app_generate_service import AppGenerateService
from services.app_task_service import AppTaskService
from services.errors.llm import InvokeRateLimitError

logger = logging.getLogger(__name__)


class CompletionMessagePayload(BaseModel):
    inputs: dict[str, Any] = Field(description="Input variables for the completion")
    query: str = Field(default="", description="Query text for completion")
    files: list[dict[str, Any]] | None = Field(default=None, description="Files to be processed")
    response_mode: Literal["blocking", "streaming"] | None = Field(
        default=None, description="Response mode: blocking or streaming"
    )
    retriever_from: str = Field(default="web_app", description="Source of retriever")


class ChatMessagePayload(BaseModel):
    inputs: dict[str, Any] = Field(description="Input variables for the chat")
    query: str = Field(description="User query/message")
    files: list[dict[str, Any]] | None = Field(default=None, description="Files to be processed")
    response_mode: Literal["blocking", "streaming"] | None = Field(
        default=None, description="Response mode: blocking or streaming"
    )
    conversation_id: str | None = Field(default=None, description="Conversation ID")
    parent_message_id: str | None = Field(default=None, description="Parent message ID")
    retriever_from: str = Field(default="web_app", description="Source of retriever")

    @field_validator("conversation_id", "parent_message_id")
    @classmethod
    def validate_uuid(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


register_schema_models(web_ns, CompletionMessagePayload, ChatMessagePayload)


# define completion api for user
@web_ns.route("/completion-messages")
class CompletionApi(WebApiResource):
    @web_ns.doc("Create Completion Message")
    @web_ns.doc(description="Create a completion message for text generation applications.")
    @web_ns.expect(web_ns.models[CompletionMessagePayload.__name__])
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user):
        if app_model.mode != AppMode.COMPLETION:
            raise NotCompletionAppError()

        payload = CompletionMessagePayload.model_validate(web_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        streaming = payload.response_mode == "streaming"
        args["auto_generate_name"] = False

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.WEB_APP, streaming=streaming
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


@web_ns.route("/completion-messages/<string:task_id>/stop")
class CompletionStopApi(WebApiResource):
    @web_ns.doc("Stop Completion Message")
    @web_ns.doc(description="Stop a running completion message task.")
    @web_ns.doc(params={"task_id": {"description": "Task ID to stop", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Task Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user, task_id):
        if app_model.mode != AppMode.COMPLETION:
            raise NotCompletionAppError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.WEB_APP,
            user_id=end_user.id,
            app_mode=AppMode.value_of(app_model.mode),
        )

        return {"result": "success"}, 200


@web_ns.route("/chat-messages")
class ChatApi(WebApiResource):
    @web_ns.doc("Create Chat Message")
    @web_ns.doc(description="Create a chat message for conversational applications.")
    @web_ns.expect(web_ns.models[ChatMessagePayload.__name__])
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        payload = ChatMessagePayload.model_validate(web_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        streaming = payload.response_mode == "streaming"
        args["auto_generate_name"] = False

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.WEB_APP, streaming=streaming
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


@web_ns.route("/chat-messages/<string:task_id>/stop")
class ChatStopApi(WebApiResource):
    @web_ns.doc("Stop Chat Message")
    @web_ns.doc(description="Stop a running chat message task.")
    @web_ns.doc(params={"task_id": {"description": "Task ID to stop", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Task Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user, task_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.WEB_APP,
            user_id=end_user.id,
            app_mode=app_mode,
        )

        return {"result": "success"}, 200

```

### api/controllers/web/passport.py
```py
import uuid
from datetime import UTC, datetime, timedelta

from flask import make_response, request
from flask_restx import Resource
from sqlalchemy import func, select
from werkzeug.exceptions import NotFound, Unauthorized

from configs import dify_config
from constants import HEADER_NAME_APP_CODE
from controllers.web import web_ns
from controllers.web.error import WebAppAuthRequiredError
from extensions.ext_database import db
from libs.passport import PassportService
from libs.token import extract_webapp_access_token
from models.model import App, EndUser, Site
from services.feature_service import FeatureService
from services.webapp_auth_service import WebAppAuthService, WebAppAuthType


@web_ns.route("/passport")
class PassportResource(Resource):
    """Base resource for passport."""

    @web_ns.doc("get_passport")
    @web_ns.doc(description="Get authentication passport for web application access")
    @web_ns.doc(
        responses={
            200: "Passport retrieved successfully",
            401: "Unauthorized - missing app code or invalid authentication",
            404: "Application or user not found",
        }
    )
    def get(self):
        system_features = FeatureService.get_system_features()
        app_code = request.headers.get(HEADER_NAME_APP_CODE)
        user_id = request.args.get("user_id")
        access_token = extract_webapp_access_token(request)
        if app_code is None:
            raise Unauthorized("X-App-Code header is missing.")
        if system_features.webapp_auth.enabled:
            enterprise_user_decoded = decode_enterprise_webapp_user_id(access_token)
            app_auth_type = WebAppAuthService.get_app_auth_type(app_code=app_code)
            if app_auth_type != WebAppAuthType.PUBLIC:
                if not enterprise_user_decoded:
                    raise WebAppAuthRequiredError()
                return exchange_token_for_existing_web_user(
                    app_code=app_code, enterprise_user_decoded=enterprise_user_decoded, auth_type=app_auth_type
                )

        # get site from db and check if it is normal
        site = db.session.scalar(select(Site).where(Site.code == app_code, Site.status == "normal"))
        if not site:
            raise NotFound()
        # get app from db and check if it is normal and enable_site
        app_model = db.session.scalar(select(App).where(App.id == site.app_id))
        if not app_model or app_model.status != "normal" or not app_model.enable_site:
            raise NotFound()

        if user_id:
            end_user = db.session.scalar(
                select(EndUser).where(EndUser.app_id == app_model.id, EndUser.session_id == user_id)
            )

            if end_user:
                pass
            else:
                end_user = EndUser(
                    tenant_id=app_model.tenant_id,
                    app_id=app_model.id,
                    type="browser",
                    is_anonymous=True,
                    session_id=user_id,
                )
                db.session.add(end_user)
                db.session.commit()
        else:
            end_user = EndUser(
                tenant_id=app_model.tenant_id,
                app_id=app_model.id,
                type="browser",
                is_anonymous=True,
                session_id=generate_session_id(),
            )
            db.session.add(end_user)
            db.session.commit()

        payload = {
            "iss": site.app_id,
            "sub": "Web API Passport",
            "app_id": site.app_id,
            "app_code": app_code,
            "end_user_id": end_user.id,
        }

        tk = PassportService().issue(payload)

        response = make_response(
            {
                "access_token": tk,
            }
        )
        return response


def decode_enterprise_webapp_user_id(jwt_token: str | None):
    """
    Decode the enterprise user session from the Authorization header.
    """
    if not jwt_token:
        return None

    decoded = PassportService().verify(jwt_token)
    source = decoded.get("token_source")
    if not source or source != "webapp_login_token":
        raise Unauthorized("Invalid token source. Expected 'webapp_login_token'.")
    return decoded


def exchange_token_for_existing_web_user(app_code: str, enterprise_user_decoded: dict, auth_type: WebAppAuthType):
    """
    Exchange a token for an existing web user session.
    """
    user_id = enterprise_user_decoded.get("user_id")
    end_user_id = enterprise_user_decoded.get("end_user_id")
    session_id = enterprise_user_decoded.get("session_id")
    user_auth_type = enterprise_user_decoded.get("auth_type")
    exchanged_token_expires_unix = enterprise_user_decoded.get("exp")

    if not user_auth_type:
        raise Unauthorized("Missing auth_type in the token.")

    site = db.session.scalar(select(Site).where(Site.code == app_code, Site.status == "normal"))
    if not site:
        raise NotFound()

    app_model = db.session.scalar(select(App).where(App.id == site.app_id))
    if not app_model or app_model.status != "normal" or not app_model.enable_site:
        raise NotFound()

    if auth_type == WebAppAuthType.PUBLIC:
        return _exchange_for_public_app_token(app_model, site, enterprise_user_decoded)
    elif auth_type == WebAppAuthType.EXTERNAL and user_auth_type != "external":
        raise WebAppAuthRequiredError("Please login as external user.")
    elif auth_type == WebAppAuthType.INTERNAL and user_auth_type != "internal":
        raise WebAppAuthRequiredError("Please login as internal user.")

    end_user = None
    if end_user_id:
        end_user = db.session.scalar(select(EndUser).where(EndUser.id == end_user_id))
    if session_id:
        end_user = db.session.scalar(
            select(EndUser).where(
                EndUser.session_id == session_id,
                EndUser.tenant_id == app_model.tenant_id,
                EndUser.app_id == app_model.id,
            )
        )
    if not end_user:
        if not session_id:
            raise NotFound("Missing session_id for existing web user.")
        end_user = EndUser(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            type="browser",
            is_anonymous=True,
            session_id=session_id,
        )
        db.session.add(end_user)
        db.session.commit()

    exp = int((datetime.now(UTC) + timedelta(minutes=dify_config.ACCESS_TOKEN_EXPIRE_MINUTES)).timestamp())
    if exchanged_token_expires_unix:
        exp = int(exchanged_token_expires_unix)

    payload = {
        "iss": site.id,
        "sub": "Web API Passport",
        "app_id": site.app_id,
        "app_code": site.code,
        "user_id": user_id,
        "end_user_id": end_user.id,
        "auth_type": user_auth_type,
        "granted_at": int(datetime.now(UTC).timestamp()),
        "token_source": "webapp",
        "exp": exp,
    }
    token: str = PassportService().issue(payload)
    resp = make_response(
        {
            "access_token": token,
        }
    )
    return resp


def _exchange_for_public_app_token(app_model, site, token_decoded):
    user_id = token_decoded.get("user_id")
    end_user = None
    if user_id:
        end_user = db.session.scalar(
            select(EndUser).where(EndUser.app_id == app_model.id, EndUser.session_id == user_id)
        )

    if not end_user:
        end_user = EndUser(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            type="browser",
            is_anonymous=True,
            session_id=generate_session_id(),
        )

        db.session.add(end_user)
        db.session.commit()

    payload = {
        "iss": site.app_id,
        "sub": "Web API Passport",
        "app_id": site.app_id,
        "app_code": site.code,
        "end_user_id": end_user.id,
    }

    tk = PassportService().issue(payload)

    resp = make_response(
        {
            "access_token": tk,
        }
    )
    return resp


def generate_session_id():
    """
    Generate a unique session ID.
    """
    while True:
        session_id = str(uuid.uuid4())
        existing_count = db.session.scalar(
            select(func.count()).select_from(EndUser).where(EndUser.session_id == session_id)
        )
        if existing_count == 0:
            return session_id

```

### api/controllers/web/saved_message.py
```py
from flask import request
from pydantic import BaseModel, Field, TypeAdapter
from werkzeug.exceptions import NotFound

from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.error import NotCompletionAppError
from controllers.web.wraps import WebApiResource
from fields.conversation_fields import ResultResponse
from fields.message_fields import SavedMessageInfiniteScrollPagination, SavedMessageItem
from libs.helper import UUIDStrOrEmpty
from services.errors.message import MessageNotExistsError
from services.saved_message_service import SavedMessageService


class SavedMessageListQuery(BaseModel):
    last_id: UUIDStrOrEmpty | None = None
    limit: int = Field(default=20, ge=1, le=100)


class SavedMessageCreatePayload(BaseModel):
    message_id: UUIDStrOrEmpty


register_schema_models(web_ns, SavedMessageListQuery, SavedMessageCreatePayload)


@web_ns.route("/saved-messages")
class SavedMessageListApi(WebApiResource):
    @web_ns.doc("Get Saved Messages")
    @web_ns.doc(description="Retrieve paginated list of saved messages for a completion application.")
    @web_ns.doc(
        params={
            "last_id": {"description": "Last message ID for pagination", "type": "string", "required": False},
            "limit": {
                "description": "Number of messages to return (1-100)",
                "type": "integer",
                "required": False,
                "default": 20,
            },
        }
    )
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request - Not a completion app",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model, end_user):
        if app_model.mode != "completion":
            raise NotCompletionAppError()

        raw_args = request.args.to_dict()
        query = SavedMessageListQuery.model_validate(raw_args)

        pagination = SavedMessageService.pagination_by_last_id(app_model, end_user, query.last_id, query.limit)
        adapter = TypeAdapter(SavedMessageItem)
        items = [adapter.validate_python(message, from_attributes=True) for message in pagination.data]
        return SavedMessageInfiniteScrollPagination(
            limit=pagination.limit,
            has_more=pagination.has_more,
            data=items,
        ).model_dump(mode="json")

    @web_ns.doc("Save Message")
    @web_ns.doc(description="Save a specific message for later reference.")
    @web_ns.doc(
        params={
            "message_id": {"description": "Message UUID to save", "type": "string", "required": True},
        }
    )
    @web_ns.doc(
        responses={
            200: "Message saved successfully",
            400: "Bad Request - Not a completion app",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Message Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user):
        if app_model.mode != "completion":
            raise NotCompletionAppError()

        payload = SavedMessageCreatePayload.model_validate(web_ns.payload or {})

        try:
            SavedMessageService.save(app_model, end_user, payload.message_id)
        except MessageNotExistsError:
            raise NotFound("Message Not Exists.")

        return ResultResponse(result="success").model_dump(mode="json")


@web_ns.route("/saved-messages/<uuid:message_id>")
class SavedMessageApi(WebApiResource):
    @web_ns.doc("Delete Saved Message")
    @web_ns.doc(description="Remove a message from saved messages.")
    @web_ns.doc(params={"message_id": {"description": "Message UUID to delete", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            204: "Message removed successfully",
            400: "Bad Request - Not a completion app",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Message Not Found",
            500: "Internal Server Error",
        }
    )
    def delete(self, app_model, end_user, message_id):
        message_id = str(message_id)

        if app_model.mode != "completion":
            raise NotCompletionAppError()

        SavedMessageService.delete(app_model, end_user, message_id)

        return ResultResponse(result="success").model_dump(mode="json"), 204

```

### api/controllers/web/conversation.py
```py
from typing import Literal

from flask import request
from pydantic import BaseModel, Field, TypeAdapter, field_validator, model_validator
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import NotFound

from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.error import NotChatAppError
from controllers.web.wraps import WebApiResource
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from fields.conversation_fields import (
    ConversationInfiniteScrollPagination,
    ResultResponse,
    SimpleConversation,
)
from libs.helper import uuid_value
from models.model import AppMode
from services.conversation_service import ConversationService
from services.errors.conversation import ConversationNotExistsError, LastConversationNotExistsError
from services.web_conversation_service import WebConversationService


class ConversationListQuery(BaseModel):
    last_id: str | None = None
    limit: int = Field(default=20, ge=1, le=100)
    pinned: bool | None = None
    sort_by: Literal["created_at", "-created_at", "updated_at", "-updated_at"] = "-updated_at"

    @field_validator("last_id")
    @classmethod
    def validate_last_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class ConversationRenamePayload(BaseModel):
    name: str | None = None
    auto_generate: bool = False

    @model_validator(mode="after")
    def validate_name_requirement(self):
        if not self.auto_generate:
            if self.name is None or not self.name.strip():
                raise ValueError("name is required when auto_generate is false")
        return self


register_schema_models(web_ns, ConversationListQuery, ConversationRenamePayload)


@web_ns.route("/conversations")
class ConversationListApi(WebApiResource):
    @web_ns.doc("Get Conversation List")
    @web_ns.doc(description="Retrieve paginated list of conversations for a chat application.")
    @web_ns.doc(
        params={
            "last_id": {"description": "Last conversation ID for pagination", "type": "string", "required": False},
            "limit": {
                "description": "Number of conversations to return (1-100)",
                "type": "integer",
                "required": False,
                "default": 20,
            },
            "pinned": {
                "description": "Filter by pinned status",
                "type": "string",
                "enum": ["true", "false"],
                "required": False,
            },
            "sort_by": {
                "description": "Sort order",
                "type": "string",
                "enum": ["created_at", "-created_at", "updated_at", "-updated_at"],
                "required": False,
                "default": "-updated_at",
            },
        }
    )
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model, end_user):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        raw_args = request.args.to_dict()
        query = ConversationListQuery.model_validate(raw_args)

        try:
            with sessionmaker(db.engine).begin() as session:
                pagination = WebConversationService.pagination_by_last_id(
                    session=session,
                    app_model=app_model,
                    user=end_user,
                    last_id=query.last_id,
                    limit=query.limit,
                    invoke_from=InvokeFrom.WEB_APP,
                    pinned=query.pinned,
                    sort_by=query.sort_by,
                )
                adapter = TypeAdapter(SimpleConversation)
                conversations = [adapter.validate_python(item, from_attributes=True) for item in pagination.data]
                return ConversationInfiniteScrollPagination(
                    limit=pagination.limit,
                    has_more=pagination.has_more,
                    data=conversations,
                ).model_dump(mode="json")
        except LastConversationNotExistsError:
            raise NotFound("Last Conversation Not Exists.")


@web_ns.route("/conversations/<uuid:c_id>")
class ConversationApi(WebApiResource):
    @web_ns.doc("Delete Conversation")
    @web_ns.doc(description="Delete a specific conversation.")
    @web_ns.doc(params={"c_id": {"description": "Conversation UUID", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            204: "Conversation deleted successfully",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Conversation Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def delete(self, app_model, end_user, c_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)
        try:
            ConversationService.delete(app_model, conversation_id, end_user)
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        return ResultResponse(result="success").model_dump(mode="json"), 204


@web_ns.route("/conversations/<uuid:c_id>/name")
class ConversationRenameApi(WebApiResource):
    @web_ns.doc("Rename Conversation")
    @web_ns.doc(description="Rename a specific conversation with a custom name or auto-generate one.")
    @web_ns.doc(params={"c_id": {"description": "Conversation UUID", "type": "string", "required": True}})
    @web_ns.doc(
        params={
            "name": {"description": "New conversation name", "type": "string", "required": False},
            "auto_generate": {
                "description": "Auto-generate conversation name",
                "type": "boolean",
                "required": False,
                "default": False,
            },
        }
    )
    @web_ns.doc(
        responses={
            200: "Conversation renamed successfully",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Conversation Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user, c_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)

        payload = ConversationRenamePayload.model_validate(web_ns.payload or {})

        try:
            conversation = ConversationService.rename(
                app_model, conversation_id, end_user, payload.name, payload.auto_generate
            )
            return (
                TypeAdapter(SimpleConversation)
                .validate_python(conversation, from_attributes=True)
                .model_dump(mode="json")
            )
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")


@web_ns.route("/conversations/<uuid:c_id>/pin")
class ConversationPinApi(WebApiResource):
    @web_ns.doc("Pin Conversation")
    @web_ns.doc(description="Pin a specific conversation to keep it at the top of the list.")
    @web_ns.doc(params={"c_id": {"description": "Conversation UUID", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Conversation pinned successfully",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Conversation Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def patch(self, app_model, end_user, c_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)

        try:
            WebConversationService.pin(app_model, conversation_id, end_user)
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")

        return ResultResponse(result="success").model_dump(mode="json")


@web_ns.route("/conversations/<uuid:c_id>/unpin")
class ConversationUnPinApi(WebApiResource):
    @web_ns.doc("Unpin Conversation")
    @web_ns.doc(description="Unpin a specific conversation to remove it from the top of the list.")
    @web_ns.doc(params={"c_id": {"description": "Conversation UUID", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Conversation unpinned successfully",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Conversation Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def patch(self, app_model, end_user, c_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)
        WebConversationService.unpin(app_model, conversation_id, end_user)

        return ResultResponse(result="success").model_dump(mode="json")

```

### api/controllers/web/human_input_form.py
```py
"""
Web App Human Input Form APIs.
"""

import json
import logging
from datetime import datetime

from flask import Response, request
from flask_restx import Resource, reqparse
from sqlalchemy import select
from werkzeug.exceptions import Forbidden

from configs import dify_config
from controllers.web import web_ns
from controllers.web.error import NotFoundError, WebFormRateLimitExceededError
from controllers.web.site import serialize_app_site_payload
from extensions.ext_database import db
from libs.helper import RateLimiter, extract_remote_ip
from models.account import TenantStatus
from models.model import App, Site
from services.human_input_service import Form, FormNotFoundError, HumanInputService

logger = logging.getLogger(__name__)

_FORM_SUBMIT_RATE_LIMITER = RateLimiter(
    prefix="web_form_submit_rate_limit",
    max_attempts=dify_config.WEB_FORM_SUBMIT_RATE_LIMIT_MAX_ATTEMPTS,
    time_window=dify_config.WEB_FORM_SUBMIT_RATE_LIMIT_WINDOW_SECONDS,
)
_FORM_ACCESS_RATE_LIMITER = RateLimiter(
    prefix="web_form_access_rate_limit",
    max_attempts=dify_config.WEB_FORM_SUBMIT_RATE_LIMIT_MAX_ATTEMPTS,
    time_window=dify_config.WEB_FORM_SUBMIT_RATE_LIMIT_WINDOW_SECONDS,
)


def _stringify_default_values(values: dict[str, object]) -> dict[str, str]:
    result: dict[str, str] = {}
    for key, value in values.items():
        if value is None:
            result[key] = ""
        elif isinstance(value, (dict, list)):
            result[key] = json.dumps(value, ensure_ascii=False)
        else:
            result[key] = str(value)
    return result


def _to_timestamp(value: datetime) -> int:
    return int(value.timestamp())


def _jsonify_form_definition(form: Form, site_payload: dict | None = None) -> Response:
    """Return the form payload (optionally with site) as a JSON response."""
    definition_payload = form.get_definition().model_dump()
    payload = {
        "form_content": definition_payload["rendered_content"],
        "inputs": definition_payload["inputs"],
        "resolved_default_values": _stringify_default_values(definition_payload["default_values"]),
        "user_actions": definition_payload["user_actions"],
        "expiration_time": _to_timestamp(form.expiration_time),
    }
    if site_payload is not None:
        payload["site"] = site_payload
    return Response(json.dumps(payload, ensure_ascii=False), mimetype="application/json")


@web_ns.route("/form/human_input/<string:form_token>")
class HumanInputFormApi(Resource):
    """API for getting and submitting human input forms via the web app."""

    # NOTE(QuantumGhost): this endpoint is unauthenticated on purpose for now.

    # def get(self, _app_model: App, _end_user: EndUser, form_token: str):
    def get(self, form_token: str):
        """
        Get human input form definition by token.

        GET /api/form/human_input/<form_token>
        """
        ip_address = extract_remote_ip(request)
        if _FORM_ACCESS_RATE_LIMITER.is_rate_limited(ip_address):
            raise WebFormRateLimitExceededError()
        _FORM_ACCESS_RATE_LIMITER.increment_rate_limit(ip_address)

        service = HumanInputService(db.engine)
        # TODO(QuantumGhost): forbid submision for form tokens
        # that are only for console.
        form = service.get_form_by_token(form_token)

        if form is None:
            raise NotFoundError("Form not found")

        service.ensure_form_active(form)
        app_model, site = _get_app_site_from_form(form)

        return _jsonify_form_definition(form, site_payload=serialize_app_site_payload(app_model, site, None))

    # def post(self, _app_model: App, _end_user: EndUser, form_token: str):
    def post(self, form_token: str):
        """
        Submit human input form by token.

        POST /api/form/human_input/<form_token>

        Request body:
        {
            "inputs": {
                "content": "User input content"
            },
            "action": "Approve"
        }
        """
        parser = reqparse.RequestParser()
        parser.add_argument("inputs", type=dict, required=True, location="json")
        parser.add_argument("action", type=str, required=True, location="json")
        args = parser.parse_args()

        ip_address = extract_remote_ip(request)
        if _FORM_SUBMIT_RATE_LIMITER.is_rate_limited(ip_address):
            raise WebFormRateLimitExceededError()
        _FORM_SUBMIT_RATE_LIMITER.increment_rate_limit(ip_address)

        service = HumanInputService(db.engine)
        form = service.get_form_by_token(form_token)
        if form is None:
            raise NotFoundError("Form not found")

        if (recipient_type := form.recipient_type) is None:
            logger.warning("Recipient type is None for form, form_id=%", form.id)
            raise AssertionError("Recipient type is None")

        try:
            service.submit_form_by_token(
                recipient_type=recipient_type,
                form_token=form_token,
                selected_action_id=args["action"],
                form_data=args["inputs"],
                submission_end_user_id=None,
                # submission_end_user_id=_end_user.id,
            )
        except FormNotFoundError:
            raise NotFoundError("Form not found")

        return {}, 200


def _get_app_site_from_form(form: Form) -> tuple[App, Site]:
    """Resolve App/Site for the form's app and validate tenant status."""
    app_model = db.session.get(App, form.app_id)
    if app_model is None or app_model.tenant_id != form.tenant_id:
        raise NotFoundError("Form not found")

    site = db.session.scalar(select(Site).where(Site.app_id == app_model.id).limit(1))
    if site is None:
        raise Forbidden()

    if app_model.tenant and app_model.tenant.status == TenantStatus.ARCHIVE:
        raise Forbidden()

    return app_model, site

```

### api/controllers/web/__init__.py
```py
from flask import Blueprint
from flask_restx import Namespace

from libs.external_api import ExternalApi

bp = Blueprint("web", __name__, url_prefix="/api")

api = ExternalApi(
    bp,
    version="1.0",
    title="Web API",
    description="Public APIs for web applications including file uploads, chat interactions, and app management",
)

# Create namespace
web_ns = Namespace("web", description="Web application API operations", path="/")

from . import (
    app,
    audio,
    completion,
    conversation,
    feature,
    files,
    forgot_password,
    human_input_form,
    login,
    message,
    passport,
    remote_files,
    saved_message,
    site,
    workflow,
    workflow_events,
)

api.add_namespace(web_ns)

__all__ = [
    "api",
    "app",
    "audio",
    "bp",
    "completion",
    "conversation",
    "feature",
    "files",
    "forgot_password",
    "human_input_form",
    "login",
    "message",
    "passport",
    "remote_files",
    "saved_message",
    "site",
    "web_ns",
    "workflow",
    "workflow_events",
]

```

### api/controllers/web/message.py
```py
import logging
from typing import Literal

from flask import request
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, TypeAdapter, field_validator
from werkzeug.exceptions import InternalServerError, NotFound

from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.error import (
    AppMoreLikeThisDisabledError,
    AppSuggestedQuestionsAfterAnswerDisabledError,
    CompletionRequestError,
    NotChatAppError,
    NotCompletionAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.web.wraps import WebApiResource
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from fields.conversation_fields import ResultResponse
from fields.message_fields import SuggestedQuestionsResponse, WebMessageInfiniteScrollPagination, WebMessageListItem
from libs import helper
from libs.helper import uuid_value
from models.enums import FeedbackRating
from models.model import AppMode
from services.app_generate_service import AppGenerateService
from services.errors.app import MoreLikeThisDisabledError
from services.errors.conversation import ConversationNotExistsError
from services.errors.message import (
    FirstMessageNotExistsError,
    MessageNotExistsError,
    SuggestedQuestionsAfterAnswerDisabledError,
)
from services.message_service import MessageService

logger = logging.getLogger(__name__)


class MessageListQuery(BaseModel):
    conversation_id: str = Field(description="Conversation UUID")
    first_id: str | None = Field(default=None, description="First message ID for pagination")
    limit: int = Field(default=20, ge=1, le=100, description="Number of messages to return (1-100)")

    @field_validator("conversation_id", "first_id")
    @classmethod
    def validate_uuid(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


class MessageFeedbackPayload(BaseModel):
    rating: Literal["like", "dislike"] | None = Field(default=None, description="Feedback rating")
    content: str | None = Field(default=None, description="Feedback content")


class MessageMoreLikeThisQuery(BaseModel):
    response_mode: Literal["blocking", "streaming"] = Field(
        description="Response mode",
    )


register_schema_models(web_ns, MessageListQuery, MessageFeedbackPayload, MessageMoreLikeThisQuery)


@web_ns.route("/messages")
class MessageListApi(WebApiResource):
    @web_ns.doc("Get Message List")
    @web_ns.doc(description="Retrieve paginated list of messages from a conversation in a chat application.")
    @web_ns.doc(
        params={
            "conversation_id": {"description": "Conversation UUID", "type": "string", "required": True},
            "first_id": {
                "description": "First message ID for pagination",
                "type": "string",
                "required": False,
            },
            "limit": {
                "description": "Number of messages to return (1-100)",
                "type": "integer",
                "required": False,
                "default": 20,
            },
        }
    )
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Conversation Not Found or Not a Chat App",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model, end_user):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        raw_args = request.args.to_dict()
        query = MessageListQuery.model_validate(raw_args)

        try:
            pagination = MessageService.pagination_by_first_id(
                app_model, end_user, query.conversation_id, query.first_id, query.limit
            )
            adapter = TypeAdapter(WebMessageListItem)
            items = [adapter.validate_python(message, from_attributes=True) for message in pagination.data]
            return WebMessageInfiniteScrollPagination(
                limit=pagination.limit,
                has_more=pagination.has_more,
                data=items,
            ).model_dump(mode="json")
        except ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except FirstMessageNotExistsError:
            raise NotFound("First Message Not Exists.")


@web_ns.route("/messages/<uuid:message_id>/feedbacks")
class MessageFeedbackApi(WebApiResource):
    @web_ns.doc("Create Message Feedback")
    @web_ns.doc(description="Submit feedback (like/dislike) for a specific message.")
    @web_ns.doc(params={"message_id": {"description": "Message UUID", "type": "string", "required": True}})
    @web_ns.doc(
        params={
            "rating": {
                "description": "Feedback rating",
                "type": "string",
                "enum": ["like", "dislike"],
                "required": False,
            },
            "content": {"description": "Feedback content", "type": "string", "required": False},
        }
    )
    @web_ns.doc(
        responses={
            200: "Feedback submitted successfully",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Message Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model, end_user, message_id):
        message_id = str(message_id)

        payload = MessageFeedbackPayload.model_validate(web_ns.payload or {})

        try:
            MessageService.create_feedback(
                app_model=app_model,
                message_id=message_id,
                user=end_user,
                rating=FeedbackRating(payload.rating) if payload.rating else None,
                content=payload.content,
            )
        except MessageNotExistsError:
            raise NotFound("Message Not Exists.")

        return ResultResponse(result="success").model_dump(mode="json")


@web_ns.route("/messages/<uuid:message_id>/more-like-this")
class MessageMoreLikeThisApi(WebApiResource):
    @web_ns.doc("Generate More Like This")
    @web_ns.doc(description="Generate a new completion similar to an existing message (completion apps only).")
    @web_ns.expect(web_ns.models[MessageMoreLikeThisQuery.__name__])
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request - Not a completion app or feature disabled",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Message Not Found",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model, end_user, message_id):
        if app_model.mode != "completion":
            raise NotCompletionAppError()

        message_id = str(message_id)

        raw_args = request.args.to_dict()
        query = MessageMoreLikeThisQuery.model_validate(raw_args)

        streaming = query.response_mode == "streaming"

        try:
            response = AppGenerateService.generate_more_like_this(
                app_model=app_model,
                user=end_user,
                message_id=message_id,
                invoke_from=InvokeFrom.WEB_APP,
                streaming=streaming,
            )

            return helper.compact_generate_response(response)
        except MessageNotExistsError:
            raise NotFound("Message Not Exists.")
        except MoreLikeThisDisabledError:
            raise AppMoreLikeThisDisabledError()
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
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@web_ns.route("/messages/<uuid:message_id>/suggested-questions")
class MessageSuggestedQuestionApi(WebApiResource):
    @web_ns.doc("Get Suggested Questions")
    @web_ns.doc(description="Get suggested follow-up questions after a message (chat apps only).")
    @web_ns.doc(params={"message_id": {"description": "Message UUID", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request - Not a chat app or feature disabled",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Message Not Found or Conversation Not Found",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model, end_user, message_id):
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        message_id = str(message_id)

        try:
            questions = MessageService.get_suggested_questions_after_answer(
                app_model=app_model, user=end_user, message_id=message_id, invoke_from=InvokeFrom.WEB_APP
            )
            # questions is a list of strings, not a list of Message objects
        except MessageNotExistsError:
            raise NotFound("Message not found")
        except ConversationNotExistsError:
            raise NotFound("Conversation not found")
        except SuggestedQuestionsAfterAnswerDisabledError:
            raise AppSuggestedQuestionsAfterAnswerDisabledError()
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()

        return SuggestedQuestionsResponse(data=questions).model_dump(mode="json")

```

### api/controllers/web/feature.py
```py
from flask_restx import Resource

from controllers.web import web_ns
from services.feature_service import FeatureService


@web_ns.route("/system-features")
class SystemFeatureApi(Resource):
    @web_ns.doc("get_system_features")
    @web_ns.doc(description="Get system feature flags and configuration")
    @web_ns.doc(responses={200: "System features retrieved successfully", 500: "Internal server error"})
    def get(self):
        """Get system feature flags and configuration.

        Returns the current system feature flags and configuration
        that control various functionalities across the platform.

        Returns:
            dict: System feature configuration object

        This endpoint is akin to the `SystemFeatureApi` endpoint in api/controllers/console/feature.py,
        except it is intended for use by the web app, instead of the console dashboard.

        NOTE: This endpoint is unauthenticated by design, as it provides system features
        data required for webapp initialization.

        Authentication would create circular dependency (can't authenticate without webapp loading).

        Only non-sensitive configuration data should be returned by this endpoint.
        """
        return FeatureService.get_system_features().model_dump()

```

### api/controllers/web/forgot_password.py
```py
import base64
import secrets

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from controllers.common.schema import register_schema_models
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    EmailPasswordResetLimitError,
    InvalidEmailError,
    InvalidTokenError,
    PasswordMismatchError,
)
from controllers.console.error import EmailSendIpLimitError
from controllers.console.wraps import email_password_login_enabled, only_edition_enterprise, setup_required
from controllers.web import web_ns
from extensions.ext_database import db
from libs.helper import EmailStr, extract_remote_ip
from libs.password import hash_password, valid_password
from models.account import Account
from services.account_service import AccountService


class ForgotPasswordSendPayload(BaseModel):
    email: EmailStr
    language: str | None = None


class ForgotPasswordCheckPayload(BaseModel):
    email: EmailStr
    code: str
    token: str = Field(min_length=1)


class ForgotPasswordResetPayload(BaseModel):
    token: str = Field(min_length=1)
    new_password: str
    password_confirm: str

    @field_validator("new_password", "password_confirm")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


register_schema_models(web_ns, ForgotPasswordSendPayload, ForgotPasswordCheckPayload, ForgotPasswordResetPayload)


@web_ns.route("/forgot-password")
class ForgotPasswordSendEmailApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordSendPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("send_forgot_password_email")
    @web_ns.doc(description="Send password reset email")
    @web_ns.doc(
        responses={
            200: "Password reset email sent successfully",
            400: "Bad request - invalid email format",
            404: "Account not found",
            429: "Too many requests - rate limit exceeded",
        }
    )
    def post(self):
        payload = ForgotPasswordSendPayload.model_validate(web_ns.payload or {})

        request_email = payload.email
        normalized_email = request_email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if payload.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(request_email, session=session)
        token = None
        if account is None:
            raise AuthenticationFailedError()
        else:
            token = AccountService.send_reset_password_email(account=account, email=normalized_email, language=language)

        return {"result": "success", "data": token}


@web_ns.route("/forgot-password/validity")
class ForgotPasswordCheckApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordCheckPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("check_forgot_password_token")
    @web_ns.doc(description="Verify password reset token validity")
    @web_ns.doc(
        responses={200: "Token is valid", 400: "Bad request - invalid token format", 401: "Invalid or expired token"}
    )
    def post(self):
        payload = ForgotPasswordCheckPayload.model_validate(web_ns.payload or {})

        user_email = payload.email.lower()

        is_forgot_password_error_rate_limit = AccountService.is_forgot_password_error_rate_limit(user_email)
        if is_forgot_password_error_rate_limit:
            raise EmailPasswordResetLimitError()

        token_data = AccountService.get_reset_password_data(payload.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        if not isinstance(token_email, str):
            raise InvalidEmailError()
        normalized_token_email = token_email.lower()

        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if payload.code != token_data.get("code"):
            AccountService.add_forgot_password_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_reset_password_token(payload.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_reset_password_token(
            token_email, code=payload.code, additional_data={"phase": "reset"}
        )

        AccountService.reset_forgot_password_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@web_ns.route("/forgot-password/resets")
class ForgotPasswordResetApi(Resource):
    @web_ns.expect(web_ns.models[ForgotPasswordResetPayload.__name__])
    @only_edition_enterprise
    @setup_required
    @email_password_login_enabled
    @web_ns.doc("reset_password")
    @web_ns.doc(description="Reset user password with verification token")
    @web_ns.doc(
        responses={
            200: "Password reset successfully",
            400: "Bad request - invalid parameters or password mismatch",
            401: "Invalid or expired token",
            404: "Account not found",
        }
    )
    def post(self):
        payload = ForgotPasswordResetPayload.model_validate(web_ns.payload or {})

        # Validate passwords match
        if payload.new_password != payload.password_confirm:
            raise PasswordMismatchError()

        # Validate token and get reset data
        reset_data = AccountService.get_reset_password_data(payload.token)
        if not reset_data:
            raise InvalidTokenError()
        # Must use token in reset phase
        if reset_data.get("phase", "") != "reset":
            raise InvalidTokenError()

        # Revoke token to prevent reuse
        AccountService.revoke_reset_password_token(payload.token)

        # Generate secure salt and hash password
        salt = secrets.token_bytes(16)
        password_hashed = hash_password(payload.new_password, salt)

        email = reset_data.get("email", "")

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(email, session=session)

            if account:
                self._update_existing_account(account, password_hashed, salt)
            else:
                raise AuthenticationFailedError()

        return {"result": "success"}

    def _update_existing_account(self, account: Account, password_hashed, salt):
        # Update existing account credentials
        account.password = base64.b64encode(password_hashed).decode()
        account.password_salt = base64.b64encode(salt).decode()

```

### api/controllers/web/site.py
```py
from typing import cast

from flask_restx import fields, marshal, marshal_with
from sqlalchemy import select
from werkzeug.exceptions import Forbidden

from configs import dify_config
from controllers.web import web_ns
from controllers.web.wraps import WebApiResource
from extensions.ext_database import db
from libs.helper import AppIconUrlField
from models.account import TenantStatus
from models.model import App, Site
from services.feature_service import FeatureService


@web_ns.route("/site")
class AppSiteApi(WebApiResource):
    """Resource for app sites."""

    model_config_fields = {
        "opening_statement": fields.String,
        "suggested_questions": fields.Raw(attribute="suggested_questions_list"),
        "suggested_questions_after_answer": fields.Raw(attribute="suggested_questions_after_answer_dict"),
        "more_like_this": fields.Raw(attribute="more_like_this_dict"),
        "model": fields.Raw(attribute="model_dict"),
        "user_input_form": fields.Raw(attribute="user_input_form_list"),
        "pre_prompt": fields.String,
    }

    site_fields = {
        "title": fields.String,
        "chat_color_theme": fields.String,
        "chat_color_theme_inverted": fields.Boolean,
        "icon_type": fields.String,
        "icon": fields.String,
        "icon_background": fields.String,
        "icon_url": AppIconUrlField,
        "description": fields.String,
        "copyright": fields.String,
        "privacy_policy": fields.String,
        "custom_disclaimer": fields.String,
        "default_language": fields.String,
        "prompt_public": fields.Boolean,
        "show_workflow_steps": fields.Boolean,
        "use_icon_as_answer_icon": fields.Boolean,
    }

    app_fields = {
        "app_id": fields.String,
        "end_user_id": fields.String,
        "enable_site": fields.Boolean,
        "site": fields.Nested(site_fields),
        "model_config": fields.Nested(model_config_fields, allow_null=True),
        "plan": fields.String,
        "can_replace_logo": fields.Boolean,
        "custom_config": fields.Raw(attribute="custom_config"),
    }

    @web_ns.doc("Get App Site Info")
    @web_ns.doc(description="Retrieve app site information and configuration.")
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    @marshal_with(app_fields)
    def get(self, app_model, end_user):
        """Retrieve app site info."""
        # get site
        site = db.session.scalar(select(Site).where(Site.app_id == app_model.id).limit(1))

        if not site:
            raise Forbidden()

        if app_model.tenant.status == TenantStatus.ARCHIVE:
            raise Forbidden()

        can_replace_logo = FeatureService.get_features(app_model.tenant_id).can_replace_logo

        return AppSiteInfo(app_model.tenant, app_model, site, end_user.id, can_replace_logo)


class AppSiteInfo:
    """Class to store site information."""

    def __init__(self, tenant, app, site, end_user, can_replace_logo):
        """Initialize AppSiteInfo instance."""
        self.app_id = app.id
        self.end_user_id = end_user
        self.enable_site = app.enable_site
        self.site = site
        self.model_config = None
        self.plan = tenant.plan
        self.can_replace_logo = can_replace_logo

        if can_replace_logo:
            base_url = dify_config.FILES_URL
            remove_webapp_brand = tenant.custom_config_dict.get("remove_webapp_brand", False)
            replace_webapp_logo = (
                f"{base_url}/files/workspaces/{tenant.id}/webapp-logo"
                if tenant.custom_config_dict.get("replace_webapp_logo")
                else None
            )
            self.custom_config = {
                "remove_webapp_brand": remove_webapp_brand,
                "replace_webapp_logo": replace_webapp_logo,
            }


def serialize_site(site: Site) -> dict:
    """Serialize Site model using the same schema as AppSiteApi."""
    return cast(dict, marshal(site, AppSiteApi.site_fields))


def serialize_app_site_payload(app_model: App, site: Site, end_user_id: str | None) -> dict:
    can_replace_logo = FeatureService.get_features(app_model.tenant_id).can_replace_logo
    app_site_info = AppSiteInfo(app_model.tenant, app_model, site, end_user_id, can_replace_logo)
    return cast(dict, marshal(app_site_info, AppSiteApi.app_fields))

```

### api/controllers/web/app.py
```py
import logging
from typing import Any, cast

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, ConfigDict, Field
from werkzeug.exceptions import Unauthorized

from constants import HEADER_NAME_APP_CODE
from controllers.common import fields
from controllers.common.schema import register_schema_models
from core.app.app_config.common.parameters_mapping import get_parameters_from_feature_dict
from libs.passport import PassportService
from libs.token import extract_webapp_passport
from models.model import App, AppMode
from services.app_service import AppService
from services.enterprise.enterprise_service import EnterpriseService
from services.feature_service import FeatureService
from services.webapp_auth_service import WebAppAuthService

from . import web_ns
from .error import AppUnavailableError
from .wraps import WebApiResource

logger = logging.getLogger(__name__)


class AppAccessModeQuery(BaseModel):
    model_config = ConfigDict(populate_by_name=True)

    app_id: str | None = Field(default=None, alias="appId", description="Application ID")
    app_code: str | None = Field(default=None, alias="appCode", description="Application code")


register_schema_models(web_ns, AppAccessModeQuery)


@web_ns.route("/parameters")
class AppParameterApi(WebApiResource):
    """Resource for app variables."""

    @web_ns.doc("Get App Parameters")
    @web_ns.doc(description="Retrieve the parameters for a specific app.")
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model: App, end_user):
        """Retrieve app parameters."""
        if app_model.mode in {AppMode.ADVANCED_CHAT, AppMode.WORKFLOW}:
            workflow = app_model.workflow
            if workflow is None:
                raise AppUnavailableError()

            features_dict: dict[str, Any] = workflow.features_dict
            user_input_form = workflow.user_input_form(to_old_structure=True)
        else:
            app_model_config = app_model.app_model_config
            if app_model_config is None:
                raise AppUnavailableError()

            features_dict = cast(dict[str, Any], app_model_config.to_dict())

            user_input_form = features_dict.get("user_input_form", [])

        parameters = get_parameters_from_feature_dict(features_dict=features_dict, user_input_form=user_input_form)
        return fields.Parameters.model_validate(parameters).model_dump(mode="json")


@web_ns.route("/meta")
class AppMeta(WebApiResource):
    @web_ns.doc("Get App Meta")
    @web_ns.doc(description="Retrieve the metadata for a specific app.")
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def get(self, app_model: App, end_user):
        """Get app meta"""
        return AppService().get_app_meta(app_model)


@web_ns.route("/webapp/access-mode")
class AppAccessMode(Resource):
    @web_ns.doc("Get App Access Mode")
    @web_ns.doc(description="Retrieve the access mode for a web application (public or restricted).")
    @web_ns.doc(
        params={
            "appId": {"description": "Application ID", "type": "string", "required": False},
            "appCode": {"description": "Application code", "type": "string", "required": False},
        }
    )
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            500: "Internal Server Error",
        }
    )
    def get(self):
        raw_args = request.args.to_dict()
        args = AppAccessModeQuery.model_validate(raw_args)

        features = FeatureService.get_system_features()
        if not features.webapp_auth.enabled:
            return {"accessMode": "public"}

        app_id = args.app_id
        if args.app_code:
            app_id = AppService.get_app_id_by_code(args.app_code)

        if not app_id:
            raise ValueError("appId or appCode must be provided")

        res = EnterpriseService.WebAppAuth.get_app_access_mode_by_id(app_id)

        return {"accessMode": res.access_mode}


@web_ns.route("/webapp/permission")
class AppWebAuthPermission(Resource):
    @web_ns.doc("Check App Permission")
    @web_ns.doc(description="Check if user has permission to access a web application.")
    @web_ns.doc(params={"appId": {"description": "Application ID", "type": "string", "required": True}})
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            500: "Internal Server Error",
        }
    )
    def get(self):
        user_id = "visitor"
        app_code = request.headers.get(HEADER_NAME_APP_CODE)
        app_id = request.args.get("appId")
        if not app_id or not app_code:
            raise ValueError("appId must be provided")

        require_permission_check = WebAppAuthService.is_app_require_permission_check(app_id=app_id)
        if not require_permission_check:
            return {"result": True}

        try:
            tk = extract_webapp_passport(app_code, request)
            if not tk:
                raise Unauthorized("Access token is missing.")
            decoded = PassportService().verify(tk)
            user_id = decoded.get("user_id", "visitor")
        except Unauthorized:
            raise
        except Exception:
            logger.exception("Unexpected error during auth verification")
            raise

        features = FeatureService.get_system_features()
        if not features.webapp_auth.enabled:
            return {"result": True}

        res = True
        if WebAppAuthService.is_app_require_permission_check(app_id=app_id):
            res = EnterpriseService.WebAppAuth.is_user_allowed_to_access_webapp(str(user_id), app_id)
        return {"result": res}

```

### api/controllers/web/remote_files.py
```py
import urllib.parse

import httpx
from graphon.file import helpers as file_helpers
from pydantic import BaseModel, Field, HttpUrl

import services
from controllers.common import helpers
from controllers.common.errors import (
    FileTooLargeError,
    RemoteFileUploadError,
    UnsupportedFileTypeError,
)
from core.helper import ssrf_proxy
from extensions.ext_database import db
from fields.file_fields import FileWithSignedUrl, RemoteFileInfo
from services.file_service import FileService

from ..common.schema import register_schema_models
from . import web_ns
from .wraps import WebApiResource


class RemoteFileUploadPayload(BaseModel):
    url: HttpUrl = Field(description="Remote file URL")


register_schema_models(web_ns, RemoteFileUploadPayload, RemoteFileInfo, FileWithSignedUrl)


@web_ns.route("/remote-files/<path:url>")
class RemoteFileInfoApi(WebApiResource):
    @web_ns.doc("get_remote_file_info")
    @web_ns.doc(description="Get information about a remote file")
    @web_ns.doc(
        responses={
            200: "Remote file information retrieved successfully",
            400: "Bad request - invalid URL",
            404: "Remote file not found",
            500: "Failed to fetch remote file",
        }
    )
    @web_ns.response(200, "Remote file info", web_ns.models[RemoteFileInfo.__name__])
    def get(self, app_model, end_user, url):
        """Get information about a remote file.

        Retrieves basic information about a file located at a remote URL,
        including content type and content length.

        Args:
            app_model: The associated application model
            end_user: The end user making the request
            url: URL-encoded path to the remote file

        Returns:
            dict: Remote file information including type and length

        Raises:
            HTTPException: If the remote file cannot be accessed
        """
        decoded_url = urllib.parse.unquote(url)
        resp = ssrf_proxy.head(decoded_url)
        if resp.status_code != httpx.codes.OK:
            # failed back to get method
            resp = ssrf_proxy.get(decoded_url, timeout=3)
        resp.raise_for_status()
        info = RemoteFileInfo(
            file_type=resp.headers.get("Content-Type", "application/octet-stream"),
            file_length=int(resp.headers.get("Content-Length", -1)),
        )
        return info.model_dump(mode="json")


@web_ns.route("/remote-files/upload")
class RemoteFileUploadApi(WebApiResource):
    @web_ns.doc("upload_remote_file")
    @web_ns.doc(description="Upload a file from a remote URL")
    @web_ns.doc(
        responses={
            201: "Remote file uploaded successfully",
            400: "Bad request - invalid URL or parameters",
            413: "File too large",
            415: "Unsupported file type",
            500: "Failed to fetch remote file",
        }
    )
    @web_ns.response(201, "Remote file uploaded", web_ns.models[FileWithSignedUrl.__name__])
    def post(self, app_model, end_user):
        """Upload a file from a remote URL.

        Downloads a file from the provided remote URL and uploads it
        to the platform storage for use in web applications.

        Args:
            app_model: The associated application model
            end_user: The end user making the request

        JSON Parameters:
            url: The remote URL to download the file from (required)

        Returns:
            dict: File information including ID, signed URL, and metadata
            int: HTTP status code 201 for success

        Raises:
            RemoteFileUploadError: Failed to fetch file from remote URL
            FileTooLargeError: File exceeds size limit
            UnsupportedFileTypeError: File type not supported
        """
        payload = RemoteFileUploadPayload.model_validate(web_ns.payload or {})
        url = str(payload.url)

        try:
            resp = ssrf_proxy.head(url=url)
            if resp.status_code != httpx.codes.OK:
                resp = ssrf_proxy.get(url=url, timeout=3, follow_redirects=True)
            if resp.status_code != httpx.codes.OK:
                raise RemoteFileUploadError(f"Failed to fetch file from {url}: {resp.text}")
        except httpx.RequestError as e:
            raise RemoteFileUploadError(f"Failed to fetch file from {url}: {str(e)}")

        file_info = helpers.guess_file_info_from_response(resp)

        if not FileService.is_file_size_within_limit(extension=file_info.extension, file_size=file_info.size):
            raise FileTooLargeError

        content = resp.content if resp.request.method == "GET" else ssrf_proxy.get(url).content

        try:
            upload_file = FileService(db.engine).upload_file(
                filename=file_info.filename,
                content=content,
                mimetype=file_info.mimetype,
                user=end_user,
                source_url=url,
            )
        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError

        payload1 = FileWithSignedUrl(
            id=upload_file.id,
            name=upload_file.name,
            size=upload_file.size,
            extension=upload_file.extension,
            url=file_helpers.get_signed_file_url(upload_file_id=upload_file.id),
            mime_type=upload_file.mime_type,
            created_by=upload_file.created_by,
            created_at=int(upload_file.created_at.timestamp()),
        )
        return payload1.model_dump(mode="json"), 201

```

### api/controllers/web/audio.py
```py
import logging

from flask import request
from flask_restx import fields, marshal_with
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, field_validator
from werkzeug.exceptions import InternalServerError

import services
from controllers.web import web_ns
from controllers.web.error import (
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
from controllers.web.wraps import WebApiResource
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from libs.helper import uuid_value
from models.model import App
from services.audio_service import AudioService
from services.errors.audio import (
    AudioTooLargeServiceError,
    NoAudioUploadedServiceError,
    ProviderNotSupportSpeechToTextServiceError,
    UnsupportedAudioTypeServiceError,
)

from ..common.schema import register_schema_models


class TextToAudioPayload(BaseModel):
    message_id: str | None = None
    voice: str | None = None
    text: str | None = None
    streaming: bool | None = None

    @field_validator("message_id")
    @classmethod
    def validate_message_id(cls, value: str | None) -> str | None:
        if value is None:
            return value
        return uuid_value(value)


register_schema_models(web_ns, TextToAudioPayload)

logger = logging.getLogger(__name__)


@web_ns.route("/audio-to-text")
class AudioApi(WebApiResource):
    audio_to_text_response_fields = {
        "text": fields.String,
    }

    @marshal_with(audio_to_text_response_fields)
    @web_ns.doc("Audio to Text")
    @web_ns.doc(description="Convert audio file to text using speech-to-text service.")
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            413: "Audio file too large",
            415: "Unsupported audio type",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model: App, end_user):
        """Convert audio to text"""
        file = request.files["file"]

        try:
            response = AudioService.transcript_asr(app_model=app_model, file=file, end_user=end_user)

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
            logger.exception("Failed to handle post request to AudioApi")
            raise InternalServerError()


@web_ns.route("/text-to-audio")
class TextApi(WebApiResource):
    @web_ns.expect(web_ns.models[TextToAudioPayload.__name__])
    @web_ns.doc("Text to Audio")
    @web_ns.doc(description="Convert text to audio using text-to-speech service.")
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model: App, end_user):
        """Convert text to audio"""
        try:
            payload = TextToAudioPayload.model_validate(web_ns.payload or {})

            message_id = payload.message_id
            text = payload.text
            voice = payload.voice
            response = AudioService.transcript_tts(
                app_model=app_model, text=text, voice=voice, end_user=end_user.external_user_id, message_id=message_id
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
            logger.exception("Failed to handle post request to TextApi")
            raise InternalServerError()

```

### api/controllers/web/workflow.py
```py
import logging
from typing import Any

from graphon.graph_engine.manager import GraphEngineManager
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from werkzeug.exceptions import InternalServerError

from controllers.common.schema import register_schema_models
from controllers.web import web_ns
from controllers.web.error import (
    CompletionRequestError,
    NotWorkflowAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from controllers.web.wraps import WebApiResource
from core.app.apps.base_app_queue_manager import AppQueueManager
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from extensions.ext_redis import redis_client
from libs import helper
from models.model import App, AppMode, EndUser
from services.app_generate_service import AppGenerateService
from services.errors.llm import InvokeRateLimitError


class WorkflowRunPayload(BaseModel):
    inputs: dict[str, Any] = Field(description="Input variables for the workflow")
    files: list[dict[str, Any]] | None = Field(default=None, description="Files to be processed by the workflow")


logger = logging.getLogger(__name__)

register_schema_models(web_ns, WorkflowRunPayload)


@web_ns.route("/workflows/run")
class WorkflowRunApi(WebApiResource):
    @web_ns.doc("Run Workflow")
    @web_ns.doc(description="Execute a workflow with provided inputs and files.")
    @web_ns.expect(web_ns.models[WorkflowRunPayload.__name__])
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "App Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model: App, end_user: EndUser):
        """
        Run workflow
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        payload = WorkflowRunPayload.model_validate(web_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.WEB_APP, streaming=True
            )

            return helper.compact_generate_response(response)
        except ProviderTokenNotInitError as ex:
            raise ProviderNotInitializeError(ex.description)
        except QuotaExceededError:
            raise ProviderQuotaExceededError()
        except ModelCurrentlyNotSupportError:
            raise ProviderModelCurrentlyNotSupportError()
        except InvokeError as e:
            raise CompletionRequestError(e.description)
        except InvokeRateLimitError as ex:
            raise InvokeRateLimitHttpError(ex.description)
        except ValueError as e:
            raise e
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@web_ns.route("/workflows/tasks/<string:task_id>/stop")
class WorkflowTaskStopApi(WebApiResource):
    @web_ns.doc("Stop Workflow Task")
    @web_ns.doc(description="Stop a running workflow task.")
    @web_ns.doc(
        params={
            "task_id": {"description": "Task ID to stop", "type": "string", "required": True},
        }
    )
    @web_ns.doc(
        responses={
            200: "Success",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Task Not Found",
            500: "Internal Server Error",
        }
    )
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """
        Stop workflow task
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        # Stop using both mechanisms for backward compatibility
        # Legacy stop flag mechanism (without user check)
        AppQueueManager.set_stop_flag_no_user_check(task_id)

        # New graph engine command channel mechanism
        GraphEngineManager(redis_client).send_stop_command(task_id)

        return {"result": "success"}

```

### api/controllers/web/login.py
```py
from flask import make_response, request
from flask_restx import Resource
from jwt import InvalidTokenError
from pydantic import BaseModel, Field, field_validator

import services
from configs import dify_config
from controllers.common.schema import register_schema_models
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    InvalidEmailError,
)
from controllers.console.error import AccountBannedError
from controllers.console.wraps import (
    decrypt_code_field,
    decrypt_password_field,
    only_edition_enterprise,
    setup_required,
)
from controllers.web import web_ns
from controllers.web.wraps import decode_jwt_token
from libs.helper import EmailStr
from libs.passport import PassportService
from libs.password import valid_password
from libs.token import (
    clear_webapp_access_token_from_cookie,
    extract_webapp_access_token,
)
from services.account_service import AccountService
from services.app_service import AppService
from services.webapp_auth_service import WebAppAuthService


class LoginPayload(BaseModel):
    email: EmailStr
    password: str

    @field_validator("password")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


class EmailCodeLoginSendPayload(BaseModel):
    email: EmailStr
    language: str | None = None


class EmailCodeLoginVerifyPayload(BaseModel):
    email: EmailStr
    code: str
    token: str = Field(min_length=1)


register_schema_models(web_ns, LoginPayload, EmailCodeLoginSendPayload, EmailCodeLoginVerifyPayload)


@web_ns.route("/login")
class LoginApi(Resource):
    """Resource for web app email/password login."""

    @web_ns.expect(web_ns.models[LoginPayload.__name__])
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("web_app_login")
    @web_ns.doc(description="Authenticate user for web application access")
    @web_ns.doc(
        responses={
            200: "Authentication successful",
            400: "Bad request - invalid email or password format",
            401: "Authentication failed - email or password mismatch",
            403: "Account banned or login disabled",
            404: "Account not found",
        }
    )
    @decrypt_password_field
    def post(self):
        """Authenticate user and login."""
        payload = LoginPayload.model_validate(web_ns.payload or {})

        try:
            account = WebAppAuthService.authenticate(payload.email, payload.password)
        except services.errors.account.AccountLoginError:
            raise AccountBannedError()
        except services.errors.account.AccountPasswordError:
            raise AuthenticationFailedError()
        except services.errors.account.AccountNotFoundError:
            raise AuthenticationFailedError()

        token = WebAppAuthService.login(account=account)
        response = make_response({"result": "success", "data": {"access_token": token}})
        # set_access_token_to_cookie(request, response, token, samesite="None", httponly=False)
        return response


# this api helps frontend to check whether user is authenticated
# TODO: remove in the future. frontend should redirect to login page by catching 401 status
@web_ns.route("/login/status")
class LoginStatusApi(Resource):
    @setup_required
    @web_ns.doc("web_app_login_status")
    @web_ns.doc(description="Check login status")
    @web_ns.doc(
        responses={
            200: "Login status",
            401: "Login status",
        }
    )
    def get(self):
        app_code = request.args.get("app_code")
        user_id = request.args.get("user_id")
        token = extract_webapp_access_token(request)
        if not app_code:
            return {
                "logged_in": bool(token),
                "app_logged_in": False,
            }
        app_id = AppService.get_app_id_by_code(app_code)
        is_public = not dify_config.ENTERPRISE_ENABLED or not WebAppAuthService.is_app_require_permission_check(
            app_id=app_id
        )
        user_logged_in = False

        if is_public:
            user_logged_in = True
        else:
            try:
                PassportService().verify(token=token)
                user_logged_in = True
            except Exception:
                user_logged_in = False

        try:
            _ = decode_jwt_token(app_code=app_code, user_id=user_id)
            app_logged_in = True
        except Exception:
            app_logged_in = False

        return {
            "logged_in": user_logged_in,
            "app_logged_in": app_logged_in,
        }


@web_ns.route("/logout")
class LogoutApi(Resource):
    @setup_required
    @web_ns.doc("web_app_logout")
    @web_ns.doc(description="Logout user from web application")
    @web_ns.doc(
        responses={
            200: "Logout successful",
        }
    )
    def post(self):
        response = make_response({"result": "success"})
        # enterprise SSO sets same site to None in https deployment
        # so we need to logout by calling api
        clear_webapp_access_token_from_cookie(response, samesite="None")
        return response


@web_ns.route("/email-code-login")
class EmailCodeLoginSendEmailApi(Resource):
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("send_email_code_login")
    @web_ns.doc(description="Send email verification code for login")
    @web_ns.expect(web_ns.models[EmailCodeLoginSendPayload.__name__])
    @web_ns.doc(
        responses={
            200: "Email code sent successfully",
            400: "Bad request - invalid email format",
            404: "Account not found",
        }
    )
    def post(self):
        payload = EmailCodeLoginSendPayload.model_validate(web_ns.payload or {})

        if payload.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        account = WebAppAuthService.get_user_through_email(payload.email)
        if account is None:
            raise AuthenticationFailedError()
        else:
            token = WebAppAuthService.send_email_code_login_email(account=account, language=language)
        return {"result": "success", "data": token}


@web_ns.route("/email-code-login/validity")
class EmailCodeLoginApi(Resource):
    @setup_required
    @only_edition_enterprise
    @web_ns.doc("verify_email_code_login")
    @web_ns.doc(description="Verify email code and complete login")
    @web_ns.expect(web_ns.models[EmailCodeLoginVerifyPayload.__name__])
    @web_ns.doc(
        responses={
            200: "Email code verified and login successful",
            400: "Bad request - invalid code or token",
            401: "Invalid token or expired code",
            404: "Account not found",
        }
    )
    @decrypt_code_field
    def post(self):
        payload = EmailCodeLoginVerifyPayload.model_validate(web_ns.payload or {})

        user_email = payload.email.lower()

        token_data = WebAppAuthService.get_email_code_login_data(payload.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        if not isinstance(token_email, str):
            raise InvalidEmailError()
        normalized_token_email = token_email.lower()
        if normalized_token_email != user_email:
            raise InvalidEmailError()

        if token_data["code"] != payload.code:
            raise EmailCodeError()

        WebAppAuthService.revoke_email_code_login_token(payload.token)
        account = WebAppAuthService.get_user_through_email(token_email)
        if not account:
            raise AuthenticationFailedError()

        token = WebAppAuthService.login(account=account)
        AccountService.reset_login_error_rate_limit(user_email)
        response = make_response({"result": "success", "data": {"access_token": token}})
        # set_access_token_to_cookie(request, response, token, samesite="None", httponly=False)
        return response

```

### api/controllers/web/wraps.py
```py
from collections.abc import Callable
from datetime import UTC, datetime
from functools import wraps
from typing import Concatenate

from flask import request
from flask_restx import Resource
from sqlalchemy import select
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, NotFound, Unauthorized

from constants import HEADER_NAME_APP_CODE
from controllers.web.error import WebAppAuthAccessDeniedError, WebAppAuthRequiredError
from extensions.ext_database import db
from libs.passport import PassportService
from libs.token import extract_webapp_passport
from models.model import App, EndUser, Site
from services.app_service import AppService
from services.enterprise.enterprise_service import EnterpriseService, WebAppSettings
from services.feature_service import FeatureService
from services.webapp_auth_service import WebAppAuthService


def validate_jwt_token[**P, R](
    view: Callable[Concatenate[App, EndUser, P], R] | None = None,
) -> Callable[P, R] | Callable[[Callable[Concatenate[App, EndUser, P], R]], Callable[P, R]]:
    def decorator(view: Callable[Concatenate[App, EndUser, P], R]) -> Callable[P, R]:
        @wraps(view)
        def decorated(*args: P.args, **kwargs: P.kwargs) -> R:
            app_model, end_user = decode_jwt_token()
            return view(app_model, end_user, *args, **kwargs)

        return decorated

    if view:
        return decorator(view)
    return decorator


def decode_jwt_token(app_code: str | None = None, user_id: str | None = None) -> tuple[App, EndUser]:
    system_features = FeatureService.get_system_features()
    if not app_code:
        app_code = str(request.headers.get(HEADER_NAME_APP_CODE))
    try:
        tk = extract_webapp_passport(app_code, request)
        if not tk:
            raise Unauthorized("App token is missing.")
        decoded = PassportService().verify(tk)
        app_code = decoded.get("app_code")
        app_id = decoded.get("app_id")
        with sessionmaker(db.engine, expire_on_commit=False).begin() as session:
            app_model = session.scalar(select(App).where(App.id == app_id))
            site = session.scalar(select(Site).where(Site.code == app_code))
            if not app_model:
                raise NotFound()
            if not app_code or not site:
                raise BadRequest("Site URL is no longer valid.")
            if app_model.enable_site is False:
                raise BadRequest("Site is disabled.")
            end_user_id = decoded.get("end_user_id")
            end_user = session.scalar(select(EndUser).where(EndUser.id == end_user_id))
            if not end_user:
                raise NotFound()

            # Validate user_id against end_user's session_id if provided
            if user_id is not None and end_user.session_id != user_id:
                raise Unauthorized("Authentication has expired.")

        # for enterprise webapp auth
        app_web_auth_enabled = False
        webapp_settings = None
        if system_features.webapp_auth.enabled:
            app_id = AppService.get_app_id_by_code(app_code)
            webapp_settings = EnterpriseService.WebAppAuth.get_app_access_mode_by_id(app_id)
            if not webapp_settings:
                raise NotFound("Web app settings not found.")
            app_web_auth_enabled = webapp_settings.access_mode != "public"

        _validate_webapp_token(decoded, app_web_auth_enabled, system_features.webapp_auth.enabled)
        _validate_user_accessibility(
            decoded, app_code, app_web_auth_enabled, system_features.webapp_auth.enabled, webapp_settings
        )

        return app_model, end_user
    except Unauthorized as e:
        if system_features.webapp_auth.enabled:
            if not app_code:
                raise Unauthorized("Please re-login to access the web app.")
            app_id = AppService.get_app_id_by_code(app_code)
            app_web_auth_enabled = (
                EnterpriseService.WebAppAuth.get_app_access_mode_by_id(app_id=app_id).access_mode != "public"
            )
            if app_web_auth_enabled:
                raise WebAppAuthRequiredError()

        raise Unauthorized(e.description)


def _validate_webapp_token(decoded, app_web_auth_enabled: bool, system_webapp_auth_enabled: bool):
    # Check if authentication is enforced for web app, and if the token source is not webapp,
    # raise an error and redirect to login
    if system_webapp_auth_enabled and app_web_auth_enabled:
        source = decoded.get("token_source")
        if not source or source != "webapp":
            raise WebAppAuthRequiredError()

    # Check if authentication is not enforced for web, and if the token source is webapp,
    # raise an error and redirect to normal passport login
    if not system_webapp_auth_enabled or not app_web_auth_enabled:
        source = decoded.get("token_source")
        if source and source == "webapp":
            raise Unauthorized("webapp token expired.")


def _validate_user_accessibility(
    decoded,
    app_code,
    app_web_auth_enabled: bool,
    system_webapp_auth_enabled: bool,
    webapp_settings: WebAppSettings | None,
):
    if system_webapp_auth_enabled and app_web_auth_enabled:
        # Check if the user is allowed to access the web app
        user_id = decoded.get("user_id")
        if not user_id:
            raise WebAppAuthRequiredError()

        if not webapp_settings:
            raise WebAppAuthRequiredError("Web app settings not found.")

        if WebAppAuthService.is_app_require_permission_check(access_mode=webapp_settings.access_mode):
            app_id = AppService.get_app_id_by_code(app_code)
            if not EnterpriseService.WebAppAuth.is_user_allowed_to_access_webapp(user_id, app_id):
                raise WebAppAuthAccessDeniedError()

        auth_type = decoded.get("auth_type")
        granted_at = decoded.get("granted_at")
        if not auth_type:
            raise WebAppAuthAccessDeniedError("Missing auth_type in the token.")
        if not granted_at:
            raise WebAppAuthAccessDeniedError("Missing granted_at in the token.")
        # check if sso has been updated
        if auth_type == "external":
            last_update_time = EnterpriseService.get_app_sso_settings_last_update_time()
            if granted_at and datetime.fromtimestamp(granted_at, tz=UTC) < last_update_time:
                raise WebAppAuthAccessDeniedError("SSO settings have been updated. Please re-login.")
        elif auth_type == "internal":
            last_update_time = EnterpriseService.get_workspace_sso_settings_last_update_time()
            if granted_at and datetime.fromtimestamp(granted_at, tz=UTC) < last_update_time:
                raise WebAppAuthAccessDeniedError("SSO settings have been updated. Please re-login.")


class WebApiResource(Resource):
    method_decorators = [validate_jwt_token]

```

### api/controllers/web/workflow_events.py
```py
"""
Web App Workflow Resume APIs.
"""

import json
from collections.abc import Generator

from flask import Response, request
from sqlalchemy.orm import sessionmaker

from controllers.web import api
from controllers.web.error import InvalidArgumentError, NotFoundError
from controllers.web.wraps import WebApiResource
from core.app.apps.advanced_chat.app_generator import AdvancedChatAppGenerator
from core.app.apps.base_app_generator import BaseAppGenerator
from core.app.apps.common.workflow_response_converter import WorkflowResponseConverter
from core.app.apps.message_generator import MessageGenerator
from core.app.apps.workflow.app_generator import WorkflowAppGenerator
from extensions.ext_database import db
from models.enums import CreatorUserRole
from models.model import App, AppMode, EndUser
from repositories.factory import DifyAPIRepositoryFactory
from services.workflow_event_snapshot_service import build_workflow_event_stream


class WorkflowEventsApi(WebApiResource):
    """API for getting workflow execution events after resume."""

    def get(self, app_model: App, end_user: EndUser, task_id: str):
        """
        Get workflow execution events stream after resume.

        GET /api/workflow/<task_id>/events

        Returns Server-Sent Events stream.
        """
        workflow_run_id = task_id
        session_maker = sessionmaker(db.engine)
        repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)
        workflow_run = repo.get_workflow_run_by_id_and_tenant_id(
            tenant_id=app_model.tenant_id,
            run_id=workflow_run_id,
        )

        if workflow_run is None:
            raise NotFoundError(f"WorkflowRun not found, id={workflow_run_id}")

        if workflow_run.app_id != app_model.id:
            raise NotFoundError(f"WorkflowRun not found, id={workflow_run_id}")

        if workflow_run.created_by_role != CreatorUserRole.END_USER:
            raise NotFoundError(f"WorkflowRun not created by end user, id={workflow_run_id}")

        if workflow_run.created_by != end_user.id:
            raise NotFoundError(f"WorkflowRun not created by the current end user, id={workflow_run_id}")

        if workflow_run.finished_at is not None:
            response = WorkflowResponseConverter.workflow_run_result_to_finish_response(
                task_id=workflow_run.id,
                workflow_run=workflow_run,
                creator_user=end_user,
            )

            payload = response.model_dump(mode="json")
            payload["event"] = response.event.value

            def _generate_finished_events() -> Generator[str, None, None]:
                yield f"data: {json.dumps(payload)}\n\n"

            event_generator = _generate_finished_events
        else:
            app_mode = AppMode.value_of(app_model.mode)
            msg_generator = MessageGenerator()
            generator: BaseAppGenerator
            if app_mode == AppMode.ADVANCED_CHAT:
                generator = AdvancedChatAppGenerator()
            elif app_mode == AppMode.WORKFLOW:
                generator = WorkflowAppGenerator()
            else:
                raise InvalidArgumentError(f"cannot subscribe to workflow run, workflow_run_id={workflow_run.id}")

            include_state_snapshot = request.args.get("include_state_snapshot", "false").lower() == "true"

            def _generate_stream_events():
                if include_state_snapshot:
                    return generator.convert_to_event_stream(
                        build_workflow_event_stream(
                            app_mode=app_mode,
                            workflow_run=workflow_run,
                            tenant_id=app_model.tenant_id,
                            app_id=app_model.id,
                            session_maker=session_maker,
                        )
                    )
                return generator.convert_to_event_stream(
                    msg_generator.retrieve_events(app_mode, workflow_run.id),
                )

            event_generator = _generate_stream_events

        return Response(
            event_generator(),
            mimetype="text/event-stream",
            headers={
                "Cache-Control": "no-cache",
                "Connection": "keep-alive",
            },
        )


# Register the APIs
api.add_resource(WorkflowEventsApi, "/workflow/<string:task_id>/events")

```

### web/app/(shareLayout)/chat/[token]/page.tsx
```tsx
'use client'
import * as React from 'react'
import ChatWithHistoryWrap from '@/app/components/base/chat/chat-with-history'
import AuthenticatedLayout from '../../components/authenticated-layout'

const Chat = () => {
  return (
    <AuthenticatedLayout>
      <ChatWithHistoryWrap />
    </AuthenticatedLayout>
  )
}

export default React.memo(Chat)

```

### web/app/(shareLayout)/chatbot/[token]/page.tsx
```tsx
'use client'
import * as React from 'react'
import EmbeddedChatbot from '@/app/components/base/chat/embedded-chatbot'
import AuthenticatedLayout from '../../components/authenticated-layout'

const Chatbot = () => {
  return (
    <AuthenticatedLayout>
      <EmbeddedChatbot />
    </AuthenticatedLayout>
  )
}

export default React.memo(Chatbot)

```

### web/app/(shareLayout)/completion/[token]/page.tsx
```tsx
import * as React from 'react'
import Main from '@/app/components/share/text-generation'
import AuthenticatedLayout from '../../components/authenticated-layout'

const Completion = () => {
  return (
    <AuthenticatedLayout>
      <Main />
    </AuthenticatedLayout>
  )
}

export default React.memo(Completion)

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-003.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
