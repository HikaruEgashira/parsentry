You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-002
- **Kind**: endpoint
- **Identifier**: Service API App Endpoints (/v1/chat-messages, /v1/completion-messages, /v1/workflows/run)
- **Description**: External-facing API endpoints authenticated by API key. Accept untrusted user input for LLM prompts, enabling prompt injection, API key leakage, and rate limit bypass.
- **Locations**: api/controllers/service_api/app/

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

### api/controllers/service_api/app/file_preview.py
```py
import logging
from urllib.parse import quote

from flask import Response, request
from flask_restx import Resource
from pydantic import BaseModel, Field
from sqlalchemy import select

from controllers.common.file_response import enforce_download_for_html
from controllers.common.schema import register_schema_model
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
    FileAccessDeniedError,
    FileNotFoundError,
)
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from extensions.ext_database import db
from extensions.ext_storage import storage
from models.model import App, EndUser, Message, MessageFile, UploadFile

logger = logging.getLogger(__name__)


class FilePreviewQuery(BaseModel):
    as_attachment: bool = Field(default=False, description="Download as attachment")


register_schema_model(service_api_ns, FilePreviewQuery)


@service_api_ns.route("/files/<uuid:file_id>/preview")
class FilePreviewApi(Resource):
    """
    Service API File Preview endpoint

    Provides secure file preview/download functionality for external API users.
    Files can only be accessed if they belong to messages within the requesting app's context.
    """

    @service_api_ns.expect(service_api_ns.models[FilePreviewQuery.__name__])
    @service_api_ns.doc("preview_file")
    @service_api_ns.doc(description="Preview or download a file uploaded via Service API")
    @service_api_ns.doc(params={"file_id": "UUID of the file to preview"})
    @service_api_ns.doc(
        responses={
            200: "File retrieved successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - file access denied",
            404: "File not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.QUERY))
    def get(self, app_model: App, end_user: EndUser, file_id: str):
        """
        Preview/Download a file that was uploaded via Service API.

        Provides secure file preview/download functionality.
        Files can only be accessed if they belong to messages within the requesting app's context.
        """
        file_id = str(file_id)

        # Parse query parameters
        args = FilePreviewQuery.model_validate(request.args.to_dict())

        # Validate file ownership and get file objects
        _, upload_file = self._validate_file_ownership(file_id, app_model.id)

        # Get file content generator
        try:
            generator = storage.load(upload_file.key, stream=True)
        except Exception as e:
            raise FileNotFoundError(f"Failed to load file content: {str(e)}")

        # Build response with appropriate headers
        response = self._build_file_response(generator, upload_file, args.as_attachment)

        return response

    def _validate_file_ownership(self, file_id: str, app_id: str) -> tuple[MessageFile, UploadFile]:
        """
        Validate that the file belongs to a message within the requesting app's context

        Security validations performed:
        1. File exists in MessageFile table (was used in a conversation)
        2. Message belongs to the requesting app
        3. UploadFile record exists and is accessible
        4. File tenant matches app tenant (additional security layer)

        Args:
            file_id: UUID of the file to validate
            app_id: UUID of the requesting app

        Returns:
            Tuple of (MessageFile, UploadFile) if validation passes

        Raises:
            FileNotFoundError: File or related records not found
            FileAccessDeniedError: File does not belong to the app's context
        """
        try:
            # Input validation
            if not file_id or not app_id:
                raise FileAccessDeniedError("Invalid file or app identifier")

            # First, find the MessageFile that references this upload file
            message_file = db.session.scalar(select(MessageFile).where(MessageFile.upload_file_id == file_id).limit(1))

            if not message_file:
                raise FileNotFoundError("File not found in message context")

            # Get the message and verify it belongs to the requesting app
            message = db.session.scalar(
                select(Message).where(Message.id == message_file.message_id, Message.app_id == app_id).limit(1)
            )

            if not message:
                raise FileAccessDeniedError("File access denied: not owned by requesting app")

            # Get the actual upload file record
            upload_file = db.session.get(UploadFile, file_id)

            if not upload_file:
                raise FileNotFoundError("Upload file record not found")

            # Additional security: verify tenant isolation
            app = db.session.get(App, app_id)
            if app and upload_file.tenant_id != app.tenant_id:
                raise FileAccessDeniedError("File access denied: tenant mismatch")

            return message_file, upload_file

        except (FileNotFoundError, FileAccessDeniedError):
            # Re-raise our custom exceptions
            raise
        except Exception as e:
            # Log unexpected errors for debugging
            logger.exception(
                "Unexpected error during file ownership validation",
                extra={"file_id": file_id, "app_id": app_id, "error": str(e)},
            )
            raise FileAccessDeniedError("File access validation failed")

    def _build_file_response(self, generator, upload_file: UploadFile, as_attachment: bool = False) -> Response:
        """
        Build Flask Response object with appropriate headers for file streaming

        Args:
            generator: File content generator from storage
            upload_file: UploadFile database record
            as_attachment: Whether to set Content-Disposition as attachment

        Returns:
            Flask Response object with streaming file content
        """
        response = Response(
            generator,
            mimetype=upload_file.mime_type,
            direct_passthrough=True,
            headers={},
        )

        # Add Content-Length if known
        if upload_file.size and upload_file.size > 0:
            response.headers["Content-Length"] = str(upload_file.size)

        # Add Accept-Ranges header for audio/video files to support seeking
        if upload_file.mime_type in [
            "audio/mpeg",
            "audio/wav",
            "audio/mp4",
            "audio/ogg",
            "audio/flac",
            "audio/aac",
            "video/mp4",
            "video/webm",
            "video/quicktime",
            "audio/x-m4a",
        ]:
            response.headers["Accept-Ranges"] = "bytes"

        # Set Content-Disposition for downloads
        if as_attachment and upload_file.name:
            encoded_filename = quote(upload_file.name)
            response.headers["Content-Disposition"] = f"attachment; filename*=UTF-8''{encoded_filename}"
            # Override content-type for downloads to force download
            response.headers["Content-Type"] = "application/octet-stream"

        enforce_download_for_html(
            response,
            mime_type=upload_file.mime_type,
            filename=upload_file.name,
            extension=upload_file.extension,
        )

        # Add caching headers for performance
        response.headers["Cache-Control"] = "public, max-age=3600"  # Cache for 1 hour

        return response

```

