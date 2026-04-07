You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-016
- **Kind**: endpoint
- **Identifier**: Celery Async Task Workers (Document Indexing, App Generation)
- **Description**: Celery workers processing async tasks including document indexing and app generation. Risk of deserialization attacks via task payloads, resource exhaustion from unbounded task queues, and privilege escalation if workers run with elevated permissions.
- **Locations**: api/tasks/, api/tasks/annotation/, api/tasks/app_generate/, api/tasks/rag_pipeline/, api/celery_entrypoint.py

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

### api/tasks/rag_pipeline/priority_rag_pipeline_run_task.py
```py
import contextvars
import json
import logging
import time
import uuid
from collections.abc import Mapping
from concurrent.futures import ThreadPoolExecutor
from typing import Any

import click
from celery import shared_task  # type: ignore
from flask import current_app, g
from sqlalchemy.orm import Session, sessionmaker

from configs import dify_config
from core.app.entities.app_invoke_entities import InvokeFrom, RagPipelineGenerateEntity
from core.app.entities.rag_pipeline_invoke_entities import RagPipelineInvokeEntity
from core.rag.pipeline.queue import TenantIsolatedTaskQueue
from core.repositories.factory import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models import Account, Tenant
from models.dataset import Pipeline
from models.enums import WorkflowRunTriggeredFrom
from models.workflow import Workflow, WorkflowNodeExecutionTriggeredFrom
from services.file_service import FileService

logger = logging.getLogger(__name__)


@shared_task(queue="priority_pipeline")
def priority_rag_pipeline_run_task(
    rag_pipeline_invoke_entities_file_id: str,
    tenant_id: str,
):
    """
    Async Run rag pipeline task using high priority queue.

    :param rag_pipeline_invoke_entities_file_id: File ID containing serialized RAG pipeline invoke entities
    :param tenant_id: Tenant ID for the pipeline execution
    """
    # run with threading, thread pool size is 10

    try:
        start_at = time.perf_counter()
        rag_pipeline_invoke_entities_content = FileService(db.engine).get_file_content(
            rag_pipeline_invoke_entities_file_id
        )
        rag_pipeline_invoke_entities = json.loads(rag_pipeline_invoke_entities_content)

        logger.info("tenant %s received %d rag pipeline invoke entities", tenant_id, len(rag_pipeline_invoke_entities))

        # Get Flask app object for thread context
        flask_app = current_app._get_current_object()  # type: ignore

        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = []
            for rag_pipeline_invoke_entity in rag_pipeline_invoke_entities:
                # Submit task to thread pool with Flask app
                future = executor.submit(run_single_rag_pipeline_task, rag_pipeline_invoke_entity, flask_app)
                futures.append(future)

            # Wait for all tasks to complete
            for future in futures:
                try:
                    future.result()  # This will raise any exceptions that occurred in the thread
                except Exception:
                    logging.exception("Error in pipeline task")
        end_at = time.perf_counter()
        logging.info(
            click.style(
                f"tenant_id: {tenant_id}, Rag pipeline run completed. Latency: {end_at - start_at}s", fg="green"
            )
        )
    except Exception:
        logging.exception(click.style(f"Error running rag pipeline, tenant_id: {tenant_id}", fg="red"))
        raise
    finally:
        tenant_isolated_task_queue = TenantIsolatedTaskQueue(tenant_id, "pipeline")

        # Check if there are waiting tasks in the queue
        # Use rpop to get the next task from the queue (FIFO order)
        next_file_ids = tenant_isolated_task_queue.pull_tasks(count=dify_config.TENANT_ISOLATED_TASK_CONCURRENCY)
        logger.info("priority rag pipeline tenant isolation queue %s next files: %s", tenant_id, next_file_ids)

        if next_file_ids:
            for next_file_id in next_file_ids:
                # Process the next waiting task
                # Keep the flag set to indicate a task is running
                tenant_isolated_task_queue.set_task_waiting_time()
                priority_rag_pipeline_run_task.delay(  # type: ignore
                    rag_pipeline_invoke_entities_file_id=next_file_id.decode("utf-8")
                    if isinstance(next_file_id, bytes)
                    else next_file_id,
                    tenant_id=tenant_id,
                )
        else:
            # No more waiting tasks, clear the flag
            tenant_isolated_task_queue.delete_task_key()
        file_service = FileService(db.engine)
        file_service.delete_file(rag_pipeline_invoke_entities_file_id)
        db.session.close()


def run_single_rag_pipeline_task(rag_pipeline_invoke_entity: Mapping[str, Any], flask_app):
    """Run a single RAG pipeline task within Flask app context."""
    # Create Flask application context for this thread
    with flask_app.app_context():
        try:
            rag_pipeline_invoke_entity_model = RagPipelineInvokeEntity.model_validate(rag_pipeline_invoke_entity)
            user_id = rag_pipeline_invoke_entity_model.user_id
            tenant_id = rag_pipeline_invoke_entity_model.tenant_id
            pipeline_id = rag_pipeline_invoke_entity_model.pipeline_id
            workflow_id = rag_pipeline_invoke_entity_model.workflow_id
            streaming = rag_pipeline_invoke_entity_model.streaming
            workflow_execution_id = rag_pipeline_invoke_entity_model.workflow_execution_id
            workflow_thread_pool_id = rag_pipeline_invoke_entity_model.workflow_thread_pool_id
            application_generate_entity = rag_pipeline_invoke_entity_model.application_generate_entity

            with Session(db.engine, expire_on_commit=False) as session:
                # Load required entities
                account = session.query(Account).where(Account.id == user_id).first()
                if not account:
                    raise ValueError(f"Account {user_id} not found")

                tenant = session.query(Tenant).where(Tenant.id == tenant_id).first()
                if not tenant:
                    raise ValueError(f"Tenant {tenant_id} not found")
                account.current_tenant = tenant

                pipeline = session.query(Pipeline).where(Pipeline.id == pipeline_id).first()
                if not pipeline:
                    raise ValueError(f"Pipeline {pipeline_id} not found")

                workflow = session.query(Workflow).where(Workflow.id == pipeline.workflow_id).first()
                if not workflow:
                    raise ValueError(f"Workflow {pipeline.workflow_id} not found")

                if workflow_execution_id is None:
                    workflow_execution_id = str(uuid.uuid4())

                # Create application generate entity from dict
                entity = RagPipelineGenerateEntity.model_validate(application_generate_entity)

                # Create workflow repositories
                session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
                workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
                    session_factory=session_factory,
                    user=account,
                    app_id=entity.app_config.app_id,
                    triggered_from=WorkflowRunTriggeredFrom.RAG_PIPELINE_RUN,
                )

                workflow_node_execution_repository = (
                    DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
                        session_factory=session_factory,
                        user=account,
                        app_id=entity.app_config.app_id,
                        triggered_from=WorkflowNodeExecutionTriggeredFrom.RAG_PIPELINE_RUN,
                    )
                )

            # Set the user directly in g for preserve_flask_contexts
            g._login_user = account

            # Copy context for passing to pipeline generator
            context = contextvars.copy_context()

            # Direct execution without creating another thread
            # Since we're already in a thread pool, no need for nested threading
            from core.app.apps.pipeline.pipeline_generator import PipelineGenerator

            pipeline_generator = PipelineGenerator()
            # Using protected method intentionally for async execution
            pipeline_generator._generate(  # type: ignore[attr-defined]
                flask_app=flask_app,
                context=context,
                pipeline=pipeline,
                workflow_id=workflow_id,
                user=account,
                application_generate_entity=entity,
                invoke_from=InvokeFrom.PUBLISHED_PIPELINE,
                workflow_execution_repository=workflow_execution_repository,
                workflow_node_execution_repository=workflow_node_execution_repository,
                streaming=streaming,
                workflow_thread_pool_id=workflow_thread_pool_id,
            )
        except Exception:
            logging.exception("Error in priority pipeline task")
            raise

```

### api/tasks/rag_pipeline/rag_pipeline_run_task.py
```py
import contextvars
import json
import logging
import time
import uuid
from collections.abc import Mapping, Sequence
from concurrent.futures import ThreadPoolExecutor
from itertools import islice
from typing import Any

import click
from celery import group, shared_task
from flask import current_app, g
from sqlalchemy.orm import Session, sessionmaker

from configs import dify_config
from core.app.entities.app_invoke_entities import InvokeFrom, RagPipelineGenerateEntity
from core.app.entities.rag_pipeline_invoke_entities import RagPipelineInvokeEntity
from core.rag.pipeline.queue import TenantIsolatedTaskQueue
from core.repositories.factory import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models import Account, Tenant
from models.dataset import Pipeline
from models.enums import WorkflowRunTriggeredFrom
from models.workflow import Workflow, WorkflowNodeExecutionTriggeredFrom
from services.file_service import FileService

logger = logging.getLogger(__name__)


def chunked(iterable: Sequence, size: int):
    it = iter(iterable)
    return iter(lambda: list(islice(it, size)), [])


@shared_task(queue="pipeline")
def rag_pipeline_run_task(
    rag_pipeline_invoke_entities_file_id: str,
    tenant_id: str,
):
    """
    Async Run rag pipeline task using regular priority queue.

    :param rag_pipeline_invoke_entities_file_id: File ID containing serialized RAG pipeline invoke entities
    :param tenant_id: Tenant ID for the pipeline execution
    """
    # run with threading, thread pool size is 10

    try:
        start_at = time.perf_counter()
        rag_pipeline_invoke_entities_content = FileService(db.engine).get_file_content(
            rag_pipeline_invoke_entities_file_id
        )
        rag_pipeline_invoke_entities = json.loads(rag_pipeline_invoke_entities_content)

        logger.info("tenant %s received %d rag pipeline invoke entities", tenant_id, len(rag_pipeline_invoke_entities))

        # Get Flask app object for thread context
        flask_app = current_app._get_current_object()  # type: ignore

        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = []
            for rag_pipeline_invoke_entity in rag_pipeline_invoke_entities:
                # Submit task to thread pool with Flask app
                future = executor.submit(run_single_rag_pipeline_task, rag_pipeline_invoke_entity, flask_app)
                futures.append(future)

            # Wait for all tasks to complete
            for future in futures:
                try:
                    future.result()  # This will raise any exceptions that occurred in the thread
                except Exception:
                    logging.exception("Error in pipeline task")
        end_at = time.perf_counter()
        logging.info(
            click.style(
                f"tenant_id: {tenant_id}, Rag pipeline run completed. Latency: {end_at - start_at}s", fg="green"
            )
        )
    except Exception:
        logging.exception(click.style(f"Error running rag pipeline, tenant_id: {tenant_id}", fg="red"))
        raise
    finally:
        tenant_isolated_task_queue = TenantIsolatedTaskQueue(tenant_id, "pipeline")

        # Check if there are waiting tasks in the queue
        # Use rpop to get the next task from the queue (FIFO order)
        next_file_ids = tenant_isolated_task_queue.pull_tasks(count=dify_config.TENANT_ISOLATED_TASK_CONCURRENCY)
        logger.info("rag pipeline tenant isolation queue %s next files: %s", tenant_id, next_file_ids)

        if next_file_ids:
            for batch in chunked(next_file_ids, 100):
                jobs = []
                for next_file_id in batch:
                    tenant_isolated_task_queue.set_task_waiting_time()

                    file_id = (
                        next_file_id.decode("utf-8") if isinstance(next_file_id, (bytes, bytearray)) else next_file_id
                    )

                    jobs.append(
                        rag_pipeline_run_task.s(
                            rag_pipeline_invoke_entities_file_id=file_id,
                            tenant_id=tenant_id,
                        )
                    )

                if jobs:
                    group(jobs).apply_async()
        else:
            # No more waiting tasks, clear the flag
            tenant_isolated_task_queue.delete_task_key()
        file_service = FileService(db.engine)
        file_service.delete_file(rag_pipeline_invoke_entities_file_id)
        db.session.close()


def run_single_rag_pipeline_task(rag_pipeline_invoke_entity: Mapping[str, Any], flask_app):
    """Run a single RAG pipeline task within Flask app context."""
    # Create Flask application context for this thread
    with flask_app.app_context():
        try:
            rag_pipeline_invoke_entity_model = RagPipelineInvokeEntity.model_validate(rag_pipeline_invoke_entity)
            user_id = rag_pipeline_invoke_entity_model.user_id
            tenant_id = rag_pipeline_invoke_entity_model.tenant_id
            pipeline_id = rag_pipeline_invoke_entity_model.pipeline_id
            workflow_id = rag_pipeline_invoke_entity_model.workflow_id
            streaming = rag_pipeline_invoke_entity_model.streaming
            workflow_execution_id = rag_pipeline_invoke_entity_model.workflow_execution_id
            workflow_thread_pool_id = rag_pipeline_invoke_entity_model.workflow_thread_pool_id
            application_generate_entity = rag_pipeline_invoke_entity_model.application_generate_entity

            with Session(db.engine) as session:
                # Load required entities
                account = session.query(Account).where(Account.id == user_id).first()
                if not account:
                    raise ValueError(f"Account {user_id} not found")

                tenant = session.query(Tenant).where(Tenant.id == tenant_id).first()
                if not tenant:
                    raise ValueError(f"Tenant {tenant_id} not found")
                account.current_tenant = tenant

                pipeline = session.query(Pipeline).where(Pipeline.id == pipeline_id).first()
                if not pipeline:
                    raise ValueError(f"Pipeline {pipeline_id} not found")

                workflow = session.query(Workflow).where(Workflow.id == pipeline.workflow_id).first()
                if not workflow:
                    raise ValueError(f"Workflow {pipeline.workflow_id} not found")

                if workflow_execution_id is None:
                    workflow_execution_id = str(uuid.uuid4())

                # Create application generate entity from dict
                entity = RagPipelineGenerateEntity.model_validate(application_generate_entity)

                # Create workflow repositories
                session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
                workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
                    session_factory=session_factory,
                    user=account,
                    app_id=entity.app_config.app_id,
                    triggered_from=WorkflowRunTriggeredFrom.RAG_PIPELINE_RUN,
                )

                workflow_node_execution_repository = (
                    DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
                        session_factory=session_factory,
                        user=account,
                        app_id=entity.app_config.app_id,
                        triggered_from=WorkflowNodeExecutionTriggeredFrom.RAG_PIPELINE_RUN,
                    )
                )

                # Set the user directly in g for preserve_flask_contexts
                g._login_user = account

                # Copy context for passing to pipeline generator
                context = contextvars.copy_context()

                # Direct execution without creating another thread
                # Since we're already in a thread pool, no need for nested threading
                from core.app.apps.pipeline.pipeline_generator import PipelineGenerator

                pipeline_generator = PipelineGenerator()
                # Using protected method intentionally for async execution
                pipeline_generator._generate(  # type: ignore[attr-defined]
                    flask_app=flask_app,
                    context=context,
                    pipeline=pipeline,
                    workflow_id=workflow_id,
                    user=account,
                    application_generate_entity=entity,
                    invoke_from=InvokeFrom.PUBLISHED_PIPELINE,
                    workflow_execution_repository=workflow_execution_repository,
                    workflow_node_execution_repository=workflow_node_execution_repository,
                    streaming=streaming,
                    workflow_thread_pool_id=workflow_thread_pool_id,
                )
        except Exception:
            logging.exception("Error in pipeline task")
            raise

```

### api/tasks/regenerate_summary_index_task.py
```py
"""Task for regenerating summary indexes when dataset settings change."""

import logging
import time
from collections import defaultdict

import click
from celery import shared_task
from sqlalchemy import or_, select

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from models.dataset import Dataset, DocumentSegment, DocumentSegmentSummary
from models.dataset import Document as DatasetDocument
from services.summary_index_service import SummaryIndexService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset_summary")
def regenerate_summary_index_task(
    dataset_id: str,
    regenerate_reason: str = "summary_model_changed",
    regenerate_vectors_only: bool = False,
):
    """
    Regenerate summary indexes for all documents in a dataset.

    This task is triggered when:
    1. summary_index_setting model changes (regenerate_reason="summary_model_changed")
       - Regenerates summary content and vectors for all existing summaries
    2. embedding_model changes (regenerate_reason="embedding_model_changed")
       - Only regenerates vectors for existing summaries (keeps summary content)

    Args:
        dataset_id: Dataset ID
        regenerate_reason: Reason for regeneration ("summary_model_changed" or "embedding_model_changed")
        regenerate_vectors_only: If True, only regenerate vectors without regenerating summary content
    """
    logger.info(
        click.style(
            f"Start regenerate summary index for dataset {dataset_id}, reason: {regenerate_reason}",
            fg="green",
        )
    )
    start_at = time.perf_counter()

    try:
        with session_factory.create_session() as session:
            dataset = session.query(Dataset).filter_by(id=dataset_id).first()
            if not dataset:
                logger.error(click.style(f"Dataset not found: {dataset_id}", fg="red"))
                return

            # Only regenerate summary index for high_quality indexing technique
            if dataset.indexing_technique != IndexTechniqueType.HIGH_QUALITY:
                logger.info(
                    click.style(
                        f"Skipping summary regeneration for dataset {dataset_id}: "
                        f"indexing_technique is {dataset.indexing_technique}, not 'high_quality'",
                        fg="cyan",
                    )
                )
                return

            # Check if summary index is enabled (only for summary_model change)
            # For embedding_model change, we still re-vectorize existing summaries even if setting is disabled
            summary_index_setting = dataset.summary_index_setting
            if not regenerate_vectors_only:
                # For summary_model change, require summary_index_setting to be enabled
                if not summary_index_setting or not summary_index_setting.get("enable"):
                    logger.info(
                        click.style(
                            f"Summary index is disabled for dataset {dataset_id}",
                            fg="cyan",
                        )
                    )
                    return

            total_segments_processed = 0
            total_segments_failed = 0

            if regenerate_vectors_only:
                # For embedding_model change: directly query all segments with existing summaries
                # Don't require document indexing_status == "completed"
                # Include summaries with status "completed" or "error" (if they have content)
                segments_with_summaries = (
                    session.query(DocumentSegment, DocumentSegmentSummary)
                    .join(
                        DocumentSegmentSummary,
                        DocumentSegment.id == DocumentSegmentSummary.chunk_id,
                    )
                    .join(
                        DatasetDocument,
                        DocumentSegment.document_id == DatasetDocument.id,
                    )
                    .where(
                        DocumentSegment.dataset_id == dataset_id,
                        DocumentSegment.status == "completed",  # Segment must be completed
                        DocumentSegment.enabled == True,
                        DocumentSegmentSummary.dataset_id == dataset_id,
                        DocumentSegmentSummary.summary_content.isnot(None),  # Must have summary content
                        # Include completed summaries or error summaries (with content)
                        or_(
                            DocumentSegmentSummary.status == "completed",
                            DocumentSegmentSummary.status == "error",
                        ),
                        DatasetDocument.enabled == True,  # Document must be enabled
                        DatasetDocument.archived == False,  # Document must not be archived
                        DatasetDocument.doc_form != IndexStructureType.QA_INDEX,  # Skip qa_model documents
                    )
                    .order_by(DocumentSegment.document_id.asc(), DocumentSegment.position.asc())
                    .all()
                )

                if not segments_with_summaries:
                    logger.info(
                        click.style(
                            f"No segments with summaries found for re-vectorization in dataset {dataset_id}",
                            fg="cyan",
                        )
                    )
                    return

                logger.info(
                    "Found %s segments with summaries for re-vectorization in dataset %s",
                    len(segments_with_summaries),
                    dataset_id,
                )

                # Group by document for logging
                segments_by_document = defaultdict(list)
                for segment, summary_record in segments_with_summaries:
                    segments_by_document[segment.document_id].append((segment, summary_record))

                logger.info(
                    "Segments grouped into %s documents for re-vectorization",
                    len(segments_by_document),
                )

                for document_id, segment_summary_pairs in segments_by_document.items():
                    logger.info(
                        "Re-vectorizing summaries for %s segments in document %s",
                        len(segment_summary_pairs),
                        document_id,
                    )

                    for segment, summary_record in segment_summary_pairs:
                        try:
                            # Delete old vector
                            if summary_record.summary_index_node_id:
                                try:
                                    from core.rag.datasource.vdb.vector_factory import Vector

                                    vector = Vector(dataset)
                                    vector.delete_by_ids([summary_record.summary_index_node_id])
                                except Exception as e:
                                    logger.warning(
                                        "Failed to delete old summary vector for segment %s: %s",
                                        segment.id,
                                        str(e),
                                    )

                            # Re-vectorize with new embedding model
                            SummaryIndexService.vectorize_summary(summary_record, segment, dataset)
                            session.commit()
                            total_segments_processed += 1

                        except Exception as e:
                            logger.error(
                                "Failed to re-vectorize summary for segment %s: %s",
                                segment.id,
                                str(e),
                                exc_info=True,
                            )
                            total_segments_failed += 1
                            # Update summary record with error status
                            summary_record.status = "error"
                            summary_record.error = f"Re-vectorization failed: {str(e)}"
                            session.add(summary_record)
                            session.commit()
                            continue

            else:
                # For summary_model change: require document indexing_status == "completed"
                # Get all documents with completed indexing status
                dataset_documents = session.scalars(
                    select(DatasetDocument).where(
                        DatasetDocument.dataset_id == dataset_id,
                        DatasetDocument.indexing_status == "completed",
                        DatasetDocument.enabled == True,
                        DatasetDocument.archived == False,
                    )
                ).all()

                if not dataset_documents:
                    logger.info(
                        click.style(
                            f"No documents found for summary regeneration in dataset {dataset_id}",
                            fg="cyan",
                        )
                    )
                    return

                logger.info(
                    "Found %s documents for summary regeneration in dataset %s",
                    len(dataset_documents),
                    dataset_id,
                )

                for dataset_document in dataset_documents:
                    # Skip qa_model documents
                    if dataset_document.doc_form == IndexStructureType.QA_INDEX:
                        continue

                    try:
                        # Get all segments with existing summaries
                        segments = (
                            session.query(DocumentSegment)
                            .join(
                                DocumentSegmentSummary,
                                DocumentSegment.id == DocumentSegmentSummary.chunk_id,
                            )
                            .where(
                                DocumentSegment.document_id == dataset_document.id,
                                DocumentSegment.dataset_id == dataset_id,
                                DocumentSegment.status == "completed",
                                DocumentSegment.enabled == True,
                                DocumentSegmentSummary.dataset_id == dataset_id,
                            )
                            .order_by(DocumentSegment.position.asc())
                            .all()
                        )

                        if not segments:
                            continue

                        logger.info(
                            "Regenerating summaries for %s segments in document %s",
                            len(segments),
                            dataset_document.id,
                        )

                        for segment in segments:
                            summary_record = None
                            try:
                                # Get existing summary record
                                summary_record = (
                                    session.query(DocumentSegmentSummary)
                                    .filter_by(
                                        chunk_id=segment.id,
                                        dataset_id=dataset_id,
                                    )
                                    .first()
                                )

                                if not summary_record:
                                    logger.warning("Summary record not found for segment %s, skipping", segment.id)
                                    continue

                                # Regenerate both summary content and vectors (for summary_model change)
                                SummaryIndexService.generate_and_vectorize_summary(
                                    segment, dataset, summary_index_setting
                                )
                                session.commit()
                                total_segments_processed += 1

                            except Exception as e:
                                logger.error(
                                    "Failed to regenerate summary for segment %s: %s",
                                    segment.id,
                                    str(e),
                                    exc_info=True,
                                )
                                total_segments_failed += 1
                                # Update summary record with error status
                                if summary_record:
                                    summary_record.status = "error"
                                    summary_record.error = f"Regeneration failed: {str(e)}"
                                    session.add(summary_record)
                                    session.commit()
                                continue

                    except Exception as e:
                        logger.error(
                            "Failed to process document %s for summary regeneration: %s",
                            dataset_document.id,
                            str(e),
                            exc_info=True,
                        )
                        continue

            end_at = time.perf_counter()
            if regenerate_vectors_only:
                logger.info(
                    click.style(
                        f"Summary re-vectorization completed for dataset {dataset_id}: "
                        f"{total_segments_processed} segments processed successfully, "
                        f"{total_segments_failed} segments failed, "
                        f"latency: {end_at - start_at:.2f}s",
                        fg="green",
                    )
                )
            else:
                logger.info(
                    click.style(
                        f"Summary index regeneration completed for dataset {dataset_id}: "
                        f"{total_segments_processed} segments processed successfully, "
                        f"{total_segments_failed} segments failed, "
                        f"latency: {end_at - start_at:.2f}s",
                        fg="green",
                    )
                )

    except Exception:
        logger.exception("Regenerate summary index failed for dataset %s", dataset_id)

```

### api/tasks/workflow_draft_var_tasks.py
```py
"""
Celery tasks for asynchronous workflow execution storage operations.

These tasks provide asynchronous storage capabilities for workflow execution data,
improving performance by offloading storage operations to background workers.
"""

from celery import shared_task  # type: ignore[import-untyped]

from core.db.session_factory import session_factory
from services.workflow_draft_variable_service import DraftVarFileDeletion, WorkflowDraftVariableService


@shared_task(queue="workflow_draft_var", bind=True, max_retries=3, default_retry_delay=60)
def save_workflow_execution_task(
    self,
    deletions: list[DraftVarFileDeletion],
):
    with session_factory.create_session() as session, session.begin():
        srv = WorkflowDraftVariableService(session=session)
        srv.delete_workflow_draft_variable_file(deletions=deletions)

```

### api/tasks/mail_register_task.py
```py
import logging
import time

import click
from celery import shared_task

from configs import dify_config
from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_email_register_mail_task(language: str, to: str, code: str) -> None:
    """
    Send email register email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        code: Email register code
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start email register mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.EMAIL_REGISTER,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "code": code,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send email register mail to {to} succeeded: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send email register mail to %s failed", to)


@shared_task(queue="mail")
def send_email_register_mail_task_when_account_exist(language: str, to: str, account_name: str) -> None:
    """
    Send email register email with internationalization support when account exist.

    Args:
        language: Language code for email localization
        to: Recipient email address
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start email register mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        login_url = f"{dify_config.CONSOLE_WEB_URL}/signin"
        reset_password_url = f"{dify_config.CONSOLE_WEB_URL}/reset-password"

        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.EMAIL_REGISTER_WHEN_ACCOUNT_EXIST,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "login_url": login_url,
                "reset_password_url": reset_password_url,
                "account_name": account_name,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send email register mail to {to} succeeded: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send email register mail to %s failed", to)

```

### api/tasks/generate_summary_index_task.py
```py
"""Async task for generating summary indexes."""

import logging
import time

import click
from celery import shared_task

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from models.dataset import Dataset, DocumentSegment
from models.dataset import Document as DatasetDocument
from services.summary_index_service import SummaryIndexService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset_summary")
def generate_summary_index_task(dataset_id: str, document_id: str, segment_ids: list[str] | None = None):
    """
    Async generate summary index for document segments.

    Args:
        dataset_id: Dataset ID
        document_id: Document ID
        segment_ids: Optional list of specific segment IDs to process. If None, process all segments.

    Usage:
        generate_summary_index_task.delay(dataset_id, document_id)
        generate_summary_index_task.delay(dataset_id, document_id, segment_ids)
    """
    logger.info(
        click.style(
            f"Start generating summary index for document {document_id} in dataset {dataset_id}",
            fg="green",
        )
    )
    start_at = time.perf_counter()

    try:
        with session_factory.create_session() as session:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if not dataset:
                logger.error(click.style(f"Dataset not found: {dataset_id}", fg="red"))
                return

            document = session.query(DatasetDocument).where(DatasetDocument.id == document_id).first()
            if not document:
                logger.error(click.style(f"Document not found: {document_id}", fg="red"))
                return

            # Check if document needs summary
            if not document.need_summary:
                logger.info(
                    click.style(
                        f"Skipping summary generation for document {document_id}: need_summary is False",
                        fg="cyan",
                    )
                )
                return

            # Only generate summary index for high_quality indexing technique
            if dataset.indexing_technique != IndexTechniqueType.HIGH_QUALITY:
                logger.info(
                    click.style(
                        f"Skipping summary generation for dataset {dataset_id}: "
                        f"indexing_technique is {dataset.indexing_technique}, not 'high_quality'",
                        fg="cyan",
                    )
                )
                return

            # Check if summary index is enabled
            summary_index_setting = dataset.summary_index_setting
            if not summary_index_setting or not summary_index_setting.get("enable"):
                logger.info(
                    click.style(
                        f"Summary index is disabled for dataset {dataset_id}",
                        fg="cyan",
                    )
                )
                return

            # Determine if only parent chunks should be processed
            only_parent_chunks = dataset.chunk_structure == "parent_child_index"

            # Generate summaries
            summary_records = SummaryIndexService.generate_summaries_for_document(
                dataset=dataset,
                document=document,
                summary_index_setting=summary_index_setting,
                segment_ids=segment_ids,
                only_parent_chunks=only_parent_chunks,
            )

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"Summary index generation completed for document {document_id}: "
                    f"{len(summary_records)} summaries generated, latency: {end_at - start_at}",
                    fg="green",
                )
            )

    except Exception as e:
        logger.exception("Failed to generate summary index for document %s", document_id)
        # Update document segments with error status if needed
        if segment_ids:
            error_message = f"Summary generation failed: {str(e)}"
            with session_factory.create_session() as session:
                session.query(DocumentSegment).filter(
                    DocumentSegment.id.in_(segment_ids),
                    DocumentSegment.dataset_id == dataset_id,
                ).update(
                    {
                        DocumentSegment.error: error_message,
                    },
                    synchronize_session=False,
                )
                session.commit()

```

### api/tasks/delete_account_task.py
```py
import logging

from celery import shared_task

from configs import dify_config
from core.db.session_factory import session_factory
from models import Account
from services.billing_service import BillingService
from tasks.mail_account_deletion_task import send_deletion_success_task

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def delete_account_task(account_id):
    with session_factory.create_session() as session:
        account = session.query(Account).where(Account.id == account_id).first()
        try:
            if dify_config.BILLING_ENABLED:
                BillingService.delete_account(account_id)
        except Exception:
            logger.exception("Failed to delete account %s from billing service.", account_id)
            raise

        if not account:
            logger.error("Account %s not found.", account_id)
            return
        # send success email
        send_deletion_success_task.delay(account.email)

```

