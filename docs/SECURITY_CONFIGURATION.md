# Security Configuration Guide

## Overview

This guide provides detailed configuration instructions for the AvoRed Rust CMS security framework. Proper configuration is essential for maintaining a secure system.

## Configuration Files

### 1. Main Security Configuration

**File**: `config/security.toml`

```toml
[security]
# Enable/disable security framework
enabled = true

# Strict mode enforces additional security checks
strict_mode = true

# Security framework version
version = "1.0.0"

# Debug mode (disable in production)
debug_mode = false

[authentication]
# Available providers: ldap, local, oauth, saml
providers = ["ldap", "local"]

# Default authentication provider
default_provider = "ldap"

# Session configuration
session_timeout = 3600  # seconds
max_concurrent_sessions = 5
session_cookie_secure = true
session_cookie_httponly = true
session_cookie_samesite = "Strict"

# Password policy
password_min_length = 12
password_max_length = 128
password_require_uppercase = true
password_require_lowercase = true
password_require_numbers = true
password_require_symbols = true
password_history_count = 5
password_expiry_days = 90

[ldap]
# LDAP server configuration
url = "ldap://localhost:389"
bind_dn = "cn=admin,dc=example,dc=com"
bind_password = "admin_password"
base_dn = "dc=example,dc=com"
user_filter = "(uid={})"
group_filter = "(memberUid={})"

# Connection settings
connection_timeout = 30
search_timeout = 30
use_tls = true
verify_certificates = true

# Connection pool
pool_max_connections = 10
pool_min_connections = 2
pool_idle_timeout = 300

[rate_limiting]
# Enable rate limiting
enabled = true

# Global rate limits
max_requests_per_minute = 60
max_requests_per_hour = 1000

# Authentication rate limits
auth_max_attempts = 5
auth_window_seconds = 300
auth_lockout_duration = 900

# API rate limits
api_max_requests_per_minute = 100
api_burst_limit = 20

[input_validation]
# Enable strict input validation
strict_mode = true

# Maximum input lengths
max_username_length = 50
max_password_length = 128
max_email_length = 254
max_input_length = 1000

# Content filtering
sanitize_html = true
block_suspicious_patterns = true
allow_unicode = true

# File upload restrictions
max_file_size = 10485760  # 10MB
allowed_file_types = ["jpg", "jpeg", "png", "gif", "pdf", "doc", "docx"]
scan_uploads = true

[monitoring]
# Enable security monitoring
enabled = true

# Logging configuration
log_level = "INFO"
log_format = "json"
log_file = "logs/security.log"
log_rotation = "daily"
log_retention_days = 90

# Alert configuration
alert_threshold = "MEDIUM"
alert_email = "security@example.com"
alert_webhook = "https://hooks.example.com/security"

# Health check configuration
health_check_interval = 60
health_check_timeout = 10

[audit]
# Enable audit logging
enabled = true

# Audit configuration
log_all_events = true
log_sensitive_data = false
export_format = "json"
retention_days = 90

# Events to audit
audit_authentication = true
audit_authorization = true
audit_data_access = true
audit_configuration_changes = true
audit_security_events = true

[encryption]
# Encryption settings
algorithm = "AES-256-GCM"
key_rotation_days = 30
use_hardware_security_module = false

# TLS configuration
tls_min_version = "1.2"
tls_cipher_suites = [
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"
]

[compliance]
# Compliance frameworks
frameworks = ["OWASP", "NIST", "ISO27001"]

# Data protection
gdpr_compliance = true
ccpa_compliance = true
data_retention_policy = true

# Security controls
access_control = "RBAC"
data_classification = true
incident_response = true
```

### 2. Environment Variables

**File**: `.env`

```bash
# Database Configuration
DATABASE_URL=postgresql://user:password@localhost/avored_cms

# LDAP Configuration
LDAP_URL=ldap://localhost:389
LDAP_BIND_DN=cn=admin,dc=example,dc=com
LDAP_BIND_PASSWORD=admin_password
LDAP_BASE_DN=dc=example,dc=com
LDAP_USER_FILTER=(uid={})
LDAP_GROUP_FILTER=(memberUid={})

# Security Keys
JWT_SECRET=your-super-secret-jwt-key-here
ENCRYPTION_KEY=your-encryption-key-here
CSRF_SECRET=your-csrf-secret-here

# Security Configuration
SECURITY_ENABLED=true
SECURITY_STRICT_MODE=true
SECURITY_DEBUG_MODE=false

# Rate Limiting
RATE_LIMIT_ENABLED=true
RATE_LIMIT_MAX_ATTEMPTS=5
RATE_LIMIT_WINDOW_SECONDS=300
RATE_LIMIT_LOCKOUT_DURATION=900

# Session Configuration
SESSION_TIMEOUT=3600
SESSION_SECURE=true
SESSION_HTTPONLY=true
SESSION_SAMESITE=Strict

# Monitoring
MONITORING_ENABLED=true
MONITORING_LOG_LEVEL=INFO
MONITORING_ALERT_THRESHOLD=MEDIUM

# Audit Configuration
AUDIT_ENABLED=true
AUDIT_LOG_ALL_EVENTS=true
AUDIT_RETENTION_DAYS=90

# TLS Configuration
TLS_ENABLED=true
TLS_MIN_VERSION=1.2
TLS_CERT_PATH=/path/to/cert.pem
TLS_KEY_PATH=/path/to/key.pem

# External Services
SECURITY_WEBHOOK_URL=https://hooks.example.com/security
ALERT_EMAIL=security@example.com
SIEM_ENDPOINT=https://siem.example.com/api/events
```

## Configuration Validation

### Validation Script

Create `scripts/validate_security_config.sh`:

