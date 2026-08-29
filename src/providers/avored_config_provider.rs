use std::env;

use crate::error::{Error, Result};
use dotenvy::dotenv;

pub struct AvoRedConfigProvider {
    pub database_folder: String,
    pub database_name: String,
    pub database_namespace: String,
    pub password_salt: String,
    pub jwt_secret_key: String,
    pub cors_allowed_app_url: Vec<String>,
}

impl AvoRedConfigProvider {
    pub fn new() -> Result<AvoRedConfigProvider> {
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
        let cors_allowed_app_url = vec_cors_urls.iter().map(|url| (*url).to_string()).collect();

        Ok(AvoRedConfigProvider {
            database_namespace: get_env("AVORED_DATABASE_NAMESPACE")?,
            database_name: get_env("AVORED_DATABASE_NAME")?,
            database_folder: get_env("AVORED_DATABASE_FOLDER_NAME")?,
            password_salt: get_env("AVORED_PASSWORD_SALT")?,
            jwt_secret_key: get_env("AVORED_JWT_SECRET")?,
            cors_allowed_app_url,
        })
    }
}

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name.to_string()))
}