### api/tasks/enable_segments_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import AttachmentDocument, ChildDocument, Document
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, DocumentSegment
from models.dataset import Document as DatasetDocument

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def enable_segments_to_index_task(segment_ids: list, dataset_id: str, document_id: str):
    """
    Async enable segments to index
    :param segment_ids: list of segment ids
    :param dataset_id: dataset id
    :param document_id: document id

    Usage: enable_segments_to_index_task.delay(segment_ids, dataset_id, document_id)
    """
    start_at = time.perf_counter()
    with session_factory.create_session() as session:
        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if not dataset:
            logger.info(click.style(f"Dataset {dataset_id} not found, pass.", fg="cyan"))
            return

        dataset_document = session.query(DatasetDocument).where(DatasetDocument.id == document_id).first()

        if not dataset_document:
            logger.info(click.style(f"Document {document_id} not found, pass.", fg="cyan"))
            return
        if not dataset_document.enabled or dataset_document.archived or dataset_document.indexing_status != "completed":
            logger.info(click.style(f"Document {document_id} status is invalid, pass.", fg="cyan"))
            return
        # sync index processor
        index_processor = IndexProcessorFactory(dataset_document.doc_form).init_index_processor()

        segments = session.scalars(
            select(DocumentSegment).where(
                DocumentSegment.id.in_(segment_ids),
                DocumentSegment.dataset_id == dataset_id,
                DocumentSegment.document_id == document_id,
            )
        ).all()
        if not segments:
            logger.info(click.style(f"Segments not found: {segment_ids}", fg="cyan"))
            return

        try:
            documents = []
            multimodal_documents = []
            for segment in segments:
                document = Document(
                    page_content=segment.content,
                    metadata={
                        "doc_id": segment.index_node_id,
                        "doc_hash": segment.index_node_hash,
                        "document_id": document_id,
                        "dataset_id": dataset_id,
                    },
                )

                if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                    child_chunks = segment.get_child_chunks()
                    if child_chunks:
                        child_documents = []
                        for child_chunk in child_chunks:
                            child_document = ChildDocument(
                                page_content=child_chunk.content,
                                metadata={
                                    "doc_id": child_chunk.index_node_id,
                                    "doc_hash": child_chunk.index_node_hash,
                                    "document_id": document_id,
                                    "dataset_id": dataset_id,
                                },
                            )
                            child_documents.append(child_document)
                        document.children = child_documents

                if dataset.is_multimodal:
                    for attachment in segment.attachments:
                        multimodal_documents.append(
                            AttachmentDocument(
                                page_content=attachment["name"],
                                metadata={
                                    "doc_id": attachment["id"],
                                    "doc_hash": "",
                                    "document_id": segment.document_id,
                                    "dataset_id": segment.dataset_id,
                                    "doc_type": DocType.IMAGE,
                                },
                            )
                        )
                documents.append(document)
            # save vector index
            index_processor.load(dataset, documents, multimodal_documents=multimodal_documents)

            # Enable summary indexes for these segments
            from services.summary_index_service import SummaryIndexService

            segment_ids_list = [segment.id for segment in segments]
            try:
                SummaryIndexService.enable_summaries_for_segments(
                    dataset=dataset,
                    segment_ids=segment_ids_list,
                )
            except Exception as e:
                logger.warning("Failed to enable summaries for segments: %s", str(e))

            end_at = time.perf_counter()
            logger.info(click.style(f"Segments enabled to index latency: {end_at - start_at}", fg="green"))
        except Exception as e:
            logger.exception("enable segments to index failed")
            # update segment error msg
            session.query(DocumentSegment).where(
                DocumentSegment.id.in_(segment_ids),
                DocumentSegment.dataset_id == dataset_id,
                DocumentSegment.document_id == document_id,
            ).update(
                {
                    "error": str(e),
                    "status": "error",
                    "disabled_at": naive_utc_now(),
                    "enabled": False,
                }
            )
            session.commit()
        finally:
            for segment in segments:
                indexing_cache_key = f"segment_{segment.id}_indexing"
                redis_client.delete(indexing_cache_key)

```

### api/tasks/deal_dataset_index_update_task.py
```py
import logging
import time

import click
from celery import shared_task  # type: ignore

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import AttachmentDocument, ChildDocument, Document
from models.dataset import Dataset, DocumentSegment
from models.dataset import Document as DatasetDocument


@shared_task(queue="dataset")
def deal_dataset_index_update_task(dataset_id: str, action: str):
    """
    Async deal dataset from index
    :param dataset_id: dataset_id
    :param action: action
    Usage: deal_dataset_index_update_task.delay(dataset_id, action)
    """
    logging.info(click.style("Start deal dataset index update: {}".format(dataset_id), fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).filter_by(id=dataset_id).first()

            if not dataset:
                raise Exception("Dataset not found")
            index_type = dataset.doc_form or IndexStructureType.PARAGRAPH_INDEX
            index_processor = IndexProcessorFactory(index_type).init_index_processor()
            if action == "upgrade":
                dataset_documents = (
                    session.query(DatasetDocument)
                    .where(
                        DatasetDocument.dataset_id == dataset_id,
                        DatasetDocument.indexing_status == "completed",
                        DatasetDocument.enabled == True,
                        DatasetDocument.archived == False,
                    )
                    .all()
                )

                if dataset_documents:
                    dataset_documents_ids = [doc.id for doc in dataset_documents]
                    session.query(DatasetDocument).where(DatasetDocument.id.in_(dataset_documents_ids)).update(
                        {"indexing_status": "indexing"}, synchronize_session=False
                    )
                    session.commit()

                    for dataset_document in dataset_documents:
                        try:
                            # add from vector index
                            segments = (
                                session.query(DocumentSegment)
                                .where(
                                    DocumentSegment.document_id == dataset_document.id,
                                    DocumentSegment.enabled == True,
                                )
                                .order_by(DocumentSegment.position.asc())
                                .all()
                            )
                            if segments:
                                documents = []
                                for segment in segments:
                                    document = Document(
                                        page_content=segment.content,
                                        metadata={
                                            "doc_id": segment.index_node_id,
                                            "doc_hash": segment.index_node_hash,
                                            "document_id": segment.document_id,
                                            "dataset_id": segment.dataset_id,
                                        },
                                    )

                                    documents.append(document)
                                # save vector index
                                # clean keywords
                                index_processor.clean(dataset, None, with_keywords=True, delete_child_chunks=False)
                                index_processor.load(dataset, documents, with_keywords=False)
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "completed"}, synchronize_session=False
                            )
                            session.commit()
                        except Exception as e:
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "error", "error": str(e)}, synchronize_session=False
                            )
                            session.commit()
            elif action == "update":
                dataset_documents = (
                    session.query(DatasetDocument)
                    .where(
                        DatasetDocument.dataset_id == dataset_id,
                        DatasetDocument.indexing_status == "completed",
                        DatasetDocument.enabled == True,
                        DatasetDocument.archived == False,
                    )
                    .all()
                )
                # add new index
                if dataset_documents:
                    # update document status
                    dataset_documents_ids = [doc.id for doc in dataset_documents]
                    session.query(DatasetDocument).where(DatasetDocument.id.in_(dataset_documents_ids)).update(
                        {"indexing_status": "indexing"}, synchronize_session=False
                    )
                    session.commit()

                    # clean index
                    index_processor.clean(dataset, None, with_keywords=False, delete_child_chunks=False)

                    for dataset_document in dataset_documents:
                        # update from vector index
                        try:
                            segments = (
                                session.query(DocumentSegment)
                                .where(
                                    DocumentSegment.document_id == dataset_document.id,
                                    DocumentSegment.enabled == True,
                                )
                                .order_by(DocumentSegment.position.asc())
                                .all()
                            )
                            if segments:
                                documents = []
                                multimodal_documents = []
                                for segment in segments:
                                    document = Document(
                                        page_content=segment.content,
                                        metadata={
                                            "doc_id": segment.index_node_id,
                                            "doc_hash": segment.index_node_hash,
                                            "document_id": segment.document_id,
                                            "dataset_id": segment.dataset_id,
                                        },
                                    )
                                    if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                                        child_chunks = segment.get_child_chunks()
                                        if child_chunks:
                                            child_documents = []
                                            for child_chunk in child_chunks:
                                                child_document = ChildDocument(
                                                    page_content=child_chunk.content,
                                                    metadata={
                                                        "doc_id": child_chunk.index_node_id,
                                                        "doc_hash": child_chunk.index_node_hash,
                                                        "document_id": segment.document_id,
                                                        "dataset_id": segment.dataset_id,
                                                    },
                                                )
                                                child_documents.append(child_document)
                                            document.children = child_documents
                                    if dataset.is_multimodal:
                                        for attachment in segment.attachments:
                                            multimodal_documents.append(
                                                AttachmentDocument(
                                                    page_content=attachment["name"],
                                                    metadata={
                                                        "doc_id": attachment["id"],
                                                        "doc_hash": "",
                                                        "document_id": segment.document_id,
                                                        "dataset_id": segment.dataset_id,
                                                        "doc_type": DocType.IMAGE,
                                                    },
                                                )
                                            )
                                    documents.append(document)
                                # save vector index
                                index_processor.load(
                                    dataset, documents, multimodal_documents=multimodal_documents, with_keywords=False
                                )
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "completed"}, synchronize_session=False
                            )
                            session.commit()
                        except Exception as e:
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "error", "error": str(e)}, synchronize_session=False
                            )
                            session.commit()
                else:
                    # clean collection
                    index_processor.clean(dataset, None, with_keywords=False, delete_child_chunks=False)

            end_at = time.perf_counter()
            logging.info(
                click.style(
                    "Deal dataset vector index: {} latency: {}".format(dataset_id, end_at - start_at),
                    fg="green",
                )
            )
        except Exception:
            logging.exception("Deal dataset vector index failed")

```

### api/tasks/mail_inner_task.py
```py
import logging
import time
from collections.abc import Mapping
from typing import Any

import click
from celery import shared_task
from flask import render_template_string
from jinja2.runtime import Context
from jinja2.sandbox import ImmutableSandboxedEnvironment

from configs import dify_config
from configs.feature import TemplateMode
from extensions.ext_mail import mail
from libs.email_i18n import get_email_i18n_service

logger = logging.getLogger(__name__)


class SandboxedEnvironment(ImmutableSandboxedEnvironment):
    def __init__(self, timeout: int, *args: Any, **kwargs: Any):
        self._timeout_time = time.time() + timeout
        super().__init__(*args, **kwargs)

    def call(self, context: Context, obj: Any, *args: Any, **kwargs: Any) -> Any:
        if time.time() > self._timeout_time:
            raise TimeoutError("Template rendering timeout")
        return super().call(context, obj, *args, **kwargs)


def _render_template_with_strategy(body: str, substitutions: Mapping[str, str]) -> str:
    mode = dify_config.MAIL_TEMPLATING_MODE
    timeout = dify_config.MAIL_TEMPLATING_TIMEOUT
    if mode == TemplateMode.UNSAFE:
        return render_template_string(body, **substitutions)
    if mode == TemplateMode.SANDBOX:
        tmpl = SandboxedEnvironment(timeout=timeout).from_string(body)
        return tmpl.render(substitutions)
    if mode == TemplateMode.DISABLED:
        return body
    raise ValueError(f"Unsupported mail templating mode: {mode}")


@shared_task(queue="mail")
def send_inner_email_task(to: list[str], subject: str, body: str, substitutions: Mapping[str, str]):
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start enterprise mail to {to} with subject {subject}", fg="green"))
    start_at = time.perf_counter()

    try:
        html_content = _render_template_with_strategy(body, substitutions)

        email_service = get_email_i18n_service()
        email_service.send_raw_email(to=to, subject=subject, html_content=html_content)

        end_at = time.perf_counter()
        logger.info(click.style(f"Send enterprise mail to {to} succeeded: latency: {end_at - start_at}", fg="green"))
    except Exception:
        logger.exception("Send enterprise mail to %s failed", to)

```

### api/tasks/remove_app_and_related_data_task.py
```py
import logging
import time
from collections.abc import Callable
from typing import Any, cast

import click
import sqlalchemy as sa
from celery import shared_task
from sqlalchemy import delete
from sqlalchemy.engine import CursorResult
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from core.db.session_factory import session_factory
from extensions.ext_database import db
from libs.archive_storage import ArchiveStorageNotConfiguredError, get_archive_storage
from models import (
    ApiToken,
    AppAnnotationHitHistory,
    AppAnnotationSetting,
    AppDatasetJoin,
    AppMCPServer,
    AppModelConfig,
    AppTrigger,
    Conversation,
    EndUser,
    InstalledApp,
    Message,
    MessageAgentThought,
    MessageAnnotation,
    MessageChain,
    MessageFeedback,
    MessageFile,
    RecommendedApp,
    Site,
    TagBinding,
    TraceAppConfig,
    WorkflowSchedulePlan,
)
from models.tools import WorkflowToolProvider
from models.trigger import WorkflowPluginTrigger, WorkflowTriggerLog, WorkflowWebhookTrigger
from models.web import PinnedConversation, SavedMessage
from models.workflow import (
    ConversationVariable,
    Workflow,
    WorkflowAppLog,
    WorkflowArchiveLog,
)
from repositories.factory import DifyAPIRepositoryFactory
from services.api_token_service import ApiTokenCache

logger = logging.getLogger(__name__)


@shared_task(queue="app_deletion", bind=True, max_retries=3)
def remove_app_and_related_data_task(self, tenant_id: str, app_id: str):
    logger.info(click.style(f"Start deleting app and related data: {tenant_id}:{app_id}", fg="green"))
    start_at = time.perf_counter()
    try:
        # Delete related data
        _delete_app_model_configs(tenant_id, app_id)
        _delete_app_site(tenant_id, app_id)
        _delete_app_mcp_servers(tenant_id, app_id)
        _delete_app_api_tokens(tenant_id, app_id)
        _delete_installed_apps(tenant_id, app_id)
        _delete_recommended_apps(tenant_id, app_id)
        _delete_app_annotation_data(tenant_id, app_id)
        _delete_app_dataset_joins(tenant_id, app_id)
        _delete_app_workflows(tenant_id, app_id)
        _delete_app_workflow_runs(tenant_id, app_id)
        _delete_app_workflow_node_executions(tenant_id, app_id)
        _delete_app_workflow_app_logs(tenant_id, app_id)
        if dify_config.BILLING_ENABLED and dify_config.ARCHIVE_STORAGE_ENABLED:
            _delete_app_workflow_archive_logs(tenant_id, app_id)
            _delete_archived_workflow_run_files(tenant_id, app_id)
        _delete_app_conversations(tenant_id, app_id)
        _delete_app_messages(tenant_id, app_id)
        _delete_workflow_tool_providers(tenant_id, app_id)
        _delete_app_tag_bindings(tenant_id, app_id)
        _delete_end_users(tenant_id, app_id)
        _delete_trace_app_configs(tenant_id, app_id)
        _delete_conversation_variables(app_id=app_id)
        _delete_draft_variables(app_id)
        _delete_app_triggers(tenant_id, app_id)
        _delete_workflow_plugin_triggers(tenant_id, app_id)
        _delete_workflow_webhook_triggers(tenant_id, app_id)
        _delete_workflow_schedule_plans(tenant_id, app_id)
        _delete_workflow_trigger_logs(tenant_id, app_id)
        end_at = time.perf_counter()
        logger.info(click.style(f"App and related data deleted: {app_id} latency: {end_at - start_at}", fg="green"))
    except SQLAlchemyError as e:
        logger.exception(click.style(f"Database error occurred while deleting app {app_id} and related data", fg="red"))
        raise self.retry(exc=e, countdown=60)  # Retry after 60 seconds
    except Exception as e:
        logger.exception(click.style(f"Error occurred while deleting app {app_id} and related data", fg="red"))
        raise self.retry(exc=e, countdown=60)  # Retry after 60 seconds


def _delete_app_model_configs(tenant_id: str, app_id: str):
    def del_model_config(session, model_config_id: str):
        session.query(AppModelConfig).where(AppModelConfig.id == model_config_id).delete(synchronize_session=False)

    _delete_records(
        """select id from app_model_configs where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_model_config,
        "app model config",
    )


def _delete_app_site(tenant_id: str, app_id: str):
    def del_site(session, site_id: str):
        session.query(Site).where(Site.id == site_id).delete(synchronize_session=False)

    _delete_records(
        """select id from sites where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_site,
        "site",
    )


def _delete_app_mcp_servers(tenant_id: str, app_id: str):
    def del_mcp_server(session, mcp_server_id: str):
        session.query(AppMCPServer).where(AppMCPServer.id == mcp_server_id).delete(synchronize_session=False)

    _delete_records(
        """select id from app_mcp_servers where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_mcp_server,
        "app mcp server",
    )


def _delete_app_api_tokens(tenant_id: str, app_id: str):
    def del_api_token(session, api_token_id: str):
        # Fetch token details for cache invalidation
        token_obj = session.query(ApiToken).where(ApiToken.id == api_token_id).first()
        if token_obj:
            # Invalidate cache before deletion
            ApiTokenCache.delete(token_obj.token, token_obj.type)

        session.query(ApiToken).where(ApiToken.id == api_token_id).delete(synchronize_session=False)

    _delete_records(
        """select id from api_tokens where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_api_token,
        "api token",
    )


def _delete_installed_apps(tenant_id: str, app_id: str):
    def del_installed_app(session, installed_app_id: str):
        session.query(InstalledApp).where(InstalledApp.id == installed_app_id).delete(synchronize_session=False)

    _delete_records(
        """select id from installed_apps where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_installed_app,
        "installed app",
    )


def _delete_recommended_apps(tenant_id: str, app_id: str):
    def del_recommended_app(session, recommended_app_id: str):
        session.query(RecommendedApp).where(RecommendedApp.id == recommended_app_id).delete(synchronize_session=False)

    _delete_records(
        """select id from recommended_apps where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_recommended_app,
        "recommended app",
    )


def _delete_app_annotation_data(tenant_id: str, app_id: str):
    def del_annotation_hit_history(session, annotation_hit_history_id: str):
        session.query(AppAnnotationHitHistory).where(AppAnnotationHitHistory.id == annotation_hit_history_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from app_annotation_hit_histories where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_annotation_hit_history,
        "annotation hit history",
    )

    def del_annotation_setting(session, annotation_setting_id: str):
        session.query(AppAnnotationSetting).where(AppAnnotationSetting.id == annotation_setting_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from app_annotation_settings where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_annotation_setting,
        "annotation setting",
    )


def _delete_app_dataset_joins(tenant_id: str, app_id: str):
    def del_dataset_join(session, dataset_join_id: str):
        session.query(AppDatasetJoin).where(AppDatasetJoin.id == dataset_join_id).delete(synchronize_session=False)

    _delete_records(
        """select id from app_dataset_joins where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_dataset_join,
        "dataset join",
    )


def _delete_app_workflows(tenant_id: str, app_id: str):
    def del_workflow(session, workflow_id: str):
        session.query(Workflow).where(Workflow.id == workflow_id).delete(synchronize_session=False)

    _delete_records(
        """select id from workflows where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_workflow,
        "workflow",
    )


def _delete_app_workflow_runs(tenant_id: str, app_id: str):
    """Delete all workflow runs for an app using the service repository."""
    session_maker = sessionmaker(bind=db.engine)
    workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker)

    deleted_count = workflow_run_repo.delete_runs_by_app(
        tenant_id=tenant_id,
        app_id=app_id,
        batch_size=1000,
    )

    logger.info("Deleted %s workflow runs for app %s", deleted_count, app_id)


def _delete_app_workflow_node_executions(tenant_id: str, app_id: str):
    """Delete all workflow node executions for an app using the service repository."""
    session_maker = sessionmaker(bind=db.engine)
    node_execution_repo = DifyAPIRepositoryFactory.create_api_workflow_node_execution_repository(session_maker)

    deleted_count = node_execution_repo.delete_executions_by_app(
        tenant_id=tenant_id,
        app_id=app_id,
        batch_size=1000,
    )

    logger.info("Deleted %s workflow node executions for app %s", deleted_count, app_id)


def _delete_app_workflow_app_logs(tenant_id: str, app_id: str):
    def del_workflow_app_log(session, workflow_app_log_id: str):
        session.query(WorkflowAppLog).where(WorkflowAppLog.id == workflow_app_log_id).delete(synchronize_session=False)

    _delete_records(
        """select id from workflow_app_logs where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_workflow_app_log,
        "workflow app log",
    )


def _delete_app_workflow_archive_logs(tenant_id: str, app_id: str):
    def del_workflow_archive_log(session, workflow_archive_log_id: str):
        session.query(WorkflowArchiveLog).where(WorkflowArchiveLog.id == workflow_archive_log_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from workflow_archive_logs where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_workflow_archive_log,
        "workflow archive log",
    )


def _delete_archived_workflow_run_files(tenant_id: str, app_id: str):
    prefix = f"{tenant_id}/app_id={app_id}/"
    try:
        archive_storage = get_archive_storage()
    except ArchiveStorageNotConfiguredError as e:
        logger.info("Archive storage not configured, skipping archive file cleanup: %s", e)
        return

    try:
        keys = archive_storage.list_objects(prefix)
    except Exception:
        logger.exception("Failed to list archive files for app %s", app_id)
        return

    deleted = 0
    for key in keys:
        try:
            archive_storage.delete_object(key)
            deleted += 1
        except Exception:
            logger.exception("Failed to delete archive object %s", key)

    logger.info("Deleted %s archive objects for app %s", deleted, app_id)


def _delete_app_conversations(tenant_id: str, app_id: str):
    def del_conversation(session, conversation_id: str):
        session.query(PinnedConversation).where(PinnedConversation.conversation_id == conversation_id).delete(
            synchronize_session=False
        )
        session.query(Conversation).where(Conversation.id == conversation_id).delete(synchronize_session=False)

    _delete_records(
        """select id from conversations where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_conversation,
        "conversation",
    )


def _delete_conversation_variables(*, app_id: str):
    with session_factory.create_session() as session:
        stmt = delete(ConversationVariable).where(ConversationVariable.app_id == app_id)
        session.execute(stmt)
        session.commit()
        logger.info(click.style(f"Deleted conversation variables for app {app_id}", fg="green"))


def _delete_app_messages(tenant_id: str, app_id: str):
    def del_message(session, message_id: str):
        session.query(MessageFeedback).where(MessageFeedback.message_id == message_id).delete(synchronize_session=False)
        session.query(MessageAnnotation).where(MessageAnnotation.message_id == message_id).delete(
            synchronize_session=False
        )
        session.query(MessageChain).where(MessageChain.message_id == message_id).delete(synchronize_session=False)
        session.query(MessageAgentThought).where(MessageAgentThought.message_id == message_id).delete(
            synchronize_session=False
        )
        session.query(MessageFile).where(MessageFile.message_id == message_id).delete(synchronize_session=False)
        session.query(SavedMessage).where(SavedMessage.message_id == message_id).delete(synchronize_session=False)
        session.query(Message).where(Message.id == message_id).delete()

    _delete_records(
        """select id from messages where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_message,
        "message",
    )


def _delete_workflow_tool_providers(tenant_id: str, app_id: str):
    def del_tool_provider(session, tool_provider_id: str):
        session.query(WorkflowToolProvider).where(WorkflowToolProvider.id == tool_provider_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from tool_workflow_providers where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_tool_provider,
        "tool workflow provider",
    )


def _delete_app_tag_bindings(tenant_id: str, app_id: str):
    def del_tag_binding(session, tag_binding_id: str):
        session.query(TagBinding).where(TagBinding.id == tag_binding_id).delete(synchronize_session=False)

    _delete_records(
        """select id from tag_bindings where tenant_id=:tenant_id and target_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_tag_binding,
        "tag binding",
    )


def _delete_end_users(tenant_id: str, app_id: str):
    def del_end_user(session, end_user_id: str):
        session.query(EndUser).where(EndUser.id == end_user_id).delete(synchronize_session=False)

    _delete_records(
        """select id from end_users where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_end_user,
        "end user",
    )


def _delete_trace_app_configs(tenant_id: str, app_id: str):
    def del_trace_app_config(session, trace_app_config_id: str):
        session.query(TraceAppConfig).where(TraceAppConfig.id == trace_app_config_id).delete(synchronize_session=False)

    _delete_records(
        """select id from trace_app_config where app_id=:app_id limit 1000""",
        {"app_id": app_id},
        del_trace_app_config,
        "trace app config",
    )


def _delete_draft_variables(app_id: str):
    """Delete all workflow draft variables for an app in batches."""
    return delete_draft_variables_batch(app_id, batch_size=1000)


def delete_draft_variables_batch(app_id: str, batch_size: int = 1000) -> int:
    """
    Delete draft variables for an app in batches.

    This function now handles cleanup of associated Offload data including:
    - WorkflowDraftVariableFile records
    - UploadFile records
    - Object storage files

    Args:
        app_id: The ID of the app whose draft variables should be deleted
        batch_size: Number of records to delete per batch

    Returns:
        Total number of records deleted
    """
    if batch_size <= 0:
        raise ValueError("batch_size must be positive")

    total_deleted = 0
    total_files_deleted = 0

    while True:
        with session_factory.create_session() as session, session.begin():
            # Get a batch of draft variable IDs along with their file_ids
            query_sql = """
                SELECT id, file_id FROM workflow_draft_variables
                WHERE app_id = :app_id
                LIMIT :batch_size
            """
            result = session.execute(sa.text(query_sql), {"app_id": app_id, "batch_size": batch_size})

            rows = list(result)
            if not rows:
                break

            draft_var_ids = [row[0] for row in rows]
            file_ids = [row[1] for row in rows if row[1] is not None]

            # Clean up associated Offload data first
            if file_ids:
                files_deleted = _delete_draft_variable_offload_data(session, file_ids)
                total_files_deleted += files_deleted

            # Delete the draft variables
            delete_sql = """
                DELETE FROM workflow_draft_variables
                WHERE id IN :ids
            """
            deleted_result = cast(
                CursorResult[Any],
                session.execute(sa.text(delete_sql), {"ids": tuple(draft_var_ids)}),
            )
            batch_deleted: int = int(getattr(deleted_result, "rowcount", 0) or 0)
            total_deleted += batch_deleted

            logger.info(click.style(f"Deleted {batch_deleted} draft variables (batch) for app {app_id}", fg="green"))

    logger.info(
        click.style(
            f"Deleted {total_deleted} total draft variables for app {app_id}. "
            f"Cleaned up {total_files_deleted} total associated files.",
            fg="green",
        )
    )
    return total_deleted


def _delete_draft_variable_offload_data(session, file_ids: list[str]) -> int:
    """
    Delete Offload data associated with WorkflowDraftVariable file_ids.

    This function:
    1. Finds WorkflowDraftVariableFile records by file_ids
    2. Deletes associated files from object storage
    3. Deletes UploadFile records
    4. Deletes WorkflowDraftVariableFile records

    Args:
        session: Database connection
        file_ids: List of WorkflowDraftVariableFile IDs

    Returns:
        Number of files cleaned up
    """
    from extensions.ext_storage import storage

    if not file_ids:
        return 0

    files_deleted = 0

    try:
        # Get WorkflowDraftVariableFile records and their associated UploadFile keys
        query_sql = """
                    SELECT wdvf.id, uf.key, uf.id as upload_file_id
                    FROM workflow_draft_variable_files wdvf
                             JOIN upload_files uf ON wdvf.upload_file_id = uf.id
                    WHERE wdvf.id IN :file_ids \
                    """
        result = session.execute(sa.text(query_sql), {"file_ids": tuple(file_ids)})
        file_records = list(result)

        # Delete from object storage and collect upload file IDs
        upload_file_ids = []
        for _, storage_key, upload_file_id in file_records:
            try:
                storage.delete(storage_key)
                upload_file_ids.append(upload_file_id)
                files_deleted += 1
            except Exception:
                logging.exception("Failed to delete storage object %s", storage_key)
                # Continue with database cleanup even if storage deletion fails
                upload_file_ids.append(upload_file_id)

        # Delete UploadFile records
        if upload_file_ids:
            delete_upload_files_sql = """
                                      DELETE \
                                      FROM upload_files
                                      WHERE id IN :upload_file_ids \
                                      """
            session.execute(sa.text(delete_upload_files_sql), {"upload_file_ids": tuple(upload_file_ids)})

        # Delete WorkflowDraftVariableFile records
        delete_variable_files_sql = """
                                    DELETE \
                                    FROM workflow_draft_variable_files
                                    WHERE id IN :file_ids \
                                    """
        session.execute(sa.text(delete_variable_files_sql), {"file_ids": tuple(file_ids)})

    except Exception:
        logging.exception("Error deleting draft variable offload data:")
        # Don't raise, as we want to continue with the main deletion process

    return files_deleted


def _delete_app_triggers(tenant_id: str, app_id: str):
    def del_app_trigger(session, trigger_id: str):
        session.query(AppTrigger).where(AppTrigger.id == trigger_id).delete(synchronize_session=False)

    _delete_records(
        """select id from app_triggers where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_app_trigger,
        "app trigger",
    )


def _delete_workflow_plugin_triggers(tenant_id: str, app_id: str):
    def del_plugin_trigger(session, trigger_id: str):
        session.query(WorkflowPluginTrigger).where(WorkflowPluginTrigger.id == trigger_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from workflow_plugin_triggers where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_plugin_trigger,
        "workflow plugin trigger",
    )


def _delete_workflow_webhook_triggers(tenant_id: str, app_id: str):
    def del_webhook_trigger(session, trigger_id: str):
        session.query(WorkflowWebhookTrigger).where(WorkflowWebhookTrigger.id == trigger_id).delete(
            synchronize_session=False
        )

    _delete_records(
        """select id from workflow_webhook_triggers where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_webhook_trigger,
        "workflow webhook trigger",
    )


def _delete_workflow_schedule_plans(tenant_id: str, app_id: str):
    def del_schedule_plan(session, plan_id: str):
        session.query(WorkflowSchedulePlan).where(WorkflowSchedulePlan.id == plan_id).delete(synchronize_session=False)

    _delete_records(
        """select id from workflow_schedule_plans where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_schedule_plan,
        "workflow schedule plan",
    )


def _delete_workflow_trigger_logs(tenant_id: str, app_id: str):
    def del_trigger_log(session, log_id: str):
        session.query(WorkflowTriggerLog).where(WorkflowTriggerLog.id == log_id).delete(synchronize_session=False)

    _delete_records(
        """select id from workflow_trigger_logs where tenant_id=:tenant_id and app_id=:app_id limit 1000""",
        {"tenant_id": tenant_id, "app_id": app_id},
        del_trigger_log,
        "workflow trigger log",
    )


def _delete_records(query_sql: str, params: dict, delete_func: Callable, name: str) -> None:
    while True:
        with session_factory.create_session() as session:
            rs = session.execute(sa.text(query_sql), params)
            rows = rs.fetchall()
            if not rows:
                break

            for i in rows:
                record_id = str(i.id)
                try:
                    delete_func(session, record_id)
                    logger.info(click.style(f"Deleted {name} {record_id}", fg="green"))
                except Exception:
                    logger.exception("Error occurred while deleting %s %s", name, record_id)
                    # continue with next record even if one deletion fails
                    session.rollback()
                    break
                session.commit()

            rs.close()

