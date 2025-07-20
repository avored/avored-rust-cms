# Security Troubleshooting Guide

## Overview

This guide provides comprehensive troubleshooting information for the AvoRed Rust CMS security framework. It covers common issues, diagnostic procedures, and resolution steps.

## Table of Contents

1. [Common Security Issues](#common-security-issues)
2. [Authentication Problems](#authentication-problems)
3. [LDAP Connection Issues](#ldap-connection-issues)
4. [Rate Limiting Problems](#rate-limiting-problems)
5. [Input Validation Issues](#input-validation-issues)
6. [Monitoring and Logging Issues](#monitoring-and-logging-issues)
7. [Performance Issues](#performance-issues)
8. [Diagnostic Tools](#diagnostic-tools)

## Common Security Issues

### 1. Service Startup Failures

#### Symptoms
- Application fails to start
- Security services not initializing
- Configuration errors in logs

#### Diagnostic Steps

```bash
# Check service status
systemctl status avored-cms

# View startup logs
journalctl -u avored-cms -f

# Check configuration syntax
toml get config/security.toml

# Validate environment variables
env | grep -E "(LDAP|SECURITY|JWT)"
```

#### Common Causes and Solutions

**Invalid Configuration**
```bash
# Validate TOML syntax
toml get config/security.toml > /dev/null
echo $? # Should return 0

# Check for missing required fields
grep -E "(enabled|url|secret)" config/security.toml
```

**Missing Environment Variables**
```bash
# Check required variables
required_vars=("DATABASE_URL" "LDAP_URL" "JWT_SECRET")
for var in "${required_vars[@]}"; do
    if [ -z "${!var}" ]; then
        echo "ERROR: $var is not set"
    fi
done
```

**Permission Issues**
```bash
# Fix file permissions
chmod 600 config/security.toml
chmod 600 .env
chown avored-cms:avored-cms config/security.toml
```

### 2. Security Framework Not Loading

#### Symptoms
- Security services disabled
- No security logging
- Authentication bypassed

#### Diagnostic Steps

```rust
// Check security framework status
use crate::security::SecurityFramework;

async fn check_security_status() -> Result<()> {
    let framework = SecurityFramework::instance().await?;
    
    println!("Security enabled: {}", framework.is_enabled());
    println!("Strict mode: {}", framework.is_strict_mode());
    println!("Services loaded: {:?}", framework.loaded_services());
    
    Ok(())
}
```

#### Solutions

**Enable Security Framework**
```toml
[security]
enabled = true
strict_mode = true
```

**Check Service Dependencies**
```bash
# Verify database connection
psql "$DATABASE_URL" -c "SELECT 1;"

# Test LDAP connectivity
ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD"
```

## Authentication Problems

### 1. LDAP Authentication Failures

#### Symptoms
- Users cannot log in
- LDAP connection errors
- Authentication timeouts

#### Diagnostic Steps

```bash
# Test LDAP connection
ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD" -b "$LDAP_BASE_DN" "(objectClass=*)"

# Check LDAP server logs
tail -f /var/log/ldap/slapd.log

# Test user authentication
ldapwhoami -x -H "$LDAP_URL" -D "uid=testuser,$LDAP_BASE_DN" -w "password"
```

#### Common Issues and Solutions

**Connection Refused**
```bash
# Check LDAP server status
systemctl status slapd

# Verify network connectivity
telnet ldap.example.com 389

# Check firewall rules
iptables -L | grep 389
```

**Authentication Failed**
```bash
# Verify bind credentials
ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD" -s base

# Check user DN format
ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD" -b "$LDAP_BASE_DN" "(uid=username)"
```

**TLS/SSL Issues**
```bash
# Test TLS connection
openssl s_client -connect ldap.example.com:636

# Verify certificate
openssl x509 -in /path/to/ldap-cert.pem -text -noout
```

### 2. Local Authentication Issues

#### Symptoms
- Password validation failures
- Hash verification errors
- Account lockouts

#### Diagnostic Steps

```rust
// Test password hashing
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

fn test_password_hash() -> Result<()> {
    let password = "test_password";
    let argon2 = Argon2::default();
    
    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    
    // Verify password
    let parsed_hash = PasswordHash::new(&password_hash.to_string())?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;
    
    println!("Password hashing working correctly");
    Ok(())
}
```

#### Solutions

**Password Policy Issues**
```toml
[authentication.password_policy]
min_length = 8  # Reduce if too strict
require_symbols = false  # Disable if causing issues
```

**Database Connection Issues**
```bash
# Test database connection
psql "$DATABASE_URL" -c "SELECT * FROM users LIMIT 1;"

# Check user table structure
psql "$DATABASE_URL" -c "\d users"
```

## LDAP Connection Issues

### 1. Connection Pool Problems

#### Symptoms
- Connection timeouts
- Pool exhaustion
- Slow authentication

#### Diagnostic Steps

```rust
// Monitor connection pool
async fn monitor_pool_health() -> Result<()> {
    let pool = LdapConnectionPool::instance().await?;
    let stats = pool.get_stats().await;
    
    println!("Active connections: {}", stats.active_connections);
    println!("Idle connections: {}", stats.idle_connections);
    println!("Total connections: {}", stats.total_connections);
    println!("Failed connections: {}", stats.failed_connections);
    
    Ok(())
}
```

#### Solutions

**Increase Pool Size**
```toml
[ldap]
pool_max_connections = 20
pool_min_connections = 5
pool_idle_timeout = 600
```

**Connection Health Checks**
```rust
// Implement connection health check
async fn health_check_connection(conn: &mut LdapConnection) -> Result<bool> {
    match conn.simple_bind("", "").await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
```

### 2. LDAP Server Connectivity

#### Symptoms
- DNS resolution failures
- Network timeouts
- Certificate errors

#### Diagnostic Steps

```bash
# Test DNS resolution
nslookup ldap.example.com

# Test network connectivity
ping ldap.example.com
telnet ldap.example.com 389

# Test LDAP over TLS
openssl s_client -connect ldap.example.com:636 -verify_return_error
```

#### Solutions

**DNS Configuration**
```bash
# Add to /etc/hosts if needed
echo "192.168.1.100 ldap.example.com" >> /etc/hosts

# Configure DNS servers
echo "nameserver 8.8.8.8" >> /etc/resolv.conf
```

**Certificate Issues**
```bash
# Add CA certificate
cp ldap-ca.crt /usr/local/share/ca-certificates/
update-ca-certificates

# Disable certificate verification (not recommended for production)
export LDAPTLS_REQCERT=never
```

## Rate Limiting Problems

### 1. Excessive Rate Limiting

#### Symptoms
- Legitimate users blocked
- High false positive rate
- Performance degradation

#### Diagnostic Steps

```rust
// Check rate limiting statistics
async fn check_rate_limiting_stats() -> Result<()> {
    let rate_limiter = AuthRateLimiter::instance().await?;
    
    let stats = rate_limiter.get_statistics().await?;
    println!("Total requests: {}", stats.total_requests);
    println!("Blocked requests: {}", stats.blocked_requests);
    println!("Block rate: {:.2}%", stats.block_rate * 100.0);
    
    // Check specific IP
    let remaining = rate_limiter.remaining_attempts("192.168.1.100").await?;
    println!("Remaining attempts for IP: {}", remaining);
    
    Ok(())
}
```

#### Solutions

**Adjust Rate Limits**
```toml
[rate_limiting]
auth_max_attempts = 10  # Increase from 5
auth_window_seconds = 600  # Increase window
auth_lockout_duration = 300  # Reduce lockout time
```

**Whitelist Trusted IPs**
```rust
// Add IP whitelist
const TRUSTED_IPS: &[&str] = &[
    "192.168.1.0/24",
    "10.0.0.0/8",
    "172.16.0.0/12",
];

fn is_trusted_ip(ip: &str) -> bool {
    TRUSTED_IPS.iter().any(|network| {
        ip_in_network(ip, network)
    })
}
```

### 2. Rate Limiting Not Working

#### Symptoms
- No rate limiting applied
- Brute force attacks succeeding
- High authentication failure rates

#### Diagnostic Steps

```bash
# Check rate limiting configuration
grep -A 10 "\[rate_limiting\]" config/security.toml

# Test rate limiting
for i in {1..10}; do
    curl -X POST http://localhost:8080/auth/login \
         -d "username=test&password=wrong" \
         -w "Status: %{http_code}\n"
done
```

#### Solutions

**Enable Rate Limiting**
```toml
[rate_limiting]
enabled = true
auth_max_attempts = 5
```

**Check Service Initialization**
```rust
// Ensure rate limiter is initialized
async fn initialize_rate_limiter() -> Result<()> {
    let config = RateLimitConfig::from_file("config/security.toml")?;
    let rate_limiter = AuthRateLimiter::new(config).await?;
    
    // Register with service container
    ServiceContainer::register("rate_limiter", rate_limiter).await?;
    
    Ok(())
}
```

## Input Validation Issues

### 1. Validation Too Strict

#### Symptoms
- Valid inputs rejected
- Users unable to enter data
- High validation error rates

#### Diagnostic Steps

```rust
// Test input validation
use crate::services::input_validation_service::InputValidationService;

fn test_validation() -> Result<()> {
    let test_inputs = vec![
        "user123",
        "user.name",
        "user-name",
        "user_name",
        "user@domain",
    ];
    
    for input in test_inputs {
        match InputValidationService::validate_username(input) {
            Ok(validated) => println!("✓ '{}' -> '{}'", input, validated),
            Err(e) => println!("✗ '{}' failed: {}", input, e),
        }
    }
    
    Ok(())
}
```

#### Solutions

**Relax Validation Rules**
```rust
// Adjust validation patterns
const USERNAME_PATTERN: &str = r"^[a-zA-Z0-9._-]{3,50}$";
const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
```

**Custom Validation Rules**
```toml
[input_validation]
strict_mode = false
allow_unicode = true
max_username_length = 100
```

### 2. Validation Bypassed

#### Symptoms
- Malicious input accepted
- Injection attacks succeeding
- Security alerts for bypassed validation

#### Diagnostic Steps

```bash
# Test injection patterns
curl -X POST http://localhost:8080/api/users \
     -H "Content-Type: application/json" \
     -d '{"username": "admin'\'''; DROP TABLE users; --"}'

# Check validation logs
grep "VALIDATION_ERROR" logs/security.log
```

#### Solutions

**Enable Strict Validation**
```toml
[input_validation]
strict_mode = true
block_suspicious_patterns = true
```

**Add Custom Patterns**
```rust
// Add injection detection patterns
const INJECTION_PATTERNS: &[&str] = &[
    r"(?i)(union|select|insert|update|delete|drop|create|alter)",
    r"(?i)(script|javascript|vbscript|onload|onerror)",
    r"(?i)(\$\{|<%|%>|<\?php)",
    r"\.\.[\\/]",
    r"\x00",
];
```

## Monitoring and Logging Issues

### 1. Missing Security Logs

#### Symptoms
- No security events logged
- Empty log files
- Missing audit trail

#### Diagnostic Steps

```bash
# Check log file permissions
ls -la logs/security.log

# Test log writing
echo "Test log entry" >> logs/security.log

# Check disk space
df -h /var/log

# Verify log configuration
grep -A 5 "\[logging\]" config/security.toml
```

#### Solutions

**Fix Log Permissions**
```bash
# Create log directory
mkdir -p logs
chown avored-cms:avored-cms logs
chmod 755 logs

# Fix log file permissions
touch logs/security.log
chown avored-cms:avored-cms logs/security.log
chmod 644 logs/security.log
```

**Enable Logging**
```toml
[logging]
enabled = true
level = "INFO"
file_path = "logs/security.log"
```

### 2. Log Rotation Issues

#### Symptoms
- Large log files
- Disk space exhaustion
- Performance degradation

#### Solutions

**Configure Log Rotation**
```bash
# Create logrotate configuration
cat > /etc/logrotate.d/avored-cms << EOF
/opt/avored-cms/logs/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 avored-cms avored-cms
    postrotate
        systemctl reload avored-cms
    endscript
}
EOF
```

**Implement Application-Level Rotation**
```rust
// Configure log rotation in application
use tracing_appender::rolling::{RollingFileAppender, Rotation};

fn setup_logging() -> Result<()> {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "security.log"
    );
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .json()
        .init();
    
    Ok(())
}
```

## Performance Issues

### 1. Slow Authentication

#### Symptoms
- High authentication latency
- Timeouts during login
- Poor user experience

#### Diagnostic Steps

```rust
// Measure authentication performance
use std::time::Instant;

async fn measure_auth_performance() -> Result<()> {
    let start = Instant::now();
    
    let result = ldap_service.authenticate("username", "password").await?;
    
    let duration = start.elapsed();
    println!("Authentication took: {:?}", duration);
    
    if duration > Duration::from_secs(5) {
        println!("WARNING: Authentication is slow");
    }
    
    Ok(())
}
```

#### Solutions

**Optimize LDAP Queries**
```rust
// Use more specific LDAP filters
const USER_FILTER: &str = "(uid={})";
const USER_ATTRIBUTES: &[&str] = &["uid", "cn", "mail"];

// Implement connection pooling
let pool_config = PoolConfig {
    max_connections: 20,
    min_connections: 5,
    connection_timeout: Duration::from_secs(10),
};
```

**Cache Authentication Results**
```rust
// Implement authentication caching
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct AuthCache {
    cache: HashMap<String, (bool, Instant)>,
    ttl: Duration,
}

impl AuthCache {
    fn get(&self, key: &str) -> Option<bool> {
        if let Some((result, timestamp)) = self.cache.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(*result);
            }
        }
        None
    }
}
```

### 2. High Memory Usage

#### Symptoms
- Increasing memory consumption
- Out of memory errors
- System slowdown

#### Diagnostic Steps

```bash
# Monitor memory usage
ps aux | grep avored-cms
top -p $(pgrep avored-cms)

# Check for memory leaks
valgrind --tool=memcheck --leak-check=full ./avored-rust-cms
```

#### Solutions

**Optimize Data Structures**
```rust
// Use bounded collections
use std::collections::VecDeque;

struct BoundedEventLog {
    events: VecDeque<SecurityEvent>,
    max_size: usize,
}

impl BoundedEventLog {
    fn push(&mut self, event: SecurityEvent) {
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }
}
```

**Configure Memory Limits**
```toml
[monitoring]
max_events_in_memory = 1000
cleanup_interval = 300  # seconds
```

## Diagnostic Tools

### 1. Security Health Check Script

```bash
#!/bin/bash
# security_health_check.sh

echo "=== AvoRed CMS Security Health Check ==="

# Check service status
echo "1. Service Status:"
systemctl is-active avored-cms

# Check configuration
echo "2. Configuration:"
if [ -f "config/security.toml" ]; then
    echo "✓ Security configuration found"
else
    echo "✗ Security configuration missing"
fi

# Test LDAP connection
echo "3. LDAP Connectivity:"
if ldapsearch -x -H "$LDAP_URL" -D "$LDAP_BIND_DN" -w "$LDAP_BIND_PASSWORD" -s base > /dev/null 2>&1; then
    echo "✓ LDAP connection successful"
else
    echo "✗ LDAP connection failed"
fi

# Check database
echo "4. Database Connectivity:"
if psql "$DATABASE_URL" -c "SELECT 1;" > /dev/null 2>&1; then
    echo "✓ Database connection successful"
else
    echo "✗ Database connection failed"
fi

# Check logs
echo "5. Security Logs:"
if [ -f "logs/security.log" ] && [ -s "logs/security.log" ]; then
    echo "✓ Security logs present"
    echo "   Last 5 entries:"
    tail -5 logs/security.log
else
    echo "✗ Security logs missing or empty"
fi

# Check disk space
echo "6. Disk Space:"
df -h | grep -E "(/$|/var|/opt)"

echo "=== Health Check Complete ==="
```

### 2. Performance Monitoring Script

```bash
#!/bin/bash
# performance_monitor.sh

echo "=== Performance Monitoring ==="

# CPU usage
echo "CPU Usage:"
top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1

# Memory usage
echo "Memory Usage:"
free -h

# Network connections
echo "Network Connections:"
netstat -an | grep :8080 | wc -l

# Response time test
echo "Response Time Test:"
curl -w "Time: %{time_total}s\n" -o /dev/null -s http://localhost:8080/health

# Log analysis
echo "Recent Errors:"
grep -i error logs/security.log | tail -5
```

---

For additional support, please refer to the main [Security Documentation](SECURITY.md) or contact the security team.
