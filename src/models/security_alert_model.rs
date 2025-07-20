use std::collections::BTreeMap;
use std::time::SystemTime;
use prost_types::Timestamp;
use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value, Uuid};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum AlertType {
    AuthenticationFailure,
    InjectionAttempt,
    RateLimitExceeded,
    SuspiciousActivity,
    PrivilegeEscalation,
    DataBreachAttempt,
    UnauthorizedAccess,
    MalformedRequest,
    BruteForceAttack,
    SessionHijacking,
}

impl Default for AlertType {
    fn default() -> Self {
        AlertType::SuspiciousActivity
    }
}

impl AlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertType::AuthenticationFailure => "authentication_failure",
            AlertType::InjectionAttempt => "injection_attempt",
            AlertType::RateLimitExceeded => "rate_limit_exceeded",
            AlertType::SuspiciousActivity => "suspicious_activity",
            AlertType::PrivilegeEscalation => "privilege_escalation",
            AlertType::DataBreachAttempt => "data_breach_attempt",
            AlertType::UnauthorizedAccess => "unauthorized_access",
            AlertType::MalformedRequest => "malformed_request",
            AlertType::BruteForceAttack => "brute_force_attack",
            AlertType::SessionHijacking => "session_hijacking",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "authentication_failure" => Ok(AlertType::AuthenticationFailure),
            "injection_attempt" => Ok(AlertType::InjectionAttempt),
            "rate_limit_exceeded" => Ok(AlertType::RateLimitExceeded),
            "suspicious_activity" => Ok(AlertType::SuspiciousActivity),
            "privilege_escalation" => Ok(AlertType::PrivilegeEscalation),
            "data_breach_attempt" => Ok(AlertType::DataBreachAttempt),
            "unauthorized_access" => Ok(AlertType::UnauthorizedAccess),
            "malformed_request" => Ok(AlertType::MalformedRequest),
            "brute_force_attack" => Ok(AlertType::BruteForceAttack),
            "session_hijacking" => Ok(AlertType::SessionHijacking),
            _ => Err(Error::Generic(format!("Invalid alert type: {}", s))),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        AlertSeverity::Medium
    }
}

impl AlertSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertSeverity::Low => "low",
            AlertSeverity::Medium => "medium",
            AlertSeverity::High => "high",
            AlertSeverity::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "low" => Ok(AlertSeverity::Low),
            "medium" => Ok(AlertSeverity::Medium),
            "high" => Ok(AlertSeverity::High),
            "critical" => Ok(AlertSeverity::Critical),
            _ => Err(Error::Generic(format!("Invalid alert severity: {}", s))),
        }
    }

    pub fn priority_score(&self) -> u8 {
        match self {
            AlertSeverity::Low => 1,
            AlertSeverity::Medium => 2,
            AlertSeverity::High => 3,
            AlertSeverity::Critical => 4,
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAlertModel {
    pub id: String,
    pub alert_id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub source: String,
    pub affected_resource: Option<String>,
    pub metadata: Option<BTreeMap<String, Value>>,
    pub resolved: bool,
    pub resolved_at: Option<Datetime>,
    pub resolved_by: Option<String>,
    pub created_at: Datetime,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreateSecurityAlertModel {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub source: String,
    pub affected_resource: Option<String>,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdateSecurityAlertModel {
    pub resolved: Option<bool>,
    pub resolved_by: Option<String>,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SecurityAlertPaginationModel {
    pub data: Vec<SecurityAlertModel>,
    pub pagination: Pagination,
}

impl TryFrom<Object> for SecurityAlertModel {
    type Error = Error;

    fn try_from(val: Object) -> Result<SecurityAlertModel> {
        let id = val.get("id").get_id()?;

        let alert_id = match val.get("alert_id") {
            Some(Value::Strand(val)) => val.to_string(),
            _ => return Err(Error::Generic("alert_id field is not a string".to_string())),
        };

        let alert_type = match val.get("alert_type") {
            Some(Value::Strand(val)) => AlertType::from_str(&val.to_string())?,
            _ => return Err(Error::Generic("alert_type field is not a valid string".to_string())),
        };

        let severity = match val.get("severity") {
            Some(Value::Strand(val)) => AlertSeverity::from_str(&val.to_string())?,
            _ => return Err(Error::Generic("severity field is not a valid string".to_string())),
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
            _ => return Err(Error::Generic("affected_resource field is not a string or null".to_string())),
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

        let resolved = match val.get("resolved") {
            Some(Value::Bool(val)) => *val,
            _ => false,
        };

        let resolved_at = match val.get("resolved_at") {
            Some(Value::Datetime(val)) => Some(val.clone()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("resolved_at field is not a datetime or null".to_string())),
        };

        let resolved_by = match val.get("resolved_by") {
            Some(Value::Strand(val)) => Some(val.to_string()),
            Some(Value::None) | None => None,
            _ => return Err(Error::Generic("resolved_by field is not a string or null".to_string())),
        };

        let created_at = match val.get("created_at") {
            Some(Value::Datetime(val)) => val.clone(),
            _ => return Err(Error::Generic("created_at field is not a datetime".to_string())),
        };

        Ok(SecurityAlertModel {
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

    /// Converts resolved_at to protobuf Timestamp
    pub fn resolved_at_timestamp(&self) -> Option<Timestamp> {
        self.resolved_at.as_ref().map(|dt| {
            let chrono_utc = dt.to_utc();
            let system_time = SystemTime::from(chrono_utc);
            let duration = system_time.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();

            Timestamp {
                seconds: duration.as_secs() as i64,
                nanos: duration.subsec_nanos() as i32,
            }
        })
    }

    /// Checks if alert requires immediate attention
    pub fn requires_immediate_attention(&self) -> bool {
        matches!(self.severity, AlertSeverity::High | AlertSeverity::Critical) && !self.resolved
    }

    /// Gets alert age in seconds
    pub fn age_seconds(&self) -> i64 {
        let now = SystemTime::now();
        let chrono_utc = self.created_at.to_utc();
        let created = SystemTime::from(chrono_utc);
        now.duration_since(created)
            .unwrap_or_default()
            .as_secs() as i64
    }
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

    /// Creates a new alert with auto-generated ID
    pub fn new_with_auto_id(
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        source: String,
    ) -> Self {
        Self {
            alert_id: format!("alert_{}", Uuid::new_v4()),
            alert_type,
            severity,
            message,
            source,
            affected_resource: None,
            metadata: None,
        }
    }
}