### api/controllers/service_api/app/annotation.py
```py
from typing import Literal

from flask import request
from flask_restx import Resource
from flask_restx.api import HTTPStatus
from pydantic import BaseModel, Field, TypeAdapter

from controllers.common.schema import register_schema_models
from controllers.console.wraps import edit_permission_required
from controllers.service_api import service_api_ns
from controllers.service_api.wraps import validate_app_token
from extensions.ext_redis import redis_client
from fields.annotation_fields import Annotation, AnnotationList
from models.model import App
from services.annotation_service import AppAnnotationService


class AnnotationCreatePayload(BaseModel):
    question: str = Field(description="Annotation question")
    answer: str = Field(description="Annotation answer")


class AnnotationReplyActionPayload(BaseModel):
    score_threshold: float = Field(description="Score threshold for annotation matching")
    embedding_provider_name: str = Field(description="Embedding provider name")
    embedding_model_name: str = Field(description="Embedding model name")


register_schema_models(
    service_api_ns, AnnotationCreatePayload, AnnotationReplyActionPayload, Annotation, AnnotationList
)


@service_api_ns.route("/apps/annotation-reply/<string:action>")
class AnnotationReplyActionApi(Resource):
    @service_api_ns.expect(service_api_ns.models[AnnotationReplyActionPayload.__name__])
    @service_api_ns.doc("annotation_reply_action")
    @service_api_ns.doc(description="Enable or disable annotation reply feature")
    @service_api_ns.doc(params={"action": "Action to perform: 'enable' or 'disable'"})
    @service_api_ns.doc(
        responses={
            200: "Action completed successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @validate_app_token
    def post(self, app_model: App, action: Literal["enable", "disable"]):
        """Enable or disable annotation reply feature."""
        args = AnnotationReplyActionPayload.model_validate(service_api_ns.payload or {}).model_dump()
        match action:
            case "enable":
                result = AppAnnotationService.enable_app_annotation(args, app_model.id)
            case "disable":
                result = AppAnnotationService.disable_app_annotation(app_model.id)
        return result, 200


@service_api_ns.route("/apps/annotation-reply/<string:action>/status/<uuid:job_id>")
class AnnotationReplyActionStatusApi(Resource):
    @service_api_ns.doc("get_annotation_reply_action_status")
    @service_api_ns.doc(description="Get the status of an annotation reply action job")
    @service_api_ns.doc(params={"action": "Action type", "job_id": "Job ID"})
    @service_api_ns.doc(
        responses={
            200: "Job status retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Job not found",
        }
    )
    @validate_app_token
    def get(self, app_model: App, job_id, action):
        """Get the status of an annotation reply action job."""
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


@service_api_ns.route("/apps/annotations")
class AnnotationListApi(Resource):
    @service_api_ns.doc("list_annotations")
    @service_api_ns.doc(description="List annotations for the application")
    @service_api_ns.doc(
        responses={
            200: "Annotations retrieved successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @service_api_ns.response(
        200,
        "Annotations retrieved successfully",
        service_api_ns.models[AnnotationList.__name__],
    )
    @validate_app_token
    def get(self, app_model: App):
        """List annotations for the application."""
        page = request.args.get("page", default=1, type=int)
        limit = request.args.get("limit", default=20, type=int)
        keyword = request.args.get("keyword", default="", type=str)

        annotation_list, total = AppAnnotationService.get_annotation_list_by_app_id(app_model.id, page, limit, keyword)
        annotation_models = TypeAdapter(list[Annotation]).validate_python(annotation_list, from_attributes=True)
        response = AnnotationList(
            data=annotation_models,
            has_more=len(annotation_list) == limit,
            limit=limit,
            total=total,
            page=page,
        )
        return response.model_dump(mode="json")

    @service_api_ns.expect(service_api_ns.models[AnnotationCreatePayload.__name__])
    @service_api_ns.doc("create_annotation")
    @service_api_ns.doc(description="Create a new annotation")
    @service_api_ns.doc(
        responses={
            201: "Annotation created successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @service_api_ns.response(
        HTTPStatus.CREATED,
        "Annotation created successfully",
        service_api_ns.models[Annotation.__name__],
    )
    @validate_app_token
    def post(self, app_model: App):
        """Create a new annotation."""
        args = AnnotationCreatePayload.model_validate(service_api_ns.payload or {}).model_dump()
        annotation = AppAnnotationService.insert_app_annotation_directly(args, app_model.id)
        response = Annotation.model_validate(annotation, from_attributes=True)
        return response.model_dump(mode="json"), HTTPStatus.CREATED


@service_api_ns.route("/apps/annotations/<uuid:annotation_id>")
class AnnotationUpdateDeleteApi(Resource):
    @service_api_ns.expect(service_api_ns.models[AnnotationCreatePayload.__name__])
    @service_api_ns.doc("update_annotation")
    @service_api_ns.doc(description="Update an existing annotation")
    @service_api_ns.doc(params={"annotation_id": "Annotation ID"})
    @service_api_ns.doc(
        responses={
            200: "Annotation updated successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - insufficient permissions",
            404: "Annotation not found",
        }
    )
    @service_api_ns.response(
        200,
        "Annotation updated successfully",
        service_api_ns.models[Annotation.__name__],
    )
    @validate_app_token
    @edit_permission_required
    def put(self, app_model: App, annotation_id: str):
        """Update an existing annotation."""
        args = AnnotationCreatePayload.model_validate(service_api_ns.payload or {}).model_dump()
        annotation = AppAnnotationService.update_app_annotation_directly(args, app_model.id, annotation_id)
        response = Annotation.model_validate(annotation, from_attributes=True)
        return response.model_dump(mode="json")

    @service_api_ns.doc("delete_annotation")
    @service_api_ns.doc(description="Delete an annotation")
    @service_api_ns.doc(params={"annotation_id": "Annotation ID"})
    @service_api_ns.doc(
        responses={
            204: "Annotation deleted successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - insufficient permissions",
            404: "Annotation not found",
        }
    )
    @validate_app_token
    @edit_permission_required
    def delete(self, app_model: App, annotation_id: str):
        """Delete an annotation."""
        AppAnnotationService.delete_app_annotation(app_model.id, annotation_id)
        return "", 204

```