```bash
#!/bin/bash

echo "Validating AvoRed CMS Security Configuration..."

# Check required files
if [ ! -f "config/security.toml" ]; then
    echo "ERROR: security.toml not found"
    exit 1
fi

if [ ! -f ".env" ]; then
    echo "ERROR: .env file not found"
    exit 1
fi

# Validate LDAP connection
echo "Testing LDAP connection..."
ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD" -b "$LDAP_BASE_DN" "(objectClass=*)" dn

# Check TLS certificates
if [ "$TLS_ENABLED" = "true" ]; then
    echo "Validating TLS certificates..."
    openssl x509 -in "$TLS_CERT_PATH" -text -noout
    openssl rsa -in "$TLS_KEY_PATH" -check
fi

# Validate database connection
echo "Testing database connection..."
psql "$DATABASE_URL" -c "SELECT 1;"

echo "Configuration validation complete!"
```

## Security Hardening

### 1. Production Configuration

```toml
[security]
enabled = true
strict_mode = true
debug_mode = false

[authentication]
session_timeout = 1800  # 30 minutes
max_concurrent_sessions = 3
password_expiry_days = 60

[rate_limiting]
auth_max_attempts = 3
auth_window_seconds = 300
auth_lockout_duration = 1800

[monitoring]
log_level = "WARN"
alert_threshold = "LOW"

[audit]
log_sensitive_data = false
retention_days = 365
```

### 2. Development Configuration

```toml
[security]
enabled = true
strict_mode = false
debug_mode = true

[authentication]
session_timeout = 7200  # 2 hours
max_concurrent_sessions = 10
password_expiry_days = 0  # No expiry

[rate_limiting]
auth_max_attempts = 10
auth_window_seconds = 60
auth_lockout_duration = 300

[monitoring]
log_level = "DEBUG"
alert_threshold = "HIGH"

[audit]
log_sensitive_data = true
retention_days = 30
```

## Configuration Management

### 1. Configuration Deployment

```bash
# Production deployment
cp config/security.prod.toml config/security.toml
cp .env.prod .env

# Development deployment
cp config/security.dev.toml config/security.toml
cp .env.dev .env

# Staging deployment
cp config/security.staging.toml config/security.toml
cp .env.staging .env
```

### 2. Configuration Backup

```bash
#!/bin/bash
# backup_config.sh

BACKUP_DIR="backups/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup configuration files
cp config/security.toml "$BACKUP_DIR/"
cp .env "$BACKUP_DIR/"

# Create encrypted backup
tar -czf "$BACKUP_DIR.tar.gz" "$BACKUP_DIR"
gpg --symmetric --cipher-algo AES256 "$BACKUP_DIR.tar.gz"

# Clean up
rm -rf "$BACKUP_DIR"
rm "$BACKUP_DIR.tar.gz"

echo "Configuration backed up to $BACKUP_DIR.tar.gz.gpg"
```

### 3. Configuration Monitoring

```bash
#!/bin/bash
# monitor_config.sh

# Check for configuration changes
inotifywait -m -e modify config/security.toml .env |
while read path action file; do
    echo "Configuration file $file was $action at $(date)"
    
    # Validate configuration
    ./scripts/validate_security_config.sh
    
    # Restart services if needed
    if [ "$action" = "MODIFY" ]; then
        systemctl reload avored-cms
    fi
done
```

## Troubleshooting Configuration Issues

### Common Configuration Problems

1. **LDAP Connection Issues**
   ```bash
   # Test LDAP connectivity
   ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD"
   
   # Check LDAP logs
   tail -f /var/log/ldap/slapd.log
   ```

2. **Database Connection Issues**
   ```bash
   # Test database connection
   psql "$DATABASE_URL" -c "SELECT version();"
   
   # Check database logs
   tail -f /var/log/postgresql/postgresql.log
   ```

3. **TLS Certificate Issues**
   ```bash
   # Validate certificate
   openssl x509 -in cert.pem -text -noout
   
   # Check certificate expiry
   openssl x509 -in cert.pem -noout -dates
   ```

4. **Permission Issues**
   ```bash
   # Check file permissions
   ls -la config/security.toml
   ls -la .env
   
   # Fix permissions
   chmod 600 config/security.toml
   chmod 600 .env
   ```

### Configuration Validation Tools

1. **TOML Validator**
   ```bash
   # Install toml-cli
   cargo install toml-cli
   
   # Validate TOML syntax
   toml get config/security.toml
   ```

2. **Environment Variable Checker**
   ```bash
   #!/bin/bash
   # check_env.sh
   
   required_vars=(
       "DATABASE_URL"
       "LDAP_URL"
       "JWT_SECRET"
       "ENCRYPTION_KEY"
   )
   
   for var in "${required_vars[@]}"; do
       if [ -z "${!var}" ]; then
           echo "ERROR: $var is not set"
           exit 1
       fi
   done
   
   echo "All required environment variables are set"
   ```

## Security Configuration Checklist

### Pre-Production Checklist

- [ ] Security framework enabled
- [ ] Strict mode enabled
- [ ] Debug mode disabled
- [ ] Strong passwords enforced
- [ ] Rate limiting configured
- [ ] TLS enabled and configured
- [ ] Audit logging enabled
- [ ] Monitoring configured
- [ ] Backup procedures in place
- [ ] Configuration validated
- [ ] Security testing completed

### Regular Maintenance

- [ ] Review security logs weekly
- [ ] Update configurations monthly
- [ ] Rotate encryption keys quarterly
- [ ] Review access controls quarterly
- [ ] Update security policies annually
- [ ] Conduct security audits annually

---

For more information, see the main [Security Documentation](SECURITY.md).
