You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-022
- **Kind**: external_call
- **Identifier**: MCP Client SSE/HTTP Transport
- **Description**: HTTP-based MCP client connecting to external tool servers via SSE transport; URL-based routing with SSRF-protected connections
- **Locations**: api/core/mcp/mcp_client.py, api/core/mcp/client/sse_client.py

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

### api/core/mcp/mcp_client.py
```py
import logging
from collections.abc import Callable
from contextlib import AbstractContextManager, ExitStack
from types import TracebackType
from typing import Any
from urllib.parse import urlparse

from core.mcp.client.sse_client import sse_client
from core.mcp.client.streamable_client import streamablehttp_client
from core.mcp.error import MCPConnectionError
from core.mcp.session.client_session import ClientSession
from core.mcp.types import CallToolResult, Tool

logger = logging.getLogger(__name__)


class MCPClient:
    def __init__(
        self,
        server_url: str,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
        sse_read_timeout: float | None = None,
    ):
        self.server_url = server_url
        self.headers = headers or {}
        self.timeout = timeout
        self.sse_read_timeout = sse_read_timeout

        # Initialize session and client objects
        self._session: ClientSession | None = None
        self._exit_stack = ExitStack()
        self._initialized = False

    def __enter__(self):
        self._initialize()
        self._initialized = True
        return self

    def __exit__(self, exc_type: type | None, exc_value: BaseException | None, traceback: TracebackType | None):
        self.cleanup()

    def _initialize(
        self,
    ):
        """Initialize the client with fallback to SSE if streamable connection fails"""
        connection_methods: dict[str, Callable[..., AbstractContextManager[Any]]] = {
            "mcp": streamablehttp_client,
            "sse": sse_client,
        }

        parsed_url = urlparse(self.server_url)
        path = parsed_url.path or ""
        method_name = path.rstrip("/").split("/")[-1] if path else ""
        if method_name in connection_methods:
            client_factory = connection_methods[method_name]
            self.connect_server(client_factory, method_name)
        else:
            try:
                logger.debug("Not supported method %s found in URL path, trying default 'mcp' method.", method_name)
                self.connect_server(sse_client, "sse")
            except (MCPConnectionError, ValueError):
                logger.debug("MCP connection failed with 'sse', falling back to 'mcp' method.")
                self.connect_server(streamablehttp_client, "mcp")

    def connect_server(self, client_factory: Callable[..., AbstractContextManager[Any]], method_name: str) -> None:
        """
        Connect to the MCP server using streamable http or sse.
        Default to streamable http.
        Args:
            client_factory: The client factory to use(streamablehttp_client or sse_client).
            method_name: The method name to use(mcp or sse).
        """
        streams_context = client_factory(
            url=self.server_url,
            headers=self.headers,
            timeout=self.timeout,
            sse_read_timeout=self.sse_read_timeout,
        )

        # Use exit_stack to manage context managers properly
        if method_name == "mcp":
            read_stream, write_stream, _ = self._exit_stack.enter_context(streams_context)
            streams = (read_stream, write_stream)
        else:  # sse_client
            streams = self._exit_stack.enter_context(streams_context)

        session_context = ClientSession(*streams)
        self._session = self._exit_stack.enter_context(session_context)
        self._session.initialize()

    def list_tools(self) -> list[Tool]:
        """List available tools from the MCP server"""
        if not self._session:
            raise ValueError("Session not initialized.")
        response = self._session.list_tools()
        return response.tools

    def invoke_tool(self, tool_name: str, tool_args: dict[str, Any]) -> CallToolResult:
        """Call a tool"""
        if not self._session:
            raise ValueError("Session not initialized.")
        return self._session.call_tool(tool_name, tool_args)

    def cleanup(self):
        """Clean up resources"""
        try:
            # ExitStack will handle proper cleanup of all managed context managers
            self._exit_stack.close()
        except Exception as e:
            logger.exception("Error during cleanup")
            raise ValueError(f"Error during cleanup: {e}")
        finally:
            self._session = None
            self._initialized = False

```

