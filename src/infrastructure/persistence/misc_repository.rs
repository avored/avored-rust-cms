use surrealdb::types::Value;

use crate::core::domain::entities::UserModel;
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
    async fn create_user(&self, storable_user: StorableUser) -> Result<UserModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = "CREATE users SET name=$name, email=$email, password=$password;, created_at=now(), updated_at=now(), created_by=$performing_user, updated_by=$performing_user;";
        let data: BTreeMap<String, Value> = [
            ("name".into(), Value::String(storable_user.name.into())),
            ("email".into(), Value::String(storable_user.email.into())),
            ("password".into(), Value::String(storable_user.password.into())),
            ("performing_user".into(), Value::String(storable_user.processing_user.into())),
        ]
        .into();

        let responses = datastore
            .execute(sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No user returned from insert".to_string()))??;

        let user: UserModel = result_object.try_into()?;

        Ok(user)
    }
}

pub async fn test_misc_repository() -> MiscRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    MiscRepositoryImpl::new(Arc::new(provider))
}
