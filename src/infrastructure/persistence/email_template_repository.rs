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
