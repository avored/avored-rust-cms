use crate::api::proto::security_audit::{
    AlertSeverity as GrpcAlertSeverity, AlertStatistics as GrpcAlertStatistics,
    AlertType as GrpcAlertType, CreateSecurityAlertAutoIdRequest,
    CreateSecurityAlertAutoIdResponse, CreateSecurityAlertRequest, CreateSecurityAlertResponse,
    DeleteSecurityAlertResponse, GetAlertStatisticsResponse, GetAlertsBySourceResponse,
    GetAlertsByTypeResponse, GetCriticalUnresolvedAlertsResponse, GetSecurityAlertResponse,
    GetSecurityAlertsPaginatedResponse, GetUnresolvedAlertsBySeverityResponse,
    Pagination as GrpcPagination, ResolveSecurityAlertResponse,
    SecurityAlertModel as GrpcSecurityAlertModel,
    SecurityAlertPaginationModel as GrpcSecurityAlertPaginationModel,
};
use crate::error::Result;
use crate::models::security_alert_model::{
    AlertSeverity, AlertType, CreateSecurityAlertModel, SecurityAlertModel,
    SecurityAlertPaginationModel
};
use crate::providers::avored_database_provider::DB;
use crate::repositories::security_alert_repository::SecurityAlertRepository;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

/// Security alert service
#[derive(Clone)]
pub struct SecurityAlertService {
    security_alert_repository: SecurityAlertRepository,
}

