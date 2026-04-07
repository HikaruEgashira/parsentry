You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-020
- **Kind**: endpoint
- **Identifier**: Trigger/Debug Endpoints
- **Description**: Webhook triggers and debug endpoints for app execution. Risk of unauthenticated webhook invocation, debug endpoint exposure in production, and request forgery via webhook replay.
- **Locations**: api/controllers/trigger/, api/core/trigger/debug/, api/core/trigger/

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

### api/controllers/trigger/webhook.py
```py
import logging
import time

from flask import jsonify, request
from werkzeug.exceptions import NotFound, RequestEntityTooLarge

from controllers.trigger import bp
from core.trigger.debug.event_bus import TriggerDebugEventBus
from core.trigger.debug.events import WebhookDebugEvent, build_webhook_pool_key
from services.trigger.webhook_service import RawWebhookDataDict, WebhookService

logger = logging.getLogger(__name__)


def _prepare_webhook_execution(webhook_id: str, is_debug: bool = False):
    """Fetch trigger context, extract request data, and validate payload using unified processing.

    Args:
        webhook_id: The webhook ID to process
        is_debug: If True, skip status validation for debug mode
    """
    webhook_trigger, workflow, node_config = WebhookService.get_webhook_trigger_and_workflow(
        webhook_id, is_debug=is_debug
    )

    webhook_data: RawWebhookDataDict
    try:
        # Use new unified extraction and validation
        webhook_data = WebhookService.extract_and_validate_webhook_data(webhook_trigger, node_config)
        return webhook_trigger, workflow, node_config, webhook_data, None
    except ValueError as e:
        # Provide minimal context for error reporting without risking another parse failure
        webhook_data = {
            "method": request.method,
            "headers": dict(request.headers),
            "query_params": dict(request.args),
            "body": {},
            "files": {},
        }
        return webhook_trigger, workflow, node_config, webhook_data, str(e)


@bp.route("/webhook/<string:webhook_id>", methods=["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
def handle_webhook(webhook_id: str):
    """
    Handle webhook trigger calls.

    This endpoint receives webhook calls and processes them according to the
    configured webhook trigger settings.
    """
    try:
        webhook_trigger, workflow, node_config, webhook_data, error = _prepare_webhook_execution(webhook_id)
        if error:
            return jsonify({"error": "Bad Request", "message": error}), 400

        # Process webhook call (send to Celery)
        WebhookService.trigger_workflow_execution(webhook_trigger, webhook_data, workflow)

        # Return configured response
        response_data, status_code = WebhookService.generate_webhook_response(node_config)
        return jsonify(response_data), status_code

    except ValueError as e:
        raise NotFound(str(e))
    except RequestEntityTooLarge:
        raise
    except Exception as e:
        logger.exception("Webhook processing failed for %s", webhook_id)
        return jsonify({"error": "Internal server error", "message": str(e)}), 500


@bp.route("/webhook-debug/<string:webhook_id>", methods=["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
def handle_webhook_debug(webhook_id: str):
    """Handle webhook debug calls without triggering production workflow execution.

    The debug webhook endpoint is only for draft inspection flows. It never enqueues
    Celery work for the published workflow; instead it dispatches an in-memory debug
    event to an active Variable Inspector listener. Returning a clear error when no
    listener is registered prevents a misleading 200 response for requests that are
    effectively dropped.
    """
    try:
        webhook_trigger, _, node_config, webhook_data, error = _prepare_webhook_execution(webhook_id, is_debug=True)
        if error:
            return jsonify({"error": "Bad Request", "message": error}), 400

        workflow_inputs = WebhookService.build_workflow_inputs(webhook_data)

        # Generate pool key and dispatch debug event
        pool_key: str = build_webhook_pool_key(
            tenant_id=webhook_trigger.tenant_id,
            app_id=webhook_trigger.app_id,
            node_id=webhook_trigger.node_id,
        )
        event = WebhookDebugEvent(
            request_id=f"webhook_debug_{webhook_trigger.webhook_id}_{int(time.time() * 1000)}",
            timestamp=int(time.time()),
            node_id=webhook_trigger.node_id,
            payload={
                "inputs": workflow_inputs,
                "webhook_data": webhook_data,
                "method": webhook_data.get("method"),
            },
        )
        dispatch_count = TriggerDebugEventBus.dispatch(
            tenant_id=webhook_trigger.tenant_id,
            event=event,
            pool_key=pool_key,
        )
        if dispatch_count == 0:
            logger.warning(
                "Webhook debug request dropped without an active listener for webhook %s (tenant=%s, app=%s, node=%s)",
                webhook_trigger.webhook_id,
                webhook_trigger.tenant_id,
                webhook_trigger.app_id,
                webhook_trigger.node_id,
            )
            return (
                jsonify(
                    {
                        "error": "No active debug listener",
                        "message": (
                            "The webhook debug URL only works while the Variable Inspector is listening. "
                            "Use the published webhook URL to execute the workflow in Celery."
                        ),
                        "execution_url": webhook_trigger.webhook_url,
                    }
                ),
                409,
            )
        response_data, status_code = WebhookService.generate_webhook_response(node_config)
        return jsonify(response_data), status_code

    except ValueError as e:
        raise NotFound(str(e))
    except RequestEntityTooLarge:
        raise
    except Exception as e:
        logger.exception("Webhook debug processing failed for %s", webhook_id)
        return jsonify({"error": "Internal server error", "message": "An internal error has occurred."}), 500

```

### api/controllers/trigger/__init__.py
```py
from flask import Blueprint

# Create trigger blueprint
bp = Blueprint("trigger", __name__, url_prefix="/triggers")

# Import routes after blueprint creation to avoid circular imports
from . import trigger, webhook

__all__ = [
    "trigger",
    "webhook",
]

```

### api/controllers/trigger/trigger.py
```py
import logging
import re

from flask import jsonify, request
from werkzeug.exceptions import NotFound

from controllers.trigger import bp
from services.trigger.trigger_service import TriggerService
from services.trigger.trigger_subscription_builder_service import TriggerSubscriptionBuilderService

logger = logging.getLogger(__name__)

UUID_PATTERN = r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
UUID_MATCHER = re.compile(UUID_PATTERN)


@bp.route("/plugin/<string:endpoint_id>", methods=["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
def trigger_endpoint(endpoint_id: str):
    """
    Handle endpoint trigger calls.
    """
    # endpoint_id must be UUID
    if not UUID_MATCHER.match(endpoint_id):
        raise NotFound("Invalid endpoint ID")
    handling_chain = [
        TriggerService.process_endpoint,
        TriggerSubscriptionBuilderService.process_builder_validation_endpoint,
    ]
    response = None
    try:
        for handler in handling_chain:
            response = handler(endpoint_id, request)
            if response:
                break
        if not response:
            logger.info("Endpoint not found for %s", endpoint_id)
            return jsonify({"error": "Endpoint not found"}), 404
        return response
    except ValueError as e:
        return jsonify({"error": "Endpoint processing failed", "message": str(e)}), 400
    except Exception:
        logger.exception("Webhook processing failed for {endpoint_id}")
        return jsonify({"error": "Internal server error"}), 500

```

