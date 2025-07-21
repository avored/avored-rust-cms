#[cfg(test)]
mod security_alert_integration_tests {
    use std::collections::BTreeMap;
    use surrealdb::sql::Value;
    use avored_rust_cms::models::security_alert_model::{
        AlertType, AlertSeverity, CreateSecurityAlertModel, SecurityAlertModel
    };
    use avored_rust_cms::services::security_alert_service::SecurityAlertService;
    use avored_rust_cms::repositories::security_alert_repository::SecurityAlertRepository;
    use surrealdb::kvs::Datastore;
    use surrealdb::dbs::Session;

    async fn setup_test_db() -> (Datastore, Session) {
        let datastore = Datastore::new("memory").await.unwrap();
        let session = Session::for_db("test", "test");
        (datastore, session)
    }

    async fn setup_service() -> (SecurityAlertService, Datastore, Session) {
        let (datastore, session) = setup_test_db().await;
        let repository = SecurityAlertRepository::new();
        let service = SecurityAlertService::new(repository);
        (service, datastore, session)
    }

    #[tokio::test]
    async fn test_create_and_retrieve_alert() {
        let (service, datastore, session) = setup_service().await;

        let create_model = CreateSecurityAlertModel {
            alert_id: "alert_test_001".to_string(),
            alert_type: AlertType::AuthenticationFailure,
            severity: AlertSeverity::Medium,
            message: "Multiple failed login attempts detected".to_string(),
            source: "192.168.1.100".to_string(),
            affected_resource: Some("/api/auth/login".to_string()),
            metadata: None,
        };

        // Create alert
        let created_alert = service.create_alert(&datastore, &session, create_model).await;
        assert!(created_alert.is_ok());
        let alert = created_alert.unwrap();

        // Verify created alert
        assert_eq!(alert.alert_id, "alert_test_001");
        assert!(matches!(alert.alert_type, AlertType::AuthenticationFailure));
        assert!(matches!(alert.severity, AlertSeverity::Medium));
        assert_eq!(alert.message, "Multiple failed login attempts detected");
        assert_eq!(alert.source, "192.168.1.100");
        assert!(!alert.resolved);

        // Retrieve by ID
        let retrieved_alert = service.get_alert_by_id(&datastore, &session, &alert.id).await;
        assert!(retrieved_alert.is_ok());
        let retrieved = retrieved_alert.unwrap();
        assert_eq!(retrieved.alert_id, alert.alert_id);
    }

    #[tokio::test]
    async fn test_auto_id_alert_creation() {
        let (service, datastore, session) = setup_service().await;

        // Create alert with auto-generated ID
        let created_alert = service.create_alert_auto_id(
            &datastore,
            &session,
            AlertType::InjectionAttempt,
            AlertSeverity::High,
            "SQL injection attempt detected".to_string(),
            "192.168.1.200".to_string(),
            Some("/api/users".to_string()),
            None,
        ).await;

        assert!(created_alert.is_ok());
        let alert = created_alert.unwrap();

        // Verify auto-generated ID
        assert!(alert.alert_id.starts_with("alert_"));
        assert!(matches!(alert.alert_type, AlertType::InjectionAttempt));
        assert!(matches!(alert.severity, AlertSeverity::High));
        assert_eq!(alert.message, "SQL injection attempt detected");
        assert!(!alert.resolved);
    }

    #[tokio::test]
    async fn test_alert_resolution() {
        let (service, datastore, session) = setup_service().await;

        // Create alert
        let create_model = CreateSecurityAlertModel {
            alert_id: "resolve_test_001".to_string(),
            alert_type: AlertType::SuspiciousActivity,
            severity: AlertSeverity::Low,
            message: "Unusual access pattern detected".to_string(),
            source: "192.168.1.150".to_string(),
            affected_resource: Some("/api/data".to_string()),
            metadata: None,
        };

        let created_alert = service.create_alert(&datastore, &session, create_model).await.unwrap();
        assert!(!created_alert.resolved);

        // Resolve alert
        let resolved_alert = service.resolve_alert(
            &datastore,
            &session,
            &created_alert.id,
            "admin_user_123".to_string(),
        ).await;

        assert!(resolved_alert.is_ok());
        let resolved = resolved_alert.unwrap();
        assert!(resolved.resolved);
        assert_eq!(resolved.resolved_by, Some("admin_user_123".to_string()));
        assert!(resolved.resolved_at.is_some());
    }

    #[tokio::test]
    async fn test_alerts_by_severity() {
        let (service, datastore, session) = setup_service().await;

        // Create alerts with different severities
        let severities = vec![
            AlertSeverity::Low,
            AlertSeverity::Medium,
            AlertSeverity::High,
            AlertSeverity::Critical,
            AlertSeverity::High,
            AlertSeverity::Critical,
        ];

        for (i, severity) in severities.iter().enumerate() {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("severity_test_{:03}", i),
                alert_type: AlertType::SuspiciousActivity,
                severity: severity.clone(),
                message: format!("Test alert with {:?} severity", severity),
                source: format!("192.168.1.{}", 100 + i),
                affected_resource: Some("/api/test".to_string()),
                metadata: None,
            };

            let _ = service.create_alert(&datastore, &session, create_model).await;
        }