```

### api/tasks/clean_document_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.tools.utils.web_reader_tool import get_image_upload_file_ids
from extensions.ext_storage import storage
from models.dataset import Dataset, DatasetMetadataBinding, DocumentSegment, SegmentAttachmentBinding
from models.model import UploadFile

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def clean_document_task(document_id: str, dataset_id: str, doc_form: str, file_id: str | None):
    """
    Clean document when document deleted.
    :param document_id: document id
    :param dataset_id: dataset id
    :param doc_form: doc_form
    :param file_id: file id

    Usage: clean_document_task.delay(document_id, dataset_id)
    """
    logger.info(click.style(f"Start clean document when document deleted: {document_id}", fg="green"))
    start_at = time.perf_counter()
    total_attachment_files = []

    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()

            if not dataset:
                raise Exception("Document has no dataset")

            segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document_id)).all()
            # Use JOIN to fetch attachments with bindings in a single query
            attachments_with_bindings = session.execute(
                select(SegmentAttachmentBinding, UploadFile)
                .join(UploadFile, UploadFile.id == SegmentAttachmentBinding.attachment_id)
                .where(
                    SegmentAttachmentBinding.tenant_id == dataset.tenant_id,
                    SegmentAttachmentBinding.dataset_id == dataset_id,
                    SegmentAttachmentBinding.document_id == document_id,
                )
            ).all()

            attachment_ids = [attachment_file.id for _, attachment_file in attachments_with_bindings]
            binding_ids = [binding.id for binding, _ in attachments_with_bindings]
            total_attachment_files.extend([attachment_file.key for _, attachment_file in attachments_with_bindings])

            index_node_ids = [segment.index_node_id for segment in segments]
            segment_contents = [segment.content for segment in segments]
        except Exception:
            logger.exception("Cleaned document when document deleted failed")
            return

    # check segment is exist
    if index_node_ids:
        index_processor = IndexProcessorFactory(doc_form).init_index_processor()
        with session_factory.create_session() as session:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if dataset:
                index_processor.clean(
                    dataset, index_node_ids, with_keywords=True, delete_child_chunks=True, delete_summaries=True
                )

    total_image_files = []
    with session_factory.create_session() as session, session.begin():
        for segment_content in segment_contents:
            image_upload_file_ids = get_image_upload_file_ids(segment_content)
            image_files = session.scalars(select(UploadFile).where(UploadFile.id.in_(image_upload_file_ids))).all()
            total_image_files.extend([image_file.key for image_file in image_files])
            image_file_delete_stmt = delete(UploadFile).where(UploadFile.id.in_(image_upload_file_ids))
            session.execute(image_file_delete_stmt)

    with session_factory.create_session() as session, session.begin():
        segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.document_id == document_id)
        session.execute(segment_delete_stmt)

    for image_file_key in total_image_files:
        try:
            storage.delete(image_file_key)
        except Exception:
            logger.exception(
                "Delete image_files failed when storage deleted, \
                                          image_upload_file_is: %s",
                image_file_key,
            )

    with session_factory.create_session() as session, session.begin():
        if file_id:
            file = session.query(UploadFile).where(UploadFile.id == file_id).first()
            if file:
                try:
                    storage.delete(file.key)
                except Exception:
                    logger.exception("Delete file failed when document deleted, file_id: %s", file_id)
                session.delete(file)

    with session_factory.create_session() as session, session.begin():
        # delete segment attachments
        if attachment_ids:
            attachment_file_delete_stmt = delete(UploadFile).where(UploadFile.id.in_(attachment_ids))
            session.execute(attachment_file_delete_stmt)

        if binding_ids:
            binding_delete_stmt = delete(SegmentAttachmentBinding).where(SegmentAttachmentBinding.id.in_(binding_ids))
            session.execute(binding_delete_stmt)

    for attachment_file_key in total_attachment_files:
        try:
            storage.delete(attachment_file_key)
        except Exception:
            logger.exception(
                "Delete attachment_file failed when storage deleted, \
                                    attachment_file_id: %s",
                attachment_file_key,
            )

    with session_factory.create_session() as session, session.begin():
        # delete dataset metadata binding
        session.query(DatasetMetadataBinding).where(
            DatasetMetadataBinding.dataset_id == dataset_id,
            DatasetMetadataBinding.document_id == document_id,
        ).delete()

    end_at = time.perf_counter()
    logger.info(
        click.style(
            f"Cleaned document when document deleted: {document_id} latency: {end_at - start_at}",
            fg="green",
        )
    )

```

### api/tasks/retry_document_indexing_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.indexing_runner import IndexingRunner
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models import Account, Tenant
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import IndexingStatus
from services.feature_service import FeatureService
from services.rag_pipeline.rag_pipeline import RagPipelineService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def retry_document_indexing_task(dataset_id: str, document_ids: list[str], user_id: str):
    """
    Async process document
    :param dataset_id:
    :param document_ids:
    :param user_id:

    Usage: retry_document_indexing_task.delay(dataset_id, document_ids, user_id)
    """
    start_at = time.perf_counter()
    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if not dataset:
                logger.info(click.style(f"Dataset not found: {dataset_id}", fg="red"))
                return
            user = session.query(Account).where(Account.id == user_id).first()
            if not user:
                logger.info(click.style(f"User not found: {user_id}", fg="red"))
                return
            tenant = session.query(Tenant).where(Tenant.id == dataset.tenant_id).first()
            if not tenant:
                raise ValueError("Tenant not found")
            user.current_tenant = tenant

            for document_id in document_ids:
                retry_indexing_cache_key = f"document_{document_id}_is_retried"
                # check document limit
                features = FeatureService.get_features(tenant.id)
                try:
                    if features.billing.enabled:
                        vector_space = features.vector_space
                        if 0 < vector_space.limit <= vector_space.size:
                            raise ValueError(
                                "Your total number of documents plus the number of uploads have over the limit of "
                                "your subscription."
                            )
                except Exception as e:
                    document = (
                        session.query(Document)
                        .where(Document.id == document_id, Document.dataset_id == dataset_id)
                        .first()
                    )
                    if document:
                        document.indexing_status = IndexingStatus.ERROR
                        document.error = str(e)
                        document.stopped_at = naive_utc_now()
                        session.add(document)
                        session.commit()
                    redis_client.delete(retry_indexing_cache_key)
                    return

                logger.info(click.style(f"Start retry document: {document_id}", fg="green"))
                document = (
                    session.query(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).first()
                )
                if not document:
                    logger.info(click.style(f"Document not found: {document_id}", fg="yellow"))
                    return
                try:
                    # clean old data
                    index_processor = IndexProcessorFactory(document.doc_form).init_index_processor()

                    segments = session.scalars(
                        select(DocumentSegment).where(DocumentSegment.document_id == document_id)
                    ).all()
                    if segments:
                        index_node_ids = [segment.index_node_id for segment in segments]
                        # delete from vector index
                        index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)

                    segment_ids = [segment.id for segment in segments]
                    segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.id.in_(segment_ids))
                    session.execute(segment_delete_stmt)
                    session.commit()

                    document.indexing_status = IndexingStatus.PARSING
                    document.processing_started_at = naive_utc_now()
                    session.add(document)
                    session.commit()

                    if dataset.runtime_mode == "rag_pipeline":
                        rag_pipeline_service = RagPipelineService()
                        rag_pipeline_service.retry_error_document(dataset, document, user)
                    else:
                        indexing_runner = IndexingRunner()
                        indexing_runner.run([document])
                    redis_client.delete(retry_indexing_cache_key)
                except Exception as ex:
                    document.indexing_status = IndexingStatus.ERROR
                    document.error = str(ex)
                    document.stopped_at = naive_utc_now()
                    session.add(document)
                    session.commit()
                    logger.info(click.style(str(ex), fg="yellow"))
                    redis_client.delete(retry_indexing_cache_key)
                    logger.exception("retry_document_indexing_task failed, document_id: %s", document_id)
            end_at = time.perf_counter()
            logger.info(click.style(f"Retry dataset: {dataset_id} latency: {end_at - start_at}", fg="green"))
        except Exception as e:
            logger.exception(
                "retry_document_indexing_task failed, dataset_id: %s, document_ids: %s", dataset_id, document_ids
            )
            raise e

```

### api/tasks/disable_segment_from_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from extensions.ext_redis import redis_client
from models.dataset import DocumentSegment

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def disable_segment_from_index_task(segment_id: str):
    """
    Async disable segment from index
    :param segment_id:

    Usage: disable_segment_from_index_task.delay(segment_id)
    """
    logger.info(click.style(f"Start disable segment from index: {segment_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        segment = session.scalar(select(DocumentSegment).where(DocumentSegment.id == segment_id).limit(1))
        if not segment:
            logger.info(click.style(f"Segment not found: {segment_id}", fg="red"))
            return

        if segment.status != "completed":
            logger.info(click.style(f"Segment is not completed, disable is not allowed: {segment_id}", fg="red"))
            return

        indexing_cache_key = f"segment_{segment.id}_indexing"

        try:
            dataset = segment.dataset

            if not dataset:
                logger.info(click.style(f"Segment {segment.id} has no dataset, pass.", fg="cyan"))
                return

            dataset_document = segment.document

            if not dataset_document:
                logger.info(click.style(f"Segment {segment.id} has no document, pass.", fg="cyan"))
                return

            if (
                not dataset_document.enabled
                or dataset_document.archived
                or dataset_document.indexing_status != "completed"
            ):
                logger.info(click.style(f"Segment {segment.id} document status is invalid, pass.", fg="cyan"))
                return

            index_type = dataset_document.doc_form
            index_processor = IndexProcessorFactory(index_type).init_index_processor()
            index_processor.clean(dataset, [segment.index_node_id])

            # Disable summary index for this segment
            from services.summary_index_service import SummaryIndexService

            try:
                SummaryIndexService.disable_summaries_for_segments(
                    dataset=dataset,
                    segment_ids=[segment.id],
                    disabled_by=segment.disabled_by,
                )
            except Exception as e:
                logger.warning("Failed to disable summary for segment %s: %s", segment.id, str(e))

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"Segment removed from index: {segment.id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception:
            logger.exception("remove segment from index failed")
            segment.enabled = True
            session.commit()
        finally:
            redis_client.delete(indexing_cache_key)

```

### api/tasks/duplicate_document_indexing_task.py
```py
import logging
import time
from collections.abc import Callable, Sequence

import click
from celery import shared_task
from sqlalchemy import delete, select

from configs import dify_config
from core.db.session_factory import session_factory
from core.entities.document_task import DocumentTask
from core.indexing_runner import DocumentIsPausedError, IndexingRunner
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.pipeline.queue import TenantIsolatedTaskQueue
from enums.cloud_plan import CloudPlan
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import IndexingStatus
from services.feature_service import FeatureService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def duplicate_document_indexing_task(dataset_id: str, document_ids: list):
    """
    Async process document
    :param dataset_id:
    :param document_ids:

    .. warning:: TO BE DEPRECATED
        This function will be deprecated and removed in a future version.
        Use normal_duplicate_document_indexing_task or priority_duplicate_document_indexing_task instead.

    Usage: duplicate_document_indexing_task.delay(dataset_id, document_ids)
    """
    logger.warning("duplicate document indexing task received: %s - %s", dataset_id, document_ids)
    _duplicate_document_indexing_task(dataset_id, document_ids)


def _duplicate_document_indexing_task_with_tenant_queue(
    tenant_id: str, dataset_id: str, document_ids: Sequence[str], task_func: Callable[[str, str, Sequence[str]], None]
):
    try:
        _duplicate_document_indexing_task(dataset_id, document_ids)
    except Exception:
        logger.exception(
            "Error processing duplicate document indexing %s for tenant %s: %s",
            dataset_id,
            tenant_id,
            document_ids,
            exc_info=True,
        )
    finally:
        tenant_isolated_task_queue = TenantIsolatedTaskQueue(tenant_id, "duplicate_document_indexing")

        # Check if there are waiting tasks in the queue
        # Use rpop to get the next task from the queue (FIFO order)
        next_tasks = tenant_isolated_task_queue.pull_tasks(count=dify_config.TENANT_ISOLATED_TASK_CONCURRENCY)

        logger.info("duplicate document indexing tenant isolation queue %s next tasks: %s", tenant_id, next_tasks)

        if next_tasks:
            for next_task in next_tasks:
                document_task = DocumentTask(**next_task)
                # Process the next waiting task
                # Keep the flag set to indicate a task is running
                tenant_isolated_task_queue.set_task_waiting_time()
                task_func.delay(  # type: ignore
                    tenant_id=document_task.tenant_id,
                    dataset_id=document_task.dataset_id,
                    document_ids=document_task.document_ids,
                )
        else:
            # No more waiting tasks, clear the flag
            tenant_isolated_task_queue.delete_task_key()


def _duplicate_document_indexing_task(dataset_id: str, document_ids: Sequence[str]):
    documents: list[Document] = []
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if dataset is None:
                logger.info(click.style(f"Dataset not found: {dataset_id}", fg="red"))
                return

            # check document limit
            features = FeatureService.get_features(dataset.tenant_id)
            try:
                if features.billing.enabled:
                    vector_space = features.vector_space
                    count = len(document_ids)
                    if features.billing.subscription.plan == CloudPlan.SANDBOX and count > 1:
                        raise ValueError("Your current plan does not support batch upload, please upgrade your plan.")
                    batch_upload_limit = int(dify_config.BATCH_UPLOAD_LIMIT)
                    if count > batch_upload_limit:
                        raise ValueError(f"You have reached the batch upload limit of {batch_upload_limit}.")
                    current = int(getattr(vector_space, "size", 0) or 0)
                    limit = int(getattr(vector_space, "limit", 0) or 0)
                    if limit > 0 and (current + count) > limit:
                        raise ValueError(
                            "Your total number of documents plus the number of uploads have exceeded the limit of "
                            "your subscription."
                        )
            except Exception as e:
                documents = list(
                    session.scalars(
                        select(Document).where(Document.id.in_(document_ids), Document.dataset_id == dataset_id)
                    ).all()
                )
                for document in documents:
                    if document:
                        document.indexing_status = IndexingStatus.ERROR
                        document.error = str(e)
                        document.stopped_at = naive_utc_now()
                        session.add(document)
                session.commit()
                return

            documents = list(
                session.scalars(
                    select(Document).where(Document.id.in_(document_ids), Document.dataset_id == dataset_id)
                ).all()
            )

            for document in documents:
                logger.info(click.style(f"Start process document: {document.id}", fg="green"))

                # clean old data
                index_type = document.doc_form
                index_processor = IndexProcessorFactory(index_type).init_index_processor()

                segments = session.scalars(
                    select(DocumentSegment).where(DocumentSegment.document_id == document.id)
                ).all()
                if segments:
                    index_node_ids = [segment.index_node_id for segment in segments]

                    # delete from vector index
                    index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)

                    segment_ids = [segment.id for segment in segments]
                    segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.id.in_(segment_ids))
                    session.execute(segment_delete_stmt)
                    session.commit()

                document.indexing_status = IndexingStatus.PARSING
                document.processing_started_at = naive_utc_now()
                session.add(document)
            session.commit()

            indexing_runner = IndexingRunner()
            indexing_runner.run(list(documents))
            end_at = time.perf_counter()
            logger.info(click.style(f"Processed dataset: {dataset_id} latency: {end_at - start_at}", fg="green"))
        except DocumentIsPausedError as ex:
            logger.info(click.style(str(ex), fg="yellow"))
        except Exception:
            logger.exception("duplicate_document_indexing_task failed, dataset_id: %s", dataset_id)


@shared_task(queue="dataset")
def normal_duplicate_document_indexing_task(tenant_id: str, dataset_id: str, document_ids: Sequence[str]):
    """
    Async process duplicate documents
    :param tenant_id:
    :param dataset_id:
    :param document_ids:

    Usage: normal_duplicate_document_indexing_task.delay(tenant_id, dataset_id, document_ids)
    """
    logger.info("normal duplicate document indexing task received: %s - %s - %s", tenant_id, dataset_id, document_ids)
    _duplicate_document_indexing_task_with_tenant_queue(
        tenant_id, dataset_id, document_ids, normal_duplicate_document_indexing_task
    )


@shared_task(queue="priority_dataset")
def priority_duplicate_document_indexing_task(tenant_id: str, dataset_id: str, document_ids: Sequence[str]):
    """
    Async process duplicate documents
    :param tenant_id:
    :param dataset_id:
    :param document_ids:

    Usage: priority_duplicate_document_indexing_task.delay(tenant_id, dataset_id, document_ids)
    """
    logger.info("priority duplicate document indexing task received: %s - %s - %s", tenant_id, dataset_id, document_ids)
    _duplicate_document_indexing_task_with_tenant_queue(
        tenant_id, dataset_id, document_ids, priority_duplicate_document_indexing_task
    )

```

### api/tasks/mail_email_code_login.py
```py
import logging
import time

import click
from celery import shared_task

from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_email_code_login_mail_task(language: str, to: str, code: str):
    """
    Send email code login email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        code: Email verification code
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start email code login mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.EMAIL_CODE_LOGIN,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "code": code,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send email code login mail to {to} succeeded: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send email code login mail to %s failed", to)

```

### api/tasks/enable_segment_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import AttachmentDocument, ChildDocument, Document
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import DocumentSegment
from models.enums import IndexingStatus, SegmentStatus

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def enable_segment_to_index_task(segment_id: str):
    """
    Async enable segment to index
    :param segment_id:

    Usage: enable_segment_to_index_task.delay(segment_id)
    """
    logger.info(click.style(f"Start enable segment to index: {segment_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        segment = session.scalar(select(DocumentSegment).where(DocumentSegment.id == segment_id).limit(1))
        if not segment:
            logger.info(click.style(f"Segment not found: {segment_id}", fg="red"))
            return

        if segment.status != SegmentStatus.COMPLETED:
            logger.info(click.style(f"Segment is not completed, enable is not allowed: {segment_id}", fg="red"))
            return

        indexing_cache_key = f"segment_{segment.id}_indexing"

        try:
            document = Document(
                page_content=segment.content,
                metadata={
                    "doc_id": segment.index_node_id,
                    "doc_hash": segment.index_node_hash,
                    "document_id": segment.document_id,
                    "dataset_id": segment.dataset_id,
                },
            )

            dataset = segment.dataset

            if not dataset:
                logger.info(click.style(f"Segment {segment.id} has no dataset, pass.", fg="cyan"))
                return

            dataset_document = segment.document

            if not dataset_document:
                logger.info(click.style(f"Segment {segment.id} has no document, pass.", fg="cyan"))
                return

            if (
                not dataset_document.enabled
                or dataset_document.archived
                or dataset_document.indexing_status != IndexingStatus.COMPLETED
            ):
                logger.info(click.style(f"Segment {segment.id} document status is invalid, pass.", fg="cyan"))
                return

            index_processor = IndexProcessorFactory(dataset_document.doc_form).init_index_processor()
            if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                child_chunks = segment.get_child_chunks()
                if child_chunks:
                    child_documents = []
                    for child_chunk in child_chunks:
                        child_document = ChildDocument(
                            page_content=child_chunk.content,
                            metadata={
                                "doc_id": child_chunk.index_node_id,
                                "doc_hash": child_chunk.index_node_hash,
                                "document_id": segment.document_id,
                                "dataset_id": segment.dataset_id,
                            },
                        )
                        child_documents.append(child_document)
                    document.children = child_documents
            multimodel_documents = []
            if dataset.is_multimodal:
                for attachment in segment.attachments:
                    multimodel_documents.append(
                        AttachmentDocument(
                            page_content=attachment["name"],
                            metadata={
                                "doc_id": attachment["id"],
                                "doc_hash": "",
                                "document_id": segment.document_id,
                                "dataset_id": segment.dataset_id,
                                "doc_type": DocType.IMAGE,
                            },
                        )
                    )

            # save vector index
            index_processor.load(dataset, [document], multimodal_documents=multimodel_documents)

            # Enable summary index for this segment
            from services.summary_index_service import SummaryIndexService

            try:
                SummaryIndexService.enable_summaries_for_segments(
                    dataset=dataset,
                    segment_ids=[segment.id],
                )
            except Exception as e:
                logger.warning("Failed to enable summary for segment %s: %s", segment.id, str(e))

            end_at = time.perf_counter()
            logger.info(click.style(f"Segment enabled to index: {segment.id} latency: {end_at - start_at}", fg="green"))
        except Exception as e:
            logger.exception("enable segment to index failed")
            segment.enabled = False
            segment.disabled_at = naive_utc_now()
            segment.status = SegmentStatus.ERROR
            segment.error = str(e)
            session.commit()
        finally:
            redis_client.delete(indexing_cache_key)

```

### api/tasks/create_segment_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import DocumentSegment
from models.enums import IndexingStatus, SegmentStatus

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def create_segment_to_index_task(segment_id: str, keywords: list[str] | None = None):
    """
    Async create segment to index
    :param segment_id:
    :param keywords:
    Usage: create_segment_to_index_task.delay(segment_id)
    """
    logger.info(click.style(f"Start create segment to index: {segment_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        segment = session.query(DocumentSegment).where(DocumentSegment.id == segment_id).first()
        if not segment:
            logger.info(click.style(f"Segment not found: {segment_id}", fg="red"))
            return

        if segment.status != SegmentStatus.WAITING:
            return

        indexing_cache_key = f"segment_{segment.id}_indexing"

        try:
            # update segment status to indexing
            session.query(DocumentSegment).filter_by(id=segment.id).update(
                {
                    DocumentSegment.status: SegmentStatus.INDEXING,
                    DocumentSegment.indexing_at: naive_utc_now(),
                }
            )
            session.commit()
            document = Document(
                page_content=segment.content,
                metadata={
                    "doc_id": segment.index_node_id,
                    "doc_hash": segment.index_node_hash,
                    "document_id": segment.document_id,
                    "dataset_id": segment.dataset_id,
                },
            )

            dataset = segment.dataset

            if not dataset:
                logger.info(click.style(f"Segment {segment.id} has no dataset, pass.", fg="cyan"))
                return

            dataset_document = segment.document

            if not dataset_document:
                logger.info(click.style(f"Segment {segment.id} has no document, pass.", fg="cyan"))
                return

            if (
                not dataset_document.enabled
                or dataset_document.archived
                or dataset_document.indexing_status != IndexingStatus.COMPLETED
            ):
                logger.info(click.style(f"Segment {segment.id} document status is invalid, pass.", fg="cyan"))
                return

            index_type = dataset.doc_form
            index_processor = IndexProcessorFactory(index_type).init_index_processor()
            index_processor.load(dataset, [document])

            # update segment to completed
            session.query(DocumentSegment).filter_by(id=segment.id).update(
                {
                    DocumentSegment.status: SegmentStatus.COMPLETED,
                    DocumentSegment.completed_at: naive_utc_now(),
                }
            )
            session.commit()

            end_at = time.perf_counter()
            logger.info(click.style(f"Segment created to index: {segment.id} latency: {end_at - start_at}", fg="green"))
        except Exception as e:
            logger.exception("create segment to index failed")
            segment.enabled = False
            segment.disabled_at = naive_utc_now()
            segment.status = SegmentStatus.ERROR
            segment.error = str(e)
            session.commit()
        finally:
            redis_client.delete(indexing_cache_key)

```

### api/tasks/process_tenant_plugin_autoupgrade_check_task.py
```py
import json
import logging
import operator
import typing

import click
from celery import shared_task

from core.plugin.entities.marketplace import MarketplacePluginSnapshot
from core.plugin.entities.plugin import PluginInstallationSource
from core.plugin.impl.plugin import PluginInstaller
from extensions.ext_redis import redis_client
from models.account import TenantPluginAutoUpgradeStrategy

logger = logging.getLogger(__name__)

RETRY_TIMES_OF_ONE_PLUGIN_IN_ONE_TENANT = 3
CACHE_REDIS_KEY_PREFIX = "plugin_autoupgrade_check_task:cached_plugin_snapshot:"
CACHE_REDIS_TTL = 60 * 60  # 1 hour


def _get_redis_cache_key(plugin_id: str) -> str:
    """Generate Redis cache key for plugin manifest."""
    return f"{CACHE_REDIS_KEY_PREFIX}{plugin_id}"


def _get_cached_manifest(plugin_id: str) -> typing.Union[MarketplacePluginSnapshot, None, bool]:
    """
    Get cached plugin manifest from Redis.
    Returns:
        - MarketplacePluginSnapshot: if found in cache
        - None: if cached as not found (marketplace returned no result)
        - False: if not in cache at all
    """
    try:
        key = _get_redis_cache_key(plugin_id)
        cached_data = redis_client.get(key)
        if cached_data is None:
            return False

        cached_json = json.loads(cached_data)
        if cached_json is None:
            return None

        return MarketplacePluginSnapshot.model_validate(cached_json)
    except Exception:
        logger.exception("Failed to get cached manifest for plugin %s", plugin_id)
        return False


def marketplace_batch_fetch_plugin_manifests(
    plugin_ids_plain_list: list[str],
) -> list[MarketplacePluginSnapshot]:
    """
    Fetch plugin manifests from Redis cache only.
    This function assumes fetch_global_plugin_manifest() has been called
    to pre-populate the cache with all marketplace plugins.
    """
    result: list[MarketplacePluginSnapshot] = []

    # Check Redis cache for each plugin
    for plugin_id in plugin_ids_plain_list:
        cached_result = _get_cached_manifest(plugin_id)
        if not isinstance(cached_result, MarketplacePluginSnapshot):
            # cached_result is False (not in cache) or None (cached as not found)
            logger.warning("plugin %s not found in cache, skipping", plugin_id)
            continue

        result.append(cached_result)

    return result


@shared_task(queue="plugin")
def process_tenant_plugin_autoupgrade_check_task(
    tenant_id: str,
    strategy_setting: TenantPluginAutoUpgradeStrategy.StrategySetting,
    upgrade_time_of_day: int,
    upgrade_mode: TenantPluginAutoUpgradeStrategy.UpgradeMode,
    exclude_plugins: list[str],
    include_plugins: list[str],
):
    try:
        manager = PluginInstaller()

        click.echo(
            click.style(
                f"Checking upgradable plugin for tenant: {tenant_id}",
                fg="green",
            )
        )

        if strategy_setting == TenantPluginAutoUpgradeStrategy.StrategySetting.DISABLED:
            return

        # get plugin_ids to check
        plugin_ids: list[tuple[str, str, str]] = []  # plugin_id, version, unique_identifier
        click.echo(click.style(f"Upgrade mode: {upgrade_mode}", fg="green"))

        if upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.PARTIAL and include_plugins:
            all_plugins = manager.list_plugins(tenant_id)

            for plugin in all_plugins:
                if plugin.source == PluginInstallationSource.Marketplace and plugin.plugin_id in include_plugins:
                    plugin_ids.append(
                        (
                            plugin.plugin_id,
                            plugin.version,
                            plugin.plugin_unique_identifier,
                        )
                    )

        elif upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.EXCLUDE:
            # get all plugins and remove excluded plugins
            all_plugins = manager.list_plugins(tenant_id)
            plugin_ids = [
                (plugin.plugin_id, plugin.version, plugin.plugin_unique_identifier)
                for plugin in all_plugins
                if plugin.source == PluginInstallationSource.Marketplace and plugin.plugin_id not in exclude_plugins
            ]
        elif upgrade_mode == TenantPluginAutoUpgradeStrategy.UpgradeMode.ALL:
            all_plugins = manager.list_plugins(tenant_id)
            plugin_ids = [
                (plugin.plugin_id, plugin.version, plugin.plugin_unique_identifier)
                for plugin in all_plugins
                if plugin.source == PluginInstallationSource.Marketplace
            ]

        if not plugin_ids:
            return

        plugin_ids_plain_list = [plugin_id for plugin_id, _, _ in plugin_ids]

        manifests = marketplace_batch_fetch_plugin_manifests(plugin_ids_plain_list)

        if not manifests:
            return

        for manifest in manifests:
            for plugin_id, version, original_unique_identifier in plugin_ids:
                if manifest.plugin_id != plugin_id:
                    continue

                try:
                    current_version = version
                    latest_version = manifest.latest_version

                    def fix_only_checker(latest_version: str, current_version: str):
                        latest_version_tuple = tuple(int(val) for val in latest_version.split("."))
                        current_version_tuple = tuple(int(val) for val in current_version.split("."))

                        if (
                            latest_version_tuple[0] == current_version_tuple[0]
                            and latest_version_tuple[1] == current_version_tuple[1]
                        ):
                            return latest_version_tuple[2] != current_version_tuple[2]
                        return False

                    version_checker = {
                        TenantPluginAutoUpgradeStrategy.StrategySetting.LATEST: operator.ne,
                        TenantPluginAutoUpgradeStrategy.StrategySetting.FIX_ONLY: fix_only_checker,
                    }

                    if version_checker[strategy_setting](latest_version, current_version):
                        # execute upgrade
                        new_unique_identifier = manifest.latest_package_identifier

                        click.echo(
                            click.style(
                                f"Upgrade plugin: {original_unique_identifier} -> {new_unique_identifier}",
                                fg="green",
                            )
                        )
                        _ = manager.upgrade_plugin(
                            tenant_id,
                            original_unique_identifier,
                            new_unique_identifier,
                            PluginInstallationSource.Marketplace,
                            {
                                "plugin_unique_identifier": new_unique_identifier,
                            },
                        )
                except Exception as e:
                    click.echo(click.style(f"Error when upgrading plugin: {e}", fg="red"))
                    # traceback.print_exc()
                break

    except Exception as e:
        click.echo(click.style(f"Error when checking upgradable plugin: {e}", fg="red"))
        # traceback.print_exc()
        return

```

### api/tasks/__init__.py
```py

```

### api/tasks/document_indexing_task.py
```py
import logging
import time
from collections.abc import Sequence
from typing import Any, Protocol

import click
from celery import current_app, shared_task

from configs import dify_config
from core.db.session_factory import session_factory
from core.entities.document_task import DocumentTask
from core.indexing_runner import DocumentIsPausedError, IndexingRunner
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from core.rag.pipeline.queue import TenantIsolatedTaskQueue
from enums.cloud_plan import CloudPlan
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document
from models.enums import IndexingStatus
from services.feature_service import FeatureService
from tasks.generate_summary_index_task import generate_summary_index_task

logger = logging.getLogger(__name__)


class CeleryTaskLike(Protocol):
    def delay(self, *args: Any, **kwargs: Any) -> Any: ...

    def apply_async(self, *args: Any, **kwargs: Any) -> Any: ...


@shared_task(queue="dataset")
def document_indexing_task(dataset_id: str, document_ids: list):
    """
    Async process document
    :param dataset_id:
    :param document_ids:

    .. warning:: TO BE DEPRECATED
        This function will be deprecated and removed in a future version.
        Use normal_document_indexing_task or priority_document_indexing_task instead.

    Usage: document_indexing_task.delay(dataset_id, document_ids)
    """
    logger.warning("document indexing legacy mode received: %s - %s", dataset_id, document_ids)
    _document_indexing(dataset_id, document_ids)


def _document_indexing(dataset_id: str, document_ids: Sequence[str]):
    """
    Process document for tasks
    :param dataset_id:
    :param document_ids:

    Usage: _document_indexing(dataset_id, document_ids)
    """
    documents = []
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if not dataset:
            logger.info(click.style(f"Dataset is not found: {dataset_id}", fg="yellow"))
            return
        # check document limit
        features = FeatureService.get_features(dataset.tenant_id)
        try:
            if features.billing.enabled:
                vector_space = features.vector_space
                count = len(document_ids)
                batch_upload_limit = int(dify_config.BATCH_UPLOAD_LIMIT)
                if features.billing.subscription.plan == CloudPlan.SANDBOX and count > 1:
                    raise ValueError("Your current plan does not support batch upload, please upgrade your plan.")
                if count > batch_upload_limit:
                    raise ValueError(f"You have reached the batch upload limit of {batch_upload_limit}.")
                if 0 < vector_space.limit <= vector_space.size:
                    raise ValueError(
                        "Your total number of documents plus the number of uploads have over the limit of "
                        "your subscription."
                    )
        except Exception as e:
            for document_id in document_ids:
                document = (
                    session.query(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).first()
                )
                if document:
                    document.indexing_status = IndexingStatus.ERROR
                    document.error = str(e)
                    document.stopped_at = naive_utc_now()
                    session.add(document)
            session.commit()
            return

    # Phase 1: Update status to parsing (short transaction)
    with session_factory.create_session() as session, session.begin():
        documents = (
            session.query(Document).where(Document.id.in_(document_ids), Document.dataset_id == dataset_id).all()
        )

        for document in documents:
            if document:
                document.indexing_status = IndexingStatus.PARSING
                document.processing_started_at = naive_utc_now()
                session.add(document)
    # Transaction committed and closed

    # Phase 2: Execute indexing (no transaction - IndexingRunner creates its own sessions)
    has_error = False
    try:
        indexing_runner = IndexingRunner()
        indexing_runner.run(documents)
        end_at = time.perf_counter()
        logger.info(click.style(f"Processed dataset: {dataset_id} latency: {end_at - start_at}", fg="green"))
    except DocumentIsPausedError as ex:
        logger.info(click.style(str(ex), fg="yellow"))
        has_error = True
    except Exception:
        logger.exception("Document indexing task failed, dataset_id: %s", dataset_id)
        has_error = True

    if not has_error:
        with session_factory.create_session() as session:
            # Trigger summary index generation for completed documents if enabled
            # Only generate for high_quality indexing technique and when summary_index_setting is enabled
            # Re-query dataset to get latest summary_index_setting (in case it was updated)
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if not dataset:
                logger.warning("Dataset %s not found after indexing", dataset_id)
                return

            if dataset.indexing_technique == IndexTechniqueType.HIGH_QUALITY:
                summary_index_setting = dataset.summary_index_setting
                if summary_index_setting and summary_index_setting.get("enable"):
                    # expire all session to get latest document's indexing status
                    session.expire_all()
                    # Check each document's indexing status and trigger summary generation if completed

                    documents = (
                        session.query(Document)
                        .where(Document.id.in_(document_ids), Document.dataset_id == dataset_id)
                        .all()
                    )

                    for document in documents:
                        if document:
                            logger.info(
                                "Checking document %s for summary generation: status=%s, doc_form=%s, need_summary=%s",
                                document.id,
                                document.indexing_status,
                                document.doc_form,
                                document.need_summary,
                            )
                            if (
                                document.indexing_status == IndexingStatus.COMPLETED
                                and document.doc_form != IndexStructureType.QA_INDEX
                                and document.need_summary is True
                            ):
                                try:
                                    generate_summary_index_task.delay(dataset.id, document.id, None)
                                    logger.info(
                                        "Queued summary index generation task for document %s in dataset %s "
                                        "after indexing completed",
                                        document.id,
                                        dataset.id,
                                    )
                                except Exception:
                                    logger.exception(
                                        "Failed to queue summary index generation task for document %s",
                                        document.id,
                                    )
                                    # Don't fail the entire indexing process if summary task queuing fails
                            else:
                                logger.info(
                                    "Skipping summary generation for document %s: "
                                    "status=%s, doc_form=%s, need_summary=%s",
                                    document.id,
                                    document.indexing_status,
                                    document.doc_form,
                                    document.need_summary,
                                )
                        else:
                            logger.warning("Document %s not found after indexing", document.id)
            else:
                logger.info(
                    "Summary index generation skipped for dataset %s: indexing_technique=%s (not 'high_quality')",
                    dataset.id,
                    dataset.indexing_technique,
                )


def _document_indexing_with_tenant_queue(
    tenant_id: str, dataset_id: str, document_ids: Sequence[str], task_func: CeleryTaskLike
) -> None:
    try:
        _document_indexing(dataset_id, document_ids)
    except Exception:
        logger.exception(
            "Error processing document indexing %s for tenant %s: %s",
            dataset_id,
            tenant_id,
            document_ids,
            exc_info=True,
        )
    finally:
        tenant_isolated_task_queue = TenantIsolatedTaskQueue(tenant_id, "document_indexing")

        # Check if there are waiting tasks in the queue
        # Use rpop to get the next task from the queue (FIFO order)
        next_tasks = tenant_isolated_task_queue.pull_tasks(count=dify_config.TENANT_ISOLATED_TASK_CONCURRENCY)

        logger.info("document indexing tenant isolation queue %s next tasks: %s", tenant_id, next_tasks)

        if next_tasks:
            with current_app.producer_or_acquire() as producer:  # type: ignore
                for next_task in next_tasks:
                    document_task = DocumentTask(**next_task)
                    # Keep the flag set to indicate a task is running
                    tenant_isolated_task_queue.set_task_waiting_time()
                    task_func.apply_async(
                        kwargs={
                            "tenant_id": document_task.tenant_id,
                            "dataset_id": document_task.dataset_id,
                            "document_ids": document_task.document_ids,
                        },
                        producer=producer,
                    )

        else:
            # No more waiting tasks, clear the flag
            tenant_isolated_task_queue.delete_task_key()


@shared_task(queue="dataset")
def normal_document_indexing_task(tenant_id: str, dataset_id: str, document_ids: Sequence[str]):
    """
    Async process document
    :param tenant_id:
    :param dataset_id:
    :param document_ids:

    Usage: normal_document_indexing_task.delay(tenant_id, dataset_id, document_ids)
    """
    logger.info("normal document indexing task received: %s - %s - %s", tenant_id, dataset_id, document_ids)
    _document_indexing_with_tenant_queue(tenant_id, dataset_id, document_ids, normal_document_indexing_task)


@shared_task(queue="priority_dataset")
def priority_document_indexing_task(tenant_id: str, dataset_id: str, document_ids: Sequence[str]):
    """
    Priority async process document
    :param tenant_id:
    :param dataset_id:
    :param document_ids:

    Usage: priority_document_indexing_task.delay(tenant_id, dataset_id, document_ids)
    """
    logger.info("priority document indexing task received: %s - %s - %s", tenant_id, dataset_id, document_ids)
    _document_indexing_with_tenant_queue(tenant_id, dataset_id, document_ids, priority_document_indexing_task)

```

### api/tasks/deal_dataset_vector_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import AttachmentDocument, ChildDocument, Document
from models.dataset import Dataset, DocumentSegment
from models.dataset import Document as DatasetDocument

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def deal_dataset_vector_index_task(dataset_id: str, action: str):
    """
    Async deal dataset from index
    :param dataset_id: dataset_id
    :param action: action
    Usage: deal_dataset_vector_index_task.delay(dataset_id, action)
    """
    logger.info(click.style(f"Start deal dataset vector index: {dataset_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).filter_by(id=dataset_id).first()

            if not dataset:
                raise Exception("Dataset not found")
            index_type = dataset.doc_form or IndexStructureType.PARAGRAPH_INDEX
            index_processor = IndexProcessorFactory(index_type).init_index_processor()
            if action == "remove":
                index_processor.clean(dataset, None, with_keywords=False)
            elif action == "add":
                dataset_documents = session.scalars(
                    select(DatasetDocument).where(
                        DatasetDocument.dataset_id == dataset_id,
                        DatasetDocument.indexing_status == "completed",
                        DatasetDocument.enabled == True,
                        DatasetDocument.archived == False,
                    )
                ).all()

                if dataset_documents:
                    dataset_documents_ids = [doc.id for doc in dataset_documents]
                    session.query(DatasetDocument).where(DatasetDocument.id.in_(dataset_documents_ids)).update(
                        {"indexing_status": "indexing"}, synchronize_session=False
                    )
                    session.commit()

                    for dataset_document in dataset_documents:
                        try:
                            # add from vector index
                            segments = (
                                session.query(DocumentSegment)
                                .where(
                                    DocumentSegment.document_id == dataset_document.id,
                                    DocumentSegment.enabled == True,
                                )
                                .order_by(DocumentSegment.position.asc())
                                .all()
                            )
                            if segments:
                                documents = []
                                for segment in segments:
                                    document = Document(
                                        page_content=segment.content,
                                        metadata={
                                            "doc_id": segment.index_node_id,
                                            "doc_hash": segment.index_node_hash,
                                            "document_id": segment.document_id,
                                            "dataset_id": segment.dataset_id,
                                        },
                                    )

                                    documents.append(document)
                                # save vector index
                                index_processor.load(dataset, documents, with_keywords=False)
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "completed"}, synchronize_session=False
                            )
                            session.commit()
                        except Exception as e:
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "error", "error": str(e)}, synchronize_session=False
                            )
                            session.commit()
            elif action == "update":
                dataset_documents = session.scalars(
                    select(DatasetDocument).where(
                        DatasetDocument.dataset_id == dataset_id,
                        DatasetDocument.indexing_status == "completed",
                        DatasetDocument.enabled == True,
                        DatasetDocument.archived == False,
                    )
                ).all()
                # add new index
                if dataset_documents:
                    # update document status
                    dataset_documents_ids = [doc.id for doc in dataset_documents]
                    session.query(DatasetDocument).where(DatasetDocument.id.in_(dataset_documents_ids)).update(
                        {"indexing_status": "indexing"}, synchronize_session=False
                    )
                    session.commit()

                    # clean index
                    index_processor.clean(dataset, None, with_keywords=False, delete_child_chunks=False)

                    for dataset_document in dataset_documents:
                        # update from vector index
                        try:
                            segments = (
                                session.query(DocumentSegment)
                                .where(
                                    DocumentSegment.document_id == dataset_document.id,
                                    DocumentSegment.enabled == True,
                                )
                                .order_by(DocumentSegment.position.asc())
                                .all()
                            )
                            if segments:
                                documents = []
                                multimodal_documents = []
                                for segment in segments:
                                    document = Document(
                                        page_content=segment.content,
                                        metadata={
                                            "doc_id": segment.index_node_id,
                                            "doc_hash": segment.index_node_hash,
                                            "document_id": segment.document_id,
                                            "dataset_id": segment.dataset_id,
                                        },
                                    )
                                    if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                                        child_chunks = segment.get_child_chunks()
                                        if child_chunks:
                                            child_documents = []
                                            for child_chunk in child_chunks:
                                                child_document = ChildDocument(
                                                    page_content=child_chunk.content,
                                                    metadata={
                                                        "doc_id": child_chunk.index_node_id,
                                                        "doc_hash": child_chunk.index_node_hash,
                                                        "document_id": segment.document_id,
                                                        "dataset_id": segment.dataset_id,
                                                    },
                                                )
                                                child_documents.append(child_document)
                                            document.children = child_documents
                                    if dataset.is_multimodal:
                                        for attachment in segment.attachments:
                                            multimodal_documents.append(
                                                AttachmentDocument(
                                                    page_content=attachment["name"],
                                                    metadata={
                                                        "doc_id": attachment["id"],
                                                        "doc_hash": "",
                                                        "document_id": segment.document_id,
                                                        "dataset_id": segment.dataset_id,
                                                        "doc_type": DocType.IMAGE,
                                                    },
                                                )
                                            )
                                    documents.append(document)
                                # save vector index
                                index_processor.load(
                                    dataset, documents, multimodal_documents=multimodal_documents, with_keywords=False
                                )
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "completed"}, synchronize_session=False
                            )
                            session.commit()
                        except Exception as e:
                            session.query(DatasetDocument).where(DatasetDocument.id == dataset_document.id).update(
                                {"indexing_status": "error", "error": str(e)}, synchronize_session=False
                            )
                            session.commit()
                else:
                    # clean collection
                    index_processor.clean(dataset, None, with_keywords=False, delete_child_chunks=False)

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"Deal dataset vector index: {dataset_id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception:
            logger.exception("Deal dataset vector index failed")