impl SecurityAlertService {
    /// initialize security alert service
    #[must_use] pub const fn new(security_alert_repository: SecurityAlertRepository) -> Self {
        Self {
            security_alert_repository,
        }
    }

    /// Create a new security alert
    pub async fn create_alert(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_security_alert_model: CreateSecurityAlertModel,
    ) -> Result<SecurityAlertModel> {
        self.security_alert_repository
            .create(datastore, database_session, creatable_security_alert_model)
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

        self.create_alert(datastore, database_session, create_model)
            .await
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

    // /// Update security alert
    // pub async fn update_alert(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     id: &str,
    //     updateable_security_alert_model: UpdateSecurityAlertModel,
    // ) -> Result<SecurityAlertModel> {
    //     self.security_alert_repository
    //         .update(
    //             datastore,
    //             database_session,
    //             id,
    //             updateable_security_alert_model,
    //         )
    //         .await
    // }

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

    // /// Create alert based on security event
    // pub async fn create_alert_from_event(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     event: SecurityAlertEvent,
    // ) -> Result<SecurityAlertModel> {
    //     let (alert_type, severity, message) = self.classify_security_event(&event);

    //     let mut metadata = BTreeMap::new();
    //     metadata.insert(
    //         "event_details".to_string(),
    //         Value::from(event.details.clone()),
    //     );
    //     metadata.insert(
    //         "timestamp".to_string(),
    //         Value::from(chrono::Utc::now().to_rfc3339()),
    //     );

    //     if let Some(user_agent) = &event.user_agent {
    //         metadata.insert("user_agent".to_string(), Value::from(user_agent.clone()));
    //     }

    //     if let Some(endpoint) = &event.endpoint {
    //         metadata.insert("endpoint".to_string(), Value::from(endpoint.clone()));
    //     }

    //     self.create_alert_auto_id(
    //         datastore,
    //         database_session,
    //         alert_type,
    //         severity,
    //         message,
    //         event.source,
    //         event.affected_resource,
    //         Some(metadata),
    //     )
    //     .await
    // }

    // /// Classify security event and determine alert type, severity, and message
    // fn classify_security_event(
    //     &self,
    //     event: &SecurityAlertEvent,
    // ) -> (AlertType, AlertSeverity, String) {
    //     match event.event_type.as_str() {
    //         "multiple_failed_logins" => (
    //             AlertType::AuthenticationFailure,
    //             AlertSeverity::Medium,
    //             format!(
    //                 "Multiple failed login attempts detected from {}",
    //                 event.source
    //             ),
    //         ),
    //         "sql_injection_attempt" => (
    //             AlertType::InjectionAttempt,
    //             AlertSeverity::High,
    //             format!("SQL injection attempt detected from {}", event.source),
    //         ),
    //         "rate_limit_exceeded" => (
    //             AlertType::RateLimitExceeded,
    //             AlertSeverity::Low,
    //             format!("Rate limit exceeded for {}", event.source),
    //         ),
    //         "brute_force_attack" => (
    //             AlertType::BruteForceAttack,
    //             AlertSeverity::High,
    //             format!("Brute force attack detected from {}", event.source),
    //         ),
    //         "privilege_escalation" => (
    //             AlertType::PrivilegeEscalation,
    //             AlertSeverity::Critical,
    //             format!(
    //                 "Privilege escalation attempt detected from {}",
    //                 event.source
    //             ),
    //         ),
    //         "data_breach_attempt" => (
    //             AlertType::DataBreachAttempt,
    //             AlertSeverity::Critical,
    //             format!("Data breach attempt detected from {}", event.source),
    //         ),
    //         "unauthorized_access" => (
    //             AlertType::UnauthorizedAccess,
    //             AlertSeverity::High,
    //             format!("Unauthorized access attempt from {}", event.source),
    //         ),
    //         "malformed_request" => (
    //             AlertType::MalformedRequest,
    //             AlertSeverity::Low,
    //             format!("Malformed request detected from {}", event.source),
    //         ),
    //         "session_hijacking" => (
    //             AlertType::SessionHijacking,
    //             AlertSeverity::Critical,
    //             format!("Session hijacking attempt detected from {}", event.source),
    //         ),
    //         _ => (
    //             AlertType::SuspiciousActivity,
    //             AlertSeverity::Medium,
    //             format!(
    //                 "Suspicious activity detected from {}: {}",
    //                 event.source, event.details
    //             ),
    //         ),
    //     }
    // }

    /// Get alert statistics
    pub async fn get_alert_statistics(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<AlertStatistics> {
        // Get critical unresolved alerts
        let critical_alerts = self
            .get_critical_unresolved_alerts(datastore, database_session)
            .await?;

        // Get recent alerts (last 100)
        let recent_alerts = self
            .get_alerts_paginated(datastore, database_session, 1, 100)
            .await?;

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

// /// security alert event
// #[derive(Debug, Clone)]
// pub struct SecurityAlertEvent {
//     /// event type
//     pub event_type: String,
//     /// source
//     pub source: String,
//     /// details
//     pub details: String,
//     /// affected resource
//     pub affected_resource: Option<String>,
//     /// user agent
//     pub user_agent: Option<String>,
//     /// end point
//     pub endpoint: Option<String>,
// }

/// alert statistics
#[derive(Debug, Clone, Default)]
pub struct AlertStatistics {
    /// total alerts
    pub total_alerts: i64,
    /// total unresolved
    pub total_unresolved: i64,
    /// total critical unresolved
    pub total_critical_unresolved: i64,
    /// total low
    pub total_low: i64,
    /// total medium
    pub total_medium: i64,
    /// total high
    pub total_high: i64,
    /// total critical
    pub total_critical: i64,
}

// gRPC Integration Methods
impl SecurityAlertService {
    /// Create alert from gRPC request
    pub async fn create_alert_from_grpc(
        &self,
        request: CreateSecurityAlertRequest,
        (datastore, database_session): &DB,
    ) -> Result<CreateSecurityAlertResponse> {
        let alert_data = request
            .alert
            .ok_or_else(|| crate::error::Error::Generic("Missing alert data".to_string()))?;

        let createable_model = CreateSecurityAlertModel {
            alert_id: alert_data.alert_id,
            alert_type: AlertType::from_grpc_alert_type(
                GrpcAlertType::try_from(alert_data.alert_type)
                    .unwrap_or(GrpcAlertType::Unspecified),
            ),
            severity: AlertSeverity::from_grpc_alert_severity(
                GrpcAlertSeverity::try_from(alert_data.severity)
                    .unwrap_or(GrpcAlertSeverity::Unspecified),
            ),
            message: alert_data.message,
            source: alert_data.source,
            affected_resource: alert_data.affected_resource,
            metadata: if let Some(json_str) = alert_data.metadata_json {
                serde_json::from_str(&json_str).ok()
            } else {
                None
            },
        };

        let model = self
            .create_alert(datastore, database_session, createable_model)
            .await?;
        let grpc_model: GrpcSecurityAlertModel = model.try_into()?;

        Ok(CreateSecurityAlertResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Create alert with auto-generated ID from gRPC request
    pub async fn create_alert_auto_id_from_grpc(
        &self,
        request: CreateSecurityAlertAutoIdRequest,
        (datastore, database_session): &DB,
    ) -> Result<CreateSecurityAlertAutoIdResponse> {
        let model = self
            .create_alert_auto_id(
                datastore,
                database_session,
                AlertType::from_grpc_alert_type(
                    GrpcAlertType::try_from(request.alert_type)
                        .unwrap_or(GrpcAlertType::Unspecified),
                ),
                AlertSeverity::from_grpc_alert_severity(
                    GrpcAlertSeverity::try_from(request.severity)
                        .unwrap_or(GrpcAlertSeverity::Unspecified),
                ),
                request.message,
                request.source,
                request.affected_resource,
                if let Some(json_str) = request.metadata_json {
                    serde_json::from_str(&json_str).ok()
                } else {
                    None
                },
            )
            .await?;

        let grpc_model: GrpcSecurityAlertModel = model.try_into()?;

        Ok(CreateSecurityAlertAutoIdResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Get alert by ID for gRPC
    pub async fn get_alert_by_id_grpc(
        &self,
        id: String,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAlertResponse> {
        let model = self
            .get_alert_by_id(datastore, database_session, &id)
            .await?;
        let grpc_model: GrpcSecurityAlertModel = model.try_into()?;

        Ok(GetSecurityAlertResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Get unresolved alerts by severity for gRPC
    pub async fn get_unresolved_alerts_by_severity_grpc(
        &self,
        severity: i32,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetUnresolvedAlertsBySeverityResponse> {
        let alert_severity = AlertSeverity::from_grpc_alert_severity(
            GrpcAlertSeverity::try_from(severity).unwrap_or(GrpcAlertSeverity::Unspecified),
        );

        let pagination_model = self
            .get_unresolved_alerts_by_severity(
                datastore,
                database_session,
                alert_severity,
                page,
                per_page,
            )
            .await?;

        let grpc_pagination = convert_alert_pagination_to_grpc(pagination_model)?;

        Ok(GetUnresolvedAlertsBySeverityResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Get alerts by type for gRPC
    pub async fn get_alerts_by_type_grpc(
        &self,
        alert_type: i32,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetAlertsByTypeResponse> {
        let alert_type = AlertType::from_grpc_alert_type(
            GrpcAlertType::try_from(alert_type).unwrap_or(GrpcAlertType::Unspecified),
        );

        let pagination_model = self
            .get_alerts_by_type(datastore, database_session, alert_type, page, per_page)
            .await?;

        let grpc_pagination = convert_alert_pagination_to_grpc(pagination_model)?;

        Ok(GetAlertsByTypeResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Get alerts by source for gRPC
    pub async fn get_alerts_by_source_grpc(
        &self,
        source: String,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetAlertsBySourceResponse> {
        let pagination_model = self
            .get_alerts_by_source(datastore, database_session, &source, page, per_page)
            .await?;

        let grpc_pagination = convert_alert_pagination_to_grpc(pagination_model)?;

        Ok(GetAlertsBySourceResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Resolve alert for gRPC
    pub async fn resolve_alert_grpc(
        &self,
        id: String,
        resolved_by: String,
        (datastore, database_session): &DB,
    ) -> Result<ResolveSecurityAlertResponse> {
        let model = self
            .resolve_alert(datastore, database_session, &id, &resolved_by)
            .await?;
        let grpc_model: GrpcSecurityAlertModel = model.try_into()?;

        Ok(ResolveSecurityAlertResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Get paginated alerts for gRPC
    pub async fn get_alerts_paginated_grpc(
        &self,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAlertsPaginatedResponse> {
        let pagination_model = self
            .get_alerts_paginated(datastore, database_session, page, per_page)
            .await?;

        let grpc_pagination = convert_alert_pagination_to_grpc(pagination_model)?;

        Ok(GetSecurityAlertsPaginatedResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Get critical unresolved alerts for gRPC
    pub async fn get_critical_unresolved_alerts_grpc(
        &self,
        (datastore, database_session): &DB,
    ) -> Result<GetCriticalUnresolvedAlertsResponse> {
        let models = self
            .get_critical_unresolved_alerts(datastore, database_session)
            .await?;
        let mut grpc_models = Vec::new();

        for model in models {
            let grpc_model: GrpcSecurityAlertModel = model.try_into()?;
            grpc_models.push(grpc_model);
        }

        Ok(GetCriticalUnresolvedAlertsResponse {
            status: true,
            data: grpc_models,
        })
    }

    /// Delete alert for gRPC
    pub async fn delete_alert_grpc(
        &self,
        id: String,
        (datastore, database_session): &DB,
    ) -> Result<DeleteSecurityAlertResponse> {
        self.delete_alert(datastore, database_session, &id).await?;

        Ok(DeleteSecurityAlertResponse { status: true })
    }

    /// Get alert statistics for gRPC
    pub async fn get_alert_statistics_grpc(
        &self,
        (datastore, database_session): &DB,
    ) -> Result<GetAlertStatisticsResponse> {
        let stats = self
            .get_alert_statistics(datastore, database_session)
            .await?;
        let grpc_stats = GrpcAlertStatistics {
            total_alerts: stats.total_alerts,
            total_unresolved: stats.total_unresolved,
            total_critical_unresolved: stats.total_critical_unresolved,
            total_low: stats.total_low,
            total_medium: stats.total_medium,
            total_high: stats.total_high,
            total_critical: stats.total_critical,
        };

        Ok(GetAlertStatisticsResponse {
            status: true,
            data: Some(grpc_stats),
        })
    }
}

/// Helper function to convert alert pagination model to gRPC
fn convert_alert_pagination_to_grpc(
    pagination_model: SecurityAlertPaginationModel,
) -> Result<GrpcSecurityAlertPaginationModel> {
    let mut grpc_data = Vec::new();
    for model in pagination_model.data {
        let grpc_model: GrpcSecurityAlertModel = model.try_into()?;
        grpc_data.push(grpc_model);
    }

    let grpc_pagination = GrpcPagination {
        total: pagination_model.pagination.total,
        per_page: pagination_model.pagination.per_page,
        current_page: pagination_model.pagination.current_page,
        from: pagination_model.pagination.from,
        to: pagination_model.pagination.to,
        has_next_page: pagination_model.pagination.has_next_page,
        has_previous_page: pagination_model.pagination.has_previous_page,
        next_page_number: pagination_model.pagination.next_page_number,
        previous_page_number: pagination_model.pagination.previous_page_number,
    };

    Ok(GrpcSecurityAlertPaginationModel {
        data: grpc_data,
        pagination: Some(grpc_pagination),
    })
}
