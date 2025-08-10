use crate::api::proto::security_audit::{
    CreateSecurityAuditRequest, CreateSecurityAuditResponse, DeleteSecurityAuditResponse,
    GetIpSecuritySummaryResponse, GetSecurityAuditResponse, GetSecurityAuditsByIpResponse,
    GetSecurityAuditsByUserResponse, GetSecurityAuditsPaginatedResponse, LogSecurityEventRequest,
    LogSecurityEventResponse, Pagination as GrpcPagination,
    SecurityAuditModel as GrpcSecurityAuditModel,
    SecurityAuditPaginationModel as GrpcSecurityAuditPaginationModel,
    SecurityEventType as GrpcSecurityEventType, SecuritySummary as GrpcSecuritySummary,
    UpdateSecurityAuditResponse,
};
use crate::error::Result;
use crate::models::security_audit_model::{
    CreateSecurityAuditModel, SecurityAuditModel, SecurityAuditPaginationModel,
    UpdateSecurityAuditModel,
};
use crate::providers::avored_database_provider::DB;
use crate::repositories::security_audit_repository::SecurityAuditRepository;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

/// Service for managing security audit records and events.
#[derive(Clone)]
pub struct SecurityAuditService {
    security_audit_repository: SecurityAuditRepository,
}

impl SecurityAuditService {

