You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-021
- **Kind**: public_api
- **Identifier**: Celery Task Workers (document indexing, app generation, account deletion)
- **Description**: Asynchronous Celery workers processing user data including document indexing and app generation. Task deserialization, Redis queue poisoning, and privilege escalation in background processing
- **Locations**: api/celery_entrypoint.py, api/tasks/document_indexing_task.py, api/tasks/app_generate/, api/tasks/delete_account_task.py, api/tasks/rag_pipeline/rag_pipeline_run_task.py

## Source Code

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-021.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
