# Security Audit Database Schema Documentation

## Overview

This document describes the database schema for the security audit and alerting system in AvoRed Rust CMS. The schema consists of two main tables designed to provide comprehensive security monitoring and incident tracking.

## Tables

### 1. security_audits

The `security_audits` table stores comprehensive security metrics and audit logs for all security-related events in the system.

#### Fields

| Field Name | Type | Required | Default | Description |
|------------|------|----------|---------|-------------|
| `security_audit_id` | string | Yes | - | Unique identifier for the audit record |
| `admin_user_id` | option<string> | No | - | Reference to the admin user (if authenticated) |
| `session_id` | option<string> | No | - | Session identifier for tracking user sessions |
| `ip_address` | string | Yes | - | Client IP address (IPv4/IPv6 or "unknown") |
| `user_agent` | option<string> | No | - | Client browser/application user agent |
| `endpoint` | option<string> | No | - | API endpoint that was accessed |
| `request_method` | option<string> | No | - | HTTP method (GET, POST, PUT, DELETE, etc.) |
| `total_authentication_attempts` | int | No | 0 | Total number of authentication attempts |
| `failed_authentication_attempts` | int | No | 0 | Number of failed authentication attempts |
| `blocked_injection_attempts` | int | No | 0 | Number of blocked injection attempts |
| `rate_limited_requests` | int | No | 0 | Number of rate-limited requests |
| `suspicious_activities_detected` | int | No | 0 | Count of suspicious activities detected |
| `security_violations` | int | No | 0 | Number of security policy violations |
| `uptime_seconds` | int | No | 0 | System uptime in seconds |
| `security_health_score` | float | No | 100.0 | Security health score (0-100) |
| `created_at` | datetime | No | time::now() | Record creation timestamp |
| `updated_at` | datetime | No | time::now() | Record last update timestamp |
| `metadata` | option<object> | No | - | Additional context data in JSON format |

#### Constraints

- `security_audit_id`: Must be non-empty string
- `ip_address`: Must be valid IPv4, IPv6, or "unknown"
- `security_health_score`: Must be between 0.0 and 100.0

#### Indexes

- `idx_security_audits_ip`: Index on `ip_address`
- `idx_security_audits_user`: Index on `admin_user_id`
- `idx_security_audits_created`: Index on `created_at`
- `idx_security_audits_endpoint`: Index on `endpoint`
- `idx_security_audits_session`: Index on `session_id`
- `idx_security_audits_user_created`: Composite index on `admin_user_id, created_at`

### 2. security_alerts

The `security_alerts` table stores security incident alerts and their resolution status.

#### Fields

| Field Name | Type | Required | Default | Description |
|------------|------|----------|---------|-------------|
| `alert_id` | string | Yes | - | Unique identifier for the alert |
| `alert_type` | string | Yes | - | Type of security alert (see allowed values) |
| `severity` | string | Yes | - | Alert severity level |
| `message` | string | Yes | - | Human-readable alert message |
| `source` | string | Yes | - | Source of the alert (IP, service, module) |
| `affected_resource` | option<string> | No | - | Resource/endpoint that was affected |
| `metadata` | option<object> | No | - | Additional alert context in JSON format |
| `resolved` | bool | No | false | Whether the alert has been resolved |
| `resolved_at` | option<datetime> | No | - | When the alert was resolved |
| `resolved_by` | option<string> | No | - | Who resolved the alert |
| `created_at` | datetime | No | time::now() | Alert creation timestamp |

#### Constraints

- `alert_id`: Must be non-empty string
- `alert_type`: Must be one of the predefined alert types
- `severity`: Must be one of: 'low', 'medium', 'high', 'critical'
- `message`: Must be non-empty string
- `source`: Must be non-empty string

#### Alert Types

The following alert types are supported:

- `authentication_failure`: Failed login attempts
- `injection_attempt`: SQL/code injection attempts
- `rate_limit_exceeded`: Rate limiting triggered
- `suspicious_activity`: General suspicious behavior
- `privilege_escalation`: Unauthorized privilege escalation attempts
- `data_breach_attempt`: Potential data breach attempts
- `unauthorized_access`: Access to restricted resources
- `malformed_request`: Malformed or invalid requests
- `brute_force_attack`: Brute force attack patterns
- `session_hijacking`: Session hijacking attempts

#### Severity Levels

- `low`: Minor security events that require monitoring
- `medium`: Moderate security events that may require attention
- `high`: Serious security events that require immediate attention
- `critical`: Critical security events that require immediate action

#### Indexes

- `idx_security_alerts_severity`: Index on `severity`
- `idx_security_alerts_type`: Index on `alert_type`
- `idx_security_alerts_created`: Index on `created_at`
- `idx_security_alerts_resolved`: Index on `resolved`
- `idx_security_alerts_source`: Index on `source`
- `idx_security_alerts_severity_created`: Composite index on `severity, created_at`

## Usage Examples

### Creating an Audit Record

```sql
INSERT INTO security_audits {
    security_audit_id: "audit_" + rand::uuid::v4(),
    admin_user_id: "user_123",
    ip_address: "192.168.1.100",
    endpoint: "/api/auth/login",
    request_method: "POST",
    total_authentication_attempts: 1,
    failed_authentication_attempts: 0,
    security_health_score: 100.0
};
```

### Creating a Security Alert

```sql
INSERT INTO security_alerts {
    alert_id: "alert_" + rand::uuid::v4(),
    alert_type: "authentication_failure",
    severity: "medium",
    message: "Multiple failed login attempts from IP 192.168.1.100",
    source: "192.168.1.100",
    affected_resource: "/api/auth/login"
};
```

### Querying Recent High-Severity Alerts

```sql
SELECT * FROM security_alerts 
WHERE severity IN ['high', 'critical'] 
AND resolved = false 
ORDER BY created_at DESC 
LIMIT 10;
```

## Data Retention

- Audit records should be retained for compliance requirements (typically 90 days minimum)
- Alert records should be retained for historical analysis
- Consider implementing automated cleanup for old records based on organizational policies

## Security Considerations

- All audit data should be encrypted at rest and in transit
- Access to audit tables should be restricted to authorized personnel only
- IP addresses may contain PII and should be handled according to privacy regulations
- Consider data anonymization for long-term storage

## Migration Notes

- The schema includes sample data for testing purposes
- Remove sample data before production deployment
- Ensure proper backup procedures before applying schema changes
- Test schema changes in staging environment first