```

### api/tasks/batch_create_segment_to_index_task.py
```py
import logging
import tempfile
import time
import uuid
from pathlib import Path

import click
import pandas as pd
from celery import shared_task
from graphon.model_runtime.entities.model_entities import ModelType
from sqlalchemy import func

from core.db.session_factory import session_factory
from core.model_manager import ModelManager
from core.rag.index_processor.constant.index_type import IndexStructureType, IndexTechniqueType
from extensions.ext_redis import redis_client
from extensions.ext_storage import storage
from libs import helper
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document, DocumentSegment
from models.model import UploadFile
from services.vector_service import VectorService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def batch_create_segment_to_index_task(
    job_id: str,
    upload_file_id: str,
    dataset_id: str,
    document_id: str,
    tenant_id: str,
    user_id: str,
):
    """
    Async batch create segment to index
    :param job_id:
    :param upload_file_id:
    :param dataset_id:
    :param document_id:
    :param tenant_id:
    :param user_id:

    Usage: batch_create_segment_to_index_task.delay(job_id, upload_file_id, dataset_id, document_id, tenant_id, user_id)
    """
    logger.info(click.style(f"Start batch create segment jobId: {job_id}", fg="green"))
    start_at = time.perf_counter()

    indexing_cache_key = f"segment_batch_import_{job_id}"

    # Initialize variables with default values
    upload_file_key: str | None = None
    dataset_config: dict | None = None
    document_config: dict | None = None

    with session_factory.create_session() as session:
        try:
            dataset = session.get(Dataset, dataset_id)
            if not dataset:
                raise ValueError("Dataset not exist.")

            dataset_document = session.get(Document, document_id)
            if not dataset_document:
                raise ValueError("Document not exist.")

            if (
                not dataset_document.enabled
                or dataset_document.archived
                or dataset_document.indexing_status != "completed"
            ):
                raise ValueError("Document is not available.")

            upload_file = session.get(UploadFile, upload_file_id)
            if not upload_file:
                raise ValueError("UploadFile not found.")

            dataset_config = {
                "id": dataset.id,
                "indexing_technique": dataset.indexing_technique,
                "tenant_id": dataset.tenant_id,
                "embedding_model_provider": dataset.embedding_model_provider,
                "embedding_model": dataset.embedding_model,
            }

            document_config = {
                "id": dataset_document.id,
                "doc_form": dataset_document.doc_form,
                "word_count": dataset_document.word_count or 0,
            }

            upload_file_key = upload_file.key

        except Exception:
            logger.exception("Segments batch created index failed")
            redis_client.setex(indexing_cache_key, 600, "error")
            return

    # Ensure required variables are set before proceeding
    if upload_file_key is None or dataset_config is None or document_config is None:
        logger.error("Required configuration not set due to session error")
        redis_client.setex(indexing_cache_key, 600, "error")
        return

    with tempfile.TemporaryDirectory() as temp_dir:
        suffix = Path(upload_file_key).suffix
        file_path = f"{temp_dir}/{next(tempfile._get_candidate_names())}{suffix}"  # type: ignore
        storage.download(upload_file_key, file_path)

        df = pd.read_csv(file_path)
        content = []
        for _, row in df.iterrows():
            if document_config["doc_form"] == IndexStructureType.QA_INDEX:
                data = {"content": row.iloc[0], "answer": row.iloc[1]}
            else:
                data = {"content": row.iloc[0]}
            content.append(data)
        if len(content) == 0:
            raise ValueError("The CSV file is empty.")

    document_segments = []
    embedding_model = None
    if dataset_config["indexing_technique"] == IndexTechniqueType.HIGH_QUALITY:
        model_manager = ModelManager.for_tenant(tenant_id=dataset_config["tenant_id"])
        embedding_model = model_manager.get_model_instance(
            tenant_id=dataset_config["tenant_id"],
            provider=dataset_config["embedding_model_provider"],
            model_type=ModelType.TEXT_EMBEDDING,
            model=dataset_config["embedding_model"],
        )

    word_count_change = 0
    if embedding_model:
        tokens_list = embedding_model.get_text_embedding_num_tokens(texts=[segment["content"] for segment in content])
    else:
        tokens_list = [0] * len(content)

    with session_factory.create_session() as session, session.begin():
        for segment, tokens in zip(content, tokens_list):
            content = segment["content"]
            doc_id = str(uuid.uuid4())
            segment_hash = helper.generate_text_hash(content)
            max_position = (
                session.query(func.max(DocumentSegment.position))
                .where(DocumentSegment.document_id == document_config["id"])
                .scalar()
            )
            segment_document = DocumentSegment(
                tenant_id=tenant_id,
                dataset_id=dataset_id,
                document_id=document_id,
                index_node_id=doc_id,
                index_node_hash=segment_hash,
                position=max_position + 1 if max_position else 1,
                content=content,
                word_count=len(content),
                tokens=tokens,
                created_by=user_id,
                indexing_at=naive_utc_now(),
                status="completed",
                completed_at=naive_utc_now(),
            )
            if document_config["doc_form"] == IndexStructureType.QA_INDEX:
                segment_document.answer = segment["answer"]
                segment_document.word_count += len(segment["answer"])
            word_count_change += segment_document.word_count
            session.add(segment_document)
            document_segments.append(segment_document)

    with session_factory.create_session() as session, session.begin():
        dataset_document = session.get(Document, document_id)
        if dataset_document:
            assert dataset_document.word_count is not None
            dataset_document.word_count += word_count_change
            session.add(dataset_document)

    with session_factory.create_session() as session:
        dataset = session.get(Dataset, dataset_id)
        if dataset:
            VectorService.create_segments_vector(None, document_segments, dataset, document_config["doc_form"])

    redis_client.setex(indexing_cache_key, 600, "completed")
    end_at = time.perf_counter()
    logger.info(
        click.style(
            f"Segment batch created job: {job_id} latency: {end_at - start_at}",
            fg="green",
        )
    )

```

### api/tasks/annotation/update_annotation_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.models.document import Document
from models.dataset import Dataset
from services.dataset_service import DatasetCollectionBindingService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def update_annotation_to_index_task(
    annotation_id: str, question: str, tenant_id: str, app_id: str, collection_binding_id: str
):
    """
    Update annotation to index.
    :param annotation_id: annotation id
    :param question: question
    :param tenant_id: tenant id
    :param app_id: app id
    :param collection_binding_id: embedding binding id

    Usage: clean_dataset_task.delay(dataset_id, tenant_id, indexing_technique, index_struct)
    """
    logger.info(click.style(f"Start update index for annotation: {annotation_id}", fg="green"))
    start_at = time.perf_counter()

    try:
        dataset_collection_binding = DatasetCollectionBindingService.get_dataset_collection_binding_by_id_and_type(
            collection_binding_id, "annotation"
        )

        dataset = Dataset(
            id=app_id,
            tenant_id=tenant_id,
            indexing_technique=IndexTechniqueType.HIGH_QUALITY,
            embedding_model_provider=dataset_collection_binding.provider_name,
            embedding_model=dataset_collection_binding.model_name,
            collection_binding_id=dataset_collection_binding.id,
        )

        document = Document(
            page_content=question, metadata={"annotation_id": annotation_id, "app_id": app_id, "doc_id": annotation_id}
        )
        vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
        vector.delete_by_metadata_field("annotation_id", annotation_id)
        vector.add_texts([document])
        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Build index successful for annotation: {annotation_id} latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("Build index for annotation failed")

```

### api/tasks/annotation/add_annotation_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.models.document import Document
from models.dataset import Dataset
from services.dataset_service import DatasetCollectionBindingService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def add_annotation_to_index_task(
    annotation_id: str, question: str, tenant_id: str, app_id: str, collection_binding_id: str
):
    """
    Add annotation to index.
    :param annotation_id: annotation id
    :param question: question
    :param tenant_id: tenant id
    :param app_id: app id
    :param collection_binding_id: embedding binding id

    Usage: clean_dataset_task.delay(dataset_id, tenant_id, indexing_technique, index_struct)
    """
    logger.info(click.style(f"Start build index for annotation: {annotation_id}", fg="green"))
    start_at = time.perf_counter()

    try:
        dataset_collection_binding = DatasetCollectionBindingService.get_dataset_collection_binding_by_id_and_type(
            collection_binding_id, "annotation"
        )
        dataset = Dataset(
            id=app_id,
            tenant_id=tenant_id,
            indexing_technique=IndexTechniqueType.HIGH_QUALITY,
            embedding_model_provider=dataset_collection_binding.provider_name,
            embedding_model=dataset_collection_binding.model_name,
            collection_binding_id=dataset_collection_binding.id,
        )

        document = Document(
            page_content=question, metadata={"annotation_id": annotation_id, "app_id": app_id, "doc_id": annotation_id}
        )
        vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
        vector.create([document], duplicate_check=True)

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Build index successful for annotation: {annotation_id} latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("Build index for annotation failed")

```

### api/tasks/annotation/enable_annotation_reply_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset
from models.enums import CollectionBindingType
from models.model import App, AppAnnotationSetting, MessageAnnotation
from services.dataset_service import DatasetCollectionBindingService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def enable_annotation_reply_task(
    job_id: str,
    app_id: str,
    user_id: str,
    tenant_id: str,
    score_threshold: float,
    embedding_provider_name: str,
    embedding_model_name: str,
):
    """
    Async enable annotation reply task
    """
    logger.info(click.style(f"Start add app annotation to index: {app_id}", fg="green"))
    start_at = time.perf_counter()
    # get app info
    with session_factory.create_session() as session:
        app = session.scalar(
            select(App).where(App.id == app_id, App.tenant_id == tenant_id, App.status == "normal").limit(1)
        )

        if not app:
            logger.info(click.style(f"App not found: {app_id}", fg="red"))
            return

        annotations = session.scalars(select(MessageAnnotation).where(MessageAnnotation.app_id == app_id)).all()
        enable_app_annotation_key = f"enable_app_annotation_{str(app_id)}"
        enable_app_annotation_job_key = f"enable_app_annotation_job_{str(job_id)}"

        try:
            documents = []
            dataset_collection_binding = DatasetCollectionBindingService.get_dataset_collection_binding(
                embedding_provider_name, embedding_model_name, CollectionBindingType.ANNOTATION
            )
            annotation_setting = session.scalar(
                select(AppAnnotationSetting).where(AppAnnotationSetting.app_id == app_id).limit(1)
            )
            if annotation_setting:
                if dataset_collection_binding.id != annotation_setting.collection_binding_id:
                    old_dataset_collection_binding = (
                        DatasetCollectionBindingService.get_dataset_collection_binding_by_id_and_type(
                            annotation_setting.collection_binding_id, CollectionBindingType.ANNOTATION
                        )
                    )
                    if old_dataset_collection_binding and annotations:
                        old_dataset = Dataset(
                            id=app_id,
                            tenant_id=tenant_id,
                            indexing_technique=IndexTechniqueType.HIGH_QUALITY,
                            embedding_model_provider=old_dataset_collection_binding.provider_name,
                            embedding_model=old_dataset_collection_binding.model_name,
                            collection_binding_id=old_dataset_collection_binding.id,
                        )

                        old_vector = Vector(old_dataset, attributes=["doc_id", "annotation_id", "app_id"])
                        try:
                            old_vector.delete()
                        except Exception as e:
                            logger.info(click.style(f"Delete annotation index error: {str(e)}", fg="red"))
                annotation_setting.score_threshold = score_threshold
                annotation_setting.collection_binding_id = dataset_collection_binding.id
                annotation_setting.updated_user_id = user_id
                annotation_setting.updated_at = naive_utc_now()
                session.add(annotation_setting)
            else:
                new_app_annotation_setting = AppAnnotationSetting(
                    app_id=app_id,
                    score_threshold=score_threshold,
                    collection_binding_id=dataset_collection_binding.id,
                    created_user_id=user_id,
                    updated_user_id=user_id,
                )
                session.add(new_app_annotation_setting)

            dataset = Dataset(
                id=app_id,
                tenant_id=tenant_id,
                indexing_technique=IndexTechniqueType.HIGH_QUALITY,
                embedding_model_provider=embedding_provider_name,
                embedding_model=embedding_model_name,
                collection_binding_id=dataset_collection_binding.id,
            )
            if annotations:
                for annotation in annotations:
                    document = Document(
                        page_content=annotation.question_text,
                        metadata={"annotation_id": annotation.id, "app_id": app_id, "doc_id": annotation.id},
                    )
                    documents.append(document)

                vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
                try:
                    vector.delete_by_metadata_field("app_id", app_id)
                except Exception as e:
                    logger.info(click.style(f"Delete annotation index error: {str(e)}", fg="red"))
                vector.create(documents)
            session.commit()
            redis_client.setex(enable_app_annotation_job_key, 600, "completed")
            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"App annotations added to index: {app_id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception as e:
            logger.exception("Annotation batch created index failed")
            redis_client.setex(enable_app_annotation_job_key, 600, "error")
            enable_app_annotation_error_key = f"enable_app_annotation_error_{str(job_id)}"
            redis_client.setex(enable_app_annotation_error_key, 600, str(e))
            session.rollback()
        finally:
            redis_client.delete(enable_app_annotation_key)

```

### api/tasks/annotation/batch_import_annotations_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select
from werkzeug.exceptions import NotFound

from core.db.session_factory import session_factory
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from models.dataset import Dataset
from models.model import App, AppAnnotationSetting, MessageAnnotation
from services.dataset_service import DatasetCollectionBindingService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def batch_import_annotations_task(job_id: str, content_list: list[dict], app_id: str, tenant_id: str, user_id: str):
    """
    Add annotation to index.
    :param job_id: job_id
    :param content_list: content list
    :param app_id: app id
    :param tenant_id: tenant id
    :param user_id: user_id

    """
    logger.info(click.style(f"Start batch import annotation: {job_id}", fg="green"))
    start_at = time.perf_counter()
    indexing_cache_key = f"app_annotation_batch_import_{str(job_id)}"
    active_jobs_key = f"annotation_import_active:{tenant_id}"

    with session_factory.create_session() as session:
        # get app info
        app = session.scalar(
            select(App).where(App.id == app_id, App.tenant_id == tenant_id, App.status == "normal").limit(1)
        )

        if app:
            try:
                documents = []
                for content in content_list:
                    annotation = MessageAnnotation(
                        app_id=app.id, content=content["answer"], question=content["question"], account_id=user_id
                    )
                    session.add(annotation)
                    session.flush()

                    document = Document(
                        page_content=content["question"],
                        metadata={"annotation_id": annotation.id, "app_id": app_id, "doc_id": annotation.id},
                    )
                    documents.append(document)
                # if annotation reply is enabled , batch add annotations' index
                app_annotation_setting = session.scalar(
                    select(AppAnnotationSetting).where(AppAnnotationSetting.app_id == app_id).limit(1)
                )

                if app_annotation_setting:
                    dataset_collection_binding = (
                        DatasetCollectionBindingService.get_dataset_collection_binding_by_id_and_type(
                            app_annotation_setting.collection_binding_id, "annotation"
                        )
                    )
                    if not dataset_collection_binding:
                        raise NotFound("App annotation setting not found")
                    dataset = Dataset(
                        id=app_id,
                        tenant_id=tenant_id,
                        indexing_technique=IndexTechniqueType.HIGH_QUALITY,
                        embedding_model_provider=dataset_collection_binding.provider_name,
                        embedding_model=dataset_collection_binding.model_name,
                        collection_binding_id=dataset_collection_binding.id,
                    )

                    vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
                    vector.create(documents, duplicate_check=True)

                session.commit()
                redis_client.setex(indexing_cache_key, 600, "completed")
                end_at = time.perf_counter()
                logger.info(
                    click.style(
                        "Build index successful for batch import annotation: {} latency: {}".format(
                            job_id, end_at - start_at
                        ),
                        fg="green",
                    )
                )
            except Exception as e:
                session.rollback()
                redis_client.setex(indexing_cache_key, 600, "error")
                indexing_error_msg_key = f"app_annotation_batch_import_error_msg_{str(job_id)}"
                redis_client.setex(indexing_error_msg_key, 600, str(e))
                logger.exception("Build index for batch import annotations failed")
            finally:
                # Clean up active job tracking to release concurrency slot
                try:
                    redis_client.zrem(active_jobs_key, job_id)
                    logger.debug("Released concurrency slot for job: %s", job_id)
                except Exception as cleanup_error:
                    # Log but don't fail if cleanup fails - the job will be auto-expired
                    logger.warning("Failed to clean up active job tracking for %s: %s", job_id, cleanup_error)

```

### api/tasks/annotation/disable_annotation_reply_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import exists, select

from core.db.session_factory import session_factory
from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from extensions.ext_redis import redis_client
from models.dataset import Dataset
from models.model import App, AppAnnotationSetting, MessageAnnotation

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def disable_annotation_reply_task(job_id: str, app_id: str, tenant_id: str):
    """
    Async enable annotation reply task
    """
    logger.info(click.style(f"Start delete app annotations index: {app_id}", fg="green"))
    start_at = time.perf_counter()
    # get app info
    with session_factory.create_session() as session:
        app = session.scalar(
            select(App).where(App.id == app_id, App.tenant_id == tenant_id, App.status == "normal").limit(1)
        )
        annotations_exists = session.scalar(select(exists().where(MessageAnnotation.app_id == app_id)))
        if not app:
            logger.info(click.style(f"App not found: {app_id}", fg="red"))
            return

        app_annotation_setting = session.scalar(
            select(AppAnnotationSetting).where(AppAnnotationSetting.app_id == app_id).limit(1)
        )

        if not app_annotation_setting:
            logger.info(click.style(f"App annotation setting not found: {app_id}", fg="red"))
            return

        disable_app_annotation_key = f"disable_app_annotation_{str(app_id)}"
        disable_app_annotation_job_key = f"disable_app_annotation_job_{str(job_id)}"

        try:
            dataset = Dataset(
                id=app_id,
                tenant_id=tenant_id,
                indexing_technique=IndexTechniqueType.HIGH_QUALITY,
                collection_binding_id=app_annotation_setting.collection_binding_id,
            )

            try:
                if annotations_exists:
                    vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
                    vector.delete()
            except Exception:
                logger.exception("Delete annotation index failed when annotation deleted.")
            redis_client.setex(disable_app_annotation_job_key, 600, "completed")

            # delete annotation setting
            session.delete(app_annotation_setting)
            session.commit()

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"App annotations index deleted : {app_id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception as e:
            logger.exception("Annotation batch deleted index failed")
            redis_client.setex(disable_app_annotation_job_key, 600, "error")
            disable_app_annotation_error_key = f"disable_app_annotation_error_{str(job_id)}"
            redis_client.setex(disable_app_annotation_error_key, 600, str(e))
        finally:
            redis_client.delete(disable_app_annotation_key)

```

### api/tasks/annotation/delete_annotation_index_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.rag.datasource.vdb.vector_factory import Vector
from core.rag.index_processor.constant.index_type import IndexTechniqueType
from models.dataset import Dataset
from services.dataset_service import DatasetCollectionBindingService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def delete_annotation_index_task(annotation_id: str, app_id: str, tenant_id: str, collection_binding_id: str):
    """
    Async delete annotation index task
    """
    logger.info(click.style(f"Start delete app annotation index: {app_id}", fg="green"))
    start_at = time.perf_counter()
    try:
        dataset_collection_binding = DatasetCollectionBindingService.get_dataset_collection_binding_by_id_and_type(
            collection_binding_id, "annotation"
        )

        dataset = Dataset(
            id=app_id,
            tenant_id=tenant_id,
            indexing_technique=IndexTechniqueType.HIGH_QUALITY,
            collection_binding_id=dataset_collection_binding.id,
        )

        try:
            vector = Vector(dataset, attributes=["doc_id", "annotation_id", "app_id"])
            vector.delete_by_metadata_field("annotation_id", annotation_id)
        except Exception:
            logger.exception("Delete annotation index failed when annotation deleted.")
        end_at = time.perf_counter()
        logger.info(click.style(f"App annotations index deleted : {app_id} latency: {end_at - start_at}", fg="green"))
    except Exception:
        logger.exception("Annotation deleted index failed")

```

### api/tasks/recover_document_indexing_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.indexing_runner import DocumentIsPausedError, IndexingRunner
from models.dataset import Document

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def recover_document_indexing_task(dataset_id: str, document_id: str):
    """
    Async recover document
    :param dataset_id:
    :param document_id:

    Usage: recover_document_indexing_task.delay(dataset_id, document_id)
    """
    logger.info(click.style(f"Recover document: {document_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        document = session.scalar(
            select(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).limit(1)
        )

        if not document:
            logger.info(click.style(f"Document not found: {document_id}", fg="red"))
            return

        try:
            indexing_runner = IndexingRunner()
            if document.indexing_status in {"waiting", "parsing", "cleaning"}:
                indexing_runner.run([document])
            elif document.indexing_status == "splitting":
                indexing_runner.run_in_splitting_status(document)
            elif document.indexing_status == "indexing":
                indexing_runner.run_in_indexing_status(document)
            end_at = time.perf_counter()
            logger.info(click.style(f"Processed document: {document.id} latency: {end_at - start_at}", fg="green"))
        except DocumentIsPausedError as ex:
            logger.info(click.style(str(ex), fg="yellow"))
        except Exception:
            logger.exception("recover_document_indexing_task failed, document_id: %s", document_id)

```

