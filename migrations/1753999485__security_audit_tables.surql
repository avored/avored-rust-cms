-- Security Audit Tables Migration
-- Created: 2025-01-20
-- Purpose: Add comprehensive security auditing and alerting capabilities
-- Related Issue: https://github.com/avored/avored-rust-cms/issues/320

-- =====================================================
-- SECURITY AUDITS TABLE
-- =====================================================

-- Define the security_audits table with full schema
DEFINE TABLE security_audits SCHEMAFULL;

-- Primary identifier
DEFINE FIELD security_audit_id ON security_audits TYPE string 
    ASSERT $value != NONE AND string::len($value) > 0;

-- Admin user reference (optional for anonymous attempts)
DEFINE FIELD admin_user_id ON security_audits TYPE option<string>;

-- Session tracking
DEFINE FIELD session_id ON security_audits TYPE option<string>;

-- IP address with validation for both IPv4 and IPv6
DEFINE FIELD ip_address ON security_audits TYPE string 
    ASSERT $value != NONE AND (
        string::is::ipv4($value) OR 
        string::is::ipv6($value) OR 
        $value = "unknown"
    );

-- User agent for client identification
DEFINE FIELD user_agent ON security_audits TYPE option<string>;

-- API endpoint that was accessed
DEFINE FIELD endpoint ON security_audits TYPE option<string>;

-- Request method (GET, POST, PUT, DELETE, etc.)
DEFINE FIELD request_method ON security_audits TYPE option<string>;

-- Security metrics counters
DEFINE FIELD total_authentication_attempts ON security_audits TYPE int DEFAULT 0;
DEFINE FIELD failed_authentication_attempts ON security_audits TYPE int DEFAULT 0;
DEFINE FIELD blocked_injection_attempts ON security_audits TYPE int DEFAULT 0;
DEFINE FIELD rate_limited_requests ON security_audits TYPE int DEFAULT 0;
DEFINE FIELD suspicious_activities_detected ON security_audits TYPE int DEFAULT 0;
DEFINE FIELD security_violations ON security_audits TYPE int DEFAULT 0;

-- System metrics
DEFINE FIELD uptime_seconds ON security_audits TYPE int DEFAULT 0;

-- Security health score (0-100)
DEFINE FIELD security_health_score ON security_audits TYPE float DEFAULT 100.0 
    ASSERT $value >= 0.0 AND $value <= 100.0;

-- Timestamps
DEFINE FIELD created_at ON security_audits TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON security_audits TYPE datetime DEFAULT time::now();

-- Audit metadata (JSON object for additional context)
DEFINE FIELD metadata ON security_audits TYPE option<object>;

-- =====================================================
-- SECURITY ALERTS TABLE
-- =====================================================

-- Define the security_alerts table with full schema
DEFINE TABLE security_alerts SCHEMAFULL;

-- Primary identifier
DEFINE FIELD alert_id ON security_alerts TYPE string 
    ASSERT $value != NONE AND string::len($value) > 0;

-- Alert type with predefined categories
DEFINE FIELD alert_type ON security_alerts TYPE string 
    ASSERT $value INSIDE [
        'authentication_failure',
        'injection_attempt', 
        'rate_limit_exceeded',
        'suspicious_activity',
        'privilege_escalation',
        'data_breach_attempt',
        'unauthorized_access',
        'malformed_request',
        'brute_force_attack',
        'session_hijacking'
    ];

-- Severity levels
DEFINE FIELD severity ON security_alerts TYPE string 
    ASSERT $value INSIDE ['low', 'medium', 'high', 'critical'];

-- Alert message
DEFINE FIELD message ON security_alerts TYPE string 
    ASSERT $value != NONE AND string::len($value) > 0;

-- Source of the alert (IP, service, module)
DEFINE FIELD source ON security_alerts TYPE string 
    ASSERT $value != NONE;

-- What resource/endpoint was affected
DEFINE FIELD affected_resource ON security_alerts TYPE option<string>;

-- Additional alert context
DEFINE FIELD metadata ON security_alerts TYPE option<object>;

-- Alert resolution tracking
DEFINE FIELD resolved ON security_alerts TYPE bool DEFAULT false;
DEFINE FIELD resolved_at ON security_alerts TYPE option<datetime>;
DEFINE FIELD resolved_by ON security_alerts TYPE option<string>;

-- Timestamps
DEFINE FIELD created_at ON security_alerts TYPE datetime DEFAULT time::now();

-- =====================================================
-- INDEXES FOR PERFORMANCE
-- =====================================================

-- Security Audits Indexes
DEFINE INDEX idx_security_audits_ip ON security_audits FIELDS ip_address;
DEFINE INDEX idx_security_audits_user ON security_audits FIELDS admin_user_id;
DEFINE INDEX idx_security_audits_created ON security_audits FIELDS created_at;
DEFINE INDEX idx_security_audits_endpoint ON security_audits FIELDS endpoint;
DEFINE INDEX idx_security_audits_session ON security_audits FIELDS session_id;

-- Security Alerts Indexes
DEFINE INDEX idx_security_alerts_severity ON security_alerts FIELDS severity;
DEFINE INDEX idx_security_alerts_type ON security_alerts FIELDS alert_type;
DEFINE INDEX idx_security_alerts_created ON security_alerts FIELDS created_at;
DEFINE INDEX idx_security_alerts_resolved ON security_alerts FIELDS resolved;
DEFINE INDEX idx_security_alerts_source ON security_alerts FIELDS source;

-- Composite indexes for common queries
DEFINE INDEX idx_security_audits_user_created ON security_audits FIELDS admin_user_id, created_at;
DEFINE INDEX idx_security_alerts_severity_created ON security_alerts FIELDS severity, created_at;

-- =====================================================
-- SAMPLE DATA FOR TESTING (Optional - can be removed in production)
-- =====================================================

-- Sample security audit record
INSERT INTO security_audits {
    security_audit_id: "audit_001",
    admin_user_id: "admin_123",
    session_id: "session_456",
    ip_address: "192.168.1.100",
    user_agent: "Mozilla/5.0 (Test Browser)",
    endpoint: "/api/auth/login",
    request_method: "POST",
    total_authentication_attempts: 1,
    failed_authentication_attempts: 0,
    security_health_score: 100.0,
    metadata: {
        "test_data": true,
        "environment": "development"
    }
};

-- Sample security alert record
INSERT INTO security_alerts {
    alert_id: "alert_001",
    alert_type: "authentication_failure",
    severity: "medium",
    message: "Multiple failed login attempts detected",
    source: "192.168.1.100",
    affected_resource: "/api/auth/login",
    resolved: false,
    metadata: {
        "attempt_count": 3,
        "time_window": "5 minutes",
        "test_data": true
    }
};
