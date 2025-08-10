use crate::error::{Error, Result};
use crate::models::ldap_config_model::LdapConfig;
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use tracing::{debug, error, info, warn};

/// LDAP Connection Pool for efficient connection management
pub struct LdapConnectionPool {
    config: Arc<LdapConfig>,
    pool: Arc<Mutex<VecDeque<PooledConnection>>>,
    semaphore: Arc<Semaphore>,
    // max_connections: usize,
    max_idle_time: Duration,
}

struct PooledConnection {
    connection: Ldap,
    created_at: Instant,
    last_used: Instant,
}

impl LdapConnectionPool {
    /// new instance ldap connection pool
    #[must_use] pub fn new(config: LdapConfig, max_connections: usize) -> Self {
        Self {
            config: Arc::new(config),
            pool: Arc::new(Mutex::new(VecDeque::new())),
            semaphore: Arc::new(Semaphore::new(max_connections)),
            // max_connections,
            max_idle_time: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Get a connection from the pool or create a new one
    pub async fn get_connection(&self) -> Result<PooledLdapConnection> {
        // Acquire semaphore permit to limit concurrent connections
        let permit = self.semaphore.clone().acquire_owned().await.map_err(|_| {
            Error::LdapConnectionError("Failed to acquire connection permit".to_string())
        })?;

        // Try to get a connection from the pool
        let mut pool = self.pool.lock().await;

        // Clean up expired connections
        self.cleanup_expired_connections(&mut pool).await;

        if let Some(pooled_conn) = pool.pop_front() {
            debug!("Reusing pooled LDAP connection");
            return Ok(PooledLdapConnection {
                connection: pooled_conn.connection,
                // pool: self.pool.clone(),
                _permit: permit,
            });
        }

        drop(pool); // Release lock before creating new connection

        // Create new connection
        debug!("Creating new LDAP connection");
        let connection = self.create_new_connection().await?;

        Ok(PooledLdapConnection {
            connection,
            // pool: self.pool.clone(),
            _permit: permit,
        })
    }

    async fn create_new_connection(&self) -> Result<Ldap> {
        let ldap_url = self.config.get_ldap_url();

        let settings = LdapConnSettings::new()
            .set_conn_timeout(Duration::from_secs(self.config.connection_timeout))
            .set_starttls(self.config.use_tls);

        let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &ldap_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to LDAP server: {}", e);
                Error::LdapConnectionError("Connection failed".to_string())
            })?;

        // Start the connection
        ldap3::drive!(conn);

        // Bind with service account
        ldap.simple_bind(&self.config.bind_dn, &self.config.bind_password)
            .await
            .map_err(|e| {
                error!("Failed to bind to LDAP server: {}", e);
                Error::LdapAuthenticationError("Bind failed".to_string())
            })?;

        info!("Successfully created and bound new LDAP connection");
        Ok(ldap)
    }

    async fn cleanup_expired_connections(&self, pool: &mut VecDeque<PooledConnection>) {
        let now = Instant::now();
        let mut expired_count = 0;

        pool.retain(|conn| {
            let is_expired = now.duration_since(conn.last_used) > self.max_idle_time;
            let is_too_old = now.duration_since(conn.created_at) > Duration::from_secs(3600); // 1 hour max lifetime
            if is_expired || is_too_old {
                expired_count += 1;
            }
            !is_expired && !is_too_old
        });

        if expired_count > 0 {
            debug!("Cleaned up {} expired LDAP connections", expired_count);
        }
    }

    // /// Get pool statistics for monitoring
    // pub async fn get_stats(&self) -> PoolStats {
    //     let pool = self.pool.lock().await;
    //     PoolStats {
    //         active_connections: self.max_connections - self.semaphore.available_permits(),
    //         idle_connections: pool.len(),
    //         max_connections: self.max_connections,
    //     }
    // }
}

/// A connection borrowed from the pool
pub struct PooledLdapConnection {
    connection: Ldap,
    // pool: Arc<Mutex<VecDeque<PooledConnection>>>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl PooledLdapConnection {
    /// Get mutable reference to the underlying LDAP connection
    pub fn as_mut(&mut self) -> &mut Ldap {
        &mut self.connection
    }

    // /// Get pool statistics (for monitoring purposes)
    // pub async fn pool_size(&self) -> usize {
    //     self.pool.lock().await.len()
    // }
}

impl Drop for PooledLdapConnection {
    fn drop(&mut self) {
        // Note: Connection will be dropped naturally when this struct is dropped
        // The pool reference is used for potential future connection return logic
        // For now, we let connections drop to avoid complexity in the Drop trait
        debug!("Dropping LDAP connection from pool");
    }
}


// /// pool stats
// #[derive(Debug, Clone)]
// pub struct PoolStats {
//     /// active connections
//     pub active_connections: usize,
//     /// idle connections
//     pub idle_connections: usize,
//     /// max connections
//     pub max_connections: usize,
// }

/// Rate limiter for authentication attempts
pub struct AuthRateLimiter {
    attempts: Arc<Mutex<std::collections::HashMap<String, Vec<Instant>>>>,
    max_attempts: usize,
    window_duration: Duration,
}

impl AuthRateLimiter {
    /// new instance for auth rate limiter
    #[must_use] pub fn new(max_attempts: usize, window_duration: Duration) -> Self {
        Self {
            attempts: Arc::new(Mutex::new(std::collections::HashMap::new())),
            max_attempts,
            window_duration,
        }
    }

    /// Check if authentication attempt is allowed for the given identifier
    pub async fn is_allowed(&self, identifier: &str) -> bool {
        let mut attempts = self.attempts.lock().await;
        let now = Instant::now();

        // Clean up old attempts
        let cutoff = now.checked_sub(self.window_duration).unwrap();

        let user_attempts = attempts
            .entry(identifier.to_string())
            .or_insert_with(Vec::new);
        user_attempts.retain(|&attempt_time| attempt_time > cutoff);

        // Check if under limit
        if user_attempts.len() >= self.max_attempts {
            warn!("Rate limit exceeded for identifier: {}", identifier);
            return false;
        }

        // Record this attempt
        user_attempts.push(now);
        true
    }

    // /// Get remaining attempts for an identifier
    // pub async fn remaining_attempts(&self, identifier: &str) -> usize {
    //     let attempts = self.attempts.lock().await;
    //     let now = Instant::now();
    //     let cutoff = now - self.window_duration;

    //     if let Some(user_attempts) = attempts.get(identifier) {
    //         let recent_attempts = user_attempts.iter().filter(|&&t| t > cutoff).count();
    //         self.max_attempts.saturating_sub(recent_attempts)
    //     } else {
    //         self.max_attempts
    //     }
    // }
}
