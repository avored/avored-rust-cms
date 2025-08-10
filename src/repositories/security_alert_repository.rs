use crate::error::{Error, Result};
use crate::models::security_alert_model::{
    AlertSeverity, AlertType, CreateSecurityAlertModel, SecurityAlertModel,
    SecurityAlertPaginationModel, UpdateSecurityAlertModel,
};
use crate::models::{ModelCount, Pagination};
use crate::repositories::into_iter_objects;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

/// security alert repository
#[derive(Clone)]
pub struct SecurityAlertRepository;

impl SecurityAlertRepository {
    /// new instance 
    pub fn new() -> Self {
        Self
    }

    /// Create a new security alert record
    pub async fn create(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_security_alert_model: CreateSecurityAlertModel,
    ) -> Result<SecurityAlertModel> {
        // Validate the input
        createable_security_alert_model.validate()?;

        let sql = r#"
            CREATE security_alerts CONTENT {
                alert_id: $alert_id,
                alert_type: $alert_type,
                severity: $severity,
                message: $message,
                source: $source,
                affected_resource: $affected_resource,
                metadata: $metadata,
                resolved: false,
                resolved_at: NONE,
                resolved_by: NONE,
                created_at: time::now()
            };
        "#;

        let vars: BTreeMap<String, Value> = [
            (
                "alert_id".into(),
                createable_security_alert_model.alert_id.into(),
            ),
            (
                "alert_type".into(),
                createable_security_alert_model.alert_type.as_str().into(),
            ),
            (
                "severity".into(),
                createable_security_alert_model.severity.as_str().into(),
            ),
            (
                "message".into(),
                createable_security_alert_model.message.into(),
            ),
            (
                "source".into(),
                createable_security_alert_model.source.into(),
            ),
            (
                "affected_resource".into(),
                match createable_security_alert_model.affected_resource {
                    Some(val) => val.into(),
                    None => Value::None,
                },
            ),
            (
                "metadata".into(),
                match createable_security_alert_model.metadata {
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
                    "Failed to create security alert record".to_string(),
                ))
            }
        };