### api/core/trigger/debug/event_selectors.py
```py
"""Trigger debug service supporting plugin and webhook debugging in draft workflows."""

import hashlib
import logging
import time
from abc import ABC, abstractmethod
from collections.abc import Mapping
from datetime import datetime
from typing import Any

from graphon.entities.graph_config import NodeConfigDict
from pydantic import BaseModel

from core.plugin.entities.request import TriggerInvokeEventResponse
from core.trigger.constants import (
    TRIGGER_PLUGIN_NODE_TYPE,
    TRIGGER_SCHEDULE_NODE_TYPE,
    TRIGGER_WEBHOOK_NODE_TYPE,
)
from core.trigger.debug.event_bus import TriggerDebugEventBus
from core.trigger.debug.events import (
    PluginTriggerDebugEvent,
    ScheduleDebugEvent,
    WebhookDebugEvent,
    build_plugin_pool_key,
    build_webhook_pool_key,
)
from core.workflow.nodes.trigger_plugin.entities import TriggerEventNodeData
from core.workflow.nodes.trigger_schedule.entities import ScheduleConfig
from extensions.ext_redis import redis_client
from libs.datetime_utils import ensure_naive_utc, naive_utc_now
from libs.schedule_utils import calculate_next_run_at
from models.model import App
from models.provider_ids import TriggerProviderID
from models.workflow import Workflow

logger = logging.getLogger(__name__)


class TriggerDebugEvent(BaseModel):
    workflow_args: Mapping[str, Any]
    node_id: str


class TriggerDebugEventPoller(ABC):
    app_id: str
    user_id: str
    tenant_id: str
    node_config: NodeConfigDict
    node_id: str

    def __init__(self, tenant_id: str, user_id: str, app_id: str, node_config: NodeConfigDict, node_id: str):
        self.tenant_id = tenant_id
        self.user_id = user_id
        self.app_id = app_id
        self.node_config = node_config
        self.node_id = node_id

    @abstractmethod
    def poll(self) -> TriggerDebugEvent | None:
        raise NotImplementedError


class PluginTriggerDebugEventPoller(TriggerDebugEventPoller):
    def poll(self) -> TriggerDebugEvent | None:
        from services.trigger.trigger_service import TriggerService

        plugin_trigger_data = TriggerEventNodeData.model_validate(self.node_config["data"], from_attributes=True)
        provider_id = TriggerProviderID(plugin_trigger_data.provider_id)
        pool_key: str = build_plugin_pool_key(
            name=plugin_trigger_data.event_name,
            provider_id=str(provider_id),
            tenant_id=self.tenant_id,
            subscription_id=plugin_trigger_data.subscription_id,
        )
        plugin_trigger_event: PluginTriggerDebugEvent | None = TriggerDebugEventBus.poll(
            event_type=PluginTriggerDebugEvent,
            pool_key=pool_key,
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            app_id=self.app_id,
            node_id=self.node_id,
        )
        if not plugin_trigger_event:
            return None
        trigger_event_response: TriggerInvokeEventResponse = TriggerService.invoke_trigger_event(
            event=plugin_trigger_event,
            user_id=plugin_trigger_event.user_id,
            tenant_id=self.tenant_id,
            node_config=self.node_config,
        )

        if trigger_event_response.cancelled:
            return None

        return TriggerDebugEvent(
            workflow_args={
                "inputs": trigger_event_response.variables,
                "files": [],
            },
            node_id=self.node_id,
        )


class WebhookTriggerDebugEventPoller(TriggerDebugEventPoller):
    def poll(self) -> TriggerDebugEvent | None:
        pool_key = build_webhook_pool_key(
            tenant_id=self.tenant_id,
            app_id=self.app_id,
            node_id=self.node_id,
        )
        webhook_event: WebhookDebugEvent | None = TriggerDebugEventBus.poll(
            event_type=WebhookDebugEvent,
            pool_key=pool_key,
            tenant_id=self.tenant_id,
            user_id=self.user_id,
            app_id=self.app_id,
            node_id=self.node_id,
        )
        if not webhook_event:
            return None

        from services.trigger.webhook_service import WebhookService

        payload = webhook_event.payload or {}
        workflow_inputs = payload.get("inputs")
        if workflow_inputs is None:
            webhook_data = payload.get("webhook_data", {})
            workflow_inputs = WebhookService.build_workflow_inputs(webhook_data)

        workflow_args: Mapping[str, Any] = {
            "inputs": workflow_inputs or {},
            "files": [],
        }
        return TriggerDebugEvent(workflow_args=workflow_args, node_id=self.node_id)


class ScheduleTriggerDebugEventPoller(TriggerDebugEventPoller):
    """
    Poller for schedule trigger debug events.

    This poller will simulate the schedule trigger event by creating a schedule debug runtime cache
    and calculating the next run at.
    """

    RUNTIME_CACHE_TTL = 60 * 5

    class ScheduleDebugRuntime(BaseModel):
        cache_key: str
        timezone: str
        cron_expression: str
        next_run_at: datetime

    def schedule_debug_runtime_key(self, cron_hash: str) -> str:
        return f"schedule_debug_runtime:{self.tenant_id}:{self.user_id}:{self.app_id}:{self.node_id}:{cron_hash}"

    def get_or_create_schedule_debug_runtime(self):
        from services.trigger.schedule_service import ScheduleService

        schedule_config: ScheduleConfig = ScheduleService.to_schedule_config(self.node_config)
        cron_hash = hashlib.sha256(schedule_config.cron_expression.encode()).hexdigest()
        cache_key = self.schedule_debug_runtime_key(cron_hash)
        runtime_cache = redis_client.get(cache_key)
        if runtime_cache is None:
            schedule_debug_runtime = self.ScheduleDebugRuntime(
                cron_expression=schedule_config.cron_expression,
                timezone=schedule_config.timezone,
                cache_key=cache_key,
                next_run_at=ensure_naive_utc(
                    calculate_next_run_at(schedule_config.cron_expression, schedule_config.timezone)
                ),
            )
            redis_client.setex(
                name=self.schedule_debug_runtime_key(cron_hash),
                time=self.RUNTIME_CACHE_TTL,
                value=schedule_debug_runtime.model_dump_json(),
            )
            return schedule_debug_runtime
        else:
            redis_client.expire(cache_key, self.RUNTIME_CACHE_TTL)
            runtime = self.ScheduleDebugRuntime.model_validate_json(runtime_cache)
            runtime.next_run_at = ensure_naive_utc(runtime.next_run_at)
            return runtime

    def create_schedule_event(self, schedule_debug_runtime: ScheduleDebugRuntime) -> ScheduleDebugEvent:
        redis_client.delete(schedule_debug_runtime.cache_key)
        return ScheduleDebugEvent(
            timestamp=int(time.time()),
            node_id=self.node_id,
            inputs={},
        )

    def poll(self) -> TriggerDebugEvent | None:
        schedule_debug_runtime = self.get_or_create_schedule_debug_runtime()
        if schedule_debug_runtime.next_run_at > naive_utc_now():
            return None

        schedule_event: ScheduleDebugEvent = self.create_schedule_event(schedule_debug_runtime)
        workflow_args: Mapping[str, Any] = {
            "inputs": schedule_event.inputs or {},
            "files": [],
        }
        return TriggerDebugEvent(workflow_args=workflow_args, node_id=self.node_id)


def create_event_poller(
    draft_workflow: Workflow, tenant_id: str, user_id: str, app_id: str, node_id: str
) -> TriggerDebugEventPoller:
    node_config = draft_workflow.get_node_config_by_id(node_id=node_id)
    if not node_config:
        raise ValueError("Node data not found for node %s", node_id)
    node_type = draft_workflow.get_node_type_from_node_config(node_config)
    if node_type == TRIGGER_PLUGIN_NODE_TYPE:
        return PluginTriggerDebugEventPoller(
            tenant_id=tenant_id, user_id=user_id, app_id=app_id, node_config=node_config, node_id=node_id
        )
    if node_type == TRIGGER_WEBHOOK_NODE_TYPE:
        return WebhookTriggerDebugEventPoller(
            tenant_id=tenant_id, user_id=user_id, app_id=app_id, node_config=node_config, node_id=node_id
        )
    if node_type == TRIGGER_SCHEDULE_NODE_TYPE:
        return ScheduleTriggerDebugEventPoller(
            tenant_id=tenant_id, user_id=user_id, app_id=app_id, node_config=node_config, node_id=node_id
        )
    raise ValueError("unable to create event poller for node type %s", node_type)


def select_trigger_debug_events(
    draft_workflow: Workflow, app_model: App, user_id: str, node_ids: list[str]
) -> TriggerDebugEvent | None:
    event: TriggerDebugEvent | None = None
    for node_id in node_ids:
        node_config = draft_workflow.get_node_config_by_id(node_id=node_id)
        if not node_config:
            raise ValueError("Node data not found for node %s", node_id)
        poller: TriggerDebugEventPoller = create_event_poller(
            draft_workflow=draft_workflow,
            tenant_id=app_model.tenant_id,
            user_id=user_id,
            app_id=app_model.id,
            node_id=node_id,
        )
        event = poller.poll()
        if event is not None:
            return event
    return None

```