### api/tasks/clean_notion_document_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from models.dataset import Dataset, Document, DocumentSegment

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def clean_notion_document_task(document_ids: list[str], dataset_id: str):
    """
    Clean document when document deleted.
    :param document_ids: document ids
    :param dataset_id: dataset id

    Usage: clean_notion_document_task.delay(document_ids, dataset_id)
    """
    logger.info(click.style(f"Start clean document when import form notion document deleted: {dataset_id}", fg="green"))
    start_at = time.perf_counter()
    total_index_node_ids = []

    with session_factory.create_session() as session:
        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()

        if not dataset:
            raise Exception("Document has no dataset")
        index_type = dataset.doc_form
        index_processor = IndexProcessorFactory(index_type).init_index_processor()

        document_delete_stmt = delete(Document).where(Document.id.in_(document_ids))
        session.execute(document_delete_stmt)

        for document_id in document_ids:
            segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document_id)).all()
            total_index_node_ids.extend([segment.index_node_id for segment in segments])

    with session_factory.create_session() as session:
        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if dataset:
            index_processor.clean(
                dataset, total_index_node_ids, with_keywords=True, delete_child_chunks=True, delete_summaries=True
            )

    with session_factory.create_session() as session, session.begin():
        segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.document_id.in_(document_ids))
        session.execute(segment_delete_stmt)

    end_at = time.perf_counter()
    logger.info(
        click.style(
            "Clean document when import form notion document deleted end :: {} latency: {}".format(
                dataset_id, end_at - start_at
            ),
            fg="green",
        )
    )

```

### api/tasks/trigger_subscription_refresh_tasks.py
```py
import logging
import time
from collections.abc import Mapping
from typing import Any

from celery import shared_task
from sqlalchemy import select
from sqlalchemy.orm import Session

from configs import dify_config
from core.db.session_factory import session_factory
from core.plugin.entities.plugin_daemon import CredentialType
from core.trigger.utils.locks import build_trigger_refresh_lock_key
from extensions.ext_redis import redis_client
from models.trigger import TriggerSubscription
from services.trigger.trigger_provider_service import TriggerProviderService

logger = logging.getLogger(__name__)


def _now_ts() -> int:
    return int(time.time())


def _load_subscription(session: Session, tenant_id: str, subscription_id: str) -> TriggerSubscription | None:
    return session.scalar(
        select(TriggerSubscription)
        .where(TriggerSubscription.tenant_id == tenant_id, TriggerSubscription.id == subscription_id)
        .limit(1)
    )


def _refresh_oauth_if_expired(tenant_id: str, subscription: TriggerSubscription, now: int) -> None:
    threshold_seconds: int = int(dify_config.TRIGGER_PROVIDER_CREDENTIAL_THRESHOLD_SECONDS)
    if (
        subscription.credential_expires_at != -1
        and int(subscription.credential_expires_at) <= now + threshold_seconds
        and CredentialType.of(subscription.credential_type) == CredentialType.OAUTH2
    ):
        logger.info(
            "Refreshing OAuth token: tenant=%s subscription_id=%s expires_at=%s now=%s",
            tenant_id,
            subscription.id,
            subscription.credential_expires_at,
            now,
        )
        try:
            result: Mapping[str, Any] = TriggerProviderService.refresh_oauth_token(
                tenant_id=tenant_id, subscription_id=subscription.id
            )
            logger.info(
                "OAuth token refreshed: tenant=%s subscription_id=%s result=%s", tenant_id, subscription.id, result
            )
        except Exception:
            logger.exception("OAuth refresh failed: tenant=%s subscription_id=%s", tenant_id, subscription.id)


def _refresh_subscription_if_expired(
    tenant_id: str,
    subscription: TriggerSubscription,
    now: int,
) -> None:
    threshold_seconds: int = int(dify_config.TRIGGER_PROVIDER_SUBSCRIPTION_THRESHOLD_SECONDS)
    if subscription.expires_at == -1 or int(subscription.expires_at) > now + threshold_seconds:
        logger.debug(
            "Subscription not due: tenant=%s subscription_id=%s expires_at=%s now=%s threshold=%s",
            tenant_id,
            subscription.id,
            subscription.expires_at,
            now,
            threshold_seconds,
        )
        return

    try:
        result: Mapping[str, Any] = TriggerProviderService.refresh_subscription(
            tenant_id=tenant_id, subscription_id=subscription.id, now=now
        )
        logger.info(
            "Subscription refreshed: tenant=%s subscription_id=%s result=%s",
            tenant_id,
            subscription.id,
            result.get("result"),
        )
    except Exception:
        logger.exception("Subscription refresh failed: tenant=%s id=%s", tenant_id, subscription.id)


@shared_task(queue="trigger_refresh_executor")
def trigger_subscription_refresh(tenant_id: str, subscription_id: str) -> None:
    """Refresh a trigger subscription if needed, guarded by a Redis in-flight lock."""
    lock_key: str = build_trigger_refresh_lock_key(tenant_id, subscription_id)
    if not redis_client.get(lock_key):
        logger.debug("Refresh lock missing, skip: %s", lock_key)
        return

    logger.info("Begin subscription refresh: tenant=%s id=%s", tenant_id, subscription_id)
    try:
        now: int = _now_ts()
        with session_factory.create_session() as session:
            subscription: TriggerSubscription | None = _load_subscription(session, tenant_id, subscription_id)

            if not subscription:
                logger.warning("Subscription not found: tenant=%s id=%s", tenant_id, subscription_id)
                return

            logger.debug(
                "Loaded subscription: tenant=%s id=%s cred_exp=%s sub_exp=%s now=%s",
                tenant_id,
                subscription.id,
                subscription.credential_expires_at,
                subscription.expires_at,
                now,
            )

            _refresh_oauth_if_expired(tenant_id=tenant_id, subscription=subscription, now=now)
            _refresh_subscription_if_expired(tenant_id=tenant_id, subscription=subscription, now=now)
    finally:
        try:
            redis_client.delete(lock_key)
            logger.debug("Lock released: %s", lock_key)
        except Exception:
            # Best-effort lock cleanup
            logger.warning("Failed to release lock: %s", lock_key, exc_info=True)

```

### api/tasks/document_indexing_sync_task.py
```py
import json
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.indexing_runner import DocumentIsPausedError, IndexingRunner
from core.rag.extractor.notion_extractor import NotionExtractor
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import IndexingStatus
from services.datasource_provider_service import DatasourceProviderService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def document_indexing_sync_task(dataset_id: str, document_id: str):
    """
    Async update document
    :param dataset_id:
    :param document_id:

    Usage: document_indexing_sync_task.delay(dataset_id, document_id)
    """
    logger.info(click.style(f"Start sync document: {document_id}", fg="green"))
    start_at = time.perf_counter()
    tenant_id = None

    with session_factory.create_session() as session, session.begin():
        document = session.query(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).first()

        if not document:
            logger.info(click.style(f"Document not found: {document_id}", fg="red"))
            return

        if document.indexing_status == IndexingStatus.PARSING:
            logger.info(click.style(f"Document {document_id} is already being processed, skipping", fg="yellow"))
            return

        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if not dataset:
            raise Exception("Dataset not found")

        data_source_info = document.data_source_info_dict
        if document.data_source_type != "notion_import":
            logger.info(click.style(f"Document {document_id} is not a notion_import, skipping", fg="yellow"))
            return

        if (
            not data_source_info
            or "notion_page_id" not in data_source_info
            or "notion_workspace_id" not in data_source_info
        ):
            raise ValueError("no notion page found")

        workspace_id = data_source_info["notion_workspace_id"]
        page_id = data_source_info["notion_page_id"]
        page_type = data_source_info["type"]
        page_edited_time = data_source_info["last_edited_time"]
        credential_id = data_source_info.get("credential_id")
        tenant_id = document.tenant_id
        index_type = document.doc_form

        segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document_id)).all()
        index_node_ids = [segment.index_node_id for segment in segments]

    # Get credentials from datasource provider
    datasource_provider_service = DatasourceProviderService()
    credential = datasource_provider_service.get_datasource_credentials(
        tenant_id=tenant_id,
        credential_id=credential_id,
        provider="notion_datasource",
        plugin_id="langgenius/notion_datasource",
    )

    if not credential:
        logger.error(
            "Datasource credential not found for document %s, tenant_id: %s, credential_id: %s",
            document_id,
            tenant_id,
            credential_id,
        )

        with session_factory.create_session() as session, session.begin():
            document = session.query(Document).filter_by(id=document_id).first()
            if document:
                document.indexing_status = IndexingStatus.ERROR
                document.error = "Datasource credential not found. Please reconnect your Notion workspace."
                document.stopped_at = naive_utc_now()
        return

    loader = NotionExtractor(
        notion_workspace_id=workspace_id,
        notion_obj_id=page_id,
        notion_page_type=page_type,
        notion_access_token=credential.get("integration_secret"),
        tenant_id=tenant_id,
    )

    last_edited_time = loader.get_notion_last_edited_time()
    if last_edited_time == page_edited_time:
        logger.info(click.style(f"Document {document_id} content unchanged, skipping sync", fg="yellow"))
        return

    logger.info(click.style(f"Document {document_id} content changed, starting sync", fg="green"))

    try:
        index_processor = IndexProcessorFactory(index_type).init_index_processor()
        with session_factory.create_session() as session:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if dataset:
                index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)
        logger.info(click.style(f"Cleaned vector index for document {document_id}", fg="green"))
    except Exception:
        logger.exception("Failed to clean vector index for document %s", document_id)

    with session_factory.create_session() as session, session.begin():
        document = session.query(Document).filter_by(id=document_id).first()
        if not document:
            logger.warning(click.style(f"Document {document_id} not found during sync", fg="yellow"))
            return

        data_source_info = document.data_source_info_dict
        data_source_info["last_edited_time"] = last_edited_time
        document.data_source_info = json.dumps(data_source_info)

        document.indexing_status = IndexingStatus.PARSING
        document.processing_started_at = naive_utc_now()

        segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.document_id == document_id)
        session.execute(segment_delete_stmt)

        logger.info(click.style(f"Deleted segments for document {document_id}", fg="green"))

    try:
        indexing_runner = IndexingRunner()
        with session_factory.create_session() as session:
            document = session.query(Document).filter_by(id=document_id).first()
            if document:
                indexing_runner.run([document])
        end_at = time.perf_counter()
        logger.info(click.style(f"Sync completed for document {document_id} latency: {end_at - start_at}", fg="green"))
    except DocumentIsPausedError as ex:
        logger.info(click.style(str(ex), fg="yellow"))
    except Exception as e:
        logger.exception("document_indexing_sync_task failed for document_id: %s", document_id)
        with session_factory.create_session() as session, session.begin():
            document = session.query(Document).filter_by(id=document_id).first()
            if document:
                document.indexing_status = IndexingStatus.ERROR
                document.error = str(e)
                document.stopped_at = naive_utc_now()

```

### api/tasks/enterprise_telemetry_task.py
```py
"""Celery worker for enterprise metric/log telemetry events.

This module defines the Celery task that processes telemetry envelopes
from the enterprise_telemetry queue. It deserializes envelopes and
dispatches them to the EnterpriseMetricHandler.
"""

import json
import logging

from celery import shared_task

from enterprise.telemetry.contracts import TelemetryEnvelope
from enterprise.telemetry.metric_handler import EnterpriseMetricHandler

logger = logging.getLogger(__name__)


@shared_task(queue="enterprise_telemetry")
def process_enterprise_telemetry(envelope_json: str) -> None:
    """Process enterprise metric/log telemetry envelope.

    This task is enqueued by the TelemetryGateway for metric/log-only
    events. It deserializes the envelope and dispatches to the handler.

    Best-effort processing: logs errors but never raises, to avoid
    failing user requests due to telemetry issues.

    Args:
        envelope_json: JSON-serialized TelemetryEnvelope.
    """
    try:
        # Deserialize envelope
        envelope_dict = json.loads(envelope_json)
        envelope = TelemetryEnvelope.model_validate(envelope_dict)

        # Process through handler
        handler = EnterpriseMetricHandler()
        handler.handle(envelope)

        logger.debug(
            "Successfully processed telemetry envelope: tenant_id=%s, event_id=%s, case=%s",
            envelope.tenant_id,
            envelope.event_id,
            envelope.case,
        )
    except Exception:
        # Best-effort: log and drop on error, never fail user request
        logger.warning(
            "Failed to process enterprise telemetry envelope, dropping event",
            exc_info=True,
        )

```

### api/tasks/app_generate/workflow_execute_task.py
```py
import contextlib
import logging
import uuid
from collections.abc import Generator, Mapping
from enum import StrEnum
from typing import Annotated, Any

from celery import shared_task
from flask import current_app, json
from graphon.runtime import GraphRuntimeState
from pydantic import BaseModel, Discriminator, Field, Tag
from sqlalchemy import Engine, select
from sqlalchemy.orm import Session, sessionmaker

from core.app.apps.advanced_chat.app_generator import AdvancedChatAppGenerator
from core.app.apps.message_based_app_generator import MessageBasedAppGenerator
from core.app.apps.workflow.app_generator import WorkflowAppGenerator
from core.app.entities.app_invoke_entities import (
    AdvancedChatAppGenerateEntity,
    InvokeFrom,
    WorkflowAppGenerateEntity,
)
from core.app.layers.pause_state_persist_layer import PauseStateLayerConfig, WorkflowResumptionContext
from core.repositories import DifyCoreRepositoryFactory
from extensions.ext_database import db
from libs.flask_utils import set_login_user
from models.account import Account
from models.enums import CreatorUserRole, WorkflowRunTriggeredFrom
from models.model import App, AppMode, Conversation, EndUser, Message
from models.workflow import Workflow, WorkflowNodeExecutionTriggeredFrom, WorkflowRun
from repositories.factory import DifyAPIRepositoryFactory

logger = logging.getLogger(__name__)

WORKFLOW_BASED_APP_EXECUTION_QUEUE = "workflow_based_app_execution"


class _UserType(StrEnum):
    ACCOUNT = "account"
    END_USER = "end_user"


class _Account(BaseModel):
    TYPE: _UserType = _UserType.ACCOUNT

    user_id: str


class _EndUser(BaseModel):
    TYPE: _UserType = _UserType.END_USER
    end_user_id: str


def _get_user_type_descriminator(value: Any):
    if isinstance(value, (_Account, _EndUser)):
        return value.TYPE
    elif isinstance(value, dict):
        user_type_str = value.get("TYPE")
        if user_type_str is None:
            return None
        try:
            user_type = _UserType(user_type_str)
        except ValueError:
            return None
        return user_type
    else:
        # return None if the discriminator value isn't found
        return None


type User = Annotated[
    (Annotated[_Account, Tag(_UserType.ACCOUNT)] | Annotated[_EndUser, Tag(_UserType.END_USER)]),
    Discriminator(_get_user_type_descriminator),
]


class AppExecutionParams(BaseModel):
    app_id: str
    workflow_id: str
    tenant_id: str
    app_mode: AppMode = AppMode.ADVANCED_CHAT
    user: User
    args: Mapping[str, Any]

    invoke_from: InvokeFrom
    streaming: bool = True
    call_depth: int = 0
    root_node_id: str | None = None
    workflow_run_id: str = Field(default_factory=lambda: str(uuid.uuid4()))

    @classmethod
    def new(
        cls,
        app_model: App,
        workflow: Workflow,
        user: Account | EndUser,
        args: Mapping[str, Any],
        invoke_from: InvokeFrom,
        streaming: bool = True,
        call_depth: int = 0,
        root_node_id: str | None = None,
        workflow_run_id: str | None = None,
    ):
        user_params: _Account | _EndUser
        if isinstance(user, Account):
            user_params = _Account(user_id=user.id)
        elif isinstance(user, EndUser):
            user_params = _EndUser(end_user_id=user.id)
        else:
            raise AssertionError("this statement should be unreachable.")
        return cls(
            app_id=app_model.id,
            workflow_id=workflow.id,
            tenant_id=app_model.tenant_id,
            app_mode=AppMode.value_of(app_model.mode),
            user=user_params,
            args=args,
            invoke_from=invoke_from,
            streaming=streaming,
            call_depth=call_depth,
            root_node_id=root_node_id,
            workflow_run_id=workflow_run_id or str(uuid.uuid4()),
        )


class _AppRunner:
    def __init__(self, session_factory: sessionmaker | Engine, exec_params: AppExecutionParams):
        if isinstance(session_factory, Engine):
            session_factory = sessionmaker(bind=session_factory)
        self._session_factory = session_factory
        self._exec_params = exec_params

    @contextlib.contextmanager
    def _session(self):
        with self._session_factory(expire_on_commit=False) as session, session.begin():
            yield session

    @contextlib.contextmanager
    def _setup_flask_context(self, user: Account | EndUser):
        flask_app = current_app._get_current_object()  # type: ignore
        with flask_app.app_context():
            set_login_user(user)
            yield

    def run(self):
        exec_params = self._exec_params
        with self._session() as session:
            workflow = session.get(Workflow, exec_params.workflow_id)
            if workflow is None:
                logger.warning("Workflow %s not found for execution", exec_params.workflow_id)
                return None
            app = session.get(App, workflow.app_id)
            if app is None:
                logger.warning("App %s not found for workflow %s", workflow.app_id, exec_params.workflow_id)
                return None

        pause_config = PauseStateLayerConfig(
            session_factory=self._session_factory,
            state_owner_user_id=workflow.created_by,
        )

        user = self._resolve_user()

        with self._setup_flask_context(user):
            response = self._run_app(
                app=app,
                workflow=workflow,
                user=user,
                pause_state_config=pause_config,
            )
            if not exec_params.streaming:
                return response

            assert isinstance(response, Generator)
            _publish_streaming_response(response, exec_params.workflow_run_id, exec_params.app_mode)

    def _run_app(
        self,
        *,
        app: App,
        workflow: Workflow,
        user: Account | EndUser,
        pause_state_config: PauseStateLayerConfig,
    ):
        exec_params = self._exec_params
        if exec_params.app_mode == AppMode.ADVANCED_CHAT:
            return AdvancedChatAppGenerator().generate(
                app_model=app,
                workflow=workflow,
                user=user,
                args=exec_params.args,
                invoke_from=exec_params.invoke_from,
                streaming=exec_params.streaming,
                workflow_run_id=exec_params.workflow_run_id,
                pause_state_config=pause_state_config,
            )
        if exec_params.app_mode == AppMode.WORKFLOW:
            return WorkflowAppGenerator().generate(
                app_model=app,
                workflow=workflow,
                user=user,
                args=exec_params.args,
                invoke_from=exec_params.invoke_from,
                streaming=exec_params.streaming,
                call_depth=exec_params.call_depth,
                root_node_id=exec_params.root_node_id,
                workflow_run_id=exec_params.workflow_run_id,
                pause_state_config=pause_state_config,
            )

        logger.error("Unsupported app mode for execution: %s", exec_params.app_mode)
        return None

    def _resolve_user(self) -> Account | EndUser:
        user_params = self._exec_params.user

        if isinstance(user_params, _EndUser):
            with self._session() as session:
                return session.get(EndUser, user_params.end_user_id)
        elif not isinstance(user_params, _Account):
            raise AssertionError(f"user should only be _Account or _EndUser, got {type(user_params)}")

        with self._session() as session:
            user: Account = session.get(Account, user_params.user_id)
            user.set_tenant_id(self._exec_params.tenant_id)

        return user


def _resolve_user_for_run(session: Session, workflow_run: WorkflowRun) -> Account | EndUser | None:
    role = CreatorUserRole(workflow_run.created_by_role)
    if role == CreatorUserRole.ACCOUNT:
        user = session.get(Account, workflow_run.created_by)
        if user:
            user.set_tenant_id(workflow_run.tenant_id)
        return user

    return session.get(EndUser, workflow_run.created_by)


def _publish_streaming_response(
    response_stream: Generator[str | Mapping[str, Any] | BaseModel, None, None],
    workflow_run_id: str,
    app_mode: AppMode,
) -> None:
    topic = MessageBasedAppGenerator.get_response_topic(app_mode, workflow_run_id)
    for event in response_stream:
        try:
            if isinstance(event, BaseModel):
                payload = json.dumps(event.model_dump(mode="json"), ensure_ascii=False)
            else:
                payload = json.dumps(event, ensure_ascii=False, default=str)
        except (TypeError, ValueError):
            logger.exception("error while encoding event")
            continue

        topic.publish(payload.encode())


@shared_task(queue=WORKFLOW_BASED_APP_EXECUTION_QUEUE)
def workflow_based_app_execution_task(
    payload: str,
) -> Generator[Mapping[str, Any] | str, None, None] | Mapping[str, Any] | None:
    exec_params = AppExecutionParams.model_validate_json(payload)

    logger.info("workflow_based_app_execution_task run with params: %s", exec_params)

    runner = _AppRunner(db.engine, exec_params=exec_params)
    return runner.run()


def _resume_app_execution(payload: dict[str, Any]) -> None:
    workflow_run_id = payload["workflow_run_id"]

    session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
    workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_maker=session_factory)

    pause_entity = workflow_run_repo.get_workflow_pause(workflow_run_id)
    if pause_entity is None:
        logger.warning("No pause entity found for workflow run %s", workflow_run_id)
        return

    try:
        resumption_context = WorkflowResumptionContext.loads(pause_entity.get_state().decode())
    except Exception:
        logger.exception("Failed to load resumption context for workflow run %s", workflow_run_id)
        return

    generate_entity = resumption_context.get_generate_entity()

    graph_runtime_state = GraphRuntimeState.from_snapshot(resumption_context.serialized_graph_runtime_state)

    conversation = None
    message = None
    with Session(db.engine, expire_on_commit=False) as session:
        workflow_run = session.get(WorkflowRun, workflow_run_id)
        if workflow_run is None:
            logger.warning("Workflow run %s not found during resume", workflow_run_id)
            return

        workflow = session.get(Workflow, workflow_run.workflow_id)
        if workflow is None:
            logger.warning("Workflow %s not found during resume", workflow_run.workflow_id)
            return

        app_model = session.get(App, workflow_run.app_id)
        if app_model is None:
            logger.warning("App %s not found during resume", workflow_run.app_id)
            return

        user = _resolve_user_for_run(session, workflow_run)
        if user is None:
            logger.warning("User %s not found for workflow run %s", workflow_run.created_by, workflow_run_id)
            return

        if isinstance(generate_entity, AdvancedChatAppGenerateEntity):
            if generate_entity.conversation_id is None:
                logger.warning("Conversation id missing in resumption context for workflow run %s", workflow_run_id)
                return

            conversation = session.get(Conversation, generate_entity.conversation_id)
            if conversation is None:
                logger.warning(
                    "Conversation %s not found for workflow run %s", generate_entity.conversation_id, workflow_run_id
                )
                return

            message = session.scalar(
                select(Message)
                .where(
                    Message.conversation_id == conversation.id,
                    Message.workflow_run_id == workflow_run_id,
                )
                .order_by(Message.created_at.desc())
                .limit(1)
            )
            if message is None:
                logger.warning("Message not found for workflow run %s", workflow_run_id)
                return

    if not isinstance(generate_entity, (AdvancedChatAppGenerateEntity, WorkflowAppGenerateEntity)):
        logger.error(
            "Unsupported resumption entity for workflow run %s (found %s)",
            workflow_run_id,
            type(generate_entity),
        )
        return

    workflow_run_repo.resume_workflow_pause(workflow_run_id, pause_entity)

    pause_config = PauseStateLayerConfig(
        session_factory=session_factory,
        state_owner_user_id=workflow.created_by,
    )

    if isinstance(generate_entity, AdvancedChatAppGenerateEntity):
        assert conversation is not None
        assert message is not None
        _resume_advanced_chat(
            app_model=app_model,
            workflow=workflow,
            user=user,
            conversation=conversation,
            message=message,
            generate_entity=generate_entity,
            graph_runtime_state=graph_runtime_state,
            session_factory=session_factory,
            pause_state_config=pause_config,
            workflow_run_id=workflow_run_id,
            workflow_run=workflow_run,
        )
    elif isinstance(generate_entity, WorkflowAppGenerateEntity):
        _resume_workflow(
            app_model=app_model,
            workflow=workflow,
            user=user,
            generate_entity=generate_entity,
            graph_runtime_state=graph_runtime_state,
            session_factory=session_factory,
            pause_state_config=pause_config,
            workflow_run_id=workflow_run_id,
            workflow_run=workflow_run,
            workflow_run_repo=workflow_run_repo,
            pause_entity=pause_entity,
        )


def _resume_advanced_chat(
    *,
    app_model: App,
    workflow: Workflow,
    user: Account | EndUser,
    conversation: Conversation,
    message: Message,
    generate_entity: AdvancedChatAppGenerateEntity,
    graph_runtime_state: GraphRuntimeState,
    session_factory: sessionmaker,
    pause_state_config: PauseStateLayerConfig,
    workflow_run_id: str,
    workflow_run: WorkflowRun,
) -> None:
    try:
        triggered_from = WorkflowRunTriggeredFrom(workflow_run.triggered_from)
    except ValueError:
        triggered_from = WorkflowRunTriggeredFrom.APP_RUN

    workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=app_model.id,
        triggered_from=triggered_from,
    )
    workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=app_model.id,
        triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
    )

    generator = AdvancedChatAppGenerator()

    try:
        response = generator.resume(
            app_model=app_model,
            workflow=workflow,
            user=user,
            conversation=conversation,
            message=message,
            application_generate_entity=generate_entity,
            workflow_execution_repository=workflow_execution_repository,
            workflow_node_execution_repository=workflow_node_execution_repository,
            graph_runtime_state=graph_runtime_state,
            pause_state_config=pause_state_config,
        )
    except Exception:
        logger.exception("Failed to resume chatflow execution for workflow run %s", workflow_run_id)
        raise

    if generate_entity.stream:
        assert isinstance(response, Generator)
        _publish_streaming_response(response, workflow_run_id, AppMode.ADVANCED_CHAT)


def _resume_workflow(
    *,
    app_model: App,
    workflow: Workflow,
    user: Account | EndUser,
    generate_entity: WorkflowAppGenerateEntity,
    graph_runtime_state: GraphRuntimeState,
    session_factory: sessionmaker,
    pause_state_config: PauseStateLayerConfig,
    workflow_run_id: str,
    workflow_run: WorkflowRun,
    workflow_run_repo,
    pause_entity,
) -> None:
    try:
        triggered_from = WorkflowRunTriggeredFrom(workflow_run.triggered_from)
    except ValueError:
        triggered_from = WorkflowRunTriggeredFrom.APP_RUN

    workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=app_model.id,
        triggered_from=triggered_from,
    )
    workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=app_model.id,
        triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
    )

    generator = WorkflowAppGenerator()

    try:
        response = generator.resume(
            app_model=app_model,
            workflow=workflow,
            user=user,
            application_generate_entity=generate_entity,
            graph_runtime_state=graph_runtime_state,
            workflow_execution_repository=workflow_execution_repository,
            workflow_node_execution_repository=workflow_node_execution_repository,
            pause_state_config=pause_state_config,
        )
    except Exception:
        logger.exception("Failed to resume workflow execution for workflow run %s", workflow_run_id)
        raise

    if generate_entity.stream:
        assert isinstance(response, Generator)
        _publish_streaming_response(response, workflow_run_id, AppMode.WORKFLOW)

    workflow_run_repo.delete_workflow_pause(pause_entity)


@shared_task(queue=WORKFLOW_BASED_APP_EXECUTION_QUEUE, name="resume_app_execution")
def resume_app_execution(payload: dict[str, Any]) -> None:
    _resume_app_execution(payload)

```

### api/tasks/app_generate/__init__.py
```py
from .workflow_execute_task import AppExecutionParams, resume_app_execution, workflow_based_app_execution_task

__all__ = ["AppExecutionParams", "resume_app_execution", "workflow_based_app_execution_task"]

```

### api/tasks/mail_invite_member_task.py
```py
import logging
import time

import click
from celery import shared_task

from configs import dify_config
from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_invite_member_mail_task(language: str, to: str, token: str, inviter_name: str, workspace_name: str):
    """
    Send invite member email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        token: Invitation token
        inviter_name: Name of the person sending the invitation
        workspace_name: Name of the workspace
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start send invite member mail to {to} in workspace {workspace_name}", fg="green"))
    start_at = time.perf_counter()

    try:
        url = f"{dify_config.CONSOLE_WEB_URL}/activate?token={token}"
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.INVITE_MEMBER,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "inviter_name": inviter_name,
                "workspace_name": workspace_name,
                "url": url,
            },
        )

        end_at = time.perf_counter()
        logger.info(click.style(f"Send invite member mail to {to} succeeded: latency: {end_at - start_at}", fg="green"))
    except Exception:
        logger.exception("Send invite member mail to %s failed", to)

```

### api/tasks/mail_account_deletion_task.py
```py
import logging
import time

import click
from celery import shared_task

from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_deletion_success_task(to: str, language: str = "en-US"):
    """
    Send account deletion success email with internationalization support.

    Args:
        to: Recipient email address
        language: Language code for email localization
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start send account deletion success email to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.ACCOUNT_DELETION_SUCCESS,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "email": to,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send account deletion success email to {to}: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send account deletion success email to %s failed", to)


