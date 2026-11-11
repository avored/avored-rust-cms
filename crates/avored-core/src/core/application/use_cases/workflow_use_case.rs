use serde_json::Value as JsonValue;

use crate::{
    core::{
        application::dtos::{
            CreateWorkflowCommand, PaginatedResponse, Pagination, SetWorkflowActiveCommand,
            UpdateWorkflowCommand, WorkflowExecutionPaginationResponse, WorkflowPaginationQuery,
            WorkflowPaginationResponse,
        },
        domain::{
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
        let page = query.current_page();
        let page_size = query.per_page();
        let (data, total) = self.repository.find_all(page, page_size).await?;
        Ok(PaginatedResponse {
            data,
            pagination: Pagination::new(total, page_size, page),
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
        let page = query.current_page();
        let page_size = query.per_page();
        let (data, total) = self
            .repository
            .find_executions(workflow_id, page, page_size)
            .await?;
        Ok(PaginatedResponse {
            data,
            pagination: Pagination::new(total, page_size, page),
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
            crate::core::domain::entities::ApiErrorResponse {
                status: false,
                errors,
                request_id: None,
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
                            if uri.scheme_str() == Some("https")
                                && uri.authority().is_some_and(|authority| {
                                    !authority.host().is_empty()
                                        && !authority.as_str().contains('@')
                                }) =>
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
            crate::core::domain::entities::ApiErrorResponse {
                status: false,
                errors,
                request_id: None,
            },
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        application::dtos::CreateWorkflowCommand,
        domain::entities::{
            WorkflowAction, WorkflowEvent, WorkflowExecutionModel, WorkflowExecutionStatus,
            WorkflowModel, WorkflowTrigger,
        },
    };
    use async_trait::async_trait;
    use serde_json::json;
    use surrealdb::types::Datetime;

    #[derive(Clone, Default)]
    struct MockWorkflowRepository;

    #[async_trait]
    impl WorkflowRepository for MockWorkflowRepository {
        async fn create(
            &self,
            name: String,
            trigger: WorkflowTrigger,
            is_active: bool,
            actions: Vec<WorkflowAction>,
        ) -> Result<WorkflowModel> {
            Ok(WorkflowModel {
                id: "workflow-1".to_string(),
                name,
                trigger,
                is_active,
                actions,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn find_all(&self, _page: u64, _page_size: u64) -> Result<(Vec<WorkflowModel>, u64)> {
            Ok((vec![], 0))
        }

        async fn find_by_id(&self, id: &str) -> Result<WorkflowModel> {
            Ok(WorkflowModel {
                id: id.to_string(),
                name: "Example workflow".to_string(),
                trigger: WorkflowTrigger {
                    entity_type: "user".to_string(),
                    event: WorkflowEvent::Created,
                },
                is_active: true,
                actions: vec![WorkflowAction::Webhook {
                    url: "https://example.com/webhook".to_string(),
                }],
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn find_active_by_trigger(
            &self,
            _entity_type: &str,
            _event: WorkflowEvent,
        ) -> Result<Vec<WorkflowModel>> {
            Ok(vec![])
        }

        async fn update(
            &self,
            id: &str,
            name: String,
            trigger: WorkflowTrigger,
            is_active: bool,
            actions: Vec<WorkflowAction>,
        ) -> Result<WorkflowModel> {
            Ok(WorkflowModel {
                id: id.to_string(),
                name,
                trigger,
                is_active,
                actions,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn set_active(&self, id: &str, is_active: bool) -> Result<WorkflowModel> {
            Ok(WorkflowModel {
                id: id.to_string(),
                name: "Example workflow".to_string(),
                trigger: WorkflowTrigger {
                    entity_type: "user".to_string(),
                    event: WorkflowEvent::Created,
                },
                is_active,
                actions: vec![WorkflowAction::Webhook {
                    url: "https://example.com/webhook".to_string(),
                }],
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn delete(&self, _id: &str) -> Result<()> {
            Ok(())
        }

        async fn create_execution(
            &self,
            workflow_id: &str,
            payload: serde_json::Value,
        ) -> Result<WorkflowExecutionModel> {
            Ok(WorkflowExecutionModel {
                id: "execution-1".to_string(),
                workflow_id: workflow_id.to_string(),
                status: WorkflowExecutionStatus::Running,
                payload,
                error_message: None,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn update_execution_status(
            &self,
            id: &str,
            status: WorkflowExecutionStatus,
            error_message: Option<String>,
        ) -> Result<WorkflowExecutionModel> {
            Ok(WorkflowExecutionModel {
                id: id.to_string(),
                workflow_id: "workflow-1".to_string(),
                status,
                payload: json!({}),
                error_message,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
        }

        async fn find_executions(
            &self,
            _workflow_id: &str,
            _page: u64,
            _page_size: u64,
        ) -> Result<(Vec<WorkflowExecutionModel>, u64)> {
            Ok((vec![], 0))
        }
    }

    #[tokio::test]
    async fn create_rejects_empty_name_or_entity_type() {
        let use_case = WorkflowUseCase::new(MockWorkflowRepository);
        let result = use_case
            .create(CreateWorkflowCommand {
                name: "   ".to_string(),
                trigger: WorkflowTrigger {
                    entity_type: "   ".to_string(),
                    event: WorkflowEvent::Created,
                },
                is_active: Some(true),
                actions: vec![WorkflowAction::Webhook {
                    url: "https://example.com/webhook".to_string(),
                }],
            })
            .await;

        assert!(matches!(result, Err(Error::BadRequest(_))));
    }

    #[tokio::test]
    async fn create_accepts_valid_workflow() {
        let use_case = WorkflowUseCase::new(MockWorkflowRepository);
        let result = use_case
            .create(CreateWorkflowCommand {
                name: "User onboarding".to_string(),
                trigger: WorkflowTrigger {
                    entity_type: "user".to_string(),
                    event: WorkflowEvent::Created,
                },
                is_active: Some(true),
                actions: vec![WorkflowAction::Webhook {
                    url: "https://example.com/webhook".to_string(),
                }],
            })
            .await
            .expect("valid workflow should be accepted");

        assert_eq!(result.name, "User onboarding");
        assert!(result.is_active);
        assert_eq!(result.trigger.entity_type, "user");
    }
}
