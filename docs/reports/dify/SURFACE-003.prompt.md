You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-003
- **Kind**: endpoint
- **Identifier**: POST /console/api/files/upload, POST /v1/files/upload, POST /api/files/upload
- **Description**: File upload endpoints across all API layers accepting multipart form data. Requires review for file type validation bypass, path traversal, and malicious file content
- **Locations**: api/controllers/console/files.py, api/controllers/service_api/app/file.py, api/controllers/web/files.py, api/factories/file_factory/validation.py

## Source Code

### api/controllers/console/files.py
```py
from typing import Literal

from flask import request
from flask_restx import Resource
from werkzeug.exceptions import Forbidden

import services
from configs import dify_config
from constants import DOCUMENT_EXTENSIONS
from controllers.common.errors import (
    BlockedFileExtensionError,
    FilenameNotExistsError,
    FileTooLargeError,
    NoFileUploadedError,
    TooManyFilesError,
    UnsupportedFileTypeError,
)
from controllers.common.schema import register_schema_models
from controllers.console.wraps import (
    account_initialization_required,
    cloud_edition_billing_resource_check,
    setup_required,
)
from extensions.ext_database import db
from fields.file_fields import FileResponse, UploadConfig
from libs.login import current_account_with_tenant, login_required
from services.file_service import FileService

from . import console_ns

register_schema_models(console_ns, UploadConfig, FileResponse)

PREVIEW_WORDS_LIMIT = 3000


@console_ns.route("/files/upload")
class FileApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @console_ns.response(200, "Success", console_ns.models[UploadConfig.__name__])
    def get(self):
        config = UploadConfig(
            file_size_limit=dify_config.UPLOAD_FILE_SIZE_LIMIT,
            batch_count_limit=dify_config.UPLOAD_FILE_BATCH_LIMIT,
            file_upload_limit=dify_config.BATCH_UPLOAD_LIMIT,
            image_file_size_limit=dify_config.UPLOAD_IMAGE_FILE_SIZE_LIMIT,
            video_file_size_limit=dify_config.UPLOAD_VIDEO_FILE_SIZE_LIMIT,
            audio_file_size_limit=dify_config.UPLOAD_AUDIO_FILE_SIZE_LIMIT,
            workflow_file_upload_limit=dify_config.WORKFLOW_FILE_UPLOAD_LIMIT,
            image_file_batch_limit=dify_config.IMAGE_FILE_BATCH_LIMIT,
            single_chunk_attachment_limit=dify_config.SINGLE_CHUNK_ATTACHMENT_LIMIT,
            attachment_image_file_size_limit=dify_config.ATTACHMENT_IMAGE_FILE_SIZE_LIMIT,
        )
        return config.model_dump(mode="json"), 200

    @setup_required
    @login_required
    @account_initialization_required
    @cloud_edition_billing_resource_check("documents")
    @console_ns.response(201, "File uploaded successfully", console_ns.models[FileResponse.__name__])
    def post(self):
        current_user, _ = current_account_with_tenant()
        source_str = request.form.get("source")
        source: Literal["datasets"] | None = "datasets" if source_str == "datasets" else None

        if "file" not in request.files:
            raise NoFileUploadedError()

        if len(request.files) > 1:
            raise TooManyFilesError()
        file = request.files["file"]

        if not file.filename:
            raise FilenameNotExistsError
        if source == "datasets" and not current_user.is_dataset_editor:
            raise Forbidden()

        if source not in ("datasets", None):
            source = None

        try:
            upload_file = FileService(db.engine).upload_file(
                filename=file.filename,
                content=file.read(),
                mimetype=file.mimetype,
                user=current_user,
                source=source,
            )
        except services.errors.file.FileTooLargeError as file_too_large_error:
            raise FileTooLargeError(file_too_large_error.description)
        except services.errors.file.UnsupportedFileTypeError:
            raise UnsupportedFileTypeError()
        except services.errors.file.BlockedFileExtensionError as blocked_extension_error:
            raise BlockedFileExtensionError(blocked_extension_error.description)

        response = FileResponse.model_validate(upload_file, from_attributes=True)
        return response.model_dump(mode="json"), 201


@console_ns.route("/files/<uuid:file_id>/preview")
class FilePreviewApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, file_id):
        file_id = str(file_id)
        text = FileService(db.engine).get_file_preview(file_id)
        return {"content": text}


@console_ns.route("/files/support-type")
class FileSupportTypeApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        return {"allowed_extensions": list(DOCUMENT_EXTENSIONS)}

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-003.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