### api/core/trigger/debug/events.py
```py
from collections.abc import Mapping
from enum import StrEnum
from typing import Any

from pydantic import BaseModel, Field


class TriggerDebugPoolKey(StrEnum):
    """Trigger debug pool key."""

    SCHEDULE = "schedule_trigger_debug_waiting_pool"
    WEBHOOK = "webhook_trigger_debug_waiting_pool"
    PLUGIN = "plugin_trigger_debug_waiting_pool"


class BaseDebugEvent(BaseModel):
    """Base class for all debug events."""

    timestamp: int


class ScheduleDebugEvent(BaseDebugEvent):
    """Debug event for schedule triggers."""

    node_id: str
    inputs: Mapping[str, Any]


class WebhookDebugEvent(BaseDebugEvent):
    """Debug event for webhook triggers."""

    request_id: str
    node_id: str
    payload: dict[str, Any] = Field(default_factory=dict)


def build_webhook_pool_key(tenant_id: str, app_id: str, node_id: str) -> str:
    """Generate pool key for webhook events.

    Args:
        tenant_id: Tenant ID
        app_id: App ID
        node_id: Node ID
    """
    return f"{TriggerDebugPoolKey.WEBHOOK}:{{{tenant_id}}}:{app_id}:{node_id}"


class PluginTriggerDebugEvent(BaseDebugEvent):
    """Debug event for plugin triggers."""

    name: str
    user_id: str = Field(description="This is end user id, only for trigger the event. no related with account user id")
    request_id: str
    subscription_id: str
    provider_id: str


def build_plugin_pool_key(tenant_id: str, provider_id: str, subscription_id: str, name: str) -> str:
    """Generate pool key for plugin trigger events.

    Args:
        name: Event name
        tenant_id: Tenant ID
        provider_id: Provider ID
        subscription_id: Subscription ID
    """
    return f"{TriggerDebugPoolKey.PLUGIN}:{{{tenant_id}}}:{str(provider_id)}:{subscription_id}:{name}"

```

### api/core/trigger/debug/event_bus.py
```py
import hashlib
import logging

from redis import RedisError

from core.trigger.debug.events import BaseDebugEvent
from extensions.ext_redis import redis_client

logger = logging.getLogger(__name__)

TRIGGER_DEBUG_EVENT_TTL = 300


class TriggerDebugEventBus:
    """
    Unified Redis-based trigger debug service with polling support.

    Uses {tenant_id} hash tags for Redis Cluster compatibility.
    Supports multiple event types through a generic dispatch/poll interface.
    """

    # LUA_SELECT: Atomic poll or register for event
    # KEYS[1] = trigger_debug_inbox:{<tenant_id>}:<address_id>
    # KEYS[2] = trigger_debug_waiting_pool:{<tenant_id>}:...
    # ARGV[1] = address_id
    LUA_SELECT = (
        "local v=redis.call('GET',KEYS[1]);"
        "if v then redis.call('DEL',KEYS[1]);return v end;"
        "redis.call('SADD',KEYS[2],ARGV[1]);"
        f"redis.call('EXPIRE',KEYS[2],{TRIGGER_DEBUG_EVENT_TTL});"
        "return false"
    )

    # LUA_DISPATCH: Dispatch event to all waiting addresses
    # KEYS[1] = trigger_debug_waiting_pool:{<tenant_id>}:...
    # ARGV[1] = tenant_id
    # ARGV[2] = event_json
    LUA_DISPATCH = (
        "local a=redis.call('SMEMBERS',KEYS[1]);"
        "if #a==0 then return 0 end;"
        "redis.call('DEL',KEYS[1]);"
        "for i=1,#a do "
        f"redis.call('SET','trigger_debug_inbox:{{'..ARGV[1]..'}}'..':'..a[i],ARGV[2],'EX',{TRIGGER_DEBUG_EVENT_TTL});"
        "end;"
        "return #a"
    )

    @classmethod
    def dispatch(
        cls,
        tenant_id: str,
        event: BaseDebugEvent,
        pool_key: str,
    ) -> int:
        """
        Dispatch event to all waiting addresses in the pool.

        Args:
            tenant_id: Tenant ID for hash tag
            event: Event object to dispatch
            pool_key: Pool key (generate using build_{?}_pool_key(...))

        Returns:
            Number of addresses the event was dispatched to
        """
        event_data = event.model_dump_json()
        try:
            result = redis_client.eval(
                cls.LUA_DISPATCH,
                1,
                pool_key,
                tenant_id,
                event_data,
            )
            return int(result)
        except RedisError:
            logger.exception("Failed to dispatch event to pool: %s", pool_key)
            return 0

    @classmethod
    def poll[T: BaseDebugEvent](
        cls,
        event_type: type[T],
        pool_key: str,
        tenant_id: str,
        user_id: str,
        app_id: str,
        node_id: str,
    ) -> T | None:
        """
        Poll for an event or register to the waiting pool.

        If an event is available in the inbox, return it immediately.
        Otherwise, register the address to the waiting pool for future dispatch.

        Args:
            event_class: Event class for deserialization and type safety
            pool_key: Pool key (generate using build_{?}_pool_key(...))
            tenant_id: Tenant ID
            user_id: User ID for address calculation
            app_id: App ID for address calculation
            node_id: Node ID for address calculation

        Returns:
            Event object if available, None otherwise
        """
        address_id: str = hashlib.sha256(f"{user_id}|{app_id}|{node_id}".encode()).hexdigest()
        address: str = f"trigger_debug_inbox:{{{tenant_id}}}:{address_id}"

        try:
            event_data = redis_client.eval(
                cls.LUA_SELECT,
                2,
                address,
                pool_key,
                address_id,
            )
            return event_type.model_validate_json(json_data=event_data) if event_data else None
        except RedisError:
            logger.exception("Failed to poll event from pool: %s", pool_key)
            return None

```

