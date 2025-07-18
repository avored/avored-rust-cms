use crate::error::{Error, Result};
use crate::models::admin_user_model::{AdminUserModel, CreatableAdminUserModel};
use crate::models::ldap_config_model::{LdapConfig, LdapUser};
use crate::providers::auth_provider::{AuthProvider, AuthenticationResult};
use crate::providers::avored_database_provider::DB;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::services::ldap_connection_pool::{AuthRateLimiter, LdapConnectionPool};
use crate::services::security_monitoring_service::SecurityMonitoringService;
use async_trait::async_trait;
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, Scope, SearchEntry};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

pub struct LdapAuthService {
    config: Arc<LdapConfig>,
    admin_user_repository: AdminUserRepository,
    connection_pool: Arc<LdapConnectionPool>,
    rate_limiter: Arc<AuthRateLimiter>,
    security_monitor: Arc<SecurityMonitoringService>,
}

impl LdapAuthService {
    pub fn new(config: LdapConfig, admin_user_repository: AdminUserRepository) -> Self {
        let config_arc = Arc::new(config);
        let connection_pool = Arc::new(LdapConnectionPool::new((*config_arc).clone(), 10)); // Max 10 connections
        let rate_limiter = Arc::new(AuthRateLimiter::new(5, Duration::from_secs(300))); // 5 attempts per 5 minutes
        let security_monitor = Arc::new(SecurityMonitoringService::new());

        Self {
            config: config_arc,
            admin_user_repository,
            connection_pool,
            rate_limiter,
            security_monitor,
        }
    }

    pub async fn create_ldap_connection(&self) -> Result<Ldap> {
        let ldap_url = self.config.get_ldap_url();

        let settings = LdapConnSettings::new()
            .set_conn_timeout(Duration::from_secs(self.config.connection_timeout));

        let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &ldap_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to LDAP server {}: {}", ldap_url, e);
                Error::LdapConnectionError(format!("Connection failed: {}", e))
            })?;

        // Start the connection
        ldap3::drive!(conn);

        // Bind with service account
        ldap.simple_bind(&self.config.bind_dn, &self.config.bind_password)
            .await
            .map_err(|e| {
                error!(
                    "Failed to bind to LDAP server with DN {}: {}",
                    self.config.bind_dn, e
                );
                Error::LdapAuthenticationError(format!("Bind failed: {}", e))
            })?;

        info!("Successfully connected and bound to LDAP server");
        Ok(ldap)
    }

    async fn search_user(&self, ldap: &mut Ldap, username: &str) -> Result<Option<LdapUser>> {
        // Validate and sanitize username input
        if username.is_empty() || username.len() > 256 {
            return Err(Error::InvalidArgument(
                "Invalid username format".to_string(),
            ));
        }

        let search_filter = self.config.get_user_search_filter(username)?;
        let search_base = &self.config.user_search_base;

        debug!("Searching for user in LDAP directory"); // Don't log username or filter for security

        let search_result = ldap
            .search(
                search_base,
                Scope::Subtree,
                &search_filter,
                vec![
                    &self.config.user_attribute_email,
                    &self.config.user_attribute_name,
                    "uid",
                    "cn",
                    "sAMAccountName", // For Active Directory
                ],
            )
            .await;

        let search_result = match search_result {
            Ok(result) => result,
            Err(e) => {
                error!("LDAP search failed: {}", e);
                return Err(Error::LdapSearchError("Search failed".to_string()));
            }
        };

        let rs = search_result.0; // Extract the Vec<ResultEntry> from SearchResult

        if rs.is_empty() {
            debug!("User not found in LDAP directory");
            return Ok(None);
        }

        if rs.len() > 1 {
            warn!("Multiple users found in LDAP search, using first result");
        }

        let entry = SearchEntry::construct(rs[0].clone());
        let dn = entry.dn.clone();

        // Extract email with validation
        let email = entry
            .attrs
            .get(&self.config.user_attribute_email)
            .and_then(|v| v.first())
            .map(|e| e.clone())
            .unwrap_or_else(|| {
                // Create a valid email from username if no email attribute found
                if username.contains('@') {
                    username.to_string()
                } else {
                    format!("{}@ldap.local", username)
                }
            });

        // Extract full name
        let full_name = entry
            .attrs
            .get(&self.config.user_attribute_name)
            .and_then(|v| v.first())
            .or_else(|| entry.attrs.get("cn").and_then(|v| v.first()))
            .unwrap_or(&username.to_string())
            .clone();

        debug!("Successfully found user in LDAP directory");

        Ok(Some(LdapUser::new(
            username.to_string(),
            email,
            full_name,
            dn,
        )))
    }

    async fn authenticate_user(&self, ldap_user: &LdapUser, password: &str) -> Result<bool> {
        let start_time = Instant::now();

        let ldap_url = self.config.get_ldap_url();

        let settings = LdapConnSettings::new()
            .set_conn_timeout(Duration::from_secs(self.config.connection_timeout));

        let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &ldap_url)
            .await
            .map_err(|e| {
                error!(
                    "Failed to connect to LDAP server for user authentication: {}",
                    e
                );
                Error::LdapConnectionError("Connection failed".to_string())
            })?;

        ldap3::drive!(conn);

        // Try to bind with user credentials
        let auth_result = match ldap.simple_bind(&ldap_user.dn, password).await {
            Ok(_) => {
                debug!("User authenticated successfully via LDAP");
                true
            }
            Err(_) => {
                debug!("LDAP authentication failed");
                false
            }
        };

        // Prevent timing attacks by ensuring consistent response time
        let min_duration = Duration::from_millis(100);
        let elapsed = start_time.elapsed();
        if elapsed < min_duration {
            tokio::time::sleep(min_duration - elapsed).await;
        }

        Ok(auth_result)
    }

    async fn sync_user_to_local_db(&self, ldap_user: &LdapUser, db: &DB) -> Result<AdminUserModel> {
        // First, try to find existing user by email
        match self
            .admin_user_repository
            .find_by_email(&db.0, &db.1, &ldap_user.email)
            .await
        {
            Ok(existing_user) => {
                info!("Found existing local user for LDAP authentication");
                Ok(existing_user)
            }
            Err(_) => {
                // User doesn't exist locally, create new user
                info!("Creating new local user for LDAP authentication");

                let creatable_user = CreatableAdminUserModel {
                    full_name: ldap_user.full_name.clone(),
                    email: ldap_user.email.clone(),
                    password: "".to_string(), // Empty password for LDAP users
                    profile_image: "".to_string(),
                    is_super_admin: false, // LDAP users are not super admins by default
                    logged_in_username: "system".to_string(),
                };

                self.admin_user_repository
                    .create_admin_user(&db.0, &db.1, creatable_user)
                    .await
                    .map_err(|e| {
                        error!("Failed to create local user for LDAP authentication: {}", e);
                        e
                    })
            }
        }
    }
}

