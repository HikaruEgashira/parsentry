You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-007
- **Kind**: public_api
- **Identifier**: Code Executor (Python3, JavaScript, Jinja2)
- **Description**: Executes user-provided Python, JavaScript, and Jinja2 code via AST transformation and sandboxing. Sandbox escape, AST transformation bypass, and Jinja2 SSTI are critical risks
- **Locations**: api/core/helper/code_executor/code_executor.py, api/core/helper/code_executor/python3/python3_code_provider.py, api/core/helper/code_executor/python3/python3_transformer.py, api/core/helper/code_executor/javascript/javascript_code_provider.py, api/core/helper/code_executor/jinja2/jinja2_formatter.py, api/core/helper/code_executor/template_transformer.py

## Source Code

### api/core/helper/code_executor/code_executor.py
```py
import logging
from collections.abc import Mapping
from threading import Lock
from typing import Any

import httpx
from graphon.nodes.code.entities import CodeLanguage
from pydantic import BaseModel
from yarl import URL

from configs import dify_config
from core.helper.code_executor.javascript.javascript_transformer import NodeJsTemplateTransformer
from core.helper.code_executor.jinja2.jinja2_transformer import Jinja2TemplateTransformer
from core.helper.code_executor.python3.python3_transformer import Python3TemplateTransformer
from core.helper.code_executor.template_transformer import TemplateTransformer
from core.helper.http_client_pooling import get_pooled_http_client

logger = logging.getLogger(__name__)
code_execution_endpoint_url = URL(str(dify_config.CODE_EXECUTION_ENDPOINT))
CODE_EXECUTION_SSL_VERIFY = dify_config.CODE_EXECUTION_SSL_VERIFY
_CODE_EXECUTOR_CLIENT_LIMITS = httpx.Limits(
    max_connections=dify_config.CODE_EXECUTION_POOL_MAX_CONNECTIONS,
    max_keepalive_connections=dify_config.CODE_EXECUTION_POOL_MAX_KEEPALIVE_CONNECTIONS,
    keepalive_expiry=dify_config.CODE_EXECUTION_POOL_KEEPALIVE_EXPIRY,
)
_CODE_EXECUTOR_CLIENT_KEY = "code_executor:http_client"


class CodeExecutionError(Exception):
    pass


class CodeExecutionResponse(BaseModel):
    class Data(BaseModel):
        stdout: str | None = None
        error: str | None = None

    code: int
    message: str
    data: Data


def _build_code_executor_client() -> httpx.Client:
    return httpx.Client(
        verify=CODE_EXECUTION_SSL_VERIFY,
        limits=_CODE_EXECUTOR_CLIENT_LIMITS,
    )


class CodeExecutor:
    dependencies_cache: dict[str, str] = {}
    dependencies_cache_lock = Lock()

    code_template_transformers: dict[CodeLanguage, type[TemplateTransformer]] = {
        CodeLanguage.PYTHON3: Python3TemplateTransformer,
        CodeLanguage.JINJA2: Jinja2TemplateTransformer,
        CodeLanguage.JAVASCRIPT: NodeJsTemplateTransformer,
    }

    code_language_to_running_language = {
        CodeLanguage.JAVASCRIPT: "nodejs",
        CodeLanguage.JINJA2: CodeLanguage.PYTHON3,
        CodeLanguage.PYTHON3: CodeLanguage.PYTHON3,
    }

    supported_dependencies_languages: set[CodeLanguage] = {CodeLanguage.PYTHON3}

    @classmethod
    def execute_code(cls, language: CodeLanguage, preload: str, code: str) -> str:
        """
        Execute code
        :param language: code language
        :param preload: the preload script
        :param code: code
        :return:
        """
        url = code_execution_endpoint_url / "v1" / "sandbox" / "run"

        headers = {"X-Api-Key": dify_config.CODE_EXECUTION_API_KEY}

        data = {
            "language": cls.code_language_to_running_language.get(language),
            "code": code,
            "preload": preload,
            "enable_network": True,
        }

        timeout = httpx.Timeout(
            connect=dify_config.CODE_EXECUTION_CONNECT_TIMEOUT,
            read=dify_config.CODE_EXECUTION_READ_TIMEOUT,
            write=dify_config.CODE_EXECUTION_WRITE_TIMEOUT,
            pool=None,
        )

        client = get_pooled_http_client(_CODE_EXECUTOR_CLIENT_KEY, _build_code_executor_client)

        try:
            response = client.post(
                str(url),
                json=data,
                headers=headers,
                timeout=timeout,
            )
            if response.status_code == 503:
                raise CodeExecutionError("Code execution service is unavailable")
            elif response.status_code != 200:
                raise Exception(
                    f"Failed to execute code, got status code {response.status_code},"
                    f" please check if the sandbox service is running"
                )
        except CodeExecutionError as e:
            raise e
        except Exception as e:
            raise CodeExecutionError(
                "Failed to execute code, which is likely a network issue,"
                " please check if the sandbox service is running."
                f" ( Error: {str(e)} )"
            )

        try:
            response_data = response.json()
        except Exception as e:
            raise CodeExecutionError("Failed to parse response") from e

        if (code := response_data.get("code")) != 0:
            raise CodeExecutionError(f"Got error code: {code}. Got error msg: {response_data.get('message')}")

        response_code = CodeExecutionResponse.model_validate(response_data)

        if response_code.data.error:
            raise CodeExecutionError(response_code.data.error)

        return response_code.data.stdout or ""

    @classmethod
    def execute_workflow_code_template(cls, language: CodeLanguage, code: str, inputs: Mapping[str, Any]):
        """
        Execute code
        :param language: code language
        :param code: code
        :param inputs: inputs
        :return:
        """
        template_transformer = cls.code_template_transformers.get(language)
        if not template_transformer:
            raise CodeExecutionError(f"Unsupported language {language}")

        runner, preload = template_transformer.transform_caller(code, inputs)
        response = cls.execute_code(language, preload, runner)
        return template_transformer.transform_response(response)

```

