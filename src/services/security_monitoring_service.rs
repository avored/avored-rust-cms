use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{error, info, warn};


/// Represents the result of a health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// The status of the health check
    pub status: HealthStatus,
    /// A message describing the result of the health check
    pub message: String,

    /// The timestamp of when the health check was performed
    pub timestamp: u64,

    /// The name of the health check
    pub check_name: String,

    /// The last time the health check was performed
    pub last_check: u64,

    /// An optional error message if the health check failed
    pub error_message: Option<String>,

    /// The duration of the health check in milliseconds
    pub check_duration_ms: u64,
}

/// Represents the result of a threat assessment
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatAssessment {
    /// The identifier for the threat assessment
    pub threat_level: ThreatLevel,
    /// The risk score associated with the threat
    pub risk_score: u8,
    /// A description of the threat
    pub recommended_actions: Vec<String>,
    /// Actions recommended to mitigate the threat
    pub affected_resources: Vec<String>,
}

/// Represents the level of threat
#[derive(Debug, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// No threat detected
    Low,
    /// Potential threat detected
    Medium,
    /// Significant threat detected
    High,
    /// Critical threat detected
    Critical,
}

/// Real-time security monitoring service for continuous security validation
/// This service ensures that security measures are always active and functioning
#[derive(Debug, Clone)]
pub struct ThreatDetector {
    // /// Track failed authentication attempts
    // pub failed_auth_attempts: HashMap<String, FailedAuthTracker>,
    // /// Track suspicious patterns such as injection attempts and unusual endpoints
    // pub suspicious_patterns: HashMap<String, SuspiciousActivityTracker>,

    // /// Track rate limiting events
    // pub rate_limit_tracking: HashMap<String, RateLimitTracker>,
}

// /// Represents an authentication attempt tracker
// #[derive(Debug, Clone)]
// pub struct AuthAttemptTracker {
//     /// The number of failed authentication attempts
//     pub count: u32,
//     /// The timestamp of the first authentication attempt
//     pub last_attempt: u64,
// }

// /// Represents a tracker for suspicious patterns
// #[derive(Debug, Clone)]
// pub struct SuspiciousPatternTracker {
//     /// The number of injection attempts detected
//     pub injection_attempts: u32,

//     /// A list of unusual endpoints that have been accessed
//     pub unusual_endpoints: Vec<String>,
// }


/// Represents a tracker for rate limiting events
#[derive(Debug, Clone)]
pub struct RateLimitTracker {
    /// The number of requests made within the rate limit window
    pub requests: u32,

    /// window start  
    pub window_start: std::time::Instant,
}

/// Failed auth tracker
#[derive(Debug, Clone)]
pub struct FailedAuthTracker {
    /// count failed auth tracker
    pub count: u32,
    /// first attempt for failed auth tracker
    pub first_attempt: std::time::Instant,
    /// last attempt for failed auth tracker
    pub last_attempt: std::time::Instant,
}


/// suspicious activity tracker
#[derive(Debug, Clone)]
pub struct SuspiciousActivityTracker {
    /// injection attempts 
    pub injection_attempts: u32,

    /// un usual endpoins 
    pub unusual_endpoints: Vec<String>,

    /// last activity
    pub last_activity: std::time::Instant,
}

/// security monitoring services
#[derive(Debug, Clone)]
pub struct SecurityMonitoringService {
    metrics: Arc<Mutex<SecurityMetrics>>,
    alerts: Arc<Mutex<Vec<SecurityAlert>>>,
    // health_checks: Arc<Mutex<HashMap<String, HealthCheckResult>>>,
    // threat_detector: Arc<RwLock<ThreatDetector>>,
}

/// security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {

    /// total authentication attempts
    pub total_authentication_attempts: u64,
    /// failed authentication attempts
    pub failed_authentication_attempts: u64,
    /// blocked injection attempts
    pub blocked_injection_attempts: u64,
    /// rate limited requests
    pub rate_limited_requests: u64,

    /// suspicious activities detected
    pub suspicious_activities_detected: u64,
    /// security violations
    pub security_violations: u64,
    /// last updated
    pub last_updated: u64,
    /// uptime seconds
    pub uptime_seconds: u64,

    /// security health score
    pub security_health_score: f64,
    /// Total events last hours
    pub total_events_last_hour: u32,
    /// failed auth attempts last hours
    pub failed_auth_attempts_last_hour: u32,

    /// injection attempts last hour
    pub injection_attempts_last_hour: u32,

    /// active threats
    pub active_threats: u32,

    /// list of high risk ips
    pub high_risk_ips: Vec<String>,
}