#[async_trait]
impl AuthProvider for LdapAuthService {
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
        db: &DB,
    ) -> Result<AuthenticationResult> {
        let start_time = Instant::now();

        if !self.config.enabled {
            return Ok(AuthenticationResult::Failed(
                "LDAP authentication is disabled".to_string(),
            ));
        }

        // Input validation
        if username.is_empty() || password.is_empty() {
            return Ok(AuthenticationResult::Failed(
                "Username and password are required".to_string(),
            ));
        }

        if username.len() > 256 || password.len() > 1024 {
            return Ok(AuthenticationResult::Failed(
                "Invalid credentials format".to_string(),
            ));
        }

        // Rate limiting check
        if !self.rate_limiter.is_allowed(username).await {
            warn!("Rate limit exceeded for authentication attempt");
            self.security_monitor.record_rate_limit_exceeded(username).await;
            return Ok(AuthenticationResult::Failed(
                "Too many authentication attempts. Please try again later.".to_string(),
            ));
        }

        // Get connection from pool
        let mut pooled_conn = match self.connection_pool.get_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to get LDAP connection from pool: {}", e);
                return Ok(AuthenticationResult::Failed(
                    "Authentication service unavailable".to_string(),
                ));
            }
        };

        let ldap = pooled_conn.as_mut();

        // Search for user in LDAP
        let ldap_user = match self.search_user(ldap, username).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                // Ensure consistent timing even for non-existent users
                let min_duration = Duration::from_millis(100);
                let elapsed = start_time.elapsed();
                if elapsed < min_duration {
                    tokio::time::sleep(min_duration - elapsed).await;
                }
                return Ok(AuthenticationResult::UserNotFound);
            }
            Err(e) => {
                error!("LDAP user search failed: {}", e);
                return Ok(AuthenticationResult::Failed(
                    "Authentication failed".to_string(),
                ));
            }
        };

        // Authenticate user with LDAP
        let is_authenticated = match self.authenticate_user(&ldap_user, password).await {
            Ok(auth_result) => auth_result,
            Err(e) => {
                error!("LDAP authentication error: {}", e);
                return Ok(AuthenticationResult::Failed(
                    "Authentication failed".to_string(),
                ));
            }
        };

        if !is_authenticated {
            self.security_monitor.record_authentication_attempt(false, "ldap").await;
            return Ok(AuthenticationResult::Failed(
                "Invalid credentials".to_string(),
            ));
        }

        // Sync user to local database
        match self.sync_user_to_local_db(&ldap_user, db).await {
            Ok(admin_user) => {
                self.security_monitor.record_authentication_attempt(true, "ldap").await;
                Ok(AuthenticationResult::Success(admin_user))
            },
            Err(e) => {
                error!("Failed to sync LDAP user to local database: {}", e);
                self.security_monitor.record_authentication_attempt(false, "ldap").await;
                Ok(AuthenticationResult::Failed(
                    "User synchronization failed".to_string(),
                ))
            }
        }
    }

    fn provider_name(&self) -> &'static str {
        "ldap"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
