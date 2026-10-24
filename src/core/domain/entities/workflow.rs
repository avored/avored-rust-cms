use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub entity_type: String,
    pub event: WorkflowEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowEvent {
    Created,
    Updated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowAction {
    Webhook {
        url: String,
    },
    Email {
        template_id: String,
        recipient: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowModel {
    pub id: String,
    pub name: String,
    pub trigger: WorkflowTrigger,
    pub is_active: bool,
    pub actions: Vec<WorkflowAction>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowExecutionStatus {
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowExecutionModel {
    pub id: String,
    pub workflow_id: String,
    pub status: WorkflowExecutionStatus,
    pub payload: serde_json::Value,
    pub error_message: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use super::{WorkflowAction, WorkflowEvent, WorkflowExecutionStatus};

    #[test]
    fn workflow_events_use_stable_snake_case_names() -> Result<()> {
        assert_eq!(
            serde_json::to_value(WorkflowEvent::Created)?,
            serde_json::json!("created")
        );
        assert_eq!(
            serde_json::to_value(WorkflowEvent::Updated)?,
            serde_json::json!("updated")
        );

        Ok(())
    }

    #[test]
    fn workflow_actions_use_a_discriminated_ordered_action_shape() -> Result<()> {
        let action = WorkflowAction::Email {
            template_id: "welcome".to_string(),
            recipient: "admin@example.test".to_string(),
        };

        let encoded = serde_json::to_value(&action)?;
        assert_eq!(
            encoded,
            serde_json::json!({
                "type": "email",
                "template_id": "welcome",
                "recipient": "admin@example.test"
            })
        );
        assert_eq!(
            serde_json::from_value::<WorkflowAction>(encoded)?,
            action
        );

        Ok(())
    }

    #[test]
    fn workflow_execution_status_names_are_stable() -> Result<()> {
        assert_eq!(
            serde_json::to_value(WorkflowExecutionStatus::Running)?,
            serde_json::json!("running")
        );
        assert_eq!(
            serde_json::to_value(WorkflowExecutionStatus::Success)?,
            serde_json::json!("success")
        );
        assert_eq!(
            serde_json::to_value(WorkflowExecutionStatus::Failed)?,
            serde_json::json!("failed")
        );
        Ok(())
    }
}