@shared_task(queue="mail")
def send_account_deletion_verification_code(to: str, code: str, language: str = "en-US"):
    """
    Send account deletion verification code email with internationalization support.

    Args:
        to: Recipient email address
        code: Verification code
        language: Language code for email localization
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start send account deletion verification code email to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.ACCOUNT_DELETION_VERIFICATION,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "code": code,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                "Send account deletion verification code email to {} succeeded: latency: {}".format(
                    to, end_at - start_at
                ),
                fg="green",
            )
        )
    except Exception:
        logger.exception("Send account deletion verification code email to %s failed", to)

```

### api/tasks/batch_clean_document_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.tools.utils.web_reader_tool import get_image_upload_file_ids
from extensions.ext_storage import storage
from models.dataset import Dataset, DatasetMetadataBinding, DocumentSegment
from models.model import UploadFile

logger = logging.getLogger(__name__)

# Batch size for database operations to keep transactions short
BATCH_SIZE = 1000


@shared_task(queue="dataset")
def batch_clean_document_task(document_ids: list[str], dataset_id: str, doc_form: str | None, file_ids: list[str]):
    """
    Clean document when document deleted.
    :param document_ids: document ids
    :param dataset_id: dataset id
    :param doc_form: doc_form
    :param file_ids: file ids

    Usage: batch_clean_document_task.delay(document_ids, dataset_id)
    """
    logger.info(click.style("Start batch clean documents when documents deleted", fg="green"))
    start_at = time.perf_counter()
    if not doc_form:
        raise ValueError("doc_form is required")

    storage_keys_to_delete: list[str] = []
    index_node_ids: list[str] = []
    segment_ids: list[str] = []
    total_image_upload_file_ids: list[str] = []

    try:
        # ============ Step 1: Query segment and file data (short read-only transaction) ============
        with session_factory.create_session() as session:
            # Get segments info
            segments = session.scalars(
                select(DocumentSegment).where(DocumentSegment.document_id.in_(document_ids))
            ).all()

            if segments:
                index_node_ids = [segment.index_node_id for segment in segments]
                segment_ids = [segment.id for segment in segments]

                # Collect image file IDs from segment content
                for segment in segments:
                    image_upload_file_ids = get_image_upload_file_ids(segment.content)
                    total_image_upload_file_ids.extend(image_upload_file_ids)

            # Query storage keys for image files
            if total_image_upload_file_ids:
                image_files = session.scalars(
                    select(UploadFile).where(UploadFile.id.in_(total_image_upload_file_ids))
                ).all()
                storage_keys_to_delete.extend([f.key for f in image_files if f and f.key])

            # Query storage keys for document files
            if file_ids:
                files = session.scalars(select(UploadFile).where(UploadFile.id.in_(file_ids))).all()
                storage_keys_to_delete.extend([f.key for f in files if f and f.key])

        # ============ Step 2: Clean vector index (external service, fresh session for dataset) ============
        if index_node_ids:
            try:
                # Fetch dataset in a fresh session to avoid DetachedInstanceError
                with session_factory.create_session() as session:
                    dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
                    if not dataset:
                        logger.warning("Dataset not found for vector index cleanup, dataset_id: %s", dataset_id)
                    else:
                        index_processor = IndexProcessorFactory(doc_form).init_index_processor()
                        index_processor.clean(
                            dataset, index_node_ids, with_keywords=True, delete_child_chunks=True, delete_summaries=True
                        )
            except Exception:
                logger.exception(
                    "Failed to clean vector index for dataset_id: %s, document_ids: %s, index_node_ids count: %d",
                    dataset_id,
                    document_ids,
                    len(index_node_ids),
                )

        # ============ Step 3: Delete metadata binding (separate short transaction) ============
        try:
            with session_factory.create_session() as session:
                deleted_count = (
                    session.query(DatasetMetadataBinding)
                    .where(
                        DatasetMetadataBinding.dataset_id == dataset_id,
                        DatasetMetadataBinding.document_id.in_(document_ids),
                    )
                    .delete(synchronize_session=False)
                )
                session.commit()
                logger.debug("Deleted %d metadata bindings for dataset_id: %s", deleted_count, dataset_id)
        except Exception:
            logger.exception(
                "Failed to delete metadata bindings for dataset_id: %s, document_ids: %s",
                dataset_id,
                document_ids,
            )

        # ============ Step 4: Batch delete UploadFile records (multiple short transactions) ============
        if total_image_upload_file_ids:
            failed_batches = 0
            total_batches = (len(total_image_upload_file_ids) + BATCH_SIZE - 1) // BATCH_SIZE
            for i in range(0, len(total_image_upload_file_ids), BATCH_SIZE):
                batch = total_image_upload_file_ids[i : i + BATCH_SIZE]
                try:
                    with session_factory.create_session() as session:
                        stmt = delete(UploadFile).where(UploadFile.id.in_(batch))
                        session.execute(stmt)
                        session.commit()
                except Exception:
                    failed_batches += 1
                    logger.exception(
                        "Failed to delete image UploadFile batch %d-%d for dataset_id: %s",
                        i,
                        i + len(batch),
                        dataset_id,
                    )
            if failed_batches > 0:
                logger.warning(
                    "Image UploadFile deletion: %d/%d batches failed for dataset_id: %s",
                    failed_batches,
                    total_batches,
                    dataset_id,
                )

        # ============ Step 5: Batch delete DocumentSegment records (multiple short transactions) ============
        if segment_ids:
            failed_batches = 0
            total_batches = (len(segment_ids) + BATCH_SIZE - 1) // BATCH_SIZE
            for i in range(0, len(segment_ids), BATCH_SIZE):
                batch = segment_ids[i : i + BATCH_SIZE]
                try:
                    with session_factory.create_session() as session:
                        segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.id.in_(batch))
                        session.execute(segment_delete_stmt)
                        session.commit()
                except Exception:
                    failed_batches += 1
                    logger.exception(
                        "Failed to delete DocumentSegment batch %d-%d for dataset_id: %s, document_ids: %s",
                        i,
                        i + len(batch),
                        dataset_id,
                        document_ids,
                    )
            if failed_batches > 0:
                logger.warning(
                    "DocumentSegment deletion: %d/%d batches failed, document_ids: %s",
                    failed_batches,
                    total_batches,
                    document_ids,
                )

        # ============ Step 6: Delete document-associated files (separate short transaction) ============
        if file_ids:
            try:
                with session_factory.create_session() as session:
                    stmt = delete(UploadFile).where(UploadFile.id.in_(file_ids))
                    session.execute(stmt)
                    session.commit()
            except Exception:
                logger.exception(
                    "Failed to delete document UploadFile records for dataset_id: %s, file_ids: %s",
                    dataset_id,
                    file_ids,
                )

        # ============ Step 7: Delete storage files (I/O operations, no DB transaction) ============
        storage_delete_failures = 0
        for storage_key in storage_keys_to_delete:
            try:
                storage.delete(storage_key)
            except Exception:
                storage_delete_failures += 1
                logger.exception("Failed to delete file from storage, key: %s", storage_key)
        if storage_delete_failures > 0:
            logger.warning(
                "Storage file deletion completed with %d failures out of %d total files for dataset_id: %s",
                storage_delete_failures,
                len(storage_keys_to_delete),
                dataset_id,
            )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Cleaned documents when documents deleted latency: {end_at - start_at:.2f}s, "
                f"dataset_id: {dataset_id}, document_ids: {document_ids}, "
                f"segments: {len(segment_ids)}, image_files: {len(total_image_upload_file_ids)}, "
                f"storage_files: {len(storage_keys_to_delete)}",
                fg="green",
            )
        )
    except Exception:
        logger.exception(
            "Batch clean documents failed for dataset_id: %s, document_ids: %s",
            dataset_id,
            document_ids,
        )

```

### api/tasks/add_document_to_index_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.db.session_factory import session_factory
from core.rag.index_processor.constant.doc_type import DocType
from core.rag.index_processor.constant.index_type import IndexStructureType
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.rag.models.document import AttachmentDocument, ChildDocument, Document
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import DatasetAutoDisableLog, DocumentSegment
from models.dataset import Document as DatasetDocument
from models.enums import IndexingStatus, SegmentStatus

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def add_document_to_index_task(dataset_document_id: str):
    """
    Async Add document to index
    :param dataset_document_id:

    Usage: add_document_to_index_task.delay(dataset_document_id)
    """
    logger.info(click.style(f"Start add document to index: {dataset_document_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        dataset_document = session.query(DatasetDocument).where(DatasetDocument.id == dataset_document_id).first()
        if not dataset_document:
            logger.info(click.style(f"Document not found: {dataset_document_id}", fg="red"))
            return

        if dataset_document.indexing_status != IndexingStatus.COMPLETED:
            return

        indexing_cache_key = f"document_{dataset_document.id}_indexing"

        try:
            dataset = dataset_document.dataset
            if not dataset:
                raise Exception(f"Document {dataset_document.id} dataset {dataset_document.dataset_id} doesn't exist.")

            segments = (
                session.query(DocumentSegment)
                .where(
                    DocumentSegment.document_id == dataset_document.id,
                    DocumentSegment.status == SegmentStatus.COMPLETED,
                )
                .order_by(DocumentSegment.position.asc())
                .all()
            )

            documents = []
            multimodal_documents = []
            for segment in segments:
                document = Document(
                    page_content=segment.content,
                    metadata={
                        "doc_id": segment.index_node_id,
                        "doc_hash": segment.index_node_hash,
                        "document_id": segment.document_id,
                        "dataset_id": segment.dataset_id,
                    },
                )
                if dataset_document.doc_form == IndexStructureType.PARENT_CHILD_INDEX:
                    child_chunks = segment.get_child_chunks()
                    if child_chunks:
                        child_documents = []
                        for child_chunk in child_chunks:
                            child_document = ChildDocument(
                                page_content=child_chunk.content,
                                metadata={
                                    "doc_id": child_chunk.index_node_id,
                                    "doc_hash": child_chunk.index_node_hash,
                                    "document_id": segment.document_id,
                                    "dataset_id": segment.dataset_id,
                                },
                            )
                            child_documents.append(child_document)
                        document.children = child_documents
                if dataset.is_multimodal:
                    for attachment in segment.attachments:
                        multimodal_documents.append(
                            AttachmentDocument(
                                page_content=attachment["name"],
                                metadata={
                                    "doc_id": attachment["id"],
                                    "doc_hash": "",
                                    "document_id": segment.document_id,
                                    "dataset_id": segment.dataset_id,
                                    "doc_type": DocType.IMAGE,
                                },
                            )
                        )
                documents.append(document)

            index_type = dataset.doc_form
            index_processor = IndexProcessorFactory(index_type).init_index_processor()
            index_processor.load(dataset, documents, multimodal_documents=multimodal_documents)

            # delete auto disable log
            session.query(DatasetAutoDisableLog).where(
                DatasetAutoDisableLog.document_id == dataset_document.id
            ).delete()

            # update segment to enable
            session.query(DocumentSegment).where(DocumentSegment.document_id == dataset_document.id).update(
                {
                    DocumentSegment.enabled: True,
                    DocumentSegment.disabled_at: None,
                    DocumentSegment.disabled_by: None,
                    DocumentSegment.updated_at: naive_utc_now(),
                }
            )
            session.commit()

            # Enable summary indexes for all segments in this document
            from services.summary_index_service import SummaryIndexService

            segment_ids_list = [segment.id for segment in segments]
            if segment_ids_list:
                try:
                    SummaryIndexService.enable_summaries_for_segments(
                        dataset=dataset,
                        segment_ids=segment_ids_list,
                    )
                except Exception as e:
                    logger.warning("Failed to enable summaries for document %s: %s", dataset_document.id, str(e))

            end_at = time.perf_counter()
            logger.info(
                click.style(f"Document added to index: {dataset_document.id} latency: {end_at - start_at}", fg="green")
            )
        except Exception as e:
            logger.exception("add document to index failed")
            dataset_document.enabled = False
            dataset_document.disabled_at = naive_utc_now()
            dataset_document.indexing_status = IndexingStatus.ERROR
            dataset_document.error = str(e)
            session.commit()
        finally:
            redis_client.delete(indexing_cache_key)

```

### api/tasks/workflow_node_execution_tasks.py
```py
"""
Celery tasks for asynchronous workflow node execution storage operations.

These tasks provide asynchronous storage capabilities for workflow node execution data,
improving performance by offloading storage operations to background workers.
"""

import json
import logging

from celery import shared_task
from graphon.entities.workflow_node_execution import (
    WorkflowNodeExecution,
)
from graphon.workflow_type_encoder import WorkflowRuntimeTypeConverter
from sqlalchemy import select

from core.db.session_factory import session_factory
from models import CreatorUserRole, WorkflowNodeExecutionModel
from models.workflow import WorkflowNodeExecutionTriggeredFrom

logger = logging.getLogger(__name__)


@shared_task(queue="workflow_storage", bind=True, max_retries=3, default_retry_delay=60)
def save_workflow_node_execution_task(
    self,
    execution_data: dict,
    tenant_id: str,
    app_id: str,
    triggered_from: str,
    creator_user_id: str,
    creator_user_role: str,
) -> bool:
    """
    Asynchronously save or update a workflow node execution to the database.

    Args:
        execution_data: Serialized WorkflowNodeExecution data
        tenant_id: Tenant ID for multi-tenancy
        app_id: Application ID
        triggered_from: Source of the execution trigger
        creator_user_id: ID of the user who created the execution
        creator_user_role: Role of the user who created the execution

    Returns:
        True if successful, False otherwise
    """
    try:
        with session_factory.create_session() as session:
            # Deserialize execution data
            execution = WorkflowNodeExecution.model_validate(execution_data)

            # Check if node execution already exists
            existing_execution = session.scalar(
                select(WorkflowNodeExecutionModel).where(WorkflowNodeExecutionModel.id == execution.id)
            )

            if existing_execution:
                # Update existing node execution
                _update_node_execution_from_domain(existing_execution, execution)
                logger.debug("Updated existing workflow node execution: %s", execution.id)
            else:
                # Create new node execution
                node_execution = _create_node_execution_from_domain(
                    execution=execution,
                    tenant_id=tenant_id,
                    app_id=app_id,
                    triggered_from=WorkflowNodeExecutionTriggeredFrom(triggered_from),
                    creator_user_id=creator_user_id,
                    creator_user_role=CreatorUserRole(creator_user_role),
                )
                session.add(node_execution)
                logger.debug("Created new workflow node execution: %s", execution.id)

            session.commit()
            return True

    except Exception as e:
        logger.exception("Failed to save workflow node execution %s", execution_data.get("id", "unknown"))
        # Retry the task with exponential backoff
        raise self.retry(exc=e, countdown=60 * (2**self.request.retries))


def _create_node_execution_from_domain(
    execution: WorkflowNodeExecution,
    tenant_id: str,
    app_id: str,
    triggered_from: WorkflowNodeExecutionTriggeredFrom,
    creator_user_id: str,
    creator_user_role: CreatorUserRole,
) -> WorkflowNodeExecutionModel:
    """
    Create a WorkflowNodeExecutionModel database model from a WorkflowNodeExecution domain entity.
    """
    node_execution = WorkflowNodeExecutionModel()
    node_execution.id = execution.id
    node_execution.tenant_id = tenant_id
    node_execution.app_id = app_id
    node_execution.workflow_id = execution.workflow_id
    node_execution.triggered_from = triggered_from
    node_execution.workflow_run_id = execution.workflow_execution_id
    node_execution.index = execution.index
    node_execution.predecessor_node_id = execution.predecessor_node_id
    node_execution.node_id = execution.node_id
    node_execution.node_type = execution.node_type
    node_execution.title = execution.title
    node_execution.node_execution_id = execution.node_execution_id

    # Serialize complex data as JSON
    json_converter = WorkflowRuntimeTypeConverter()
    node_execution.inputs = json.dumps(json_converter.to_json_encodable(execution.inputs)) if execution.inputs else "{}"
    node_execution.process_data = (
        json.dumps(json_converter.to_json_encodable(execution.process_data)) if execution.process_data else "{}"
    )
    node_execution.outputs = (
        json.dumps(json_converter.to_json_encodable(execution.outputs)) if execution.outputs else "{}"
    )
    # Convert metadata enum keys to strings for JSON serialization
    if execution.metadata:
        metadata_for_json = {
            key.value if hasattr(key, "value") else str(key): value for key, value in execution.metadata.items()
        }
        node_execution.execution_metadata = json.dumps(json_converter.to_json_encodable(metadata_for_json))
    else:
        node_execution.execution_metadata = "{}"

    node_execution.status = execution.status
    node_execution.error = execution.error
    node_execution.elapsed_time = execution.elapsed_time
    node_execution.created_by_role = creator_user_role
    node_execution.created_by = creator_user_id
    node_execution.created_at = execution.created_at
    node_execution.finished_at = execution.finished_at

    return node_execution


def _update_node_execution_from_domain(node_execution: WorkflowNodeExecutionModel, execution: WorkflowNodeExecution):
    """
    Update a WorkflowNodeExecutionModel database model from a WorkflowNodeExecution domain entity.
    """
    # Update serialized data
    json_converter = WorkflowRuntimeTypeConverter()
    node_execution.inputs = json.dumps(json_converter.to_json_encodable(execution.inputs)) if execution.inputs else "{}"
    node_execution.process_data = (
        json.dumps(json_converter.to_json_encodable(execution.process_data)) if execution.process_data else "{}"
    )
    node_execution.outputs = (
        json.dumps(json_converter.to_json_encodable(execution.outputs)) if execution.outputs else "{}"
    )
    # Convert metadata enum keys to strings for JSON serialization
    if execution.metadata:
        metadata_for_json = {
            key.value if hasattr(key, "value") else str(key): value for key, value in execution.metadata.items()
        }
        node_execution.execution_metadata = json.dumps(json_converter.to_json_encodable(metadata_for_json))
    else:
        node_execution.execution_metadata = "{}"

    # Update other fields
    node_execution.status = execution.status
    node_execution.error = execution.error
    node_execution.elapsed_time = execution.elapsed_time
    node_execution.finished_at = execution.finished_at

```

### api/tasks/remove_document_from_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import Document, DocumentSegment

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def remove_document_from_index_task(document_id: str):
    """
    Async Remove document from index
    :param document_id: document id

    Usage: remove_document_from_index.delay(document_id)
    """
    logger.info(click.style(f"Start remove document segments from index: {document_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        document = session.query(Document).where(Document.id == document_id).first()
        if not document:
            logger.info(click.style(f"Document not found: {document_id}", fg="red"))
            return

        if document.indexing_status != "completed":
            logger.info(click.style(f"Document is not completed, remove is not allowed: {document_id}", fg="red"))
            return

        indexing_cache_key = f"document_{document.id}_indexing"

        try:
            dataset = document.dataset

            if not dataset:
                raise Exception("Document has no dataset")

            index_processor = IndexProcessorFactory(document.doc_form).init_index_processor()

            segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document.id)).all()

            # Disable summary indexes for all segments in this document
            from services.summary_index_service import SummaryIndexService

            segment_ids_list = [segment.id for segment in segments]
            if segment_ids_list:
                try:
                    SummaryIndexService.disable_summaries_for_segments(
                        dataset=dataset,
                        segment_ids=segment_ids_list,
                        disabled_by=document.disabled_by,
                    )
                except Exception as e:
                    logger.warning("Failed to disable summaries for document %s: %s", document.id, str(e))

            index_node_ids = [segment.index_node_id for segment in segments]
            if index_node_ids:
                try:
                    index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=False)
                except Exception:
                    logger.exception("clean dataset %s from index failed", dataset.id)
            # update segment to disable
            session.query(DocumentSegment).where(DocumentSegment.document_id == document.id).update(
                {
                    DocumentSegment.enabled: False,
                    DocumentSegment.disabled_at: naive_utc_now(),
                    DocumentSegment.disabled_by: document.disabled_by,
                    DocumentSegment.updated_at: naive_utc_now(),
                }
            )
            session.commit()

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"Document removed from index: {document.id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception:
            logger.exception("remove document from index failed")
            if not document.archived:
                document.enabled = True
                session.commit()
        finally:
            redis_client.delete(indexing_cache_key)

```

### api/tasks/sync_website_document_indexing_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.indexing_runner import IndexingRunner
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from extensions.ext_redis import redis_client
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import IndexingStatus
from services.feature_service import FeatureService

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def sync_website_document_indexing_task(dataset_id: str, document_id: str):
    """
    Async process document
    :param dataset_id:
    :param document_id:

    Usage: sync_website_document_indexing_task.delay(dataset_id, document_id)
    """
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        dataset = session.scalar(select(Dataset).where(Dataset.id == dataset_id).limit(1))
        if dataset is None:
            raise ValueError("Dataset not found")

        sync_indexing_cache_key = f"document_{document_id}_is_sync"
        # check document limit
        features = FeatureService.get_features(dataset.tenant_id)
        try:
            if features.billing.enabled:
                vector_space = features.vector_space
                if 0 < vector_space.limit <= vector_space.size:
                    raise ValueError(
                        "Your total number of documents plus the number of uploads have over the limit of "
                        "your subscription."
                    )
        except Exception as e:
            document = session.scalar(
                select(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).limit(1)
            )
            if document:
                document.indexing_status = IndexingStatus.ERROR
                document.error = str(e)
                document.stopped_at = naive_utc_now()
                session.add(document)
                session.commit()
            redis_client.delete(sync_indexing_cache_key)
            return

        logger.info(click.style(f"Start sync website document: {document_id}", fg="green"))
        document = session.scalar(
            select(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).limit(1)
        )
        if not document:
            logger.info(click.style(f"Document not found: {document_id}", fg="yellow"))
            return
        try:
            # clean old data
            index_processor = IndexProcessorFactory(document.doc_form).init_index_processor()

            segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document_id)).all()
            if segments:
                index_node_ids = [segment.index_node_id for segment in segments]
                # delete from vector index
                index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)

            segment_ids = [segment.id for segment in segments]
            segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.id.in_(segment_ids))
            session.execute(segment_delete_stmt)
            session.commit()

            document.indexing_status = IndexingStatus.PARSING
            document.processing_started_at = naive_utc_now()
            session.add(document)
            session.commit()

            indexing_runner = IndexingRunner()
            indexing_runner.run([document])
            redis_client.delete(sync_indexing_cache_key)
        except Exception as ex:
            document.indexing_status = IndexingStatus.ERROR
            document.error = str(ex)
            document.stopped_at = naive_utc_now()
            session.add(document)
            session.commit()
            logger.info(click.style(str(ex), fg="yellow"))
            redis_client.delete(sync_indexing_cache_key)
            logger.exception("sync_website_document_indexing_task failed, document_id: %s", document_id)
        end_at = time.perf_counter()
        logger.info(click.style(f"Sync document: {document_id} latency: {end_at - start_at}", fg="green"))

```

### api/tasks/workflow_execution_tasks.py
```py
"""
Celery tasks for asynchronous workflow execution storage operations.

These tasks provide asynchronous storage capabilities for workflow execution data,
improving performance by offloading storage operations to background workers.
"""

import json
import logging

from celery import shared_task
from graphon.entities import WorkflowExecution
from graphon.workflow_type_encoder import WorkflowRuntimeTypeConverter
from sqlalchemy import select

from core.db.session_factory import session_factory
from models import CreatorUserRole, WorkflowRun
from models.enums import WorkflowRunTriggeredFrom

logger = logging.getLogger(__name__)


@shared_task(queue="workflow_storage", bind=True, max_retries=3, default_retry_delay=60)
def save_workflow_execution_task(
    self,
    execution_data: dict,
    tenant_id: str,
    app_id: str,
    triggered_from: str,
    creator_user_id: str,
    creator_user_role: str,
) -> bool:
    """
    Asynchronously save or update a workflow execution to the database.

    Args:
        execution_data: Serialized WorkflowExecution data
        tenant_id: Tenant ID for multi-tenancy
        app_id: Application ID
        triggered_from: Source of the execution trigger
        creator_user_id: ID of the user who created the execution
        creator_user_role: Role of the user who created the execution

    Returns:
        True if successful, False otherwise
    """
    try:
        with session_factory.create_session() as session:
            # Deserialize execution data
            execution = WorkflowExecution.model_validate(execution_data)

            # Check if workflow run already exists
            existing_run = session.scalar(select(WorkflowRun).where(WorkflowRun.id == execution.id_))

            if existing_run:
                # Update existing workflow run
                _update_workflow_run_from_execution(existing_run, execution)
                logger.debug("Updated existing workflow run: %s", execution.id_)
            else:
                # Create new workflow run
                workflow_run = _create_workflow_run_from_execution(
                    execution=execution,
                    tenant_id=tenant_id,
                    app_id=app_id,
                    triggered_from=WorkflowRunTriggeredFrom(triggered_from),
                    creator_user_id=creator_user_id,
                    creator_user_role=CreatorUserRole(creator_user_role),
                )
                session.add(workflow_run)
                logger.debug("Created new workflow run: %s", execution.id_)

            session.commit()
            return True

    except Exception as e:
        logger.exception("Failed to save workflow execution %s", execution_data.get("id_", "unknown"))
        # Retry the task with exponential backoff
        raise self.retry(exc=e, countdown=60 * (2**self.request.retries))


def _create_workflow_run_from_execution(
    execution: WorkflowExecution,
    tenant_id: str,
    app_id: str,
    triggered_from: WorkflowRunTriggeredFrom,
    creator_user_id: str,
    creator_user_role: CreatorUserRole,
) -> WorkflowRun:
    """
    Create a WorkflowRun database model from a WorkflowExecution domain entity.
    """
    workflow_run = WorkflowRun()
    workflow_run.id = execution.id_
    workflow_run.tenant_id = tenant_id
    workflow_run.app_id = app_id
    workflow_run.workflow_id = execution.workflow_id
    from models.workflow import WorkflowType as ModelWorkflowType

    workflow_run.type = ModelWorkflowType(execution.workflow_type.value)
    workflow_run.triggered_from = triggered_from
    workflow_run.version = execution.workflow_version
    json_converter = WorkflowRuntimeTypeConverter()
    workflow_run.graph = json.dumps(json_converter.to_json_encodable(execution.graph))
    workflow_run.inputs = json.dumps(json_converter.to_json_encodable(execution.inputs))
    workflow_run.status = execution.status
    workflow_run.outputs = (
        json.dumps(json_converter.to_json_encodable(execution.outputs)) if execution.outputs else "{}"
    )
    workflow_run.error = execution.error_message
    workflow_run.elapsed_time = execution.elapsed_time
    workflow_run.total_tokens = execution.total_tokens
    workflow_run.total_steps = execution.total_steps
    workflow_run.created_by_role = creator_user_role
    workflow_run.created_by = creator_user_id
    workflow_run.created_at = execution.started_at
    workflow_run.finished_at = execution.finished_at

    return workflow_run


def _update_workflow_run_from_execution(workflow_run: WorkflowRun, execution: WorkflowExecution):
    """
    Update a WorkflowRun database model from a WorkflowExecution domain entity.
    """
    json_converter = WorkflowRuntimeTypeConverter()
    workflow_run.status = execution.status
    workflow_run.outputs = (
        json.dumps(json_converter.to_json_encodable(execution.outputs)) if execution.outputs else "{}"
    )
    workflow_run.error = execution.error_message
    workflow_run.elapsed_time = execution.elapsed_time
    workflow_run.total_tokens = execution.total_tokens
    workflow_run.total_steps = execution.total_steps
    workflow_run.finished_at = execution.finished_at

```

### api/tasks/clean_dataset_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from core.tools.utils.web_reader_tool import get_image_upload_file_ids
from extensions.ext_storage import storage
from models import WorkflowType
from models.dataset import (
    AppDatasetJoin,
    Dataset,
    DatasetMetadata,
    DatasetMetadataBinding,
    DatasetProcessRule,
    DatasetQuery,
    Document,
    DocumentSegment,
    Pipeline,
    SegmentAttachmentBinding,
)
from models.model import UploadFile
from models.workflow import Workflow

logger = logging.getLogger(__name__)


# Add import statement for ValueError
@shared_task(queue="dataset")
def clean_dataset_task(
    dataset_id: str,
    tenant_id: str,
    indexing_technique: str,
    index_struct: str,
    collection_binding_id: str,
    doc_form: str,
    pipeline_id: str | None = None,
):
    """
    Clean dataset when dataset deleted.
    :param dataset_id: dataset id
    :param tenant_id: tenant id
    :param indexing_technique: indexing technique
    :param index_struct: index struct dict
    :param collection_binding_id: collection binding id
    :param doc_form: dataset form

    Usage: clean_dataset_task.delay(dataset_id, tenant_id, indexing_technique, index_struct)
    """
    logger.info(click.style(f"Start clean dataset when dataset deleted: {dataset_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        try:
            dataset = Dataset(
                id=dataset_id,
                tenant_id=tenant_id,
                indexing_technique=indexing_technique,
                index_struct=index_struct,
                collection_binding_id=collection_binding_id,
            )
            documents = session.scalars(select(Document).where(Document.dataset_id == dataset_id)).all()
            segments = session.scalars(select(DocumentSegment).where(DocumentSegment.dataset_id == dataset_id)).all()
            # Use JOIN to fetch attachments with bindings in a single query
            attachments_with_bindings = session.execute(
                select(SegmentAttachmentBinding, UploadFile)
                .join(UploadFile, UploadFile.id == SegmentAttachmentBinding.attachment_id)
                .where(
                    SegmentAttachmentBinding.tenant_id == tenant_id,
                    SegmentAttachmentBinding.dataset_id == dataset_id,
                )
            ).all()

            # Enhanced validation: Check if doc_form is None, empty string, or contains only whitespace
            # This ensures all invalid doc_form values are properly handled
            if doc_form is None or (isinstance(doc_form, str) and not doc_form.strip()):
                # Use default paragraph index type for empty/invalid datasets to enable vector database cleanup
                from core.rag.index_processor.constant.index_type import IndexStructureType

                doc_form = IndexStructureType.PARAGRAPH_INDEX
                logger.info(
                    click.style(
                        f"Invalid doc_form detected, using default index type for cleanup: {doc_form}",
                        fg="yellow",
                    )
                )

            # Add exception handling around IndexProcessorFactory.clean() to prevent single point of failure
            # This ensures Document/Segment deletion can continue even if vector database cleanup fails
            try:
                index_processor = IndexProcessorFactory(doc_form).init_index_processor()
                index_processor.clean(dataset, None, with_keywords=True, delete_child_chunks=True)
                logger.info(click.style(f"Successfully cleaned vector database for dataset: {dataset_id}", fg="green"))
            except Exception:
                logger.exception(click.style(f"Failed to clean vector database for dataset {dataset_id}", fg="red"))
                # Continue with document and segment deletion even if vector cleanup fails
                logger.info(
                    click.style(f"Continuing with document and segment deletion for dataset: {dataset_id}", fg="yellow")
                )

            if documents is None or len(documents) == 0:
                logger.info(click.style(f"No documents found for dataset: {dataset_id}", fg="green"))
            else:
                logger.info(click.style(f"Cleaning documents for dataset: {dataset_id}", fg="green"))

                for document in documents:
                    session.delete(document)

                segment_ids = [segment.id for segment in segments]
                for segment in segments:
                    image_upload_file_ids = get_image_upload_file_ids(segment.content)
                    image_files = session.query(UploadFile).where(UploadFile.id.in_(image_upload_file_ids)).all()
                    for image_file in image_files:
                        if image_file is None:
                            continue
                        try:
                            storage.delete(image_file.key)
                        except Exception:
                            logger.exception(
                                "Delete image_files failed when storage deleted, \
                                              image_upload_file_is: %s",
                                image_file.id,
                            )
                    stmt = delete(UploadFile).where(UploadFile.id.in_(image_upload_file_ids))
                    session.execute(stmt)

                segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.id.in_(segment_ids))
                session.execute(segment_delete_stmt)
            # delete segment attachments
            if attachments_with_bindings:
                attachment_ids = [attachment_file.id for _, attachment_file in attachments_with_bindings]
                binding_ids = [binding.id for binding, _ in attachments_with_bindings]
                for binding, attachment_file in attachments_with_bindings:
                    try:
                        storage.delete(attachment_file.key)
                    except Exception:
                        logger.exception(
                            "Delete attachment_file failed when storage deleted, \
                                            attachment_file_id: %s",
                            binding.attachment_id,
                        )
                attachment_file_delete_stmt = delete(UploadFile).where(UploadFile.id.in_(attachment_ids))
                session.execute(attachment_file_delete_stmt)

                binding_delete_stmt = delete(SegmentAttachmentBinding).where(
                    SegmentAttachmentBinding.id.in_(binding_ids)
                )
                session.execute(binding_delete_stmt)

            session.query(DatasetProcessRule).where(DatasetProcessRule.dataset_id == dataset_id).delete()
            session.query(DatasetQuery).where(DatasetQuery.dataset_id == dataset_id).delete()
            session.query(AppDatasetJoin).where(AppDatasetJoin.dataset_id == dataset_id).delete()
            # delete dataset metadata
            session.query(DatasetMetadata).where(DatasetMetadata.dataset_id == dataset_id).delete()
            session.query(DatasetMetadataBinding).where(DatasetMetadataBinding.dataset_id == dataset_id).delete()
            # delete pipeline and workflow
            if pipeline_id:
                session.query(Pipeline).where(Pipeline.id == pipeline_id).delete()
                session.query(Workflow).where(
                    Workflow.tenant_id == tenant_id,
                    Workflow.app_id == pipeline_id,
                    Workflow.type == WorkflowType.RAG_PIPELINE,
                ).delete()
            # delete files
            if documents:
                file_ids = []
                for document in documents:
                    if document.data_source_type == "upload_file":
                        if document.data_source_info:
                            data_source_info = document.data_source_info_dict
                            if data_source_info and "upload_file_id" in data_source_info:
                                file_id = data_source_info["upload_file_id"]
                                file_ids.append(file_id)
                files = session.query(UploadFile).where(UploadFile.id.in_(file_ids)).all()
                for file in files:
                    storage.delete(file.key)

                file_delete_stmt = delete(UploadFile).where(UploadFile.id.in_(file_ids))
                session.execute(file_delete_stmt)

            session.commit()
            end_at = time.perf_counter()
            logger.info(
                click.style(
                    f"Cleaned dataset when dataset deleted: {dataset_id} latency: {end_at - start_at}",
                    fg="green",
                )
            )
        except Exception:
            # Add rollback to prevent dirty session state in case of exceptions
            # This ensures the database session is properly cleaned up
            try:
                session.rollback()
                logger.info(click.style(f"Rolled back database session for dataset: {dataset_id}", fg="yellow"))
            except Exception:
                logger.exception("Failed to rollback database session")

            logger.exception("Cleaned dataset when dataset deleted failed")
        finally:
            # Explicitly close the session for test expectations and safety
            try:
                session.close()
            except Exception:
                logger.exception("Failed to close database session")

```

### api/tasks/mail_human_input_delivery_task.py
```py
import json
import logging
import time
from dataclasses import dataclass
from typing import Any

import click
from celery import shared_task
from graphon.runtime import GraphRuntimeState, VariablePool
from sqlalchemy import select
from sqlalchemy.orm import Session, sessionmaker

from configs import dify_config
from core.app.layers.pause_state_persist_layer import WorkflowResumptionContext
from core.workflow.human_input_compat import EmailDeliveryConfig, EmailDeliveryMethod
from extensions.ext_database import db
from extensions.ext_mail import mail
from models.human_input import (
    DeliveryMethodType,
    HumanInputDelivery,
    HumanInputForm,
    HumanInputFormRecipient,
    RecipientType,
)
from repositories.factory import DifyAPIRepositoryFactory
from services.feature_service import FeatureService

logger = logging.getLogger(__name__)


@dataclass(frozen=True)
class _EmailRecipient:
    email: str
    token: str


@dataclass(frozen=True)
class _EmailDeliveryJob:
    form_id: str
    subject: str
    body: str
    form_content: str
    recipients: list[_EmailRecipient]


def _build_form_link(token: str) -> str:
    base_url = dify_config.APP_WEB_URL
    return f"{base_url.rstrip('/')}/form/{token}"


def _parse_recipient_payload(payload: str) -> tuple[str | None, RecipientType | None]:
    try:
        payload_dict: dict[str, Any] = json.loads(payload)
    except Exception:
        logger.exception("Failed to parse recipient payload")
        return None, None

    return payload_dict.get("email"), payload_dict.get("TYPE")


def _load_email_jobs(session: Session, form: HumanInputForm) -> list[_EmailDeliveryJob]:
    deliveries = session.scalars(
        select(HumanInputDelivery).where(
            HumanInputDelivery.form_id == form.id,
            HumanInputDelivery.delivery_method_type == DeliveryMethodType.EMAIL,
        )
    ).all()
    jobs: list[_EmailDeliveryJob] = []
    for delivery in deliveries:
        delivery_config = EmailDeliveryMethod.model_validate_json(delivery.channel_payload)

        recipients = session.scalars(
            select(HumanInputFormRecipient).where(HumanInputFormRecipient.delivery_id == delivery.id)
        ).all()

        recipient_entities: list[_EmailRecipient] = []
        for recipient in recipients:
            email, recipient_type = _parse_recipient_payload(recipient.recipient_payload)
            if recipient_type not in {RecipientType.EMAIL_MEMBER, RecipientType.EMAIL_EXTERNAL}:
                continue
            if not email:
                continue
            token = recipient.access_token
            if not token:
                continue
            recipient_entities.append(_EmailRecipient(email=email, token=token))

        if not recipient_entities:
            continue

        jobs.append(
            _EmailDeliveryJob(
                form_id=form.id,
                subject=delivery_config.config.subject,
                body=delivery_config.config.body,
                form_content=form.rendered_content,
                recipients=recipient_entities,
            )
        )
    return jobs


def _render_body(
    body_template: str,
    form_link: str,
    *,
    variable_pool: VariablePool | None,
) -> str:
    body = EmailDeliveryConfig.render_body_template(
        body=body_template,
        url=form_link,
        variable_pool=variable_pool,
    )
    return EmailDeliveryConfig.render_markdown_body(body)


def _load_variable_pool(workflow_run_id: str | None) -> VariablePool | None:
    if not workflow_run_id:
        return None

    session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
    workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_factory)
    pause_entity = workflow_run_repo.get_workflow_pause(workflow_run_id)
    if pause_entity is None:
        logger.info("No pause state found for workflow run %s", workflow_run_id)
        return None

    try:
        resumption_context = WorkflowResumptionContext.loads(pause_entity.get_state().decode())
    except Exception:
        logger.exception("Failed to load resumption context for workflow run %s", workflow_run_id)
        return None

    graph_runtime_state = GraphRuntimeState.from_snapshot(resumption_context.serialized_graph_runtime_state)
    return graph_runtime_state.variable_pool


def _open_session(session_factory: sessionmaker | Session | None):
    if session_factory is None:
        return Session(db.engine)
    if isinstance(session_factory, Session):
        return session_factory
    return session_factory()


@shared_task(queue="mail")
def dispatch_human_input_email_task(form_id: str, node_title: str | None = None, session_factory=None):
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start human input email delivery for form {form_id}", fg="green"))
    start_at = time.perf_counter()

    try:
        with _open_session(session_factory) as session:
            form = session.get(HumanInputForm, form_id)
            if form is None:
                logger.warning("Human input form not found, form_id=%s", form_id)
                return
            features = FeatureService.get_features(form.tenant_id)
            if not features.human_input_email_delivery_enabled:
                logger.info(
                    "Human input email delivery is not available for tenant=%s, form_id=%s",
                    form.tenant_id,
                    form_id,
                )
                return
            jobs = _load_email_jobs(session, form)

        variable_pool = _load_variable_pool(form.workflow_run_id)

        for job in jobs:
            for recipient in job.recipients:
                form_link = _build_form_link(recipient.token)
                body = _render_body(job.body, form_link, variable_pool=variable_pool)
                subject = EmailDeliveryConfig.sanitize_subject(job.subject)

                mail.send(
                    to=recipient.email,
                    subject=subject,
                    html=body,
                )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Human input email delivery succeeded for form {form_id}: latency: {end_at - start_at}", fg="green"
            )
        )
    except Exception:
        logger.exception("Send human input email failed, form_id=%s", form_id)

```

### api/tasks/human_input_timeout_tasks.py
```py
import logging
from datetime import timedelta

from celery import shared_task
from graphon.enums import WorkflowExecutionStatus
from graphon.nodes.human_input.enums import HumanInputFormKind, HumanInputFormStatus
from sqlalchemy import or_, select
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from core.repositories.human_input_repository import HumanInputFormSubmissionRepository
from extensions.ext_database import db
from extensions.ext_storage import storage
from libs.datetime_utils import ensure_naive_utc, naive_utc_now
from models.human_input import HumanInputForm
from models.workflow import WorkflowPause, WorkflowRun
from services.human_input_service import HumanInputService

logger = logging.getLogger(__name__)


def _is_global_timeout(form_model: HumanInputForm, global_timeout_seconds: int, *, now) -> bool:
    if global_timeout_seconds <= 0:
        return False
    if form_model.workflow_run_id is None:
        return False
    created_at = ensure_naive_utc(form_model.created_at)
    global_deadline = created_at + timedelta(seconds=global_timeout_seconds)
    return global_deadline <= now


def _handle_global_timeout(*, form_id: str, workflow_run_id: str, node_id: str, session_factory: sessionmaker) -> None:
    now = naive_utc_now()
    with session_factory() as session, session.begin():
        workflow_run = session.get(WorkflowRun, workflow_run_id)
        if workflow_run is not None:
            workflow_run.status = WorkflowExecutionStatus.STOPPED
            workflow_run.error = f"Human input global timeout at node {node_id}"
            workflow_run.finished_at = now
            session.add(workflow_run)

        pause_model = session.scalar(select(WorkflowPause).where(WorkflowPause.workflow_run_id == workflow_run_id))
        if pause_model is not None:
            try:
                storage.delete(pause_model.state_object_key)
            except Exception:
                logger.exception(
                    "Failed to delete pause state object for workflow_run_id=%s, pause_id=%s",
                    workflow_run_id,
                    pause_model.id,
                )
            pause_model.resumed_at = now
            session.add(pause_model)


@shared_task(name="human_input_form_timeout.check_and_resume", queue="schedule_executor")
def check_and_handle_human_input_timeouts(limit: int = 100) -> None:
    """Scan for expired human input forms and resume or end workflows."""

    session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
    form_repo = HumanInputFormSubmissionRepository()
    service = HumanInputService(session_factory, form_repository=form_repo)
    now = naive_utc_now()
    global_timeout_seconds = dify_config.HUMAN_INPUT_GLOBAL_TIMEOUT_SECONDS

    with session_factory() as session:
        global_deadline = now - timedelta(seconds=global_timeout_seconds) if global_timeout_seconds > 0 else None
        timeout_filter = HumanInputForm.expiration_time <= now
        if global_deadline is not None:
            timeout_filter = or_(timeout_filter, HumanInputForm.created_at <= global_deadline)
        stmt = (
            select(HumanInputForm)
            .where(
                HumanInputForm.status == HumanInputFormStatus.WAITING,
                timeout_filter,
            )
            .order_by(HumanInputForm.id.asc())
            .limit(limit)
        )
        expired_forms = session.scalars(stmt).all()

    for form_model in expired_forms:
        try:
            if form_model.form_kind == HumanInputFormKind.DELIVERY_TEST:
                form_repo.mark_timeout(
                    form_id=form_model.id,
                    timeout_status=HumanInputFormStatus.TIMEOUT,
                    reason="delivery_test_timeout",
                )
                continue

            is_global = _is_global_timeout(form_model, global_timeout_seconds, now=now)
            record = form_repo.mark_timeout(
                form_id=form_model.id,
                timeout_status=HumanInputFormStatus.EXPIRED if is_global else HumanInputFormStatus.TIMEOUT,
                reason="global_timeout" if is_global else "node_timeout",
            )
            assert record.workflow_run_id is not None, "workflow_run_id should not be None for non-test form"
            if is_global:
                _handle_global_timeout(
                    form_id=record.form_id,
                    workflow_run_id=record.workflow_run_id,
                    node_id=record.node_id,
                    session_factory=session_factory,
                )
            else:
                service.enqueue_resume(record.workflow_run_id)
        except Exception:
            logger.exception(
                "Failed to handle timeout for form_id=%s workflow_run_id=%s",
                form_model.id,
                form_model.workflow_run_id,
            )

```

### api/tasks/trigger_processing_tasks.py
```py
"""
Celery tasks for async trigger processing.

