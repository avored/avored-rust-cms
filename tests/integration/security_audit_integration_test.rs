// Integration tests for security audit functionality
// Run with: cargo test --test security_audit_integration_test

#[cfg(test)]
mod security_audit_integration_tests {
    use std::collections::BTreeMap;
    use surrealdb::sql::Value;
    use avored_rust_cms::models::security_audit_model::{
        CreateSecurityAuditModel, SecurityAuditModel, UpdateSecurityAuditModel
    };
    use avored_rust_cms::services::security_audit_service::{SecurityAuditService, SecurityEventType};
    use avored_rust_cms::repositories::security_audit_repository::SecurityAuditRepository;
    use avored_rust_cms::providers::avored_database_provider::DB;
    use surrealdb::kvs::Datastore;
    use surrealdb::dbs::Session;

    async fn setup_test_db() -> (Datastore, Session) {
        let datastore = Datastore::new("memory").await.unwrap();
        let session = Session::for_db("test", "test");
        (datastore, session)
    }

    async fn setup_service() -> (SecurityAuditService, Datastore, Session) {
        let (datastore, session) = setup_test_db().await;
        let repository = SecurityAuditRepository::new();
        let service = SecurityAuditService::new(repository);
        (service, datastore, session)
    }

    #[tokio::test]
    async fn test_create_and_retrieve_audit_record() {
        let (service, datastore, session) = setup_service().await;
        let db = (&datastore, &session);

        let create_model = CreateSecurityAuditModel {
            security_audit_id: "test_audit_001".to_string(),
            admin_user_id: Some("user_123".to_string()),
            session_id: Some("session_456".to_string()),
            ip_address: "192.168.1.100".to_string(),
            user_agent: Some("Mozilla/5.0 Test Browser".to_string()),
            endpoint: Some("/api/login".to_string()),
            request_method: Some("POST".to_string()),
            total_authentication_attempts: Some(1),
            failed_authentication_attempts: Some(0),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(0),
            security_violations: Some(0),
            uptime_seconds: Some(3600),
            security_health_score: Some(95.5),
            metadata: None,
        };

        // Create audit record
        let created_audit = service.create_audit(&datastore, &session, create_model).await;
        assert!(created_audit.is_ok());
        let audit = created_audit.unwrap();

        // Verify created record
        assert_eq!(audit.security_audit_id, "test_audit_001");
        assert_eq!(audit.admin_user_id, Some("user_123".to_string()));
        assert_eq!(audit.ip_address, "192.168.1.100");
        assert_eq!(audit.security_health_score, 95.5);

        // Retrieve by ID
        let retrieved_audit = service.get_audit_by_id(&datastore, &session, &audit.id).await;
        assert!(retrieved_audit.is_ok());
        let retrieved = retrieved_audit.unwrap();
        assert_eq!(retrieved.security_audit_id, audit.security_audit_id);
    }

    #[tokio::test]
    async fn test_security_event_logging() {
        let (service, datastore, session) = setup_service().await;

        // Test authentication success event
        let auth_success = service.log_security_event(
            &datastore,
            &session,
            Some("user_456".to_string()),
            Some("session_789".to_string()),
            "10.0.0.1".to_string(),
            Some("Chrome/91.0".to_string()),
            Some("/api/auth/login".to_string()),
            Some("POST".to_string()),
            SecurityEventType::AuthenticationSuccess,
            None,
        ).await;

        assert!(auth_success.is_ok());
        let audit = auth_success.unwrap();
        assert_eq!(audit.total_authentication_attempts, 1);
        assert_eq!(audit.failed_authentication_attempts, 0);
        assert!(audit.security_health_score > 90.0);

        // Test authentication failure event
        let auth_failure = service.log_security_event(
            &datastore,
            &session,
            None,
            None,
            "10.0.0.1".to_string(),
            Some("Chrome/91.0".to_string()),
            Some("/api/auth/login".to_string()),
            Some("POST".to_string()),
            SecurityEventType::AuthenticationFailure,
            None,
        ).await;

        assert!(auth_failure.is_ok());
        let audit = auth_failure.unwrap();
        assert_eq!(audit.failed_authentication_attempts, 1);
        assert!(audit.security_health_score < 100.0);
    }

    #[tokio::test]
    async fn test_ip_security_summary() {
        let (service, datastore, session) = setup_service().await;
        let test_ip = "192.168.1.200";

        // Create multiple audit records for the same IP
        for i in 0..5 {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("test_audit_{:03}", i),
                admin_user_id: Some(format!("user_{}", i)),
                session_id: Some(format!("session_{}", i)),
                ip_address: test_ip.to_string(),
                user_agent: Some("Test Browser".to_string()),
                endpoint: Some("/api/test".to_string()),
                request_method: Some("GET".to_string()),
                total_authentication_attempts: Some(i as i32 + 1),
                failed_authentication_attempts: Some(if i > 2 { 1 } else { 0 }),
                blocked_injection_attempts: Some(0),
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(0),
                security_violations: Some(0),
                uptime_seconds: Some(3600),
                security_health_score: Some(95.0 - (i as f64 * 2.0)),
                metadata: None,
            };

            let _ = service.create_audit(&datastore, &session, create_model).await;
        }

        // Get IP security summary
        let summary = service.get_ip_security_summary(&datastore, &session, test_ip).await;
        assert!(summary.is_ok());
        let summary = summary.unwrap();

