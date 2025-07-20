use std::collections::BTreeMap;
use std::time::SystemTime;
use prost_types::Timestamp;
use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAuditModel {
    pub id: String,
    pub security_audit_id: String,
    pub admin_user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub endpoint: Option<String>,
    pub request_method: Option<String>,
    pub total_authentication_attempts: i32,
    pub failed_authentication_attempts: i32,
    pub blocked_injection_attempts: i32,
    pub rate_limited_requests: i32,
    pub suspicious_activities_detected: i32,
    pub security_violations: i32,
    pub uptime_seconds: i32,
    pub security_health_score: f64,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreateSecurityAuditModel {
    pub security_audit_id: String,
    pub admin_user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub endpoint: Option<String>,
    pub request_method: Option<String>,
    pub total_authentication_attempts: Option<i32>,
    pub failed_authentication_attempts: Option<i32>,
    pub blocked_injection_attempts: Option<i32>,
    pub rate_limited_requests: Option<i32>,
    pub suspicious_activities_detected: Option<i32>,
    pub security_violations: Option<i32>,
    pub uptime_seconds: Option<i32>,
    pub security_health_score: Option<f64>,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdateSecurityAuditModel {
    pub total_authentication_attempts: Option<i32>,
    pub failed_authentication_attempts: Option<i32>,
    pub blocked_injection_attempts: Option<i32>,
    pub rate_limited_requests: Option<i32>,
    pub suspicious_activities_detected: Option<i32>,
    pub security_violations: Option<i32>,
    pub uptime_seconds: Option<i32>,
    pub security_health_score: Option<f64>,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAuditPaginationModel {
    pub data: Vec<SecurityAuditModel>,
    pub pagination: Pagination,
}

impl TryFrom<Object> for SecurityAuditModel {
    type Error = Error;

    fn try_from(val: Object) -> Result<SecurityAuditModel> {
        let id = val.get("id").get_id()?;

        let security_audit_id = match val.get("security_audit_id") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("security_audit_id field is not a string".to_string())),
        };

        let admin_user_id = match val.get("admin_user_id") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("admin_user_id field is not a string or null".to_string())),
        };

        let session_id = match val.get("session_id") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("session_id field is not a string or null".to_string())),
        };

        let ip_address = match val.get("ip_address") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("ip_address field is not a string".to_string())),
        };

        let user_agent = match val.get("user_agent") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("user_agent field is not a string or null".to_string())),
        };

        let endpoint = match val.get("endpoint") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("endpoint field is not a string or null".to_string())),
        };

        let request_method = match val.get("request_method") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("request_method field is not a string or null".to_string())),
        };

        let total_authentication_attempts = match val.get("total_authentication_attempts") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let failed_authentication_attempts = match val.get("failed_authentication_attempts") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let blocked_injection_attempts = match val.get("blocked_injection_attempts") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let rate_limited_requests = match val.get("rate_limited_requests") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let suspicious_activities_detected = match val.get("suspicious_activities_detected") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let security_violations = match val.get("security_violations") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let uptime_seconds = match val.get("uptime_seconds") {
            Some(Value::Number(val)) => val.as_int() as i32,
            _ => 0,
        };

        let security_health_score = match val.get("security_health_score") {
            Some(Value::Number(val)) => val.as_float(),
            _ => 100.0,
        };

        let created_at = match val.get("created_at") {
            Some(Value::Datetime(val)) => val.clone(),
            _ => return Err(Error::Generic("created_at field is not a datetime".to_string())),
        };

        let updated_at = match val.get("updated_at") {
            Some(Value::Datetime(val)) => val.clone(),
            _ => return Err(Error::Generic("updated_at field is not a datetime".to_string())),
        };

        let metadata = match val.get("metadata") {
            Some(Value::Object(obj)) => {
                let mut map = BTreeMap::new();
                for (key, value) in obj.iter() {
                    map.insert(key.clone(), value.clone());
                }
                Some(map)
            },
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("metadata field is not an object or null".to_string())),
        };

        Ok(SecurityAuditModel {
            id,
            security_audit_id,
            admin_user_id,
            session_id,
            ip_address,
            user_agent,
            endpoint,
            request_method,
            total_authentication_attempts,
            failed_authentication_attempts,
            blocked_injection_attempts,
            rate_limited_requests,
            suspicious_activities_detected,
            security_violations,
            uptime_seconds,
            security_health_score,
            created_at,
            updated_at,
            metadata,
        })
    }
}

impl SecurityAuditModel {
    /// Validates IP address format
    pub fn validate_ip_address(ip: &str) -> bool {
        if ip == "unknown" {
            return true;
        }
        
        // Basic IPv4 validation
        if ip.split('.').count() == 4 {
            return ip.split('.').all(|part| {
                part.parse::<u8>().is_ok()
            });
        }
        
        // Basic IPv6 validation (simplified)
        if ip.contains(':') {
            return ip.chars().all(|c| c.is_ascii_hexdigit() || c == ':');
        }
        
        false
    }

