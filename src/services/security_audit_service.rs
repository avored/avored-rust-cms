use std::collections::BTreeMap;
use crate::error::Result;
use crate::models::security_audit_model::{
    CreateSecurityAuditModel, SecurityAuditModel, SecurityAuditPaginationModel, UpdateSecurityAuditModel
};
use crate::repositories::security_audit_repository::SecurityAuditRepository;
use surrealdb::kvs::Datastore;
use surrealdb::dbs::Session;
use surrealdb::sql::Value;

#[derive(Clone)]
pub struct SecurityAuditService {
    security_audit_repository: SecurityAuditRepository,
}

impl SecurityAuditService {
    pub fn new(security_audit_repository: SecurityAuditRepository) -> Self {
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

        self.create_audit(datastore, database_session, create_model).await
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
            .update(datastore, database_session, id, updateable_security_audit_model)
            .await
    }

    /// Increment security metrics for an existing audit record
    pub async fn increment_security_metrics(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        event_type: SecurityEventType,
    ) -> Result<SecurityAuditModel> {
        // First get the current audit record
        let current_audit = self.get_audit_by_id(datastore, database_session, id).await?;
        
        let mut update_model = UpdateSecurityAuditModel::default();
        
        // Increment appropriate counters
        match event_type {
            SecurityEventType::AuthenticationSuccess => {
                update_model.total_authentication_attempts = Some(current_audit.total_authentication_attempts + 1);
            }
            SecurityEventType::AuthenticationFailure => {
                update_model.total_authentication_attempts = Some(current_audit.total_authentication_attempts + 1);
                update_model.failed_authentication_attempts = Some(current_audit.failed_authentication_attempts + 1);
            }
            SecurityEventType::InjectionAttempt => {
                update_model.blocked_injection_attempts = Some(current_audit.blocked_injection_attempts + 1);
            }
            SecurityEventType::RateLimitExceeded => {
                update_model.rate_limited_requests = Some(current_audit.rate_limited_requests + 1);
            }
            SecurityEventType::SuspiciousActivity => {
                update_model.suspicious_activities_detected = Some(current_audit.suspicious_activities_detected + 1);
            }
            SecurityEventType::SecurityViolation => {
                update_model.security_violations = Some(current_audit.security_violations + 1);
            }
        }

        // Recalculate health score based on updated metrics
        let updated_audit = SecurityAuditModel {
            total_authentication_attempts: update_model.total_authentication_attempts.unwrap_or(current_audit.total_authentication_attempts),
            failed_authentication_attempts: update_model.failed_authentication_attempts.unwrap_or(current_audit.failed_authentication_attempts),
            blocked_injection_attempts: update_model.blocked_injection_attempts.unwrap_or(current_audit.blocked_injection_attempts),
            rate_limited_requests: update_model.rate_limited_requests.unwrap_or(current_audit.rate_limited_requests),
            suspicious_activities_detected: update_model.suspicious_activities_detected.unwrap_or(current_audit.suspicious_activities_detected),
            security_violations: update_model.security_violations.unwrap_or(current_audit.security_violations),
            ..current_audit
        };
        
        update_model.security_health_score = Some(updated_audit.calculate_health_score());

        self.update_audit(datastore, database_session, id, update_model).await
    }

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
    fn calculate_event_health_score(&self, event_type: &SecurityEventType) -> f64 {
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
        let audits = self.get_audits_by_ip_address(datastore, database_session, ip_address, 1, 100).await?;
        
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

#[derive(Debug, Clone)]
pub enum SecurityEventType {
    AuthenticationSuccess,
    AuthenticationFailure,
    InjectionAttempt,
    RateLimitExceeded,
    SuspiciousActivity,
    SecurityViolation,
}

#[derive(Debug, Clone)]
pub struct SecuritySummary {
    pub ip_address: String,
    pub total_records: i64,
    pub total_authentication_attempts: i32,
    pub failed_authentication_attempts: i32,
    pub blocked_injection_attempts: i32,
    pub rate_limited_requests: i32,
    pub suspicious_activities_detected: i32,
    pub security_violations: i32,
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
