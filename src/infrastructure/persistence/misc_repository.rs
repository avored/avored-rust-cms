use surrealdb::types::Value;

use crate::core::domain::entities::User;
use crate::core::domain::entities::user::StorableUser;
use crate::core::domain::repositories::MiscRepository;
use crate::error::Result;
use crate::infrastructure::persistence::into_iter_objects;
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct MiscRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl MiscRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

#[async_trait::async_trait]
impl MiscRepository for MiscRepositoryImpl {
    async fn create_user(&self, storable_user: StorableUser) -> Result<User> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = "CREATE users SET name=$name, email=$email, password=$password;";
        let data: BTreeMap<String, Value> = [
            ("name".into(), Value::String(storable_user.name.into())),
            ("email".into(), Value::String(storable_user.email.into())),
            ("password".into(), Value::String(storable_user.password.into())),
        ]
        .into();

        let responses = datastore
            .execute(sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No user returned from insert".to_string()))??;

        let user: User = result_object.try_into()?;

        Ok(user)
    }
}

