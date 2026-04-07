You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-004
- **Kind**: endpoint
- **Identifier**: File Upload/Download Endpoints (POST /files/upload, GET /files/{file_id})
- **Description**: File upload accepting user files with MIME type detection. Risk of path traversal in filename, unrestricted file type upload, SSRF via file URL fetch, and storage backend misconfiguration (S3, Azure, local).
- **Locations**: api/controllers/files/, api/factories/file_factory/, api/extensions/storage/

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

### api/controllers/files/tool_files.py
```py
from urllib.parse import quote

from flask import Response, request
from flask_restx import Resource
from pydantic import BaseModel, Field
from werkzeug.exceptions import Forbidden, NotFound

from controllers.common.errors import UnsupportedFileTypeError
from controllers.common.file_response import enforce_download_for_html
from controllers.files import files_ns
from core.tools.signature import verify_tool_file_signature
from core.tools.tool_file_manager import ToolFileManager

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ToolFileQuery(BaseModel):
    timestamp: str = Field(..., description="Unix timestamp")
    nonce: str = Field(..., description="Random nonce")
    sign: str = Field(..., description="HMAC signature")
    as_attachment: bool = Field(default=False, description="Download as attachment")


files_ns.schema_model(
    ToolFileQuery.__name__, ToolFileQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@files_ns.route("/tools/<uuid:file_id>.<string:extension>")
class ToolFileApi(Resource):
    @files_ns.doc("get_tool_file")
    @files_ns.doc(description="Download a tool file by ID using signed parameters")
    @files_ns.doc(
        params={
            "file_id": "Tool file identifier",
            "extension": "Expected file extension",
            "timestamp": "Unix timestamp used in the signature",
            "nonce": "Random string used in the signature",
            "sign": "HMAC signature verifying the request",
            "as_attachment": "Whether to download the file as an attachment",
        }
    )
    @files_ns.doc(
        responses={
            200: "Tool file stream returned successfully",
            403: "Forbidden - invalid signature",
            404: "File not found",
            415: "Unsupported file type",
        }
    )
    def get(self, file_id, extension):
        file_id = str(file_id)

        args = ToolFileQuery.model_validate(request.args.to_dict())
        if not verify_tool_file_signature(file_id=file_id, timestamp=args.timestamp, nonce=args.nonce, sign=args.sign):
            raise Forbidden("Invalid request.")

        try:
            tool_file_manager = ToolFileManager()
            stream, tool_file = tool_file_manager.get_file_generator_by_tool_file_id(
                file_id,
            )

            if not stream or not tool_file:
                raise NotFound("file is not found")

        except NotFound:
            raise

        except Exception:
            raise UnsupportedFileTypeError()

        mime_type = tool_file.mime_type
        filename = tool_file.filename

        response = Response(
            stream,
            mimetype=mime_type,
            direct_passthrough=True,
            headers={},
        )
        if tool_file.size > 0:
            response.headers["Content-Length"] = str(tool_file.size)
        if args.as_attachment and filename:
            encoded_filename = quote(filename)
            response.headers["Content-Disposition"] = f"attachment; filename*=UTF-8''{encoded_filename}"

        enforce_download_for_html(
            response,
            mime_type=mime_type,
            filename=filename,
            extension=extension,
        )

        return response

```

### api/controllers/files/upload.py
```py
from mimetypes import guess_extension

from flask import request
from flask_restx import Resource
from flask_restx.api import HTTPStatus
from pydantic import BaseModel, Field
from werkzeug.exceptions import Forbidden

import services
from core.tools.signature import verify_plugin_file_signature
from core.tools.tool_file_manager import ToolFileManager
from fields.file_fields import FileResponse

from ..common.errors import (
    FileTooLargeError,
    UnsupportedFileTypeError,
)
from ..common.schema import register_schema_models
from ..console.wraps import setup_required
from ..files import files_ns
from ..inner_api.plugin.wraps import get_user

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class PluginUploadQuery(BaseModel):
    timestamp: str = Field(..., description="Unix timestamp for signature verification")
    nonce: str = Field(..., description="Random nonce for signature verification")
    sign: str = Field(..., description="HMAC signature")
    tenant_id: str = Field(..., description="Tenant identifier")
    user_id: str | None = Field(default=None, description="User identifier")


files_ns.schema_model(
    PluginUploadQuery.__name__, PluginUploadQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)

register_schema_models(files_ns, FileResponse)


@files_ns.route("/upload/for-plugin")
class PluginUploadFileApi(Resource):
    @setup_required
    @files_ns.expect(files_ns.models[PluginUploadQuery.__name__])
    @files_ns.doc("upload_plugin_file")
    @files_ns.doc(description="Upload a file for plugin usage with signature verification")
    @files_ns.doc(
        responses={
            201: "File uploaded successfully",
            400: "Invalid request parameters",
            403: "Forbidden - Invalid signature or missing parameters",
            413: "File too large",
            415: "Unsupported file type",
        }
    )
    @files_ns.response(HTTPStatus.CREATED, "File uploaded", files_ns.models[FileResponse.__name__])
    def post(self):
        """Upload a file for plugin usage.

        Accepts a file upload with signature verification for security.
        The file must be accompanied by valid timestamp, nonce, and signature parameters.

        Returns:
            dict: File metadata including ID, URLs, and properties
            int: HTTP status code (201 for success)

        Raises:
            Forbidden: Invalid signature or missing required parameters
            FileTooLargeError: File exceeds size limit
            UnsupportedFileTypeError: File type not supported
        """
        args = PluginUploadQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        file = request.files.get("file")
        if file is None:
            raise Forbidden("File is required.")

        timestamp = args.timestamp
        nonce = args.nonce
        sign = args.sign
        tenant_id = args.tenant_id
        user_id = args.user_id
        user = get_user(tenant_id, user_id)

        filename = file.filename
        mimetype = file.mimetype

        if not filename or not mimetype:
            raise Forbidden("Invalid request.")

        if not verify_plugin_file_signature(
            filename=filename,
            mimetype=mimetype,
            tenant_id=tenant_id,
            user_id=user.id,
            timestamp=timestamp,
            nonce=nonce,
            sign=sign,
        ):
            raise Forbidden("Invalid request.")

        try:
            tool_file = ToolFileManager().create_file_by_raw(
                user_id=user.id,
                tenant_id=tenant_id,
                file_binary=file.read(),
                mimetype=mimetype,
                filename=filename,
                conversation_id=None,
            )

            extension = guess_extension(tool_file.mimetype) or ".bin"
            preview_url = ToolFileManager.sign_file(tool_file_id=tool_file.id, extension=extension)

            # Create a dictionary with all the necessary attributes
            result = FileResponse(
                id=tool_file.id,
                name=tool_file.name,
                size=tool_file.size,
                extension=extension,
                mime_type=mimetype,
                preview_url=preview_url,
                source_url=tool_file.original_url,
                original_url=tool_file.original_url,
                user_id=tool_file.user_id,
                tenant_id=tool_file.tenant_id,
                conversation_id=tool_file.conversation_id,
                file_key=tool_file.file_key,
            )

            return result.model_dump(mode="json"), 201
        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

```

### api/controllers/files/__init__.py
```py
from flask import Blueprint
from flask_restx import Namespace

from libs.external_api import ExternalApi

bp = Blueprint("files", __name__, url_prefix="/files")

api = ExternalApi(
    bp,
    version="1.0",
    title="Files API",
    description="API for file operations including upload and preview",
)

files_ns = Namespace("files", description="File operations", path="/")

from . import image_preview, tool_files, upload

api.add_namespace(files_ns)

__all__ = [
    "api",
    "bp",
    "files_ns",
    "image_preview",
    "tool_files",
    "upload",
]

```

### api/controllers/files/image_preview.py
```py
from urllib.parse import quote

from flask import Response, request
from flask_restx import Resource
from pydantic import BaseModel, Field
from werkzeug.exceptions import NotFound

import services
from controllers.common.errors import UnsupportedFileTypeError
from controllers.common.file_response import enforce_download_for_html
from controllers.files import files_ns
from extensions.ext_database import db
from services.account_service import TenantService
from services.file_service import FileService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class FileSignatureQuery(BaseModel):
    timestamp: str = Field(..., description="Unix timestamp used in the signature")
    nonce: str = Field(..., description="Random string for signature")
    sign: str = Field(..., description="HMAC signature")


class FilePreviewQuery(FileSignatureQuery):
    as_attachment: bool = Field(default=False, description="Whether to download as attachment")


files_ns.schema_model(
    FileSignatureQuery.__name__, FileSignatureQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)
files_ns.schema_model(
    FilePreviewQuery.__name__, FilePreviewQuery.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0)
)


@files_ns.route("/<uuid:file_id>/image-preview")
class ImagePreviewApi(Resource):
    """Deprecated endpoint for retrieving image previews."""

    @files_ns.doc("get_image_preview")
    @files_ns.doc(description="Retrieve a signed image preview for a file")
    @files_ns.doc(
        params={
            "file_id": "ID of the file to preview",
            "timestamp": "Unix timestamp used in the signature",
            "nonce": "Random string used in the signature",
            "sign": "HMAC signature verifying the request",
        }
    )
    @files_ns.doc(
        responses={
            200: "Image preview returned successfully",
            400: "Missing or invalid signature parameters",
            415: "Unsupported file type",
        }
    )
    def get(self, file_id):
        file_id = str(file_id)

        args = FileSignatureQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore
        timestamp = args.timestamp
        nonce = args.nonce
        sign = args.sign

        try:
            generator, mimetype = FileService(db.engine).get_image_preview(
                file_id=file_id,
                timestamp=timestamp,
                nonce=nonce,
                sign=sign,
            )
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        return Response(generator, mimetype=mimetype)


@files_ns.route("/<uuid:file_id>/file-preview")
class FilePreviewApi(Resource):
    @files_ns.doc("get_file_preview")
    @files_ns.doc(description="Download a file preview or attachment using signed parameters")
    @files_ns.doc(
        params={
            "file_id": "ID of the file to preview",
            "timestamp": "Unix timestamp used in the signature",
            "nonce": "Random string used in the signature",
            "sign": "HMAC signature verifying the request",
            "as_attachment": "Whether to download the file as an attachment",
        }
    )
    @files_ns.doc(
        responses={
            200: "File stream returned successfully",
            400: "Missing or invalid signature parameters",
            404: "File not found",
            415: "Unsupported file type",
        }
    )
    def get(self, file_id):
        file_id = str(file_id)

        args = FilePreviewQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        try:
            generator, upload_file = FileService(db.engine).get_file_generator_by_file_id(
                file_id=file_id,
                timestamp=args.timestamp,
                nonce=args.nonce,
                sign=args.sign,
            )
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        response = Response(
            generator,
            mimetype=upload_file.mime_type,
            direct_passthrough=True,
            headers={},
        )
        # add Accept-Ranges header for audio/video files
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
        if upload_file.size > 0:
            response.headers["Content-Length"] = str(upload_file.size)
        if args.as_attachment:
            encoded_filename = quote(upload_file.name)
            response.headers["Content-Disposition"] = f"attachment; filename*=UTF-8''{encoded_filename}"
        response.headers["Content-Type"] = "application/octet-stream"

        enforce_download_for_html(
            response,
            mime_type=upload_file.mime_type,
            filename=upload_file.name,
            extension=upload_file.extension,
        )

        return response


@files_ns.route("/workspaces/<uuid:workspace_id>/webapp-logo")
class WorkspaceWebappLogoApi(Resource):
    @files_ns.doc("get_workspace_webapp_logo")
    @files_ns.doc(description="Fetch the custom webapp logo for a workspace")
    @files_ns.doc(
        params={
            "workspace_id": "Workspace identifier",
        }
    )
    @files_ns.doc(
        responses={
            200: "Logo returned successfully",
            404: "Webapp logo not configured",
            415: "Unsupported file type",
        }
    )
    def get(self, workspace_id):
        workspace_id = str(workspace_id)

        custom_config = TenantService.get_custom_config(workspace_id)
        webapp_logo_file_id = custom_config.get("replace_webapp_logo") if custom_config is not None else None

        if not webapp_logo_file_id:
            raise NotFound("webapp logo is not found")

        try:
            generator, mimetype = FileService(db.engine).get_public_image_preview(
                webapp_logo_file_id,
            )
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()

        return Response(generator, mimetype=mimetype)

```