### api/core/mcp/client/sse_client.py
```py
import logging
import queue
from collections.abc import Generator
from concurrent.futures import ThreadPoolExecutor
from contextlib import contextmanager
from typing import Any, final
from urllib.parse import urljoin, urlparse

import httpx
from httpx_sse import EventSource, ServerSentEvent
from sseclient import SSEClient

from core.mcp import types
from core.mcp.error import MCPAuthError, MCPConnectionError
from core.mcp.types import SessionMessage
from core.mcp.utils import create_ssrf_proxy_mcp_http_client, ssrf_proxy_sse_connect

logger = logging.getLogger(__name__)

DEFAULT_QUEUE_READ_TIMEOUT = 3


@final
class _StatusReady:
    def __init__(self, endpoint_url: str):
        self.endpoint_url = endpoint_url


@final
class _StatusError:
    def __init__(self, exc: Exception):
        self.exc = exc


# Type aliases for better readability
type ReadQueue = queue.Queue[SessionMessage | Exception | None]
type WriteQueue = queue.Queue[SessionMessage | Exception | None]
type StatusQueue = queue.Queue[_StatusReady | _StatusError]


class SSETransport:
    """SSE client transport implementation."""

    def __init__(
        self,
        url: str,
        headers: dict[str, Any] | None = None,
        timeout: float = 5.0,
        sse_read_timeout: float = 1 * 60,
    ):
        """Initialize the SSE transport.

        Args:
            url: The SSE endpoint URL.
            headers: Optional headers to include in requests.
            timeout: HTTP timeout for regular operations.
            sse_read_timeout: Timeout for SSE read operations.
        """
        self.url = url
        self.headers = headers or {}
        self.timeout = timeout
        self.sse_read_timeout = sse_read_timeout
        self.endpoint_url: str | None = None
        self.event_source: EventSource | None = None

    def _validate_endpoint_url(self, endpoint_url: str) -> bool:
        """Validate that the endpoint URL matches the connection origin.

        Args:
            endpoint_url: The endpoint URL to validate.

        Returns:
            True if valid, False otherwise.
        """
        url_parsed = urlparse(self.url)
        endpoint_parsed = urlparse(endpoint_url)

        return url_parsed.netloc == endpoint_parsed.netloc and url_parsed.scheme == endpoint_parsed.scheme

    def _handle_endpoint_event(self, sse_data: str, status_queue: StatusQueue):
        """Handle an 'endpoint' SSE event.

        Args:
            sse_data: The SSE event data.
            status_queue: Queue to put status updates.
        """
        endpoint_url = urljoin(self.url, sse_data)
        logger.info("Received endpoint URL: %s", endpoint_url)

        if not self._validate_endpoint_url(endpoint_url):
            error_msg = f"Endpoint origin does not match connection origin: {endpoint_url}"
            logger.error(error_msg)
            status_queue.put(_StatusError(ValueError(error_msg)))
            return

        status_queue.put(_StatusReady(endpoint_url))

    def _handle_message_event(self, sse_data: str, read_queue: ReadQueue):
        """Handle a 'message' SSE event.

        Args:
            sse_data: The SSE event data.
            read_queue: Queue to put parsed messages.
        """
        try:
            message = types.JSONRPCMessage.model_validate_json(sse_data)
            logger.debug("Received server message: %s", message)
            session_message = SessionMessage(message)
            read_queue.put(session_message)
        except Exception as exc:
            logger.exception("Error parsing server message")
            read_queue.put(exc)

    def _handle_sse_event(self, sse: ServerSentEvent, read_queue: ReadQueue, status_queue: StatusQueue):
        """Handle a single SSE event.

        Args:
            sse: The SSE event object.
            read_queue: Queue for message events.
            status_queue: Queue for status events.
        """
        match sse.event:
            case "endpoint":
                self._handle_endpoint_event(sse.data, status_queue)
            case "message":
                self._handle_message_event(sse.data, read_queue)
            case _:
                logger.warning("Unknown SSE event: %s", sse.event)

    def sse_reader(self, event_source: EventSource, read_queue: ReadQueue, status_queue: StatusQueue):
        """Read and process SSE events.

        Args:
            event_source: The SSE event source.
            read_queue: Queue to put received messages.
            status_queue: Queue to put status updates.
        """
        try:
            for sse in event_source.iter_sse():
                self._handle_sse_event(sse, read_queue, status_queue)
        except httpx.ReadError as exc:
            logger.debug("SSE reader shutting down normally: %s", exc)
        except Exception as exc:
            read_queue.put(exc)
        finally:
            read_queue.put(None)

    def _send_message(self, client: httpx.Client, endpoint_url: str, message: SessionMessage):
        """Send a single message to the server.

        Args:
            client: HTTP client to use.
            endpoint_url: The endpoint URL to send to.
            message: The message to send.
        """
        response = client.post(
            endpoint_url,
            json=message.message.model_dump(
                by_alias=True,
                mode="json",
                exclude_none=True,
            ),
        )
        response.raise_for_status()
        logger.debug("Client message sent successfully: %s", response.status_code)

    def post_writer(self, client: httpx.Client, endpoint_url: str, write_queue: WriteQueue):
        """Handle writing messages to the server.

        Args:
            client: HTTP client to use.
            endpoint_url: The endpoint URL to send messages to.
            write_queue: Queue to read messages from.
        """
        try:
            while True:
                try:
                    message = write_queue.get(timeout=DEFAULT_QUEUE_READ_TIMEOUT)
                    if message is None:
                        break
                    if isinstance(message, Exception):
                        write_queue.put(message)
                        continue

                    self._send_message(client, endpoint_url, message)

                except queue.Empty:
                    continue
        except httpx.ReadError as exc:
            logger.debug("Post writer shutting down normally: %s", exc)
        except Exception as exc:
            logger.exception("Error writing messages")
            write_queue.put(exc)
        finally:
            write_queue.put(None)

    def _wait_for_endpoint(self, status_queue: StatusQueue) -> str:
        """Wait for the endpoint URL from the status queue.

        Args:
            status_queue: Queue to read status from.

        Returns:
            The endpoint URL.

        Raises:
            ValueError: If endpoint URL is not received or there's an error.
        """
        try:
            status = status_queue.get(timeout=1)
        except queue.Empty:
            raise ValueError("failed to get endpoint URL")

        if isinstance(status, _StatusReady):
            return status.endpoint_url
        elif isinstance(status, _StatusError):
            raise status.exc
        else:
            raise ValueError("failed to get endpoint URL")

    def connect(
        self,
        executor: ThreadPoolExecutor,
        client: httpx.Client,
        event_source: EventSource,
    ) -> tuple[ReadQueue, WriteQueue]:
        """Establish connection and start worker threads.

        Args:
            executor: Thread pool executor.
            client: HTTP client.
            event_source: SSE event source.

        Returns:
            Tuple of (read_queue, write_queue).
        """
        read_queue: ReadQueue = queue.Queue()
        write_queue: WriteQueue = queue.Queue()
        status_queue: StatusQueue = queue.Queue()

        # Store event_source for graceful shutdown
        self.event_source = event_source

        # Start SSE reader thread
        executor.submit(self.sse_reader, event_source, read_queue, status_queue)

        # Wait for endpoint URL
        endpoint_url = self._wait_for_endpoint(status_queue)
        self.endpoint_url = endpoint_url

        # Start post writer thread
        executor.submit(self.post_writer, client, endpoint_url, write_queue)

        return read_queue, write_queue


@contextmanager
def sse_client(
    url: str,
    headers: dict[str, Any] | None = None,
    timeout: float = 5.0,
    sse_read_timeout: float = 1 * 60,
) -> Generator[tuple[ReadQueue, WriteQueue], None, None]:
    """
    Client transport for SSE.
    `sse_read_timeout` determines how long (in seconds) the client will wait for a new
    event before disconnecting. All other HTTP operations are controlled by `timeout`.

    Args:
        url: The SSE endpoint URL.
        headers: Optional headers to include in requests.
        timeout: HTTP timeout for regular operations.
        sse_read_timeout: Timeout for SSE read operations.

    Yields:
        Tuple of (read_queue, write_queue) for message communication.
    """
    transport = SSETransport(url, headers, timeout, sse_read_timeout)

    read_queue: ReadQueue | None = None
    write_queue: WriteQueue | None = None

    executor = ThreadPoolExecutor()
    try:
        with create_ssrf_proxy_mcp_http_client(headers=transport.headers) as client:
            with ssrf_proxy_sse_connect(
                url, timeout=httpx.Timeout(timeout, read=sse_read_timeout), client=client
            ) as event_source:
                event_source.response.raise_for_status()

                read_queue, write_queue = transport.connect(executor, client, event_source)

                yield read_queue, write_queue

    except httpx.HTTPStatusError as exc:
        if exc.response.status_code == 401:
            raise MCPAuthError(response=exc.response)
        raise MCPConnectionError()
    except Exception:
        logger.exception("Error connecting to SSE endpoint")
        raise
    finally:
        # Close the SSE connection to unblock the reader thread
        if transport.event_source is not None:
            try:
                transport.event_source.response.close()
            except RuntimeError:
                pass

        # Clean up queues
        if read_queue:
            read_queue.put(None)
        if write_queue:
            write_queue.put(None)

        # Shutdown executor without waiting to prevent hanging
        executor.shutdown(wait=False)


def send_message(http_client: httpx.Client, endpoint_url: str, session_message: SessionMessage):
    """
    Send a message to the server using the provided HTTP client.

    Args:
        http_client: The HTTP client to use for sending
        endpoint_url: The endpoint URL to send the message to
        session_message: The message to send
    """
    try:
        response = http_client.post(
            endpoint_url,
            json=session_message.message.model_dump(
                by_alias=True,
                mode="json",
                exclude_none=True,
            ),
        )
        response.raise_for_status()
        logger.debug("Client message sent successfully: %s", response.status_code)
    except Exception:
        logger.exception("Error sending message")
        raise


def read_messages(
    sse_client: SSEClient,
) -> Generator[SessionMessage | Exception, None, None]:
    """
    Read messages from the SSE client.

    Args:
        sse_client: The SSE client to read from

    Yields:
        SessionMessage or Exception for each event received
    """
    try:
        for sse in sse_client.events():
            if sse.event == "message":
                try:
                    message = types.JSONRPCMessage.model_validate_json(sse.data)
                    logger.debug("Received server message: %s", message)
                    yield SessionMessage(message)
                except Exception as exc:
                    logger.exception("Error parsing server message")
                    yield exc
            else:
                logger.warning("Unknown SSE event: %s", sse.event)
    except Exception as exc:
        logger.exception("Error reading SSE messages")
        yield exc

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
