use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;
use surrealdb::sql::{Datetime, Object, Value};

/// Represents different types of security alerts
#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum AlertType {

    /// Authentication failure alert
    AuthenticationFailure,

    /// SQL injection attempt alert
    InjectionAttempt,

    /// Rate limit exceeded alert
    RateLimitExceeded,

    /// Suspicious activity alert
    SuspiciousActivity,

    /// privilege escalation alert
    PrivilegeEscalation,

    /// Data breach attempt alert
    DataBreachAttempt,

    /// Unauthorized access alert
    UnauthorizedAccess,

    /// Malformed request alert
    MalformedRequest,

    /// Brute force attack alert
    BruteForceAttack,

    /// Session hijacking alert
    SessionHijacking,
}

impl Default for AlertType {
    fn default() -> Self {
        Self::SuspiciousActivity
    }
}

impl AlertType {

    /// Converts the alert type to a string representation
    #[must_use] pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AuthenticationFailure => "authentication_failure",
            Self::InjectionAttempt => "injection_attempt",
            Self::RateLimitExceeded => "rate_limit_exceeded",
            Self::SuspiciousActivity => "suspicious_activity",
            Self::PrivilegeEscalation => "privilege_escalation",
            Self::DataBreachAttempt => "data_breach_attempt",
            Self::UnauthorizedAccess => "unauthorized_access",
            Self::MalformedRequest => "malformed_request",
            Self::BruteForceAttack => "brute_force_attack",
            Self::SessionHijacking => "session_hijacking",
        }
    }


    /// Converts a string to an `AlertType`
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "authentication_failure" => Ok(Self::AuthenticationFailure),
            "injection_attempt" => Ok(Self::InjectionAttempt),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            "suspicious_activity" => Ok(Self::SuspiciousActivity),
            "privilege_escalation" => Ok(Self::PrivilegeEscalation),
            "data_breach_attempt" => Ok(Self::DataBreachAttempt),
            "unauthorized_access" => Ok(Self::UnauthorizedAccess),
            "malformed_request" => Ok(Self::MalformedRequest),
            "brute_force_attack" => Ok(Self::BruteForceAttack),
            "session_hijacking" => Ok(Self::SessionHijacking),
            _ => Err(Error::Generic(format!("Invalid alert type: {s}"))),
        }
    }

    /// Converts a gRPC `AlertType` to an `AlertType`
    #[must_use] pub const fn from_grpc_alert_type(grpc_type: crate::api::proto::security_audit::AlertType) -> Self {
        match grpc_type {
            crate::api::proto::security_audit::AlertType::AuthenticationFailure => {
                Self::AuthenticationFailure
            }
            crate::api::proto::security_audit::AlertType::InjectionAttempt => {
                Self::InjectionAttempt
            }
            crate::api::proto::security_audit::AlertType::RateLimitExceeded => {
                Self::RateLimitExceeded
            }
            crate::api::proto::security_audit::AlertType::SuspiciousActivity => {
                Self::SuspiciousActivity
            }
            crate::api::proto::security_audit::AlertType::PrivilegeEscalation => {
                Self::PrivilegeEscalation
            }
            crate::api::proto::security_audit::AlertType::DataBreachAttempt => {
                Self::DataBreachAttempt
            }
            crate::api::proto::security_audit::AlertType::UnauthorizedAccess => {
                Self::UnauthorizedAccess
            }
            crate::api::proto::security_audit::AlertType::MalformedRequest => {
                Self::MalformedRequest
            }
            crate::api::proto::security_audit::AlertType::BruteForceAttack => {
                Self::BruteForceAttack
            }
            crate::api::proto::security_audit::AlertType::SessionHijacking => {
                Self::SessionHijacking
            }
            crate::api::proto::security_audit::AlertType::Unspecified => {
                Self::SuspiciousActivity
            } // Default fallback
        }
    }

    /// Converts the `AlertType` to a gRPC `AlertType`
    #[must_use] pub const fn to_grpc_alert_type(&self) -> crate::api::proto::security_audit::AlertType {
        match self {
            Self::AuthenticationFailure => {
                crate::api::proto::security_audit::AlertType::AuthenticationFailure
            }
            Self::InjectionAttempt => {
                crate::api::proto::security_audit::AlertType::InjectionAttempt
            }
            Self::RateLimitExceeded => {
                crate::api::proto::security_audit::AlertType::RateLimitExceeded
            }
            Self::SuspiciousActivity => {
                crate::api::proto::security_audit::AlertType::SuspiciousActivity
            }
            Self::PrivilegeEscalation => {
                crate::api::proto::security_audit::AlertType::PrivilegeEscalation
            }
            Self::DataBreachAttempt => {
                crate::api::proto::security_audit::AlertType::DataBreachAttempt
            }
            Self::UnauthorizedAccess => {
                crate::api::proto::security_audit::AlertType::UnauthorizedAccess
            }
            Self::MalformedRequest => {
                crate::api::proto::security_audit::AlertType::MalformedRequest
            }
            Self::BruteForceAttack => {
                crate::api::proto::security_audit::AlertType::BruteForceAttack
            }
            Self::SessionHijacking => {
                crate::api::proto::security_audit::AlertType::SessionHijacking
            }
        }
    }
}


