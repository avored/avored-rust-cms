use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;


/// avored config provider
#[derive(Debug, Clone)]
pub struct AvoRedConfigProvider {
    /// database folder name
    pub database_folder_name: String,
    /// database namespace 
    pub database_namespace: String,
    /// database name
    pub database_name: String,
    /// jwt secret key
    pub jwt_secret_key: String,
    /// react admin app url
    pub react_admin_app_url: String,
    // pub react_frontend_app_url: String,
    // pub back_end_app_url: String,

    /// cors allowed app url
    pub cors_allowed_app_url: Vec<String>,

    /// password salt
    pub password_salt: String,

    /// smtp host
    pub smtp_host: String,

    /// smtp user name
    pub smtp_username: String,

    /// smtp password
    pub smtp_password: String,

    /// smtp port
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

    /// register avored config provider
    pub fn register() -> Result<AvoRedConfigProvider> {
        dotenv()?;

        match get_env("APP_ENV")?.as_str() {
            "prod" => dotenvy::from_filename_override(".env.prod")?,
            "stag" => dotenvy::from_filename_override(".env.stag")?,
            "test" => dotenvy::from_filename_override(".env.test")?,
            "dev" => dotenvy::from_filename_override(".env.dev")?,
            // as if it won't match any we load dev as default
            _ => dotenvy::from_filename_override(".env")?,
        };

        let env_str_allowed_cors = get_env("AVORED_CORS_ALLOWED_APP_URL")?;
        let vec_cors_urls = env_str_allowed_cors.split(',').collect::<Vec<&str>>();
        let cors_urls = vec_cors_urls.iter().map(|url| url.to_string()).collect();

        Ok(AvoRedConfigProvider {
            database_folder_name: get_env("AVORED_DATABASE_FOLDER_NAME")?,
            database_namespace: get_env("AVORED_DATABASE_NAMESPACE")?,
            database_name: get_env("AVORED_DATABASE_NAME")?,
            jwt_secret_key: get_env("AVORED_JWT_SECRET")?,
            react_admin_app_url: get_env("AVORED_REACT_ADMIN_APP_URL")?,
            // react_frontend_app_url: get_env("AVORED_REACT_FRONTEND_APP_URL")?,
            // back_end_app_url: get_env("AVORED_BACK_END_APP_URL")?,
            cors_allowed_app_url: cors_urls,
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
