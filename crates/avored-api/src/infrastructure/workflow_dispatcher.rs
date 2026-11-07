use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use reqwest::{redirect::Policy, Client, Url};
use serde_json::Value as JsonValue;

use crate::{
    core::{
        application::use_cases::WorkflowUseCase,
        domain::entities::{WorkflowAction, WorkflowEvent, WorkflowExecutionStatus, WorkflowModel},
    },
    error::Result,
    infrastructure::persistence::WorkflowRepositoryImpl,
};

#[async_trait::async_trait]
pub trait WorkflowActionExecutor: Send + Sync {
    async fn execute(
        &self,
        action: &WorkflowAction,
        payload: &JsonValue,
    ) -> std::result::Result<(), String>;
}

#[derive(Clone, Default)]
pub struct HttpWebhookExecutor;

impl HttpWebhookExecutor {
    pub fn new() -> Self {
        Self
    }

    async fn post_json(&self, url: &str, payload: &JsonValue) -> std::result::Result<(), String> {
        let url = Url::parse(url).map_err(|error| format!("Invalid webhook URL: {error}"))?;
        if url.scheme() != "https" {
            return Err("Webhook URLs must use HTTPS".to_string());
        }
        if !url.username().is_empty() || url.password().is_some() {
            return Err("Webhook URLs must not contain credentials".to_string());
        }

        let host = url
            .host_str()
            .ok_or_else(|| "Webhook URL must include a host".to_string())?
            .to_string();
        let port = url
            .port_or_known_default()
            .ok_or_else(|| "Webhook URL must include a valid port".to_string())?;
        let addresses = resolve_public_addresses(&host, port).await?;

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(Policy::none())
            .resolve_to_addrs(&host, &addresses)
            .build()
            .map_err(|error| format!("Failed to create pinned webhook client: {error}"))?;
        let response = client
            .post(url)
            .json(payload)
            .send()
            .await
            .map_err(|error| format!("Webhook request failed: {error}"))?;

        if !response.status().is_success() {
            return Err(format!(
                "Webhook returned unsuccessful HTTP status {}",
                response.status()
            ));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl WorkflowActionExecutor for HttpWebhookExecutor {
    async fn execute(
        &self,
        action: &WorkflowAction,
        payload: &JsonValue,
    ) -> std::result::Result<(), String> {
        match action {
            WorkflowAction::Webhook { url } => self.post_json(url, payload).await,
            WorkflowAction::Email { .. } => Err(
                "Email actions are not executable until an email sender is configured".to_string(),
            ),
        }
    }
}

#[derive(Clone)]
pub struct WorkflowDispatcher {
    workflow_use_case: WorkflowUseCase<WorkflowRepositoryImpl>,
    action_executor: Arc<dyn WorkflowActionExecutor>,
}

impl WorkflowDispatcher {
    pub fn new(
        workflow_use_case: WorkflowUseCase<WorkflowRepositoryImpl>,
        action_executor: HttpWebhookExecutor,
    ) -> Self {
        Self::with_executor(workflow_use_case, action_executor)
    }

    pub fn with_executor<A>(
        workflow_use_case: WorkflowUseCase<WorkflowRepositoryImpl>,
        action_executor: A,
    ) -> Self
    where
        A: WorkflowActionExecutor + 'static,
    {
        Self {
            workflow_use_case,
            action_executor: Arc::new(action_executor),
        }
    }

    pub async fn dispatch(
        &self,
        entity_type: &str,
        event: WorkflowEvent,
        payload: JsonValue,
    ) -> Result<()> {
        let workflows = self
            .workflow_use_case
            .find_active_by_trigger(entity_type, event)
            .await?;

        for workflow in workflows {
            self.execute_workflow(&workflow, &payload).await;
        }
        Ok(())
    }

    async fn execute_workflow(&self, workflow: &WorkflowModel, payload: &JsonValue) {
        let execution = match self
            .workflow_use_case
            .create_execution(&workflow.id, payload.clone())
            .await
        {
            Ok(execution) => execution,
            Err(error) => {
                tracing::error!(
                    workflow_id = %workflow.id,
                    error = %error,
                    "Failed to create workflow execution record"
                );
                return;
            }
        };

        let mut failure = None;
        for action in &workflow.actions {
            if let Err(error) = self.action_executor.execute(action, payload).await {
                failure = Some(error);
                break;
            }
        }

        let (status, error_message) = match failure {
            Some(error) => (WorkflowExecutionStatus::Failed, Some(error)),
            None => (WorkflowExecutionStatus::Success, None),
        };

        if let Err(error) = self
            .workflow_use_case
            .update_execution_status(&execution.id, status, error_message.clone())
            .await
        {
            tracing::error!(
                workflow_id = %workflow.id,
                execution_id = %execution.id,
                error = %error,
                "Failed to persist workflow execution result"
            );
        }

        if let Some(error) = error_message {
            tracing::error!(
                workflow_id = %workflow.id,
                execution_id = %execution.id,
                error = %error,
                "Workflow execution failed"
            );
        }
    }
}

async fn resolve_public_addresses(
    host: &str,
    port: u16,
) -> std::result::Result<Vec<SocketAddr>, String> {
    if let Ok(address) = host.parse::<IpAddr>() {
        if !is_public_ip(address) {
            return Err("Webhook destination must resolve to a public IP address".to_string());
        }
        return Ok(vec![SocketAddr::new(address, port)]);
    }

    let addresses = tokio::net::lookup_host((host, port))
        .await
        .map_err(|error| format!("Failed to resolve webhook host: {error}"))?
        .collect::<Vec<_>>();

    if addresses.is_empty() || addresses.iter().any(|address| !is_public_ip(address.ip())) {
        return Err("Webhook destination must resolve only to public IP addresses".to_string());
    }
    Ok(addresses)
}

fn is_public_ip(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => is_public_ipv4(address),
        IpAddr::V6(address) => is_public_ipv6(address),
    }
}

fn is_public_ipv4(address: Ipv4Addr) -> bool {
    let [a, b, c, _] = address.octets();
    !(address.is_private()
        || address.is_loopback()
        || address.is_link_local()
        || address.is_broadcast()
        || address.is_unspecified()
        || address.is_multicast()
        || a == 0
        || (a == 100 && (64..=127).contains(&b))
        || (a == 192 && b == 0 && c == 0)
        || (a == 192 && b == 0 && c == 2)
        || (a == 192 && b == 88 && c == 99)
        || (a == 198 && (b == 18 || b == 19))
        || (a == 198 && b == 51 && c == 100)
        || (a == 203 && b == 0 && c == 113)
        || a >= 224)
}

fn is_public_ipv6(address: Ipv6Addr) -> bool {
    let segments = address.segments();
    (segments[0] & 0xe000) == 0x2000
        && !address.is_loopback()
        && !address.is_unspecified()
        && !address.is_multicast()
        && !address.is_unique_local()
        && !address.is_unicast_link_local()
        && segments[0] != 0x2002
        && !(segments[0] == 0x2001 && (segments[1] & 0xfe00) == 0x0000)
        && !(segments[0] == 0x2001 && segments[1] == 0x0db8)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::is_public_ip;
    use super::{HttpWebhookExecutor, WorkflowActionExecutor};
    use crate::core::domain::entities::WorkflowAction;
    use crate::error::Result;
    use serde_json::json;

    #[test]
    fn rejects_non_public_ipv4_addresses() {
        for address in [
            Ipv4Addr::LOCALHOST,
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv4Addr::new(100, 64, 0, 1),
            Ipv4Addr::new(169, 254, 1, 1),
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv4Addr::new(198, 51, 100, 20),
            Ipv4Addr::new(203, 0, 113, 20),
        ] {
            assert!(
                !is_public_ip(IpAddr::V4(address)),
                "{address} must be blocked"
            );
        }
        assert!(is_public_ip(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
    }

    #[test]
    fn rejects_non_public_ipv6_addresses() -> Result<()> {
        for address in [
            Ipv6Addr::LOCALHOST,
            "fc00::1".parse()?,
            "fe80::1".parse()?,
            "2001:db8::1".parse()?,
            "2002::1".parse()?,
        ] {
            assert!(
                !is_public_ip(IpAddr::V6(address)),
                "{address} must be blocked"
            );
        }
        assert!(is_public_ip(IpAddr::V6("2606:4700:4700::1111".parse()?)));

        Ok(())
    }

    #[tokio::test]
    async fn webhook_executor_rejects_http_and_private_destinations_before_connecting() {
        let executor = HttpWebhookExecutor::new();
        for url in ["http://1.1.1.1/hook", "https://127.0.0.1/hook"] {
            let result = executor
                .execute(
                    &WorkflowAction::Webhook {
                        url: url.to_string(),
                    },
                    &json!({"event": "test"}),
                )
                .await;
            assert!(result.is_err(), "{url} should be rejected");
        }
    }
}