### api/factories/file_factory/builders.py
```py
"""Core builders for workflow file mappings."""

from __future__ import annotations

import mimetypes
import uuid
from collections.abc import Mapping, Sequence
from typing import Any

from graphon.file import File, FileTransferMethod, FileType, FileUploadConfig, helpers, standardize_file_type
from sqlalchemy import select

from core.app.file_access import FileAccessControllerProtocol
from core.workflow.file_reference import build_file_reference
from extensions.ext_database import db
from models import ToolFile, UploadFile

from .common import resolve_mapping_file_id
from .remote import get_remote_file_info
from .validation import is_file_valid_with_config


def build_from_mapping(
    *,
    mapping: Mapping[str, Any],
    tenant_id: str,
    config: FileUploadConfig | None = None,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> File:
    transfer_method_value = mapping.get("transfer_method")
    if not transfer_method_value:
        raise ValueError("transfer_method is required in file mapping")

    transfer_method = FileTransferMethod.value_of(transfer_method_value)
    build_func = _get_build_function(transfer_method)
    file = build_func(
        mapping=mapping,
        tenant_id=tenant_id,
        transfer_method=transfer_method,
        strict_type_validation=strict_type_validation,
        access_controller=access_controller,
    )

    if config and not is_file_valid_with_config(
        input_file_type=mapping.get("type", FileType.CUSTOM),
        file_extension=file.extension or "",
        file_transfer_method=file.transfer_method,
        config=config,
    ):
        raise ValueError(f"File validation failed for file: {file.filename}")

    return file


def build_from_mappings(
    *,
    mappings: Sequence[Mapping[str, Any]],
    config: FileUploadConfig | None = None,
    tenant_id: str,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> Sequence[File]:
    # TODO(QuantumGhost): Performance concern - each mapping triggers a separate database query.
    # Implement batch processing to reduce database load when handling multiple files.
    valid_mappings = [mapping for mapping in mappings if _is_valid_mapping(mapping)]
    files = [
        build_from_mapping(
            mapping=mapping,
            tenant_id=tenant_id,
            config=config,
            strict_type_validation=strict_type_validation,
            access_controller=access_controller,
        )
        for mapping in valid_mappings
    ]

    if (
        config
        and config.image_config
        and sum(1 for file in files if file.type == FileType.IMAGE) > config.image_config.number_limits
    ):
        raise ValueError(f"Number of image files exceeds the maximum limit {config.image_config.number_limits}")
    if config and config.number_limits and len(files) > config.number_limits:
        raise ValueError(f"Number of files exceeds the maximum limit {config.number_limits}")

    return files


def _get_build_function(transfer_method: FileTransferMethod):
    build_functions = {
        FileTransferMethod.LOCAL_FILE: _build_from_local_file,
        FileTransferMethod.REMOTE_URL: _build_from_remote_url,
        FileTransferMethod.TOOL_FILE: _build_from_tool_file,
        FileTransferMethod.DATASOURCE_FILE: _build_from_datasource_file,
    }
    build_func = build_functions.get(transfer_method)
    if build_func is None:
        raise ValueError(f"Invalid file transfer method: {transfer_method}")
    return build_func


def _resolve_file_type(
    *,
    detected_file_type: FileType,
    specified_type: str | None,
    strict_type_validation: bool,
) -> FileType:
    if strict_type_validation and specified_type and detected_file_type.value != specified_type:
        raise ValueError("Detected file type does not match the specified type. Please verify the file.")

    if specified_type and specified_type != "custom":
        return FileType(specified_type)
    return detected_file_type


def _build_from_local_file(
    *,
    mapping: Mapping[str, Any],
    tenant_id: str,
    transfer_method: FileTransferMethod,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> File:
    upload_file_id = resolve_mapping_file_id(mapping, "upload_file_id")
    if not upload_file_id:
        raise ValueError("Invalid upload file id")

    try:
        uuid.UUID(upload_file_id)
    except ValueError as exc:
        raise ValueError("Invalid upload file id format") from exc

    stmt = select(UploadFile).where(
        UploadFile.id == upload_file_id,
        UploadFile.tenant_id == tenant_id,
    )
    row = db.session.scalar(access_controller.apply_upload_file_filters(stmt))
    if row is None:
        raise ValueError("Invalid upload file")

    detected_file_type = standardize_file_type(extension="." + row.extension, mime_type=row.mime_type)
    file_type = _resolve_file_type(
        detected_file_type=detected_file_type,
        specified_type=mapping.get("type", "custom"),
        strict_type_validation=strict_type_validation,
    )

    return File(
        id=mapping.get("id"),
        filename=row.name,
        extension="." + row.extension,
        mime_type=row.mime_type,
        type=file_type,
        transfer_method=transfer_method,
        remote_url=row.source_url,
        reference=build_file_reference(record_id=str(row.id)),
        size=row.size,
        storage_key=row.key,
    )


def _build_from_remote_url(
    *,
    mapping: Mapping[str, Any],
    tenant_id: str,
    transfer_method: FileTransferMethod,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> File:
    upload_file_id = resolve_mapping_file_id(mapping, "upload_file_id")
    if upload_file_id:
        try:
            uuid.UUID(upload_file_id)
        except ValueError as exc:
            raise ValueError("Invalid upload file id format") from exc

        stmt = select(UploadFile).where(
            UploadFile.id == upload_file_id,
            UploadFile.tenant_id == tenant_id,
        )
        upload_file = db.session.scalar(access_controller.apply_upload_file_filters(stmt))
        if upload_file is None:
            raise ValueError("Invalid upload file")

        detected_file_type = standardize_file_type(
            extension="." + upload_file.extension,
            mime_type=upload_file.mime_type,
        )
        file_type = _resolve_file_type(
            detected_file_type=detected_file_type,
            specified_type=mapping.get("type"),
            strict_type_validation=strict_type_validation,
        )

        return File(
            id=mapping.get("id"),
            filename=upload_file.name,
            extension="." + upload_file.extension,
            mime_type=upload_file.mime_type,
            type=file_type,
            transfer_method=transfer_method,
            remote_url=helpers.get_signed_file_url(upload_file_id=str(upload_file_id)),
            reference=build_file_reference(record_id=str(upload_file.id)),
            size=upload_file.size,
            storage_key=upload_file.key,
        )

    url = mapping.get("url") or mapping.get("remote_url")
    if not url:
        raise ValueError("Invalid file url")

    mime_type, filename, file_size = get_remote_file_info(url)
    extension = mimetypes.guess_extension(mime_type) or ("." + filename.split(".")[-1] if "." in filename else ".bin")
    detected_file_type = standardize_file_type(extension=extension, mime_type=mime_type)
    file_type = _resolve_file_type(
        detected_file_type=detected_file_type,
        specified_type=mapping.get("type"),
        strict_type_validation=strict_type_validation,
    )

    return File(
        id=mapping.get("id"),
        filename=filename,
        type=file_type,
        transfer_method=transfer_method,
        remote_url=url,
        mime_type=mime_type,
        extension=extension,
        size=file_size,
    )


def _build_from_tool_file(
    *,
    mapping: Mapping[str, Any],
    tenant_id: str,
    transfer_method: FileTransferMethod,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> File:
    tool_file_id = resolve_mapping_file_id(mapping, "tool_file_id")
    if not tool_file_id:
        raise ValueError(f"ToolFile {tool_file_id} not found")

    stmt = select(ToolFile).where(
        ToolFile.id == tool_file_id,
        ToolFile.tenant_id == tenant_id,
    )
    tool_file = db.session.scalar(access_controller.apply_tool_file_filters(stmt))
    if tool_file is None:
        raise ValueError(f"ToolFile {tool_file_id} not found")

    extension = "." + tool_file.file_key.split(".")[-1] if "." in tool_file.file_key else ".bin"
    detected_file_type = standardize_file_type(extension=extension, mime_type=tool_file.mimetype)
    file_type = _resolve_file_type(
        detected_file_type=detected_file_type,
        specified_type=mapping.get("type"),
        strict_type_validation=strict_type_validation,
    )

    return File(
        id=mapping.get("id"),
        filename=tool_file.name,
        type=file_type,
        transfer_method=transfer_method,
        remote_url=tool_file.original_url,
        reference=build_file_reference(record_id=str(tool_file.id)),
        extension=extension,
        mime_type=tool_file.mimetype,
        size=tool_file.size,
        storage_key=tool_file.file_key,
    )


def _build_from_datasource_file(
    *,
    mapping: Mapping[str, Any],
    tenant_id: str,
    transfer_method: FileTransferMethod,
    strict_type_validation: bool = False,
    access_controller: FileAccessControllerProtocol,
) -> File:
    datasource_file_id = resolve_mapping_file_id(mapping, "datasource_file_id")
    if not datasource_file_id:
        raise ValueError(f"DatasourceFile {datasource_file_id} not found")

    stmt = select(UploadFile).where(
        UploadFile.id == datasource_file_id,
        UploadFile.tenant_id == tenant_id,
    )
    datasource_file = db.session.scalar(access_controller.apply_upload_file_filters(stmt))
    if datasource_file is None:
        raise ValueError(f"DatasourceFile {mapping.get('datasource_file_id')} not found")

    extension = "." + datasource_file.key.split(".")[-1] if "." in datasource_file.key else ".bin"
    detected_file_type = standardize_file_type(extension="." + extension, mime_type=datasource_file.mime_type)
    file_type = _resolve_file_type(
        detected_file_type=detected_file_type,
        specified_type=mapping.get("type"),
        strict_type_validation=strict_type_validation,
    )

    return File(
        id=mapping.get("datasource_file_id"),
        filename=datasource_file.name,
        type=file_type,
        transfer_method=FileTransferMethod.TOOL_FILE,
        remote_url=datasource_file.source_url,
        reference=build_file_reference(record_id=str(datasource_file.id)),
        extension=extension,
        mime_type=datasource_file.mime_type,
        size=datasource_file.size,
        storage_key=datasource_file.key,
        url=datasource_file.source_url,
    )


def _is_valid_mapping(mapping: Mapping[str, Any]) -> bool:
    if not mapping or not mapping.get("transfer_method"):
        return False

    if mapping.get("transfer_method") == FileTransferMethod.REMOTE_URL:
        url = mapping.get("url") or mapping.get("remote_url")
        if not url:
            return False

    return True

```

### api/factories/file_factory/remote.py
```py
"""Remote file metadata helpers used by workflow file normalization.

These helpers are part of the ``factories.file_factory`` package surface
because both workflow builders and tests rely on the same RFC5987 filename
parsing and HEAD-response normalization rules.
"""

from __future__ import annotations

import mimetypes
import os
import re
import urllib.parse
import uuid

import httpx
from werkzeug.http import parse_options_header

from core.helper import ssrf_proxy


def extract_filename(url_path: str, content_disposition: str | None) -> str | None:
    """Extract a safe filename from Content-Disposition or the request URL path."""
    filename: str | None = None
    if content_disposition:
        filename_star_match = re.search(r"filename\*=([^;]+)", content_disposition)
        if filename_star_match:
            raw_star = filename_star_match.group(1).strip()
            raw_star = raw_star.removesuffix('"')
            try:
                parts = raw_star.split("'", 2)
                charset = (parts[0] or "utf-8").lower() if len(parts) >= 1 else "utf-8"
                value = parts[2] if len(parts) == 3 else parts[-1]
                filename = urllib.parse.unquote(value, encoding=charset, errors="replace")
            except Exception:
                if "''" in raw_star:
                    filename = urllib.parse.unquote(raw_star.split("''")[-1])
                else:
                    filename = urllib.parse.unquote(raw_star)

        if not filename:
            _, params = parse_options_header(content_disposition)
            raw = params.get("filename")
            if raw:
                if len(raw) >= 2 and raw[0] == raw[-1] == '"':
                    raw = raw[1:-1]
                filename = urllib.parse.unquote(raw)

    if not filename:
        candidate = os.path.basename(url_path)
        filename = urllib.parse.unquote(candidate) if candidate else None

    if filename:
        filename = os.path.basename(filename)
        if not filename or not filename.strip():
            filename = None

    return filename or None


def _guess_mime_type(filename: str) -> str:
    guessed_mime, _ = mimetypes.guess_type(filename)
    return guessed_mime or ""


def get_remote_file_info(url: str) -> tuple[str, str, int]:
    """Resolve remote file metadata with SSRF-safe HEAD probing."""
    file_size = -1
    parsed_url = urllib.parse.urlparse(url)
    url_path = parsed_url.path
    filename = os.path.basename(url_path)
    mime_type = _guess_mime_type(filename)

    resp = ssrf_proxy.head(url, follow_redirects=True)
    if resp.status_code == httpx.codes.OK:
        content_disposition = resp.headers.get("Content-Disposition")
        extracted_filename = extract_filename(url_path, content_disposition)
        if extracted_filename:
            filename = extracted_filename
            mime_type = _guess_mime_type(filename)
        file_size = int(resp.headers.get("Content-Length", file_size))
        if not mime_type:
            mime_type = resp.headers.get("Content-Type", "").split(";")[0].strip()

    if not filename:
        extension = mimetypes.guess_extension(mime_type) or ".bin"
        filename = f"{uuid.uuid4().hex}{extension}"
        if not mime_type:
            mime_type = _guess_mime_type(filename)

    return mime_type, filename, file_size

```

### api/factories/file_factory/__init__.py
```py
"""Workflow file factory package.

This package normalizes workflow-layer file payloads into graph-layer ``File``
values. It keeps tenancy and ownership checks in the application layer and
exports the workflow-facing file builders for callers.
"""

from .builders import build_from_mapping, build_from_mappings
from .message_files import build_from_message_file, build_from_message_files
from .storage_keys import StorageKeyLoader

__all__ = [
    "StorageKeyLoader",
    "build_from_mapping",
    "build_from_mappings",
    "build_from_message_file",
    "build_from_message_files",
]

```

### api/factories/file_factory/message_files.py
```py
"""Adapters from persisted message files to graph-layer file values."""

from __future__ import annotations

from collections.abc import Sequence

from graphon.file import File, FileBelongsTo, FileTransferMethod, FileUploadConfig

from core.app.file_access import FileAccessControllerProtocol
from models import MessageFile

from .builders import build_from_mapping


def build_from_message_files(
    *,
    message_files: Sequence[MessageFile],
    tenant_id: str,
    config: FileUploadConfig | None = None,
    access_controller: FileAccessControllerProtocol,
) -> Sequence[File]:
    return [
        build_from_message_file(
            message_file=message_file,
            tenant_id=tenant_id,
            config=config,
            access_controller=access_controller,
        )
        for message_file in message_files
        if message_file.belongs_to != FileBelongsTo.ASSISTANT
    ]


def build_from_message_file(
    *,
    message_file: MessageFile,
    tenant_id: str,
    config: FileUploadConfig | None,
    access_controller: FileAccessControllerProtocol,
) -> File:
    mapping = {
        "transfer_method": message_file.transfer_method,
        "url": message_file.url,
        "type": message_file.type,
    }

    if message_file.id:
        mapping["id"] = message_file.id

    if message_file.transfer_method == FileTransferMethod.TOOL_FILE:
        mapping["tool_file_id"] = message_file.upload_file_id
    else:
        mapping["upload_file_id"] = message_file.upload_file_id

    return build_from_mapping(
        mapping=mapping,
        tenant_id=tenant_id,
        config=config,
        access_controller=access_controller,
    )

```