### api/core/trigger/provider.py
```py
"""
Trigger Provider Controller for managing trigger providers
"""

import logging
from collections.abc import Mapping
from typing import Any

from flask import Request

from core.entities.provider_entities import BasicProviderConfig
from core.plugin.entities.plugin_daemon import CredentialType
from core.plugin.entities.request import (
    TriggerDispatchResponse,
    TriggerInvokeEventResponse,
    TriggerSubscriptionResponse,
)
from core.plugin.impl.trigger import PluginTriggerClient
from core.trigger.entities.api_entities import EventApiEntity, TriggerProviderApiEntity
from core.trigger.entities.entities import (
    EventEntity,
    EventParameter,
    ProviderConfig,
    Subscription,
    SubscriptionConstructor,
    TriggerCreationMethod,
    TriggerProviderEntity,
    TriggerProviderIdentity,
    UnsubscribeResult,
)
from core.trigger.errors import TriggerProviderCredentialValidationError
from models.provider_ids import TriggerProviderID
from services.plugin.plugin_service import PluginService

logger = logging.getLogger(__name__)


class PluginTriggerProviderController:
    """
    Controller for plugin trigger providers
    """

    def __init__(
        self,
        entity: TriggerProviderEntity,
        plugin_id: str,
        plugin_unique_identifier: str,
        provider_id: TriggerProviderID,
        tenant_id: str,
    ):
        """
        Initialize plugin trigger provider controller

        :param entity: Trigger provider entity
        :param plugin_id: Plugin ID
        :param plugin_unique_identifier: Plugin unique identifier
        :param provider_id: Provider ID
        :param tenant_id: Tenant ID
        """
        self.entity = entity
        self.tenant_id = tenant_id
        self.plugin_id = plugin_id
        self.provider_id = provider_id
        self.plugin_unique_identifier = plugin_unique_identifier

    def get_provider_id(self) -> TriggerProviderID:
        """
        Get provider ID
        """
        return self.provider_id

    def to_api_entity(self) -> TriggerProviderApiEntity:
        """
        Convert to API entity
        """
        icon = (
            PluginService.get_plugin_icon_url(self.tenant_id, self.entity.identity.icon)
            if self.entity.identity.icon
            else None
        )
        icon_dark = (
            PluginService.get_plugin_icon_url(self.tenant_id, self.entity.identity.icon_dark)
            if self.entity.identity.icon_dark
            else None
        )
        subscription_constructor = self.entity.subscription_constructor
        supported_creation_methods = [TriggerCreationMethod.MANUAL]
        if subscription_constructor and subscription_constructor.oauth_schema:
            supported_creation_methods.append(TriggerCreationMethod.OAUTH)
        if subscription_constructor and subscription_constructor.credentials_schema:
            supported_creation_methods.append(TriggerCreationMethod.APIKEY)
        return TriggerProviderApiEntity(
            author=self.entity.identity.author,
            name=self.entity.identity.name,
            label=self.entity.identity.label,
            description=self.entity.identity.description,
            icon=icon,
            icon_dark=icon_dark,
            tags=self.entity.identity.tags,
            plugin_id=self.plugin_id,
            plugin_unique_identifier=self.plugin_unique_identifier,
            subscription_constructor=subscription_constructor,
            subscription_schema=self.entity.subscription_schema,
            supported_creation_methods=supported_creation_methods,
            events=[
                EventApiEntity(
                    name=event.identity.name,
                    identity=event.identity,
                    description=event.description,
                    parameters=event.parameters,
                    output_schema=event.output_schema,
                )
                for event in self.entity.events
            ],
        )

    @property
    def identity(self) -> TriggerProviderIdentity:
        """Get provider identity"""
        return self.entity.identity

    def get_events(self) -> list[EventEntity]:
        """
        Get all events for this provider

        :return: List of event entities
        """
        return self.entity.events

    def get_event(self, event_name: str) -> EventEntity | None:
        """
        Get a specific event by name

        :param event_name: Event name
        :return: Event entity or None
        """
        for event in self.entity.events:
            if event.identity.name == event_name:
                return event
        return None

    def get_subscription_default_properties(self) -> Mapping[str, Any]:
        """
        Get default properties for this provider

        :return: Default properties
        """
        return {prop.name: prop.default for prop in self.entity.subscription_schema if prop.default}

    def get_subscription_constructor(self) -> SubscriptionConstructor | None:
        """
        Get subscription constructor for this provider

        :return: Subscription constructor
        """
        return self.entity.subscription_constructor

    def validate_credentials(self, user_id: str, credentials: Mapping[str, str]) -> None:
        """
        Validate credentials against schema

        :param credentials: Credentials to validate
        :return: Validation response
        """
        # First validate against schema
        subscription_constructor: SubscriptionConstructor | None = self.entity.subscription_constructor
        if not subscription_constructor:
            raise ValueError("Subscription constructor not found")
        for config in subscription_constructor.credentials_schema or []:
            if config.required and config.name not in credentials:
                raise TriggerProviderCredentialValidationError(f"Missing required credential field: {config.name}")

        # Then validate with the plugin daemon
        manager = PluginTriggerClient()
        provider_id = self.get_provider_id()
        response = manager.validate_provider_credentials(
            tenant_id=self.tenant_id,
            user_id=user_id,
            provider=str(provider_id),
            credentials=credentials,
        )
        if not response:
            raise TriggerProviderCredentialValidationError(
                "Invalid credentials",
            )

    def get_supported_credential_types(self) -> list[CredentialType]:
        """
        Get supported credential types for this provider.

        :return: List of supported credential types
        """
        types: list[CredentialType] = []
        subscription_constructor = self.entity.subscription_constructor
        if subscription_constructor and subscription_constructor.oauth_schema:
            types.append(CredentialType.OAUTH2)
        if subscription_constructor and subscription_constructor.credentials_schema:
            types.append(CredentialType.API_KEY)
        return types

    def get_credentials_schema(self, credential_type: CredentialType | str) -> list[ProviderConfig]:
        """
        Get credentials schema by credential type

        :param credential_type: The type of credential (oauth or api_key)
        :return: List of provider config schemas
        """
        subscription_constructor = self.entity.subscription_constructor
        if not subscription_constructor:
            return []
        credential_type = CredentialType.of(credential_type)
        if credential_type == CredentialType.OAUTH2:
            return (
                subscription_constructor.oauth_schema.credentials_schema.copy()
                if subscription_constructor and subscription_constructor.oauth_schema
                else []
            )
        if credential_type == CredentialType.API_KEY:
            return (
                subscription_constructor.credentials_schema.copy() or []
                if subscription_constructor and subscription_constructor.credentials_schema
                else []
            )
        if credential_type == CredentialType.UNAUTHORIZED:
            return []
        raise ValueError(f"Invalid credential type: {credential_type}")

    def get_credential_schema_config(self, credential_type: CredentialType | str) -> list[BasicProviderConfig]:
        """
        Get credential schema config by credential type
        """
        return [x.to_basic_provider_config() for x in self.get_credentials_schema(credential_type)]

    def get_oauth_client_schema(self) -> list[ProviderConfig]:
        """
        Get OAuth client schema for this provider

        :return: List of OAuth client config schemas
        """
        subscription_constructor = self.entity.subscription_constructor
        return (
            subscription_constructor.oauth_schema.client_schema.copy()
            if subscription_constructor and subscription_constructor.oauth_schema
            else []
        )

    def get_properties_schema(self) -> list[BasicProviderConfig]:
        """
        Get properties schema for this provider

        :return: List of properties config schemas
        """
        return (
            [x.to_basic_provider_config() for x in self.entity.subscription_schema.copy()]
            if self.entity.subscription_schema
            else []
        )

    def get_event_parameters(self, event_name: str) -> Mapping[str, EventParameter]:
        """
        Get event parameters for this provider
        """
        event = self.get_event(event_name)
        if not event:
            return {}
        return {parameter.name: parameter for parameter in event.parameters}

    def dispatch(
        self,
        request: Request,
        subscription: Subscription,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> TriggerDispatchResponse:
        """
        Dispatch a trigger through plugin runtime

        :param user_id: User ID
        :param request: Flask request object
        :param subscription: Subscription
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Dispatch response with triggers and raw HTTP response
        """
        manager = PluginTriggerClient()
        provider_id: TriggerProviderID = self.get_provider_id()

        response: TriggerDispatchResponse = manager.dispatch_event(
            tenant_id=self.tenant_id,
            provider=str(provider_id),
            subscription=subscription.model_dump(),
            request=request,
            credentials=credentials,
            credential_type=credential_type,
        )
        return response

    def invoke_trigger_event(
        self,
        user_id: str,
        event_name: str,
        parameters: Mapping[str, Any],
        credentials: Mapping[str, str],
        credential_type: CredentialType,
        subscription: Subscription,
        request: Request,
        payload: Mapping[str, Any],
    ) -> TriggerInvokeEventResponse:
        """
        Execute a trigger through plugin runtime

        :param user_id: User ID
        :param event_name: Event name
        :param parameters: Trigger parameters
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :param request: Request
        :param payload: Payload
        :return: Trigger execution result
        """
        manager = PluginTriggerClient()
        provider_id: TriggerProviderID = self.get_provider_id()

        return manager.invoke_trigger_event(
            tenant_id=self.tenant_id,
            user_id=user_id,
            provider=str(provider_id),
            event_name=event_name,
            credentials=credentials,
            credential_type=credential_type,
            request=request,
            parameters=parameters,
            subscription=subscription,
            payload=payload,
        )

    def subscribe_trigger(
        self,
        user_id: str,
        endpoint: str,
        parameters: Mapping[str, Any],
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> Subscription:
        """
        Subscribe to a trigger through plugin runtime

        :param user_id: User ID
        :param endpoint: Subscription endpoint
        :param subscription_params: Subscription parameters
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Subscription result
        """
        manager = PluginTriggerClient()
        provider_id: TriggerProviderID = self.get_provider_id()

        response: TriggerSubscriptionResponse = manager.subscribe(
            tenant_id=self.tenant_id,
            user_id=user_id,
            provider=str(provider_id),
            endpoint=endpoint,
            parameters=parameters,
            credentials=credentials,
            credential_type=credential_type,
        )

        return Subscription.model_validate(response.subscription)

    def unsubscribe_trigger(
        self, user_id: str, subscription: Subscription, credentials: Mapping[str, str], credential_type: CredentialType
    ) -> UnsubscribeResult:
        """
        Unsubscribe from a trigger through plugin runtime

        :param user_id: User ID
        :param subscription: Subscription metadata
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Unsubscribe result
        """
        manager = PluginTriggerClient()
        provider_id: TriggerProviderID = self.get_provider_id()

        response: TriggerSubscriptionResponse = manager.unsubscribe(
            tenant_id=self.tenant_id,
            user_id=user_id,
            provider=str(provider_id),
            subscription=subscription,
            credentials=credentials,
            credential_type=credential_type,
        )

        return UnsubscribeResult.model_validate(response.subscription)

    def refresh_trigger(
        self, subscription: Subscription, credentials: Mapping[str, str], credential_type: CredentialType
    ) -> Subscription:
        """
        Refresh a trigger subscription through plugin runtime

        :param subscription: Subscription metadata
        :param credentials: Provider credentials
        :return: Refreshed subscription result
        """
        manager = PluginTriggerClient()
        provider_id: TriggerProviderID = self.get_provider_id()

        response: TriggerSubscriptionResponse = manager.refresh(
            tenant_id=self.tenant_id,
            user_id="system",  # System refresh
            provider=str(provider_id),
            subscription=subscription,
            credentials=credentials,
            credential_type=credential_type,
        )

        return Subscription.model_validate(response.subscription)


__all__ = ["PluginTriggerProviderController"]

```

