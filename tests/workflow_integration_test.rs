#![recursion_limit = "512"]

use avored_rust_cms::{
    avored_state::test_avored_state,
    core::{
        application::dtos::{
            CreateWorkflowCommand, SetWorkflowActiveCommand, UpdateWorkflowCommand,
            WorkflowPaginationQuery,
        },
        domain::entities::{
            WorkflowAction, WorkflowEvent, WorkflowExecutionStatus, WorkflowTrigger,
        },
    },
    infrastructure::workflow_dispatcher::{WorkflowActionExecutor, WorkflowDispatcher},
    infrastructure::{migrations::db_init, persistence::into_iter_objects},
};
use serde_json::json;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use surrealdb_core::{dbs::Session, kvs::Datastore};

struct TestActionExecutor {
    calls: Arc<AtomicUsize>,
    fail_on_call: Option<usize>,
}

#[async_trait::async_trait]
impl WorkflowActionExecutor for TestActionExecutor {
    async fn execute(
        &self,
        _action: &WorkflowAction,
        _payload: &serde_json::Value,
    ) -> Result<(), String> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        if self.fail_on_call == Some(call) {
            Err("simulated webhook failure".to_string())
        } else {
            Ok(())
        }
    }
}

fn command(name: &str, event: WorkflowEvent, is_active: bool) -> CreateWorkflowCommand {
    CreateWorkflowCommand {
        name: name.to_string(),
        trigger: WorkflowTrigger {
            entity_type: "customers".to_string(),
            event,
        },
        is_active: Some(is_active),
        actions: vec![WorkflowAction::Webhook {
            url: "https://example.test/hook".to_string(),
        }],
    }
}

#[tokio::test]
async fn workflows_can_be_created_updated_activated_and_deleted() {
    let state = test_avored_state().await;
    let created = state
        .workflow_use_case
        .create(command("Customer created", WorkflowEvent::Created, false))
        .await
        .expect("workflow should be created");

    assert_eq!(created.name, "Customer created");
    assert!(!created.is_active);
    assert!(state
        .workflow_use_case
        .find_active_by_trigger("customers", WorkflowEvent::Created)
        .await
        .expect("trigger lookup should succeed")
        .is_empty());

    let updated = state
        .workflow_use_case
        .update(
            &created.id,
            UpdateWorkflowCommand {
                name: "Customer updated".to_string(),
                trigger: created.trigger.clone(),
                is_active: true,
                actions: vec![WorkflowAction::Webhook {
                    url: "https://example.test/updated".to_string(),
                }],
            },
        )
        .await
        .expect("workflow should be updated");
    assert_eq!(updated.name, "Customer updated");
    assert!(updated.is_active);

    let matches = state
        .workflow_use_case
        .find_active_by_trigger("customers", WorkflowEvent::Created)
        .await
        .expect("trigger lookup should succeed");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id, created.id);
    assert!(state
        .workflow_use_case
        .find_active_by_trigger("customers", WorkflowEvent::Updated)
        .await
        .expect("non-matching trigger lookup should succeed")
        .is_empty());

    state
        .workflow_use_case
        .set_active(&created.id, SetWorkflowActiveCommand { is_active: false })
        .await
        .expect("workflow should be deactivated");
    assert!(state
        .workflow_use_case
        .find_active_by_trigger("customers", WorkflowEvent::Created)
        .await
        .expect("trigger lookup should succeed")
        .is_empty());
    state
        .workflow_use_case
        .set_active(&created.id, SetWorkflowActiveCommand { is_active: true })
        .await
        .expect("workflow should be activated");

    let execution = state
        .workflow_use_case
        .create_execution(&created.id, json!({"preserve_history": true}))
        .await
        .expect("execution should be created before delete");
    state
        .workflow_use_case
        .delete(&created.id)
        .await
        .expect("workflow should be deleted");
    assert!(state
        .workflow_use_case
        .find_by_id(&created.id)
        .await
        .is_err());
    let history = state
        .workflow_use_case
        .find_executions(&created.id, WorkflowPaginationQuery::default())
        .await
        .expect("execution history should remain available after workflow deletion");
    assert_eq!(history.total, 1);
    assert_eq!(history.data[0].id, execution.id);
}