### api/factories/file_factory/common.py
```py
"""Shared helpers for workflow file factory modules."""

from __future__ import annotations

from collections.abc import Mapping
from typing import Any

from core.workflow.file_reference import resolve_file_record_id


def resolve_mapping_file_id(mapping: Mapping[str, Any], *keys: str) -> str | None:
    """Resolve historical file identifiers from persisted mapping payloads.

    Workflow and model payloads can outlive file schema changes. Older rows may
    still carry concrete identifiers in legacy fields such as ``related_id``,
    while newer payloads use opaque references. Keep this compatibility lookup in
    the factory layer so historical data remains readable without reintroducing
    storage details into graph-layer ``File`` values.
    """

    for key in (*keys, "reference", "related_id"):
        raw_value = mapping.get(key)
        if isinstance(raw_value, str) and raw_value:
            resolved_value = resolve_file_record_id(raw_value)
            if resolved_value:
                return resolved_value
    return None

```

### api/factories/file_factory/storage_keys.py
```py
"""Batched storage-key hydration for workflow files."""

from __future__ import annotations

import uuid
from collections.abc import Mapping, Sequence

from graphon.file import File, FileTransferMethod
from sqlalchemy import select
from sqlalchemy.orm import Session

from core.app.file_access import FileAccessControllerProtocol
from core.workflow.file_reference import build_file_reference, parse_file_reference
from models import ToolFile, UploadFile


class StorageKeyLoader:
    """Load storage keys for files with a constant number of database queries."""

    _session: Session
    _tenant_id: str
    _access_controller: FileAccessControllerProtocol

    def __init__(
        self,
        session: Session,
        tenant_id: str,
        access_controller: FileAccessControllerProtocol,
    ) -> None:
        self._session = session
        self._tenant_id = tenant_id
        self._access_controller = access_controller

    def _load_upload_files(self, upload_file_ids: Sequence[uuid.UUID]) -> Mapping[uuid.UUID, UploadFile]:
        stmt = select(UploadFile).where(
            UploadFile.id.in_(upload_file_ids),
            UploadFile.tenant_id == self._tenant_id,
        )
        scoped_stmt = self._access_controller.apply_upload_file_filters(stmt)
        return {uuid.UUID(upload_file.id): upload_file for upload_file in self._session.scalars(scoped_stmt)}

    def _load_tool_files(self, tool_file_ids: Sequence[uuid.UUID]) -> Mapping[uuid.UUID, ToolFile]:
        stmt = select(ToolFile).where(
            ToolFile.id.in_(tool_file_ids),
            ToolFile.tenant_id == self._tenant_id,
        )
        scoped_stmt = self._access_controller.apply_tool_file_filters(stmt)
        return {uuid.UUID(tool_file.id): tool_file for tool_file in self._session.scalars(scoped_stmt)}

    def load_storage_keys(self, files: Sequence[File]) -> None:
        """Hydrate storage keys by loading their backing file rows in batches.

        The sequence shape is preserved. Each file is updated in place with a
        canonical record reference and storage key loaded from an authorized
        database row. Tenant scoping is enforced by this loader's context
        rather than by embedding tenant identity or storage paths inside
        graph-layer ``File`` values.

        For best performance, prefer batches smaller than 1000 files.
        """

        upload_file_ids: list[uuid.UUID] = []
        tool_file_ids: list[uuid.UUID] = []
        for file in files:
            parsed_reference = parse_file_reference(file.reference)
            if parsed_reference is None:
                raise ValueError("file id should not be None.")

            model_id = uuid.UUID(parsed_reference.record_id)
            if file.transfer_method in (
                FileTransferMethod.LOCAL_FILE,
                FileTransferMethod.REMOTE_URL,
                FileTransferMethod.DATASOURCE_FILE,
            ):
                upload_file_ids.append(model_id)
            elif file.transfer_method == FileTransferMethod.TOOL_FILE:
                tool_file_ids.append(model_id)

        tool_files = self._load_tool_files(tool_file_ids)
        upload_files = self._load_upload_files(upload_file_ids)
        for file in files:
            parsed_reference = parse_file_reference(file.reference)
            if parsed_reference is None:
                raise ValueError("file id should not be None.")

            model_id = uuid.UUID(parsed_reference.record_id)
            if file.transfer_method in (
                FileTransferMethod.LOCAL_FILE,
                FileTransferMethod.REMOTE_URL,
                FileTransferMethod.DATASOURCE_FILE,
            ):
                upload_file_row = upload_files.get(model_id)
                if upload_file_row is None:
                    raise ValueError(f"Upload file not found for id: {model_id}")
                file.reference = build_file_reference(
                    record_id=str(upload_file_row.id),
                )
                file.storage_key = upload_file_row.key
            elif file.transfer_method == FileTransferMethod.TOOL_FILE:
                tool_file_row = tool_files.get(model_id)
                if tool_file_row is None:
                    raise ValueError(f"Tool file not found for id: {model_id}")
                file.reference = build_file_reference(
                    record_id=str(tool_file_row.id),
                )
                file.storage_key = tool_file_row.file_key

```

### api/factories/file_factory/validation.py
```py
"""Validation helpers for workflow file inputs."""

from __future__ import annotations

from graphon.file import FileTransferMethod, FileType, FileUploadConfig


def is_file_valid_with_config(
    *,
    input_file_type: str,
    file_extension: str,
    file_transfer_method: FileTransferMethod,
    config: FileUploadConfig,
) -> bool:
    # FIXME(QIN2DIM): Always allow tool files (files generated by the assistant/model)
    # These are internally generated and should bypass user upload restrictions
    if file_transfer_method == FileTransferMethod.TOOL_FILE:
        return True

    if (
        config.allowed_file_types
        and input_file_type not in config.allowed_file_types
        and input_file_type != FileType.CUSTOM
    ):
        return False

    if (
        input_file_type == FileType.CUSTOM
        and config.allowed_file_extensions is not None
        and file_extension not in config.allowed_file_extensions
    ):
        return False

    if input_file_type == FileType.IMAGE:
        if (
            config.image_config
            and config.image_config.transfer_methods
            and file_transfer_method not in config.image_config.transfer_methods
        ):
            return False
    elif config.allowed_file_upload_methods and file_transfer_method not in config.allowed_file_upload_methods:
        return False

    return True

```

### api/extensions/storage/opendal_storage.py
```py
import logging
import os
from collections.abc import Generator
from pathlib import Path

import opendal
from dotenv import dotenv_values
from opendal import Operator

from extensions.storage.base_storage import BaseStorage

logger = logging.getLogger(__name__)


def _get_opendal_kwargs(*, scheme: str, env_file_path: str = ".env", prefix: str = "OPENDAL_"):
    kwargs = {}
    config_prefix = prefix + scheme.upper() + "_"
    for key, value in os.environ.items():
        if key.startswith(config_prefix):
            kwargs[key[len(config_prefix) :].lower()] = value

    file_env_vars: dict = dotenv_values(env_file_path) or {}
    for key, value in file_env_vars.items():
        if key.startswith(config_prefix) and key[len(config_prefix) :].lower() not in kwargs and value:
            kwargs[key[len(config_prefix) :].lower()] = value

    return kwargs


class OpenDALStorage(BaseStorage):
    def __init__(self, scheme: str, **kwargs):
        kwargs = kwargs or _get_opendal_kwargs(scheme=scheme)

        if scheme == "fs":
            root = kwargs.setdefault("root", "storage")
            Path(root).mkdir(parents=True, exist_ok=True)

        retry_layer = opendal.layers.RetryLayer(max_times=3, factor=2.0, jitter=True)
        self.op = Operator(scheme=scheme, **kwargs).layer(retry_layer)
        logger.debug("opendal operator created with scheme %s", scheme)
        logger.debug("added retry layer to opendal operator")

    def save(self, filename: str, data: bytes):
        self.op.write(path=filename, bs=data)
        logger.debug("file %s saved", filename)

    def load_once(self, filename: str) -> bytes:
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        content: bytes = self.op.read(path=filename)
        logger.debug("file %s loaded", filename)
        return content

    def load_stream(self, filename: str) -> Generator:
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        batch_size = 4096
        with self.op.open(
            path=filename,
            mode="rb",
            chunck=batch_size,
        ) as file:
            while chunk := file.read(batch_size):
                yield chunk
        logger.debug("file %s loaded as stream", filename)

    def download(self, filename: str, target_filepath: str):
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        Path(target_filepath).write_bytes(self.op.read(path=filename))
        logger.debug("file %s downloaded to %s", filename, target_filepath)

    def exists(self, filename: str) -> bool:
        return self.op.exists(path=filename)

    def delete(self, filename: str):
        if self.exists(filename):
            self.op.delete(path=filename)
            logger.debug("file %s deleted", filename)
            return
        logger.debug("file %s not found, skip delete", filename)

    def scan(self, path: str, files: bool = True, directories: bool = False) -> list[str]:
        if not self.exists(path):
            raise FileNotFoundError("Path not found")

        # Use the new OpenDAL 0.46.0+ API with recursive listing
        lister = self.op.list(path, recursive=True)
        if files and directories:
            logger.debug("files and directories on %s scanned", path)
            return [entry.path for entry in lister]
        if files:
            logger.debug("files on %s scanned", path)
            return [entry.path for entry in lister if not entry.metadata.is_dir]
        elif directories:
            logger.debug("directories on %s scanned", path)
            return [entry.path for entry in lister if entry.metadata.is_dir]
        else:
            raise ValueError("At least one of files or directories must be True")

```

### api/extensions/storage/aliyun_oss_storage.py
```py
import posixpath
from collections.abc import Generator

import oss2 as aliyun_s3

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class AliyunOssStorage(BaseStorage):
    """Implementation for Aliyun OSS storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.ALIYUN_OSS_BUCKET_NAME
        self.folder = dify_config.ALIYUN_OSS_PATH
        oss_auth_method = aliyun_s3.Auth
        region = None
        if dify_config.ALIYUN_OSS_AUTH_VERSION == "v4":
            oss_auth_method = aliyun_s3.AuthV4
            region = dify_config.ALIYUN_OSS_REGION
        oss_auth = oss_auth_method(dify_config.ALIYUN_OSS_ACCESS_KEY, dify_config.ALIYUN_OSS_SECRET_KEY)
        self.client = aliyun_s3.Bucket(
            oss_auth,
            dify_config.ALIYUN_OSS_ENDPOINT,
            self.bucket_name,
            connect_timeout=30,
            region=region,
            cloudbox_id=dify_config.ALIYUN_CLOUDBOX_ID,
        )

    def save(self, filename, data):
        self.client.put_object(self.__wrapper_folder_filename(filename), data)

    def load_once(self, filename: str) -> bytes:
        obj = self.client.get_object(self.__wrapper_folder_filename(filename))
        data = obj.read()
        if not isinstance(data, bytes):
            return b""
        return data

    def load_stream(self, filename: str) -> Generator:
        obj = self.client.get_object(self.__wrapper_folder_filename(filename))
        while chunk := obj.read(4096):
            yield chunk

    def download(self, filename: str, target_filepath):
        self.client.get_object_to_file(self.__wrapper_folder_filename(filename), target_filepath)

    def exists(self, filename: str):
        return self.client.object_exists(self.__wrapper_folder_filename(filename))

    def delete(self, filename: str):
        self.client.delete_object(self.__wrapper_folder_filename(filename))

    def __wrapper_folder_filename(self, filename: str) -> str:
        return posixpath.join(self.folder, filename) if self.folder else filename

```

### api/extensions/storage/azure_blob_storage.py
```py
from collections.abc import Generator
from datetime import timedelta

from azure.identity import ChainedTokenCredential, DefaultAzureCredential
from azure.storage.blob import AccountSasPermissions, BlobServiceClient, ResourceTypes, generate_account_sas

from configs import dify_config
from extensions.ext_redis import redis_client
from extensions.storage.base_storage import BaseStorage
from libs.datetime_utils import naive_utc_now


class AzureBlobStorage(BaseStorage):
    """Implementation for Azure Blob storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.AZURE_BLOB_CONTAINER_NAME
        self.account_url = dify_config.AZURE_BLOB_ACCOUNT_URL
        self.account_name = dify_config.AZURE_BLOB_ACCOUNT_NAME
        self.account_key = dify_config.AZURE_BLOB_ACCOUNT_KEY

        self.credential: ChainedTokenCredential | None = None
        if self.account_key == "managedidentity":
            self.credential = DefaultAzureCredential()
        else:
            self.credential = None

    def save(self, filename, data):
        if not self.bucket_name:
            return

        client = self._sync_client()
        blob_container = client.get_container_client(container=self.bucket_name)
        blob_container.upload_blob(filename, data)

    def load_once(self, filename: str) -> bytes:
        if not self.bucket_name:
            raise FileNotFoundError("Azure bucket name is not configured.")

        client = self._sync_client()
        blob = client.get_container_client(container=self.bucket_name)
        blob = blob.get_blob_client(blob=filename)
        data = blob.download_blob().readall()
        if not isinstance(data, bytes):
            raise TypeError(f"Expected bytes from blob.readall(), got {type(data).__name__}")
        return data

    def load_stream(self, filename: str) -> Generator:
        if not self.bucket_name:
            raise FileNotFoundError("Azure bucket name is not configured.")

        client = self._sync_client()
        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        blob_data = blob.download_blob()
        yield from blob_data.chunks()

    def download(self, filename, target_filepath):
        if not self.bucket_name:
            return

        client = self._sync_client()

        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        with open(target_filepath, "wb") as my_blob:
            blob_data = blob.download_blob()
            blob_data.readinto(my_blob)

    def exists(self, filename):
        if not self.bucket_name:
            return False

        client = self._sync_client()

        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        return blob.exists()

    def delete(self, filename: str):
        if not self.bucket_name:
            return

        client = self._sync_client()

        blob_container = client.get_container_client(container=self.bucket_name)
        blob_container.delete_blob(filename)

    def _sync_client(self):
        if self.account_key == "managedidentity":
            return BlobServiceClient(account_url=self.account_url, credential=self.credential)  # type: ignore

        cache_key = f"azure_blob_sas_token_{self.account_name}_{self.account_key}"
        cache_result = redis_client.get(cache_key)
        if cache_result is not None:
            sas_token = cache_result.decode("utf-8")
        else:
            sas_token = generate_account_sas(
                account_name=self.account_name or "",
                account_key=self.account_key or "",
                resource_types=ResourceTypes(service=True, container=True, object=True),
                permission=AccountSasPermissions(read=True, write=True, delete=True, list=True, add=True, create=True),
                expiry=naive_utc_now() + timedelta(hours=1),
            )
            redis_client.set(cache_key, sas_token, ex=3000)
        return BlobServiceClient(account_url=self.account_url or "", credential=sas_token)

```