/// Represents different severities of security alerts
#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum AlertSeverity {

    /// Low severity alert
    Low,

    /// Medium severity alert
    Medium,

    /// High severity alert
    High,

    /// Critical severity alert
    Critical,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        Self::Medium
    }
}

impl AlertSeverity {
    /// Converts the alert severity to a string representation
    #[must_use] pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }


    /// Converts a string to an `AlertSeverity`
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            _ => Err(Error::Generic(format!("Invalid alert severity: {s}"))),
        }
    }

    // /// Returns the priority score of the alert severity
    // pub fn priority_score(&self) -> u8 {
    //     match self {
    //         AlertSeverity::Low => 1,
    //         AlertSeverity::Medium => 2,
    //         AlertSeverity::High => 3,
    //         AlertSeverity::Critical => 4,
    //     }
    // }

    /// Converts a gRPC `AlertSeverity` to an `AlertSeverity`
    #[must_use] pub const fn from_grpc_alert_severity(
        grpc_severity: crate::api::proto::security_audit::AlertSeverity,
    ) -> Self {
        match grpc_severity {
            crate::api::proto::security_audit::AlertSeverity::Low => Self::Low,
            crate::api::proto::security_audit::AlertSeverity::Medium => Self::Medium,
            crate::api::proto::security_audit::AlertSeverity::High => Self::High,
            crate::api::proto::security_audit::AlertSeverity::Critical => Self::Critical,
            crate::api::proto::security_audit::AlertSeverity::Unspecified => Self::Medium, // Default fallback
        }
    }

    /// Converts the `AlertSeverity` to a gRPC `AlertSeverity`
    #[must_use] pub const fn to_grpc_alert_severity(&self) -> crate::api::proto::security_audit::AlertSeverity {
        match self {
            Self::Low => crate::api::proto::security_audit::AlertSeverity::Low,
            Self::Medium => crate::api::proto::security_audit::AlertSeverity::Medium,
            Self::High => crate::api::proto::security_audit::AlertSeverity::High,
            Self::Critical => crate::api::proto::security_audit::AlertSeverity::Critical,
        }
    }
}


/// Represents a security alert model used for tracking security issues
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAlertModel {
    /// Unique identifier for the security alert
    pub id: String,

    /// Unique identifier for the alert, used for API access
    pub alert_id: String,

    /// Type of the security alert
    pub alert_type: AlertType,

    /// Severity of the security alert
    pub severity: AlertSeverity,

    /// Message describing the security alert
    pub message: String,

    /// Source of the security alert (e.g., IP address, user agent) 
    pub source: String,

    /// Optional affected resource (e.g., user ID, resource ID) related to the alert
    pub affected_resource: Option<String>,

    /// Optional metadata associated with the security alert
    pub metadata: Option<BTreeMap<String, Value>>,

    /// Indicates whether the alert has been resolved
    pub resolved: bool,

    /// Optional timestamp when the alert was resolved
    pub resolved_at: Option<Datetime>,

    /// Optional identifier of the user who resolved the alert
    pub resolved_by: Option<String>,

    /// Timestamp when the alert was created
    pub created_at: Datetime,
}

