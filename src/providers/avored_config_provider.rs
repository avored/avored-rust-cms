use std::env;
use dotenvy::dotenv;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct AvoRedConfigProvider {
    pub database_folder_name: String,
    pub database_namespace: String,
    pub database_name: String,
    pub jwt_secret_key: String,
    pub react_admin_app_url: String,
    pub react_frontend_app_url: String,
    pub back_end_app_url: String,
    pub password_salt: String,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_port: u16,
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
            database_folder_name: get_env("AVORED_DATABASE_FOLDER_NAME")?,
            database_namespace: get_env("AVORED_DATABASE_NAMESPACE")?,
            database_name: get_env("AVORED_DATABASE_NAME")?,
            jwt_secret_key: get_env("AVORED_JWT_SECRET")?,
            react_admin_app_url: get_env("AVORED_REACT_ADMIN_APP_URL")?,
            react_frontend_app_url: get_env("AVORED_REACT_FRONTEND_APP_URL")?,
            back_end_app_url: get_env("AVORED_BACK_END_APP_URL")?,
            password_salt: get_env("AVORED_PASSWORD_SALT")?,
            smtp_host: get_env("SMTP_HOST")?,
            smtp_username: get_env("SMTP_USERNAME")?,
            smtp_password: get_env("SMTP_PASSWORD")?,
            smtp_port: get_env("SMTP_PORT")?.parse::<u16>()?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name.to_string()))
}