### api/extensions/storage/volcengine_tos_storage.py
```py
from collections.abc import Generator

import tos

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class VolcengineTosStorage(BaseStorage):
    """Implementation for Volcengine TOS storage."""

    def __init__(self):
        super().__init__()
        if not dify_config.VOLCENGINE_TOS_ACCESS_KEY:
            raise ValueError("VOLCENGINE_TOS_ACCESS_KEY is not set")
        if not dify_config.VOLCENGINE_TOS_SECRET_KEY:
            raise ValueError("VOLCENGINE_TOS_SECRET_KEY is not set")
        if not dify_config.VOLCENGINE_TOS_ENDPOINT:
            raise ValueError("VOLCENGINE_TOS_ENDPOINT is not set")
        if not dify_config.VOLCENGINE_TOS_REGION:
            raise ValueError("VOLCENGINE_TOS_REGION is not set")
        self.bucket_name = dify_config.VOLCENGINE_TOS_BUCKET_NAME
        self.client = tos.TosClientV2(
            ak=dify_config.VOLCENGINE_TOS_ACCESS_KEY,
            sk=dify_config.VOLCENGINE_TOS_SECRET_KEY,
            endpoint=dify_config.VOLCENGINE_TOS_ENDPOINT,
            region=dify_config.VOLCENGINE_TOS_REGION,
        )

    def save(self, filename, data):
        if not self.bucket_name:
            raise ValueError("VOLCENGINE_TOS_BUCKET_NAME is not set")
        self.client.put_object(bucket=self.bucket_name, key=filename, content=data)

    def load_once(self, filename: str) -> bytes:
        if not self.bucket_name:
            raise FileNotFoundError("VOLCENGINE_TOS_BUCKET_NAME is not set")
        data = self.client.get_object(bucket=self.bucket_name, key=filename).read()
        if not isinstance(data, bytes):
            raise TypeError(f"Expected bytes, got {type(data).__name__}")
        return data

    def load_stream(self, filename: str) -> Generator:
        if not self.bucket_name:
            raise FileNotFoundError("VOLCENGINE_TOS_BUCKET_NAME is not set")
        response = self.client.get_object(bucket=self.bucket_name, key=filename)
        while chunk := response.read(4096):
            yield chunk

    def download(self, filename, target_filepath):
        if not self.bucket_name:
            raise ValueError("VOLCENGINE_TOS_BUCKET_NAME is not set")
        self.client.get_object_to_file(bucket=self.bucket_name, key=filename, file_path=target_filepath)

    def exists(self, filename):
        if not self.bucket_name:
            return False
        res = self.client.head_object(bucket=self.bucket_name, key=filename)
        if res.status_code != 200:
            return False
        return True

    def delete(self, filename: str):
        if not self.bucket_name:
            return
        self.client.delete_object(bucket=self.bucket_name, key=filename)

```

### api/extensions/storage/google_cloud_storage.py
```py
import base64
import io
from collections.abc import Generator
from typing import Any

from google.cloud import storage as google_cloud_storage  # type: ignore
from pydantic import TypeAdapter

from configs import dify_config
from extensions.storage.base_storage import BaseStorage

_service_account_adapter: TypeAdapter[dict[str, Any]] = TypeAdapter(dict[str, Any])


class GoogleCloudStorage(BaseStorage):
    """Implementation for Google Cloud storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.GOOGLE_STORAGE_BUCKET_NAME
        service_account_json_str = dify_config.GOOGLE_STORAGE_SERVICE_ACCOUNT_JSON_BASE64
        # if service_account_json_str is empty, use Application Default Credentials
        if service_account_json_str:
            service_account_json = base64.b64decode(service_account_json_str).decode("utf-8")
            # convert str to object
            service_account_obj = _service_account_adapter.validate_json(service_account_json)
            self.client = google_cloud_storage.Client.from_service_account_info(service_account_obj)
        else:
            self.client = google_cloud_storage.Client()

    def save(self, filename, data):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.blob(filename)
        with io.BytesIO(data) as stream:
            blob.upload_from_file(stream)

    def load_once(self, filename: str) -> bytes:
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        data: bytes = blob.download_as_bytes()
        return data

    def load_stream(self, filename: str) -> Generator:
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        with blob.open(mode="rb") as blob_stream:
            while chunk := blob_stream.read(4096):
                yield chunk

    def download(self, filename, target_filepath):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        blob.download_to_filename(target_filepath)

    def exists(self, filename):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.blob(filename)
        return blob.exists()

    def delete(self, filename: str):
        bucket = self.client.get_bucket(self.bucket_name)
        bucket.delete_blob(filename)

```

### api/extensions/storage/base_storage.py
```py
"""Abstract interface for file storage implementations."""

from abc import ABC, abstractmethod
from collections.abc import Generator


class BaseStorage(ABC):
    """Interface for file storage."""

    @abstractmethod
    def save(self, filename: str, data: bytes):
        raise NotImplementedError

    @abstractmethod
    def load_once(self, filename: str) -> bytes:
        raise NotImplementedError

    @abstractmethod
    def load_stream(self, filename: str) -> Generator:
        raise NotImplementedError

    @abstractmethod
    def download(self, filename: str, target_filepath: str) -> None:
        raise NotImplementedError

    @abstractmethod
    def exists(self, filename: str) -> bool:
        raise NotImplementedError

    @abstractmethod
    def delete(self, filename: str):
        raise NotImplementedError

    def scan(self, path, files=True, directories=False) -> list[str]:
        """
        Scan files and directories in the given path.
        This method is implemented only in some storage backends.
        If a storage backend doesn't support scanning, it will raise NotImplementedError.
        """
        raise NotImplementedError("This storage backend doesn't support scanning")

```

### api/extensions/storage/oracle_oci_storage.py
```py
from collections.abc import Generator

import boto3
from botocore.exceptions import ClientError

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class OracleOCIStorage(BaseStorage):
    """Implementation for Oracle OCI storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.OCI_BUCKET_NAME
        self.client = boto3.client(
            "s3",
            aws_secret_access_key=dify_config.OCI_SECRET_KEY,
            aws_access_key_id=dify_config.OCI_ACCESS_KEY,
            endpoint_url=dify_config.OCI_ENDPOINT,
            region_name=dify_config.OCI_REGION,
        )

    def save(self, filename, data):
        self.client.put_object(Bucket=self.bucket_name, Key=filename, Body=data)

    def load_once(self, filename: str) -> bytes:
        try:
            data: bytes = self.client.get_object(Bucket=self.bucket_name, Key=filename)["Body"].read()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("File not found")
            else:
                raise
        return data

    def load_stream(self, filename: str) -> Generator:
        try:
            response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
            yield from response["Body"].iter_chunks()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("File not found")
            else:
                raise

    def download(self, filename, target_filepath):
        self.client.download_file(self.bucket_name, filename, target_filepath)

    def exists(self, filename):
        try:
            self.client.head_object(Bucket=self.bucket_name, Key=filename)
            return True
        except:
            return False

    def delete(self, filename: str):
        self.client.delete_object(Bucket=self.bucket_name, Key=filename)

```

### api/extensions/storage/supabase_storage.py
```py
import io
from collections.abc import Generator
from pathlib import Path

from supabase import Client

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class SupabaseStorage(BaseStorage):
    """Implementation for supabase obs storage."""

    def __init__(self):
        super().__init__()
        if dify_config.SUPABASE_URL is None:
            raise ValueError("SUPABASE_URL is not set")
        if dify_config.SUPABASE_API_KEY is None:
            raise ValueError("SUPABASE_API_KEY is not set")
        if dify_config.SUPABASE_BUCKET_NAME is None:
            raise ValueError("SUPABASE_BUCKET_NAME is not set")

        self.bucket_name = dify_config.SUPABASE_BUCKET_NAME
        self.client = Client(supabase_url=dify_config.SUPABASE_URL, supabase_key=dify_config.SUPABASE_API_KEY)
        self.create_bucket(id=dify_config.SUPABASE_BUCKET_NAME, bucket_name=dify_config.SUPABASE_BUCKET_NAME)

    def create_bucket(self, id, bucket_name):
        if not self.bucket_exists():
            self.client.storage.create_bucket(id=id, name=bucket_name)

    def save(self, filename, data):
        self.client.storage.from_(self.bucket_name).upload(filename, data)

    def load_once(self, filename: str) -> bytes:
        content: bytes = self.client.storage.from_(self.bucket_name).download(filename)
        return content

    def load_stream(self, filename: str) -> Generator:
        result = self.client.storage.from_(self.bucket_name).download(filename)
        byte_stream = io.BytesIO(result)
        while chunk := byte_stream.read(4096):  # Read in chunks of 4KB
            yield chunk

    def download(self, filename, target_filepath):
        result = self.client.storage.from_(self.bucket_name).download(filename)
        Path(target_filepath).write_bytes(result)

    def exists(self, filename):
        result = self.client.storage.from_(self.bucket_name).list(path=filename)
        if len(result) > 0:
            return True
        return False

    def delete(self, filename: str):
        self.client.storage.from_(self.bucket_name).remove([filename])

    def bucket_exists(self):
        buckets = self.client.storage.list_buckets()
        return any(bucket.name == self.bucket_name for bucket in buckets)

```

### api/extensions/storage/tencent_cos_storage.py
```py
from collections.abc import Generator

from qcloud_cos import CosConfig, CosS3Client

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class TencentCosStorage(BaseStorage):
    """Implementation for Tencent Cloud COS storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.TENCENT_COS_BUCKET_NAME
        if dify_config.TENCENT_COS_CUSTOM_DOMAIN:
            config = CosConfig(
                Domain=dify_config.TENCENT_COS_CUSTOM_DOMAIN,
                SecretId=dify_config.TENCENT_COS_SECRET_ID,
                SecretKey=dify_config.TENCENT_COS_SECRET_KEY,
                Scheme=dify_config.TENCENT_COS_SCHEME,
            )
        else:
            config = CosConfig(
                Region=dify_config.TENCENT_COS_REGION,
                SecretId=dify_config.TENCENT_COS_SECRET_ID,
                SecretKey=dify_config.TENCENT_COS_SECRET_KEY,
                Scheme=dify_config.TENCENT_COS_SCHEME,
            )
        self.client = CosS3Client(config)

    def save(self, filename, data):
        self.client.put_object(Bucket=self.bucket_name, Body=data, Key=filename)

    def load_once(self, filename: str) -> bytes:
        data: bytes = self.client.get_object(Bucket=self.bucket_name, Key=filename)["Body"].get_raw_stream().read()
        return data

    def load_stream(self, filename: str) -> Generator:
        response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
        yield from response["Body"].get_stream(chunk_size=4096)

    def download(self, filename, target_filepath):
        response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
        response["Body"].get_stream_to_file(target_filepath)

    def exists(self, filename):
        return self.client.object_exists(Bucket=self.bucket_name, Key=filename)

    def delete(self, filename: str):
        self.client.delete_object(Bucket=self.bucket_name, Key=filename)

```

### api/extensions/storage/aws_s3_storage.py
```py
import logging
from collections.abc import Generator

import boto3
from botocore.client import Config
from botocore.exceptions import ClientError

from configs import dify_config
from extensions.storage.base_storage import BaseStorage

logger = logging.getLogger(__name__)


class AwsS3Storage(BaseStorage):
    """Implementation for Amazon Web Services S3 storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.S3_BUCKET_NAME
        if dify_config.S3_USE_AWS_MANAGED_IAM:
            logger.info("Using AWS managed IAM role for S3")

            session = boto3.Session()
            region_name = dify_config.S3_REGION
            self.client = session.client(service_name="s3", region_name=region_name)
        else:
            logger.info("Using ak and sk for S3")

            self.client = boto3.client(
                "s3",
                aws_secret_access_key=dify_config.S3_SECRET_KEY,
                aws_access_key_id=dify_config.S3_ACCESS_KEY,
                endpoint_url=dify_config.S3_ENDPOINT,
                region_name=dify_config.S3_REGION,
                config=Config(s3={"addressing_style": dify_config.S3_ADDRESS_STYLE}),
            )
        # create bucket
        try:
            self.client.head_bucket(Bucket=self.bucket_name)
        except ClientError as e:
            # if bucket not exists, create it
            if e.response.get("Error", {}).get("Code") == "404":
                self.client.create_bucket(Bucket=self.bucket_name)
            # if bucket is not accessible, pass, maybe the bucket is existing but not accessible
            elif e.response.get("Error", {}).get("Code") == "403":
                pass
            else:
                # other error, raise exception
                raise

    def save(self, filename, data):
        self.client.put_object(Bucket=self.bucket_name, Key=filename, Body=data)

    def load_once(self, filename: str) -> bytes:
        try:
            data: bytes = self.client.get_object(Bucket=self.bucket_name, Key=filename)["Body"].read()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("File not found")
            else:
                raise
        return data

    def load_stream(self, filename: str) -> Generator:
        try:
            response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
            yield from response["Body"].iter_chunks()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("file not found")
            elif "reached max retries" in str(ex):
                raise ValueError("please do not request the same file too frequently")
            else:
                raise

    def download(self, filename, target_filepath):
        self.client.download_file(self.bucket_name, filename, target_filepath)

    def exists(self, filename):
        try:
            self.client.head_object(Bucket=self.bucket_name, Key=filename)
            return True
        except:
            return False

    def delete(self, filename: str):
        self.client.delete_object(Bucket=self.bucket_name, Key=filename)

```