### api/core/trigger/constants.py
```py
from typing import Final

TRIGGER_WEBHOOK_NODE_TYPE: Final[str] = "trigger-webhook"
TRIGGER_SCHEDULE_NODE_TYPE: Final[str] = "trigger-schedule"
TRIGGER_PLUGIN_NODE_TYPE: Final[str] = "trigger-plugin"

TRIGGER_NODE_TYPES: Final[frozenset[str]] = frozenset(
    (
        TRIGGER_WEBHOOK_NODE_TYPE,
        TRIGGER_SCHEDULE_NODE_TYPE,
        TRIGGER_PLUGIN_NODE_TYPE,
    )
)


def is_trigger_node_type(node_type: str) -> bool:
    return node_type in TRIGGER_NODE_TYPES

```

### api/core/trigger/__init__.py
```py
# Core trigger module initialization

```

### api/core/trigger/utils/encryption.py
```py
from collections.abc import Mapping
from typing import Union

from core.entities.provider_entities import BasicProviderConfig, ProviderConfig
from core.helper.provider_cache import ProviderCredentialsCache
from core.helper.provider_encryption import ProviderConfigCache, ProviderConfigEncrypter, create_provider_encrypter
from core.plugin.entities.plugin_daemon import CredentialType
from core.trigger.entities.api_entities import TriggerProviderSubscriptionApiEntity
from core.trigger.provider import PluginTriggerProviderController
from models.trigger import TriggerSubscription


class TriggerProviderCredentialsCache(ProviderCredentialsCache):
    """Cache for trigger provider credentials"""

    def __init__(self, tenant_id: str, provider_id: str, credential_id: str):
        super().__init__(tenant_id=tenant_id, provider_id=provider_id, credential_id=credential_id)

    def _generate_cache_key(self, **kwargs) -> str:
        tenant_id = kwargs["tenant_id"]
        provider_id = kwargs["provider_id"]
        credential_id = kwargs["credential_id"]
        return f"trigger_credentials:tenant_id:{tenant_id}:provider_id:{provider_id}:credential_id:{credential_id}"


class TriggerProviderOAuthClientParamsCache(ProviderCredentialsCache):
    """Cache for trigger provider OAuth client"""

    def __init__(self, tenant_id: str, provider_id: str):
        super().__init__(tenant_id=tenant_id, provider_id=provider_id)

    def _generate_cache_key(self, **kwargs) -> str:
        tenant_id = kwargs["tenant_id"]
        provider_id = kwargs["provider_id"]
        return f"trigger_oauth_client:tenant_id:{tenant_id}:provider_id:{provider_id}"


class TriggerProviderPropertiesCache(ProviderCredentialsCache):
    """Cache for trigger provider properties"""

    def __init__(self, tenant_id: str, provider_id: str, subscription_id: str):
        super().__init__(tenant_id=tenant_id, provider_id=provider_id, subscription_id=subscription_id)

    def _generate_cache_key(self, **kwargs) -> str:
        tenant_id = kwargs["tenant_id"]
        provider_id = kwargs["provider_id"]
        subscription_id = kwargs["subscription_id"]
        return f"trigger_properties:tenant_id:{tenant_id}:provider_id:{provider_id}:subscription_id:{subscription_id}"


def create_trigger_provider_encrypter_for_subscription(
    tenant_id: str,
    controller: PluginTriggerProviderController,
    subscription: Union[TriggerSubscription, TriggerProviderSubscriptionApiEntity],
) -> tuple[ProviderConfigEncrypter, ProviderConfigCache]:
    cache = TriggerProviderCredentialsCache(
        tenant_id=tenant_id,
        provider_id=str(controller.get_provider_id()),
        credential_id=subscription.id,
    )
    encrypter, _ = create_provider_encrypter(
        tenant_id=tenant_id,
        config=controller.get_credential_schema_config(subscription.credential_type),
        cache=cache,
    )
    return encrypter, cache


def delete_cache_for_subscription(tenant_id: str, provider_id: str, subscription_id: str):
    TriggerProviderCredentialsCache(
        tenant_id=tenant_id,
        provider_id=provider_id,
        credential_id=subscription_id,
    ).delete()
    TriggerProviderPropertiesCache(
        tenant_id=tenant_id,
        provider_id=provider_id,
        subscription_id=subscription_id,
    ).delete()


def create_trigger_provider_encrypter_for_properties(
    tenant_id: str,
    controller: PluginTriggerProviderController,
    subscription: Union[TriggerSubscription, TriggerProviderSubscriptionApiEntity],
) -> tuple[ProviderConfigEncrypter, ProviderConfigCache]:
    cache = TriggerProviderPropertiesCache(
        tenant_id=tenant_id,
        provider_id=str(controller.get_provider_id()),
        subscription_id=subscription.id,
    )
    encrypter, _ = create_provider_encrypter(
        tenant_id=tenant_id,
        config=controller.get_properties_schema(),
        cache=cache,
    )
    return encrypter, cache


def create_trigger_provider_encrypter(
    tenant_id: str, controller: PluginTriggerProviderController, credential_id: str, credential_type: CredentialType
) -> tuple[ProviderConfigEncrypter, ProviderConfigCache]:
    cache = TriggerProviderCredentialsCache(
        tenant_id=tenant_id,
        provider_id=str(controller.get_provider_id()),
        credential_id=credential_id,
    )
    encrypter, _ = create_provider_encrypter(
        tenant_id=tenant_id,
        config=controller.get_credential_schema_config(credential_type),
        cache=cache,
    )
    return encrypter, cache


def create_trigger_provider_oauth_encrypter(
    tenant_id: str, controller: PluginTriggerProviderController
) -> tuple[ProviderConfigEncrypter, ProviderConfigCache]:
    cache = TriggerProviderOAuthClientParamsCache(
        tenant_id=tenant_id,
        provider_id=str(controller.get_provider_id()),
    )
    encrypter, _ = create_provider_encrypter(
        tenant_id=tenant_id,
        config=[x.to_basic_provider_config() for x in controller.get_oauth_client_schema()],
        cache=cache,
    )
    return encrypter, cache


def masked_credentials(
    schemas: list[ProviderConfig],
    credentials: Mapping[str, str],
) -> Mapping[str, str]:
    masked_credentials = {}
    configs = {x.name: x.to_basic_provider_config() for x in schemas}
    for key, value in credentials.items():
        config = configs.get(key)
        if not config:
            masked_credentials[key] = value
            continue
        if config.type == BasicProviderConfig.Type.SECRET_INPUT:
            if len(value) <= 4:
                masked_credentials[key] = "*" * len(value)
            else:
                masked_credentials[key] = value[:2] + "*" * (len(value) - 4) + value[-2:]
        else:
            masked_credentials[key] = value
    return masked_credentials

```

