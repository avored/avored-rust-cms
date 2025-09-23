use super::BaseModel;
use crate::error::{Error, Result};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;
use surrealdb::sql::{Datetime, Object, Value};


/// Model representing a security audit entry
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAuditModel {

    /// Unique identifier for the security audit entry
    pub id: String,

    /// Unique identifier for the security audit
    pub security_audit_id: String,

    /// Optional identifier for the admin user associated with the audit
    pub admin_user_id: Option<String>,

    /// Optional session identifier for the audit
    pub session_id: Option<String>,

    /// IP address from which the request originated
    pub ip_address: String,

    /// Optional user agent string from the request
    pub user_agent: Option<String>,

    /// Optional endpoint that was accessed during the audit
    pub endpoint: Option<String>,

    /// Optional HTTP request method (e.g., GET, POST)
    pub request_method: Option<String>,
    /// Total number of authentication attempts recorded
    pub total_authentication_attempts: i32,

    /// Number of failed authentication attempts
    pub failed_authentication_attempts: i32,

    /// Number of blocked injection attempts detected
    pub blocked_injection_attempts: i32,

    /// Number of requests that were rate-limited
    pub rate_limited_requests: i32,

    /// Number of suspicious activities detected during the audit
    pub suspicious_activities_detected: i32,

    /// Number of security violations recorded
    pub security_violations: i32,

    /// Uptime in seconds for the system during the audit period
    pub uptime_seconds: i32,

    /// Security health score calculated based on various metrics
    pub security_health_score: f64,

    /// Timestamp when the audit entry was created
    pub created_at: Datetime,

    /// Timestamp when the audit entry was last updated
    pub updated_at: Datetime,

    /// Optional metadata associated with the security audit entry
    pub metadata: Option<BTreeMap<String, Value>>,
}