    /// Calculates security health score based on metrics
    pub fn calculate_health_score(&self) -> f64 {
        let mut score = 100.0;
        
        // Deduct points for security issues
        score -= (self.failed_authentication_attempts as f64) * 2.0;
        score -= (self.blocked_injection_attempts as f64) * 5.0;
        score -= (self.suspicious_activities_detected as f64) * 3.0;
        score -= (self.security_violations as f64) * 4.0;
        
        // Ensure score doesn't go below 0
        score.max(0.0).min(100.0)
    }

    /// Converts to protobuf Timestamp
    pub fn created_at_timestamp(&self) -> Timestamp {
        let chrono_utc = self.created_at.to_utc();
        let system_time = SystemTime::from(chrono_utc);
        let duration = system_time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        Timestamp {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    /// Converts to protobuf Timestamp
    pub fn updated_at_timestamp(&self) -> Timestamp {
        let chrono_utc = self.updated_at.to_utc();
        let system_time = SystemTime::from(chrono_utc);
        let duration = system_time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        Timestamp {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }
}

impl CreateSecurityAuditModel {
    /// Validates the create model
    pub fn validate(&self) -> Result<()> {
        if self.security_audit_id.is_empty() {
            return Err(Error::Generic("security_audit_id cannot be empty".to_string()));
        }

        if self.ip_address.is_empty() {
            return Err(Error::Generic("ip_address cannot be empty".to_string()));
        }

        if !SecurityAuditModel::validate_ip_address(&self.ip_address) {
            return Err(Error::Generic("invalid ip_address format".to_string()));
        }

        if let Some(score) = self.security_health_score {
            if score < 0.0 || score > 100.0 {
                return Err(Error::Generic("security_health_score must be between 0.0 and 100.0".to_string()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use surrealdb::sql::{Datetime, Object, Value};

    #[test]
    fn test_validate_ip_address() {
        // Valid IPv4 addresses
        assert!(SecurityAuditModel::validate_ip_address("192.168.1.1"));
        assert!(SecurityAuditModel::validate_ip_address("127.0.0.1"));
        assert!(SecurityAuditModel::validate_ip_address("255.255.255.255"));
        assert!(SecurityAuditModel::validate_ip_address("0.0.0.0"));

        // Valid special case
        assert!(SecurityAuditModel::validate_ip_address("unknown"));

        // Valid IPv6 addresses (simplified validation)
        assert!(SecurityAuditModel::validate_ip_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(SecurityAuditModel::validate_ip_address("::1"));

        // Invalid addresses
        assert!(!SecurityAuditModel::validate_ip_address("256.1.1.1"));
        assert!(!SecurityAuditModel::validate_ip_address("192.168.1"));
        assert!(!SecurityAuditModel::validate_ip_address("invalid"));
        assert!(!SecurityAuditModel::validate_ip_address(""));
    }

    #[test]
    fn test_calculate_health_score() {
        let mut audit = SecurityAuditModel {
            failed_authentication_attempts: 0,
            blocked_injection_attempts: 0,
            suspicious_activities_detected: 0,
            security_violations: 0,
            ..Default::default()
        };

        // Perfect score
        assert_eq!(audit.calculate_health_score(), 100.0);

        // Deduct points for failed auth attempts (2 points each)
        audit.failed_authentication_attempts = 5;
        assert_eq!(audit.calculate_health_score(), 90.0);

        // Deduct points for injection attempts (5 points each)
        audit.blocked_injection_attempts = 2;
        assert_eq!(audit.calculate_health_score(), 80.0);

        // Deduct points for suspicious activities (3 points each)
        audit.suspicious_activities_detected = 3;
        assert_eq!(audit.calculate_health_score(), 71.0);

        // Deduct points for security violations (4 points each)
        audit.security_violations = 1;
        assert_eq!(audit.calculate_health_score(), 67.0);

        // Score should not go below 0
        audit.failed_authentication_attempts = 100;
        assert_eq!(audit.calculate_health_score(), 0.0);
    }

    #[test]
    fn test_create_security_audit_model_validation() {
        // Valid model
        let valid_model = CreateSecurityAuditModel {
            security_audit_id: "audit_123".to_string(),
            ip_address: "192.168.1.1".to_string(),
            security_health_score: Some(85.5),
            ..Default::default()
        };
        assert!(valid_model.validate().is_ok());

        // Empty security_audit_id
        let invalid_model = CreateSecurityAuditModel {
            security_audit_id: "".to_string(),
            ip_address: "192.168.1.1".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Empty ip_address
        let invalid_model = CreateSecurityAuditModel {
            security_audit_id: "audit_123".to_string(),
            ip_address: "".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Invalid ip_address
        let invalid_model = CreateSecurityAuditModel {
            security_audit_id: "audit_123".to_string(),
            ip_address: "invalid_ip".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Invalid security_health_score (too high)
        let invalid_model = CreateSecurityAuditModel {
            security_audit_id: "audit_123".to_string(),
            ip_address: "192.168.1.1".to_string(),
            security_health_score: Some(150.0),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Invalid security_health_score (negative)
        let invalid_model = CreateSecurityAuditModel {
            security_audit_id: "audit_123".to_string(),
            ip_address: "192.168.1.1".to_string(),
            security_health_score: Some(-10.0),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());
    }
}
