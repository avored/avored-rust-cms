use serde::{Deserialize, Serialize};
use std::env;
use crate::error::{Error, Result};
use crate::services::input_validation_service::InputValidationService;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct LdapConfig {
    pub enabled: bool,
    pub server: String,
    pub port: u16,
    pub use_tls: bool,
    pub base_dn: String,
    pub bind_dn: String,
    pub bind_password: String,
    pub user_search_base: String,
    pub user_search_filter: String,
    pub user_attribute_email: String,
    pub user_attribute_name: String,
    pub connection_timeout: u64,
    pub search_timeout: u64,
}

impl Default for LdapConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server: "ldaps://localhost".to_string(),
            port: 636,
            use_tls: true, // Secure by default
            base_dn: "dc=example,dc=com".to_string(),
            bind_dn: "cn=admin,dc=example,dc=com".to_string(),
            bind_password: String::new(),
            user_search_base: "ou=users,dc=example,dc=com".to_string(),
            user_search_filter: "(uid={username})".to_string(),
            user_attribute_email: "mail".to_string(),
            user_attribute_name: "displayName".to_string(),
            connection_timeout: 30,
            search_timeout: 30,
        }
    }
}

impl LdapConfig {
    pub fn from_env() -> Result<Self> {
        let enabled = env::var("AVORED_LDAP_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        if !enabled {
            return Ok(Self::default());
        }

        let server = env::var("AVORED_LDAP_SERVER")
            .map_err(|_| Error::ConfigMissing("AVORED_LDAP_SERVER".to_string()))?;
        let server = InputValidationService::validate_server_url(&server)?;

        let port = env::var("AVORED_LDAP_PORT")
            .unwrap_or_else(|_| "389".to_string())
            .parse::<u16>()
            .map_err(|_| Error::ConfigMissing("Invalid AVORED_LDAP_PORT".to_string()))?;

        let use_tls = env::var("AVORED_LDAP_USE_TLS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let base_dn = env::var("AVORED_LDAP_BASE_DN")
            .map_err(|_| Error::ConfigMissing("AVORED_LDAP_BASE_DN".to_string()))?;
        let base_dn = InputValidationService::validate_dn(&base_dn)?;

        let bind_dn = env::var("AVORED_LDAP_BIND_DN")
            .map_err(|_| Error::ConfigMissing("AVORED_LDAP_BIND_DN".to_string()))?;
        let bind_dn = InputValidationService::validate_dn(&bind_dn)?;

        let bind_password = env::var("AVORED_LDAP_BIND_PASSWORD")
            .map_err(|_| Error::ConfigMissing("AVORED_LDAP_BIND_PASSWORD".to_string()))?;

        let user_search_base = env::var("AVORED_LDAP_USER_SEARCH_BASE")
            .map_err(|_| Error::ConfigMissing("AVORED_LDAP_USER_SEARCH_BASE".to_string()))?;
        let user_search_base = InputValidationService::validate_dn(&user_search_base)?;

        let user_search_filter = env::var("AVORED_LDAP_USER_SEARCH_FILTER")
            .unwrap_or_else(|_| "(uid={username})".to_string());

        let user_attribute_email = env::var("AVORED_LDAP_USER_ATTRIBUTE_EMAIL")
            .unwrap_or_else(|_| "mail".to_string());

        let user_attribute_name = env::var("AVORED_LDAP_USER_ATTRIBUTE_NAME")
            .unwrap_or_else(|_| "displayName".to_string());

        let connection_timeout = env::var("AVORED_LDAP_CONNECTION_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);

        let search_timeout = env::var("AVORED_LDAP_SEARCH_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);

        Ok(Self {
            enabled,
            server,
            port,
            use_tls,
            base_dn,
            bind_dn,
            bind_password,
            user_search_base,
            user_search_filter,
            user_attribute_email,
            user_attribute_name,
            connection_timeout,
            search_timeout,
        })
    }

    pub fn get_ldap_url(&self) -> String {
        if self.use_tls {
            format!("ldaps://{}:{}", self.server.replace("ldap://", "").replace("ldaps://", ""), self.port)
        } else {
            format!("ldap://{}:{}", self.server.replace("ldap://", "").replace("ldaps://", ""), self.port)
        }
    }

    pub fn get_user_search_filter(&self, username: &str) -> Result<String> {
        // Validate and sanitize username to prevent LDAP injection
        let sanitized_username = InputValidationService::sanitize_ldap_value(username)?;
        Ok(self.user_search_filter.replace("{username}", &sanitized_username))
    }


}

// Secure Debug implementation that doesn't expose sensitive data
impl fmt::Debug for LdapConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LdapConfig")
            .field("enabled", &self.enabled)
            .field("server", &"[REDACTED]")
            .field("port", &self.port)
            .field("use_tls", &self.use_tls)
            .field("base_dn", &"[REDACTED]")
            .field("bind_dn", &"[REDACTED]")
            .field("bind_password", &"[REDACTED]")
            .field("user_search_base", &"[REDACTED]")
            .field("user_search_filter", &"[REDACTED]")
            .field("user_attribute_email", &self.user_attribute_email)
            .field("user_attribute_name", &self.user_attribute_name)
            .field("connection_timeout", &self.connection_timeout)
            .field("search_timeout", &self.search_timeout)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapUser {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub dn: String,
}

impl LdapUser {
    pub fn new(username: String, email: String, full_name: String, dn: String) -> Self {
        Self {
            username,
            email,
            full_name,
            dn,
        }
    }
}
