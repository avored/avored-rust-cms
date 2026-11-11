use serde::{Deserialize, Serialize};

use crate::core::application::dtos::pagination_dto::PaginatedResponse;
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

pub type WorkflowPaginationQuery = crate::core::application::dtos::pagination_dto::PaginationQuery;
pub type WorkflowPaginationResponse = PaginatedResponse<WorkflowModel>;
pub type WorkflowExecutionPaginationResponse = PaginatedResponse<WorkflowExecutionModel>;
