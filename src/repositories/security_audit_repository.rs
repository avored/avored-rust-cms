use crate::error::{Error, Result};
use crate::models::security_audit_model::{
    CreateSecurityAuditModel, SecurityAuditModel, SecurityAuditPaginationModel,
    UpdateSecurityAuditModel,
};
use crate::models::{ModelCount, Pagination};
use crate::repositories::into_iter_objects;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

#[derive(Clone)]
pub struct SecurityAuditRepository;

impl SecurityAuditRepository {
    pub fn new() -> Self {
        Self
    }

    /// Create a new security audit record
    pub async fn create(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_security_audit_model: CreateSecurityAuditModel,
    ) -> Result<SecurityAuditModel> {
        // Validate the input
        createable_security_audit_model.validate()?;

        let sql = r#"
            CREATE security_audits CONTENT {
                security_audit_id: $security_audit_id,
                admin_user_id: $admin_user_id,
                session_id: $session_id,
                ip_address: $ip_address,
                user_agent: $user_agent,
                endpoint: $endpoint,
                request_method: $request_method,
                total_authentication_attempts: $total_authentication_attempts,
                failed_authentication_attempts: $failed_authentication_attempts,
                blocked_injection_attempts: $blocked_injection_attempts,
                rate_limited_requests: $rate_limited_requests,
                suspicious_activities_detected: $suspicious_activities_detected,
                security_violations: $security_violations,
                uptime_seconds: $uptime_seconds,
                security_health_score: $security_health_score,
                metadata: $metadata,
                created_at: time::now(),
                updated_at: time::now()
            };
        "#;

        let vars: BTreeMap<String, Value> = [
            (
                "security_audit_id".into(),
                createable_security_audit_model.security_audit_id.into(),
            ),
            (
                "admin_user_id".into(),
                match createable_security_audit_model.admin_user_id {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "session_id".into(),
                match createable_security_audit_model.session_id {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "ip_address".into(),
                createable_security_audit_model.ip_address.into(),
            ),
            (
                "user_agent".into(),
                match createable_security_audit_model.user_agent {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "endpoint".into(),
                match createable_security_audit_model.endpoint {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "request_method".into(),
                match createable_security_audit_model.request_method {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "total_authentication_attempts".into(),
                createable_security_audit_model
                    .total_authentication_attempts
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "failed_authentication_attempts".into(),
                createable_security_audit_model
                    .failed_authentication_attempts
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "blocked_injection_attempts".into(),
                createable_security_audit_model
                    .blocked_injection_attempts
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "rate_limited_requests".into(),
                createable_security_audit_model
                    .rate_limited_requests
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "suspicious_activities_detected".into(),
                createable_security_audit_model
                    .suspicious_activities_detected
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "security_violations".into(),
                createable_security_audit_model
                    .security_violations
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "uptime_seconds".into(),
                createable_security_audit_model
                    .uptime_seconds
                    .unwrap_or(0)
                    .into(),
            ),
            (
                "security_health_score".into(),
                createable_security_audit_model
                    .security_health_score
                    .unwrap_or(100.0)
                    .into(),
            ),
            (
                "metadata".into(),
                match createable_security_audit_model.metadata {
                    Some(val) => Value::Object(val.into()),
                    None => Value::None,
                },
            ),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => {
                return Err(Error::Generic(
                    "Failed to create security audit record".to_string(),
                ))
            }
        };

        let model: Result<SecurityAuditModel> = result_object?.try_into();
        model
    }

    /// Find security audit record by ID
    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<SecurityAuditModel> {
        let sql = "SELECT * FROM type::thing('security_audits', $id);";
        let vars: BTreeMap<String, Value> = [("id".into(), id.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => {
                return Err(Error::Generic(
                    "Security audit record not found".to_string(),
                ))
            }
        };

        let model: Result<SecurityAuditModel> = result_object?.try_into();
        model
    }

    /// Find security audit records by admin user ID
    pub async fn find_by_admin_user_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        admin_user_id: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_audits 
            WHERE admin_user_id = $admin_user_id 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("admin_user_id".into(), admin_user_id.into()),
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_audits = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_audit: SecurityAuditModel = object?.try_into()?;
            security_audits.push(security_audit);
        }

        // Get total count
        let count_sql =
            "SELECT count() FROM security_audits WHERE admin_user_id = $admin_user_id GROUP ALL;";
        let count_vars: BTreeMap<String, Value> =
            [("admin_user_id".into(), admin_user_id.into())].into();
        let count_responses = datastore
            .execute(count_sql, database_session, Some(count_vars))
            .await?;

        let total = if let Some(count_object) = into_iter_objects(count_responses)?.next() {
            let model_count: ModelCount = count_object?.try_into()?;
            model_count.total
        } else {
            0
        };

        let pagination = Pagination {
            total,
            per_page,
            current_page: page,
            from: offset + 1,
            to: offset + security_audits.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAuditPaginationModel {
            data: security_audits,
            pagination,
        })
    }

    /// Find security audit records by IP address
    pub async fn find_by_ip_address(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        ip_address: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_audits 
            WHERE ip_address = $ip_address 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("ip_address".into(), ip_address.into()),
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_audits = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_audit: SecurityAuditModel = object?.try_into()?;
            security_audits.push(security_audit);
        }

        // Get total count
        let count_sql =
            "SELECT count() FROM security_audits WHERE ip_address = $ip_address GROUP ALL;";
        let count_vars: BTreeMap<String, Value> = [("ip_address".into(), ip_address.into())].into();
        let count_responses = datastore
            .execute(count_sql, database_session, Some(count_vars))
            .await?;

        let total = if let Some(count_object) = into_iter_objects(count_responses)?.next() {
            let model_count: ModelCount = count_object?.try_into()?;
            model_count.total
        } else {
            0
        };

        let pagination = Pagination {
            total,
            per_page,
            current_page: page,
            from: offset + 1,
            to: offset + security_audits.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAuditPaginationModel {
            data: security_audits,
            pagination,
        })
    }

    /// Update security audit record
    pub async fn update(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        updateable_security_audit_model: UpdateSecurityAuditModel,
    ) -> Result<SecurityAuditModel> {
        let sql = r#"
            UPDATE type::thing('security_audits', $id) MERGE {
                total_authentication_attempts: $total_authentication_attempts,
                failed_authentication_attempts: $failed_authentication_attempts,
                blocked_injection_attempts: $blocked_injection_attempts,
                rate_limited_requests: $rate_limited_requests,
                suspicious_activities_detected: $suspicious_activities_detected,
                security_violations: $security_violations,
                uptime_seconds: $uptime_seconds,
                security_health_score: $security_health_score,
                metadata: $metadata,
                updated_at: time::now()
            };
        "#;

        let vars: BTreeMap<String, Value> = [
            ("id".into(), id.into()),
            (
                "total_authentication_attempts".into(),
                match updateable_security_audit_model.total_authentication_attempts {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "failed_authentication_attempts".into(),
                match updateable_security_audit_model.failed_authentication_attempts {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "blocked_injection_attempts".into(),
                match updateable_security_audit_model.blocked_injection_attempts {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "rate_limited_requests".into(),
                match updateable_security_audit_model.rate_limited_requests {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "suspicious_activities_detected".into(),
                match updateable_security_audit_model.suspicious_activities_detected {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "security_violations".into(),
                match updateable_security_audit_model.security_violations {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "uptime_seconds".into(),
                match updateable_security_audit_model.uptime_seconds {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "security_health_score".into(),
                match updateable_security_audit_model.security_health_score {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "metadata".into(),
                match updateable_security_audit_model.metadata {
                    Some(val) => Value::Object(val.into()),
                    None => Value::None,
                },
            ),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => {
                return Err(Error::Generic(
                    "Failed to update security audit record".to_string(),
                ))
            }
        };

        let model: Result<SecurityAuditModel> = result_object?.try_into();
        model
    }

    /// Delete security audit record
    pub async fn delete(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<()> {
        let sql = "DELETE type::thing('security_audits', $id);";
        let vars: BTreeMap<String, Value> = [("id".into(), id.into())].into();

        datastore.execute(sql, database_session, Some(vars)).await?;
        Ok(())
    }

    /// Get all security audit records with pagination
    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAuditPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_audits 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_audits = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_audit: SecurityAuditModel = object?.try_into()?;
            security_audits.push(security_audit);
        }

        // Get total count
        let count_sql = "SELECT count() FROM security_audits GROUP ALL;";
        let count_responses = datastore.execute(count_sql, database_session, None).await?;

        let total = if let Some(count_object) = into_iter_objects(count_responses)?.next() {
            let model_count: ModelCount = count_object?.try_into()?;
            model_count.total
        } else {
            0
        };

        let pagination = Pagination {
            total,
            per_page,
            current_page: page,
            from: offset + 1,
            to: offset + security_audits.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAuditPaginationModel {
            data: security_audits,
            pagination,
        })
    }
}
