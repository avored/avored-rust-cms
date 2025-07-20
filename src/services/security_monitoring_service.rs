use crate::error::{Error, Result};
use crate::models::security_alert_model::{AlertType, AlertSeverity, CreateSecurityAlertModel};
use crate::models::security_audit_model::CreateSecurityAuditModel;
use crate::providers::avored_database_provider::DB;
use crate::services::security_alert_service::SecurityAlertService;
use crate::services::security_audit_service::{SecurityAuditService, SecurityEventType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Comprehensive security monitoring service that coordinates audit logging and alerting
pub struct SecurityMonitoringService {
    audit_service: SecurityAuditService,
    alert_service: SecurityAlertService,
    threat_detector: Arc<RwLock<ThreatDetector>>,
}

/// Real-time threat detection engine
#[derive(Debug)]
pub struct ThreatDetector {
    /// Track failed authentication attempts per IP
    failed_auth_attempts: HashMap<String, FailedAuthTracker>,
    /// Track suspicious activity patterns
    suspicious_patterns: HashMap<String, SuspiciousActivityTracker>,
    /// Rate limiting tracking
    rate_limit_tracking: HashMap<String, RateLimitTracker>,
}

#[derive(Debug, Clone)]
struct FailedAuthTracker {
    count: u32,
    first_attempt: std::time::Instant,
    last_attempt: std::time::Instant,
}

#[derive(Debug, Clone)]
struct SuspiciousActivityTracker {
    injection_attempts: u32,
    unusual_endpoints: Vec<String>,
    last_activity: std::time::Instant,
}

#[derive(Debug, Clone)]
struct RateLimitTracker {
    request_count: u32,
    window_start: std::time::Instant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub total_events_last_hour: u32,
    pub failed_auth_attempts_last_hour: u32,
    pub injection_attempts_last_hour: u32,
    pub active_threats: u32,
    pub high_risk_ips: Vec<String>,
    pub security_health_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub threat_level: ThreatLevel,
    pub risk_score: u8,
    pub recommended_actions: Vec<String>,
    pub affected_resources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl SecurityMonitoringService {
    pub fn new(
        audit_service: SecurityAuditService,
        alert_service: SecurityAlertService,
    ) -> Self {
        Self {
            audit_service,
            alert_service,
            threat_detector: Arc::new(RwLock::new(ThreatDetector::new())),
        }
    }

    /// Comprehensive security event processing with automatic threat detection
    pub async fn process_security_event(
        &self,
        db: &DB,
        user_id: Option<String>,
        session_id: Option<String>,
        ip_address: String,
        user_agent: Option<String>,
        endpoint: Option<String>,
        method: Option<String>,
        event_type: SecurityEventType,
        metadata: Option<std::collections::BTreeMap<String, surrealdb::sql::Value>>,
    ) -> Result<()> {
        // Log the security event
        let audit_result = self.audit_service.log_security_event(
            &db.0,
            &db.1,
            user_id.clone(),
            session_id.clone(),
            ip_address.clone(),
            user_agent.clone(),
            endpoint.clone(),
            method.clone(),
            event_type.clone(),
            metadata.clone(),
        ).await?;

        // Perform real-time threat analysis
        let threat_assessment = self.analyze_threat(
            &ip_address,
            &event_type,
            endpoint.as_deref(),
            user_id.as_deref(),
        ).await?;

        // Generate alerts based on threat assessment
        if threat_assessment.risk_score > 70 {
            self.generate_security_alert(
                db,
                &ip_address,
                &event_type,
                &threat_assessment,
                endpoint.as_deref(),
                metadata,
            ).await?;
        }

        // Update threat tracking
        self.update_threat_tracking(&ip_address, &event_type).await?;

        Ok(())
    }

    /// Analyze threat level based on current event and historical patterns
    async fn analyze_threat(
        &self,
        ip_address: &str,
        event_type: &SecurityEventType,
        endpoint: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<ThreatAssessment> {
        let detector = self.threat_detector.read().await;
        let mut risk_score = 0u8;
        let mut recommended_actions = Vec::new();
        let mut affected_resources = Vec::new();

        // Analyze failed authentication patterns
        if let Some(auth_tracker) = detector.failed_auth_attempts.get(ip_address) {
            match auth_tracker.count {
                3..=5 => {
                    risk_score += 30;
                    recommended_actions.push("Monitor IP address closely".to_string());
                }
                6..=10 => {
                    risk_score += 50;
                    recommended_actions.push("Consider temporary IP blocking".to_string());
                }
                11.. => {
                    risk_score += 80;
                    recommended_actions.push("Immediate IP blocking recommended".to_string());
                }
                _ => {}
            }
        }

        // Analyze suspicious activity patterns
        if let Some(suspicious_tracker) = detector.suspicious_patterns.get(ip_address) {
            if suspicious_tracker.injection_attempts > 0 {
                risk_score += 60;
                recommended_actions.push("Block injection attempts".to_string());
                affected_resources.extend(suspicious_tracker.unusual_endpoints.clone());
            }
        }

        // Event-specific risk assessment
        match event_type {
            SecurityEventType::AuthenticationFailure => risk_score += 10,
            SecurityEventType::InjectionAttempt => risk_score += 70,
            SecurityEventType::SuspiciousActivity => risk_score += 40,
            SecurityEventType::SecurityViolation => risk_score += 80,
            SecurityEventType::RateLimitExceeded => risk_score += 20,
            SecurityEventType::AuthenticationSuccess => {
                // Reduce risk for successful auth from known users
                if user_id.is_some() {
                    risk_score = risk_score.saturating_sub(10);
                }
            }
        }

        // Endpoint-specific risk assessment
        if let Some(endpoint) = endpoint {
            if endpoint.contains("admin") || endpoint.contains("api/auth") {
                risk_score += 20;
                affected_resources.push(endpoint.to_string());
            }
        }

        let threat_level = match risk_score {
            0..=25 => ThreatLevel::Low,
            26..=50 => ThreatLevel::Medium,
            51..=75 => ThreatLevel::High,
            76.. => ThreatLevel::Critical,
        };

        Ok(ThreatAssessment {
            threat_level,
            risk_score,
            recommended_actions,
            affected_resources,
        })
    }

    /// Generate security alerts based on threat assessment
    async fn generate_security_alert(
        &self,
        db: &DB,
        ip_address: &str,
        event_type: &SecurityEventType,
        threat_assessment: &ThreatAssessment,
        endpoint: Option<&str>,
        metadata: Option<std::collections::BTreeMap<String, surrealdb::sql::Value>>,
    ) -> Result<()> {
        let alert_type = match event_type {
            SecurityEventType::AuthenticationFailure => AlertType::AuthenticationFailure,
            SecurityEventType::InjectionAttempt => AlertType::InjectionAttempt,
            SecurityEventType::SuspiciousActivity => AlertType::SuspiciousActivity,
            SecurityEventType::SecurityViolation => AlertType::UnauthorizedAccess,
            SecurityEventType::RateLimitExceeded => AlertType::RateLimitExceeded,
            SecurityEventType::AuthenticationSuccess => return Ok(()), // No alert for success
        };

        let severity = match threat_assessment.threat_level {
            ThreatLevel::Low => AlertSeverity::Low,
            ThreatLevel::Medium => AlertSeverity::Medium,
            ThreatLevel::High => AlertSeverity::High,
            ThreatLevel::Critical => AlertSeverity::Critical,
        };

        let message = format!(
            "Security threat detected from IP {}: {:?} (Risk Score: {})",
            ip_address, event_type, threat_assessment.risk_score
        );

        let _ = self.alert_service.create_alert_auto_id(
            &db.0,
            &db.1,
            alert_type,
            severity,
            message,
            ip_address.to_string(),
            endpoint.map(|s| s.to_string()),
            metadata,
        ).await?;

        Ok(())
    }

    /// Update threat tracking data structures
    async fn update_threat_tracking(
        &self,
        ip_address: &str,
        event_type: &SecurityEventType,
    ) -> Result<()> {
        let mut detector = self.threat_detector.write().await;
        let now = std::time::Instant::now();

        match event_type {
            SecurityEventType::AuthenticationFailure => {
                let tracker = detector.failed_auth_attempts
                    .entry(ip_address.to_string())
                    .or_insert(FailedAuthTracker {
                        count: 0,
                        first_attempt: now,
                        last_attempt: now,
                    });
                
                tracker.count += 1;
                tracker.last_attempt = now;
            }
            SecurityEventType::InjectionAttempt => {
                let tracker = detector.suspicious_patterns
                    .entry(ip_address.to_string())
                    .or_insert(SuspiciousActivityTracker {
                        injection_attempts: 0,
                        unusual_endpoints: Vec::new(),
                        last_activity: now,
                    });
                
                tracker.injection_attempts += 1;
                tracker.last_activity = now;
            }
            _ => {}
        }

        // Clean up old tracking data (older than 1 hour)
        detector.cleanup_old_data();

        Ok(())
    }

    /// Get current security metrics
    pub async fn get_security_metrics(&self, db: &DB) -> Result<SecurityMetrics> {
        let detector = self.threat_detector.read().await;
        
        // Calculate metrics from current tracking data
        let failed_auth_attempts_last_hour = detector.failed_auth_attempts
            .values()
            .map(|tracker| tracker.count)
            .sum();

        let injection_attempts_last_hour = detector.suspicious_patterns
            .values()
            .map(|tracker| tracker.injection_attempts)
            .sum();

        let active_threats = detector.failed_auth_attempts.len() as u32 + 
                           detector.suspicious_patterns.len() as u32;

        let high_risk_ips = detector.failed_auth_attempts
            .iter()
            .filter(|(_, tracker)| tracker.count > 5)
            .map(|(ip, _)| ip.clone())
            .collect();

        // Calculate security health score
        let security_health_score = self.calculate_security_health_score(
            failed_auth_attempts_last_hour,
            injection_attempts_last_hour,
            active_threats,
        );

        Ok(SecurityMetrics {
            total_events_last_hour: failed_auth_attempts_last_hour + injection_attempts_last_hour,
            failed_auth_attempts_last_hour,
            injection_attempts_last_hour,
            active_threats,
            high_risk_ips,
            security_health_score,
        })
    }

    fn calculate_security_health_score(
        &self,
        failed_auth: u32,
        injection_attempts: u32,
        active_threats: u32,
    ) -> f64 {
        let mut score = 100.0;
        
        // Deduct points for security issues
        score -= (failed_auth as f64) * 2.0;
        score -= (injection_attempts as f64) * 10.0;
        score -= (active_threats as f64) * 5.0;
        
        score.max(0.0).min(100.0)
    }
}

impl ThreatDetector {
    fn new() -> Self {
        Self {
            failed_auth_attempts: HashMap::new(),
            suspicious_patterns: HashMap::new(),
            rate_limit_tracking: HashMap::new(),
        }
    }

    /// Clean up tracking data older than 1 hour
    fn cleanup_old_data(&mut self) {
        let one_hour_ago = std::time::Instant::now() - std::time::Duration::from_secs(3600);

        self.failed_auth_attempts.retain(|_, tracker| {
            tracker.last_attempt > one_hour_ago
        });

        self.suspicious_patterns.retain(|_, tracker| {
            tracker.last_activity > one_hour_ago
        });

        self.rate_limit_tracking.retain(|_, tracker| {
            tracker.window_start > one_hour_ago
        });
    }
}