/// Represents a model for creating a new security alert
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreateSecurityAlertModel {

    /// Unique identifier for the alert, used for API access
    pub alert_id: String,
    /// Type of the security alert
    pub alert_type: AlertType,
    /// Severity of the security alert
    pub severity: AlertSeverity,
    /// Message describing the security alert
    pub message: String,

    /// Source of the security alert (e.g., IP address, user agent)
    pub source: String,
    /// Optional affected resource (e.g., user ID, resource ID) related to the alert
    pub affected_resource: Option<String>,
    /// Optional metadata associated with the security alert
    pub metadata: Option<BTreeMap<String, Value>>,
}

/// Represents a model for updating an existing security alert
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdateSecurityAlertModel {
    /// Unique identifier for the security alert
    pub resolved: Option<bool>,
    /// Optional timestamp when the alert was resolved
    pub resolved_by: Option<String>,
    /// Optional identifier of the user who resolved the alert
    pub metadata: Option<BTreeMap<String, Value>>,
}

/// Represents a paginated response for security alerts
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAlertPaginationModel {

    /// List of security alerts in the current page
    pub data: Vec<SecurityAlertModel>,
    /// Pagination information for the response
    pub pagination: Pagination,
}

impl TryFrom<Object> for SecurityAlertModel {
    type Error = Error;

    fn try_from(val: Object) -> Result<Self> {
        let id = val.get("id").get_id()?;

        let alert_id = match val.get("alert_id") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("alert_id field is not a string".to_string())),
        };

        let alert_type = match val.get("alert_type") {
            Some(Value::Strand(val)) => AlertType::from_str(&val.to_string())?,
            _ => {
                return Err(Error::Generic(
                    "alert_type field is not a valid string".to_string(),
                ))
            }
        };

        let severity = match val.get("severity") {
            Some(Value::Strand(val)) => AlertSeverity::from_str(&val.to_string())?,
            _ => {
                return Err(Error::Generic(
                    "severity field is not a valid string".to_string(),
                ))
            }
        };

        let message = match val.get("message") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("message field is not a string".to_string())),
        };

        let source = match val.get("source") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("source field is not a string".to_string())),
        };

        let affected_resource = match val.get("affected_resource") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "affected_resource field is not a string or null".to_string(),
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

        let resolved = match val.get("resolved") {
            Some(Value::Bool(val)) => *val,
            _ => false,
        };

        let resolved_at = match val.get("resolved_at") {
            Some(Value::Datetime(val)) => Some(val.clone()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "resolved_at field is not a datetime or null".to_string(),
                ))
            }
        };

        let resolved_by = match val.get("resolved_by") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => {
                return Err(Error::Generic(
                    "resolved_by field is not a string or null".to_string(),
                ))
            }
        };

        let created_at = match val.get("created_at") {
            Some(Value::Datetime(val)) => val.clone(),
            _ => {
                return Err(Error::Generic(
                    "created_at field is not a datetime".to_string(),
                ))
            }
        };

        Ok(Self {
            id,
            alert_id,
            alert_type,
            severity,
            message,
            source,
            affected_resource,
            metadata,
            resolved,
            resolved_at,
            resolved_by,
            created_at,
        })
    }
}

impl SecurityAlertModel {
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

    /// Converts `resolved_at` to protobuf Timestamp
    #[must_use] pub fn resolved_at_timestamp(&self) -> Option<Timestamp> {
        self.resolved_at.as_ref().map(|dt| {
            let chrono_utc = dt.to_utc();
            let system_time = SystemTime::from(chrono_utc);
            let duration = system_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();

            Timestamp {
                seconds: duration.as_secs() as i64,
                nanos: duration.subsec_nanos() as i32,
            }
        })
    }

    // /// Checks if alert requires immediate attention
    // pub fn requires_immediate_attention(&self) -> bool {
    //     matches!(self.severity, AlertSeverity::High | AlertSeverity::Critical) && !self.resolved
    // }

    // /// Gets alert age in seconds
    // pub fn age_seconds(&self) -> i64 {
    //     let now = SystemTime::now();
    //     let chrono_utc = self.created_at.to_utc();
    //     let created = SystemTime::from(chrono_utc);
    //     now.duration_since(created).unwrap_or_default().as_secs() as i64
    // }
}

impl CreateSecurityAlertModel {
    /// Validates the create model
    pub fn validate(&self) -> Result<()> {
        if self.alert_id.is_empty() {
            return Err(Error::Generic("alert_id cannot be empty".to_string()));
        }

        if self.message.is_empty() {
            return Err(Error::Generic("message cannot be empty".to_string()));
        }

        if self.source.is_empty() {
            return Err(Error::Generic("source cannot be empty".to_string()));
        }

        Ok(())
    }

