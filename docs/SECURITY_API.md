# Security API Documentation

## Overview

This document provides comprehensive API documentation for the AvoRed Rust CMS security framework. All security services expose well-defined APIs for integration and monitoring.

## Table of Contents

1. [Input Validation Service API](#input-validation-service-api)
2. [Authentication Service API](#authentication-service-api)
3. [Security Audit Service API](#security-audit-service-api)
4. [Security Monitoring Service API](#security-monitoring-service-api)
5. [Rate Limiting Service API](#rate-limiting-service-api)
6. [Security Health API](#security-health-api)
7. [Error Handling](#error-handling)
8. [Examples](#examples)

## Input Validation Service API

### `InputValidationService`

Provides comprehensive input validation and sanitization.

#### Methods

##### `validate_username(username: &str) -> Result<String>`

Validates and sanitizes username input.

**Parameters:**
- `username`: The username to validate

**Returns:**
- `Ok(String)`: Sanitized username
- `Err(Error)`: Validation error

**Validation Rules:**
- Length: 3-50 characters
- Allowed characters: alphanumeric, underscore, hyphen, dot
- No leading/trailing whitespace
- No consecutive special characters

**Example:**
```rust
let username = InputValidationService::validate_username("user123")?;
```

##### `validate_password(password: &str) -> Result<(String, PasswordStrength)>`

Validates password and returns strength assessment.

**Parameters:**
- `password`: The password to validate

**Returns:**
- `Ok((String, PasswordStrength))`: Validated password and strength
- `Err(Error)`: Validation error

**Password Requirements:**
- Minimum 12 characters
- At least one uppercase letter
- At least one lowercase letter
- At least one number
- At least one special character
- No common passwords
- No personal information

**Example:**
```rust
let (password, strength) = InputValidationService::validate_password("SecurePass123!")?;
match strength {
    PasswordStrength::Weak => println!("Password is weak"),
    PasswordStrength::Medium => println!("Password is medium strength"),
    PasswordStrength::Strong => println!("Password is strong"),
    PasswordStrength::VeryStrong => println!("Password is very strong"),
}
```

##### `validate_email(email: &str) -> Result<String>`

Validates email address format and domain.

**Parameters:**
- `email`: The email address to validate

**Returns:**
- `Ok(String)`: Validated email address
- `Err(Error)`: Validation error

**Validation Rules:**
- RFC 5322 compliant format
- Maximum 254 characters
- Domain validation
- No disposable email domains (configurable)

**Example:**
```rust
let email = InputValidationService::validate_email("user@example.com")?;
```

##### `validate_ldap_filter(filter: &str) -> Result<String>`

Validates and sanitizes LDAP filter expressions.

**Parameters:**
- `filter`: The LDAP filter to validate

**Returns:**
- `Ok(String)`: Sanitized LDAP filter
- `Err(Error)`: Validation error

**Security Features:**
- LDAP injection prevention
- Filter complexity limits
- Attribute name validation
- Value sanitization

**Example:**
```rust
let filter = InputValidationService::validate_ldap_filter("(uid=user123)")?;
```

##### `sanitize_log_message(message: &str) -> String`

Sanitizes log messages to prevent log injection.

**Parameters:**
- `message`: The log message to sanitize

**Returns:**
- `String`: Sanitized log message

**Sanitization:**
- Removes control characters
- Escapes special characters
- Truncates long messages
- Removes sensitive patterns

**Example:**
```rust
let safe_message = InputValidationService::sanitize_log_message("User login: admin\n\rFAKE LOG");
```

## Authentication Service API

### `LdapAuthService`

Provides secure LDAP authentication with advanced security features.

#### Methods

##### `new(config: LdapConfig, pool: LdapConnectionPool, rate_limiter: AuthRateLimiter, monitor: SecurityMonitoringService) -> Result<Self>`

Creates a new LDAP authentication service.

**Parameters:**
- `config`: LDAP configuration
- `pool`: Connection pool
- `rate_limiter`: Rate limiting service
- `monitor`: Security monitoring service

**Returns:**
- `Ok(LdapAuthService)`: Configured service
- `Err(Error)`: Configuration error

##### `authenticate(username: &str, password: &str) -> Result<AuthResult>`

Authenticates user credentials against LDAP.

**Parameters:**
- `username`: Username to authenticate
- `password`: Password to verify

**Returns:**
- `Ok(AuthResult)`: Authentication result
- `Err(Error)`: Authentication error

**Security Features:**
- Rate limiting
- Timing attack protection
- Comprehensive audit logging
- Automatic account lockout

**Example:**
```rust
match ldap_service.authenticate("user123", "password").await {
    Ok(AuthResult::Success(user_info)) => {
        println!("Authentication successful: {:?}", user_info);
    },
    Ok(AuthResult::Failure(reason)) => {
        println!("Authentication failed: {:?}", reason);
    },
    Err(e) => {
        println!("Authentication error: {:?}", e);
    }
}
```

## Security Audit Service API

### `SecurityAuditService`

Comprehensive security event logging and analysis.

#### Methods

##### `new(max_events: usize) -> Self`

Creates a new security audit service.

**Parameters:**
- `max_events`: Maximum events to keep in memory

**Returns:**
- `SecurityAuditService`: New service instance

##### `log_event(event: SecurityEvent) -> Result<()>`

Logs a security event.

**Parameters:**
- `event`: Security event to log

**Returns:**
- `Ok(())`: Event logged successfully
- `Err(Error)`: Logging error

**Event Types:**
```rust
pub enum SecurityEvent {
    AuthenticationAttempt {
        username: String,
        success: bool,
        ip_address: String,
        user_agent: String,
        timestamp: SystemTime,
    },
    AuthorizationFailure {
        username: String,
        resource: String,
        action: String,
        timestamp: SystemTime,
    },
    InjectionAttempt {
        injection_type: String,
        payload: String,
        ip_address: String,
        timestamp: SystemTime,
    },
    RateLimitExceeded {
        identifier: String,
        limit_type: String,
        timestamp: SystemTime,
    },
    SuspiciousActivity {
        activity_type: String,
        description: String,
        severity: String,
        timestamp: SystemTime,
    },
}
```

**Example:**
```rust
let event = SecurityEvent::AuthenticationAttempt {
    username: "user123".to_string(),
    success: true,
    ip_address: "192.168.1.100".to_string(),
    user_agent: "Mozilla/5.0...".to_string(),
    timestamp: SystemTime::now(),
};

audit_service.log_event(event).await?;
```

##### `get_recent_events(limit: usize) -> Result<Vec<SecurityEvent>>`

Retrieves recent security events.

**Parameters:**
- `limit`: Maximum number of events to return

**Returns:**
- `Ok(Vec<SecurityEvent>)`: List of recent events
- `Err(Error)`: Retrieval error

##### `get_auth_stats(duration: Duration) -> Result<AuthStats>`

Gets authentication statistics for a time period.

**Parameters:**
- `duration`: Time period to analyze

**Returns:**
- `Ok(AuthStats)`: Authentication statistics
- `Err(Error)`: Analysis error

**AuthStats Structure:**
```rust
pub struct AuthStats {
    pub total_attempts: u64,
    pub successful_attempts: u64,
    pub failed_attempts: u64,
    pub unique_users: u64,
    pub unique_ips: u64,
    pub success_rate: f64,
    pub average_response_time: Duration,
}
```

## Security Monitoring Service API

### `SecurityMonitoringService`

Real-time security monitoring and alerting.

#### Methods

##### `new() -> Self`

Creates a new security monitoring service.

##### `record_authentication_attempt(success: bool, username: &str) -> Result<()>`

Records an authentication attempt.

**Parameters:**
- `success`: Whether authentication was successful
- `username`: Username that attempted authentication

##### `record_injection_attempt(injection_type: &str, payload: &str) -> Result<()>`

Records an injection attempt.

**Parameters:**
- `injection_type`: Type of injection (SQL, LDAP, XSS, etc.)
- `payload`: The malicious payload

##### `perform_health_checks() -> Result<HashMap<String, HealthCheckResult>>`

Performs comprehensive security health checks.

**Returns:**
- `Ok(HashMap<String, HealthCheckResult>)`: Health check results
- `Err(Error)`: Health check error

**HealthCheckResult Structure:**
```rust
pub struct HealthCheckResult {
    pub status: HealthStatus,
    pub message: String,
    pub timestamp: SystemTime,
    pub response_time: Duration,
    pub details: HashMap<String, String>,
}

pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}
```

##### `get_metrics() -> Result<SecurityMetrics>`

Gets current security metrics.

**Returns:**
- `Ok(SecurityMetrics)`: Current security metrics
- `Err(Error)`: Metrics error

**SecurityMetrics Structure:**
```rust
pub struct SecurityMetrics {
    pub authentication_attempts: u64,
    pub authentication_failures: u64,
    pub injection_attempts: u64,
    pub rate_limit_triggers: u64,
    pub security_alerts: u64,
    pub average_response_time: Duration,
    pub uptime: Duration,
    pub last_updated: SystemTime,
}
```

## Rate Limiting Service API

### `AuthRateLimiter`

Rate limiting for authentication attempts.

#### Methods

##### `new(max_attempts: usize, window: Duration) -> Self`

Creates a new rate limiter.

**Parameters:**
- `max_attempts`: Maximum attempts allowed
- `window`: Time window for rate limiting

##### `is_allowed(identifier: &str) -> Result<bool>`

Checks if an attempt is allowed.

**Parameters:**
- `identifier`: Unique identifier (IP, username, etc.)

**Returns:**
- `Ok(bool)`: Whether attempt is allowed
- `Err(Error)`: Rate limiting error

##### `record_attempt(identifier: &str) -> Result<()>`

Records an attempt.

**Parameters:**
- `identifier`: Unique identifier

##### `remaining_attempts(identifier: &str) -> Result<usize>`

Gets remaining attempts for identifier.

**Parameters:**
- `identifier`: Unique identifier

**Returns:**
- `Ok(usize)`: Remaining attempts
- `Err(Error)`: Query error

## Security Health API

### Health Check Endpoints

#### `GET /health/security`

Returns overall security health status.

**Response:**
```json
{
    "status": "healthy",
    "timestamp": "2024-01-15T10:30:00Z",
    "services": {
        "input_validation": "healthy",
        "authentication": "healthy",
        "rate_limiting": "healthy",
        "audit_logging": "healthy",
        "monitoring": "healthy"
    },
    "metrics": {
        "uptime": 86400,
        "total_requests": 10000,
        "error_rate": 0.01
    }
}
```

#### `GET /health/security/detailed`

Returns detailed health information.

**Response:**
```json
{
    "status": "healthy",
    "timestamp": "2024-01-15T10:30:00Z",
    "services": {
        "input_validation": {
            "status": "healthy",
            "response_time": 5,
            "last_check": "2024-01-15T10:29:55Z",
            "details": {
                "validations_per_second": 100,
                "error_rate": 0.001
            }
        },
        "authentication": {
            "status": "healthy",
            "response_time": 150,
            "last_check": "2024-01-15T10:29:55Z",
            "details": {
                "ldap_connection_pool": "healthy",
                "success_rate": 0.95
            }
        }
    }
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Input validation failed: {0}")]
    ValidationError(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("Authorization failed: {0}")]
    AuthorizationError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Internal security error: {0}")]
    InternalError(String),
}
```

### Error Response Format

```json
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "Input validation failed",
        "details": "Username contains invalid characters",
        "timestamp": "2024-01-15T10:30:00Z",
        "request_id": "req_123456789"
    }
}
```

## Examples

### Complete Authentication Flow

```rust
use crate::services::{
    input_validation_service::InputValidationService,
    ldap_auth_service::LdapAuthService,
    security_audit_service::{SecurityAuditService, SecurityEvent},
    security_monitoring_service::SecurityMonitoringService,
};

async fn authenticate_user(
    username: &str,
    password: &str,
    ip_address: &str,
) -> Result<AuthResult> {
    // 1. Validate input
    let clean_username = InputValidationService::validate_username(username)?;
    let (clean_password, _) = InputValidationService::validate_password(password)?;
    
    // 2. Check rate limiting
    if !rate_limiter.is_allowed(&clean_username).await? {
        return Err(SecurityError::RateLimitError("Too many attempts".to_string()));
    }
    
    // 3. Attempt authentication
    let auth_result = ldap_service.authenticate(&clean_username, &clean_password).await?;
    
    // 4. Record attempt
    rate_limiter.record_attempt(&clean_username).await?;
    
    // 5. Log security event
    let event = SecurityEvent::AuthenticationAttempt {
        username: clean_username.clone(),
        success: matches!(auth_result, AuthResult::Success(_)),
        ip_address: ip_address.to_string(),
        user_agent: "".to_string(), // Get from request headers
        timestamp: SystemTime::now(),
    };
    audit_service.log_event(event).await?;
    
    // 6. Update monitoring
    monitoring_service.record_authentication_attempt(
        matches!(auth_result, AuthResult::Success(_)),
        &clean_username,
    ).await?;
    
    Ok(auth_result)
}
```

### Security Health Monitoring

```rust
async fn check_security_health() -> Result<SecurityHealthReport> {
    let monitoring_service = SecurityMonitoringService::new();
    
    // Perform health checks
    let health_results = monitoring_service.perform_health_checks().await?;
    
    // Get current metrics
    let metrics = monitoring_service.get_metrics().await?;
    
    // Generate report
    let report = SecurityHealthReport {
        overall_status: determine_overall_status(&health_results),
        services: health_results,
        metrics,
        timestamp: SystemTime::now(),
    };
    
    Ok(report)
}
```

---

For more information, see the main [Security Documentation](SECURITY.md) and [Configuration Guide](SECURITY_CONFIGURATION.md).