### api/controllers/service_api/app/error.py
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
    description = "Please check if your app mode matches the right API route."
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


class FileNotFoundError(BaseHTTPException):
    error_code = "file_not_found"
    description = "The requested file was not found."
    code = 404


class FileAccessDeniedError(BaseHTTPException):
    error_code = "file_access_denied"
    description = "Access to the requested file is denied."
    code = 403

```

### api/controllers/service_api/app/completion.py
```py
import logging
from typing import Any, Literal
from uuid import UUID

from flask import request
from flask_restx import Resource
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field, field_validator
from werkzeug.exceptions import BadRequest, InternalServerError, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
    AppUnavailableError,
    CompletionRequestError,
    ConversationCompletedError,
    NotChatAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from core.helper.trace_id_helper import get_external_trace_id
from libs import helper
from libs.helper import UUIDStrOrEmpty
from models.model import App, AppMode, EndUser
from services.app_generate_service import AppGenerateService
from services.app_task_service import AppTaskService
from services.errors.app import IsDraftWorkflowError, WorkflowIdFormatError, WorkflowNotFoundError
from services.errors.llm import InvokeRateLimitError

logger = logging.getLogger(__name__)


class CompletionRequestPayload(BaseModel):
    inputs: dict[str, Any]
    query: str = Field(default="")
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None
    retriever_from: str = Field(default="dev")


class ChatRequestPayload(BaseModel):
    inputs: dict[str, Any]
    query: str
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None
    conversation_id: UUIDStrOrEmpty | None = Field(default=None, description="Conversation UUID")
    retriever_from: str = Field(default="dev")
    auto_generate_name: bool = Field(default=True, description="Auto generate conversation name")
    workflow_id: str | None = Field(default=None, description="Workflow ID for advanced chat")

    @field_validator("conversation_id", mode="before")
    @classmethod
    def normalize_conversation_id(cls, value: str | UUID | None) -> str | None:
        """Allow missing or blank conversation IDs; enforce UUID format when provided."""
        if isinstance(value, str):
            value = value.strip()

        if not value:
            return None

        try:
            return helper.uuid_value(value)
        except ValueError as exc:
            raise ValueError("conversation_id must be a valid UUID") from exc


register_schema_models(service_api_ns, CompletionRequestPayload, ChatRequestPayload)