### api/core/helper/code_executor/python3/python3_code_provider.py
```py
from textwrap import dedent

from core.helper.code_executor.code_executor import CodeLanguage
from core.helper.code_executor.code_node_provider import CodeNodeProvider


class Python3CodeProvider(CodeNodeProvider):
    @staticmethod
    def get_language() -> str:
        return CodeLanguage.PYTHON3

    @classmethod
    def get_default_code(cls) -> str:
        return dedent(
            """
            def main(arg1: str, arg2: str):
                return {
                    "result": arg1 + arg2,
                }
            """
        )

```

### api/core/helper/code_executor/python3/python3_transformer.py
```py
from textwrap import dedent

from core.helper.code_executor.template_transformer import TemplateTransformer


class Python3TemplateTransformer(TemplateTransformer):
    @classmethod
    def get_runner_script(cls) -> str:
        runner_script = dedent(f"""            {cls._code_placeholder}

            import json
            from base64 import b64decode

            # decode and prepare input dict
            inputs_obj = json.loads(b64decode('{cls._inputs_placeholder}').decode('utf-8'))

            # execute main function
            output_obj = main(**inputs_obj)

            # convert output to json and print
            output_json = json.dumps(output_obj, indent=4)
            result = f'''<<RESULT>>{{output_json}}<<RESULT>>'''
            print(result)
            """)
        return runner_script

```

### api/core/helper/code_executor/javascript/javascript_code_provider.py
```py
from textwrap import dedent

from core.helper.code_executor.code_executor import CodeLanguage
from core.helper.code_executor.code_node_provider import CodeNodeProvider


class JavascriptCodeProvider(CodeNodeProvider):
    @staticmethod
    def get_language() -> str:
        return CodeLanguage.JAVASCRIPT

    @classmethod
    def get_default_code(cls) -> str:
        return dedent(
            """
            function main({arg1, arg2}) {
                return {
                    result: arg1 + arg2
                }
            }
            """
        )

```

### api/core/helper/code_executor/jinja2/jinja2_formatter.py
```py
from collections.abc import Mapping

from core.helper.code_executor.code_executor import CodeExecutor, CodeLanguage


class Jinja2Formatter:
    @classmethod
    def format(cls, template: str, inputs: Mapping[str, str]) -> str:
        """
        Format template
        :param template: template
        :param inputs: inputs
        :return:
        """
        result = CodeExecutor.execute_workflow_code_template(language=CodeLanguage.JINJA2, code=template, inputs=inputs)
        return str(result.get("result", ""))

```

