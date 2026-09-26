use std::collections::BTreeMap;
use std::sync::Arc;

use surrealdb::types::{Number, Value};

use crate::core::domain::constants::EMAIL_TEMPLATES_TABLE_NAME;
use crate::core::domain::entities::email_template::EmailTemplateModel;
use crate::core::domain::repositories::EmailTemplateRepository;
use crate::error::Result;
use crate::infrastructure::persistence::into_iter_objects;
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;

#[derive(Clone)]
pub struct EmailTemplateRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl EmailTemplateRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

#[async_trait::async_trait]
impl EmailTemplateRepository for EmailTemplateRepositoryImpl {
    async fn create(
        &self,
        name: String,
        subject: String,
        body_html: String,
        body_plain: Option<String>,
    ) -> Result<EmailTemplateModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "CREATE {} SET name=$name, subject=$subject, body_html=$body_html, body_plain=$body_plain, created_at=time::now(), updated_at=time::now(), deleted_at=NONE, deleted_by=NONE;",
            EMAIL_TEMPLATES_TABLE_NAME
        );

        let data = BTreeMap::from([
            ("name".to_string(), Value::String(name)),
            ("subject".to_string(), Value::String(subject)),
            ("body_html".to_string(), Value::String(body_html)),
            (
                "body_plain".to_string(),
                body_plain.map(Value::String).unwrap_or(Value::None),
            ),
        ]);

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let obj = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No email template returned from insert".to_string())
        })??;

        let template: EmailTemplateModel = obj.try_into()?;
        Ok(template)
    }

    async fn find_by_id(&self, id: &str) -> Result<EmailTemplateModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: EMAIL_TEMPLATES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!(
            "SELECT * FROM {} WHERE id = $id AND deleted_at = NONE;",
            EMAIL_TEMPLATES_TABLE_NAME
        );

        let data: BTreeMap<String, Value> =
            [("id".to_string(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut items = into_iter_objects(responses)?;
        if let Some(obj_res) = items.next() {
            let obj = obj_res?;
            let template: EmailTemplateModel = obj.try_into()?;
            return Ok(template);
        }

        Err(crate::error::Error::NotFound(format!(
            "Email template with ID '{}' not found",
            id
        )))
    }

    async fn update(
        &self,
        id: &str,
        name: String,
        subject: String,
        body_html: String,
        body_plain: Option<String>,
    ) -> Result<EmailTemplateModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: EMAIL_TEMPLATES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!(
            "UPDATE {} SET name=$name, subject=$subject, body_html=$body_html, body_plain=$body_plain, updated_at=time::now() WHERE id = $id AND deleted_at = NONE RETURN *;",
            EMAIL_TEMPLATES_TABLE_NAME
        );

        let data = BTreeMap::from([
            ("id".to_string(), Value::RecordId(target_record)),
            ("name".to_string(), Value::String(name)),
            ("subject".to_string(), Value::String(subject)),
            ("body_html".to_string(), Value::String(body_html)),
            (
                "body_plain".to_string(),
                body_plain.map(Value::String).unwrap_or(Value::None),
            ),
        ]);

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut items = into_iter_objects(responses)?;
        if let Some(obj_res) = items.next() {
            let obj = obj_res?;
            let template: EmailTemplateModel = obj.try_into()?;
            return Ok(template);
        }

        Err(crate::error::Error::NotFound(format!(
            "Email template with ID '{}' not found",
            id
        )))
    }

    async fn paginate(&self, page: u64, page_size: u64) -> Result<(Vec<EmailTemplateModel>, u64)> {
        let (datastore, database_session) = &self.database_provider.db;
        let skip = page.saturating_sub(1) * page_size;

        let sql = format!(
            "SELECT * FROM {} WHERE deleted_at = NONE ORDER BY created_at DESC LIMIT $limit START $skip;",
            EMAIL_TEMPLATES_TABLE_NAME
        );

        let data = std::collections::BTreeMap::from([
            ("limit".to_string(), Value::Number(Number::Int(page_size as i64))),
            ("skip".to_string(), Value::Number(Number::Int(skip as i64))),
        ]);

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut models = Vec::new();
        for obj_res in into_iter_objects(responses)? {
            let obj = obj_res?;
            let template: EmailTemplateModel = obj.try_into()?;
            models.push(template);
        }

        let count_sql = format!(
            "SELECT count() AS total FROM {} WHERE deleted_at = NONE;",
            EMAIL_TEMPLATES_TABLE_NAME
        );
        let count_responses = datastore.execute(&count_sql, database_session, None).await?;
        let total = match into_iter_objects(count_responses)?.next() {
            Some(obj_res) => {
                let obj = obj_res?;
                let total = obj.get("total").and_then(|v| match v {
                    surrealdb::types::Value::Number(num) => Some(num.clone()),
                    _ => None,
                });
                match total {
                    Some(num) => num.into_int().unwrap_or(0) as u64,
                    None => 0,
                }
            }
            None => 0,
        };

        Ok((models, total))
    }
}
