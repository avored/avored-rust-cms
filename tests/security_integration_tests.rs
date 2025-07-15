use avored_rust_cms::services::ldap_connection_pool::AuthRateLimiter;
use avored_rust_cms::services::security_audit_service::{SecurityAuditService, SecurityEvent};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

#[cfg(test)]
mod rate_limiting_security_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiting_prevents_brute_force() {
        let rate_limiter = AuthRateLimiter::new(3, Duration::from_secs(60)); // 3 attempts per minute
        let identifier = "test_user";

        // First 3 attempts should be allowed
        for i in 1..=3 {
            let allowed = rate_limiter.is_allowed(identifier).await;
            assert!(allowed, "Attempt {} should be allowed", i);
        }

        // 4th attempt should be blocked
        let blocked = rate_limiter.is_allowed(identifier).await;
        assert!(!blocked, "4th attempt should be blocked by rate limiter");

        // 5th attempt should also be blocked
        let still_blocked = rate_limiter.is_allowed(identifier).await;
        assert!(!still_blocked, "5th attempt should still be blocked");
    }

    #[tokio::test]
    async fn test_rate_limiting_per_user_isolation() {
        let rate_limiter = AuthRateLimiter::new(2, Duration::from_secs(60));

        // Exhaust rate limit for user1
        assert!(rate_limiter.is_allowed("user1").await);
        assert!(rate_limiter.is_allowed("user1").await);
        assert!(!rate_limiter.is_allowed("user1").await); // Should be blocked

        // user2 should still be allowed
        assert!(rate_limiter.is_allowed("user2").await);
        assert!(rate_limiter.is_allowed("user2").await);
        assert!(!rate_limiter.is_allowed("user2").await); // Now user2 is also blocked

        // user3 should still be allowed
        assert!(rate_limiter.is_allowed("user3").await);
    }

    #[tokio::test]
    async fn test_rate_limiting_sliding_window() {
        let rate_limiter = AuthRateLimiter::new(2, Duration::from_millis(100)); // Very short window for testing
        let identifier = "sliding_test_user";

        // Use up the rate limit
        assert!(rate_limiter.is_allowed(identifier).await);
        assert!(rate_limiter.is_allowed(identifier).await);
        assert!(!rate_limiter.is_allowed(identifier).await);

        // Wait for the window to slide
        sleep(Duration::from_millis(150)).await;

        // Should be allowed again
        assert!(rate_limiter.is_allowed(identifier).await);
    }

    #[tokio::test]
    async fn test_remaining_attempts_calculation() {
        let rate_limiter = AuthRateLimiter::new(5, Duration::from_secs(60));
        let identifier = "remaining_test_user";

        // Initially should have 5 attempts remaining
        let remaining = rate_limiter.remaining_attempts(identifier).await;
        assert_eq!(remaining, 5);

        // After one attempt, should have 4 remaining
        rate_limiter.is_allowed(identifier).await;
        let remaining = rate_limiter.remaining_attempts(identifier).await;
        assert_eq!(remaining, 4);

        // After two more attempts, should have 2 remaining
        rate_limiter.is_allowed(identifier).await;
        rate_limiter.is_allowed(identifier).await;
        let remaining = rate_limiter.remaining_attempts(identifier).await;
        assert_eq!(remaining, 2);
    }
}

#[cfg(test)]
mod security_audit_tests {
    use super::*;

    #[tokio::test]
    async fn test_security_event_logging() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Log a failed authentication attempt
        let event = SecurityEvent::AuthenticationAttempt {
            username: "test_user".to_string(),
            provider: "ldap".to_string(),
            success: false,
            ip_address: Some("192.168.1.100".parse().unwrap()),
            user_agent: Some("TestAgent/1.0".to_string()),
            timestamp,
        };

        audit_service.log_event(event).await;

        // Retrieve recent events
        let recent_events = audit_service.get_recent_events(10).await;
        assert_eq!(recent_events.len(), 1);