    // Creates a new alert with auto-generated ID
    // pub fn new_with_auto_id(
    //     alert_type: AlertType,
    //     severity: AlertSeverity,
    //     message: String,
    //     source: String,
    // ) -> Self {
    //     Self {
    //         alert_id: format!("alert_{}", Uuid::new_v4()),
    //         alert_type,
    //         severity,
    //         message,
    //         source,
    //         affected_resource: None,
    //         metadata: None,
    //     }
    // }
}

// gRPC Conversions
impl TryFrom<SecurityAlertModel> for crate::api::proto::security_audit::SecurityAlertModel {
    type Error = Error;

    fn try_from(model: SecurityAlertModel) -> Result<Self> {
        let metadata_json = if let Some(ref metadata) = model.metadata {
            Some(
                serde_json::to_string(&metadata)
                    .map_err(|e| Error::Generic(format!("Failed to serialize metadata: {e}")))?,
            )
        } else {
            None
        };

        let resolved_at_timestamp = model.resolved_at_timestamp();
        let created_at_timestamp = model.created_at_timestamp();

        Ok(Self {
            id: model.id,
            alert_id: model.alert_id,
            alert_type: model.alert_type.to_grpc_alert_type() as i32,
            severity: model.severity.to_grpc_alert_severity() as i32,
            message: model.message,
            source: model.source,
            affected_resource: model.affected_resource,
            metadata_json,
            resolved: model.resolved,
            resolved_at: resolved_at_timestamp,
            resolved_by: model.resolved_by,
            created_at: Some(created_at_timestamp),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_alert_type_conversions() {
        // Test as_str
        assert_eq!(
            AlertType::AuthenticationFailure.as_str(),
            "authentication_failure"
        );
        assert_eq!(AlertType::InjectionAttempt.as_str(), "injection_attempt");
        assert_eq!(AlertType::RateLimitExceeded.as_str(), "rate_limit_exceeded");
        assert_eq!(
            AlertType::SuspiciousActivity.as_str(),
            "suspicious_activity"
        );
        assert_eq!(
            AlertType::PrivilegeEscalation.as_str(),
            "privilege_escalation"
        );
        assert_eq!(AlertType::DataBreachAttempt.as_str(), "data_breach_attempt");
        assert_eq!(
            AlertType::UnauthorizedAccess.as_str(),
            "unauthorized_access"
        );
        assert_eq!(AlertType::MalformedRequest.as_str(), "malformed_request");
        assert_eq!(AlertType::BruteForceAttack.as_str(), "brute_force_attack");
        assert_eq!(AlertType::SessionHijacking.as_str(), "session_hijacking");

        // Test from_str
        assert!(matches!(
            AlertType::from_str("authentication_failure").unwrap(),
            AlertType::AuthenticationFailure
        ));
        assert!(matches!(
            AlertType::from_str("injection_attempt").unwrap(),
            AlertType::InjectionAttempt
        ));
        assert!(matches!(
            AlertType::from_str("rate_limit_exceeded").unwrap(),
            AlertType::RateLimitExceeded
        ));
        assert!(matches!(
            AlertType::from_str("suspicious_activity").unwrap(),
            AlertType::SuspiciousActivity
        ));
        assert!(matches!(
            AlertType::from_str("privilege_escalation").unwrap(),
            AlertType::PrivilegeEscalation
        ));
        assert!(matches!(
            AlertType::from_str("data_breach_attempt").unwrap(),
            AlertType::DataBreachAttempt
        ));
        assert!(matches!(
            AlertType::from_str("unauthorized_access").unwrap(),
            AlertType::UnauthorizedAccess
        ));
        assert!(matches!(
            AlertType::from_str("malformed_request").unwrap(),
            AlertType::MalformedRequest
        ));
        assert!(matches!(
            AlertType::from_str("brute_force_attack").unwrap(),
            AlertType::BruteForceAttack
        ));
        assert!(matches!(
            AlertType::from_str("session_hijacking").unwrap(),
            AlertType::SessionHijacking
        ));

        // Test invalid string
        assert!(AlertType::from_str("invalid_type").is_err());
    }

    // #[test]
    // fn test_alert_severity_conversions() {
    //     // Test as_str
    //     assert_eq!(AlertSeverity::Low.as_str(), "low");
    //     assert_eq!(AlertSeverity::Medium.as_str(), "medium");
    //     assert_eq!(AlertSeverity::High.as_str(), "high");
    //     assert_eq!(AlertSeverity::Critical.as_str(), "critical");

    //     // Test from_str
    //     assert!(matches!(
    //         AlertSeverity::from_str("low").unwrap(),
    //         AlertSeverity::Low
    //     ));
    //     assert!(matches!(
    //         AlertSeverity::from_str("medium").unwrap(),
    //         AlertSeverity::Medium
    //     ));
    //     assert!(matches!(
    //         AlertSeverity::from_str("high").unwrap(),
    //         AlertSeverity::High
    //     ));
    //     assert!(matches!(
    //         AlertSeverity::from_str("critical").unwrap(),
    //         AlertSeverity::Critical
    //     ));

    //     // Test invalid string
    //     assert!(AlertSeverity::from_str("invalid_severity").is_err());

    //     // Test priority scores
    //     assert_eq!(AlertSeverity::Low.priority_score(), 1);
    //     assert_eq!(AlertSeverity::Medium.priority_score(), 2);
    //     assert_eq!(AlertSeverity::High.priority_score(), 3);
    //     assert_eq!(AlertSeverity::Critical.priority_score(), 4);
    // }

    // #[test]
    // fn test_security_alert_model_requires_immediate_attention() {
    //     let mut alert = SecurityAlertModel {
    //         severity: AlertSeverity::Low,
    //         resolved: false,
    //         ..Default::default()
    //     };

    //     // Low severity, unresolved - no immediate attention
    //     assert!(!alert.requires_immediate_attention());

    //     // Medium severity, unresolved - no immediate attention
    //     alert.severity = AlertSeverity::Medium;
    //     assert!(!alert.requires_immediate_attention());

    //     // High severity, unresolved - requires immediate attention
    //     alert.severity = AlertSeverity::High;
    //     assert!(alert.requires_immediate_attention());

    //     // Critical severity, unresolved - requires immediate attention
    //     alert.severity = AlertSeverity::Critical;
    //     assert!(alert.requires_immediate_attention());

    //     // Critical severity, resolved - no immediate attention
    //     alert.resolved = true;
    //     assert!(!alert.requires_immediate_attention());
    // }

    #[test]
    fn test_create_security_alert_model_validation() {
        // Valid model
        let valid_model = CreateSecurityAlertModel {
            alert_id: "alert_123".to_string(),
            alert_type: AlertType::AuthenticationFailure,
            severity: AlertSeverity::Medium,
            message: "Test alert message".to_string(),
            source: "192.168.1.1".to_string(),
            ..Default::default()
        };
        assert!(valid_model.validate().is_ok());

        // Empty alert_id
        let invalid_model = CreateSecurityAlertModel {
            alert_id: "".to_string(),
            message: "Test alert message".to_string(),
            source: "192.168.1.1".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Empty message
        let invalid_model = CreateSecurityAlertModel {
            alert_id: "alert_123".to_string(),
            message: "".to_string(),
            source: "192.168.1.1".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());

        // Empty source
        let invalid_model = CreateSecurityAlertModel {
            alert_id: "alert_123".to_string(),
            message: "Test alert message".to_string(),
            source: "".to_string(),
            ..Default::default()
        };
        assert!(invalid_model.validate().is_err());
    }

    // #[test]
    // fn test_create_security_alert_model_new_with_auto_id() {
    //     let alert = CreateSecurityAlertModel::new_with_auto_id(
    //         AlertType::BruteForceAttack,
    //         AlertSeverity::High,
    //         "Brute force attack detected".to_string(),
    //         "192.168.1.100".to_string(),
    //     );

    //     assert!(alert.alert_id.starts_with("alert_"));
    //     assert!(matches!(alert.alert_type, AlertType::BruteForceAttack));
    //     assert!(matches!(alert.severity, AlertSeverity::High));
    //     assert_eq!(alert.message, "Brute force attack detected");
    //     assert_eq!(alert.source, "192.168.1.100");
    //     assert_eq!(alert.affected_resource, None);
    //     assert_eq!(alert.metadata, None);
    // }
}