These tasks handle trigger workflow execution asynchronously
to avoid blocking the main request thread.
"""

import json
import logging
from collections.abc import Mapping, Sequence
from datetime import UTC, datetime
from typing import Any

from celery import shared_task
from graphon.enums import WorkflowExecutionStatus
from sqlalchemy import func, select
from sqlalchemy.orm import Session

from core.app.entities.app_invoke_entities import InvokeFrom
from core.db.session_factory import session_factory
from core.plugin.entities.plugin_daemon import CredentialType
from core.plugin.entities.request import TriggerInvokeEventResponse
from core.plugin.impl.exc import PluginInvokeError
from core.trigger.constants import TRIGGER_PLUGIN_NODE_TYPE
from core.trigger.debug.event_bus import TriggerDebugEventBus
from core.trigger.debug.events import PluginTriggerDebugEvent, build_plugin_pool_key
from core.trigger.entities.entities import TriggerProviderEntity
from core.trigger.provider import PluginTriggerProviderController
from core.trigger.trigger_manager import TriggerManager
from core.workflow.nodes.trigger_plugin.entities import TriggerEventNodeData
from enums.quota_type import QuotaType, unlimited
from models.enums import (
    AppTriggerType,
    CreatorUserRole,
    WorkflowRunTriggeredFrom,
    WorkflowTriggerStatus,
)
from models.model import EndUser
from models.provider_ids import TriggerProviderID
from models.trigger import TriggerSubscription, WorkflowPluginTrigger, WorkflowTriggerLog
from models.workflow import Workflow, WorkflowAppLog, WorkflowAppLogCreatedFrom, WorkflowRun
from services.async_workflow_service import AsyncWorkflowService
from services.end_user_service import EndUserService
from services.errors.app import QuotaExceededError
from services.trigger.app_trigger_service import AppTriggerService
from services.trigger.trigger_provider_service import TriggerProviderService
from services.trigger.trigger_request_service import TriggerHttpRequestCachingService
from services.trigger.trigger_subscription_operator_service import TriggerSubscriptionOperatorService
from services.workflow.entities import PluginTriggerData, PluginTriggerDispatchData, PluginTriggerMetadata
from services.workflow.queue_dispatcher import QueueDispatcherManager

logger = logging.getLogger(__name__)

# Use workflow queue for trigger processing
TRIGGER_QUEUE = "triggered_workflow_dispatcher"


def dispatch_trigger_debug_event(
    events: list[str],
    user_id: str,
    timestamp: int,
    request_id: str,
    subscription: TriggerSubscription,
) -> int:
    debug_dispatched = 0
    try:
        for event_name in events:
            pool_key: str = build_plugin_pool_key(
                name=event_name,
                tenant_id=subscription.tenant_id,
                subscription_id=subscription.id,
                provider_id=subscription.provider_id,
            )
            trigger_debug_event: PluginTriggerDebugEvent = PluginTriggerDebugEvent(
                timestamp=timestamp,
                user_id=user_id,
                name=event_name,
                request_id=request_id,
                subscription_id=subscription.id,
                provider_id=subscription.provider_id,
            )
            debug_dispatched += TriggerDebugEventBus.dispatch(
                tenant_id=subscription.tenant_id,
                event=trigger_debug_event,
                pool_key=pool_key,
            )
            logger.debug(
                "Trigger debug dispatched %d sessions to pool %s for event %s for subscription %s provider %s",
                debug_dispatched,
                pool_key,
                event_name,
                subscription.id,
                subscription.provider_id,
            )
        return debug_dispatched
    except Exception:
        logger.exception("Failed to dispatch to debug sessions")
        return 0


def _get_latest_workflows_by_app_ids(
    session: Session, subscribers: Sequence[WorkflowPluginTrigger]
) -> Mapping[str, Workflow]:
    """Get the latest workflows by app_ids"""
    workflow_query = (
        select(Workflow.app_id, func.max(Workflow.created_at).label("max_created_at"))
        .where(
            Workflow.app_id.in_({t.app_id for t in subscribers}),
            Workflow.version != Workflow.VERSION_DRAFT,
        )
        .group_by(Workflow.app_id)
        .subquery()
    )
    workflows = session.scalars(
        select(Workflow).join(
            workflow_query,
            (Workflow.app_id == workflow_query.c.app_id) & (Workflow.created_at == workflow_query.c.max_created_at),
        )
    ).all()
    return {w.app_id: w for w in workflows}


def _record_trigger_failure_log(
    *,
    session: Session,
    workflow: Workflow,
    plugin_trigger: WorkflowPluginTrigger,
    subscription: TriggerSubscription,
    trigger_metadata: PluginTriggerMetadata,
    end_user: EndUser | None,
    error_message: str,
    event_name: str,
    request_id: str,
) -> None:
    """
    Persist a workflow run, workflow app log, and trigger log entry for failed trigger invocations.
    """
    now = datetime.now(UTC)
    if end_user:
        created_by_role = CreatorUserRole.END_USER
        created_by = end_user.id
    else:
        created_by_role = CreatorUserRole.ACCOUNT
        created_by = subscription.user_id

    failure_inputs = {
        "event_name": event_name,
        "subscription_id": subscription.id,
        "request_id": request_id,
        "plugin_trigger_id": plugin_trigger.id,
    }

    workflow_run = WorkflowRun(
        tenant_id=workflow.tenant_id,
        app_id=workflow.app_id,
        workflow_id=workflow.id,
        type=workflow.type,
        triggered_from=WorkflowRunTriggeredFrom.PLUGIN.value,
        version=workflow.version,
        graph=workflow.graph,
        inputs=json.dumps(failure_inputs),
        status=WorkflowExecutionStatus.FAILED.value,
        outputs="{}",
        error=error_message,
        elapsed_time=0.0,
        total_tokens=0,
        total_steps=0,
        created_by_role=created_by_role,
        created_by=created_by,
        created_at=now,
        finished_at=now,
        exceptions_count=0,
    )
    session.add(workflow_run)
    session.flush()

    workflow_app_log = WorkflowAppLog(
        tenant_id=workflow.tenant_id,
        app_id=workflow.app_id,
        workflow_id=workflow.id,
        workflow_run_id=workflow_run.id,
        created_from=WorkflowAppLogCreatedFrom.SERVICE_API,
        created_by_role=created_by_role,
        created_by=created_by,
    )
    session.add(workflow_app_log)

    dispatcher = QueueDispatcherManager.get_dispatcher(subscription.tenant_id)
    queue_name = dispatcher.get_queue_name()

    trigger_data = PluginTriggerData(
        app_id=plugin_trigger.app_id,
        tenant_id=subscription.tenant_id,
        workflow_id=workflow.id,
        root_node_id=plugin_trigger.node_id,
        inputs={},
        trigger_metadata=trigger_metadata,
        plugin_id=subscription.provider_id,
        endpoint_id=subscription.endpoint_id,
    )

    trigger_log = WorkflowTriggerLog(
        tenant_id=workflow.tenant_id,
        app_id=workflow.app_id,
        workflow_id=workflow.id,
        workflow_run_id=workflow_run.id,
        root_node_id=plugin_trigger.node_id,
        trigger_metadata=trigger_metadata.model_dump_json(),
        trigger_type=AppTriggerType.TRIGGER_PLUGIN,
        trigger_data=trigger_data.model_dump_json(),
        inputs=json.dumps({}),
        status=WorkflowTriggerStatus.FAILED,
        error=error_message,
        queue_name=queue_name,
        retry_count=0,
        created_by_role=created_by_role,
        created_by=created_by,
        triggered_at=now,
        finished_at=now,
        elapsed_time=0.0,
        total_tokens=0,
        outputs=None,
        celery_task_id=None,
    )
    session.add(trigger_log)
    session.commit()


def dispatch_triggered_workflow(
    user_id: str,
    subscription: TriggerSubscription,
    event_name: str,
    request_id: str,
) -> int:
    """Process triggered workflows.

    Args:
        subscription: The trigger subscription
        event: The trigger entity that was activated
        request_id: The ID of the stored request in storage system
    """
    request = TriggerHttpRequestCachingService.get_request(request_id)
    payload = TriggerHttpRequestCachingService.get_payload(request_id)

    subscribers: list[WorkflowPluginTrigger] = TriggerSubscriptionOperatorService.get_subscriber_triggers(
        tenant_id=subscription.tenant_id, subscription_id=subscription.id, event_name=event_name
    )
    if not subscribers:
        logger.warning(
            "No workflows found for trigger event '%s' in subscription '%s'",
            event_name,
            subscription.id,
        )
        return 0

    dispatched_count = 0
    provider_controller: PluginTriggerProviderController = TriggerManager.get_trigger_provider(
        tenant_id=subscription.tenant_id, provider_id=TriggerProviderID(subscription.provider_id)
    )
    trigger_entity: TriggerProviderEntity = provider_controller.entity
    with session_factory.create_session() as session:
        workflows: Mapping[str, Workflow] = _get_latest_workflows_by_app_ids(session, subscribers)

        end_users: Mapping[str, EndUser] = EndUserService.create_end_user_batch(
            type=InvokeFrom.TRIGGER,
            tenant_id=subscription.tenant_id,
            app_ids=[plugin_trigger.app_id for plugin_trigger in subscribers],
            user_id=user_id,
        )
        for plugin_trigger in subscribers:
            # Get workflow from mapping
            workflow: Workflow | None = workflows.get(plugin_trigger.app_id)
            if not workflow:
                logger.error(
                    "Workflow not found for app %s",
                    plugin_trigger.app_id,
                )
                continue

            # Find the trigger node in the workflow
            event_node = None
            for node_id, node_config in workflow.walk_nodes(TRIGGER_PLUGIN_NODE_TYPE):
                if node_id == plugin_trigger.node_id:
                    event_node = node_config
                    break

            if not event_node:
                logger.error("Trigger event node not found for app %s", plugin_trigger.app_id)
                continue

            # invoke trigger
            trigger_metadata = PluginTriggerMetadata(
                plugin_unique_identifier=provider_controller.plugin_unique_identifier or "",
                endpoint_id=subscription.endpoint_id,
                provider_id=subscription.provider_id,
                event_name=event_name,
                icon_filename=trigger_entity.identity.icon or "",
                icon_dark_filename=trigger_entity.identity.icon_dark or "",
            )

            # consume quota before invoking trigger
            quota_charge = unlimited()
            try:
                quota_charge = QuotaType.TRIGGER.consume(subscription.tenant_id)
            except QuotaExceededError:
                AppTriggerService.mark_tenant_triggers_rate_limited(subscription.tenant_id)
                logger.info(
                    "Tenant %s rate limited, skipping plugin trigger %s", subscription.tenant_id, plugin_trigger.id
                )
                return 0

            node_data: TriggerEventNodeData = TriggerEventNodeData.model_validate(event_node)
            invoke_response: TriggerInvokeEventResponse | None = None
            try:
                invoke_response = TriggerManager.invoke_trigger_event(
                    tenant_id=subscription.tenant_id,
                    user_id=user_id,
                    provider_id=TriggerProviderID(subscription.provider_id),
                    event_name=event_name,
                    parameters=node_data.resolve_parameters(
                        parameter_schemas=provider_controller.get_event_parameters(event_name=event_name)
                    ),
                    credentials=subscription.credentials,
                    credential_type=CredentialType.of(subscription.credential_type),
                    subscription=subscription.to_entity(),
                    request=request,
                    payload=payload,
                )
            except PluginInvokeError as e:
                quota_charge.refund()

                error_message = e.to_user_friendly_error(plugin_name=trigger_entity.identity.name)
                try:
                    end_user = end_users.get(plugin_trigger.app_id)
                    _record_trigger_failure_log(
                        session=session,
                        workflow=workflow,
                        plugin_trigger=plugin_trigger,
                        subscription=subscription,
                        trigger_metadata=trigger_metadata,
                        end_user=end_user,
                        error_message=error_message,
                        event_name=event_name,
                        request_id=request_id,
                    )
                except Exception:
                    logger.exception(
                        "Failed to record trigger failure log for app %s",
                        plugin_trigger.app_id,
                    )
                continue
            except Exception:
                quota_charge.refund()

                logger.exception(
                    "Failed to invoke trigger event for app %s",
                    plugin_trigger.app_id,
                )
                continue

            if invoke_response is not None and invoke_response.cancelled:
                quota_charge.refund()

                logger.info(
                    "Trigger ignored for app %s with trigger event %s",
                    plugin_trigger.app_id,
                    event_name,
                )
                continue

            # Create trigger data for async execution
            trigger_data = PluginTriggerData(
                app_id=plugin_trigger.app_id,
                tenant_id=subscription.tenant_id,
                workflow_id=workflow.id,
                root_node_id=plugin_trigger.node_id,
                plugin_id=subscription.provider_id,
                endpoint_id=subscription.endpoint_id,
                inputs=invoke_response.variables,
                trigger_metadata=trigger_metadata,
            )

            # Trigger async workflow
            try:
                end_user = end_users.get(plugin_trigger.app_id)
                if not end_user:
                    raise ValueError(f"End user not found for app {plugin_trigger.app_id}")

                AsyncWorkflowService.trigger_workflow_async(session=session, user=end_user, trigger_data=trigger_data)
                dispatched_count += 1
                logger.info(
                    "Triggered workflow for app %s with trigger event %s",
                    plugin_trigger.app_id,
                    event_name,
                )
            except Exception:
                quota_charge.refund()

                logger.exception(
                    "Failed to trigger workflow for app %s",
                    plugin_trigger.app_id,
                )

        return dispatched_count


def dispatch_triggered_workflows(
    user_id: str,
    events: list[str],
    subscription: TriggerSubscription,
    request_id: str,
) -> int:
    dispatched_count = 0
    for event_name in events:
        try:
            dispatched_count += dispatch_triggered_workflow(
                user_id=user_id,
                subscription=subscription,
                event_name=event_name,
                request_id=request_id,
            )
        except Exception:
            logger.exception(
                "Failed to dispatch trigger '%s' for subscription %s and provider %s. Continuing...",
                event_name,
                subscription.id,
                subscription.provider_id,
            )
            # Continue processing other triggers even if one fails
            continue

    logger.info(
        "Completed async trigger dispatching: processed %d/%d triggers for subscription %s and provider %s",
        dispatched_count,
        len(events),
        subscription.id,
        subscription.provider_id,
    )
    return dispatched_count


@shared_task(queue=TRIGGER_QUEUE)
def dispatch_triggered_workflows_async(
    dispatch_data: Mapping[str, Any],
) -> Mapping[str, Any]:
    """
    Dispatch triggers asynchronously.

    Args:
        endpoint_id: Endpoint ID
        provider_id: Provider ID
        subscription_id: Subscription ID
        timestamp: Timestamp of the event
        triggers: List of triggers to dispatch
        request_id: Unique ID of the stored request

    Returns:
        dict: Execution result with status and dispatched trigger count
    """
    dispatch_params: PluginTriggerDispatchData = PluginTriggerDispatchData.model_validate(dispatch_data)
    user_id = dispatch_params.user_id
    tenant_id = dispatch_params.tenant_id
    endpoint_id = dispatch_params.endpoint_id
    provider_id = dispatch_params.provider_id
    subscription_id = dispatch_params.subscription_id
    timestamp = dispatch_params.timestamp
    events = dispatch_params.events
    request_id = dispatch_params.request_id

    try:
        logger.info(
            "Starting trigger dispatching uid=%s, endpoint=%s, events=%s, req_id=%s, sub_id=%s, provider_id=%s",
            user_id,
            endpoint_id,
            events,
            request_id,
            subscription_id,
            provider_id,
        )

        subscription: TriggerSubscription | None = TriggerProviderService.get_subscription_by_id(
            tenant_id=tenant_id,
            subscription_id=subscription_id,
        )
        if not subscription:
            logger.error("Subscription not found: %s", subscription_id)
            return {"status": "failed", "error": "Subscription not found"}

        workflow_dispatched = dispatch_triggered_workflows(
            user_id=user_id,
            events=events,
            subscription=subscription,
            request_id=request_id,
        )

        debug_dispatched = dispatch_trigger_debug_event(
            events=events,
            user_id=user_id,
            timestamp=timestamp,
            request_id=request_id,
            subscription=subscription,
        )

        return {
            "status": "completed",
            "total_count": len(events),
            "workflows": workflow_dispatched,
            "debug_events": debug_dispatched,
        }

    except Exception as e:
        logger.exception(
            "Error in async trigger dispatching for endpoint %s data %s for subscription %s and provider %s",
            endpoint_id,
            dispatch_data,
            subscription_id,
            provider_id,
        )
        return {
            "status": "failed",
            "error": str(e),
        }

```

### api/tasks/delete_segment_from_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from models.dataset import Dataset, Document, SegmentAttachmentBinding
from models.model import UploadFile

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def delete_segment_from_index_task(
    index_node_ids: list, dataset_id: str, document_id: str, segment_ids: list, child_node_ids: list | None = None
):
    """
    Async Remove segment from index
    :param index_node_ids:
    :param dataset_id:
    :param document_id:

    Usage: delete_segment_from_index_task.delay(index_node_ids, dataset_id, document_id)
    """
    logger.info(click.style("Start delete segment from index", fg="green"))
    start_at = time.perf_counter()
    with session_factory.create_session() as session:
        try:
            dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
            if not dataset:
                logging.warning("Dataset %s not found, skipping index cleanup", dataset_id)
                return

            dataset_document = session.query(Document).where(Document.id == document_id).first()
            if not dataset_document:
                return

            if (
                not dataset_document.enabled
                or dataset_document.archived
                or dataset_document.indexing_status != "completed"
            ):
                logging.info("Document not in valid state for index operations, skipping")
                return
            doc_form = dataset_document.doc_form

            # Proceed with index cleanup using the index_node_ids directly
            # For actual deletion, we should delete summaries (not just disable them)
            index_processor = IndexProcessorFactory(doc_form).init_index_processor()
            index_processor.clean(
                dataset,
                index_node_ids,
                with_keywords=True,
                delete_child_chunks=True,
                precomputed_child_node_ids=child_node_ids,
                delete_summaries=True,  # Actually delete summaries when segment is deleted
            )
            if dataset.is_multimodal:
                # delete segment attachment binding
                segment_attachment_bindings = (
                    session.query(SegmentAttachmentBinding)
                    .where(SegmentAttachmentBinding.segment_id.in_(segment_ids))
                    .all()
                )
                if segment_attachment_bindings:
                    attachment_ids = [binding.attachment_id for binding in segment_attachment_bindings]
                    index_processor.clean(dataset=dataset, node_ids=attachment_ids, with_keywords=False)
                    segment_attachment_bind_ids = [i.id for i in segment_attachment_bindings]

                    for i in range(0, len(segment_attachment_bind_ids), 1000):
                        segment_attachment_bind_delete_stmt = delete(SegmentAttachmentBinding).where(
                            SegmentAttachmentBinding.id.in_(segment_attachment_bind_ids[i : i + 1000])
                        )
                        session.execute(segment_attachment_bind_delete_stmt)

                    # delete upload file
                    session.query(UploadFile).where(UploadFile.id.in_(attachment_ids)).delete(synchronize_session=False)
                    session.commit()

            end_at = time.perf_counter()
            logger.info(click.style(f"Segment deleted from index latency: {end_at - start_at}", fg="green"))
        except Exception:
            logger.exception("delete segment from index failed")

```

### api/tasks/mail_reset_password_task.py
```py
import logging
import time

import click
from celery import shared_task

from configs import dify_config
from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_reset_password_mail_task(language: str, to: str, code: str):
    """
    Send reset password email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        code: Reset password code
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start password reset mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.RESET_PASSWORD,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "code": code,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send password reset mail to {to} succeeded: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send password reset mail to %s failed", to)


@shared_task(queue="mail")
def send_reset_password_mail_task_when_account_not_exist(language: str, to: str, is_allow_register: bool) -> None:
    """
    Send reset password email with internationalization support when account not exist.

    Args:
        language: Language code for email localization
        to: Recipient email address
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start password reset mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        if is_allow_register:
            sign_up_url = f"{dify_config.CONSOLE_WEB_URL}/signup"
            email_service = get_email_i18n_service()
            email_service.send_email(
                email_type=EmailType.RESET_PASSWORD_WHEN_ACCOUNT_NOT_EXIST,
                language_code=language,
                to=to,
                template_context={
                    "to": to,
                    "sign_up_url": sign_up_url,
                },
            )
        else:
            email_service = get_email_i18n_service()
            email_service.send_email(
                email_type=EmailType.RESET_PASSWORD_WHEN_ACCOUNT_NOT_EXIST_NO_REGISTER,
                language_code=language,
                to=to,
            )

        end_at = time.perf_counter()
        logger.info(
            click.style(f"Send password reset mail to {to} succeeded: latency: {end_at - start_at}", fg="green")
        )
    except Exception:
        logger.exception("Send password reset mail to %s failed", to)

```

### api/tasks/async_workflow_tasks.py
```py
"""
Celery tasks for async workflow execution.

