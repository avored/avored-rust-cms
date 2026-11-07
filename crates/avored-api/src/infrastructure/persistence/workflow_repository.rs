use std::{collections::BTreeMap, sync::Arc};

use serde_json::Value as JsonValue;
use surrealdb::types::{Number, Object, RecordIdKey, Value};

use crate::{
    core::domain::{
        constants::{WORKFLOWS_TABLE_NAME, WORKFLOW_EXECUTIONS_TABLE_NAME},
        entities::{
            WorkflowAction, WorkflowEvent, WorkflowExecutionModel, WorkflowExecutionStatus,
            WorkflowModel, WorkflowTrigger,
        },
        extensions::object_extension::ObjectExtension,
        repositories::WorkflowRepository,
    },
    error::{Error, Result},
    infrastructure::persistence::{into_iter_objects, RecordIdExtension},
    providers::avored_database_provider::AvoRedDatabaseProvider,
};

#[derive(Clone)]
pub struct WorkflowRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl WorkflowRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

#[async_trait::async_trait]
impl WorkflowRepository for WorkflowRepositoryImpl {
    async fn create(
        &self,
        name: String,
        trigger: WorkflowTrigger,
        is_active: bool,
        actions: Vec<WorkflowAction>,
    ) -> Result<WorkflowModel> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "CREATE {WORKFLOWS_TABLE_NAME} SET \
             name = $name, trigger_entity = $trigger_entity, trigger_event = $trigger_event, \
             is_active = $is_active, actions = $actions, \
             created_at = time::now(), updated_at = time::now(), deleted_at = NONE RETURN AFTER;"
        );
        let values = BTreeMap::from([
            ("name".to_string(), Value::String(name)),
            (
                "trigger_entity".to_string(),
                Value::String(trigger.entity_type),
            ),
            (
                "trigger_event".to_string(),
                Value::String(event_name(trigger.event).to_string()),
            ),
            ("is_active".to_string(), Value::Bool(is_active)),
            (
                "actions".to_string(),
                json_to_surreal_value(serde_json::to_value(actions)?),
            ),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let object = first_object(response, "Workflow create returned no record")?;
        workflow_from_object(object)
    }

    async fn find_all(&self, page: u64, page_size: u64) -> Result<(Vec<WorkflowModel>, u64)> {
        let (datastore, session) = &self.database_provider.db;
        let skip = page.saturating_sub(1).saturating_mul(page_size);
        let sql = format!(
            "SELECT * FROM {WORKFLOWS_TABLE_NAME} WHERE deleted_at = NONE ORDER BY created_at DESC \
             LIMIT $limit START $skip;"
        );
        let values = pagination_values(page_size, skip);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let mut models = Vec::new();
        for object in into_iter_objects(response)? {
            models.push(workflow_from_object(object?)?);
        }

        let count_sql = format!(
            "SELECT count() AS total FROM {WORKFLOWS_TABLE_NAME} \
             WHERE deleted_at = NONE GROUP ALL;"
        );
        let count_response = datastore.execute(&count_sql, session, None).await?;
        let total = read_count(count_response)?;
        Ok((models, total))
    }

    async fn find_by_id(&self, id: &str) -> Result<WorkflowModel> {
        let (datastore, session) = &self.database_provider.db;
        let sql =
            format!("SELECT * FROM {WORKFLOWS_TABLE_NAME} WHERE id = $id AND deleted_at = NONE;");
        let values = BTreeMap::from([(
            "id".to_string(),
            Value::RecordId(id.to_record_id(WORKFLOWS_TABLE_NAME)),
        )]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        workflow_from_object(first_object_or_not_found(
            response,
            &format!("Workflow '{id}' was not found"),
        )?)
    }

    async fn find_active_by_trigger(
        &self,
        entity_type: &str,
        event: WorkflowEvent,
    ) -> Result<Vec<WorkflowModel>> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "SELECT * FROM {WORKFLOWS_TABLE_NAME} \
             WHERE trigger_entity = $trigger_entity AND trigger_event = $trigger_event \
             AND is_active = true AND deleted_at = NONE ORDER BY created_at ASC;"
        );
        let values = BTreeMap::from([
            (
                "trigger_entity".to_string(),
                Value::String(entity_type.to_string()),
            ),
            (
                "trigger_event".to_string(),
                Value::String(event_name(event).to_string()),
            ),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let mut workflows = Vec::new();
        for object in into_iter_objects(response)? {
            workflows.push(workflow_from_object(object?)?);
        }
        Ok(workflows)
    }

    async fn update(
        &self,
        id: &str,
        name: String,
        trigger: WorkflowTrigger,
        is_active: bool,
        actions: Vec<WorkflowAction>,
    ) -> Result<WorkflowModel> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "UPDATE {WORKFLOWS_TABLE_NAME} SET \
             name = $name, trigger_entity = $trigger_entity, trigger_event = $trigger_event, \
             is_active = $is_active, actions = $actions, updated_at = time::now() \
             WHERE id = $id AND deleted_at = NONE RETURN AFTER;"
        );
        let values = BTreeMap::from([
            (
                "id".to_string(),
                Value::RecordId(id.to_record_id(WORKFLOWS_TABLE_NAME)),
            ),
            ("name".to_string(), Value::String(name)),
            (
                "trigger_entity".to_string(),
                Value::String(trigger.entity_type),
            ),
            (
                "trigger_event".to_string(),
                Value::String(event_name(trigger.event).to_string()),
            ),
            ("is_active".to_string(), Value::Bool(is_active)),
            (
                "actions".to_string(),
                json_to_surreal_value(serde_json::to_value(actions)?),
            ),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        workflow_from_object(first_object_or_not_found(
            response,
            &format!("Workflow '{id}' was not found"),
        )?)
    }

    async fn set_active(&self, id: &str, is_active: bool) -> Result<WorkflowModel> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "UPDATE {WORKFLOWS_TABLE_NAME} SET \
            is_active = $is_active, updated_at = time::now() \
            WHERE id = $id AND deleted_at = NONE RETURN AFTER;"
        );
        let values = BTreeMap::from([
            (
                "id".to_string(),
                Value::RecordId(id.to_record_id(WORKFLOWS_TABLE_NAME)),
            ),
            ("is_active".to_string(), Value::Bool(is_active)),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        workflow_from_object(first_object_or_not_found(
            response,
            &format!("Workflow '{id}' was not found"),
        )?)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "UPDATE {WORKFLOWS_TABLE_NAME} SET is_active = false, \
             deleted_at = time::now(), updated_at = time::now() \
             WHERE id = $id AND deleted_at = NONE RETURN BEFORE;"
        );
        let values = BTreeMap::from([(
            "id".to_string(),
            Value::RecordId(id.to_record_id(WORKFLOWS_TABLE_NAME)),
        )]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        match into_iter_objects(response)?.next() {
            Some(Ok(_)) => Ok(()),
            Some(Err(error)) => Err(error),
            None => Err(Error::NotFound(format!("Workflow '{id}' was not found"))),
        }
    }

    async fn create_execution(
        &self,
        workflow_id: &str,
        payload: JsonValue,
    ) -> Result<WorkflowExecutionModel> {
        self.find_by_id(workflow_id).await?;
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "CREATE {WORKFLOW_EXECUTIONS_TABLE_NAME} SET \
             workflow_id = $workflow_id, status = 'running', payload = $payload, \
             error_message = NONE, created_at = time::now(), updated_at = time::now() RETURN AFTER;"
        );
        let values = BTreeMap::from([
            (
                "workflow_id".to_string(),
                Value::String(normalize_id(WORKFLOWS_TABLE_NAME, workflow_id)),
            ),
            ("payload".to_string(), json_to_surreal_value(payload)),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        execution_from_object(first_object(
            response,
            "Workflow execution create returned no record",
        )?)
    }

    async fn update_execution_status(
        &self,
        id: &str,
        status: WorkflowExecutionStatus,
        error_message: Option<String>,
    ) -> Result<WorkflowExecutionModel> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!(
            "UPDATE {WORKFLOW_EXECUTIONS_TABLE_NAME} SET \
             status = $status, error_message = $error_message, updated_at = time::now() \
             WHERE id = $id RETURN AFTER;"
        );
        let values = BTreeMap::from([
            (
                "id".to_string(),
                Value::RecordId(id.to_record_id(WORKFLOW_EXECUTIONS_TABLE_NAME)),
            ),
            (
                "status".to_string(),
                Value::String(status_name(status).to_string()),
            ),
            (
                "error_message".to_string(),
                error_message.map(Value::String).unwrap_or(Value::None),
            ),
        ]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        execution_from_object(first_object_or_not_found(
            response,
            &format!("Workflow execution '{id}' was not found"),
        )?)
    }

    async fn find_executions(
        &self,
        workflow_id: &str,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<WorkflowExecutionModel>, u64)> {
        let (datastore, session) = &self.database_provider.db;
        let exists_sql = format!("SELECT id FROM {WORKFLOWS_TABLE_NAME} WHERE id = $id;");
        let exists_values = BTreeMap::from([(
            "id".to_string(),
            Value::RecordId(workflow_id.to_record_id(WORKFLOWS_TABLE_NAME)),
        )]);
        let exists_response = datastore
            .execute(&exists_sql, session, Some(exists_values.into()))
            .await?;
        match into_iter_objects(exists_response)?.next() {
            Some(Ok(_)) => {}
            Some(Err(error)) => return Err(error),
            None => {
                return Err(Error::NotFound(format!(
                    "Workflow '{workflow_id}' was not found"
                )))
            }
        }

        let skip = page.saturating_sub(1).saturating_mul(page_size);
        let sql = format!(
            "SELECT * FROM {WORKFLOW_EXECUTIONS_TABLE_NAME} \
             WHERE workflow_id = $workflow_id ORDER BY created_at DESC \
             LIMIT $limit START $skip;"
        );
        let mut values = pagination_values(page_size, skip);
        values.insert(
            "workflow_id".to_string(),
            Value::String(normalize_id(WORKFLOWS_TABLE_NAME, workflow_id)),
        );
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let mut executions = Vec::new();
        for object in into_iter_objects(response)? {
            executions.push(execution_from_object(object?)?);
        }

        let count_sql = format!(
            "SELECT count() AS total FROM {WORKFLOW_EXECUTIONS_TABLE_NAME} \
             WHERE workflow_id = $workflow_id GROUP ALL;"
        );
        let count_values = BTreeMap::from([(
            "workflow_id".to_string(),
            Value::String(normalize_id(WORKFLOWS_TABLE_NAME, workflow_id)),
        )]);
        let count_response = datastore
            .execute(&count_sql, session, Some(count_values.into()))
            .await?;
        let total = read_count(count_response)?;
        Ok((executions, total))
    }
}

fn pagination_values(page_size: u64, skip: u64) -> BTreeMap<String, Value> {
    BTreeMap::from([
        (
            "limit".to_string(),
            Value::Number(Number::Int(page_size.min(i64::MAX as u64) as i64)),
        ),
        (
            "skip".to_string(),
            Value::Number(Number::Int(skip.min(i64::MAX as u64) as i64)),
        ),
    ])
}

fn first_object(responses: Vec<surrealdb_rpc::QueryResult>, message: &str) -> Result<Object> {
    into_iter_objects(responses)?
        .next()
        .ok_or_else(|| Error::Generic(message.to_string()))?
}

fn first_object_or_not_found(
    responses: Vec<surrealdb_rpc::QueryResult>,
    message: &str,
) -> Result<Object> {
    match into_iter_objects(responses)?.next() {
        Some(Ok(object)) => Ok(object),
        Some(Err(error)) => Err(error),
        None => Err(Error::NotFound(message.to_string())),
    }
}

fn read_count(responses: Vec<surrealdb_rpc::QueryResult>) -> Result<u64> {
    let object = first_object(responses, "Count query returned no result")?;
    let total = object
        .get("total")
        .and_then(|value| match value {
            Value::Number(number) => (*number).into_int().ok(),
            _ => None,
        })
        .ok_or_else(|| Error::Generic("Count query returned an invalid total".to_string()))?;
    u64::try_from(total)
        .map_err(|_| Error::Generic("Count query returned a negative total".to_string()))
}

fn workflow_from_object(object: Object) -> Result<WorkflowModel> {
    let id = required_string(&object, "id")?;
    let entity_type = required_string(&object, "trigger_entity")?;
    let event = parse_event(&required_string(&object, "trigger_event")?)?;
    let is_active = match object.get("is_active") {
        Some(Value::Bool(value)) => *value,
        _ => {
            return Err(Error::Generic(
                "Workflow is_active field is invalid".to_string(),
            ))
        }
    };
    let actions_value = object
        .get("actions")
        .ok_or_else(|| Error::Generic("Workflow actions field is missing".to_string()))?
        .clone()
        .into_json_value();
    let actions = serde_json::from_value(actions_value)?;

    Ok(WorkflowModel {
        id,
        name: required_string(&object, "name")?,
        trigger: WorkflowTrigger { entity_type, event },
        is_active,
        actions,
        created_at: required_datetime(&object, "created_at")?,
        updated_at: required_datetime(&object, "updated_at")?,
    })
}

fn execution_from_object(object: Object) -> Result<WorkflowExecutionModel> {
    let id = required_string(&object, "id")?;
    let status = parse_status(&required_string(&object, "status")?)?;
    let payload = object
        .get("payload")
        .ok_or_else(|| Error::Generic("Execution payload field is missing".to_string()))?
        .clone()
        .into_json_value();
    let error_message = object.get_optional_string("error_message")?;

    Ok(WorkflowExecutionModel {
        id,
        workflow_id: required_string(&object, "workflow_id")?,
        status,
        payload,
        error_message,
        created_at: required_datetime(&object, "created_at")?,
        updated_at: required_datetime(&object, "updated_at")?,
    })
}

fn required_string(object: &Object, field: &str) -> Result<String> {
    match object.get(field) {
        Some(Value::String(value)) if !value.is_empty() => Ok(value.to_string()),
        Some(Value::RecordId(value)) if field == "id" => match &value.key {
            RecordIdKey::String(key) => Ok(key.clone()),
            key => Ok(format!("{key:?}")),
        },
        _ => Err(Error::Generic(format!(
            "Workflow persistence field '{field}' is missing or invalid"
        ))),
    }
}

fn required_datetime(object: &Object, field: &str) -> Result<surrealdb::types::Datetime> {
    match object.get(field) {
        Some(Value::Datetime(value)) => Ok(*value),
        _ => Err(Error::Generic(format!(
            "Workflow persistence field '{field}' is missing or invalid"
        ))),
    }
}

fn json_to_surreal_value(value: JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::None,
        JsonValue::Bool(value) => Value::Bool(value),
        JsonValue::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Value::Number(Number::Int(integer))
            } else if let Some(float) = value.as_f64() {
                Value::Number(Number::Float(float))
            } else {
                Value::String(value.to_string())
            }
        }
        JsonValue::String(value) => Value::String(value),
        JsonValue::Array(items) => {
            Value::Array(items.into_iter().map(json_to_surreal_value).collect())
        }
        JsonValue::Object(items) => {
            let mut object = Object::new();
            for (key, value) in items {
                object.insert(key, json_to_surreal_value(value));
            }
            Value::Object(object)
        }
    }
}