### api/extensions/storage/clickzetta_volume/volume_permissions.py
```py
"""ClickZetta Volume permission management mechanism

This module provides Volume permission checking, validation and management features.
According to ClickZetta's permission model, different Volume types have different permission requirements.
"""

import logging
from enum import StrEnum

logger = logging.getLogger(__name__)


class VolumePermission(StrEnum):
    """Volume permission type enumeration"""

    READ = "SELECT"  # Corresponds to ClickZetta's SELECT permission
    WRITE = "INSERT,UPDATE,DELETE"  # Corresponds to ClickZetta's write permissions
    LIST = "SELECT"  # Listing files requires SELECT permission
    DELETE = "INSERT,UPDATE,DELETE"  # Deleting files requires write permissions
    USAGE = "USAGE"  # Basic permission required for External Volume


class VolumePermissionManager:
    """Volume permission manager"""

    def __init__(self, connection_or_config, volume_type: str | None = None, volume_name: str | None = None):
        """Initialize permission manager

        Args:
            connection_or_config: ClickZetta connection object or configuration dictionary
            volume_type: Volume type (user|table|external)
            volume_name: Volume name (for external volume)
        """
        # Support two initialization methods: connection object or configuration dictionary
        if isinstance(connection_or_config, dict):
            # Create connection from configuration dictionary
            import clickzetta

            config = connection_or_config
            self._connection = clickzetta.connect(
                username=config.get("username"),
                password=config.get("password"),
                instance=config.get("instance"),
                service=config.get("service"),
                workspace=config.get("workspace"),
                vcluster=config.get("vcluster"),
                schema=config.get("schema") or config.get("database"),
            )
            self._volume_type = config.get("volume_type", volume_type)
            self._volume_name = config.get("volume_name", volume_name)
        else:
            # Use connection object directly
            self._connection = connection_or_config
            self._volume_type = volume_type
            self._volume_name = volume_name

        if not self._connection:
            raise ValueError("Valid connection or config is required")
        if not self._volume_type:
            raise ValueError("volume_type is required")

        self._permission_cache: dict[str, set[str]] = {}
        self._current_username = None  # Will get current username from connection

    def check_permission(self, operation: VolumePermission, dataset_id: str | None = None) -> bool:
        """Check if user has permission to perform specific operation

        Args:
            operation: Type of operation to perform
            dataset_id: Dataset ID (for table volume)

        Returns:
            True if user has permission, False otherwise
        """
        try:
            if self._volume_type == "user":
                return self._check_user_volume_permission(operation)
            elif self._volume_type == "table":
                return self._check_table_volume_permission(operation, dataset_id)
            elif self._volume_type == "external":
                return self._check_external_volume_permission(operation)
            else:
                logger.warning("Unknown volume type: %s", self._volume_type)
                return False

        except Exception:
            logger.exception("Permission check failed")
            return False

    def _check_user_volume_permission(self, operation: VolumePermission) -> bool:
        """Check User Volume permission

        User Volume permission rules:
        - User has full permissions on their own User Volume
        - As long as user can connect to ClickZetta, they have basic User Volume permissions by default
        - Focus more on connection authentication rather than complex permission checking
        """
        try:
            # Get current username
            current_user = self._get_current_username()

            # Check basic connection status
            with self._connection.cursor() as cursor:
                # Simple connection test, if query can be executed user has basic permissions
                cursor.execute("SELECT 1")
                result = cursor.fetchone()

                if result:
                    logger.debug(
                        "User Volume permission check for %s, operation %s: granted (basic connection verified)",
                        current_user,
                        operation.name,
                    )
                    return True
                else:
                    logger.warning(
                        "User Volume permission check failed: cannot verify basic connection for %s", current_user
                    )
                    return False

        except Exception:
            logger.exception("User Volume permission check failed")
            # For User Volume, if permission check fails, it might be a configuration issue,
            # provide friendlier error message
            logger.info("User Volume permission check failed, but permission checking is disabled in this version")
            return False

    def _check_table_volume_permission(self, operation: VolumePermission, dataset_id: str | None) -> bool:
        """Check Table Volume permission

        Table Volume permission rules:
        - Table Volume permissions inherit from corresponding table permissions
        - SELECT permission -> can READ/LIST files
        - INSERT,UPDATE,DELETE permissions -> can WRITE/DELETE files
        """
        if not dataset_id:
            logger.warning("dataset_id is required for table volume permission check")
            return False

        table_name = f"dataset_{dataset_id}" if not dataset_id.startswith("dataset_") else dataset_id

        try:
            # Check table permissions
            permissions = self._get_table_permissions(table_name)
            required_permissions = set(operation.value.split(","))

            # Check if has all required permissions
            has_permission = required_permissions.issubset(permissions)

            logger.debug(
                "Table Volume permission check for %s, operation %s: required=%s, has=%s, granted=%s",
                table_name,
                operation.name,
                required_permissions,
                permissions,
                has_permission,
            )

            return has_permission

        except Exception:
            logger.exception("Table volume permission check failed for %s", table_name)
            return False

    def _check_external_volume_permission(self, operation: VolumePermission) -> bool:
        """Check External Volume permission

        External Volume permission rules:
        - Try to get permissions for External Volume
        - If permission check fails, perform fallback verification
        - For development environment, provide more lenient permission checking
        """
        if not self._volume_name:
            logger.warning("volume_name is required for external volume permission check")
            return False

        try:
            # Check External Volume permissions
            permissions = self._get_external_volume_permissions(self._volume_name)

            # External Volume permission mapping: determine required permissions based on operation type
            required_permissions = set()

            if operation in [VolumePermission.READ, VolumePermission.LIST]:
                required_permissions.add("read")
            elif operation in [VolumePermission.WRITE, VolumePermission.DELETE]:
                required_permissions.add("write")

            # Check if has all required permissions
            has_permission = required_permissions.issubset(permissions)

            logger.debug(
                "External Volume permission check for %s, operation %s: required=%s, has=%s, granted=%s",
                self._volume_name,
                operation.name,
                required_permissions,
                permissions,
                has_permission,
            )

            # If permission check fails, try fallback verification
            if not has_permission:
                logger.info("Direct permission check failed for %s, trying fallback verification", self._volume_name)

                # Fallback verification: try listing Volume to verify basic access permissions
                try:
                    with self._connection.cursor() as cursor:
                        cursor.execute("SHOW VOLUMES")
                        volumes = cursor.fetchall()
                        for volume in volumes:
                            if len(volume) > 0 and volume[0] == self._volume_name:
                                logger.info("Fallback verification successful for %s", self._volume_name)
                                return True
                except Exception as fallback_e:
                    logger.warning("Fallback verification failed for %s: %s", self._volume_name, fallback_e)

            return has_permission

        except Exception:
            logger.exception("External volume permission check failed for %s", self._volume_name)
            logger.info("External Volume permission check failed, but permission checking is disabled in this version")
            return False

    def _get_table_permissions(self, table_name: str) -> set[str]:
        """Get user permissions for specified table

        Args:
            table_name: Table name

        Returns:
            Set of user permissions for this table
        """
        cache_key = f"table:{table_name}"

        if cache_key in self._permission_cache:
            return self._permission_cache[cache_key]

        permissions = set()

        try:
            with self._connection.cursor() as cursor:
                # Use correct ClickZetta syntax to check current user permissions
                cursor.execute("SHOW GRANTS")
                grants = cursor.fetchall()

                # Parse permission results, find permissions for this table
                for grant in grants:
                    if len(grant) >= 3:  # Typical format: (privilege, object_type, object_name, ...)
                        privilege = grant[0].upper()
                        object_type = grant[1].upper() if len(grant) > 1 else ""
                        object_name = grant[2] if len(grant) > 2 else ""

                        # Check if it's permission for this table
                        if (
                            object_type == "TABLE"
                            and object_name == table_name
                            or object_type == "SCHEMA"
                            and object_name in table_name
                        ):
                            if privilege in ["SELECT", "INSERT", "UPDATE", "DELETE", "ALL"]:
                                if privilege == "ALL":
                                    permissions.update(["SELECT", "INSERT", "UPDATE", "DELETE"])
                                else:
                                    permissions.add(privilege)

                # If no explicit permissions found, try executing a simple query to verify permissions
                if not permissions:
                    try:
                        cursor.execute(f"SELECT COUNT(*) FROM {table_name} LIMIT 1")
                        permissions.add("SELECT")
                    except Exception:
                        logger.debug("Cannot query table %s, no SELECT permission", table_name)

        except Exception as e:
            logger.warning("Could not check table permissions for %s: %s", table_name, e)
            # Safe default: deny access when permission check fails
            pass

        # Cache permission information
        self._permission_cache[cache_key] = permissions
        return permissions

    def _get_current_username(self) -> str:
        """Get current username"""
        if self._current_username:
            return self._current_username

        try:
            with self._connection.cursor() as cursor:
                cursor.execute("SELECT CURRENT_USER()")
                result = cursor.fetchone()
                if result:
                    self._current_username = result[0]
                    return str(self._current_username)
        except Exception:
            logger.exception("Failed to get current username")

        return "unknown"

    def _get_user_permissions(self, username: str) -> set[str]:
        """Get user's basic permission set"""
        cache_key = f"user_permissions:{username}"

        if cache_key in self._permission_cache:
            return self._permission_cache[cache_key]

        permissions = set()

        try:
            with self._connection.cursor() as cursor:
                # Use correct ClickZetta syntax to check current user permissions
                cursor.execute("SHOW GRANTS")
                grants = cursor.fetchall()

                # Parse permission results, find user's basic permissions
                for grant in grants:
                    if len(grant) >= 3:  # Typical format: (privilege, object_type, object_name, ...)
                        privilege = grant[0].upper()
                        _ = grant[1].upper() if len(grant) > 1 else ""

                        # Collect all relevant permissions
                        if privilege in ["SELECT", "INSERT", "UPDATE", "DELETE", "ALL"]:
                            if privilege == "ALL":
                                permissions.update(["SELECT", "INSERT", "UPDATE", "DELETE"])
                            else:
                                permissions.add(privilege)

        except Exception as e:
            logger.warning("Could not check user permissions for %s: %s", username, e)
            # Safe default: deny access when permission check fails
            pass

        # Cache permission information
        self._permission_cache[cache_key] = permissions
        return permissions

    def _get_external_volume_permissions(self, volume_name: str) -> set[str]:
        """Get user permissions for specified External Volume

        Args:
            volume_name: External Volume name

        Returns:
            Set of user permissions for this Volume
        """
        cache_key = f"external_volume:{volume_name}"

        if cache_key in self._permission_cache:
            return self._permission_cache[cache_key]

        permissions = set()

        try:
            with self._connection.cursor() as cursor:
                # Use correct ClickZetta syntax to check Volume permissions
                logger.info("Checking permissions for volume: %s", volume_name)
                cursor.execute(f"SHOW GRANTS ON VOLUME {volume_name}")
                grants = cursor.fetchall()

                logger.info("Raw grants result for %s: %s", volume_name, grants)

                # Parse permission results
                # Format: (granted_type, privilege, conditions, granted_on, object_name, granted_to,
                #       grantee_name, grantor_name, grant_option, granted_time)
                for grant in grants:
                    logger.info("Processing grant: %s", grant)
                    if len(grant) >= 5:
                        granted_type = grant[0]
                        privilege = grant[1].upper()
                        granted_on = grant[3]
                        object_name = grant[4]

                        logger.info(
                            "Grant details - type: %s, privilege: %s, granted_on: %s, object_name: %s",
                            granted_type,
                            privilege,
                            granted_on,
                            object_name,
                        )

                        # Check if it's permission for this Volume or hierarchical permission
                        if (
                            granted_type == "PRIVILEGE" and granted_on == "VOLUME" and object_name.endswith(volume_name)
                        ) or (granted_type == "OBJECT_HIERARCHY" and granted_on == "VOLUME"):
                            logger.info("Matching grant found for %s", volume_name)

                            if "READ" in privilege:
                                permissions.add("read")
                                logger.info("Added READ permission for %s", volume_name)
                            if "WRITE" in privilege:
                                permissions.add("write")
                                logger.info("Added WRITE permission for %s", volume_name)
                            if "ALTER" in privilege:
                                permissions.add("alter")
                                logger.info("Added ALTER permission for %s", volume_name)
                            if privilege == "ALL":
                                permissions.update(["read", "write", "alter"])
                                logger.info("Added ALL permissions for %s", volume_name)

                logger.info("Final permissions for %s: %s", volume_name, permissions)

                # If no explicit permissions found, try viewing Volume list to verify basic permissions
                if not permissions:
                    try:
                        cursor.execute("SHOW VOLUMES")
                        volumes = cursor.fetchall()
                        for volume in volumes:
                            if len(volume) > 0 and volume[0] == volume_name:
                                permissions.add("read")  # At least has read permission
                                logger.debug("Volume %s found in SHOW VOLUMES, assuming read permission", volume_name)
                                break
                    except Exception:
                        logger.debug("Cannot access volume %s, no basic permission", volume_name)

        except Exception as e:
            logger.warning("Could not check external volume permissions for %s: %s", volume_name, e)
            # When permission check fails, try basic Volume access verification
            try:
                with self._connection.cursor() as cursor:
                    cursor.execute("SHOW VOLUMES")
                    volumes = cursor.fetchall()
                    for volume in volumes:
                        if len(volume) > 0 and volume[0] == volume_name:
                            logger.info("Basic volume access verified for %s", volume_name)
                            permissions.add("read")
                            permissions.add("write")  # Assume has write permission
                            break
            except Exception as basic_e:
                logger.warning("Basic volume access check failed for %s: %s", volume_name, basic_e)
                # Last fallback: assume basic permissions
                permissions.add("read")

        # Cache permission information
        self._permission_cache[cache_key] = permissions
        return permissions

    def clear_permission_cache(self):
        """Clear permission cache"""
        self._permission_cache.clear()
        logger.debug("Permission cache cleared")

    @property
    def volume_type(self) -> str | None:
        """Get the volume type."""
        return self._volume_type

    def get_permission_summary(self, dataset_id: str | None = None) -> dict[str, bool]:
        """Get permission summary

        Args:
            dataset_id: Dataset ID (for table volume)

        Returns:
            Permission summary dictionary
        """
        summary = {}

        for operation in VolumePermission:
            summary[operation.name.lower()] = self.check_permission(operation, dataset_id)

        return summary

    def check_inherited_permission(self, file_path: str, operation: VolumePermission) -> bool:
        """Check permission inheritance for file path

        Args:
            file_path: File path
            operation: Operation to perform

        Returns:
            True if user has permission, False otherwise
        """
        try:
            # Parse file path
            path_parts = file_path.strip("/").split("/")

            if not path_parts:
                logger.warning("Invalid file path for permission inheritance check")
                return False

            # For Table Volume, first layer is dataset_id
            if self._volume_type == "table":
                if len(path_parts) < 1:
                    return False

                dataset_id = path_parts[0]

                # Check permissions for dataset
                has_dataset_permission = self.check_permission(operation, dataset_id)

                if not has_dataset_permission:
                    logger.debug("Permission denied for dataset %s", dataset_id)
                    return False

                # Check path traversal attack
                if self._contains_path_traversal(file_path):
                    logger.warning("Path traversal attack detected: %s", file_path)
                    return False

                # Check if accessing sensitive directory
                if self._is_sensitive_path(file_path):
                    logger.warning("Access to sensitive path denied: %s", file_path)
                    return False

                logger.debug("Permission inherited for path %s", file_path)
                return True

            elif self._volume_type == "user":
                # User Volume permission inheritance
                current_user = self._get_current_username()

                # Check if attempting to access other user's directory
                if len(path_parts) > 1 and path_parts[0] != current_user:
                    logger.warning("User %s attempted to access %s's directory", current_user, path_parts[0])
                    return False

                # Check basic permissions
                return self.check_permission(operation)

            elif self._volume_type == "external":
                # External Volume permission inheritance
                # Check permissions for External Volume
                return self.check_permission(operation)

            else:
                logger.warning("Unknown volume type for permission inheritance: %s", self._volume_type)
                return False

        except Exception:
            logger.exception("Permission inheritance check failed")
            return False

    def _contains_path_traversal(self, file_path: str) -> bool:
        """Check if path contains path traversal attack"""
        # Check common path traversal patterns
        traversal_patterns = [
            "../",
            "..\\",
            "..%2f",
            "..%2F",
            "..%5c",
            "..%5C",
            "%2e%2e%2f",
            "%2e%2e%5c",
            "....//",
            "....\\\\",
        ]

        file_path_lower = file_path.lower()

        for pattern in traversal_patterns:
            if pattern in file_path_lower:
                return True

        # Check absolute path
        if file_path.startswith("/") or file_path.startswith("\\"):
            return True

        # Check Windows drive path
        if len(file_path) >= 2 and file_path[1] == ":":
            return True

        return False

    def _is_sensitive_path(self, file_path: str) -> bool:
        """Check if path is sensitive path"""
        sensitive_patterns = [
            "passwd",
            "shadow",
            "hosts",
            "config",
            "secrets",
            "private",
            "key",
            "certificate",
            "cert",
            "ssl",
            "database",
            "backup",
            "dump",
            "log",
            "tmp",
        ]

        file_path_lower = file_path.lower()

        return any(pattern in file_path_lower for pattern in sensitive_patterns)

    def validate_operation(self, operation: str, dataset_id: str | None = None) -> bool:
        """Validate operation permission

        Args:
            operation: Operation name (save|load|exists|delete|scan)
            dataset_id: Dataset ID

        Returns:
            True if operation is allowed, False otherwise
        """
        operation_mapping = {
            "save": VolumePermission.WRITE,
            "load": VolumePermission.READ,
            "load_once": VolumePermission.READ,
            "load_stream": VolumePermission.READ,
            "download": VolumePermission.READ,
            "exists": VolumePermission.READ,
            "delete": VolumePermission.DELETE,
            "scan": VolumePermission.LIST,
        }

        if operation not in operation_mapping:
            logger.warning("Unknown operation: %s", operation)
            return False

        volume_permission = operation_mapping[operation]
        return self.check_permission(volume_permission, dataset_id)


class VolumePermissionError(Exception):
    """Volume permission error exception"""

    def __init__(self, message: str, operation: str, volume_type: str, dataset_id: str | None = None):
        self.operation = operation
        self.volume_type = volume_type
        self.dataset_id = dataset_id
        super().__init__(message)


def check_volume_permission(permission_manager: VolumePermissionManager, operation: str, dataset_id: str | None = None):
    """Permission check decorator function

    Args:
        permission_manager: Permission manager
        operation: Operation name
        dataset_id: Dataset ID

    Raises:
        VolumePermissionError: If no permission
    """
    if not permission_manager.validate_operation(operation, dataset_id):
        error_message = f"Permission denied for operation '{operation}' on {permission_manager.volume_type} volume"
        if dataset_id:
            error_message += f" (dataset: {dataset_id})"

        raise VolumePermissionError(
            error_message,
            operation=operation,
            volume_type=permission_manager.volume_type or "unknown",
            dataset_id=dataset_id,
        )

```