        // Test filtering by high severity
        let high_alerts = service.get_unresolved_alerts_by_severity(
            &datastore,
            &session,
            AlertSeverity::High,
            1,
            10,
        ).await;

        assert!(high_alerts.is_ok());
        let high_alerts = high_alerts.unwrap();
        assert_eq!(high_alerts.data.len(), 2); // Two high severity alerts
        for alert in &high_alerts.data {
            assert!(matches!(alert.severity, AlertSeverity::High));
            assert!(!alert.resolved);
        }

        // Test filtering by critical severity
        let critical_alerts = service.get_unresolved_alerts_by_severity(
            &datastore,
            &session,
            AlertSeverity::Critical,
            1,
            10,
        ).await;

        assert!(critical_alerts.is_ok());
        let critical_alerts = critical_alerts.unwrap();
        assert_eq!(critical_alerts.data.len(), 2); // Two critical severity alerts
    }

    #[tokio::test]
    async fn test_alerts_by_type() {
        let (service, datastore, session) = setup_service().await;

        // Create alerts with different types
        let alert_types = vec![
            AlertType::AuthenticationFailure,
            AlertType::InjectionAttempt,
            AlertType::RateLimitExceeded,
            AlertType::AuthenticationFailure,
            AlertType::InjectionAttempt,
        ];

        for (i, alert_type) in alert_types.iter().enumerate() {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("type_test_{:03}", i),
                alert_type: alert_type.clone(),
                severity: AlertSeverity::Medium,
                message: format!("Test alert of type {:?}", alert_type),
                source: format!("192.168.1.{}", 200 + i),
                affected_resource: Some("/api/test".to_string()),
                metadata: None,
            };

            let _ = service.create_alert(&datastore, &session, create_model).await;
        }

        // Test filtering by authentication failure type
        let auth_alerts = service.get_alerts_by_type(
            &datastore,
            &session,
            AlertType::AuthenticationFailure,
            1,
            10,
        ).await;

        assert!(auth_alerts.is_ok());
        let auth_alerts = auth_alerts.unwrap();
        assert_eq!(auth_alerts.data.len(), 2); // Two authentication failure alerts
        for alert in &auth_alerts.data {
            assert!(matches!(alert.alert_type, AlertType::AuthenticationFailure));
        }

        // Test filtering by injection attempt type
        let injection_alerts = service.get_alerts_by_type(
            &datastore,
            &session,
            AlertType::InjectionAttempt,
            1,
            10,
        ).await;

        assert!(injection_alerts.is_ok());
        let injection_alerts = injection_alerts.unwrap();
        assert_eq!(injection_alerts.data.len(), 2); // Two injection attempt alerts
    }

    #[tokio::test]
    async fn test_alerts_by_source() {
        let (service, datastore, session) = setup_service().await;

        let test_source = "192.168.1.100";

        // Create multiple alerts from the same source
        for i in 0..3 {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("source_test_{:03}", i),
                alert_type: AlertType::SuspiciousActivity,
                severity: AlertSeverity::Medium,
                message: format!("Alert {} from same source", i),
                source: test_source.to_string(),
                affected_resource: Some(format!("/api/endpoint_{}", i)),
                metadata: None,
            };

            let _ = service.create_alert(&datastore, &session, create_model).await;
        }

        // Create alert from different source
        let create_model = CreateSecurityAlertModel {
            alert_id: "different_source_001".to_string(),
            alert_type: AlertType::SuspiciousActivity,
            severity: AlertSeverity::Medium,
            message: "Alert from different source".to_string(),
            source: "10.0.0.1".to_string(),
            affected_resource: Some("/api/other".to_string()),
            metadata: None,
        };
        let _ = service.create_alert(&datastore, &session, create_model).await;

        // Test filtering by source
        let source_alerts = service.get_alerts_by_source(
            &datastore,
            &session,
            test_source,
            1,
            10,
        ).await;

        assert!(source_alerts.is_ok());
        let source_alerts = source_alerts.unwrap();
        assert_eq!(source_alerts.data.len(), 3); // Three alerts from test_source
        for alert in &source_alerts.data {
            assert_eq!(alert.source, test_source);
        }
    }

    #[tokio::test]
    async fn test_critical_unresolved_alerts() {
        let (service, datastore, session) = setup_service().await;

        // Create mix of alerts with different severities and resolution status
        let test_cases = vec![
            (AlertSeverity::Critical, false), // Should be included
            (AlertSeverity::High, false),     // Should not be included
            (AlertSeverity::Critical, true),  // Should not be included (resolved)
            (AlertSeverity::Critical, false), // Should be included
            (AlertSeverity::Medium, false),   // Should not be included
        ];

        let mut created_alerts = Vec::new();
        for (i, (severity, should_resolve)) in test_cases.iter().enumerate() {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("critical_test_{:03}", i),
                alert_type: AlertType::DataBreachAttempt,
                severity: severity.clone(),
                message: format!("Critical test alert {}", i),
                source: format!("192.168.1.{}", 50 + i),
                affected_resource: Some("/api/sensitive".to_string()),
                metadata: None,
            };

            let alert = service.create_alert(&datastore, &session, create_model).await.unwrap();
            
            if *should_resolve {
                let _ = service.resolve_alert(
                    &datastore,
                    &session,
                    &alert.id,
                    "test_admin".to_string(),
                ).await;
            }
            
            created_alerts.push(alert);
        }

        // Get critical unresolved alerts
        let critical_alerts = service.get_critical_unresolved_alerts(&datastore, &session).await;
        assert!(critical_alerts.is_ok());
        let critical_alerts = critical_alerts.unwrap();

        // Should only have 2 critical unresolved alerts
        assert_eq!(critical_alerts.len(), 2);
        for alert in &critical_alerts {
            assert!(matches!(alert.severity, AlertSeverity::Critical));
            assert!(!alert.resolved);
        }
    }

    #[tokio::test]
    async fn test_alert_statistics() {
        let (service, datastore, session) = setup_service().await;

        // Create alerts with various severities and resolution status
        let test_data = vec![
            (AlertSeverity::Low, false),
            (AlertSeverity::Low, true),
            (AlertSeverity::Medium, false),
            (AlertSeverity::Medium, false),
            (AlertSeverity::High, false),
            (AlertSeverity::High, true),
            (AlertSeverity::Critical, false),
            (AlertSeverity::Critical, false),
            (AlertSeverity::Critical, true),
        ];

        for (i, (severity, should_resolve)) in test_data.iter().enumerate() {
            let create_model = CreateSecurityAlertModel {
                alert_id: format!("stats_test_{:03}", i),
                alert_type: AlertType::SuspiciousActivity,
                severity: severity.clone(),
                message: format!("Statistics test alert {}", i),
                source: format!("192.168.2.{}", 10 + i),
                affected_resource: Some("/api/stats".to_string()),
                metadata: None,
            };

            let alert = service.create_alert(&datastore, &session, create_model).await.unwrap();
            
            if *should_resolve {
                let _ = service.resolve_alert(
                    &datastore,
                    &session,
                    &alert.id,
                    "stats_admin".to_string(),
                ).await;
            }
        }

        // Get alert statistics
        let stats = service.get_alert_statistics(&datastore, &session).await;
        assert!(stats.is_ok());
        let stats = stats.unwrap();

        assert_eq!(stats.total_alerts, 9);
        assert_eq!(stats.total_unresolved, 6); // 6 unresolved alerts
        assert_eq!(stats.total_critical_unresolved, 2); // 2 critical unresolved
        assert_eq!(stats.total_low, 2);
        assert_eq!(stats.total_medium, 2);
        assert_eq!(stats.total_high, 2);
        assert_eq!(stats.total_critical, 3);
    }

    #[tokio::test]
    async fn test_alert_metadata_handling() {
        let (service, datastore, session) = setup_service().await;

        // Create metadata
        let mut metadata = BTreeMap::new();
        metadata.insert("attack_vector".to_string(), Value::from("XSS"));
        metadata.insert("payload".to_string(), Value::from("<script>alert('test')</script>"));
        metadata.insert("blocked".to_string(), Value::from(true));

        let create_model = CreateSecurityAlertModel {
            alert_id: "metadata_alert_001".to_string(),
            alert_type: AlertType::InjectionAttempt,
            severity: AlertSeverity::High,
            message: "XSS injection attempt blocked".to_string(),
            source: "192.168.1.99".to_string(),
            affected_resource: Some("/api/comments".to_string()),
            metadata: Some(metadata.clone()),
        };

        let created_alert = service.create_alert(&datastore, &session, create_model).await;
        assert!(created_alert.is_ok());
        let alert = created_alert.unwrap();

        // Verify metadata was stored correctly
        assert!(alert.metadata.is_some());
        let stored_metadata = alert.metadata.unwrap();
        assert_eq!(stored_metadata.get("attack_vector"), Some(&Value::from("XSS")));
        assert_eq!(stored_metadata.get("payload"), Some(&Value::from("<script>alert('test')</script>")));
        assert_eq!(stored_metadata.get("blocked"), Some(&Value::from(true)));
    }

    #[tokio::test]
    async fn test_alert_deletion() {
        let (service, datastore, session) = setup_service().await;

        // Create alert
        let create_model = CreateSecurityAlertModel {
            alert_id: "delete_test_001".to_string(),
            alert_type: AlertType::MalformedRequest,
            severity: AlertSeverity::Low,
            message: "Malformed request detected".to_string(),
            source: "192.168.1.77".to_string(),
            affected_resource: Some("/api/malformed".to_string()),
            metadata: None,
        };

        let created_alert = service.create_alert(&datastore, &session, create_model).await.unwrap();

        // Delete alert
        let delete_result = service.delete_alert(&datastore, &session, &created_alert.id).await;
        assert!(delete_result.is_ok());

        // Verify deletion
        let get_deleted = service.get_alert_by_id(&datastore, &session, &created_alert.id).await;
        assert!(get_deleted.is_err());
    }
}
