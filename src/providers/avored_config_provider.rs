use crate::{error::{Error, Result}, services::input_validation_service::InputValidationService};
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
    
    /// ldalp enabled
    pub ldap_enabled: bool,
    
    /// ldalp server
    pub ldap_server: String,
    
    /// ldap port
    pub ldap_port: u16,
    
    /// ldalp `use_tls`
    pub ldap_use_tls: bool,

    // /// ldalp base dn
    // pub ldap_base_dn: String,

    /// ldalp bind dn
    pub ldap_bind_dn: String,

    /// ldalp `bind_password`
    pub ldap_bind_password: String,

    /// ldalp user search base
    pub ldap_user_search_base: String,

    /// ldalp user search base
    pub ldap_user_search_filter: String,

    /// ldalp user attribute email
    pub ldap_user_attribute_email: String,

    /// ldalp user attribute name
    pub ldap_user_attribute_name: String,

    /// ldalp user connection timeout
    pub ldap_user_connection_timeout: u64,

    //  @todo we might required this one /// ldalp user search timeout
    // pub ldap_user_search_timeout: u64,
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
    pub fn register() -> Result<Self> {
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
        let cors_urls = vec_cors_urls.iter().map(|url| (*url).to_string()).collect();

        Ok(Self {
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
            ldap_enabled: get_env("AVORED_LDAP_ENABLED")?.parse::<bool>()?,
            ldap_server: get_env("AVORED_LDAP_SERVER")?,
            ldap_port: get_env("AVORED_LDAP_PORT")?.parse::<u16>()?,
            ldap_use_tls: get_env("AVORED_LDAP_USE_TLS")?.parse::<bool>()?,
            // ldap_base_dn: get_env("AVORED_LDAP_BASE_DN")?,
            ldap_bind_dn: get_env("AVORED_LDAP_BIND_DN")?,
            ldap_bind_password: get_env("AVORED_LDAP_BIND_PASSWORD")?,
            ldap_user_search_base: get_env("AVORED_LDAP_USER_SEARCH_BASE")?,
            ldap_user_search_filter: get_env("AVORED_LDAP_USER_SEARCH_FILTER")?,
            ldap_user_attribute_email: get_env("AVORED_LDAP_USER_ATTRIBUTE_EMAIL")?,
            ldap_user_attribute_name: get_env("AVORED_LDAP_USER_ATTRIBUTE_NAME")?,
            ldap_user_connection_timeout: get_env("AVORED_LDAP_CONNECTION_TIMEOUT")?.parse::<u64>()?,
            // ldap_user_search_timeout: get_env("AVORED_LDAP_SEARCH_TIMEOUT")?.parse::<u64>()?,
        })
    }

    /// Returns the LDAP URL based on the server and port
    #[must_use] pub fn get_ldap_url(&self) -> String {
        if self.ldap_use_tls {
            format!(
                "ldaps://{}:{}",
                self.ldap_server.replace("ldap://", "").replace("ldaps://", ""),
                self.ldap_port
            )
        } else {
            format!(
                "ldap://{}:{}",
                self.ldap_server.replace("ldap://", "").replace("ldaps://", ""),
                self.ldap_port
            )
        }
    }

    /// Generates the user search filter with the provided username
    pub fn get_user_search_filter(&self, username: &str, user_search_filter: String) -> Result<String> {
        // Validate and sanitize username to prevent LDAP injection
        let sanitized_username = InputValidationService::sanitize_ldap_value(username)?;
        Ok(user_search_filter.replace("{username}", &sanitized_username))
    }


}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name.to_string()))
}