### api/core/trigger/utils/locks.py
```py
from collections.abc import Sequence
from itertools import starmap


def build_trigger_refresh_lock_key(tenant_id: str, subscription_id: str) -> str:
    """Build the Redis lock key for trigger subscription refresh in-flight protection."""
    return f"trigger_provider_refresh_lock:{tenant_id}_{subscription_id}"


def build_trigger_refresh_lock_keys(pairs: Sequence[tuple[str, str]]) -> list[str]:
    """Build Redis lock keys for a sequence of (tenant_id, subscription_id) pairs."""
    return list(starmap(build_trigger_refresh_lock_key, pairs))

```

### api/core/trigger/utils/endpoint.py
```py
from yarl import URL

from configs import dify_config

"""
Basic URL for thirdparty trigger services
"""
base_url = URL(dify_config.TRIGGER_URL)


def generate_plugin_trigger_endpoint_url(endpoint_id: str) -> str:
    """
    Generate url for plugin trigger endpoint url
    """

    return str(base_url / "triggers" / "plugin" / endpoint_id)


def generate_webhook_trigger_endpoint(webhook_id: str, debug: bool = False) -> str:
    """
    Generate url for webhook trigger endpoint url
    """

    return str(base_url / "triggers" / ("webhook-debug" if debug else "webhook") / webhook_id)

```

### api/core/trigger/errors.py
```py
from core.plugin.impl.exc import PluginInvokeError


class TriggerProviderCredentialValidationError(ValueError):
    pass


class TriggerPluginInvokeError(PluginInvokeError):
    pass


class TriggerInvokeError(PluginInvokeError):
    pass


class EventIgnoreError(TriggerInvokeError):
    """
    Trigger event ignore error
    """

```

### api/core/trigger/trigger_manager.py
```py
"""
Trigger Manager for loading and managing trigger providers and triggers
"""

import logging
from collections.abc import Mapping
from threading import Lock
from typing import Any

from flask import Request

import contexts
from configs import dify_config
from core.plugin.entities.plugin_daemon import CredentialType, PluginTriggerProviderEntity
from core.plugin.entities.request import TriggerInvokeEventResponse
from core.plugin.impl.exc import PluginDaemonError, PluginNotFoundError
from core.plugin.impl.trigger import PluginTriggerClient
from core.trigger.entities.entities import (
    EventEntity,
    Subscription,
    UnsubscribeResult,
)
from core.trigger.errors import EventIgnoreError
from core.trigger.provider import PluginTriggerProviderController
from models.provider_ids import TriggerProviderID

logger = logging.getLogger(__name__)


class TriggerManager:
    """
    Manager for trigger providers and triggers
    """

    @classmethod
    def get_trigger_plugin_icon(cls, tenant_id: str, provider_id: str) -> str:
        """
        Get the icon of a trigger plugin
        """
        manager = PluginTriggerClient()
        provider: PluginTriggerProviderEntity = manager.fetch_trigger_provider(
            tenant_id=tenant_id, provider_id=TriggerProviderID(provider_id)
        )
        filename = provider.declaration.identity.icon
        base_url = f"{dify_config.CONSOLE_API_URL}/console/api/workspaces/current/plugin/icon"
        return f"{base_url}?tenant_id={tenant_id}&filename={filename}"

    @classmethod
    def list_plugin_trigger_providers(cls, tenant_id: str) -> list[PluginTriggerProviderController]:
        """
        List all plugin trigger providers for a tenant

        :param tenant_id: Tenant ID
        :return: List of trigger provider controllers
        """
        manager = PluginTriggerClient()
        provider_entities = manager.fetch_trigger_providers(tenant_id)

        controllers: list[PluginTriggerProviderController] = []
        for provider in provider_entities:
            try:
                controller = PluginTriggerProviderController(
                    entity=provider.declaration,
                    plugin_id=provider.plugin_id,
                    plugin_unique_identifier=provider.plugin_unique_identifier,
                    provider_id=TriggerProviderID(provider.provider),
                    tenant_id=tenant_id,
                )
                controllers.append(controller)
            except Exception:
                logger.exception("Failed to load trigger provider %s", provider.plugin_id)
                continue

        return controllers

    @classmethod
    def get_trigger_provider(cls, tenant_id: str, provider_id: TriggerProviderID) -> PluginTriggerProviderController:
        """
        Get a specific plugin trigger provider

        :param tenant_id: Tenant ID
        :param provider_id: Provider ID
        :return: Trigger provider controller or None
        """
        # check if context is set
        try:
            contexts.plugin_trigger_providers.get()
        except LookupError:
            contexts.plugin_trigger_providers.set({})
            contexts.plugin_trigger_providers_lock.set(Lock())

        plugin_trigger_providers = contexts.plugin_trigger_providers.get()
        provider_id_str = str(provider_id)
        if provider_id_str in plugin_trigger_providers:
            return plugin_trigger_providers[provider_id_str]

        with contexts.plugin_trigger_providers_lock.get():
            # double check
            plugin_trigger_providers = contexts.plugin_trigger_providers.get()
            if provider_id_str in plugin_trigger_providers:
                return plugin_trigger_providers[provider_id_str]

            try:
                manager = PluginTriggerClient()
                provider = manager.fetch_trigger_provider(tenant_id, provider_id)

                if not provider:
                    raise ValueError(f"Trigger provider {provider_id} not found")

                controller = PluginTriggerProviderController(
                    entity=provider.declaration,
                    plugin_id=provider.plugin_id,
                    plugin_unique_identifier=provider.plugin_unique_identifier,
                    provider_id=provider_id,
                    tenant_id=tenant_id,
                )
                plugin_trigger_providers[provider_id_str] = controller
                return controller
            except PluginNotFoundError as e:
                raise ValueError(f"Trigger provider {provider_id} not found") from e
            except PluginDaemonError as e:
                raise e
            except Exception as e:
                logger.exception("Failed to load trigger provider")
                raise e

    @classmethod
    def list_all_trigger_providers(cls, tenant_id: str) -> list[PluginTriggerProviderController]:
        """
        List all trigger providers (plugin)

        :param tenant_id: Tenant ID
        :return: List of all trigger provider controllers
        """
        return cls.list_plugin_trigger_providers(tenant_id)

    @classmethod
    def list_triggers_by_provider(cls, tenant_id: str, provider_id: TriggerProviderID) -> list[EventEntity]:
        """
        List all triggers for a specific provider

        :param tenant_id: Tenant ID
        :param provider_id: Provider ID
        :return: List of trigger entities
        """
        provider = cls.get_trigger_provider(tenant_id, provider_id)
        return provider.get_events()

    @classmethod
    def invoke_trigger_event(
        cls,
        tenant_id: str,
        user_id: str,
        provider_id: TriggerProviderID,
        event_name: str,
        parameters: Mapping[str, Any],
        credentials: Mapping[str, str],
        credential_type: CredentialType,
        subscription: Subscription,
        request: Request,
        payload: Mapping[str, Any],
    ) -> TriggerInvokeEventResponse:
        """
        Execute a trigger

        :param tenant_id: Tenant ID
        :param user_id: User ID
        :param provider_id: Provider ID
        :param event_name: Event name
        :param parameters: Trigger parameters
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :param subscription: Subscription
        :param request: Request
        :param payload: Payload
        :return: Trigger execution result
        """
        provider: PluginTriggerProviderController = cls.get_trigger_provider(
            tenant_id=tenant_id, provider_id=provider_id
        )
        try:
            return provider.invoke_trigger_event(
                user_id=user_id,
                event_name=event_name,
                parameters=parameters,
                credentials=credentials,
                credential_type=credential_type,
                subscription=subscription,
                request=request,
                payload=payload,
            )
        except EventIgnoreError:
            return TriggerInvokeEventResponse(variables={}, cancelled=True)
        except Exception as e:
            raise e

    @classmethod
    def subscribe_trigger(
        cls,
        tenant_id: str,
        user_id: str,
        provider_id: TriggerProviderID,
        endpoint: str,
        parameters: Mapping[str, Any],
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> Subscription:
        """
        Subscribe to a trigger (e.g., register webhook)

        :param tenant_id: Tenant ID
        :param user_id: User ID
        :param provider_id: Provider ID
        :param endpoint: Subscription endpoint
        :param parameters: Subscription parameters
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Subscription result
        """
        provider: PluginTriggerProviderController = cls.get_trigger_provider(
            tenant_id=tenant_id, provider_id=provider_id
        )
        return provider.subscribe_trigger(
            user_id=user_id,
            endpoint=endpoint,
            parameters=parameters,
            credentials=credentials,
            credential_type=credential_type,
        )

    @classmethod
    def unsubscribe_trigger(
        cls,
        tenant_id: str,
        user_id: str,
        provider_id: TriggerProviderID,
        subscription: Subscription,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> UnsubscribeResult:
        """
        Unsubscribe from a trigger

        :param tenant_id: Tenant ID
        :param user_id: User ID
        :param provider_id: Provider ID
        :param subscription: Subscription metadata from subscribe operation
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Unsubscription result
        """
        provider: PluginTriggerProviderController = cls.get_trigger_provider(
            tenant_id=tenant_id, provider_id=provider_id
        )
        return provider.unsubscribe_trigger(
            user_id=user_id,
            subscription=subscription,
            credentials=credentials,
            credential_type=credential_type,
        )

    @classmethod
    def refresh_trigger(
        cls,
        tenant_id: str,
        provider_id: TriggerProviderID,
        subscription: Subscription,
        credentials: Mapping[str, str],
        credential_type: CredentialType,
    ) -> Subscription:
        """
        Refresh a trigger subscription

        :param tenant_id: Tenant ID
        :param provider_id: Provider ID
        :param subscription: Subscription metadata from subscribe operation
        :param credentials: Provider credentials
        :param credential_type: Credential type
        :return: Refreshed subscription result
        """

        # TODO you should update the subscription using the return value of the refresh_trigger
        return cls.get_trigger_provider(tenant_id=tenant_id, provider_id=provider_id).refresh_trigger(
            subscription=subscription, credentials=credentials, credential_type=credential_type
        )

```

