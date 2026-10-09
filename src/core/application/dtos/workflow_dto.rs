use serde::{Deserialize, Serialize};

use crate::core::domain::entities::{
    WorkflowAction, WorkflowExecutionModel, WorkflowModel, WorkflowTrigger,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkflowCommand {
    pub name: String,
    pub trigger: WorkflowTrigger,
    pub is_active: Option<bool>,
    pub actions: Vec<WorkflowAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowCommand {
    pub name: String,
    pub trigger: WorkflowTrigger,
    pub is_active: bool,
    pub actions: Vec<WorkflowAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWorkflowActiveCommand {
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowPaginationQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPaginationResponse {
    pub data: Vec<WorkflowModel>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionPaginationResponse {
    pub data: Vec<WorkflowExecutionModel>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
