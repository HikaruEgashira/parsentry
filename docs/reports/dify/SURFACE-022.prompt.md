You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-022
- **Kind**: endpoint
- **Identifier**: POST /console/api/apps/<app_id>/audio-to-text, POST /v1/audio-to-text
- **Description**: Audio file upload and transcription endpoints. Malicious audio files could exploit transcription service vulnerabilities or cause resource exhaustion
- **Locations**: api/controllers/console/app/audio.py, api/controllers/service_api/app/audio.py, api/controllers/web/audio.py

## Source Code

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-022.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