### api/core/helper/code_executor/template_transformer.py
```py
import json
import re
from abc import ABC, abstractmethod
from base64 import b64encode
from collections.abc import Mapping
from typing import Any

from graphon.variables.utils import dumps_with_segments


class TemplateTransformer(ABC):
    _code_placeholder: str = "{{code}}"
    _inputs_placeholder: str = "{{inputs}}"
    _result_tag: str = "<<RESULT>>"

    @classmethod
    def serialize_code(cls, code: str) -> str:
        """
        Serialize template code to base64 to safely embed in generated script.
        This prevents issues with special characters like quotes breaking the script.
        """
        code_bytes = code.encode("utf-8")
        return b64encode(code_bytes).decode("utf-8")

    @classmethod
    def transform_caller(cls, code: str, inputs: Mapping[str, Any]) -> tuple[str, str]:
        """
        Transform code to python runner
        :param code: code
        :param inputs: inputs
        :return: runner, preload
        """
        runner_script = cls.assemble_runner_script(code, inputs)
        preload_script = cls.get_preload_script()

        return runner_script, preload_script

    @classmethod
    def extract_result_str_from_response(cls, response: str):
        result = re.search(rf"{cls._result_tag}(.*){cls._result_tag}", response, re.DOTALL)
        if not result:
            raise ValueError(f"Failed to parse result: no result tag found in response. Response: {response[:200]}...")
        return result.group(1)

    @classmethod
    def transform_response(cls, response: str) -> Mapping[str, Any]:
        """
        Transform response to dict
        :param response: response
        :return:
        """

        try:
            result_str = cls.extract_result_str_from_response(response)
            result = json.loads(result_str)
        except json.JSONDecodeError as e:
            raise ValueError(f"Failed to parse JSON response: {str(e)}.")
        except ValueError as e:
            # Re-raise ValueError from extract_result_str_from_response
            raise e
        except Exception as e:
            raise ValueError(f"Unexpected error during response transformation: {str(e)}")

        if not isinstance(result, dict):
            raise ValueError(f"Result must be a dict, got {type(result).__name__}")
        if not all(isinstance(k, str) for k in result):
            raise ValueError("Result keys must be strings")

        # Post-process the result to convert scientific notation strings back to numbers
        result = cls._post_process_result(result)
        return result

    @classmethod
    def _post_process_result(cls, result: dict[Any, Any]) -> dict[Any, Any]:
        """
        Post-process the result to convert scientific notation strings back to numbers
        """

        def convert_scientific_notation(value: Any) -> Any:
            if isinstance(value, str):
                # Check if the string looks like scientific notation
                if re.match(r"^-?\d+\.?\d*e[+-]\d+$", value, re.IGNORECASE):
                    try:
                        return float(value)
                    except ValueError:
                        pass
            elif isinstance(value, dict):
                return {k: convert_scientific_notation(v) for k, v in value.items()}
            elif isinstance(value, list):
                return [convert_scientific_notation(v) for v in value]
            return value

        return convert_scientific_notation(result)

    @classmethod
    @abstractmethod
    def get_runner_script(cls) -> str:
        """
        Get runner script
        """
        pass

    @classmethod
    def serialize_inputs(cls, inputs: Mapping[str, Any]) -> str:
        inputs_json_str = dumps_with_segments(inputs, ensure_ascii=False).encode()
        input_base64_encoded = b64encode(inputs_json_str).decode("utf-8")
        return input_base64_encoded

    @classmethod
    def assemble_runner_script(cls, code: str, inputs: Mapping[str, Any]) -> str:
        # assemble runner script
        script = cls.get_runner_script()
        script = script.replace(cls._code_placeholder, code)
        inputs_str = cls.serialize_inputs(inputs)
        script = script.replace(cls._inputs_placeholder, inputs_str)
        return script

    @classmethod
    def get_preload_script(cls) -> str:
        """
        Get preload script
        """
        return ""

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-007.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
