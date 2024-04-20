use std::env;
use dotenvy::dotenv;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct AvoRedConfigProvider {
    pub database_namespace: String,
    pub database_name: String,
    pub session_secret_key: String,
    pub jwt_secret_key: String,
    pub front_end_app_url: String,
    pub password_salt: String
}

// pub fn config() -> &'static AvoRedConfigProvider {
//     static INSTANCE: OnceLock<AvoRedConfigProvider> = OnceLock::new();
//
//     INSTANCE.get_or_init(|| {
//         AvoRedConfigProvider::register()
//             .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
//     })
// }

impl AvoRedConfigProvider {
    pub fn register() -> Result<AvoRedConfigProvider> {
        dotenv().ok();
        Ok(AvoRedConfigProvider {
            database_namespace: get_env("AVORED_DATABASE_NAMESPACE")?,
            database_name: get_env("AVORED_DATABASE_NAME")?,
            session_secret_key: get_env("AVORED_SESSION_SECRET_KEY")?,
            jwt_secret_key: get_env("AVORED_JWT_SECRET")?,
            front_end_app_url: get_env("AVORED_FRONT_END_APP_URL")?,
            password_salt: get_env("AVORED_PASSWORD_SALT")?
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name.to_string()))
}