// /// Model for creating a new security audit entry
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct CreateSecurityAuditModel {
//     /// Unique identifier for the security audit
//     pub security_audit_id: String,

//     /// Optional identifier for the admin user associated with the audit
//     pub admin_user_id: Option<String>,

//     /// session identifier for the audit
//     pub session_id: Option<String>,
//     /// IP address from which the request originated
//     pub ip_address: String,

//     /// Optional user agent string from the request
//     pub user_agent: Option<String>,

//     /// Optional endpoint that was accessed during the audit
//     pub endpoint: Option<String>,

//     /// Optional HTTP request method (e.g., GET, POST)
//     pub request_method: Option<String>,

//     /// Total number of authentication attempts recorded
//     pub total_authentication_attempts: Option<i32>,

//     /// Number of failed authentication attempts
//     pub failed_authentication_attempts: Option<i32>,

//     /// Number of blocked injection attempts detected
//     pub blocked_injection_attempts: Option<i32>,

//     /// Number of requests that were rate-limited
//     pub rate_limited_requests: Option<i32>,

//     /// Number of suspicious activities detected during the audit
//     pub suspicious_activities_detected: Option<i32>,

//     /// Number of security violations recorded
//     pub security_violations: Option<i32>,

//     /// Uptime in seconds for the system during the audit period
//     pub uptime_seconds: Option<i32>,
//     /// Security health score calculated based on various metrics
//     pub security_health_score: Option<f64>,

//     /// Timestamp when the audit entry was created   
//     pub metadata: Option<BTreeMap<String, Value>>,
// }

// /// Model for updating an existing security audit entry
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct UpdateSecurityAuditModel {
//     /// Unique identifier for the security audit entry
//     pub total_authentication_attempts: Option<i32>,
//     /// Optional identifier for the admin user associated with the audit
//     pub failed_authentication_attempts: Option<i32>,

//     /// Optional session identifier for the audit
//     pub blocked_injection_attempts: Option<i32>,

//     /// IP address from which the request originated
//     pub rate_limited_requests: Option<i32>,
//     /// Optional user agent string from the request
//     pub suspicious_activities_detected: Option<i32>,
//     /// Optional endpoint that was accessed during the audit
//     pub security_violations: Option<i32>,
//     /// Optional HTTP request method (e.g., GET, POST)
//     pub uptime_seconds: Option<i32>,
//     /// Timestamp when the audit entry was created
//     pub security_health_score: Option<f64>,
//     /// Timestamp when the audit entry was last updated
//     pub metadata: Option<BTreeMap<String, Value>>,
// }

// /// Model for paginated security audit results
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct SecurityAuditPaginationModel {

//     /// List of security audit entries
//     pub data: Vec<SecurityAuditModel>,
//     /// Pagination information for the results
//     pub pagination: Pagination,
// }

impl TryFrom<Object> for SecurityAuditModel {
    type Error = Error;

    fn try_from(val: Object) -> Result<Self> {
        let id = val.get("id").get_id()?;

        let security_audit_id = match val.get("security_audit_id") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => {
                return Err(Error::Generic(
                    "security_audit_id field is not a string".to_string(),
                ))
            }
        };

        let admin_user_id = match val.get("admin_user_id") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "admin_user_id field is not a string or null".to_string(),
                ))
            }
        };

        let session_id = match val.get("session_id") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "session_id field is not a string or null".to_string(),
                ))
            }
        };

        let ip_address = match val.get("ip_address") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => {
                return Err(Error::Generic(
                    "ip_address field is not a string".to_string(),
                ))
            }
        };

        let user_agent = match val.get("user_agent") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "user_agent field is not a string or null".to_string(),
                ))
            }
        };

        let endpoint = match val.get("endpoint") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "endpoint field is not a string or null".to_string(),
                ))
            }
        };

        let request_method = match val.get("request_method") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "request_method field is not a string or null".to_string(),
                ))
            }
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
            _ => {
                return Err(Error::Generic(
                    "created_at field is not a datetime".to_string(),
                ))
            }
        };

        let updated_at = match val.get("updated_at") {
            Some(Value::Datetime(val)) => val.clone(),
            _ => {
                return Err(Error::Generic(
                    "updated_at field is not a datetime".to_string(),
                ))
            }
        };

        let metadata = match val.get("metadata") {
            Some(Value::Object(obj)) => {
                let mut map = BTreeMap::new();
                for (key, value) in obj.iter() {
                    map.insert(key.clone(), value.clone());
                }
                Some(map)
            }
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "metadata field is not an object or null".to_string(),
                ))
            }
        };

        Ok(Self {
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
    // /// Validates IP address format
    // #[must_use] pub fn validate_ip_address(ip: &str) -> bool {
    //     if ip == "unknown" {
    //         return true;
    //     }

    //     // Basic IPv4 validation
    //     if ip.split('.').count() == 4 {
    //         return ip.split('.').all(|part| part.parse::<u8>().is_ok());
    //     }

    //     // Basic IPv6 validation (simplified)
    //     if ip.contains(':') {
    //         return ip.chars().all(|c| c.is_ascii_hexdigit() || c == ':');
    //     }

    //     false
    // }

    /// Calculates security health score based on metrics
    // pub fn calculate_health_score(&self) -> f64 {
    //     let mut score = 100.0;

    //     // Deduct points for security issues
    //     score -= (self.failed_authentication_attempts as f64) * 2.0;
    //     score -= (self.blocked_injection_attempts as f64) * 5.0;
    //     score -= (self.suspicious_activities_detected as f64) * 3.0;
    //     score -= (self.security_violations as f64) * 4.0;

    //     // Ensure score doesn't go below 0
    //     score.max(0.0).min(100.0)
    // }

    /// Converts to protobuf Timestamp
    #[must_use] pub fn created_at_timestamp(&self) -> Timestamp {
        let chrono_utc = self.created_at.to_utc();
        let system_time = SystemTime::from(chrono_utc);
        let duration = system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        Timestamp {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    /// Converts to protobuf Timestamp
    #[must_use] pub fn updated_at_timestamp(&self) -> Timestamp {
        let chrono_utc = self.updated_at.to_utc();
        let system_time = SystemTime::from(chrono_utc);
        let duration = system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        Timestamp {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }
}

// impl CreateSecurityAuditModel {
    // /// Validates the create model
    // pub fn validate(&self) -> Result<()> {
    //     if self.security_audit_id.is_empty() {
    //         return Err(Error::Generic(
    //             "security_audit_id cannot be empty".to_string(),
    //         ));
    //     }

    //     if self.ip_address.is_empty() {
    //         return Err(Error::Generic("ip_address cannot be empty".to_string()));
    //     }

    //     if !SecurityAuditModel::validate_ip_address(&self.ip_address) {
    //         return Err(Error::Generic("invalid ip_address format".to_string()));
    //     }

    //     if let Some(score) = self.security_health_score {
    //         if !(0.0..=100.0).contains(&score) {
    //             return Err(Error::Generic(
    //                 "security_health_score must be between 0.0 and 100.0".to_string(),
    //             ));
    //         }
    //     }

    //     Ok(())
    // }
// }

// gRPC Conversions
impl TryFrom<SecurityAuditModel> for crate::api::proto::security_audit::SecurityAuditModel {
    type Error = Error;

    fn try_from(model: SecurityAuditModel) -> Result<Self> {
        let metadata_json = if let Some(ref metadata) = model.metadata {
            Some(
                serde_json::to_string(&metadata)
                    .map_err(|e| Error::Generic(format!("Failed to serialize metadata: {e}")))?,
            )
        } else {
            None
        };

        let created_at_timestamp = model.created_at_timestamp();
        let updated_at_timestamp = model.updated_at_timestamp();

        Ok(Self {
            id: model.id,
            security_audit_id: model.security_audit_id,
            admin_user_id: model.admin_user_id,
            session_id: model.session_id,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            endpoint: model.endpoint,
            request_method: model.request_method,
            total_authentication_attempts: model.total_authentication_attempts,
            failed_authentication_attempts: model.failed_authentication_attempts,
            blocked_injection_attempts: model.blocked_injection_attempts,
            rate_limited_requests: model.rate_limited_requests,
            suspicious_activities_detected: model.suspicious_activities_detected,
            security_violations: model.security_violations,
            uptime_seconds: model.uptime_seconds,
            security_health_score: model.security_health_score,
            created_at: Some(created_at_timestamp),
            updated_at: Some(updated_at_timestamp),
            metadata_json,
        })
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_validate_ip_address() {
    //     // Valid IPv4 addresses
    //     assert!(SecurityAuditModel::validate_ip_address("192.168.1.1"));
    //     assert!(SecurityAuditModel::validate_ip_address("127.0.0.1"));
    //     assert!(SecurityAuditModel::validate_ip_address("255.255.255.255"));
    //     assert!(SecurityAuditModel::validate_ip_address("0.0.0.0"));

    //     // Valid special case
    //     assert!(SecurityAuditModel::validate_ip_address("unknown"));

    //     // Valid IPv6 addresses (simplified validation)
    //     assert!(SecurityAuditModel::validate_ip_address(
    //         "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
    //     ));
    //     assert!(SecurityAuditModel::validate_ip_address("::1"));

    //     // Invalid addresses
    //     assert!(!SecurityAuditModel::validate_ip_address("256.1.1.1"));
    //     assert!(!SecurityAuditModel::validate_ip_address("192.168.1"));
    //     assert!(!SecurityAuditModel::validate_ip_address("invalid"));
    //     assert!(!SecurityAuditModel::validate_ip_address(""));
    // }

    // #[test]
    // fn test_calculate_health_score() {
    //     let mut audit = SecurityAuditModel {
    //         failed_authentication_attempts: 0,
    //         blocked_injection_attempts: 0,
    //         suspicious_activities_detected: 0,
    //         security_violations: 0,
    //         ..Default::default()
    //     };

    //     // Perfect score
    //     assert_eq!(audit.calculate_health_score(), 100.0);

    //     // Deduct points for failed auth attempts (2 points each)
    //     audit.failed_authentication_attempts = 5;
    //     assert_eq!(audit.calculate_health_score(), 90.0);

    //     // Deduct points for injection attempts (5 points each)
    //     audit.blocked_injection_attempts = 2;
    //     assert_eq!(audit.calculate_health_score(), 80.0);

    //     // Deduct points for suspicious activities (3 points each)
    //     audit.suspicious_activities_detected = 3;
    //     assert_eq!(audit.calculate_health_score(), 71.0);

    //     // Deduct points for security violations (4 points each)
    //     audit.security_violations = 1;
    //     assert_eq!(audit.calculate_health_score(), 67.0);

    //     // Score should not go below 0
    //     audit.failed_authentication_attempts = 100;
    //     assert_eq!(audit.calculate_health_score(), 0.0);
    // }

    // #[test]
    // fn test_create_security_audit_model_validation() {
    //     // Valid model
    //     let valid_model = CreateSecurityAuditModel {
    //         security_audit_id: "audit_123".to_string(),
    //         ip_address: "192.168.1.1".to_string(),
    //         security_health_score: Some(85.5),
    //         ..Default::default()
    //     };
    //     assert!(valid_model.validate().is_ok());

    //     // Empty security_audit_id
    //     let invalid_model = CreateSecurityAuditModel {
    //         security_audit_id: "".to_string(),
    //         ip_address: "192.168.1.1".to_string(),
    //         ..Default::default()
    //     };
    //     assert!(invalid_model.validate().is_err());

    //     // Empty ip_address
    //     let invalid_model = CreateSecurityAuditModel {
    //         security_audit_id: "audit_123".to_string(),
    //         ip_address: "".to_string(),
    //         ..Default::default()
    //     };
    //     assert!(invalid_model.validate().is_err());

    //     // Invalid ip_address
    //     let invalid_model = CreateSecurityAuditModel {
    //         security_audit_id: "audit_123".to_string(),
    //         ip_address: "invalid_ip".to_string(),
    //         ..Default::default()
    //     };
    //     assert!(invalid_model.validate().is_err());

    //     // Invalid security_health_score (too high)
    //     let invalid_model = CreateSecurityAuditModel {
    //         security_audit_id: "audit_123".to_string(),
    //         ip_address: "192.168.1.1".to_string(),
    //         security_health_score: Some(150.0),
    //         ..Default::default()
    //     };
    //     assert!(invalid_model.validate().is_err());

    //     // Invalid security_health_score (negative)
    //     let invalid_model = CreateSecurityAuditModel {
    //         security_audit_id: "audit_123".to_string(),
    //         ip_address: "192.168.1.1".to_string(),
    //         security_health_score: Some(-10.0),
    //         ..Default::default()
    //     };
    //     assert!(invalid_model.validate().is_err());
    // }
}
