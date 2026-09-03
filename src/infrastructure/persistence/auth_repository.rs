use surrealdb::types::Value;

use crate::core::domain::{entities::User, repositories::AuthRepository};
use crate::infrastructure::persistence::into_iter_objects;

use crate::providers::avored_database_provider::AvoRedDatabaseProvider;
use std::collections::BTreeMap;
use std::sync::Arc;

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
    async fn authenticate(&self, email: &str, password: &str) -> Option<User> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = "SELECT * FROM users WHERE email=$email AND password=$password;";
        let data: BTreeMap<String, Value> = [
            ("email".into(), Value::String(email.into())),
            ("password".into(), Value::String(password.into())),
        ]
        .into();

        let responses = datastore
            .execute(sql, database_session, Some(data.into()))
            .await
            .ok()?;

        
        let result_object = into_iter_objects(responses).ok()?.next()?.ok()?;
        
        // println!("from admin user: {:#?}", result_object);

        let admin_user: User = result_object.try_into().ok()?;

        println!("Admin user: {:?}", admin_user);



        // TODO: check/verify hashed password with `password` before returning user
        Some(admin_user)
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