#[tokio::test]
async fn workflow_list_supports_paginated_results_and_total_count() {
    let state = test_avored_state().await;
    state
        .workflow_use_case
        .create(command("First workflow", WorkflowEvent::Created, true))
        .await
        .expect("first workflow should be created");
    state
        .workflow_use_case
        .create(command("Second workflow", WorkflowEvent::Updated, true))
        .await
        .expect("second workflow should be created");

    let page = state
        .workflow_use_case
        .find_all(WorkflowPaginationQuery {
            page: Some(0),
            page_size: Some(1),
        })
        .await
        .expect("workflow page should be fetched");
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 1);
    assert_eq!(page.data.len(), 1);
    assert_eq!(page.total, 2);
}

#[tokio::test]
async fn workflow_execution_status_and_history_are_persisted() {
    let state = test_avored_state().await;
    let workflow = state
        .workflow_use_case
        .create(command("Execution test", WorkflowEvent::Created, true))
        .await
        .expect("workflow should be created");

    let running = state
        .workflow_use_case
        .create_execution(
            &workflow.id,
            json!({"record_id": "customer-123", "email": "person@example.test"}),
        )
        .await
        .expect("execution should be created");
    assert_eq!(running.status, WorkflowExecutionStatus::Running);
    assert_eq!(running.workflow_id, workflow.id);

    let failed = state
        .workflow_use_case
        .update_execution_status(
            &running.id,
            WorkflowExecutionStatus::Failed,
            Some("Webhook returned HTTP 500".to_string()),
        )
        .await
        .expect("execution status should be updated");
    assert_eq!(failed.status, WorkflowExecutionStatus::Failed);
    assert_eq!(
        failed.error_message.as_deref(),
        Some("Webhook returned HTTP 500")
    );

    let history = state
        .workflow_use_case
        .find_executions(&workflow.id, WorkflowPaginationQuery::default())
        .await
        .expect("workflow history should be listed");
    assert_eq!(history.total, 1);
    assert_eq!(history.data[0].id, running.id);
    assert_eq!(history.data[0].payload["record_id"], "customer-123");
}

#[tokio::test]
async fn workflow_schema_migration_is_idempotent_and_preserves_records() {
    let state = test_avored_state().await;
    let workflow = state
        .workflow_use_case
        .create(command("Migration test", WorkflowEvent::Updated, true))
        .await
        .expect("workflow should be created");
    let execution = state
        .workflow_use_case
        .create_execution(&workflow.id, json!({"field": "value"}))
        .await
        .expect("execution should be created");

    let (datastore, session) = &state.database_provider.db;
    db_init::run(datastore, session)
        .await
        .expect("reapplying migrations should succeed");

    let persisted_workflow = state
        .workflow_use_case
        .find_by_id(&workflow.id)
        .await
        .expect("workflow should remain after migration");
    assert_eq!(persisted_workflow.name, "Migration test");

    let history = state
        .workflow_use_case
        .find_executions(&workflow.id, WorkflowPaginationQuery::default())
        .await
        .expect("execution history should remain after migration");
    assert_eq!(history.total, 1);
    assert_eq!(history.data[0].id, execution.id);
}

#[tokio::test]
async fn workflow_migration_runs_for_a_database_with_only_the_initial_migration() {
    let datastore = Datastore::new("mem://")
        .await
        .expect("in-memory datastore should initialize");
    let root_session = Session::owner();
    datastore
        .execute(
            "DEFINE NAMESPACE IF NOT EXISTS `workflow_legacy_test`;",
            &root_session,
            None,
        )
        .await
        .expect("namespace should be created");
    let namespace_session = Session::owner().with_ns("workflow_legacy_test");
    datastore
        .execute(
            "DEFINE DATABASE IF NOT EXISTS `workflow_legacy_test`;",
            &namespace_session,
            None,
        )
        .await
        .expect("database should be created");
    let session = Session::owner()
        .with_ns("workflow_legacy_test")
        .with_db("workflow_legacy_test");
    datastore
        .execute(
            "DEFINE TABLE IF NOT EXISTS _migrations SCHEMALESS;
             CREATE _migrations:0001_initial_schema SET name = 'initial schema';",
            &session,
            None,
        )
        .await
        .expect("existing initial migration marker should be created");

    db_init::run(&datastore, &session)
        .await
        .expect("workflow migration should run after the initial migration");

    let workflow = datastore
        .execute(
            "CREATE workflows SET name = 'Legacy workflow', \
             trigger_entity = 'customers', trigger_event = 'created', \
             is_active = true, actions = [], created_at = time::now(), \
             updated_at = time::now(), deleted_at = NONE RETURN AFTER;",
            &session,
            None,
        )
        .await
        .expect("workflow schema should be installed");
    assert!(into_iter_objects(workflow)
        .expect("created workflow response should be valid")
        .next()
        .is_some());

    let migration_marker = datastore
        .execute(
            "SELECT * FROM _migrations:0001_initial_schema;",
            &session,
            None,
        )
        .await
        .expect("initial migration marker should remain");
    assert!(into_iter_objects(migration_marker)
        .expect("initial migration response should be valid")
        .next()
        .is_some());
}