### api/extensions/storage/clickzetta_volume/__init__.py
```py
"""ClickZetta Volume storage implementation."""

from .clickzetta_volume_storage import ClickZettaVolumeStorage

__all__ = ["ClickZettaVolumeStorage"]

```

### api/extensions/storage/clickzetta_volume/file_lifecycle.py
```py
"""ClickZetta Volume file lifecycle management

This module provides file lifecycle management features including version control,
automatic cleanup, backup and restore.
Supports complete lifecycle management for knowledge base files.
"""

from __future__ import annotations

import json
import logging
import operator
from dataclasses import asdict, dataclass
from datetime import datetime
from enum import StrEnum, auto
from typing import Any, TypedDict

from pydantic import TypeAdapter

logger = logging.getLogger(__name__)

_metadata_adapter: TypeAdapter[dict[str, Any]] = TypeAdapter(dict[str, Any])


class StorageStatisticsDict(TypedDict):
    total_files: int
    active_files: int
    archived_files: int
    deleted_files: int
    total_size: int
    versions_count: int
    oldest_file: str | None
    newest_file: str | None


class FileStatus(StrEnum):
    """File status enumeration"""

    ACTIVE = auto()  # Active status
    ARCHIVED = auto()  # Archived
    DELETED = auto()  # Deleted (soft delete)
    BACKUP = auto()  # Backup file


@dataclass
class FileMetadata:
    """File metadata"""

    filename: str
    size: int | None
    created_at: datetime
    modified_at: datetime
    version: int | None
    status: FileStatus
    checksum: str | None = None
    tags: dict[str, str] | None = None
    parent_version: int | None = None

    def to_dict(self):
        """Convert to dictionary format"""
        data = asdict(self)
        data["created_at"] = self.created_at.isoformat()
        data["modified_at"] = self.modified_at.isoformat()
        data["status"] = self.status.value
        return data

    @classmethod
    def from_dict(cls, data: dict) -> FileMetadata:
        """Create instance from dictionary"""
        data = data.copy()
        data["created_at"] = datetime.fromisoformat(data["created_at"])
        data["modified_at"] = datetime.fromisoformat(data["modified_at"])
        data["status"] = FileStatus(data["status"])
        return cls(**data)


class FileLifecycleManager:
    """File lifecycle manager"""

    def __init__(self, storage, dataset_id: str | None = None):
        """Initialize lifecycle manager

        Args:
            storage: ClickZetta Volume storage instance
            dataset_id: Dataset ID (for Table Volume)
        """
        self._storage = storage
        self._dataset_id = dataset_id
        self._metadata_file = ".dify_file_metadata.json"
        self._version_prefix = ".versions/"
        self._backup_prefix = ".backups/"
        self._deleted_prefix = ".deleted/"

        # Get permission manager (if exists)
        self._permission_manager: Any | None = getattr(storage, "_permission_manager", None)

    def save_with_lifecycle(self, filename: str, data: bytes, tags: dict[str, str] | None = None) -> FileMetadata:
        """Save file and manage lifecycle

        Args:
            filename: File name
            data: File content
            tags: File tags

        Returns:
            File metadata
        """
        # Permission check
        if not self._check_permission(filename, "save"):
            from .volume_permissions import VolumePermissionError

            raise VolumePermissionError(
                f"Permission denied for lifecycle save operation on file: {filename}",
                operation="save",
                volume_type=getattr(self._storage, "_config", {}).get("volume_type", "unknown"),
                dataset_id=self._dataset_id,
            )

        try:
            # 1. Check if old version exists
            metadata_dict = self._load_metadata()
            current_metadata = metadata_dict.get(filename)

            # 2. If old version exists, create version backup
            if current_metadata:
                self._create_version_backup(filename, current_metadata)

            # 3. Calculate file information
            now = datetime.now()
            checksum = self._calculate_checksum(data)
            new_version = (current_metadata["version"] + 1) if current_metadata else 1

            # 4. Save new file
            self._storage.save(filename, data)

            # 5. Create metadata
            created_at = now
            parent_version = None

            if current_metadata:
                # If created_at is string, convert to datetime
                if isinstance(current_metadata["created_at"], str):
                    created_at = datetime.fromisoformat(current_metadata["created_at"])
                else:
                    created_at = current_metadata["created_at"]
                parent_version = current_metadata["version"]

            file_metadata = FileMetadata(
                filename=filename,
                size=len(data),
                created_at=created_at,
                modified_at=now,
                version=new_version,
                status=FileStatus.ACTIVE,
                checksum=checksum,
                tags=tags or {},
                parent_version=parent_version,
            )

            # 6. Update metadata
            metadata_dict[filename] = file_metadata.to_dict()
            self._save_metadata(metadata_dict)

            logger.info("File %s saved with lifecycle management, version %s", filename, new_version)
            return file_metadata

        except Exception:
            logger.exception("Failed to save file with lifecycle")
            raise

    def get_file_metadata(self, filename: str) -> FileMetadata | None:
        """Get file metadata

        Args:
            filename: File name

        Returns:
            File metadata, returns None if not exists
        """
        try:
            metadata_dict = self._load_metadata()
            if filename in metadata_dict:
                return FileMetadata.from_dict(metadata_dict[filename])
            return None
        except Exception:
            logger.exception("Failed to get file metadata for %s", filename)
            return None

    def list_file_versions(self, filename: str) -> list[FileMetadata]:
        """List all versions of a file

        Args:
            filename: File name

        Returns:
            File version list, sorted by version number
        """
        try:
            versions = []

            # Get current version
            current_metadata = self.get_file_metadata(filename)
            if current_metadata:
                versions.append(current_metadata)

            # Get historical versions
            try:
                version_files = self._storage.scan(self._dataset_id or "", files=True)
                for file_path in version_files:
                    if file_path.startswith(f"{self._version_prefix}{filename}.v"):
                        # Parse version number
                        version_str = file_path.split(".v")[-1].split(".")[0]
                        try:
                            _ = int(version_str)
                            # Simplified processing here, should actually read metadata from version file
                            # Temporarily create basic metadata information
                        except ValueError:
                            continue
            except Exception:
                # If cannot scan version files, only return current version
                logger.exception("Failed to scan version files for %s", filename)

            return sorted(versions, key=lambda x: x.version or 0, reverse=True)

        except Exception:
            logger.exception("Failed to list file versions for %s", filename)
            return []

    def restore_version(self, filename: str, version: int) -> bool:
        """Restore file to specified version

        Args:
            filename: File name
            version: Version number to restore

        Returns:
            Whether restore succeeded
        """
        try:
            version_filename = f"{self._version_prefix}{filename}.v{version}"

            # Check if version file exists
            if not self._storage.exists(version_filename):
                logger.warning("Version %s of %s not found", version, filename)
                return False

            # Read version file content
            version_data = self._storage.load_once(version_filename)

            # Save current version as backup
            current_metadata = self.get_file_metadata(filename)
            if current_metadata:
                self._create_version_backup(filename, current_metadata.to_dict())

            # Restore file
            self.save_with_lifecycle(filename, version_data, {"restored_from": str(version)})
            return True

        except Exception:
            logger.exception("Failed to restore %s to version %s", filename, version)
            return False

    def archive_file(self, filename: str) -> bool:
        """Archive file

        Args:
            filename: File name

        Returns:
            Whether archive succeeded
        """
        # Permission check
        if not self._check_permission(filename, "archive"):
            logger.warning("Permission denied for archive operation on file: %s", filename)
            return False

        try:
            # Update file status to archived
            metadata_dict = self._load_metadata()
            if filename not in metadata_dict:
                logger.warning("File %s not found in metadata", filename)
                return False

            metadata_dict[filename]["status"] = FileStatus.ARCHIVED
            metadata_dict[filename]["modified_at"] = datetime.now().isoformat()

            self._save_metadata(metadata_dict)

            logger.info("File %s archived successfully", filename)
            return True

        except Exception:
            logger.exception("Failed to archive file %s", filename)
            return False

    def soft_delete_file(self, filename: str) -> bool:
        """Soft delete file (move to deleted directory)

        Args:
            filename: File name

        Returns:
            Whether delete succeeded
        """
        # Permission check
        if not self._check_permission(filename, "delete"):
            logger.warning("Permission denied for soft delete operation on file: %s", filename)
            return False

        try:
            # Check if file exists
            if not self._storage.exists(filename):
                logger.warning("File %s not found", filename)
                return False

            # Read file content
            file_data = self._storage.load_once(filename)

            # Move to deleted directory
            deleted_filename = f"{self._deleted_prefix}{filename}.{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._storage.save(deleted_filename, file_data)

            # Delete original file
            self._storage.delete(filename)

            # Update metadata
            metadata_dict = self._load_metadata()
            if filename in metadata_dict:
                metadata_dict[filename]["status"] = FileStatus.DELETED
                metadata_dict[filename]["modified_at"] = datetime.now().isoformat()
                self._save_metadata(metadata_dict)

            logger.info("File %s soft deleted successfully", filename)
            return True

        except Exception:
            logger.exception("Failed to soft delete file %s", filename)
            return False

    def cleanup_old_versions(self, max_versions: int = 5, max_age_days: int = 30) -> int:
        """Cleanup old version files

        Args:
            max_versions: Maximum number of versions to keep
            max_age_days: Maximum retention days for version files

        Returns:
            Number of files cleaned
        """
        try:
            cleaned_count = 0

            # Get all version files
            try:
                all_files = self._storage.scan(self._dataset_id or "", files=True)
                version_files = [f for f in all_files if f.startswith(self._version_prefix)]

                # Group by file
                file_versions: dict[str, list[tuple[int, str]]] = {}
                for version_file in version_files:
                    # Parse filename and version
                    parts = version_file[len(self._version_prefix) :].split(".v")
                    if len(parts) >= 2:
                        base_filename = parts[0]
                        version_part = parts[1].split(".")[0]
                        try:
                            version_num = int(version_part)
                            if base_filename not in file_versions:
                                file_versions[base_filename] = []
                            file_versions[base_filename].append((version_num, version_file))
                        except ValueError:
                            continue

                # Cleanup old versions for each file
                for base_filename, versions in file_versions.items():
                    # Sort by version number
                    versions.sort(key=operator.itemgetter(0), reverse=True)

                    # Keep the newest max_versions versions, delete the rest
                    if len(versions) > max_versions:
                        to_delete = versions[max_versions:]
                        for version_num, version_file in to_delete:
                            self._storage.delete(version_file)
                            cleaned_count += 1
                            logger.debug("Cleaned old version: %s", version_file)

                logger.info("Cleaned %d old version files", cleaned_count)

            except Exception as e:
                logger.warning("Could not scan for version files: %s", e)

            return cleaned_count

        except Exception:
            logger.exception("Failed to cleanup old versions")
            return 0

    def get_storage_statistics(self) -> StorageStatisticsDict:
        """Get storage statistics

        Returns:
            Storage statistics dictionary
        """
        try:
            metadata_dict = self._load_metadata()

            stats = StorageStatisticsDict(
                total_files=len(metadata_dict),
                active_files=0,
                archived_files=0,
                deleted_files=0,
                total_size=0,
                versions_count=0,
                oldest_file=None,
                newest_file=None,
            )

            oldest_date = None
            newest_date = None

            for filename, metadata in metadata_dict.items():
                file_meta = FileMetadata.from_dict(metadata)

                # Count file status
                if file_meta.status == FileStatus.ACTIVE:
                    stats["active_files"] = (stats["active_files"] or 0) + 1
                elif file_meta.status == FileStatus.ARCHIVED:
                    stats["archived_files"] = (stats["archived_files"] or 0) + 1
                elif file_meta.status == FileStatus.DELETED:
                    stats["deleted_files"] = (stats["deleted_files"] or 0) + 1

                # Count size
                stats["total_size"] = (stats["total_size"] or 0) + (file_meta.size or 0)

                # Count versions
                stats["versions_count"] = (stats["versions_count"] or 0) + (file_meta.version or 0)

                # Find newest and oldest files
                if oldest_date is None or file_meta.created_at < oldest_date:
                    oldest_date = file_meta.created_at
                    stats["oldest_file"] = filename

                if newest_date is None or file_meta.modified_at > newest_date:
                    newest_date = file_meta.modified_at
                    stats["newest_file"] = filename

            return stats

        except Exception:
            logger.exception("Failed to get storage statistics")
            return StorageStatisticsDict(
                total_files=0,
                active_files=0,
                archived_files=0,
                deleted_files=0,
                total_size=0,
                versions_count=0,
                oldest_file=None,
                newest_file=None,
            )

    def _create_version_backup(self, filename: str, metadata: dict):
        """Create version backup"""
        try:
            # Read current file content
            current_data = self._storage.load_once(filename)

            # Save as version file
            version_filename = f"{self._version_prefix}{filename}.v{metadata['version']}"
            self._storage.save(version_filename, current_data)

            logger.debug("Created version backup: %s", version_filename)

        except Exception as e:
            logger.warning("Failed to create version backup for %s: %s", filename, e)

    def _load_metadata(self) -> dict[str, Any]:
        """Load metadata file"""
        try:
            if self._storage.exists(self._metadata_file):
                metadata_content = self._storage.load_once(self._metadata_file)
                result = _metadata_adapter.validate_json(metadata_content)
                return result or {}
            else:
                return {}
        except Exception as e:
            logger.warning("Failed to load metadata: %s", e)
            return {}

    def _save_metadata(self, metadata_dict: dict):
        """Save metadata file"""
        try:
            metadata_content = json.dumps(metadata_dict, indent=2, ensure_ascii=False)
            self._storage.save(self._metadata_file, metadata_content.encode("utf-8"))
            logger.debug("Metadata saved successfully")
        except Exception:
            logger.exception("Failed to save metadata")
            raise

    def _calculate_checksum(self, data: bytes) -> str:
        """Calculate file checksum"""
        import hashlib

        return hashlib.md5(data).hexdigest()

    def _check_permission(self, filename: str, operation: str) -> bool:
        """Check file operation permission

        Args:
            filename: File name
            operation: Operation type

        Returns:
            True if permission granted, False otherwise
        """
        # If no permission manager, allow by default
        if not self._permission_manager:
            return True

        try:
            # Map operation type to permission
            operation_mapping = {
                "save": "save",
                "load": "load_once",
                "delete": "delete",
                "archive": "delete",  # Archive requires delete permission
                "restore": "save",  # Restore requires write permission
                "cleanup": "delete",  # Cleanup requires delete permission
                "read": "load_once",
                "write": "save",
            }

            mapped_operation = operation_mapping.get(operation, operation)

            # Check permission
            result = self._permission_manager.validate_operation(mapped_operation, self._dataset_id)
            return bool(result)

        except Exception:
            logger.exception("Permission check failed for %s operation %s", filename, operation)
            # Safe default: deny access when permission check fails
            return False

```