        assert_eq!(summary.ip_address, test_ip);
        assert_eq!(summary.total_records, 5);
        assert_eq!(summary.total_authentication_attempts, 15); // 1+2+3+4+5
        assert_eq!(summary.failed_authentication_attempts, 2); // Last 2 records
        assert!(summary.lowest_health_score < 95.0);
    }

    #[tokio::test]
    async fn test_audit_pagination() {
        let (service, datastore, session) = setup_service().await;

        // Create 15 audit records
        for i in 0..15 {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("pagination_test_{:03}", i),
                admin_user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                ip_address: format!("192.168.1.{}", 100 + i),
                user_agent: Some("Test Browser".to_string()),
                endpoint: Some("/api/test".to_string()),
                request_method: Some("GET".to_string()),
                total_authentication_attempts: Some(1),
                failed_authentication_attempts: Some(0),
                blocked_injection_attempts: Some(0),
                rate_limited_requests: Some(0),
                suspicious_activities_detected: Some(0),
                security_violations: Some(0),
                uptime_seconds: Some(3600),
                security_health_score: Some(100.0),
                metadata: None,
            };

            let _ = service.create_audit(&datastore, &session, create_model).await;
        }

        // Test pagination - page 1
        let page1 = service.get_audits_paginated(&datastore, &session, 1, 5).await;
        assert!(page1.is_ok());
        let page1 = page1.unwrap();
        assert_eq!(page1.data.len(), 5);
        assert_eq!(page1.pagination.current_page, 1);
        assert_eq!(page1.pagination.per_page, 5);
        assert!(page1.pagination.has_next_page);
        assert!(!page1.pagination.has_previous_page);

        // Test pagination - page 2
        let page2 = service.get_audits_paginated(&datastore, &session, 2, 5).await;
        assert!(page2.is_ok());
        let page2 = page2.unwrap();
        assert_eq!(page2.data.len(), 5);
        assert_eq!(page2.pagination.current_page, 2);
        assert!(page2.pagination.has_next_page);
        assert!(page2.pagination.has_previous_page);
    }

    #[tokio::test]
    async fn test_audit_update_and_delete() {
        let (service, datastore, session) = setup_service().await;

        // Create audit record
        let create_model = CreateSecurityAuditModel {
            security_audit_id: "update_test_001".to_string(),
            admin_user_id: Some("user_update".to_string()),
            session_id: Some("session_update".to_string()),
            ip_address: "192.168.1.250".to_string(),
            user_agent: Some("Update Test Browser".to_string()),
            endpoint: Some("/api/update".to_string()),
            request_method: Some("PUT".to_string()),
            total_authentication_attempts: Some(1),
            failed_authentication_attempts: Some(0),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(0),
            security_violations: Some(0),
            uptime_seconds: Some(1800),
            security_health_score: Some(98.0),
            metadata: None,
        };

        let created_audit = service.create_audit(&datastore, &session, create_model).await.unwrap();

        // Update audit record
        let update_model = UpdateSecurityAuditModel {
            total_authentication_attempts: Some(3),
            failed_authentication_attempts: Some(1),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(1),
            security_violations: Some(0),
            uptime_seconds: Some(3600),
            security_health_score: Some(92.0),
            metadata: None,
        };

        let updated_audit = service.update_audit(&datastore, &session, &created_audit.id, update_model).await;
        assert!(updated_audit.is_ok());
        let updated = updated_audit.unwrap();
        assert_eq!(updated.total_authentication_attempts, 3);
        assert_eq!(updated.failed_authentication_attempts, 1);
        assert_eq!(updated.suspicious_activities_detected, 1);
        assert_eq!(updated.security_health_score, 92.0);

        // Delete audit record
        let delete_result = service.delete_audit(&datastore, &session, &created_audit.id).await;
        assert!(delete_result.is_ok());

        // Verify deletion
        let get_deleted = service.get_audit_by_id(&datastore, &session, &created_audit.id).await;
        assert!(get_deleted.is_err());
    }

    #[tokio::test]
    async fn test_metadata_handling() {
        let (service, datastore, session) = setup_service().await;

        // Create metadata
        let mut metadata = BTreeMap::new();
        metadata.insert("request_id".to_string(), Value::from("req_12345"));
        metadata.insert("correlation_id".to_string(), Value::from("corr_67890"));
        metadata.insert("client_version".to_string(), Value::from("v1.2.3"));

        let create_model = CreateSecurityAuditModel {
            security_audit_id: "metadata_test_001".to_string(),
            admin_user_id: Some("user_metadata".to_string()),
            session_id: Some("session_metadata".to_string()),
            ip_address: "192.168.1.300".to_string(),
            user_agent: Some("Metadata Test Browser".to_string()),
            endpoint: Some("/api/metadata".to_string()),
            request_method: Some("POST".to_string()),
            total_authentication_attempts: Some(1),
            failed_authentication_attempts: Some(0),
            blocked_injection_attempts: Some(0),
            rate_limited_requests: Some(0),
            suspicious_activities_detected: Some(0),
            security_violations: Some(0),
            uptime_seconds: Some(3600),
            security_health_score: Some(100.0),
            metadata: Some(metadata.clone()),
        };

        let created_audit = service.create_audit(&datastore, &session, create_model).await;
        assert!(created_audit.is_ok());
        let audit = created_audit.unwrap();

        // Verify metadata was stored correctly
        assert!(audit.metadata.is_some());
        let stored_metadata = audit.metadata.unwrap();
        assert_eq!(stored_metadata.get("request_id"), Some(&Value::from("req_12345")));
        assert_eq!(stored_metadata.get("correlation_id"), Some(&Value::from("corr_67890")));
        assert_eq!(stored_metadata.get("client_version"), Some(&Value::from("v1.2.3")));
    }
}