fn event_name(event: WorkflowEvent) -> &'static str {
    match event {
        WorkflowEvent::Created => "created",
        WorkflowEvent::Updated => "updated",
    }
}

fn parse_event(event: &str) -> Result<WorkflowEvent> {
    match event {
        "created" => Ok(WorkflowEvent::Created),
        "updated" => Ok(WorkflowEvent::Updated),
        _ => Err(Error::Generic(format!("Unknown workflow event '{event}'"))),
    }
}

fn status_name(status: WorkflowExecutionStatus) -> &'static str {
    match status {
        WorkflowExecutionStatus::Running => "running",
        WorkflowExecutionStatus::Success => "success",
        WorkflowExecutionStatus::Failed => "failed",
    }
}

fn parse_status(status: &str) -> Result<WorkflowExecutionStatus> {
    match status {
        "running" => Ok(WorkflowExecutionStatus::Running),
        "success" => Ok(WorkflowExecutionStatus::Success),
        "failed" => Ok(WorkflowExecutionStatus::Failed),
        _ => Err(Error::Generic(format!(
            "Unknown workflow execution status '{status}'"
        ))),
    }
}

fn normalize_id(table: &str, id: &str) -> String {
    id.strip_prefix(&format!("{table}:"))
        .unwrap_or(id)
        .to_string()
}