#[tokio::test]
async fn workflow_use_case_rejects_blank_required_fields_and_reports_missing_records() {
    let state = test_avored_state().await;
    let mut invalid = command(" ", WorkflowEvent::Created, true);
    invalid.trigger.entity_type.clear();
    assert!(state.workflow_use_case.create(invalid).await.is_err());

    assert!(state
        .workflow_use_case
        .set_active(
            "missing-workflow",
            SetWorkflowActiveCommand { is_active: true }
        )
        .await
        .is_err());
}

#[tokio::test]
async fn dispatcher_runs_only_active_matching_workflows_and_records_success() {
    let state = test_avored_state().await;
    let matching = state
        .workflow_use_case
        .create(command("Matching", WorkflowEvent::Created, true))
        .await
        .expect("matching workflow should be created");
    let inactive = state
        .workflow_use_case
        .create(command("Inactive", WorkflowEvent::Created, false))
        .await
        .expect("inactive workflow should be created");
    let non_matching = state
        .workflow_use_case
        .create(command("Non-matching", WorkflowEvent::Updated, true))
        .await
        .expect("non-matching workflow should be created");

    let calls = Arc::new(AtomicUsize::new(0));
    let dispatcher = WorkflowDispatcher::with_executor(
        state.workflow_use_case.clone(),
        TestActionExecutor {
            calls: calls.clone(),
            fail_on_call: None,
        },
    );
    let payload = json!({"id": "customer-1", "attributes": {"name": "Ada"}});
    dispatcher
        .dispatch("customers", WorkflowEvent::Created, payload.clone())
        .await
        .expect("event dispatch should succeed");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let matching_history = state
        .workflow_use_case
        .find_executions(&matching.id, WorkflowPaginationQuery::default())
        .await
        .expect("matching workflow history should load");
    assert_eq!(matching_history.total, 1);
    assert_eq!(
        matching_history.data[0].status,
        WorkflowExecutionStatus::Success
    );
    assert_eq!(matching_history.data[0].payload, payload);
    for workflow in [inactive, non_matching] {
        let history = state
            .workflow_use_case
            .find_executions(&workflow.id, WorkflowPaginationQuery::default())
            .await
            .expect("workflow history should load");
        assert_eq!(history.total, 0);
    }
}

#[tokio::test]
async fn dispatcher_records_action_failure_and_stops_the_action_chain() {
    let state = test_avored_state().await;
    let mut definition = command("Failing chain", WorkflowEvent::Created, true);
    definition.actions.push(WorkflowAction::Webhook {
        url: "https://example.test/second".to_string(),
    });
    let workflow = state
        .workflow_use_case
        .create(definition)
        .await
        .expect("workflow should be created");

    let calls = Arc::new(AtomicUsize::new(0));
    let dispatcher = WorkflowDispatcher::with_executor(
        state.workflow_use_case.clone(),
        TestActionExecutor {
            calls: calls.clone(),
            fail_on_call: Some(1),
        },
    );
    dispatcher
        .dispatch(
            "customers",
            WorkflowEvent::Created,
            json!({"id": "customer-2"}),
        )
        .await
        .expect("dispatch should record action failure");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let history = state
        .workflow_use_case
        .find_executions(&workflow.id, WorkflowPaginationQuery::default())
        .await
        .expect("workflow history should load");
    assert_eq!(history.total, 1);
    assert_eq!(history.data[0].status, WorkflowExecutionStatus::Failed);
    assert_eq!(
        history.data[0].error_message.as_deref(),
        Some("simulated webhook failure")
    );
}

#[tokio::test]
async fn email_actions_fail_visibly_until_a_sender_is_configured() {
    let executor = avored_rust_cms::infrastructure::workflow_dispatcher::HttpWebhookExecutor::new();
    let result = executor
        .execute(
            &WorkflowAction::Email {
                template_id: "welcome".to_string(),
                recipient: "person@example.test".to_string(),
            },
            &json!({"id": "customer-3"}),
        )
        .await;
    assert_eq!(
        result,
        Err("Email actions are not executable until an email sender is configured".to_string())
    );
}