    /// Create a new instance of `SecurityAuditService`
    #[must_use] pub const fn new(security_audit_repository: SecurityAuditRepository) -> Self {
        Self {
            security_audit_repository,
        }
    }

    /// Create a new security audit record
    pub async fn create_audit(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_security_audit_model: CreateSecurityAuditModel,
    ) -> Result<SecurityAuditModel> {
        self.security_audit_repository
            .create(datastore, database_session, createable_security_audit_model)
            .await
    }

    /// Log a security event with automatic audit record creation
    pub async fn log_security_event(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        admin_user_id: Option<String>,
        session_id: Option<String>,
        ip_address: String,
        user_agent: Option<String>,
        endpoint: Option<String>,
        request_method: Option<String>,
        event_type: SecurityEventType,
        metadata: Option<BTreeMap<String, Value>>,
    ) -> Result<SecurityAuditModel> {
        let security_audit_id = format!("audit_{}", surrealdb::sql::Uuid::new_v4());

        let mut create_model = CreateSecurityAuditModel {
            security_audit_id,
            admin_user_id,
            session_id,
            ip_address,
            user_agent,
            endpoint,
            request_method,
            metadata,
            ..Default::default()
        };

        // Set counters based on event type
        match event_type {
            SecurityEventType::AuthenticationSuccess => {
                create_model.total_authentication_attempts = Some(1);
                create_model.failed_authentication_attempts = Some(0);
            }
            SecurityEventType::AuthenticationFailure => {
                create_model.total_authentication_attempts = Some(1);
                create_model.failed_authentication_attempts = Some(1);
            }
            SecurityEventType::InjectionAttempt => {
                create_model.blocked_injection_attempts = Some(1);
            }
            SecurityEventType::RateLimitExceeded => {
                create_model.rate_limited_requests = Some(1);
            }
            SecurityEventType::SuspiciousActivity => {
                create_model.suspicious_activities_detected = Some(1);
            }
            SecurityEventType::SecurityViolation => {
                create_model.security_violations = Some(1);
            }
        }

        // Calculate security health score
        let health_score = self.calculate_event_health_score(&event_type);
        create_model.security_health_score = Some(health_score);

        self.create_audit(datastore, database_session, create_model)
            .await
    }

    /// Get security audit by ID
    pub async fn get_audit_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<SecurityAuditModel> {
        self.security_audit_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    /// Get security audits by admin user ID with pagination
    pub async fn get_audits_by_admin_user(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        admin_user_id: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        self.security_audit_repository
            .find_by_admin_user_id(datastore, database_session, admin_user_id, page, per_page)
            .await
    }

    /// Get security audits by IP address with pagination
    pub async fn get_audits_by_ip_address(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        ip_address: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        self.security_audit_repository
            .find_by_ip_address(datastore, database_session, ip_address, page, per_page)
            .await
    }

    /// Update security audit record
    pub async fn update_audit(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        updateable_security_audit_model: UpdateSecurityAuditModel,
    ) -> Result<SecurityAuditModel> {
        self.security_audit_repository
            .update(
                datastore,
                database_session,
                id,
                updateable_security_audit_model,
            )
            .await
    }

    /// Increment security metrics for an existing audit record
    // pub async fn increment_security_metrics(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     id: &str,
    //     event_type: SecurityEventType,
    // ) -> Result<SecurityAuditModel> {
    //     // First get the current audit record
    //     let current_audit = self
    //         .get_audit_by_id(datastore, database_session, id)
    //         .await?;

    //     let mut update_model = UpdateSecurityAuditModel::default();

    //     // Increment appropriate counters
    //     match event_type {
    //         SecurityEventType::AuthenticationSuccess => {
    //             update_model.total_authentication_attempts =
    //                 Some(current_audit.total_authentication_attempts + 1);
    //         }
    //         SecurityEventType::AuthenticationFailure => {
    //             update_model.total_authentication_attempts =
    //                 Some(current_audit.total_authentication_attempts + 1);
    //             update_model.failed_authentication_attempts =
    //                 Some(current_audit.failed_authentication_attempts + 1);
    //         }
    //         SecurityEventType::InjectionAttempt => {
    //             update_model.blocked_injection_attempts =
    //                 Some(current_audit.blocked_injection_attempts + 1);
    //         }
    //         SecurityEventType::RateLimitExceeded => {
    //             update_model.rate_limited_requests = Some(current_audit.rate_limited_requests + 1);
    //         }
    //         SecurityEventType::SuspiciousActivity => {
    //             update_model.suspicious_activities_detected =
    //                 Some(current_audit.suspicious_activities_detected + 1);
    //         }
    //         SecurityEventType::SecurityViolation => {
    //             update_model.security_violations = Some(current_audit.security_violations + 1);
    //         }
    //     }

    //     // Recalculate health score based on updated metrics
    //     let updated_audit = SecurityAuditModel {
    //         total_authentication_attempts: update_model
    //             .total_authentication_attempts
    //             .unwrap_or(current_audit.total_authentication_attempts),
    //         failed_authentication_attempts: update_model
    //             .failed_authentication_attempts
    //             .unwrap_or(current_audit.failed_authentication_attempts),
    //         blocked_injection_attempts: update_model
    //             .blocked_injection_attempts
    //             .unwrap_or(current_audit.blocked_injection_attempts),
    //         rate_limited_requests: update_model
    //             .rate_limited_requests
    //             .unwrap_or(current_audit.rate_limited_requests),
    //         suspicious_activities_detected: update_model
    //             .suspicious_activities_detected
    //             .unwrap_or(current_audit.suspicious_activities_detected),
    //         security_violations: update_model
    //             .security_violations
    //             .unwrap_or(current_audit.security_violations),
    //         ..current_audit
    //     };

    //     let updated_score= updated_audit.calculate_health_score();
    //     update_model.security_health_score = Some(updated_score);

    //     self.update_audit(datastore, database_session, id, update_model)
    //         .await
    // }

    /// Get paginated security audits
    pub async fn get_audits_paginated(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        self.security_audit_repository
            .paginate(datastore, database_session, page, per_page)
            .await
    }

    /// Delete security audit record
    pub async fn delete_audit(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<()> {
        self.security_audit_repository
            .delete(datastore, database_session, id)
            .await
    }

    /// Calculate health score based on security event type
    const fn calculate_event_health_score(&self, event_type: &SecurityEventType) -> f64 {
        match event_type {
            SecurityEventType::AuthenticationSuccess => 100.0,
            SecurityEventType::AuthenticationFailure => 98.0,
            SecurityEventType::InjectionAttempt => 95.0,
            SecurityEventType::RateLimitExceeded => 97.0,
            SecurityEventType::SuspiciousActivity => 96.0,
            SecurityEventType::SecurityViolation => 94.0,
        }
    }

    /// Get security metrics summary for a specific IP address
    pub async fn get_ip_security_summary(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        ip_address: &str,
    ) -> Result<SecuritySummary> {
        let audits = self
            .get_audits_by_ip_address(datastore, database_session, ip_address, 1, 100)
            .await?;

        let mut summary = SecuritySummary::default();
        summary.ip_address = ip_address.to_string();

        for audit in audits.data {
            summary.total_authentication_attempts += audit.total_authentication_attempts;
            summary.failed_authentication_attempts += audit.failed_authentication_attempts;
            summary.blocked_injection_attempts += audit.blocked_injection_attempts;
            summary.rate_limited_requests += audit.rate_limited_requests;
            summary.suspicious_activities_detected += audit.suspicious_activities_detected;
            summary.security_violations += audit.security_violations;

            if audit.security_health_score < summary.lowest_health_score {
                summary.lowest_health_score = audit.security_health_score;
            }
        }

        summary.total_records = audits.pagination.total;
        Ok(summary)
    }
}


/// Security event types
#[derive(Debug, Clone)]
pub enum SecurityEventType {
    /// Authentication was successful
    AuthenticationSuccess,

    /// Authentication failed
    AuthenticationFailure,

    /// SQL or code injection attempt detected
    InjectionAttempt,

    /// Rate limit exceeded for requests
    RateLimitExceeded,

    /// Suspicious activity detected
    SuspiciousActivity,

    /// Security violation detected
    SecurityViolation,
}


/// Security summary for an IP address
#[derive(Debug, Clone)]
pub struct SecuritySummary {

    /// The IP address for which the summary is generated
    pub ip_address: String,

    /// Total number of security audit records for this IP
    pub total_records: i64,

    /// Total authentication attempts recorded
    pub total_authentication_attempts: i32,

    /// Total failed authentication attempts recorded
    pub failed_authentication_attempts: i32,

    /// Total blocked injection attempts recorded
    pub blocked_injection_attempts: i32,

    /// Total rate-limited requests recorded
    pub rate_limited_requests: i32,

    /// Total suspicious activities detected
    pub suspicious_activities_detected: i32,

    /// Total security violations recorded
    pub security_violations: i32,
    
    /// Lowest security health score recorded for this IP
    pub lowest_health_score: f64,
}

impl Default for SecuritySummary {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            total_records: 0,
            total_authentication_attempts: 0,
            failed_authentication_attempts: 0,
            blocked_injection_attempts: 0,
            rate_limited_requests: 0,
            suspicious_activities_detected: 0,
            security_violations: 0,
            lowest_health_score: 100.0,
        }
    }
}