### api/core/trigger/entities/api_entities.py
```py
from collections.abc import Mapping
from typing import Any

from pydantic import BaseModel, Field

from core.entities.provider_entities import ProviderConfig
from core.plugin.entities.plugin_daemon import CredentialType
from core.tools.entities.common_entities import I18nObject
from core.trigger.entities.entities import (
    EventIdentity,
    EventParameter,
    SubscriptionConstructor,
    TriggerCreationMethod,
)


class TriggerProviderSubscriptionApiEntity(BaseModel):
    id: str = Field(description="The unique id of the subscription")
    name: str = Field(description="The name of the subscription")
    provider: str = Field(description="The provider id of the subscription")
    credential_type: CredentialType = Field(description="The type of the credential")
    credentials: dict[str, Any] = Field(description="The credentials of the subscription")
    endpoint: str = Field(description="The endpoint of the subscription")
    parameters: dict[str, Any] = Field(description="The parameters of the subscription")
    properties: dict[str, Any] = Field(description="The properties of the subscription")
    workflows_in_use: int = Field(description="The number of workflows using this subscription")


class EventApiEntity(BaseModel):
    name: str = Field(description="The name of the trigger")
    identity: EventIdentity = Field(description="The identity of the trigger")
    description: I18nObject = Field(description="The description of the trigger")
    parameters: list[EventParameter] = Field(description="The parameters of the trigger")
    output_schema: Mapping[str, Any] | None = Field(description="The output schema of the trigger")


class TriggerProviderApiEntity(BaseModel):
    author: str = Field(..., description="The author of the trigger provider")
    name: str = Field(..., description="The name of the trigger provider")
    label: I18nObject = Field(..., description="The label of the trigger provider")
    description: I18nObject = Field(..., description="The description of the trigger provider")
    icon: str | None = Field(default=None, description="The icon of the trigger provider")
    icon_dark: str | None = Field(default=None, description="The dark icon of the trigger provider")
    tags: list[str] = Field(default_factory=list, description="The tags of the trigger provider")

    plugin_id: str | None = Field(default="", description="The plugin id of the tool")
    plugin_unique_identifier: str | None = Field(default="", description="The unique identifier of the tool")

    supported_creation_methods: list[TriggerCreationMethod] = Field(
        default_factory=list,
        description="Supported creation methods for the trigger provider. like 'OAUTH', 'APIKEY', 'MANUAL'.",
    )

    subscription_constructor: SubscriptionConstructor | None = Field(
        default=None, description="The subscription constructor of the trigger provider"
    )

    subscription_schema: list[ProviderConfig] = Field(
        default_factory=list,
        description="The subscription schema of the trigger provider",
    )
    events: list[EventApiEntity] = Field(description="The events of the trigger provider")


class SubscriptionBuilderApiEntity(BaseModel):
    id: str = Field(description="The id of the subscription builder")
    name: str = Field(description="The name of the subscription builder")
    provider: str = Field(description="The provider id of the subscription builder")
    endpoint: str = Field(description="The endpoint id of the subscription builder")
    parameters: Mapping[str, Any] = Field(description="The parameters of the subscription builder")
    properties: Mapping[str, Any] = Field(description="The properties of the subscription builder")
    credentials: Mapping[str, str] = Field(description="The credentials of the subscription builder")
    credential_type: CredentialType = Field(description="The credential type of the subscription builder")


__all__ = ["EventApiEntity", "TriggerProviderApiEntity", "TriggerProviderSubscriptionApiEntity"]

```