@service_api_ns.route("/completion-messages")
class CompletionApi(Resource):
    @service_api_ns.expect(service_api_ns.models[CompletionRequestPayload.__name__])
    @service_api_ns.doc("create_completion")
    @service_api_ns.doc(description="Create a completion for the given prompt")
    @service_api_ns.doc(
        responses={
            200: "Completion created successfully",
            400: "Bad request - invalid parameters",
            401: "Unauthorized - invalid API token",
            404: "Conversation not found",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Create a completion for the given prompt.

        This endpoint generates a completion based on the provided inputs and query.
        Supports both blocking and streaming response modes.
        """
        if app_model.mode != AppMode.COMPLETION:
            raise AppUnavailableError()

        payload = CompletionRequestPayload.model_validate(service_api_ns.payload or {})
        external_trace_id = get_external_trace_id(request)
        args = payload.model_dump(exclude_none=True)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id

        streaming = payload.response_mode == "streaming"

        args["auto_generate_name"] = False

        try:
            response = AppGenerateService.generate(
                app_model=app_model,
                user=end_user,
                args=args,
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=streaming,
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
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/completion-messages/<string:task_id>/stop")
class CompletionStopApi(Resource):
    @service_api_ns.doc("stop_completion")
    @service_api_ns.doc(description="Stop a running completion task")
    @service_api_ns.doc(params={"task_id": "The ID of the task to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running completion task."""
        if app_model.mode != AppMode.COMPLETION:
            raise AppUnavailableError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.SERVICE_API,
            user_id=end_user.id,
            app_mode=AppMode.value_of(app_model.mode),
        )

        return {"result": "success"}, 200


@service_api_ns.route("/chat-messages")
class ChatApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ChatRequestPayload.__name__])
    @service_api_ns.doc("create_chat_message")
    @service_api_ns.doc(description="Send a message in a chat conversation")
    @service_api_ns.doc(
        responses={
            200: "Message sent successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Conversation or workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Send a message in a chat conversation.

        This endpoint handles chat messages for chat, agent chat, and advanced chat applications.
        Supports conversation management and both blocking and streaming response modes.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        payload = ChatRequestPayload.model_validate(service_api_ns.payload or {})

        external_trace_id = get_external_trace_id(request)
        args = payload.model_dump(exclude_none=True)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id

        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
        except WorkflowNotFoundError as ex:
            raise NotFound(str(ex))
        except IsDraftWorkflowError as ex:
            raise BadRequest(str(ex))
        except WorkflowIdFormatError as ex:
            raise BadRequest(str(ex))
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
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/chat-messages/<string:task_id>/stop")
class ChatStopApi(Resource):
    @service_api_ns.doc("stop_chat_message")
    @service_api_ns.doc(description="Stop a running chat message generation")
    @service_api_ns.doc(params={"task_id": "The ID of the task to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running chat message generation."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        AppTaskService.stop_task(
            task_id=task_id,
            invoke_from=InvokeFrom.SERVICE_API,
            user_id=end_user.id,
            app_mode=app_mode,
        )

        return {"result": "success"}, 200

```

### api/controllers/service_api/app/conversation.py
```py
from typing import Any, Literal

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, TypeAdapter, field_validator, model_validator
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import NotChatAppError
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from core.app.entities.app_invoke_entities import InvokeFrom
from extensions.ext_database import db
from fields.conversation_fields import (
    ConversationInfiniteScrollPagination,
    SimpleConversation,
)
from fields.conversation_variable_fields import (
    build_conversation_variable_infinite_scroll_pagination_model,
    build_conversation_variable_model,
)
from libs.helper import UUIDStrOrEmpty
from models.model import App, AppMode, EndUser
from services.conversation_service import ConversationService


class ConversationListQuery(BaseModel):
    last_id: UUIDStrOrEmpty | None = Field(default=None, description="Last conversation ID for pagination")
    limit: int = Field(default=20, ge=1, le=100, description="Number of conversations to return")
    sort_by: Literal["created_at", "-created_at", "updated_at", "-updated_at"] = Field(
        default="-updated_at", description="Sort order for conversations"
    )


class ConversationRenamePayload(BaseModel):
    name: str | None = Field(default=None, description="New conversation name (required if auto_generate is false)")
    auto_generate: bool = Field(default=False, description="Auto-generate conversation name")

    @model_validator(mode="after")
    def validate_name_requirement(self):
        if not self.auto_generate:
            if self.name is None or not self.name.strip():
                raise ValueError("name is required when auto_generate is false")
        return self


class ConversationVariablesQuery(BaseModel):
    last_id: UUIDStrOrEmpty | None = Field(default=None, description="Last variable ID for pagination")
    limit: int = Field(default=20, ge=1, le=100, description="Number of variables to return")
    variable_name: str | None = Field(
        default=None, description="Filter variables by name", min_length=1, max_length=255
    )

    @field_validator("variable_name", mode="before")
    @classmethod
    def validate_variable_name(cls, v: str | None) -> str | None:
        """
        Validate variable_name to prevent injection attacks.
        """
        if v is None:
            return v

        # Only allow safe characters: alphanumeric, underscore, hyphen, period
        if not v.replace("-", "").replace("_", "").replace(".", "").isalnum():
            raise ValueError(
                "Variable name can only contain letters, numbers, hyphens (-), underscores (_), and periods (.)"
            )

        # Prevent SQL injection patterns
        dangerous_patterns = ["'", '"', ";", "--", "/*", "*/", "xp_", "sp_"]
        for pattern in dangerous_patterns:
            if pattern in v.lower():
                raise ValueError(f"Variable name contains invalid characters: {pattern}")

        return v


class ConversationVariableUpdatePayload(BaseModel):
    value: Any


register_schema_models(
    service_api_ns,
    ConversationListQuery,
    ConversationRenamePayload,
    ConversationVariablesQuery,
    ConversationVariableUpdatePayload,
)


@service_api_ns.route("/conversations")
class ConversationApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ConversationListQuery.__name__])
    @service_api_ns.doc("list_conversations")
    @service_api_ns.doc(description="List all conversations for the current user")
    @service_api_ns.doc(
        responses={
            200: "Conversations retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Last conversation not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.QUERY))
    def get(self, app_model: App, end_user: EndUser):
        """List all conversations for the current user.

        Supports pagination using last_id and limit parameters.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        query_args = ConversationListQuery.model_validate(request.args.to_dict())
        last_id = str(query_args.last_id) if query_args.last_id else None

        try:
            with sessionmaker(db.engine).begin() as session:
                pagination = ConversationService.pagination_by_last_id(
                    session=session,
                    app_model=app_model,
                    user=end_user,
                    last_id=last_id,
                    limit=query_args.limit,
                    invoke_from=InvokeFrom.SERVICE_API,
                    sort_by=query_args.sort_by,
                )
                adapter = TypeAdapter(SimpleConversation)
                conversations = [adapter.validate_python(item, from_attributes=True) for item in pagination.data]
                return ConversationInfiniteScrollPagination(
                    limit=pagination.limit,
                    has_more=pagination.has_more,
                    data=conversations,
                ).model_dump(mode="json")
        except services.errors.conversation.LastConversationNotExistsError:
            raise NotFound("Last Conversation Not Exists.")


@service_api_ns.route("/conversations/<uuid:c_id>")
class ConversationDetailApi(Resource):
    @service_api_ns.doc("delete_conversation")
    @service_api_ns.doc(description="Delete a specific conversation")
    @service_api_ns.doc(params={"c_id": "Conversation ID"})
    @service_api_ns.doc(
        responses={
            204: "Conversation deleted successfully",
            401: "Unauthorized - invalid API token",
            404: "Conversation not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON))
    def delete(self, app_model: App, end_user: EndUser, c_id):
        """Delete a specific conversation."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)

        try:
            ConversationService.delete(app_model, conversation_id, end_user)
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        return "", 204


@service_api_ns.route("/conversations/<uuid:c_id>/name")
class ConversationRenameApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ConversationRenamePayload.__name__])
    @service_api_ns.doc("rename_conversation")
    @service_api_ns.doc(description="Rename a conversation or auto-generate a name")
    @service_api_ns.doc(params={"c_id": "Conversation ID"})
    @service_api_ns.doc(
        responses={
            200: "Conversation renamed successfully",
            401: "Unauthorized - invalid API token",
            404: "Conversation not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON))
    def post(self, app_model: App, end_user: EndUser, c_id):
        """Rename a conversation or auto-generate a name."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)

        payload = ConversationRenamePayload.model_validate(service_api_ns.payload or {})

        try:
            conversation = ConversationService.rename(
                app_model, conversation_id, end_user, payload.name, payload.auto_generate
            )
            return (
                TypeAdapter(SimpleConversation)
                .validate_python(conversation, from_attributes=True)
                .model_dump(mode="json")
            )
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")


@service_api_ns.route("/conversations/<uuid:c_id>/variables")
class ConversationVariablesApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ConversationVariablesQuery.__name__])
    @service_api_ns.doc("list_conversation_variables")
    @service_api_ns.doc(description="List all variables for a conversation")
    @service_api_ns.doc(params={"c_id": "Conversation ID"})
    @service_api_ns.doc(
        responses={
            200: "Variables retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Conversation not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.QUERY))
    @service_api_ns.marshal_with(build_conversation_variable_infinite_scroll_pagination_model(service_api_ns))
    def get(self, app_model: App, end_user: EndUser, c_id):
        """List all variables for a conversation.

        Conversational variables are only available for chat applications.
        """
        # conversational variable only for chat app
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)

        query_args = ConversationVariablesQuery.model_validate(request.args.to_dict())
        last_id = str(query_args.last_id) if query_args.last_id else None

        try:
            return ConversationService.get_conversational_variable(
                app_model, conversation_id, end_user, query_args.limit, last_id, query_args.variable_name
            )
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")


@service_api_ns.route("/conversations/<uuid:c_id>/variables/<uuid:variable_id>")
class ConversationVariableDetailApi(Resource):
    @service_api_ns.expect(service_api_ns.models[ConversationVariableUpdatePayload.__name__])
    @service_api_ns.doc("update_conversation_variable")
    @service_api_ns.doc(description="Update a conversation variable's value")
    @service_api_ns.doc(params={"c_id": "Conversation ID", "variable_id": "Variable ID"})
    @service_api_ns.doc(
        responses={
            200: "Variable updated successfully",
            400: "Bad request - type mismatch",
            401: "Unauthorized - invalid API token",
            404: "Conversation or variable not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON))
    @service_api_ns.marshal_with(build_conversation_variable_model(service_api_ns))
    def put(self, app_model: App, end_user: EndUser, c_id, variable_id):
        """Update a conversation variable's value.

        Allows updating the value of a specific conversation variable.
        The value must match the variable's expected type.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        conversation_id = str(c_id)
        variable_id = str(variable_id)

        payload = ConversationVariableUpdatePayload.model_validate(service_api_ns.payload or {})

        try:
            return ConversationService.update_conversation_variable(
                app_model, conversation_id, variable_id, end_user, payload.value
            )
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except services.errors.conversation.ConversationVariableNotExistsError:
            raise NotFound("Conversation Variable Not Exists.")
        except services.errors.conversation.ConversationVariableTypeMismatchError as e:
            raise BadRequest(str(e))

```

### api/controllers/service_api/app/__init__.py
```py

```

### api/controllers/service_api/app/message.py
```py
import logging
from typing import Literal

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, TypeAdapter
from werkzeug.exceptions import BadRequest, InternalServerError, NotFound

import services
from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import NotChatAppError
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from core.app.entities.app_invoke_entities import InvokeFrom
from fields.conversation_fields import ResultResponse
from fields.message_fields import MessageInfiniteScrollPagination, MessageListItem
from libs.helper import UUIDStrOrEmpty
from models.enums import FeedbackRating
from models.model import App, AppMode, EndUser
from services.errors.message import (
    FirstMessageNotExistsError,
    MessageNotExistsError,
    SuggestedQuestionsAfterAnswerDisabledError,
)
from services.message_service import MessageService

logger = logging.getLogger(__name__)


class MessageListQuery(BaseModel):
    conversation_id: UUIDStrOrEmpty
    first_id: UUIDStrOrEmpty | None = None
    limit: int = Field(default=20, ge=1, le=100, description="Number of messages to return")


class MessageFeedbackPayload(BaseModel):
    rating: Literal["like", "dislike"] | None = Field(default=None, description="Feedback rating")
    content: str | None = Field(default=None, description="Feedback content")


class FeedbackListQuery(BaseModel):
    page: int = Field(default=1, ge=1, description="Page number")
    limit: int = Field(default=20, ge=1, le=101, description="Number of feedbacks per page")


register_schema_models(service_api_ns, MessageListQuery, MessageFeedbackPayload, FeedbackListQuery)


@service_api_ns.route("/messages")
class MessageListApi(Resource):
    @service_api_ns.expect(service_api_ns.models[MessageListQuery.__name__])
    @service_api_ns.doc("list_messages")
    @service_api_ns.doc(description="List messages in a conversation")
    @service_api_ns.doc(
        responses={
            200: "Messages retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Conversation or first message not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.QUERY))
    def get(self, app_model: App, end_user: EndUser):
        """List messages in a conversation.

        Retrieves messages with pagination support using first_id.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        query_args = MessageListQuery.model_validate(request.args.to_dict())
        conversation_id = str(query_args.conversation_id)
        first_id = str(query_args.first_id) if query_args.first_id else None

        try:
            pagination = MessageService.pagination_by_first_id(
                app_model, end_user, conversation_id, first_id, query_args.limit
            )
            adapter = TypeAdapter(MessageListItem)
            items = [adapter.validate_python(message, from_attributes=True) for message in pagination.data]
            return MessageInfiniteScrollPagination(
                limit=pagination.limit,
                has_more=pagination.has_more,
                data=items,
            ).model_dump(mode="json")
        except services.errors.conversation.ConversationNotExistsError:
            raise NotFound("Conversation Not Exists.")
        except FirstMessageNotExistsError:
            raise NotFound("First Message Not Exists.")


@service_api_ns.route("/messages/<uuid:message_id>/feedbacks")
class MessageFeedbackApi(Resource):
    @service_api_ns.expect(service_api_ns.models[MessageFeedbackPayload.__name__])
    @service_api_ns.doc("create_message_feedback")
    @service_api_ns.doc(description="Submit feedback for a message")
    @service_api_ns.doc(params={"message_id": "Message ID"})
    @service_api_ns.doc(
        responses={
            200: "Feedback submitted successfully",
            401: "Unauthorized - invalid API token",
            404: "Message not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, message_id):
        """Submit feedback for a message.

        Allows users to rate messages as like/dislike and provide optional feedback content.
        """
        message_id = str(message_id)

        payload = MessageFeedbackPayload.model_validate(service_api_ns.payload or {})

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


@service_api_ns.route("/app/feedbacks")
class AppGetFeedbacksApi(Resource):
    @service_api_ns.expect(service_api_ns.models[FeedbackListQuery.__name__])
    @service_api_ns.doc("get_app_feedbacks")
    @service_api_ns.doc(description="Get all feedbacks for the application")
    @service_api_ns.doc(
        responses={
            200: "Feedbacks retrieved successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @validate_app_token
    def get(self, app_model: App):
        """Get all feedbacks for the application.

        Returns paginated list of all feedback submitted for messages in this app.
        """
        query_args = FeedbackListQuery.model_validate(request.args.to_dict())
        feedbacks = MessageService.get_all_messages_feedbacks(app_model, page=query_args.page, limit=query_args.limit)
        return {"data": feedbacks}


@service_api_ns.route("/messages/<uuid:message_id>/suggested")
class MessageSuggestedApi(Resource):
    @service_api_ns.doc("get_suggested_questions")
    @service_api_ns.doc(description="Get suggested follow-up questions for a message")
    @service_api_ns.doc(params={"message_id": "Message ID"})
    @service_api_ns.doc(
        responses={
            200: "Suggested questions retrieved successfully",
            400: "Suggested questions feature is disabled",
            401: "Unauthorized - invalid API token",
            404: "Message not found",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.QUERY, required=True))
    def get(self, app_model: App, end_user: EndUser, message_id):
        """Get suggested follow-up questions for a message.

        Returns AI-generated follow-up questions based on the message content.
        """
        message_id = str(message_id)
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in {AppMode.CHAT, AppMode.AGENT_CHAT, AppMode.ADVANCED_CHAT}:
            raise NotChatAppError()

        try:
            questions = MessageService.get_suggested_questions_after_answer(
                app_model=app_model, user=end_user, message_id=message_id, invoke_from=InvokeFrom.SERVICE_API
            )
        except MessageNotExistsError:
            raise NotFound("Message Not Exists.")
        except SuggestedQuestionsAfterAnswerDisabledError:
            raise BadRequest("Suggested Questions Is Disabled.")
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()

        return {"result": "success", "data": questions}

```

### api/controllers/service_api/app/file.py
```py
from flask import request
from flask_restx import Resource
from flask_restx.api import HTTPStatus

import services
from controllers.common.errors import (
    FilenameNotExistsError,
    FileTooLargeError,
    NoFileUploadedError,
    TooManyFilesError,
    UnsupportedFileTypeError,
)
from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from extensions.ext_database import db
from fields.file_fields import FileResponse
from models import App, EndUser
from services.file_service import FileService

register_schema_models(service_api_ns, FileResponse)


@service_api_ns.route("/files/upload")
class FileApi(Resource):
    @service_api_ns.doc("upload_file")
    @service_api_ns.doc(description="Upload a file for use in conversations")
    @service_api_ns.doc(
        responses={
            201: "File uploaded successfully",
            400: "Bad request - no file or invalid file",
            401: "Unauthorized - invalid API token",
            413: "File too large",
            415: "Unsupported file type",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.FORM))  # type: ignore
    @service_api_ns.response(HTTPStatus.CREATED, "File uploaded", service_api_ns.models[FileResponse.__name__])
    def post(self, app_model: App, end_user: EndUser):
        """Upload a file for use in conversations.

        Accepts a single file upload via multipart/form-data.
        """
        # check file
        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()

        file = request.files["file"]
        if not file.mimetype:
            raise UnsupportedFileTypeError()

        if not file.filename:
            raise FilenameNotExistsError

        try:
            upload_file = FileService(db.engine).upload_file(
                filename=file.filename,
                content=file.read(),
                mimetype=file.mimetype,
                user=end_user,
            )
        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        response = FileResponse.model_validate(upload_file, from_attributes=True)
        return response.model_dump(mode="json"), 201

```

### api/controllers/service_api/app/site.py
```py
from flask_restx import Resource
from sqlalchemy import select
from werkzeug.exceptions import Forbidden

from controllers.common.fields import Site as SiteResponse
from controllers.service_api import service_api_ns
from controllers.service_api.wraps import validate_app_token
from extensions.ext_database import db
from models.account import TenantStatus
from models.model import App, Site


@service_api_ns.route("/site")
class AppSiteApi(Resource):
    """Resource for app sites."""

    @service_api_ns.doc("get_app_site")
    @service_api_ns.doc(description="Get application site configuration")
    @service_api_ns.doc(
        responses={
            200: "Site configuration retrieved successfully",
            401: "Unauthorized - invalid API token",
            403: "Forbidden - site not found or tenant archived",
        }
    )
    @validate_app_token
    def get(self, app_model: App):
        """Retrieve app site info.

        Returns the site configuration for the application including theme, icons, and text.
        """
        site = db.session.scalar(select(Site).where(Site.app_id == app_model.id).limit(1))

        if not site:
            raise Forbidden()

        assert app_model.tenant
        if app_model.tenant.status == TenantStatus.ARCHIVE:
            raise Forbidden()

        return SiteResponse.model_validate(site).model_dump(mode="json")

```

### api/controllers/service_api/app/app.py
```py
from typing import Any, cast

from flask_restx import Resource

from controllers.common.fields import Parameters
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import AppUnavailableError
from controllers.service_api.wraps import validate_app_token
from core.app.app_config.common.parameters_mapping import get_parameters_from_feature_dict
from models.model import App, AppMode
from services.app_service import AppService


@service_api_ns.route("/parameters")
class AppParameterApi(Resource):
    """Resource for app variables."""

    @service_api_ns.doc("get_app_parameters")
    @service_api_ns.doc(description="Retrieve application input parameters and configuration")
    @service_api_ns.doc(
        responses={
            200: "Parameters retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Application not found",
        }
    )
    @validate_app_token
    def get(self, app_model: App):
        """Retrieve app parameters.

        Returns the input form parameters and configuration for the application.
        """
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
        return Parameters.model_validate(parameters).model_dump(mode="json")


@service_api_ns.route("/meta")
class AppMetaApi(Resource):
    @service_api_ns.doc("get_app_meta")
    @service_api_ns.doc(description="Get application metadata")
    @service_api_ns.doc(
        responses={
            200: "Metadata retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Application not found",
        }
    )
    @validate_app_token
    def get(self, app_model: App):
        """Get app metadata.

        Returns metadata about the application including configuration and settings.
        """
        return AppService().get_app_meta(app_model)


@service_api_ns.route("/info")
class AppInfoApi(Resource):
    @service_api_ns.doc("get_app_info")
    @service_api_ns.doc(description="Get basic application information")
    @service_api_ns.doc(
        responses={
            200: "Application info retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Application not found",
        }
    )
    @validate_app_token
    def get(self, app_model: App):
        """Get app information.

        Returns basic information about the application including name, description, tags, and mode.
        """
        tags = [tag.name for tag in app_model.tags]
        return {
            "name": app_model.name,
            "description": app_model.description,
            "tags": tags,
            "mode": app_model.mode,
            "author_name": app_model.author_name,
        }

```

### api/controllers/service_api/app/audio.py
```py
import logging

from flask import request
from flask_restx import Resource
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from werkzeug.exceptions import InternalServerError

import services
from controllers.common.schema import register_schema_model
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
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
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from core.errors.error import ModelCurrentlyNotSupportError, ProviderTokenNotInitError, QuotaExceededError
from models.model import App, EndUser
from services.audio_service import AudioService
from services.errors.audio import (
    AudioTooLargeServiceError,
    NoAudioUploadedServiceError,
    ProviderNotSupportSpeechToTextServiceError,
    UnsupportedAudioTypeServiceError,
)

logger = logging.getLogger(__name__)


@service_api_ns.route("/audio-to-text")
class AudioApi(Resource):
    @service_api_ns.doc("audio_to_text")
    @service_api_ns.doc(description="Convert audio to text using speech-to-text")
    @service_api_ns.doc(
        responses={
            200: "Audio successfully transcribed",
            400: "Bad request - no audio or invalid audio",
            401: "Unauthorized - invalid API token",
            413: "Audio file too large",
            415: "Unsupported audio type",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.FORM))
    def post(self, app_model: App, end_user: EndUser):
        """Convert audio to text using speech-to-text.

        Accepts an audio file upload and returns the transcribed text.
        """
        file = request.files["file"]

        try:
            response = AudioService.transcript_asr(app_model=app_model, file=file, end_user=end_user.id)

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
            logger.exception("internal server error.")
            raise InternalServerError()


class TextToAudioPayload(BaseModel):
    message_id: str | None = Field(default=None, description="Message ID")
    voice: str | None = Field(default=None, description="Voice to use for TTS")
    text: str | None = Field(default=None, description="Text to convert to audio")
    streaming: bool | None = Field(default=None, description="Enable streaming response")


register_schema_model(service_api_ns, TextToAudioPayload)


@service_api_ns.route("/text-to-audio")
class TextApi(Resource):
    @service_api_ns.expect(service_api_ns.models[TextToAudioPayload.__name__])
    @service_api_ns.doc("text_to_audio")
    @service_api_ns.doc(description="Convert text to audio using text-to-speech")
    @service_api_ns.doc(
        responses={
            200: "Text successfully converted to audio",
            400: "Bad request - invalid parameters",
            401: "Unauthorized - invalid API token",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON))
    def post(self, app_model: App, end_user: EndUser):
        """Convert text to audio using text-to-speech.

        Converts the provided text to audio using the specified voice.
        """
        try:
            payload = TextToAudioPayload.model_validate(service_api_ns.payload or {})

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
            logger.exception("internal server error.")
            raise InternalServerError()

```

### api/controllers/service_api/app/workflow.py
```py
import logging
from typing import Any, Literal

from dateutil.parser import isoparse
from flask import request
from flask_restx import Namespace, Resource, fields
from graphon.enums import WorkflowExecutionStatus
from graphon.graph_engine.manager import GraphEngineManager
from graphon.model_runtime.errors.invoke import InvokeError
from pydantic import BaseModel, Field
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import BadRequest, InternalServerError, NotFound

from controllers.common.schema import register_schema_models
from controllers.service_api import service_api_ns
from controllers.service_api.app.error import (
    CompletionRequestError,
    NotWorkflowAppError,
    ProviderModelCurrentlyNotSupportError,
    ProviderNotInitializeError,
    ProviderQuotaExceededError,
)
from controllers.service_api.wraps import FetchUserArg, WhereisUserArg, validate_app_token
from controllers.web.error import InvokeRateLimitError as InvokeRateLimitHttpError
from core.app.apps.base_app_queue_manager import AppQueueManager
from core.app.entities.app_invoke_entities import InvokeFrom
from core.errors.error import (
    ModelCurrentlyNotSupportError,
    ProviderTokenNotInitError,
    QuotaExceededError,
)
from core.helper.trace_id_helper import get_external_trace_id
from extensions.ext_database import db
from extensions.ext_redis import redis_client
from fields.workflow_app_log_fields import build_workflow_app_log_pagination_model
from libs import helper
from libs.helper import OptionalTimestampField, TimestampField
from models.model import App, AppMode, EndUser
from models.workflow import WorkflowRun
from repositories.factory import DifyAPIRepositoryFactory
from services.app_generate_service import AppGenerateService
from services.errors.app import IsDraftWorkflowError, WorkflowIdFormatError, WorkflowNotFoundError
from services.errors.llm import InvokeRateLimitError
from services.workflow_app_service import WorkflowAppService

logger = logging.getLogger(__name__)


class WorkflowRunPayload(BaseModel):
    inputs: dict[str, Any]
    files: list[dict[str, Any]] | None = None
    response_mode: Literal["blocking", "streaming"] | None = None


class WorkflowLogQuery(BaseModel):
    keyword: str | None = None
    status: Literal["succeeded", "failed", "stopped"] | None = None
    created_at__before: str | None = None
    created_at__after: str | None = None
    created_by_end_user_session_id: str | None = None
    created_by_account: str | None = None
    page: int = Field(default=1, ge=1, le=99999)
    limit: int = Field(default=20, ge=1, le=100)


register_schema_models(service_api_ns, WorkflowRunPayload, WorkflowLogQuery)


class WorkflowRunStatusField(fields.Raw):
    def output(self, key, obj: WorkflowRun, **kwargs):
        return obj.status.value


class WorkflowRunOutputsField(fields.Raw):
    def output(self, key, obj: WorkflowRun, **kwargs):
        if obj.status == WorkflowExecutionStatus.PAUSED:
            return {}

        outputs = obj.outputs_dict
        return outputs or {}


workflow_run_fields = {
    "id": fields.String,
    "workflow_id": fields.String,
    "status": WorkflowRunStatusField,
    "inputs": fields.Raw,
    "outputs": WorkflowRunOutputsField,
    "error": fields.String,
    "total_steps": fields.Integer,
    "total_tokens": fields.Integer,
    "created_at": TimestampField,
    "finished_at": OptionalTimestampField,
    "elapsed_time": fields.Float,
}


def build_workflow_run_model(api_or_ns: Namespace):
    """Build the workflow run model for the API or Namespace."""
    return api_or_ns.model("WorkflowRun", workflow_run_fields)


@service_api_ns.route("/workflows/run/<string:workflow_run_id>")
class WorkflowRunDetailApi(Resource):
    @service_api_ns.doc("get_workflow_run_detail")
    @service_api_ns.doc(description="Get workflow run details")
    @service_api_ns.doc(params={"workflow_run_id": "Workflow run ID"})
    @service_api_ns.doc(
        responses={
            200: "Workflow run details retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "Workflow run not found",
        }
    )
    @validate_app_token
    @service_api_ns.marshal_with(build_workflow_run_model(service_api_ns))
    def get(self, app_model: App, workflow_run_id: str):
        """Get a workflow task running detail.

        Returns detailed information about a specific workflow run.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode not in [AppMode.WORKFLOW, AppMode.ADVANCED_CHAT]:
            raise NotWorkflowAppError()

        # Use repository to get workflow run
        session_maker = sessionmaker(bind=db.engine, expire_on_commit=False)
        workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

        workflow_run = workflow_run_repo.get_workflow_run_by_id(
            tenant_id=app_model.tenant_id,
            app_id=app_model.id,
            run_id=workflow_run_id,
        )
        if not workflow_run:
            raise NotFound("Workflow run not found.")
        return workflow_run


@service_api_ns.route("/workflows/run")
class WorkflowRunApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowRunPayload.__name__])
    @service_api_ns.doc("run_workflow")
    @service_api_ns.doc(description="Execute a workflow")
    @service_api_ns.doc(
        responses={
            200: "Workflow executed successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser):
        """Execute a workflow.

        Runs a workflow with the provided inputs and returns the results.
        Supports both blocking and streaming response modes.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        payload = WorkflowRunPayload.model_validate(service_api_ns.payload or {})
        args = payload.model_dump(exclude_none=True)
        external_trace_id = get_external_trace_id(request)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id
        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
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
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/workflows/<string:workflow_id>/run")
class WorkflowRunByIdApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowRunPayload.__name__])
    @service_api_ns.doc("run_workflow_by_id")
    @service_api_ns.doc(description="Execute a specific workflow by ID")
    @service_api_ns.doc(params={"workflow_id": "Workflow ID to execute"})
    @service_api_ns.doc(
        responses={
            200: "Workflow executed successfully",
            400: "Bad request - invalid parameters or workflow issues",
            401: "Unauthorized - invalid API token",
            404: "Workflow not found",
            429: "Rate limit exceeded",
            500: "Internal server error",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, workflow_id: str):
        """Run specific workflow by ID.

        Executes a specific workflow version identified by its ID.
        """
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        payload = WorkflowRunPayload.model_validate(service_api_ns.payload or {})
        args = payload.model_dump(exclude_none=True)

        # Add workflow_id to args for AppGenerateService
        args["workflow_id"] = workflow_id

        external_trace_id = get_external_trace_id(request)
        if external_trace_id:
            args["external_trace_id"] = external_trace_id
        streaming = payload.response_mode == "streaming"

        try:
            response = AppGenerateService.generate(
                app_model=app_model, user=end_user, args=args, invoke_from=InvokeFrom.SERVICE_API, streaming=streaming
            )

            return helper.compact_generate_response(response)
        except WorkflowNotFoundError as ex:
            raise NotFound(str(ex))
        except IsDraftWorkflowError as ex:
            raise BadRequest(str(ex))
        except WorkflowIdFormatError as ex:
            raise BadRequest(str(ex))
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
        except Exception:
            logger.exception("internal server error.")
            raise InternalServerError()


@service_api_ns.route("/workflows/tasks/<string:task_id>/stop")
class WorkflowTaskStopApi(Resource):
    @service_api_ns.doc("stop_workflow_task")
    @service_api_ns.doc(description="Stop a running workflow task")
    @service_api_ns.doc(params={"task_id": "Task ID to stop"})
    @service_api_ns.doc(
        responses={
            200: "Task stopped successfully",
            401: "Unauthorized - invalid API token",
            404: "Task not found",
        }
    )
    @validate_app_token(fetch_user_arg=FetchUserArg(fetch_from=WhereisUserArg.JSON, required=True))
    def post(self, app_model: App, end_user: EndUser, task_id: str):
        """Stop a running workflow task."""
        app_mode = AppMode.value_of(app_model.mode)
        if app_mode != AppMode.WORKFLOW:
            raise NotWorkflowAppError()

        # Stop using both mechanisms for backward compatibility
        # Legacy stop flag mechanism (without user check)
        AppQueueManager.set_stop_flag_no_user_check(task_id)

        # New graph engine command channel mechanism
        GraphEngineManager(redis_client).send_stop_command(task_id)

        return {"result": "success"}


@service_api_ns.route("/workflows/logs")
class WorkflowAppLogApi(Resource):
    @service_api_ns.expect(service_api_ns.models[WorkflowLogQuery.__name__])
    @service_api_ns.doc("get_workflow_logs")
    @service_api_ns.doc(description="Get workflow execution logs")
    @service_api_ns.doc(
        responses={
            200: "Logs retrieved successfully",
            401: "Unauthorized - invalid API token",
        }
    )
    @validate_app_token
    @service_api_ns.marshal_with(build_workflow_app_log_pagination_model(service_api_ns))
    def get(self, app_model: App):
        """Get workflow app logs.

        Returns paginated workflow execution logs with filtering options.
        """
        args = WorkflowLogQuery.model_validate(request.args.to_dict())

        status = WorkflowExecutionStatus(args.status) if args.status else None
        created_at_before = isoparse(args.created_at__before) if args.created_at__before else None
        created_at_after = isoparse(args.created_at__after) if args.created_at__after else None

        # get paginate workflow app logs
        workflow_app_service = WorkflowAppService()
        with sessionmaker(db.engine).begin() as session:
            workflow_app_log_pagination = workflow_app_service.get_paginate_workflow_app_logs(
                session=session,
                app_model=app_model,
                keyword=args.keyword,
                status=status,
                created_at_before=created_at_before,
                created_at_after=created_at_after,
                page=args.page,
                limit=args.limit,
                created_by_end_user_session_id=args.created_by_end_user_session_id,
                created_by_account=args.created_by_account,
            )

            return workflow_app_log_pagination

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-002.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