### api/extensions/storage/clickzetta_volume/clickzetta_volume_storage.py
```py
"""ClickZetta Volume Storage Implementation

This module provides storage backend using ClickZetta Volume functionality.
Supports Table Volume, User Volume, and External Volume types.
"""

import logging
import os
import tempfile
from collections.abc import Generator
from io import BytesIO
from pathlib import Path

import clickzetta
from pydantic import BaseModel, model_validator

from extensions.storage.base_storage import BaseStorage

from .volume_permissions import VolumePermissionManager, check_volume_permission

logger = logging.getLogger(__name__)


class ClickZettaVolumeConfig(BaseModel):
    """Configuration for ClickZetta Volume storage."""

    username: str = ""
    password: str = ""
    instance: str = ""
    service: str = "api.clickzetta.com"
    workspace: str = "quick_start"
    vcluster: str = "default_ap"
    schema_name: str = "dify"
    volume_type: str = "table"  # table|user|external
    volume_name: str | None = None  # For external volumes
    table_prefix: str = "dataset_"  # Prefix for table volume names
    dify_prefix: str = "dify_km"  # Directory prefix for User Volume
    permission_check: bool = True  # Enable/disable permission checking

    @model_validator(mode="before")
    @classmethod
    def validate_config(cls, values: dict):
        """Validate the configuration values.

        This method will first try to use CLICKZETTA_VOLUME_* environment variables,
        then fall back to CLICKZETTA_* environment variables (for vector DB config).
        """

        # Helper function to get environment variable with fallback
        def get_env_with_fallback(volume_key: str, fallback_key: str, default: str | None = None) -> str:
            # First try CLICKZETTA_VOLUME_* specific config
            volume_value = values.get(volume_key.lower().replace("clickzetta_volume_", ""))
            if volume_value:
                return str(volume_value)

            # Then try environment variables
            volume_env = os.getenv(volume_key)
            if volume_env:
                return volume_env

            # Fall back to existing CLICKZETTA_* config
            fallback_env = os.getenv(fallback_key)
            if fallback_env:
                return fallback_env

            return default or ""

        # Apply environment variables with fallback to existing CLICKZETTA_* config
        values.setdefault("username", get_env_with_fallback("CLICKZETTA_VOLUME_USERNAME", "CLICKZETTA_USERNAME"))
        values.setdefault("password", get_env_with_fallback("CLICKZETTA_VOLUME_PASSWORD", "CLICKZETTA_PASSWORD"))
        values.setdefault("instance", get_env_with_fallback("CLICKZETTA_VOLUME_INSTANCE", "CLICKZETTA_INSTANCE"))
        values.setdefault(
            "service", get_env_with_fallback("CLICKZETTA_VOLUME_SERVICE", "CLICKZETTA_SERVICE", "api.clickzetta.com")
        )
        values.setdefault(
            "workspace", get_env_with_fallback("CLICKZETTA_VOLUME_WORKSPACE", "CLICKZETTA_WORKSPACE", "quick_start")
        )
        values.setdefault(
            "vcluster", get_env_with_fallback("CLICKZETTA_VOLUME_VCLUSTER", "CLICKZETTA_VCLUSTER", "default_ap")
        )
        values.setdefault("schema_name", get_env_with_fallback("CLICKZETTA_VOLUME_SCHEMA", "CLICKZETTA_SCHEMA", "dify"))

        # Volume-specific configurations (no fallback to vector DB config)
        values.setdefault("volume_type", os.getenv("CLICKZETTA_VOLUME_TYPE", "table"))
        values.setdefault("volume_name", os.getenv("CLICKZETTA_VOLUME_NAME"))
        values.setdefault("table_prefix", os.getenv("CLICKZETTA_VOLUME_TABLE_PREFIX", "dataset_"))
        values.setdefault("dify_prefix", os.getenv("CLICKZETTA_VOLUME_DIFY_PREFIX", "dify_km"))
        # Temporarily disable permission check feature, set directly to false
        values.setdefault("permission_check", False)

        # Validate required fields
        if not values.get("username"):
            raise ValueError("CLICKZETTA_VOLUME_USERNAME or CLICKZETTA_USERNAME is required")
        if not values.get("password"):
            raise ValueError("CLICKZETTA_VOLUME_PASSWORD or CLICKZETTA_PASSWORD is required")
        if not values.get("instance"):
            raise ValueError("CLICKZETTA_VOLUME_INSTANCE or CLICKZETTA_INSTANCE is required")

        # Validate volume type
        volume_type = values["volume_type"]
        if volume_type not in ["table", "user", "external"]:
            raise ValueError("CLICKZETTA_VOLUME_TYPE must be one of: table, user, external")

        if volume_type == "external" and not values.get("volume_name"):
            raise ValueError("CLICKZETTA_VOLUME_NAME is required for external volume type")

        return values


class ClickZettaVolumeStorage(BaseStorage):
    """ClickZetta Volume storage implementation."""

    def __init__(self, config: ClickZettaVolumeConfig):
        """Initialize ClickZetta Volume storage.

        Args:
            config: ClickZetta Volume configuration
        """
        self._config = config
        self._connection = None
        self._permission_manager: VolumePermissionManager | None = None
        self._init_connection()
        self._init_permission_manager()

        logger.info("ClickZetta Volume storage initialized with type: %s", config.volume_type)

    def _init_connection(self):
        """Initialize ClickZetta connection."""
        try:
            self._connection = clickzetta.connect(
                username=self._config.username,
                password=self._config.password,
                instance=self._config.instance,
                service=self._config.service,
                workspace=self._config.workspace,
                vcluster=self._config.vcluster,
                schema=self._config.schema_name,
            )
            logger.debug("ClickZetta connection established")
        except Exception:
            logger.exception("Failed to connect to ClickZetta")
            raise

    def _init_permission_manager(self):
        """Initialize permission manager."""
        try:
            self._permission_manager = VolumePermissionManager(
                self._connection, self._config.volume_type, self._config.volume_name
            )
            logger.debug("Permission manager initialized")
        except Exception:
            logger.exception("Failed to initialize permission manager")
            raise

    def _get_volume_path(self, filename: str, dataset_id: str | None = None) -> str:
        """Get the appropriate volume path based on volume type."""
        if self._config.volume_type == "user":
            # Add dify prefix for User Volume to organize files
            return f"{self._config.dify_prefix}/{filename}"
        elif self._config.volume_type == "table":
            # Check if this should use User Volume (special directories)
            if dataset_id in ["upload_files", "temp", "cache", "tools", "website_files", "privkeys"]:
                # Use User Volume with dify prefix for special directories
                return f"{self._config.dify_prefix}/{filename}"

            if dataset_id:
                return f"{self._config.table_prefix}{dataset_id}/{filename}"
            else:
                # Extract dataset_id from filename if not provided
                # Format: dataset_id/filename
                if "/" in filename:
                    return filename
                else:
                    raise ValueError("dataset_id is required for table volume or filename must include dataset_id/")
        elif self._config.volume_type == "external":
            return filename
        else:
            raise ValueError(f"Unsupported volume type: {self._config.volume_type}")

    def _get_volume_sql_prefix(self, dataset_id: str | None = None) -> str:
        """Get SQL prefix for volume operations."""
        if self._config.volume_type == "user":
            return "USER VOLUME"
        elif self._config.volume_type == "table":
            # For Dify's current file storage pattern, most files are stored in
            # paths like "upload_files/tenant_id/uuid.ext", "tools/tenant_id/uuid.ext"
            # These should use USER VOLUME for better compatibility
            if dataset_id in ["upload_files", "temp", "cache", "tools", "website_files", "privkeys"]:
                return "USER VOLUME"

            # Only use TABLE VOLUME for actual dataset-specific paths
            # like "dataset_12345/file.pdf" or paths with dataset_ prefix
            if dataset_id:
                table_name = f"{self._config.table_prefix}{dataset_id}"
            else:
                # Default table name for generic operations
                table_name = "default_dataset"
            return f"TABLE VOLUME {table_name}"
        elif self._config.volume_type == "external":
            return f"VOLUME {self._config.volume_name}"
        else:
            raise ValueError(f"Unsupported volume type: {self._config.volume_type}")

    def _execute_sql(self, sql: str, fetch: bool = False):
        """Execute SQL command."""
        try:
            if self._connection is None:
                raise RuntimeError("Connection not initialized")
            with self._connection.cursor() as cursor:
                cursor.execute(sql)
                if fetch:
                    return cursor.fetchall()
                return None
        except Exception:
            logger.exception("SQL execution failed: %s", sql)
            raise

    def _ensure_table_volume_exists(self, dataset_id: str):
        """Ensure table volume exists for the given dataset_id."""
        if self._config.volume_type != "table" or not dataset_id:
            return

        # Skip for upload_files and other special directories that use USER VOLUME
        if dataset_id in ["upload_files", "temp", "cache", "tools", "website_files", "privkeys"]:
            return

        table_name = f"{self._config.table_prefix}{dataset_id}"

        try:
            # Check if table exists
            check_sql = f"SHOW TABLES LIKE '{table_name}'"
            result = self._execute_sql(check_sql, fetch=True)

            if not result:
                # Create table with volume
                create_sql = f"""
                CREATE TABLE {table_name} (
                    id INT PRIMARY KEY AUTO_INCREMENT,
                    filename VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                    INDEX idx_filename (filename)
                ) WITH VOLUME
                """
                self._execute_sql(create_sql)
                logger.info("Created table volume: %s", table_name)

        except Exception as e:
            logger.warning("Failed to create table volume %s: %s", table_name, e)
            # Don't raise exception, let the operation continue
            # The table might exist but not be visible due to permissions

    def save(self, filename: str, data: bytes):
        """Save data to ClickZetta Volume.

        Args:
            filename: File path in volume
            data: File content as bytes
        """
        # Extract dataset_id from filename if present
        dataset_id = None
        if "/" in filename and self._config.volume_type == "table":
            parts = filename.split("/", 1)
            if parts[0].startswith(self._config.table_prefix):
                dataset_id = parts[0][len(self._config.table_prefix) :]
                filename = parts[1]
            else:
                dataset_id = parts[0]
                filename = parts[1]

        # Ensure table volume exists (for table volumes)
        if dataset_id:
            self._ensure_table_volume_exists(dataset_id)

        # Check permissions (if enabled)
        if self._config.permission_check:
            # Skip permission check for special directories that use USER VOLUME
            if dataset_id not in ["upload_files", "temp", "cache", "tools", "website_files", "privkeys"]:
                if self._permission_manager is not None:
                    check_volume_permission(self._permission_manager, "save", dataset_id)

        # Write data to temporary file
        with tempfile.NamedTemporaryFile(delete=False) as temp_file:
            temp_file.write(data)
            temp_file_path = temp_file.name

        try:
            # Upload to volume
            volume_prefix = self._get_volume_sql_prefix(dataset_id)

            # Get the actual volume path (may include dify_km prefix)
            volume_path = self._get_volume_path(filename, dataset_id)

            # For User Volume, use the full path with dify_km prefix
            if volume_prefix == "USER VOLUME":
                sql = f"PUT '{temp_file_path}' TO {volume_prefix} FILE '{volume_path}'"
            else:
                sql = f"PUT '{temp_file_path}' TO {volume_prefix} FILE '{filename}'"

            self._execute_sql(sql)
            logger.debug("File %s saved to ClickZetta Volume at path %s", filename, volume_path)
        finally:
            # Clean up temporary file
            Path(temp_file_path).unlink(missing_ok=True)

    def load_once(self, filename: str) -> bytes:
        """Load file content from ClickZetta Volume.

        Args:
            filename: File path in volume

        Returns:
            File content as bytes
        """
        # Extract dataset_id from filename if present
        dataset_id = None
        if "/" in filename and self._config.volume_type == "table":
            parts = filename.split("/", 1)
            if parts[0].startswith(self._config.table_prefix):
                dataset_id = parts[0][len(self._config.table_prefix) :]
                filename = parts[1]
            else:
                dataset_id = parts[0]
                filename = parts[1]

        # Check permissions (if enabled)
        if self._config.permission_check:
            # Skip permission check for special directories that use USER VOLUME
            if dataset_id not in ["upload_files", "temp", "cache", "tools", "website_files", "privkeys"]:
                if self._permission_manager is not None:
                    check_volume_permission(self._permission_manager, "load_once", dataset_id)

        # Download to temporary directory
        with tempfile.TemporaryDirectory() as temp_dir:
            volume_prefix = self._get_volume_sql_prefix(dataset_id)

            # Get the actual volume path (may include dify_km prefix)
            volume_path = self._get_volume_path(filename, dataset_id)

            # For User Volume, use the full path with dify_km prefix
            if volume_prefix == "USER VOLUME":
                sql = f"GET {volume_prefix} FILE '{volume_path}' TO '{temp_dir}'"
            else:
                sql = f"GET {volume_prefix} FILE '{filename}' TO '{temp_dir}'"

            self._execute_sql(sql)

            # Find the downloaded file (may be in subdirectories)
            downloaded_file = None
            for root, _, files in os.walk(temp_dir):
                for file in files:
                    if file == filename or file == os.path.basename(filename):
                        downloaded_file = Path(root) / file
                        break
                if downloaded_file:
                    break

            if not downloaded_file or not downloaded_file.exists():
                raise FileNotFoundError(f"Downloaded file not found: {filename}")

            content = downloaded_file.read_bytes()

            logger.debug("File %s loaded from ClickZetta Volume", filename)
            return content

    def load_stream(self, filename: str) -> Generator:
        """Load file as stream from ClickZetta Volume.

        Args:
            filename: File path in volume

        Yields:
            File content chunks
        """
        content = self.load_once(filename)
        batch_size = 4096
        stream = BytesIO(content)

        while chunk := stream.read(batch_size):
            yield chunk

        logger.debug("File %s loaded as stream from ClickZetta Volume", filename)

    def download(self, filename: str, target_filepath: str):
        """Download file from ClickZetta Volume to local path.

        Args:
            filename: File path in volume
            target_filepath: Local target file path
        """
        content = self.load_once(filename)

        Path(target_filepath).write_bytes(content)

        logger.debug("File %s downloaded from ClickZetta Volume to %s", filename, target_filepath)

    def exists(self, filename: str) -> bool:
        """Check if file exists in ClickZetta Volume.

        Args:
            filename: File path in volume

        Returns:
            True if file exists, False otherwise
        """
        try:
            # Extract dataset_id from filename if present
            dataset_id = None
            if "/" in filename and self._config.volume_type == "table":
                parts = filename.split("/", 1)
                if parts[0].startswith(self._config.table_prefix):
                    dataset_id = parts[0][len(self._config.table_prefix) :]
                    filename = parts[1]
                else:
                    dataset_id = parts[0]
                    filename = parts[1]

            volume_prefix = self._get_volume_sql_prefix(dataset_id)

            # Get the actual volume path (may include dify_km prefix)
            volume_path = self._get_volume_path(filename, dataset_id)

            # For User Volume, use the full path with dify_km prefix
            if volume_prefix == "USER VOLUME":
                sql = f"LIST {volume_prefix} REGEXP = '^{volume_path}$'"
            else:
                sql = f"LIST {volume_prefix} REGEXP = '^{filename}$'"

            rows = self._execute_sql(sql, fetch=True)

            exists = len(rows) > 0 if rows else False
            logger.debug("File %s exists check: %s", filename, exists)
            return exists
        except Exception as e:
            logger.warning("Error checking file existence for %s: %s", filename, e)
            return False

    def delete(self, filename: str):
        """Delete file from ClickZetta Volume.

        Args:
            filename: File path in volume
        """
        if not self.exists(filename):
            logger.debug("File %s not found, skip delete", filename)
            return

        # Extract dataset_id from filename if present
        dataset_id = None
        if "/" in filename and self._config.volume_type == "table":
            parts = filename.split("/", 1)
            if parts[0].startswith(self._config.table_prefix):
                dataset_id = parts[0][len(self._config.table_prefix) :]
                filename = parts[1]
            else:
                dataset_id = parts[0]
                filename = parts[1]

        volume_prefix = self._get_volume_sql_prefix(dataset_id)

        # Get the actual volume path (may include dify_km prefix)
        volume_path = self._get_volume_path(filename, dataset_id)

        # For User Volume, use the full path with dify_km prefix
        if volume_prefix == "USER VOLUME":
            sql = f"REMOVE {volume_prefix} FILE '{volume_path}'"
        else:
            sql = f"REMOVE {volume_prefix} FILE '{filename}'"

        self._execute_sql(sql)

        logger.debug("File %s deleted from ClickZetta Volume", filename)

    def scan(self, path: str, files: bool = True, directories: bool = False) -> list[str]:
        """Scan files and directories in ClickZetta Volume.

        Args:
            path: Path to scan (dataset_id for table volumes)
            files: Include files in results
            directories: Include directories in results

        Returns:
            List of file/directory paths
        """
        try:
            # For table volumes, path is treated as dataset_id
            dataset_id = None
            if self._config.volume_type == "table":
                dataset_id = path
                path = ""  # Root of the table volume

            volume_prefix = self._get_volume_sql_prefix(dataset_id)

            # For User Volume, add dify prefix to path
            if volume_prefix == "USER VOLUME":
                if path:
                    scan_path = f"{self._config.dify_prefix}/{path}"
                    sql = f"LIST {volume_prefix} SUBDIRECTORY '{scan_path}'"
                else:
                    sql = f"LIST {volume_prefix} SUBDIRECTORY '{self._config.dify_prefix}'"
            else:
                if path:
                    sql = f"LIST {volume_prefix} SUBDIRECTORY '{path}'"
                else:
                    sql = f"LIST {volume_prefix}"

            rows = self._execute_sql(sql, fetch=True)

            result = []
            if rows:
                for row in rows:
                    file_path = row[0]  # relative_path column

                    # For User Volume, remove dify prefix from results
                    dify_prefix_with_slash = f"{self._config.dify_prefix}/"
                    if volume_prefix == "USER VOLUME" and file_path.startswith(dify_prefix_with_slash):
                        file_path = file_path[len(dify_prefix_with_slash) :]  # Remove prefix

                    if files and not file_path.endswith("/") or directories and file_path.endswith("/"):
                        result.append(file_path)

            logger.debug("Scanned %d items in path %s", len(result), path)
            return result

        except Exception:
            logger.exception("Error scanning path %s", path)
            return []

```