These tasks handle workflow execution for different subscription tiers
with appropriate retry policies and error handling.
"""

import logging
from datetime import UTC, datetime
from typing import Any

from celery import shared_task
from graphon.runtime import GraphRuntimeState
from sqlalchemy import select
from sqlalchemy.orm import Session, sessionmaker

from configs import dify_config
from core.app.apps.workflow.app_generator import SKIP_PREPARE_USER_INPUTS_KEY, WorkflowAppGenerator
from core.app.entities.app_invoke_entities import InvokeFrom, WorkflowAppGenerateEntity
from core.app.layers.pause_state_persist_layer import PauseStateLayerConfig, WorkflowResumptionContext
from core.app.layers.timeslice_layer import TimeSliceLayer
from core.app.layers.trigger_post_layer import TriggerPostLayer
from core.db.session_factory import session_factory
from core.repositories import DifyCoreRepositoryFactory
from extensions.ext_database import db
from models.account import Account
from models.enums import CreatorUserRole, WorkflowRunTriggeredFrom, WorkflowTriggerStatus
from models.model import App, EndUser, Tenant
from models.trigger import WorkflowTriggerLog
from models.workflow import Workflow, WorkflowNodeExecutionTriggeredFrom, WorkflowRun
from repositories.factory import DifyAPIRepositoryFactory
from repositories.sqlalchemy_workflow_trigger_log_repository import SQLAlchemyWorkflowTriggerLogRepository
from services.errors.app import WorkflowNotFoundError
from services.workflow.entities import (
    TriggerData,
    WorkflowResumeTaskData,
    WorkflowTaskData,
)
from tasks.workflow_cfs_scheduler.cfs_scheduler import AsyncWorkflowCFSPlanEntity, AsyncWorkflowCFSPlanScheduler
from tasks.workflow_cfs_scheduler.entities import AsyncWorkflowQueue, AsyncWorkflowSystemStrategy

logger = logging.getLogger(__name__)


@shared_task(queue=AsyncWorkflowQueue.PROFESSIONAL_QUEUE)
def execute_workflow_professional(task_data_dict: dict[str, Any]):
    """Execute workflow for professional tier with highest priority"""
    task_data = WorkflowTaskData.model_validate(task_data_dict)
    cfs_plan_scheduler_entity = AsyncWorkflowCFSPlanEntity(
        queue=AsyncWorkflowQueue.PROFESSIONAL_QUEUE,
        schedule_strategy=AsyncWorkflowSystemStrategy,
        granularity=dify_config.ASYNC_WORKFLOW_SCHEDULER_GRANULARITY,
    )
    _execute_workflow_common(
        task_data,
        AsyncWorkflowCFSPlanScheduler(plan=cfs_plan_scheduler_entity),
        cfs_plan_scheduler_entity,
    )


@shared_task(queue=AsyncWorkflowQueue.TEAM_QUEUE)
def execute_workflow_team(task_data_dict: dict[str, Any]):
    """Execute workflow for team tier"""
    task_data = WorkflowTaskData.model_validate(task_data_dict)
    cfs_plan_scheduler_entity = AsyncWorkflowCFSPlanEntity(
        queue=AsyncWorkflowQueue.TEAM_QUEUE,
        schedule_strategy=AsyncWorkflowSystemStrategy,
        granularity=dify_config.ASYNC_WORKFLOW_SCHEDULER_GRANULARITY,
    )
    _execute_workflow_common(
        task_data,
        AsyncWorkflowCFSPlanScheduler(plan=cfs_plan_scheduler_entity),
        cfs_plan_scheduler_entity,
    )


@shared_task(queue=AsyncWorkflowQueue.SANDBOX_QUEUE)
def execute_workflow_sandbox(task_data_dict: dict[str, Any]):
    """Execute workflow for free tier with lower retry limit"""
    task_data = WorkflowTaskData.model_validate(task_data_dict)
    cfs_plan_scheduler_entity = AsyncWorkflowCFSPlanEntity(
        queue=AsyncWorkflowQueue.SANDBOX_QUEUE,
        schedule_strategy=AsyncWorkflowSystemStrategy,
        granularity=dify_config.ASYNC_WORKFLOW_SCHEDULER_GRANULARITY,
    )
    _execute_workflow_common(
        task_data,
        AsyncWorkflowCFSPlanScheduler(plan=cfs_plan_scheduler_entity),
        cfs_plan_scheduler_entity,
    )


def _build_generator_args(trigger_data: TriggerData) -> dict[str, Any]:
    """Build args passed into WorkflowAppGenerator.generate for Celery executions."""

    args: dict[str, Any] = {
        "inputs": dict(trigger_data.inputs),
        "files": list(trigger_data.files),
        SKIP_PREPARE_USER_INPUTS_KEY: True,
    }
    return args


def _execute_workflow_common(
    task_data: WorkflowTaskData,
    cfs_plan_scheduler: AsyncWorkflowCFSPlanScheduler,
    cfs_plan_scheduler_entity: AsyncWorkflowCFSPlanEntity,
):
    """Execute workflow with common logic and trigger log updates."""

    with session_factory.create_session() as session:
        trigger_log_repo = SQLAlchemyWorkflowTriggerLogRepository(session)

        # Get trigger log
        trigger_log = trigger_log_repo.get_by_id(task_data.workflow_trigger_log_id)

        if not trigger_log:
            # This should not happen, but handle gracefully
            return

        # Reconstruct execution data from trigger log
        trigger_data = TriggerData.model_validate_json(trigger_log.trigger_data)

        # Update status to running
        trigger_log.status = WorkflowTriggerStatus.RUNNING
        trigger_log_repo.update(trigger_log)
        session.commit()

        start_time = datetime.now(UTC)

        try:
            # Get app and workflow models
            app_model = session.scalar(select(App).where(App.id == trigger_log.app_id))

            if not app_model:
                raise WorkflowNotFoundError(f"App not found: {trigger_log.app_id}")

            workflow = session.scalar(select(Workflow).where(Workflow.id == trigger_log.workflow_id))
            if not workflow:
                raise WorkflowNotFoundError(f"Workflow not found: {trigger_log.workflow_id}")

            user = _get_user(session, trigger_log)

            # Execute workflow using WorkflowAppGenerator
            generator = WorkflowAppGenerator()

            # Prepare args matching AppGenerateService.generate format
            args = _build_generator_args(trigger_data)

            # If workflow_id was specified, add it to args
            if trigger_data.workflow_id:
                args["workflow_id"] = str(trigger_data.workflow_id)

            pause_config = PauseStateLayerConfig(
                session_factory=session_factory.get_session_maker(),
                state_owner_user_id=workflow.created_by,
            )

            # Execute the workflow with the trigger type
            generator.generate(
                app_model=app_model,
                workflow=workflow,
                user=user,
                args=args,
                invoke_from=InvokeFrom.SERVICE_API,
                streaming=False,
                call_depth=0,
                triggered_from=trigger_data.trigger_from,
                root_node_id=trigger_data.root_node_id,
                graph_engine_layers=[
                    # TODO: Re-enable TimeSliceLayer after the HITL release.
                    TriggerPostLayer(cfs_plan_scheduler_entity, start_time, trigger_log.id),
                ],
                pause_state_config=pause_config,
            )

        except Exception as e:
            # Calculate elapsed time for failed execution
            elapsed_time = (datetime.now(UTC) - start_time).total_seconds()

            # Update trigger log with failure
            trigger_log.status = WorkflowTriggerStatus.FAILED
            trigger_log.error = str(e)
            trigger_log.finished_at = datetime.now(UTC)
            trigger_log.elapsed_time = elapsed_time
            trigger_log_repo.update(trigger_log)

            # Final failure - no retry logic (simplified like RAG tasks)
            session.commit()


@shared_task(name="resume_workflow_execution")
def resume_workflow_execution(task_data_dict: dict[str, Any]) -> None:
    """Resume a paused workflow run via Celery."""
    task_data = WorkflowResumeTaskData.model_validate(task_data_dict)
    session_factory = sessionmaker(bind=db.engine, expire_on_commit=False)
    workflow_run_repo = DifyAPIRepositoryFactory.create_api_workflow_run_repository(session_factory)

    pause_entity = workflow_run_repo.get_workflow_pause(task_data.workflow_run_id)
    if pause_entity is None:
        logger.warning("No pause state for workflow run %s", task_data.workflow_run_id)
        return
    workflow_run = workflow_run_repo.get_workflow_run_by_id_without_tenant(pause_entity.workflow_execution_id)
    if workflow_run is None:
        logger.warning("Workflow run not found for pause entity: pause_entity_id=%s", pause_entity.id)
        return

    try:
        resumption_context = WorkflowResumptionContext.loads(pause_entity.get_state().decode())
    except Exception as exc:
        logger.exception("Failed to load resumption context for workflow run %s", task_data.workflow_run_id)
        raise exc

    generate_entity = resumption_context.get_generate_entity()
    if not isinstance(generate_entity, WorkflowAppGenerateEntity):
        logger.error(
            "Unsupported resumption entity for workflow run %s: %s",
            task_data.workflow_run_id,
            type(generate_entity),
        )
        return

    graph_runtime_state = GraphRuntimeState.from_snapshot(resumption_context.serialized_graph_runtime_state)

    with session_factory() as session:
        workflow = session.scalar(select(Workflow).where(Workflow.id == workflow_run.workflow_id))
        if workflow is None:
            raise WorkflowNotFoundError(
                "Workflow not found: workflow_run_id=%s, workflow_id=%s", workflow_run.id, workflow_run.workflow_id
            )
        user = _get_user(session, workflow_run)
        app_model = session.scalar(select(App).where(App.id == workflow_run.app_id))
        if app_model is None:
            raise _AppNotFoundError(
                "App not found: app_id=%s, workflow_run_id=%s", workflow_run.app_id, workflow_run.id
            )

    workflow_execution_repository = DifyCoreRepositoryFactory.create_workflow_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=generate_entity.app_config.app_id,
        triggered_from=WorkflowRunTriggeredFrom(workflow_run.triggered_from),
    )
    workflow_node_execution_repository = DifyCoreRepositoryFactory.create_workflow_node_execution_repository(
        session_factory=session_factory,
        user=user,
        app_id=generate_entity.app_config.app_id,
        triggered_from=WorkflowNodeExecutionTriggeredFrom.WORKFLOW_RUN,
    )

    pause_config = PauseStateLayerConfig(
        session_factory=session_factory,
        state_owner_user_id=workflow.created_by,
    )

    generator = WorkflowAppGenerator()
    start_time = datetime.now(UTC)
    graph_engine_layers = []
    trigger_log = _query_trigger_log_info(session_factory, task_data.workflow_run_id)

    if trigger_log:
        cfs_plan_scheduler_entity = AsyncWorkflowCFSPlanEntity(
            queue=AsyncWorkflowQueue(trigger_log.queue_name),
            schedule_strategy=AsyncWorkflowSystemStrategy,
            granularity=dify_config.ASYNC_WORKFLOW_SCHEDULER_GRANULARITY,
        )
        cfs_plan_scheduler = AsyncWorkflowCFSPlanScheduler(plan=cfs_plan_scheduler_entity)

        graph_engine_layers.extend(
            [
                TimeSliceLayer(cfs_plan_scheduler),
                TriggerPostLayer(cfs_plan_scheduler_entity, start_time, trigger_log.id),
            ]
        )

    workflow_run_repo.resume_workflow_pause(task_data.workflow_run_id, pause_entity)

    generator.resume(
        app_model=app_model,
        workflow=workflow,
        user=user,
        application_generate_entity=generate_entity,
        graph_runtime_state=graph_runtime_state,
        workflow_execution_repository=workflow_execution_repository,
        workflow_node_execution_repository=workflow_node_execution_repository,
        graph_engine_layers=graph_engine_layers,
        pause_state_config=pause_config,
    )
    workflow_run_repo.delete_workflow_pause(pause_entity)


def _get_user(session: Session, workflow_run: WorkflowRun | WorkflowTriggerLog) -> Account | EndUser:
    """Compose user from trigger log"""
    tenant = session.scalar(select(Tenant).where(Tenant.id == workflow_run.tenant_id))
    if not tenant:
        raise _TenantNotFoundError(
            "Tenant not found for WorkflowRun: tenant_id=%s, workflow_run_id=%s",
            workflow_run.tenant_id,
            workflow_run.id,
        )

    # Get user from trigger log
    if workflow_run.created_by_role == CreatorUserRole.ACCOUNT:
        user = session.scalar(select(Account).where(Account.id == workflow_run.created_by))
        if user:
            user.current_tenant = tenant
    else:  # CreatorUserRole.END_USER
        user = session.scalar(select(EndUser).where(EndUser.id == workflow_run.created_by))

    if not user:
        raise _UserNotFoundError(
            "User not found: user_id=%s, created_by_role=%s, workflow_run_id=%s",
            workflow_run.created_by,
            workflow_run.created_by_role,
            workflow_run.id,
        )

    return user


def _query_trigger_log_info(session_factory: sessionmaker[Session], workflow_run_id) -> WorkflowTriggerLog | None:
    with session_factory() as session, session.begin():
        trigger_log_repo = SQLAlchemyWorkflowTriggerLogRepository(session)
        trigger_log = trigger_log_repo.get_by_workflow_run_id(workflow_run_id)
        if not trigger_log:
            logger.debug("Trigger log not found for workflow_run: workflow_run_id=%s", workflow_run_id)
            return None

        return trigger_log


class _TenantNotFoundError(Exception):
    pass


class _UserNotFoundError(Exception):
    pass


class _AppNotFoundError(Exception):
    pass

```

### api/tasks/delete_conversation_task.py
```py
import logging
import time

import click
from celery import shared_task

from core.db.session_factory import session_factory
from models import ConversationVariable
from models.model import Message, MessageAnnotation, MessageFeedback
from models.tools import ToolConversationVariables, ToolFile
from models.web import PinnedConversation

logger = logging.getLogger(__name__)


@shared_task(queue="conversation")
def delete_conversation_related_data(conversation_id: str):
    """
    Delete related data conversation in correct order from datatbase to respect foreign key constraints

    Args:
        conversation_id: conversation Id
    """

    logger.info(
        click.style(f"Starting to delete conversation data from db for conversation_id {conversation_id}", fg="green")
    )
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        try:
            session.query(MessageAnnotation).where(MessageAnnotation.conversation_id == conversation_id).delete(
                synchronize_session=False
            )

            session.query(MessageFeedback).where(MessageFeedback.conversation_id == conversation_id).delete(
                synchronize_session=False
            )

            session.query(ToolConversationVariables).where(
                ToolConversationVariables.conversation_id == conversation_id
            ).delete(synchronize_session=False)

            session.query(ToolFile).where(ToolFile.conversation_id == conversation_id).delete(synchronize_session=False)

            session.query(ConversationVariable).where(ConversationVariable.conversation_id == conversation_id).delete(
                synchronize_session=False
            )

            session.query(Message).where(Message.conversation_id == conversation_id).delete(synchronize_session=False)

            session.query(PinnedConversation).where(PinnedConversation.conversation_id == conversation_id).delete(
                synchronize_session=False
            )

            session.commit()

            end_at = time.perf_counter()
            logger.info(
                click.style(
                    (
                        f"Succeeded cleaning data from db for conversation_id {conversation_id} "
                        f"latency: {end_at - start_at}"
                    ),
                    fg="green",
                )
            )

        except Exception:
            logger.exception("Failed to delete data from db for conversation_id: %s failed", conversation_id)
            session.rollback()
            raise

```

### api/tasks/mail_change_mail_task.py
```py
import logging
import time

import click
from celery import shared_task

from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_change_mail_task(language: str, to: str, code: str, phase: str):
    """
    Send change email notification with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        code: Email verification code
        phase: Change email phase ('old_email' or 'new_email')
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start change email mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_change_email(
            language_code=language,
            to=to,
            code=code,
            phase=phase,
        )

        end_at = time.perf_counter()
        logger.info(click.style(f"Send change email mail to {to} succeeded: latency: {end_at - start_at}", fg="green"))
    except Exception:
        logger.exception("Send change email mail to %s failed", to)


@shared_task(queue="mail")
def send_change_mail_completed_notification_task(language: str, to: str):
    """
    Send change email completed notification with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start change email completed notify mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.CHANGE_EMAIL_COMPLETED,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "email": to,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Send change email completed mail to {to} succeeded: latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("Send change email completed mail to %s failed", to)

```

### api/tasks/ops_trace_task.py
```py
import json
import logging

from celery import shared_task
from flask import current_app

from core.ops.entities.config_entity import OPS_FILE_PATH, OPS_TRACE_FAILED_KEY
from core.ops.entities.trace_entity import trace_info_info_map
from core.rag.models.document import Document
from extensions.ext_redis import redis_client
from extensions.ext_storage import storage
from models.model import Message
from models.workflow import WorkflowRun

logger = logging.getLogger(__name__)


@shared_task(queue="ops_trace")
def process_trace_tasks(file_info):
    """
    Async process trace tasks
    Usage: process_trace_tasks.delay(tasks_data)
    """
    from core.ops.ops_trace_manager import OpsTraceManager

    app_id = file_info.get("app_id")
    file_id = file_info.get("file_id")
    file_path = f"{OPS_FILE_PATH}{app_id}/{file_id}.json"
    file_data = json.loads(storage.load(file_path))
    trace_info = file_data.get("trace_info")
    trace_info_type = file_data.get("trace_info_type")
    trace_instance = OpsTraceManager.get_ops_trace_instance(app_id)

    if trace_info.get("message_data"):
        trace_info["message_data"] = Message.from_dict(data=trace_info["message_data"])
    if trace_info.get("workflow_data"):
        trace_info["workflow_data"] = WorkflowRun.from_dict(data=trace_info["workflow_data"])
    if trace_info.get("documents"):
        trace_info["documents"] = [Document.model_validate(doc) for doc in trace_info["documents"]]

    try:
        trace_type = trace_info_info_map.get(trace_info_type)
        if trace_type:
            trace_info = trace_type(**trace_info)

        from extensions.ext_enterprise_telemetry import is_enabled as is_ee_telemetry_enabled

        if is_ee_telemetry_enabled():
            from enterprise.telemetry.enterprise_trace import EnterpriseOtelTrace

            try:
                EnterpriseOtelTrace().trace(trace_info)
            except Exception:
                logger.exception("Enterprise trace failed for app_id: %s", app_id)

        if trace_instance:
            with current_app.app_context():
                trace_instance.trace(trace_info)

        logger.info("Processing trace tasks success, app_id: %s", app_id)
    except Exception as e:
        logger.exception("Processing trace tasks failed, app_id: %s", app_id)
        failed_key = f"{OPS_TRACE_FAILED_KEY}_{app_id}"
        redis_client.incr(failed_key)
    finally:
        try:
            storage.delete(file_path)
        except Exception as e:
            logger.warning(
                "Failed to delete trace file %s for app_id %s: %s",
                file_path,
                app_id,
                e,
            )

```

### api/tasks/disable_segments_from_index_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import select

from core.db.session_factory import session_factory
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from extensions.ext_redis import redis_client
from models.dataset import Dataset, DocumentSegment, SegmentAttachmentBinding
from models.dataset import Document as DatasetDocument

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def disable_segments_from_index_task(segment_ids: list, dataset_id: str, document_id: str):
    """
    Async disable segments from index
    :param segment_ids: list of segment ids
    :param dataset_id: dataset id
    :param document_id: document id

    Usage: disable_segments_from_index_task.delay(segment_ids, dataset_id, document_id)
    """
    start_at = time.perf_counter()

    with session_factory.create_session() as session:
        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if not dataset:
            logger.info(click.style(f"Dataset {dataset_id} not found, pass.", fg="cyan"))
            return

        dataset_document = session.query(DatasetDocument).where(DatasetDocument.id == document_id).first()

        if not dataset_document:
            logger.info(click.style(f"Document {document_id} not found, pass.", fg="cyan"))
            return
        if not dataset_document.enabled or dataset_document.archived or dataset_document.indexing_status != "completed":
            logger.info(click.style(f"Document {document_id} status is invalid, pass.", fg="cyan"))
            return
        # sync index processor
        index_processor = IndexProcessorFactory(dataset_document.doc_form).init_index_processor()

        segments = session.scalars(
            select(DocumentSegment).where(
                DocumentSegment.id.in_(segment_ids),
                DocumentSegment.dataset_id == dataset_id,
                DocumentSegment.document_id == document_id,
            )
        ).all()

        if not segments:
            return

        try:
            index_node_ids = [segment.index_node_id for segment in segments]
            if dataset.is_multimodal:
                segment_ids = [segment.id for segment in segments]
                segment_attachment_bindings = (
                    session.query(SegmentAttachmentBinding)
                    .where(SegmentAttachmentBinding.segment_id.in_(segment_ids))
                    .all()
                )
                if segment_attachment_bindings:
                    attachment_ids = [binding.attachment_id for binding in segment_attachment_bindings]
                    index_node_ids.extend(attachment_ids)
            index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=False)

            # Disable summary indexes for these segments
            from services.summary_index_service import SummaryIndexService

            segment_ids_list = [segment.id for segment in segments]
            try:
                # Get disabled_by from first segment (they should all have the same disabled_by)
                disabled_by = segments[0].disabled_by if segments else None
                SummaryIndexService.disable_summaries_for_segments(
                    dataset=dataset,
                    segment_ids=segment_ids_list,
                    disabled_by=disabled_by,
                )
            except Exception as e:
                logger.warning("Failed to disable summaries for segments: %s", str(e))

            end_at = time.perf_counter()
            logger.info(click.style(f"Segments removed from index latency: {end_at - start_at}", fg="green"))
        except Exception:
            # update segment error msg
            session.query(DocumentSegment).where(
                DocumentSegment.id.in_(segment_ids),
                DocumentSegment.dataset_id == dataset_id,
                DocumentSegment.document_id == document_id,
            ).update(
                {
                    "disabled_at": None,
                    "disabled_by": None,
                    "enabled": True,
                }
            )
            session.commit()
        finally:
            for segment in segments:
                indexing_cache_key = f"segment_{segment.id}_indexing"
                redis_client.delete(indexing_cache_key)

```

### api/tasks/document_indexing_update_task.py
```py
import logging
import time

import click
from celery import shared_task
from sqlalchemy import delete, select

from core.db.session_factory import session_factory
from core.indexing_runner import DocumentIsPausedError, IndexingRunner
from core.rag.index_processor.index_processor_factory import IndexProcessorFactory
from libs.datetime_utils import naive_utc_now
from models.dataset import Dataset, Document, DocumentSegment
from models.enums import IndexingStatus

logger = logging.getLogger(__name__)


@shared_task(queue="dataset")
def document_indexing_update_task(dataset_id: str, document_id: str):
    """
    Async update document
    :param dataset_id:
    :param document_id:

    Usage: document_indexing_update_task.delay(dataset_id, document_id)
    """
    logger.info(click.style(f"Start update document: {document_id}", fg="green"))
    start_at = time.perf_counter()

    with session_factory.create_session() as session, session.begin():
        document = session.query(Document).where(Document.id == document_id, Document.dataset_id == dataset_id).first()

        if not document:
            logger.info(click.style(f"Document not found: {document_id}", fg="red"))
            return

        document.indexing_status = IndexingStatus.PARSING
        document.processing_started_at = naive_utc_now()

        dataset = session.query(Dataset).where(Dataset.id == dataset_id).first()
        if not dataset:
            return

        index_type = document.doc_form
        segments = session.scalars(select(DocumentSegment).where(DocumentSegment.document_id == document_id)).all()
        index_node_ids = [segment.index_node_id for segment in segments]

    clean_success = False
    try:
        index_processor = IndexProcessorFactory(index_type).init_index_processor()
        if index_node_ids:
            index_processor.clean(dataset, index_node_ids, with_keywords=True, delete_child_chunks=True)
            end_at = time.perf_counter()
            logger.info(
                click.style(
                    "Cleaned document when document update data source or process rule: {} latency: {}".format(
                        document_id, end_at - start_at
                    ),
                    fg="green",
                )
            )
            clean_success = True
    except Exception:
        logger.exception("Failed to clean document index during update, document_id: %s", document_id)

    if clean_success:
        with session_factory.create_session() as session, session.begin():
            segment_delete_stmt = delete(DocumentSegment).where(DocumentSegment.document_id == document_id)
            session.execute(segment_delete_stmt)

    try:
        indexing_runner = IndexingRunner()
        indexing_runner.run([document])
        end_at = time.perf_counter()
        logger.info(click.style(f"update document: {document.id} latency: {end_at - start_at}", fg="green"))
    except DocumentIsPausedError as ex:
        logger.info(click.style(str(ex), fg="yellow"))
    except Exception:
        logger.exception("document_indexing_update_task failed, document_id: %s", document_id)

```

### api/tasks/workflow_cfs_scheduler/cfs_scheduler.py
```py
from services.workflow.entities import WorkflowScheduleCFSPlanEntity
from services.workflow.scheduler import CFSPlanScheduler, SchedulerCommand
from tasks.workflow_cfs_scheduler.entities import AsyncWorkflowQueue


class AsyncWorkflowCFSPlanEntity(WorkflowScheduleCFSPlanEntity):
    """
    Trigger workflow CFS plan entity.
    """

    queue: AsyncWorkflowQueue


class AsyncWorkflowCFSPlanScheduler(CFSPlanScheduler):
    """
    Trigger workflow CFS plan scheduler.
    """

    plan: AsyncWorkflowCFSPlanEntity

    def can_schedule(self) -> SchedulerCommand:
        """
        Check if the workflow can be scheduled.
        """
        if self.plan.queue in [AsyncWorkflowQueue.PROFESSIONAL_QUEUE, AsyncWorkflowQueue.TEAM_QUEUE]:
            """
            permitted all paid users to schedule the workflow any time
            """
            return SchedulerCommand.NONE

        # FIXME: avoid the sandbox user's workflow at a running state for ever
        return SchedulerCommand.RESOURCE_LIMIT_REACHED

```

### api/tasks/workflow_cfs_scheduler/entities.py
```py
from enum import StrEnum

from configs import dify_config
from services.workflow.entities import WorkflowScheduleCFSPlanEntity

# Determine queue names based on edition
if dify_config.EDITION == "CLOUD":
    # Cloud edition: separate queues for different tiers
    _professional_queue = "workflow_professional"
    _team_queue = "workflow_team"
    _sandbox_queue = "workflow_sandbox"
    AsyncWorkflowSystemStrategy = WorkflowScheduleCFSPlanEntity.Strategy.TimeSlice
else:
    # Community edition: single workflow queue (not dataset)
    _professional_queue = "workflow"
    _team_queue = "workflow"
    _sandbox_queue = "workflow"
    AsyncWorkflowSystemStrategy = WorkflowScheduleCFSPlanEntity.Strategy.Nop


class AsyncWorkflowQueue(StrEnum):
    # Define constants
    PROFESSIONAL_QUEUE = _professional_queue
    TEAM_QUEUE = _team_queue
    SANDBOX_QUEUE = _sandbox_queue

```

### api/tasks/workflow_schedule_tasks.py
```py
import logging

from celery import shared_task

from core.db.session_factory import session_factory
from core.workflow.nodes.trigger_schedule.exc import (
    ScheduleExecutionError,
    ScheduleNotFoundError,
    TenantOwnerNotFoundError,
)
from enums.quota_type import QuotaType, unlimited
from models.trigger import WorkflowSchedulePlan
from services.async_workflow_service import AsyncWorkflowService
from services.errors.app import QuotaExceededError
from services.trigger.app_trigger_service import AppTriggerService
from services.trigger.schedule_service import ScheduleService
from services.workflow.entities import ScheduleTriggerData

logger = logging.getLogger(__name__)


@shared_task(queue="schedule_executor")
def run_schedule_trigger(schedule_id: str) -> None:
    """
    Execute a scheduled workflow trigger.

    Note: No retry logic needed as schedules will run again at next interval.
    The execution result is tracked via WorkflowTriggerLog.

    Raises:
        ScheduleNotFoundError: If schedule doesn't exist
        TenantOwnerNotFoundError: If no owner/admin for tenant
        ScheduleExecutionError: If workflow trigger fails
    """
    with session_factory.create_session() as session:
        schedule = session.get(WorkflowSchedulePlan, schedule_id)
        if not schedule:
            raise ScheduleNotFoundError(f"Schedule {schedule_id} not found")

        tenant_owner = ScheduleService.get_tenant_owner(session, schedule.tenant_id)
        if not tenant_owner:
            raise TenantOwnerNotFoundError(f"No owner or admin found for tenant {schedule.tenant_id}")

        quota_charge = unlimited()
        try:
            quota_charge = QuotaType.TRIGGER.consume(schedule.tenant_id)
        except QuotaExceededError:
            AppTriggerService.mark_tenant_triggers_rate_limited(schedule.tenant_id)
            logger.info("Tenant %s rate limited, skipping schedule trigger %s", schedule.tenant_id, schedule_id)
            return

        try:
            # Production dispatch: Trigger the workflow normally
            response = AsyncWorkflowService.trigger_workflow_async(
                session=session,
                user=tenant_owner,
                trigger_data=ScheduleTriggerData(
                    app_id=schedule.app_id,
                    root_node_id=schedule.node_id,
                    inputs={},
                    tenant_id=schedule.tenant_id,
                ),
            )
            logger.info("Schedule %s triggered workflow: %s", schedule_id, response.workflow_trigger_log_id)
        except Exception as e:
            quota_charge.refund()
            raise ScheduleExecutionError(
                f"Failed to trigger workflow for schedule {schedule_id}, app {schedule.app_id}"
            ) from e

```

### api/tasks/mail_owner_transfer_task.py
```py
import logging
import time

import click
from celery import shared_task

from extensions.ext_mail import mail
from libs.email_i18n import EmailType, get_email_i18n_service

logger = logging.getLogger(__name__)


@shared_task(queue="mail")
def send_owner_transfer_confirm_task(language: str, to: str, code: str, workspace: str):
    """
    Send owner transfer confirmation email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        code: Verification code
        workspace: Workspace name
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start owner transfer confirm mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.OWNER_TRANSFER_CONFIRM,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "code": code,
                "WorkspaceName": workspace,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Send owner transfer confirm mail to {to} succeeded: latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("owner transfer confirm email mail to %s failed", to)


@shared_task(queue="mail")
def send_old_owner_transfer_notify_email_task(language: str, to: str, workspace: str, new_owner_email: str):
    """
    Send old owner transfer notification email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        workspace: Workspace name
        new_owner_email: New owner email address
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start old owner transfer notify mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.OWNER_TRANSFER_OLD_NOTIFY,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "WorkspaceName": workspace,
                "NewOwnerEmail": new_owner_email,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Send old owner transfer notify mail to {to} succeeded: latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("old owner transfer notify email mail to %s failed", to)


@shared_task(queue="mail")
def send_new_owner_transfer_notify_email_task(language: str, to: str, workspace: str):
    """
    Send new owner transfer notification email with internationalization support.

    Args:
        language: Language code for email localization
        to: Recipient email address
        workspace: Workspace name
    """
    if not mail.is_inited():
        return

    logger.info(click.style(f"Start new owner transfer notify mail to {to}", fg="green"))
    start_at = time.perf_counter()

    try:
        email_service = get_email_i18n_service()
        email_service.send_email(
            email_type=EmailType.OWNER_TRANSFER_NEW_NOTIFY,
            language_code=language,
            to=to,
            template_context={
                "to": to,
                "WorkspaceName": workspace,
            },
        )

        end_at = time.perf_counter()
        logger.info(
            click.style(
                f"Send new owner transfer notify mail to {to} succeeded: latency: {end_at - start_at}",
                fg="green",
            )
        )
    except Exception:
        logger.exception("new owner transfer notify email mail to %s failed", to)

```

### api/celery_entrypoint.py
```py
import psycogreen.gevent as pscycogreen_gevent  # type: ignore
from grpc.experimental import gevent as grpc_gevent  # type: ignore

# grpc gevent
grpc_gevent.init_gevent()
print("gRPC patched with gevent.", flush=True)  # noqa: T201
pscycogreen_gevent.patch_psycopg()
print("psycopg2 patched with gevent.", flush=True)  # noqa: T201


from app import app, celery

__all__ = ["app", "celery"]

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-016.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