/// security alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    /// alert type
    pub alert_type: SecurityAlertType,
    /// severity 
    pub severity: AlertSeverity,

    /// message
    pub message: String,
    /// timestamps
    pub timestamp: u64,

    /// source
    pub source: String,

    /// meta data
    pub metadata: HashMap<String, String>,
}

/// security alert type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAlertType {
    /// inection attempts
    InjectionAttempt,

    /// brute force attack
    BruteForceAttack,

    /// rate limit exceeded
    RateLimitExceeded,

    /// suspicious activity
    SuspiciousActivity,

    /// security feature disabled
    SecurityFeatureDisabled,
    /// configuration violations
    ConfigurationViolation,

    /// system compromise
    SystemCompromise,
}


/// alert security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// low alert severity
    Low,

    /// medioum alert severity
    Medium,
    
    /// high alert severity
    High,

    /// critical alert severity
    Critical,
}

/// health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// healthy heath status
    Healthy,
    /// warning heath status
    Warning,
    /// critical heath status
    Critical,
    /// unknown heath status
    Unknown,
}

impl ThreatDetector {

    /// the threat detector create new instance
    pub fn new() -> Self {
        Self {
            // failed_auth_attempts: HashMap::new(),
            // suspicious_patterns: HashMap::new(),
            // rate_limit_tracking: HashMap::new(),
        }
    }

    // /// Clean up tracking data older than 1 hour
    // pub fn cleanup_old_data(&mut self) {
    //     let one_hour_ago = std::time::Instant::now() - Duration::from_secs(3600);

    //     self.failed_auth_attempts
    //         .retain(|_, tracker| tracker.last_attempt > one_hour_ago);

    //     self.suspicious_patterns
    //         .retain(|_, tracker| tracker.last_activity > one_hour_ago);

    //     self.rate_limit_tracking
    //         .retain(|_, tracker| tracker.window_start > one_hour_ago);
    // }
}

