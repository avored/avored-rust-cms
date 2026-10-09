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
    infrastructure::{migrations::db_init, persistence::into_iter_objects},
};
use serde_json::json;
use surrealdb_core::{dbs::Session, kvs::Datastore};

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