// gRPC Integration Methods
impl SecurityAuditService {
    /// Create audit from gRPC request
    pub async fn create_audit_from_grpc(
        &self,
        request: CreateSecurityAuditRequest,
        (datastore, database_session): &DB,
    ) -> Result<CreateSecurityAuditResponse> {
        let audit_data = request
            .audit
            .ok_or_else(|| crate::error::Error::Generic("Missing audit data".to_string()))?;

        let createable_model = CreateSecurityAuditModel {
            security_audit_id: audit_data.security_audit_id,
            admin_user_id: audit_data.admin_user_id,
            session_id: audit_data.session_id,
            ip_address: audit_data.ip_address,
            user_agent: audit_data.user_agent,
            endpoint: audit_data.endpoint,
            request_method: audit_data.request_method,
            total_authentication_attempts: audit_data.total_authentication_attempts,
            failed_authentication_attempts: audit_data.failed_authentication_attempts,
            blocked_injection_attempts: audit_data.blocked_injection_attempts,
            rate_limited_requests: audit_data.rate_limited_requests,
            suspicious_activities_detected: audit_data.suspicious_activities_detected,
            security_violations: audit_data.security_violations,
            uptime_seconds: audit_data.uptime_seconds,
            security_health_score: audit_data.security_health_score,
            metadata: if let Some(json_str) = audit_data.metadata_json {
                serde_json::from_str(&json_str).ok()
            } else {
                None
            },
        };

        let model = self
            .create_audit(datastore, database_session, createable_model)
            .await?;
        let grpc_model: GrpcSecurityAuditModel = model.try_into()?;

        Ok(CreateSecurityAuditResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Log security event from gRPC request
    pub async fn log_security_event_from_grpc(
        &self,
        request: LogSecurityEventRequest,
        (datastore, database_session): &DB,
    ) -> Result<LogSecurityEventResponse> {
        let metadata = if let Some(json_str) = request.metadata_json {
            serde_json::from_str(&json_str).ok()
        } else {
            None
        };

        let model = self
            .log_security_event(
                datastore,
                database_session,
                request.admin_user_id,
                request.session_id,
                request.ip_address,
                request.user_agent,
                request.endpoint,
                request.request_method,
                convert_grpc_security_event_type(
                    GrpcSecurityEventType::try_from(request.event_type)
                        .unwrap_or(GrpcSecurityEventType::Unspecified),
                ),
                metadata,
            )
            .await?;

        let grpc_model: GrpcSecurityAuditModel = model.try_into()?;

        Ok(LogSecurityEventResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Get audit by ID for gRPC
    pub async fn get_audit_by_id_grpc(
        &self,
        id: String,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAuditResponse> {
        let model = self
            .get_audit_by_id(datastore, database_session, &id)
            .await?;
        let grpc_model: GrpcSecurityAuditModel = model.try_into()?;

        Ok(GetSecurityAuditResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Get audits by admin user for gRPC
    pub async fn get_audits_by_admin_user_grpc(
        &self,
        admin_user_id: String,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAuditsByUserResponse> {
        let pagination_model = self
            .get_audits_by_admin_user(datastore, database_session, &admin_user_id, page, per_page)
            .await?;

        let grpc_pagination = convert_pagination_to_grpc(pagination_model)?;

        Ok(GetSecurityAuditsByUserResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Get audits by IP address for gRPC
    pub async fn get_audits_by_ip_address_grpc(
        &self,
        ip_address: String,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAuditsByIpResponse> {
        let pagination_model = self
            .get_audits_by_ip_address(datastore, database_session, &ip_address, page, per_page)
            .await?;

        let grpc_pagination = convert_pagination_to_grpc(pagination_model)?;

        Ok(GetSecurityAuditsByIpResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Get paginated audits for gRPC
    pub async fn get_audits_paginated_grpc(
        &self,
        page: i64,
        per_page: i64,
        (datastore, database_session): &DB,
    ) -> Result<GetSecurityAuditsPaginatedResponse> {
        let pagination_model = self
            .get_audits_paginated(datastore, database_session, page, per_page)
            .await?;

        let grpc_pagination = convert_pagination_to_grpc(pagination_model)?;

        Ok(GetSecurityAuditsPaginatedResponse {
            status: true,
            data: Some(grpc_pagination),
        })
    }

    /// Update audit for gRPC
    pub async fn update_audit_grpc(
        &self,
        id: String,
        update_data: Option<crate::api::proto::security_audit::UpdateSecurityAuditModel>,
        (datastore, database_session): &DB,
    ) -> Result<UpdateSecurityAuditResponse> {
        let update_data = update_data
            .ok_or_else(|| crate::error::Error::Generic("Missing update data".to_string()))?;

        let updatable_model = UpdateSecurityAuditModel {
            total_authentication_attempts: update_data.total_authentication_attempts,
            failed_authentication_attempts: update_data.failed_authentication_attempts,
            blocked_injection_attempts: update_data.blocked_injection_attempts,
            rate_limited_requests: update_data.rate_limited_requests,
            suspicious_activities_detected: update_data.suspicious_activities_detected,
            security_violations: update_data.security_violations,
            uptime_seconds: update_data.uptime_seconds,
            security_health_score: update_data.security_health_score,
            metadata: if let Some(json_str) = update_data.metadata_json {
                serde_json::from_str(&json_str).ok()
            } else {
                None
            },
        };

        let model = self
            .update_audit(datastore, database_session, &id, updatable_model)
            .await?;
        let grpc_model: GrpcSecurityAuditModel = model.try_into()?;

        Ok(UpdateSecurityAuditResponse {
            status: true,
            data: Some(grpc_model),
        })
    }

    /// Delete audit for gRPC
    pub async fn delete_audit_grpc(
        &self,
        id: String,
        (datastore, database_session): &DB,
    ) -> Result<DeleteSecurityAuditResponse> {
        self.delete_audit(datastore, database_session, &id).await?;

        Ok(DeleteSecurityAuditResponse { status: true })
    }

    /// Get IP security summary for gRPC
    pub async fn get_ip_security_summary_grpc(
        &self,
        ip_address: String,
        (datastore, database_session): &DB,
    ) -> Result<GetIpSecuritySummaryResponse> {
        let summary = self
            .get_ip_security_summary(datastore, database_session, &ip_address)
            .await?;
        let grpc_summary = GrpcSecuritySummary {
            ip_address: summary.ip_address,
            total_records: summary.total_records,
            total_authentication_attempts: summary.total_authentication_attempts,
            failed_authentication_attempts: summary.failed_authentication_attempts,
            blocked_injection_attempts: summary.blocked_injection_attempts,
            rate_limited_requests: summary.rate_limited_requests,
            suspicious_activities_detected: summary.suspicious_activities_detected,
            security_violations: summary.security_violations,
            lowest_health_score: summary.lowest_health_score,
        };

        Ok(GetIpSecuritySummaryResponse {
            status: true,
            data: Some(grpc_summary),
        })
    }
}

/// Helper function to convert grpc security event type to local enum
const fn convert_grpc_security_event_type(grpc_type: GrpcSecurityEventType) -> SecurityEventType {
    match grpc_type {
        GrpcSecurityEventType::AuthenticationSuccess => SecurityEventType::AuthenticationSuccess,
        GrpcSecurityEventType::AuthenticationFailureEvent => {
            SecurityEventType::AuthenticationFailure
        }
        GrpcSecurityEventType::InjectionAttemptEvent => SecurityEventType::InjectionAttempt,
        GrpcSecurityEventType::RateLimitExceededEvent => SecurityEventType::RateLimitExceeded,
        GrpcSecurityEventType::SuspiciousActivityEvent => SecurityEventType::SuspiciousActivity,
        GrpcSecurityEventType::SecurityViolationEvent => SecurityEventType::SecurityViolation,
        GrpcSecurityEventType::Unspecified => SecurityEventType::SuspiciousActivity,
    }
}

/// Helper function to convert pagination model to gRPC
fn convert_pagination_to_grpc(
    pagination_model: SecurityAuditPaginationModel,
) -> Result<GrpcSecurityAuditPaginationModel> {
    let mut grpc_data = Vec::new();
    for model in pagination_model.data {
        let grpc_model: GrpcSecurityAuditModel = model.try_into()?;
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

    Ok(GrpcSecurityAuditPaginationModel {
        data: grpc_data,
        pagination: Some(grpc_pagination),
    })
}