        let model: Result<SecurityAlertModel> = result_object?.try_into();
        model
    }

    /// Find security alert record by ID
    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<SecurityAlertModel> {
        let sql = "SELECT * FROM type::thing('security_alerts', $id);";
        let vars: BTreeMap<String, Value> = [("id".into(), id.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => {
                return Err(Error::Generic(
                    "Security alert record not found".to_string(),
                ))
            }
        };

        let model: Result<SecurityAlertModel> = result_object?.try_into();
        model
    }

    /// Find unresolved security alerts by severity
    pub async fn find_unresolved_by_severity(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        severity: AlertSeverity,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_alerts 
            WHERE severity = $severity AND resolved = false 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("severity".into(), severity.as_str().into()),
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_alerts = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_alert: SecurityAlertModel = object?.try_into()?;
            security_alerts.push(security_alert);
        }

        // Get total count
        let count_sql = "SELECT count() FROM security_alerts WHERE severity = $severity AND resolved = false GROUP ALL;";
        let count_vars: BTreeMap<String, Value> =
            [("severity".into(), severity.as_str().into())].into();
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
            to: offset + security_alerts.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAlertPaginationModel {
            data: security_alerts,
            pagination,
        })
    }

    /// Find security alerts by type
    pub async fn find_by_alert_type(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        alert_type: AlertType,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_alerts 
            WHERE alert_type = $alert_type 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("alert_type".into(), alert_type.as_str().into()),
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_alerts = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_alert: SecurityAlertModel = object?.try_into()?;
            security_alerts.push(security_alert);
        }

        // Get total count
        let count_sql =
            "SELECT count() FROM security_alerts WHERE alert_type = $alert_type GROUP ALL;";
        let count_vars: BTreeMap<String, Value> =
            [("alert_type".into(), alert_type.as_str().into())].into();
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
            to: offset + security_alerts.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAlertPaginationModel {
            data: security_alerts,
            pagination,
        })
    }

    /// Find security alerts by source
    pub async fn find_by_source(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        source: &str,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_alerts 
            WHERE source = $source 
            ORDER BY created_at DESC 
            LIMIT $per_page 
            START $offset;
        "#;

        let vars: BTreeMap<String, Value> = [
            ("source".into(), source.into()),
            ("per_page".into(), per_page.into()),
            ("offset".into(), offset.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut security_alerts = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_alert: SecurityAlertModel = object?.try_into()?;
            security_alerts.push(security_alert);
        }

        // Get total count
        let count_sql = "SELECT count() FROM security_alerts WHERE source = $source GROUP ALL;";
        let count_vars: BTreeMap<String, Value> = [("source".into(), source.into())].into();
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
            to: offset + security_alerts.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAlertPaginationModel {
            data: security_alerts,
            pagination,
        })
    }

    /// Update security alert record (mainly for resolution)
    pub async fn update(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        updateable_security_alert_model: UpdateSecurityAlertModel,
    ) -> Result<SecurityAlertModel> {
        let mut sql_parts = Vec::new();
        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("id".into(), id.into());

        if let Some(resolved) = updateable_security_alert_model.resolved {
            sql_parts.push("resolved: $resolved");
            vars.insert("resolved".into(), resolved.into());

            if resolved {
                sql_parts.push("resolved_at: time::now()");
                if let Some(resolved_by) = updateable_security_alert_model.resolved_by {
                    sql_parts.push("resolved_by: $resolved_by");
                    vars.insert("resolved_by".into(), resolved_by.into());
                }
            }
        }

        if let Some(metadata) = updateable_security_alert_model.metadata {
            sql_parts.push("metadata: $metadata");
            vars.insert("metadata".into(), Value::Object(metadata.into()));
        }

        if sql_parts.is_empty() {
            return Err(Error::Generic("No fields to update".to_string()));
        }

        let sql = format!(
            "UPDATE type::thing('security_alerts', $id) MERGE {{ {} }};",
            sql_parts.join(", ")
        );

        let responses = datastore
            .execute(&sql, database_session, Some(vars))
            .await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => {
                return Err(Error::Generic(
                    "Failed to update security alert record".to_string(),
                ))
            }
        };

        let model: Result<SecurityAlertModel> = result_object?.try_into();
        model
    }

    /// Resolve a security alert
    pub async fn resolve(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
        resolved_by: &str,
    ) -> Result<SecurityAlertModel> {
        let update_model = UpdateSecurityAlertModel {
            resolved: Some(true),
            resolved_by: Some(resolved_by.to_string()),
            metadata: None,
        };

        self.update(datastore, database_session, id, update_model)
            .await
    }

    /// Delete security alert record
    pub async fn delete(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<()> {
        let sql = "DELETE type::thing('security_alerts', $id);";
        let vars: BTreeMap<String, Value> = [("id".into(), id.into())].into();

        datastore.execute(sql, database_session, Some(vars)).await?;
        Ok(())
    }

    /// Get all security alert records with pagination
    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page: i64,
        per_page: i64,
    ) -> Result<SecurityAlertPaginationModel> {
        let offset = (page - 1) * per_page;

        let sql = r#"
            SELECT * FROM security_alerts 
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

        let mut security_alerts = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_alert: SecurityAlertModel = object?.try_into()?;
            security_alerts.push(security_alert);
        }

        // Get total count
        let count_sql = "SELECT count() FROM security_alerts GROUP ALL;";
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
            to: offset + security_alerts.len() as i64,
            has_next_page: (offset + per_page) < total,
            has_previous_page: page > 1,
            next_page_number: if (offset + per_page) < total {
                page + 1
            } else {
                page
            },
            previous_page_number: if page > 1 { page - 1 } else { 1 },
        };

        Ok(SecurityAlertPaginationModel {
            data: security_alerts,
            pagination,
        })
    }

    /// Get critical unresolved alerts
    pub async fn get_critical_unresolved(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<Vec<SecurityAlertModel>> {
        let sql = r#"
            SELECT * FROM security_alerts 
            WHERE severity IN ['high', 'critical'] AND resolved = false 
            ORDER BY created_at DESC;
        "#;

        let responses = datastore.execute(sql, database_session, None).await?;

        let mut security_alerts = Vec::new();
        for object in into_iter_objects(responses)? {
            let security_alert: SecurityAlertModel = object?.try_into()?;
            security_alerts.push(security_alert);
        }

        Ok(security_alerts)
    }
}