### api/extensions/storage/storage_type.py
```py
from enum import StrEnum


class StorageType(StrEnum):
    ALIYUN_OSS = "aliyun-oss"
    AZURE_BLOB = "azure-blob"
    BAIDU_OBS = "baidu-obs"
    CLICKZETTA_VOLUME = "clickzetta-volume"
    GOOGLE_STORAGE = "google-storage"
    HUAWEI_OBS = "huawei-obs"
    LOCAL = "local"
    OCI_STORAGE = "oci-storage"
    OPENDAL = "opendal"
    S3 = "s3"
    TENCENT_COS = "tencent-cos"
    VOLCENGINE_TOS = "volcengine-tos"
    SUPABASE = "supabase"

```

### api/extensions/storage/baidu_obs_storage.py
```py
import base64
import hashlib
from collections.abc import Generator

from baidubce.auth.bce_credentials import BceCredentials
from baidubce.bce_client_configuration import BceClientConfiguration
from baidubce.services.bos.bos_client import BosClient

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class BaiduObsStorage(BaseStorage):
    """Implementation for Baidu OBS storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.BAIDU_OBS_BUCKET_NAME
        client_config = BceClientConfiguration(
            credentials=BceCredentials(
                access_key_id=dify_config.BAIDU_OBS_ACCESS_KEY,
                secret_access_key=dify_config.BAIDU_OBS_SECRET_KEY,
            ),
            endpoint=dify_config.BAIDU_OBS_ENDPOINT,
        )

        self.client = BosClient(config=client_config)

    def save(self, filename, data):
        md5 = hashlib.md5()
        md5.update(data)
        content_md5 = base64.standard_b64encode(md5.digest())
        self.client.put_object(
            bucket_name=self.bucket_name, key=filename, data=data, content_length=len(data), content_md5=content_md5
        )

    def load_once(self, filename: str) -> bytes:
        response = self.client.get_object(bucket_name=self.bucket_name, key=filename)
        data: bytes = response.data.read()
        return data

    def load_stream(self, filename: str) -> Generator:
        response = self.client.get_object(bucket_name=self.bucket_name, key=filename).data
        while chunk := response.read(4096):
            yield chunk

    def download(self, filename, target_filepath):
        self.client.get_object_to_file(bucket_name=self.bucket_name, key=filename, file_name=target_filepath)

    def exists(self, filename):
        res = self.client.get_object_meta_data(bucket_name=self.bucket_name, key=filename)
        if res is None:
            return False
        return True

    def delete(self, filename: str):
        self.client.delete_object(bucket_name=self.bucket_name, key=filename)

```

### api/extensions/storage/huawei_obs_storage.py
```py
from collections.abc import Generator

from obs import ObsClient

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class HuaweiObsStorage(BaseStorage):
    """Implementation for Huawei OBS storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.HUAWEI_OBS_BUCKET_NAME
        self.client = ObsClient(
            access_key_id=dify_config.HUAWEI_OBS_ACCESS_KEY,
            secret_access_key=dify_config.HUAWEI_OBS_SECRET_KEY,
            server=dify_config.HUAWEI_OBS_SERVER,
            path_style=dify_config.HUAWEI_OBS_PATH_STYLE,
        )

    def save(self, filename, data):
        self.client.putObject(bucketName=self.bucket_name, objectKey=filename, content=data)

    def load_once(self, filename: str) -> bytes:
        data: bytes = self.client.getObject(bucketName=self.bucket_name, objectKey=filename)["body"].response.read()
        return data

    def load_stream(self, filename: str) -> Generator:
        response = self.client.getObject(bucketName=self.bucket_name, objectKey=filename)["body"].response
        while chunk := response.read(4096):
            yield chunk

    def download(self, filename, target_filepath):
        self.client.getObject(bucketName=self.bucket_name, objectKey=filename, downloadPath=target_filepath)

    def exists(self, filename):
        res = self._get_meta(filename)
        if res is None:
            return False
        return True

    def delete(self, filename: str):
        self.client.deleteObject(bucketName=self.bucket_name, objectKey=filename)

    def _get_meta(self, filename):
        res = self.client.getObjectMetadata(bucketName=self.bucket_name, objectKey=filename)
        if res and res.status and res.status < 300:
            return res
        else:
            return None

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-004.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
