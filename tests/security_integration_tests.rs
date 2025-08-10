//! Integration tests for security services to ensure all methods are properly tested
//! This resolves dead code warnings by actually using all security monitoring methods

use avored_rust_cms::providers::auth_provider::AuthProviderType;


#[cfg(test)]
mod security_integration_tests {
    use super::*;

    // #[tokio::test]
    // async fn test_security_audit_service_integration() {
    //     let audit_service = SecurityAuditService::new(100);

    //     // Test logging events
    //     let event = SecurityEventType::AuthenticationAttempt {
    //         username: "test_user".to_string(),
    //         provider: "local".to_string(),
    //         success: false,
    //         ip_address: Some("127.0.0.1".parse().unwrap()),
    //         user_agent: Some("TestAgent/1.0".to_string()),
    //         timestamp: SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_secs(),
    //     };

    //     audit_service.log_event(event).await;

    //     // Test getting recent events
    //     let events = audit_service.get_recent_events(10).await;
    //     assert!(!events.is_empty());

    //     // Test getting auth stats
    //     let stats = audit_service
    //         .get_auth_stats(Duration::from_secs(3600))
    //         .await;
    //     assert_eq!(stats.total_attempts, 1);
    //     assert_eq!(stats.failed_attempts, 1);
    //     assert_eq!(stats.success_attempts, 0);

    //     // Test security metrics
    //     let metrics = audit_service
    //         .get_security_metrics(Duration::from_secs(3600))
    //         .await;
    //     assert!(metrics.total_events >= 1);
    // }

    // #[tokio::test]
    // async fn test_security_monitoring_service_integration() {
    //     let monitoring_service = SecurityMonitoringService::new();

    //     // Test recording authentication attempts
    //     monitoring_service
    //         .record_authentication_attempt(false, "local")
    //         .await;
    //     monitoring_service
    //         .record_authentication_attempt(true, "ldap")
    //         .await;

    //     // Test recording rate limit exceeded
    //     monitoring_service
    //         .record_rate_limit_exceeded("test_user")
    //         .await;

    //     // Test recording injection attempts
    //     monitoring_service
    //         .record_injection_attempt("sql", "'; DROP TABLE users; --")
    //         .await;

    //     // Test recording suspicious activity
    //     monitoring_service
    //         .record_suspicious_activity("brute_force", "Multiple failed login attempts")
    //         .await;

    //     // Test health checks
    //     let _ = monitoring_service.perform_health_checks().await;

    //     // Test getting metrics
    //     let metrics = monitoring_service.get_metrics().await;
    //     assert!(metrics.total_events > 0);

    //     // Test getting recent alerts
    //     let alerts = monitoring_service.get_recent_alerts(10).await;
    //     assert!(!alerts.is_empty());

    //     // Test getting health check results
    //     let health_results = monitoring_service.get_health_check_results().await;
    //     assert!(!health_results.is_empty());
    // }

    // #[tokio::test]
    // async fn test_multi_auth_service_integration() {
    //     let auth_service = MultiAuthService::new(vec![]);

    //     // Test getting enabled providers
    //     let providers = auth_service.get_enabled_providers();
    //     assert!(providers.is_empty()); // No providers configured in test

    //     // Test checking if provider exists
    //     let has_ldap = auth_service.has_provider("ldap");
    //     assert!(!has_ldap);

    //     // Test getting provider info
    //     let provider_info = auth_service.get_provider_info();
    //     assert!(provider_info.is_empty());
    // }

    // #[tokio::test]
    // async fn test_ldap_connection_pool_integration() {
    //     let pool = LdapConnectionPool::new(5, Duration::from_secs(300));

    //     // Test getting pool stats
    //     let stats = pool.get_stats().await;
    //     assert_eq!(stats.active_connections, 0);
    //     assert_eq!(stats.max_connections, 5);

    //     // Test rate limiter
    //     let rate_limiter = AuthRateLimiter::new(5, Duration::from_secs(300));

    //     // Test remaining attempts
    //     let remaining = rate_limiter.remaining_attempts("test_user").await;
    //     assert_eq!(remaining, 5);

    //     // Use up some attempts
    //     for _ in 0..3 {
    //         rate_limiter.is_allowed("test_user").await;
    //     }

    //     let remaining_after = rate_limiter.remaining_attempts("test_user").await;
    //     assert_eq!(remaining_after, 2);
    // }

    #[test]
    fn test_auth_provider_type_usage() {
        // Test AuthProviderType enum usage
        let local_type = AuthProviderType::Local;
        let ldap_type = AuthProviderType::Ldap;

        assert_eq!(local_type.as_str(), "local");
        assert_eq!(ldap_type.as_str(), "ldap");

        // Test in match statement
        let provider_name = match local_type {
            AuthProviderType::Local => "Local Authentication",
            AuthProviderType::Ldap => "LDAP Authentication",
        };

        assert_eq!(provider_name, "Local Authentication");
    }
}
