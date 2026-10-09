use serde_json::Value as JsonValue;

use crate::{
    core::{
        application::dtos::{
            CreateWorkflowCommand, SetWorkflowActiveCommand, UpdateWorkflowCommand,
            WorkflowExecutionPaginationResponse, WorkflowPaginationQuery,
            WorkflowPaginationResponse,
        },
        domain::{
            constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
            entities::{
                WorkflowEvent, WorkflowExecutionModel, WorkflowExecutionStatus, WorkflowModel,
            },
            repositories::WorkflowRepository,
        },
    },
    error::{Error, Result},
};

#[derive(Clone)]
pub struct WorkflowUseCase<R>
where
    R: WorkflowRepository,
{
    repository: R,
}

impl<R> WorkflowUseCase<R>
where
    R: WorkflowRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, mut command: CreateWorkflowCommand) -> Result<WorkflowModel> {
        let name = command.name.trim().to_string();
        command.trigger.entity_type = command.trigger.entity_type.trim().to_string();
        validate_definition(&name, &command.trigger.entity_type)?;
        validate_actions(&command.actions)?;
        self.repository
            .create(
                name,
                command.trigger,
                command.is_active.unwrap_or(true),
                command.actions,
            )
            .await
    }

    pub async fn find_all(
        &self,
        query: WorkflowPaginationQuery,
    ) -> Result<WorkflowPaginationResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE).max(1);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, 100);
        let (data, total) = self.repository.find_all(page, page_size).await?;
        Ok(WorkflowPaginationResponse {
            data,
            total,
            page,
            page_size,
        })
    }

    pub async fn find_by_id(&self, id: &str) -> Result<WorkflowModel> {
        self.repository.find_by_id(id).await
    }

    pub async fn find_active_by_trigger(
        &self,
        entity_type: &str,
        event: WorkflowEvent,
    ) -> Result<Vec<WorkflowModel>> {
        self.repository
            .find_active_by_trigger(entity_type, event)
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        mut command: UpdateWorkflowCommand,
    ) -> Result<WorkflowModel> {
        let name = command.name.trim().to_string();
        command.trigger.entity_type = command.trigger.entity_type.trim().to_string();
        validate_definition(&name, &command.trigger.entity_type)?;
        validate_actions(&command.actions)?;
        self.repository
            .update(
                id,
                name,
                command.trigger,
                command.is_active,
                command.actions,
            )
            .await
    }

    pub async fn set_active(
        &self,
        id: &str,
        command: SetWorkflowActiveCommand,
    ) -> Result<WorkflowModel> {
        self.repository.set_active(id, command.is_active).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.repository.delete(id).await
    }

    pub async fn create_execution(
        &self,
        workflow_id: &str,
        payload: JsonValue,
    ) -> Result<WorkflowExecutionModel> {
        self.repository.create_execution(workflow_id, payload).await
    }

    pub async fn update_execution_status(
        &self,
        id: &str,
        status: WorkflowExecutionStatus,
        error_message: Option<String>,
    ) -> Result<WorkflowExecutionModel> {
        self.repository
            .update_execution_status(id, status, error_message)
            .await
    }

    pub async fn find_executions(
        &self,
        workflow_id: &str,
        query: WorkflowPaginationQuery,
    ) -> Result<WorkflowExecutionPaginationResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE).max(1);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, 100);
        let (data, total) = self
            .repository
            .find_executions(workflow_id, page, page_size)
            .await?;
        Ok(WorkflowExecutionPaginationResponse {
            data,
            total,
            page,
            page_size,
        })
    }
}

fn validate_definition(name: &str, entity_type: &str) -> Result<()> {
    let mut errors = Vec::new();
    if name.trim().is_empty() {
        errors.push(crate::core::domain::entities::ErrorMessageResponse {
            key: "name".to_string(),
            message: "Workflow name is required.".to_string(),
        });
    }
    if entity_type.trim().is_empty() {
        errors.push(crate::core::domain::entities::ErrorMessageResponse {
            key: "trigger.entity_type".to_string(),
            message: "Trigger entity type is required.".to_string(),
        });
    }
    if !errors.is_empty() {
        return Err(Error::BadRequest(
            crate::core::domain::entities::ErrorResponse {
                status: false,
                errors,
            },
        ));
    }
    Ok(())
}

fn validate_actions(actions: &[crate::core::domain::entities::WorkflowAction]) -> Result<()> {
    use crate::core::domain::entities::WorkflowAction;

    let errors = actions
        .iter()
        .enumerate()
        .filter_map(|(index, action)| {
            let invalid_field = match action {
                WorkflowAction::Webhook { url } => {
                    let parsed = url.trim().parse::<axum::http::Uri>();
                    match parsed {
                        Ok(uri)
                            if matches!(uri.scheme_str(), Some("http" | "https"))
                                && uri
                                    .authority()
                                    .is_some_and(|authority| !authority.host().is_empty()) =>
                        {
                            None
                        }
                        _ => Some("url"),
                    }
                }
                WorkflowAction::Email {
                    template_id,
                    recipient,
                } => {
                    if template_id.trim().is_empty() {
                        Some("template_id")
                    } else if !email_address::EmailAddress::is_valid(recipient.trim()) {
                        Some("recipient")
                    } else {
                        None
                    }
                }
            }?;

            Some(crate::core::domain::entities::ErrorMessageResponse {
                key: format!("actions[{index}].{invalid_field}"),
                message: "Workflow action configuration is required and must be valid.".to_string(),
            })
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(Error::BadRequest(
            crate::core::domain::entities::ErrorResponse {
                status: false,
                errors,
            },
        ));
    }
    Ok(())
}
