use surrealdb::types::Value;

use crate::core::domain::{entities::UserModel, repositories::AuthRepository};
use crate::infrastructure::persistence::into_iter_objects;

use crate::providers::avored_database_provider::AvoRedDatabaseProvider;
use std::collections::BTreeMap;
use std::sync::Arc;
use crate::error::Result;

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl AuthRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

#[async_trait::async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn authenticate(&self, email: &str) -> Result<UserModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = "SELECT * FROM users WHERE email=$email;";
        let data: BTreeMap<String, Value> = [
            ("email".into(), Value::String(email.into())),
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



pub async fn test_auth_repository() -> AuthRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    let (datastore, session) = &provider.db;
    datastore
        .execute(
            "CREATE users:test_user SET name = 'Test User', email = 'test@example.com', password = 'secret';",
            session,
            None,
        )
        .await
        .expect("test user should be created");

    AuthRepositoryImpl::new(Arc::new(provider))
}