### api/core/trigger/entities/entities.py
```py
from collections.abc import Mapping
from datetime import datetime
from enum import StrEnum
from typing import Any, Union

from pydantic import BaseModel, ConfigDict, Field, ValidationInfo, field_validator

from core.entities.provider_entities import ProviderConfig
from core.plugin.entities.parameters import (
    PluginParameterAutoGenerate,
    PluginParameterOption,
    PluginParameterTemplate,
    PluginParameterType,
)
from core.tools.entities.common_entities import I18nObject


class EventParameterType(StrEnum):
    """The type of the parameter"""

    STRING = PluginParameterType.STRING
    NUMBER = PluginParameterType.NUMBER
    BOOLEAN = PluginParameterType.BOOLEAN
    SELECT = PluginParameterType.SELECT
    FILE = PluginParameterType.FILE
    FILES = PluginParameterType.FILES
    MODEL_SELECTOR = PluginParameterType.MODEL_SELECTOR
    APP_SELECTOR = PluginParameterType.APP_SELECTOR
    OBJECT = PluginParameterType.OBJECT
    ARRAY = PluginParameterType.ARRAY
    DYNAMIC_SELECT = PluginParameterType.DYNAMIC_SELECT
    CHECKBOX = PluginParameterType.CHECKBOX


class EventParameter(BaseModel):
    """
    The parameter of the event
    """

    name: str = Field(..., description="The name of the parameter")
    label: I18nObject = Field(..., description="The label presented to the user")
    type: EventParameterType = Field(..., description="The type of the parameter")
    auto_generate: PluginParameterAutoGenerate | None = Field(
        default=None, description="The auto generate of the parameter"
    )
    template: PluginParameterTemplate | None = Field(default=None, description="The template of the parameter")
    scope: str | None = None
    required: bool | None = False
    multiple: bool | None = Field(
        default=False,
        description="Whether the parameter is multiple select, only valid for select or dynamic-select type",
    )
    default: Union[int, float, str, list[Any], None] = None
    min: Union[float, int, None] = None
    max: Union[float, int, None] = None
    precision: int | None = None
    options: list[PluginParameterOption] | None = None
    description: I18nObject | None = None


class TriggerProviderIdentity(BaseModel):
    """
    The identity of the trigger provider
    """

    author: str = Field(..., description="The author of the trigger provider")
    name: str = Field(..., description="The name of the trigger provider")
    label: I18nObject = Field(..., description="The label of the trigger provider")
    description: I18nObject = Field(..., description="The description of the trigger provider")
    icon: str | None = Field(default=None, description="The icon of the trigger provider")
    icon_dark: str | None = Field(default=None, description="The dark icon of the trigger provider")
    tags: list[str] = Field(default_factory=list, description="The tags of the trigger provider")

    @field_validator("tags", mode="before")
    @classmethod
    def validate_tags(cls, v: list[str] | None) -> list[str]:
        return v or []


class EventIdentity(BaseModel):
    """
    The identity of the event
    """

    author: str = Field(..., description="The author of the event")
    name: str = Field(..., description="The name of the event")
    label: I18nObject = Field(..., description="The label of the event")
    provider: str | None = Field(default=None, description="The provider of the event")


class EventEntity(BaseModel):
    """
    The configuration of an event
    """

    identity: EventIdentity = Field(..., description="The identity of the event")
    parameters: list[EventParameter] = Field(
        default_factory=list[EventParameter], description="The parameters of the event"
    )
    description: I18nObject = Field(..., description="The description of the event")
    output_schema: Mapping[str, Any] | None = Field(
        default=None, description="The output schema that this event produces"
    )

    @field_validator("parameters", mode="before")
    @classmethod
    def set_parameters(cls, v, validation_info: ValidationInfo) -> list[EventParameter]:
        return v or []


class OAuthSchema(BaseModel):
    client_schema: list[ProviderConfig] = Field(default_factory=list, description="The schema of the OAuth client")
    credentials_schema: list[ProviderConfig] = Field(
        default_factory=list, description="The schema of the OAuth credentials"
    )


class SubscriptionConstructor(BaseModel):
    """
    The subscription constructor of the trigger provider
    """

    parameters: list[EventParameter] = Field(
        default_factory=list, description="The parameters schema of the subscription constructor"
    )

    credentials_schema: list[ProviderConfig] = Field(
        default_factory=list,
        description="The credentials schema of the subscription constructor",
    )

    oauth_schema: OAuthSchema | None = Field(
        default=None,
        description="The OAuth schema of the subscription constructor if OAuth is supported",
    )

    def get_default_parameters(self) -> Mapping[str, Any]:
        """Get the default parameters from the parameters schema"""
        if not self.parameters:
            return {}
        return {param.name: param.default for param in self.parameters if param.default}


class TriggerProviderEntity(BaseModel):
    """
    The configuration of a trigger provider
    """

    identity: TriggerProviderIdentity = Field(..., description="The identity of the trigger provider")
    subscription_schema: list[ProviderConfig] = Field(
        default_factory=list,
        description="The configuration schema stored in the subscription entity",
    )
    subscription_constructor: SubscriptionConstructor | None = Field(
        default=None,
        description="The subscription constructor of the trigger provider",
    )
    events: list[EventEntity] = Field(default_factory=list, description="The events of the trigger provider")


class Subscription(BaseModel):
    """
    Result of a successful trigger subscription operation.

    Contains all information needed to manage the subscription lifecycle.
    """

    expires_at: int = Field(
        ..., description="The timestamp when the subscription will expire, this for refresh the subscription"
    )

    endpoint: str = Field(..., description="The webhook endpoint URL allocated by Dify for receiving events")
    parameters: Mapping[str, Any] = Field(
        default_factory=dict, description="The parameters of the subscription constructor"
    )
    properties: Mapping[str, Any] = Field(
        ..., description="Subscription data containing all properties and provider-specific information"
    )


class UnsubscribeResult(BaseModel):
    """
    Result of a trigger unsubscription operation.

    Provides detailed information about the unsubscription attempt,
    including success status and error details if failed.
    """

    success: bool = Field(..., description="Whether the unsubscription was successful")

    message: str | None = Field(
        None,
        description="Human-readable message about the operation result. "
        "Success message for successful operations, "
        "detailed error information for failures.",
    )


class RequestLog(BaseModel):
    id: str = Field(..., description="The id of the request log")
    endpoint: str = Field(..., description="The endpoint of the request log")
    request: dict[str, Any] = Field(..., description="The request of the request log")
    response: dict[str, Any] = Field(..., description="The response of the request log")
    created_at: datetime = Field(..., description="The created at of the request log")


class SubscriptionBuilder(BaseModel):
    id: str = Field(..., description="The id of the subscription builder")
    name: str | None = Field(default=None, description="The name of the subscription builder")
    tenant_id: str = Field(..., description="The tenant id of the subscription builder")
    user_id: str = Field(..., description="The user id of the subscription builder")
    provider_id: str = Field(..., description="The provider id of the subscription builder")
    endpoint_id: str = Field(..., description="The endpoint id of the subscription builder")
    parameters: Mapping[str, Any] = Field(..., description="The parameters of the subscription builder")
    properties: Mapping[str, Any] = Field(..., description="The properties of the subscription builder")
    credentials: Mapping[str, Any] = Field(..., description="The credentials of the subscription builder")
    credential_type: str | None = Field(default=None, description="The credential type of the subscription builder")
    credential_expires_at: int | None = Field(
        default=None, description="The credential expires at of the subscription builder"
    )
    expires_at: int = Field(..., description="The expires at of the subscription builder")

    def to_subscription(self) -> Subscription:
        return Subscription(
            expires_at=self.expires_at,
            endpoint=self.endpoint_id,
            properties=self.properties,
        )


class SubscriptionBuilderUpdater(BaseModel):
    name: str | None = Field(default=None, description="The name of the subscription builder")
    parameters: Mapping[str, Any] | None = Field(default=None, description="The parameters of the subscription builder")
    properties: Mapping[str, Any] | None = Field(default=None, description="The properties of the subscription builder")
    credentials: Mapping[str, Any] | None = Field(
        default=None, description="The credentials of the subscription builder"
    )
    credential_type: str | None = Field(default=None, description="The credential type of the subscription builder")
    credential_expires_at: int | None = Field(
        default=None, description="The credential expires at of the subscription builder"
    )
    expires_at: int | None = Field(default=None, description="The expires at of the subscription builder")

    def update(self, subscription_builder: SubscriptionBuilder) -> None:
        if self.name is not None:
            subscription_builder.name = self.name
        if self.parameters is not None:
            subscription_builder.parameters = self.parameters
        if self.properties is not None:
            subscription_builder.properties = self.properties
        if self.credentials is not None:
            subscription_builder.credentials = self.credentials
        if self.credential_type is not None:
            subscription_builder.credential_type = self.credential_type
        if self.credential_expires_at is not None:
            subscription_builder.credential_expires_at = self.credential_expires_at
        if self.expires_at is not None:
            subscription_builder.expires_at = self.expires_at


class TriggerEventData(BaseModel):
    """Event data dispatched to trigger sessions."""

    subscription_id: str
    events: list[str]
    request_id: str
    timestamp: float

    model_config = ConfigDict(arbitrary_types_allowed=True)


class TriggerCreationMethod(StrEnum):
    OAUTH = "OAUTH"
    APIKEY = "APIKEY"
    MANUAL = "MANUAL"


# Export all entities
__all__: list[str] = [
    "EventEntity",
    "EventIdentity",
    "EventParameter",
    "EventParameterType",
    "OAuthSchema",
    "RequestLog",
    "Subscription",
    "SubscriptionBuilder",
    "TriggerCreationMethod",
    "TriggerEventData",
    "TriggerProviderEntity",
    "TriggerProviderIdentity",
    "UnsubscribeResult",
]

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-020.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
