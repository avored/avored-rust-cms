use serde_json::Value as JsonValue;

use crate::{
    core::domain::entities::{
        WorkflowAction, WorkflowEvent, WorkflowExecutionModel, WorkflowExecutionStatus,
        WorkflowModel, WorkflowTrigger,
    },
    error::Result,
};

#[async_trait::async_trait]
pub trait WorkflowRepository: Send + Sync {
    async fn create(
        &self,
        name: String,
        trigger: WorkflowTrigger,
        is_active: bool,
        actions: Vec<WorkflowAction>,
    ) -> Result<WorkflowModel>;

    async fn find_all(&self, page: u64, page_size: u64) -> Result<(Vec<WorkflowModel>, u64)>;

    async fn find_by_id(&self, id: &str) -> Result<WorkflowModel>;

    async fn find_active_by_trigger(
        &self,
        entity_type: &str,
        event: WorkflowEvent,
    ) -> Result<Vec<WorkflowModel>>;

    async fn update(
        &self,
        id: &str,
        name: String,
        trigger: WorkflowTrigger,
        is_active: bool,
        actions: Vec<WorkflowAction>,
    ) -> Result<WorkflowModel>;

    async fn set_active(&self, id: &str, is_active: bool) -> Result<WorkflowModel>;

    async fn delete(&self, id: &str) -> Result<()>;

    async fn create_execution(
        &self,
        workflow_id: &str,
        payload: JsonValue,
    ) -> Result<WorkflowExecutionModel>;

    async fn update_execution_status(
        &self,
        id: &str,
        status: WorkflowExecutionStatus,
        error_message: Option<String>,
    ) -> Result<WorkflowExecutionModel>;

    async fn find_executions(
        &self,
        workflow_id: &str,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<WorkflowExecutionModel>, u64)>;
}
