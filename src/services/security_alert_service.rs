use std::collections::BTreeMap;
use crate::error::Result;
use crate::models::security_alert_model::{
    AlertSeverity, AlertType, CreateSecurityAlertModel, SecurityAlertModel, 
    SecurityAlertPaginationModel, UpdateSecurityAlertModel
};
use crate::repositories::security_alert_repository::SecurityAlertRepository;
use surrealdb::kvs::Datastore;
use surrealdb::dbs::Session;
use surrealdb::sql::Value;

#[derive(Clone)]
pub struct SecurityAlertService {
    security_alert_repository: SecurityAlertRepository,
}

impl SecurityAlertService {
    pub fn new(security_alert_repository: SecurityAlertRepository) -> Self {
        Self {
            security_alert_repository,
        }
    }

    /// Create a new security alert
    pub async fn create_alert(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_security_alert_model: CreateSecurityAlertModel,
    ) -> Result<SecurityAlertModel> {
        self.security_alert_repository
            .create(datastore, database_session, createable_security_alert_model)
            .await
    }

    /// Create a security alert with auto-generated ID
    pub async fn create_alert_auto_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        source: String,
        affected_resource: Option<String>,
        metadata: Option<BTreeMap<String, Value>>,
    ) -> Result<SecurityAlertModel> {
        let create_model = CreateSecurityAlertModel {
            alert_id: format!("alert_{}", surrealdb::sql::Uuid::new_v4()),
            alert_type,
            severity,
            message,
            source,
            affected_resource,
            metadata,
        };

        self.create_alert(datastore, database_session, create_model).await
    }

    /// Get security alert by ID
    pub async fn get_alert_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<SecurityAlertModel> {
        self.security_alert_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    /// Get unresolved alerts by severity
    pub async fn get_unresolved_alerts_by_severity(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        severity: AlertSeverity,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        self.security_alert_repository
            .find_unresolved_by_severity(datastore, database_session, severity, page, per_page)
            .await
    }

    /// Get alerts by type
    pub async fn get_alerts_by_type(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        alert_type: AlertType,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        self.security_alert_repository
            .find_by_alert_type(datastore, database_session, alert_type, page, per_page)
            .await
    }

    /// Get alerts by source
    pub async fn get_alerts_by_source(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        source: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        self.security_alert_repository
            .find_by_source(datastore, database_session, source, page, per_page)
            .await
    }

    /// Resolve a security alert
    pub async fn resolve_alert(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        resolved_by: &str,
    ) -> Result<SecurityAlertModel> {
        self.security_alert_repository
            .resolve(datastore, database_session, id, resolved_by)
            .await
    }

    /// Update security alert
    pub async fn update_alert(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        updateable_security_alert_model: UpdateSecurityAlertModel,
    ) -> Result<SecurityAlertModel> {
        self.security_alert_repository
            .update(datastore, database_session, id, updateable_security_alert_model)
            .await
    }

    /// Get paginated security alerts
    pub async fn get_alerts_paginated(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        self.security_alert_repository
            .paginate(datastore, database_session, page, per_page)
            .await
    }

    /// Get critical unresolved alerts
    pub async fn get_critical_unresolved_alerts(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<Vec<SecurityAlertModel>> {
        self.security_alert_repository
            .get_critical_unresolved(datastore, database_session)
            .await
    }

    /// Delete security alert
    pub async fn delete_alert(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<()> {
        self.security_alert_repository
            .delete(datastore, database_session, id)
            .await
    }

    /// Create alert based on security event
    pub async fn create_alert_from_event(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        event: SecurityAlertEvent,
    ) -> Result<SecurityAlertModel> {
        let (alert_type, severity, message) = self.classify_security_event(&event);
        
        let mut metadata = BTreeMap::new();
        metadata.insert("event_details".to_string(), Value::from(event.details.clone()));
        metadata.insert("timestamp".to_string(), Value::from(chrono::Utc::now().to_rfc3339()));
        
        if let Some(user_agent) = &event.user_agent {
            metadata.insert("user_agent".to_string(), Value::from(user_agent.clone()));
        }
        
        if let Some(endpoint) = &event.endpoint {
            metadata.insert("endpoint".to_string(), Value::from(endpoint.clone()));
        }

        self.create_alert_auto_id(
            datastore,
            database_session,
            alert_type,
            severity,
            message,
            event.source,
            event.affected_resource,
            Some(metadata),
        ).await
    }

    /// Classify security event and determine alert type, severity, and message
    fn classify_security_event(&self, event: &SecurityAlertEvent) -> (AlertType, AlertSeverity, String) {
        match event.event_type.as_str() {
            "multiple_failed_logins" => (
                AlertType::AuthenticationFailure,
                AlertSeverity::Medium,
                format!("Multiple failed login attempts detected from {}", event.source)
            ),
            "sql_injection_attempt" => (
                AlertType::InjectionAttempt,
                AlertSeverity::High,
                format!("SQL injection attempt detected from {}", event.source)
            ),
            "rate_limit_exceeded" => (
                AlertType::RateLimitExceeded,
                AlertSeverity::Low,
                format!("Rate limit exceeded for {}", event.source)
            ),
            "brute_force_attack" => (
                AlertType::BruteForceAttack,
                AlertSeverity::High,
                format!("Brute force attack detected from {}", event.source)
            ),
            "privilege_escalation" => (
                AlertType::PrivilegeEscalation,
                AlertSeverity::Critical,
                format!("Privilege escalation attempt detected from {}", event.source)
            ),
            "data_breach_attempt" => (
                AlertType::DataBreachAttempt,
                AlertSeverity::Critical,
                format!("Data breach attempt detected from {}", event.source)
            ),
            "unauthorized_access" => (
                AlertType::UnauthorizedAccess,
                AlertSeverity::High,
                format!("Unauthorized access attempt from {}", event.source)
            ),
            "malformed_request" => (
                AlertType::MalformedRequest,
                AlertSeverity::Low,
                format!("Malformed request detected from {}", event.source)
            ),
            "session_hijacking" => (
                AlertType::SessionHijacking,
                AlertSeverity::Critical,
                format!("Session hijacking attempt detected from {}", event.source)
            ),
            _ => (
                AlertType::SuspiciousActivity,
                AlertSeverity::Medium,
                format!("Suspicious activity detected from {}: {}", event.source, event.details)
            ),
        }
    }

    /// Get alert statistics
    pub async fn get_alert_statistics(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<AlertStatistics> {
        // Get critical unresolved alerts
        let critical_alerts = self.get_critical_unresolved_alerts(datastore, database_session).await?;
        
        // Get recent alerts (last 100)
        let recent_alerts = self.get_alerts_paginated(datastore, database_session, 1, 100).await?;
        
        let mut stats = AlertStatistics::default();
        stats.total_critical_unresolved = critical_alerts.len() as i64;
        
        // Calculate statistics from recent alerts
        for alert in recent_alerts.data {
            match alert.severity {
                AlertSeverity::Low => stats.total_low += 1,
                AlertSeverity::Medium => stats.total_medium += 1,
                AlertSeverity::High => stats.total_high += 1,
                AlertSeverity::Critical => stats.total_critical += 1,
            }
            
            if !alert.resolved {
                stats.total_unresolved += 1;
            }
        }
        
        stats.total_alerts = recent_alerts.pagination.total;
        Ok(stats)
    }
}

#[derive(Debug, Clone)]
pub struct SecurityAlertEvent {
    pub event_type: String,
    pub source: String,
    pub details: String,
    pub affected_resource: Option<String>,
    pub user_agent: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AlertStatistics {
    pub total_alerts: i64,
    pub total_unresolved: i64,
    pub total_critical_unresolved: i64,
    pub total_low: i64,
    pub total_medium: i64,
    pub total_high: i64,
    pub total_critical: i64,
}