        match &recent_events[0] {
            SecurityEvent::AuthenticationAttempt {
                username, success, ..
            } => {
                assert_eq!(username, "test_user");
                assert!(!success);
            }
            _ => panic!("Expected AuthenticationAttempt event"),
        }
    }

    #[tokio::test]
    async fn test_suspicious_activity_detection() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Simulate multiple failed authentication attempts (brute force pattern)
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

        // Check that suspicious activity was detected
        let recent_events = audit_service.get_recent_events(20).await;
        let suspicious_events: Vec<_> = recent_events
            .iter()
            .filter(|event| matches!(event, SecurityEvent::SuspiciousActivity { .. }))
            .collect();

        assert!(
            !suspicious_events.is_empty(),
            "Should detect suspicious activity"
        );
    }

    #[tokio::test]
    async fn test_authentication_statistics() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Log various authentication attempts
        let events = vec![
            SecurityEvent::AuthenticationAttempt {
                username: "user1".to_string(),
                provider: "ldap".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
                timestamp,
            },
            SecurityEvent::AuthenticationAttempt {
                username: "user2".to_string(),
                provider: "local".to_string(),
                success: false,
                ip_address: None,
                user_agent: None,
                timestamp,
            },
            SecurityEvent::AuthenticationAttempt {
                username: "user3".to_string(),
                provider: "ldap".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
                timestamp,
            },
        ];

        for event in events {
            audit_service.log_event(event).await;
        }

        // Get authentication statistics
        let stats = audit_service
            .get_auth_stats(Duration::from_secs(3600))
            .await;

        assert_eq!(stats.total_attempts, 3);
        assert_eq!(stats.successful_attempts, 2);
        assert_eq!(stats.failed_attempts, 1);
        assert_eq!(stats.provider_stats.get("ldap"), Some(&2));
        assert_eq!(stats.provider_stats.get("local"), Some(&1));
    }

    #[tokio::test]
    async fn test_security_metrics() {
        let audit_service = SecurityAuditService::new(100);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Log various security events
        let events = vec![
            SecurityEvent::AuthenticationAttempt {
                username: "user1".to_string(),
                provider: "ldap".to_string(),
                success: false,
                ip_address: None,
                user_agent: None,
                timestamp,
            },
            SecurityEvent::AuthenticationAttempt {
                username: "user2".to_string(),
                provider: "ldap".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
                timestamp,
            },
            SecurityEvent::RateLimitExceeded {
                identifier: "user3".to_string(),
                ip_address: Some("192.168.1.100".parse().unwrap()),
                timestamp,
            },
            SecurityEvent::SuspiciousActivity {
                event_type: "brute_force".to_string(),
                details: "Multiple failed attempts".to_string(),
                ip_address: Some("192.168.1.100".parse().unwrap()),
                timestamp,
            },
        ];

        for event in events {
            audit_service.log_event(event).await;
        }

        // Get security metrics
        let metrics = audit_service
            .get_security_metrics(Duration::from_secs(3600))
            .await;

        assert_eq!(metrics.failed_auth_rate, 0.5); // 1 failed out of 2 total
        assert_eq!(metrics.rate_limited_requests, 1);
        assert_eq!(metrics.suspicious_activities, 1);
    }

    #[tokio::test]
    async fn test_event_retention_limit() {
        let audit_service = SecurityAuditService::new(5); // Small limit for testing
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Log more events than the retention limit
        for i in 0..10 {
            let event = SecurityEvent::AuthenticationAttempt {
                username: format!("user{}", i),
                provider: "ldap".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
                timestamp: timestamp + i,
            };
            audit_service.log_event(event).await;
        }

        // Should only retain the last 5 events
        let recent_events = audit_service.get_recent_events(10).await;
        assert_eq!(recent_events.len(), 5);

        // Should be the most recent events (user5-user9)
        for (i, event) in recent_events.iter().enumerate() {
            if let SecurityEvent::AuthenticationAttempt { username, .. } = event {
                let expected_user = format!("user{}", 9 - i); // Most recent first
                assert_eq!(username, &expected_user);
            }
        }
    }
}
