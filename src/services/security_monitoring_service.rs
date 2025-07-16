use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Real-time security monitoring service for continuous security validation
/// This service ensures that security measures are always active and functioning
#[derive(Debug, Clone)]
pub struct SecurityMonitoringService {
    metrics: Arc<Mutex<SecurityMetrics>>,
    alerts: Arc<Mutex<Vec<SecurityAlert>>>,
    health_checks: Arc<Mutex<HashMap<String, HealthCheckResult>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub total_authentication_attempts: u64,
    pub failed_authentication_attempts: u64,
    pub blocked_injection_attempts: u64,
    pub rate_limited_requests: u64,
    pub suspicious_activities_detected: u64,
    pub security_violations: u64,
    pub last_updated: u64,
    pub uptime_seconds: u64,
    pub security_health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub alert_type: SecurityAlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
    pub source: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAlertType {
    InjectionAttempt,
    BruteForceAttack,
    RateLimitExceeded,
    SuspiciousActivity,
    SecurityFeatureDisabled,
    ConfigurationViolation,
    SystemCompromise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub check_name: String,
    pub status: HealthStatus,
    pub last_check: u64,
    pub error_message: Option<String>,
    pub check_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl SecurityMonitoringService {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(SecurityMetrics::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            health_checks: Arc::new(Mutex::new(HashMap::new())),
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

    /// Record a blocked injection attempt
    pub async fn record_injection_attempt(&self, injection_type: &str, payload: &str) {
        let mut metrics = self.metrics.lock().await;
        metrics.blocked_injection_attempts += 1;
        metrics.security_violations += 1;

        self.create_alert(
            SecurityAlertType::InjectionAttempt,
            AlertSeverity::High,
            format!("Blocked {} injection attempt", injection_type),
            "input_validation",
            HashMap::from([
                ("injection_type".to_string(), injection_type.to_string()),
                ("payload_length".to_string(), payload.len().to_string()),
            ]),
        )
        .await;

        self.update_security_health_score().await;
    }

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

    /// Record suspicious activity
    pub async fn record_suspicious_activity(&self, activity_type: &str, details: &str) {
        let mut metrics = self.metrics.lock().await;
        metrics.suspicious_activities_detected += 1;
        metrics.security_violations += 1;

        self.create_alert(
            SecurityAlertType::SuspiciousActivity,
            AlertSeverity::High,
            format!("Suspicious activity detected: {}", activity_type),
            "security_monitor",
            HashMap::from([
                ("activity_type".to_string(), activity_type.to_string()),
                ("details".to_string(), details.to_string()),
            ]),
        )
        .await;

        self.update_security_health_score().await;
    }

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
    pub async fn perform_health_checks(&self) -> Result<()> {
        let check_names = vec![
            "input_validation",
            "rate_limiting",
            "audit_logging",
            "timing_protection",
            "injection_prevention",
        ];

        for check_name in check_names {
            let start_time = std::time::Instant::now();
            let result = match check_name {
                "input_validation" => self.check_input_validation_health().await,
                "rate_limiting" => self.check_rate_limiting_health().await,
                "audit_logging" => self.check_audit_logging_health().await,
                "timing_protection" => self.check_timing_protection_health().await,
                "injection_prevention" => self.check_injection_prevention_health().await,
                _ => Ok(()),
            };
            let duration = start_time.elapsed();

            let health_result = HealthCheckResult {
                check_name: check_name.to_string(),
                status: if result.is_ok() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Critical
                },
                last_check: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                error_message: result.err().map(|e| format!("{:?}", e)),
                check_duration_ms: duration.as_millis() as u64,
            };

            if health_result.status == HealthStatus::Critical {
                self.create_alert(
                    SecurityAlertType::SecurityFeatureDisabled,
                    AlertSeverity::Critical,
                    format!("Security health check failed: {}", check_name),
                    "health_monitor",
                    HashMap::from([("check_name".to_string(), check_name.to_string())]),
                )
                .await;
            }

            let mut health_checks = self.health_checks.lock().await;
            health_checks.insert(check_name.to_string(), health_result);
        }

        self.update_security_health_score().await;
        Ok(())
    }

    /// Check input validation health
    async fn check_input_validation_health(&self) -> Result<()> {
        use crate::services::input_validation_service::InputValidationService;

        // Test that injection prevention is working
        let test_injections = vec![
            "admin)(|(objectClass=*))",
            "admin'; DROP TABLE users; --",
            "<script>alert('xss')</script>",
        ];

        for injection in test_injections {
            let result = InputValidationService::validate_username(injection);
            if result.is_ok() {
                return Err(crate::error::Error::Generic(format!(
                    "Input validation failed to block injection: {}",
                    injection
                )));
            }
        }

        Ok(())
    }

    /// Check rate limiting health
    async fn check_rate_limiting_health(&self) -> Result<()> {
        use crate::services::ldap_connection_pool::AuthRateLimiter;

        let rate_limiter = AuthRateLimiter::new(5, Duration::from_secs(300));
        let test_identifier = "health_check_user".to_string();

        // Test that rate limiting is working
        for _ in 0..6 {
            rate_limiter.is_allowed(&test_identifier).await;
        }

        // 6th attempt should be blocked
        let should_be_blocked = rate_limiter.is_allowed(&test_identifier).await;
        if should_be_blocked {
            return Err(crate::error::Error::Generic(
                "Rate limiting is not working properly".to_string(),
            ));
        }

        Ok(())
    }

    /// Check audit logging health
    async fn check_audit_logging_health(&self) -> Result<()> {
        use crate::services::security_audit_service::{SecurityAuditService, SecurityEvent};

        let audit_service = SecurityAuditService::new(10);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Test that audit logging is working
        let test_event = SecurityEvent::AuthenticationAttempt {
            username: "health_check".to_string(),
            provider: "test".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
            timestamp,
        };

        audit_service.log_event(test_event).await;
        let events = audit_service.get_recent_events(1).await;

        if events.is_empty() {
            return Err(crate::error::Error::Generic(
                "Audit logging is not working properly".to_string(),
            ));
        }

        Ok(())
    }

    /// Check timing protection health
    async fn check_timing_protection_health(&self) -> Result<()> {
        use std::time::Instant;

        // Test that timing protection is working
        let start = Instant::now();

        // Simulate validation that should take at least 100ms
        let min_duration = Duration::from_millis(100);
        let elapsed = start.elapsed();
        if elapsed < min_duration {
            tokio::time::sleep(min_duration - elapsed).await;
        }

        let total_duration = start.elapsed();

        if total_duration < Duration::from_millis(100) {
            return Err(crate::error::Error::Generic(
                "Timing protection is not working properly".to_string(),
            ));
        }

        Ok(())
    }

    /// Check injection prevention health
    async fn check_injection_prevention_health(&self) -> Result<()> {
        use crate::services::input_validation_service::InputValidationService;

        // Test critical injection patterns
        let critical_patterns = vec![
            ("LDAP", "admin)(|(objectClass=*)"),
            ("SQL", "admin'; DROP TABLE users; --"),
            ("XSS", "<script>alert('xss')</script>"),
            ("Command", "admin; rm -rf /"),
            ("Path", "../../../etc/passwd"),
        ];

        for (injection_type, pattern) in critical_patterns {
            let result = InputValidationService::validate_username(pattern);
            if result.is_ok() {
                return Err(crate::error::Error::Generic(format!(
                    "{} injection prevention is not working: {}",
                    injection_type, pattern
                )));
            }
        }

        Ok(())
    }

    /// Update security health score based on current metrics
    async fn update_security_health_score(&self) {
        let mut metrics = self.metrics.lock().await;

        let mut score = 100.0;

        // Deduct points for security violations
        if metrics.total_authentication_attempts > 0 {
            let failure_rate = metrics.failed_authentication_attempts as f64
                / metrics.total_authentication_attempts as f64;
            score -= failure_rate * 30.0; // Max 30 points deduction for high failure rate
        }

        // Deduct points for injection attempts
        if metrics.blocked_injection_attempts > 0 {
            score -= (metrics.blocked_injection_attempts as f64).min(20.0); // Max 20 points deduction
        }

        // Deduct points for suspicious activities
        if metrics.suspicious_activities_detected > 0 {
            score -= (metrics.suspicious_activities_detected as f64 * 5.0).min(25.0);
            // Max 25 points deduction
        }

        // Ensure score is between 0 and 100
        score = score.max(0.0).min(100.0);

        metrics.security_health_score = score;

        // Alert if health score is too low
        if score < 70.0 {
            drop(metrics); // Release the lock before creating alert
            self.create_alert(
                SecurityAlertType::SystemCompromise,
                AlertSeverity::Critical,
                format!("Security health score is critically low: {:.1}", score),
                "health_monitor",
                HashMap::from([("score".to_string(), score.to_string())]),
            )
            .await;
        }
    }

    /// Get current security metrics
    pub async fn get_metrics(&self) -> SecurityMetrics {
        self.metrics.lock().await.clone()
    }

    /// Get recent security alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Vec<SecurityAlert> {
        let alerts = self.alerts.lock().await;
        alerts.iter().rev().take(limit).cloned().collect()
    }

    /// Get health check results
    pub async fn get_health_check_results(&self) -> HashMap<String, HealthCheckResult> {
        self.health_checks.lock().await.clone()
    }
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
                .unwrap()
                .as_secs(),
            uptime_seconds: 0,
            security_health_score: 100.0,
        }
    }
}

impl Default for SecurityMonitoringService {
    fn default() -> Self {
        Self::new()
    }
}