impl SecurityMonitoringService {
    /// created new instance for Security monitoring service
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(SecurityMetrics::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            // health_checks: Arc::new(Mutex::new(HashMap::new())),
            // threat_detector: Arc::new(RwLock::new(ThreatDetector::new())),
        }
    }

    /// Record an authentication attempt
    pub async fn record_authentication_attempt(&self, success: bool, provider: &str) {
        let mut metrics = self.metrics.lock().await;
        metrics.total_authentication_attempts += 1;

        if !success {
            metrics.failed_authentication_attempts += 1;

            // Alert if failure rate is too high
            let failure_rate = metrics.failed_authentication_attempts as f64
                / metrics.total_authentication_attempts as f64;
            if failure_rate > 0.5 && metrics.total_authentication_attempts > 10 {
                self.create_alert(
                    SecurityAlertType::SuspiciousActivity,
                    AlertSeverity::High,
                    format!(
                        "High authentication failure rate: {:.2}%",
                        failure_rate * 100.0
                    ),
                    "authentication_monitor",
                    HashMap::from([("provider".to_string(), provider.to_string())]),
                )
                .await;
            }
        }

        self.update_security_health_score().await;
    }

    // /// Record a blocked injection attempt
    // pub async fn record_injection_attempt(&self, injection_type: &str, payload: &str) {
    //     let mut metrics = self.metrics.lock().await;
    //     metrics.blocked_injection_attempts += 1;
    //     metrics.security_violations += 1;

    //     // Create alert for injection attempt
    //     let alert = SecurityAlert {
    //         alert_type: SecurityAlertType::InjectionAttempt,
    //         severity: AlertSeverity::High,
    //         message: format!("Blocked {} injection attempt", injection_type),
    //         source: "input_validation".to_string(),
    //         metadata: HashMap::from([
    //             ("injection_type".to_string(), injection_type.to_string()),
    //             ("payload_length".to_string(), payload.len().to_string()),
    //         ]),
    //         timestamp: SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_secs(),
    //     };

    //     self.alerts.lock().await.push(alert);

    //     self.update_security_health_score().await;
    // }

    /// Record a rate limiting event
    pub async fn record_rate_limit_exceeded(&self, identifier: &str) {
        let mut metrics = self.metrics.lock().await;
        metrics.rate_limited_requests += 1;

        self.create_alert(
            SecurityAlertType::RateLimitExceeded,
            AlertSeverity::Medium,
            format!("Rate limit exceeded for identifier: {}", identifier),
            "rate_limiter",
            HashMap::from([("identifier".to_string(), identifier.to_string())]),
        )
        .await;
    }

    // /// Record suspicious activity
    // pub async fn record_suspicious_activity(&self, activity_type: &str, details: &str) {
    //     let mut metrics = self.metrics.lock().await;
    //     metrics.suspicious_activities_detected += 1;
    //     metrics.security_violations += 1;

    //     self.create_alert(
    //         SecurityAlertType::SuspiciousActivity,
    //         AlertSeverity::High,
    //         format!("Suspicious activity detected: {}", activity_type),
    //         "security_monitor",
    //         HashMap::from([
    //             ("activity_type".to_string(), activity_type.to_string()),
    //             ("details".to_string(), details.to_string()),
    //         ]),
    //     )
    //     .await;

    //     self.update_security_health_score().await;
    // }

    /// Create a security alert
    async fn create_alert(
        &self,
        alert_type: SecurityAlertType,
        severity: AlertSeverity,
        message: String,
        source: &str,
        metadata: HashMap<String, String>,
    ) {
        let alert = SecurityAlert {
            alert_type,
            severity: severity.clone(),
            message: message.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source: source.to_string(),
            metadata,
        };

        let mut alerts = self.alerts.lock().await;
        alerts.push(alert);

        // Keep only the last 1000 alerts
        if alerts.len() > 1000 {
            let excess = alerts.len() - 1000;
            alerts.drain(0..excess);
        }

        // Log the alert
        match severity {
            AlertSeverity::Critical => error!("CRITICAL SECURITY ALERT: {}", message),
            AlertSeverity::High => error!("HIGH SECURITY ALERT: {}", message),
            AlertSeverity::Medium => warn!("MEDIUM SECURITY ALERT: {}", message),
            AlertSeverity::Low => info!("LOW SECURITY ALERT: {}", message),
        }
    }

    /// Perform security health checks
    // pub async fn perform_health_checks(&self) -> Result<()> {
    //     let check_names = vec![
    //         "input_validation",
    //         "rate_limiting",
    //         "audit_logging",
    //         "timing_protection",
    //         "injection_prevention",
    //     ];

    //     for check_name in check_names {
    //         let start_time = std::time::Instant::now();
    //         let result = match check_name {
    //             "input_validation" => self.check_input_validation_health().await,
    //             "rate_limiting" => self.check_rate_limiting_health().await,
    //             "audit_logging" => self.check_audit_logging_health().await,
    //             "timing_protection" => self.check_timing_protection_health().await,
    //             "injection_prevention" => self.check_injection_prevention_health().await,
    //             _ => Ok(()),
    //         };
    //         let duration = start_time.elapsed();

    //         let health_result = HealthCheckResult {
    //             check_name: check_name.to_string(),
    //             status: if result.is_ok() {
    //                 HealthStatus::Healthy
    //             } else {
    //                 HealthStatus::Critical
    //             },
    //             message: if result.is_ok() {
    //                 format!("{} check passed", check_name)
    //             } else {
    //                 format!("{} check failed", check_name)
    //             },
    //             timestamp: SystemTime::now()
    //                 .duration_since(UNIX_EPOCH)
    //                 .unwrap()
    //                 .as_secs(),
    //             last_check: SystemTime::now()
    //                 .duration_since(UNIX_EPOCH)
    //                 .unwrap()
    //                 .as_secs(),
    //             error_message: result.err().map(|e| format!("{:?}", e)),
    //             check_duration_ms: duration.as_millis() as u64,
    //         };

    //         if health_result.status == HealthStatus::Critical {
    //             self.create_alert(
    //                 SecurityAlertType::SecurityFeatureDisabled,
    //                 AlertSeverity::Critical,
    //                 format!("Security health check failed: {}", check_name),
    //                 "health_monitor",
    //                 HashMap::from([("check_name".to_string(), check_name.to_string())]),
    //             )
    //             .await;
    //         }

    //         let mut health_checks = self.health_checks.lock().await;
    //         health_checks.insert(check_name.to_string(), health_result);
    //     }

    //     self.update_security_health_score().await;
    //     Ok(())
    // }

    /// Check input validation health
    // async fn check_input_validation_health(&self) -> Result<()> {
    //     use crate::services::input_validation_service::InputValidationService;

    //     // Test that injection prevention is working
    //     let test_injections = vec![
    //         "admin)(|(objectClass=*))",
    //         "admin'; DROP TABLE users; --",
    //         "<script>alert('xss')</script>",
    //     ];

    //     for injection in test_injections {
    //         let result = InputValidationService::validate_username(injection);
    //         if result.is_ok() {
    //             return Err(crate::error::Error::Generic(format!(
    //                 "Input validation failed to block injection: {}",
    //                 injection
    //             )));
    //         }
    //     }

    //     Ok(())
    // }

    /// Update threat tracking data structures
    // async fn update_threat_tracking(
    //     &self,
    //     ip_address: &str,
    //     event_type: &SecurityEventType,
    // ) -> Result<()> {
    //     let mut detector = self.threat_detector.write().await;
    //     let now = std::time::Instant::now();

    //     match event_type {
    //         SecurityEventType::AuthenticationFailure => {
    //             let tracker = detector
    //                 .failed_auth_attempts
    //                 .entry(ip_address.to_string())
    //                 .or_insert(FailedAuthTracker {
    //                     count: 0,
    //                     first_attempt: now,
    //                     last_attempt: now,
    //                 });

    //             tracker.count += 1;
    //             tracker.last_attempt = now;
    //         }
    //         SecurityEventType::InjectionAttempt => {
    //             let tracker = detector
    //                 .suspicious_patterns
    //                 .entry(ip_address.to_string())
    //                 .or_insert(SuspiciousActivityTracker {
    //                     injection_attempts: 0,
    //                     unusual_endpoints: Vec::new(),
    //                     last_activity: now,
    //                 });

    //             tracker.injection_attempts += 1;
    //             tracker.last_activity = now;
    //         }
    //         _ => {}
    //     }

    //     // Clean up old tracking data (older than 1 hour)
    //     detector.cleanup_old_data();

    //     Ok(())
    // }

    /// Check rate limiting health
    // async fn check_rate_limiting_health(&self) -> Result<()> {
    //     use crate::services::ldap_connection_pool::AuthRateLimiter;

    //     let rate_limiter = AuthRateLimiter::new(5, Duration::from_secs(300));
    //     let test_identifier = "health_check_user".to_string();

    //     // Test that rate limiting is working
    //     for _ in 0..6 {
    //         rate_limiter.is_allowed(&test_identifier).await;
    //     }

    //     // 6th attempt should be blocked
    //     let should_be_blocked = rate_limiter.is_allowed(&test_identifier).await;
    //     if should_be_blocked {
    //         return Err(crate::error::Error::Generic(
    //             "Rate limiting is not working properly".to_string(),
    //         ));
    //     }

    //     Ok(())
    // }

    /// Get current security metrics
    // pub async fn get_security_metrics(&self, db: &DB) -> Result<SecurityMetrics> {
    //     let detector = self.threat_detector.read().await;

    //     // Calculate metrics from current tracking data
    //     let failed_auth_attempts_last_hour = detector
    //         .failed_auth_attempts
    //         .values()
    //         .map(|tracker| tracker.count)
    //         .sum();

    //     let injection_attempts_last_hour = detector
    //         .suspicious_patterns
    //         .values()
    //         .map(|tracker| tracker.injection_attempts)
    //         .sum();

    //     let active_threats =
    //         detector.failed_auth_attempts.len() as u32 + detector.suspicious_patterns.len() as u32;

    //     let high_risk_ips = detector
    //         .failed_auth_attempts
    //         .iter()
    //         .filter(|(_, tracker)| tracker.count > 5)
    //         .map(|(ip, _)| ip.clone())
    //         .collect();

    //     // Calculate security health score
    //     let security_health_score = self.calculate_security_health_score(
    //         failed_auth_attempts_last_hour,
    //         injection_attempts_last_hour,
    //         active_threats,
    //     );

    //     Ok(SecurityMetrics {
    //         total_authentication_attempts: 0,
    //         failed_authentication_attempts: failed_auth_attempts_last_hour as u64,
    //         blocked_injection_attempts: injection_attempts_last_hour as u64,
    //         rate_limited_requests: 0,
    //         suspicious_activities_detected: 0,
    //         security_violations: 0,
    //         last_updated: SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_secs(),
    //         uptime_seconds: 0,
    //         security_health_score,
    //         total_events_last_hour: failed_auth_attempts_last_hour + injection_attempts_last_hour,
    //         failed_auth_attempts_last_hour,
    //         injection_attempts_last_hour,
    //         active_threats,
    //         high_risk_ips,
    //     })
    // }

    // fn calculate_security_health_score(
    //     &self,
    //     failed_auth: u32,
    //     injection_attempts: u32,
    //     active_threats: u32,
    // ) -> f64 {
    //     let mut score = 100.0;

    //     // Deduct points for security issues
    //     score -= (failed_auth as f64) * 2.0;
    //     score -= (injection_attempts as f64) * 10.0;
    //     score -= (active_threats as f64) * 5.0;

    //     score.max(0.0).min(100.0)
    // }

    // /// Check audit logging health
    // async fn check_audit_logging_health(&self) -> Result<()> {
    //     // Simple health check - just verify the service can be created
    //     let audit_service = SecurityAuditService::new(SecurityAuditRepository::new());

    //     // Test basic functionality by checking if we can create a test audit record
    //     // In a real implementation, this would test actual logging capabilities
    //     info!("Audit logging health check passed");

    //     Ok(())
    // }

    // /// Check timing protection health
    // async fn check_timing_protection_health(&self) -> Result<()> {
    //     use std::time::Instant;

    //     // Test that timing protection is working
    //     let start = Instant::now();

    //     // Simulate validation that should take at least 100ms
    //     let min_duration = Duration::from_millis(100);
    //     let elapsed = start.elapsed();
    //     if elapsed < min_duration {
    //         tokio::time::sleep(min_duration - elapsed).await;
    //     }

    //     let total_duration = start.elapsed();

    //     if total_duration < Duration::from_millis(100) {
    //         return Err(crate::error::Error::Generic(
    //             "Timing protection is not working properly".to_string(),
    //         ));
    //     }

    //     Ok(())
    // }

    // /// Check injection prevention health
    // async fn check_injection_prevention_health(&self) -> Result<()> {
    //     // Simple health check for injection prevention
    //     info!("Injection prevention health check passed");
    //     Ok(())
    // }

    /// Create a security alert

    /// Get current security metrics
    // pub async fn get_metrics(&self) -> SecurityMetrics {
    //     self.metrics.lock().await.clone()
    // }

    // /// Get recent security alerts
    // pub async fn get_recent_alerts(&self, limit: usize) -> Vec<SecurityAlert> {
    //     let alerts = self.alerts.lock().await;
    //     alerts.iter().rev().take(limit).cloned().collect()
    // }

    // /// Get health check results
    // pub async fn get_health_check_results(&self) -> HashMap<String, HealthCheckResult> {
    //     self.health_checks.lock().await.clone()
    // }

    /// Update security health score based on current metrics
    async fn update_security_health_score(&self) {
        let mut metrics = self.metrics.lock().await;
        let mut score = 100.0f64;

        // Deduct points for failed authentication attempts
        if metrics.failed_authentication_attempts > 0 {
            score -= (metrics.failed_authentication_attempts as f64 * 0.5).min(20.0);
        }

        // Deduct points for injection attempts
        if metrics.blocked_injection_attempts > 0 {
            score -= (metrics.blocked_injection_attempts as f64 * 2.0).min(30.0);
        }

        // Deduct points for security violations
        if metrics.security_violations > 0 {
            score -= (metrics.security_violations as f64 * 1.5).min(25.0);
        }

        // Deduct points for suspicious activities
        if metrics.suspicious_activities_detected > 0 {
            score -= (metrics.suspicious_activities_detected as f64 * 1.0).min(15.0);
        }

        // Ensure score is between 0 and 100
        metrics.security_health_score = score.max(0.0).min(100.0);
        metrics.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    // /// Clean up old tracking data
    // async fn cleanup_old_data(&self) {
    //     let mut detector = self.AuthAttemptTracker.write().await;
    //     detector.cleanup_old_data();
    // }
}

impl SecurityMetrics {
    fn new() -> Self {
        Self {
            total_authentication_attempts: 0,
            failed_authentication_attempts: 0,
            blocked_injection_attempts: 0,
            rate_limited_requests: 0,
            suspicious_activities_detected: 0,
            security_violations: 0,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime_seconds: 0,
            security_health_score: 100.0,
            total_events_last_hour: 0,
            failed_auth_attempts_last_hour: 0,
            injection_attempts_last_hour: 0,
            active_threats: 0,
            high_risk_ips: Vec::new(),
        }
    }
}

impl Default for SecurityMonitoringService {
    fn default() -> Self {
        Self::new()
    }
}
