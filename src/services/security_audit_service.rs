use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Security audit events for monitoring and compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    AuthenticationAttempt {
        username: String,
        provider: String,
        success: bool,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        timestamp: u64,
    },
    RateLimitExceeded {
        identifier: String,
        ip_address: Option<IpAddr>,
        timestamp: u64,
    },
    SuspiciousActivity {
        event_type: String,
        details: String,
        ip_address: Option<IpAddr>,
        timestamp: u64,
    },
    ConfigurationChange {
        component: String,
        change_type: String,
        user: String,
        timestamp: u64,
    },
    LdapConnectionFailure {
        server: String,
        error_type: String,
        timestamp: u64,
    },
}

/// Security audit service for logging and monitoring security events
pub struct SecurityAuditService {
    events: Arc<Mutex<Vec<SecurityEvent>>>,
    max_events: usize,
    suspicious_activity_detector: Arc<Mutex<SuspiciousActivityDetector>>,
}

impl SecurityAuditService {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            max_events,
            suspicious_activity_detector: Arc::new(Mutex::new(SuspiciousActivityDetector::new())),
        }
    }

    /// Log a security event
    pub async fn log_event(&self, event: SecurityEvent) {
        // Check for suspicious activity patterns
        self.detect_suspicious_activity(&event).await;

        // Log the event
        match &event {
            SecurityEvent::AuthenticationAttempt { username, provider, success, .. } => {
                if *success {
                    info!("Authentication successful - User: {}, Provider: {}", username, provider);
                } else {
                    warn!("Authentication failed - User: {}, Provider: {}", username, provider);
                }
            }
            SecurityEvent::RateLimitExceeded { identifier, .. } => {
                warn!("Rate limit exceeded for identifier: {}", identifier);
            }
            SecurityEvent::SuspiciousActivity { event_type, details, .. } => {
                error!("Suspicious activity detected - Type: {}, Details: {}", event_type, details);
            }
            SecurityEvent::ConfigurationChange { component, change_type, user, .. } => {
                info!("Configuration changed - Component: {}, Type: {}, User: {}", component, change_type, user);
            }
            SecurityEvent::LdapConnectionFailure { server, error_type, .. } => {
                error!("LDAP connection failure - Server: {}, Error: {}", server, error_type);
            }
        }

        // Store the event
        let mut events = self.events.lock().await;
        events.push(event);

        // Maintain maximum event count
        if events.len() > self.max_events {
            events.remove(0);
        }
    }

    /// Get recent security events
    pub async fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let events = self.events.lock().await;
        events.iter().rev().take(limit).cloned().collect()
    }

    /// Get authentication statistics
    pub async fn get_auth_stats(&self, duration: Duration) -> AuthenticationStats {
        let events = self.events.lock().await;
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(duration.as_secs());

        let mut stats = AuthenticationStats::default();

        for event in events.iter() {
            if let SecurityEvent::AuthenticationAttempt { provider, success, timestamp, .. } = event {
                if *timestamp >= cutoff_time {
                    stats.total_attempts += 1;
                    if *success {
                        stats.successful_attempts += 1;
                    } else {
                        stats.failed_attempts += 1;
                    }

                    *stats.provider_stats.entry(provider.clone()).or_insert(0) += 1;
                }
            }
        }

        stats
    }

    /// Detect suspicious activity patterns
    async fn detect_suspicious_activity(&self, event: &SecurityEvent) {
        let mut detector = self.suspicious_activity_detector.lock().await;

        match event {
            SecurityEvent::AuthenticationAttempt { username, success, ip_address, timestamp, .. } => {
                if !success {
                    detector.record_failed_attempt(username.clone(), *ip_address, *timestamp);
                    
                    // Check for brute force patterns
                    if detector.is_brute_force_attack(username, *timestamp) {
                        let _suspicious_event = SecurityEvent::SuspiciousActivity {
                            event_type: "brute_force_attack".to_string(),
                            details: format!("Multiple failed authentication attempts for user: {}", username),
                            ip_address: *ip_address,
                            timestamp: *timestamp,
                        };
                        
                        // Log the suspicious activity (avoid infinite recursion by not calling log_event)
                        error!("SECURITY ALERT: Brute force attack detected for user: {}", username);
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthenticationStats {
    pub total_attempts: u64,
    pub successful_attempts: u64,
    pub failed_attempts: u64,
    pub provider_stats: HashMap<String, u64>,
}

/// Detector for suspicious activity patterns
struct SuspiciousActivityDetector {
    failed_attempts: HashMap<String, Vec<u64>>, // username -> timestamps
    ip_attempts: HashMap<IpAddr, Vec<u64>>,     // ip -> timestamps
}

impl SuspiciousActivityDetector {
    fn new() -> Self {
        Self {
            failed_attempts: HashMap::new(),
            ip_attempts: HashMap::new(),
        }
    }

    fn record_failed_attempt(&mut self, username: String, ip_address: Option<IpAddr>, timestamp: u64) {
        // Record by username
        let user_attempts = self.failed_attempts.entry(username).or_insert_with(Vec::new);
        user_attempts.push(timestamp);
        
        // Keep only recent attempts (last hour)
        let cutoff = timestamp.saturating_sub(3600);
        user_attempts.retain(|&t| t >= cutoff);

        // Record by IP address if available
        if let Some(ip) = ip_address {
            let ip_attempts = self.ip_attempts.entry(ip).or_insert_with(Vec::new);
            ip_attempts.push(timestamp);
            ip_attempts.retain(|&t| t >= cutoff);
        }
    }

    fn is_brute_force_attack(&self, username: &str, _timestamp: u64) -> bool {
        // Check if user has too many failed attempts
        if let Some(attempts) = self.failed_attempts.get(username) {
            attempts.len() >= 10 // 10 failed attempts in the last hour
        } else {
            false
        }
    }
}

/// Security metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub active_connections: usize,
    pub failed_auth_rate: f64,
    pub rate_limited_requests: u64,
    pub suspicious_activities: u64,
    pub ldap_connection_errors: u64,
}

impl SecurityAuditService {
    /// Get current security metrics
    pub async fn get_security_metrics(&self, duration: Duration) -> SecurityMetrics {
        let events = self.events.lock().await;
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(duration.as_secs());

        let mut total_auth = 0u64;
        let mut failed_auth = 0u64;
        let mut rate_limited = 0u64;
        let mut suspicious = 0u64;
        let mut ldap_errors = 0u64;

        for event in events.iter() {
            match event {
                SecurityEvent::AuthenticationAttempt { success, timestamp, .. } => {
                    if *timestamp >= cutoff_time {
                        total_auth += 1;
                        if !success {
                            failed_auth += 1;
                        }
                    }
                }
                SecurityEvent::RateLimitExceeded { timestamp, .. } => {
                    if *timestamp >= cutoff_time {
                        rate_limited += 1;
                    }
                }
                SecurityEvent::SuspiciousActivity { timestamp, .. } => {
                    if *timestamp >= cutoff_time {
                        suspicious += 1;
                    }
                }
                SecurityEvent::LdapConnectionFailure { timestamp, .. } => {
                    if *timestamp >= cutoff_time {
                        ldap_errors += 1;
                    }
                }
                _ => {}
            }
        }

        let failed_auth_rate = if total_auth > 0 {
            failed_auth as f64 / total_auth as f64
        } else {
            0.0
        };

        SecurityMetrics {
            active_connections: 0, // This would be populated from connection pool
            failed_auth_rate,
            rate_limited_requests: rate_limited,
            suspicious_activities: suspicious,
            ldap_connection_errors: ldap_errors,
        }
    }
}
