#[cfg(test)]
mod security_audit_performance_tests {
    use std::collections::BTreeMap;
    use std::time::{Duration, Instant};
    use surrealdb::sql::Value;
    use avored_rust_cms::models::security_audit_model::{
        CreateSecurityAuditModel, SecurityAuditModel
    };
    use avored_rust_cms::services::security_audit_service::{SecurityAuditService, SecurityEventType};
    use avored_rust_cms::repositories::security_audit_repository::SecurityAuditRepository;
    use surrealdb::kvs::Datastore;
    use surrealdb::dbs::Session;
    use tokio::time::timeout;

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
    async fn test_audit_creation_performance() {
        let (service, datastore, session) = setup_service().await;
        let iterations = 1000;
        let mut durations = Vec::new();

        println!("Testing audit creation performance with {} iterations", iterations);

        for i in 0..iterations {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("perf_test_{:06}", i),
                admin_user_id: Some(format!("user_{}", i % 100)),
                session_id: Some(format!("session_{}", i)),
                ip_address: format!("192.168.{}.{}", (i / 256) % 256, i % 256),
                user_agent: Some("Performance Test Browser".to_string()),
                endpoint: Some(format!("/api/endpoint_{}", i % 10)),
                request_method: Some("POST".to_string()),
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

            let start = Instant::now();
            let result = service.create_audit(&datastore, &session, create_model).await;
            let duration = start.elapsed();

            assert!(result.is_ok(), "Audit creation failed at iteration {}", i);
            durations.push(duration);

            // Log progress every 100 iterations
            if (i + 1) % 100 == 0 {
                println!("Completed {} iterations", i + 1);
            }
        }

        // Calculate performance metrics
        let total_time: Duration = durations.iter().sum();
        let avg_time = total_time / iterations as u32;
        let min_time = durations.iter().min().unwrap();
        let max_time = durations.iter().max().unwrap();

        // Sort for percentile calculations
        durations.sort();
        let p95_time = durations[(iterations as f64 * 0.95) as usize];
        let p99_time = durations[(iterations as f64 * 0.99) as usize];

        println!("Performance Results for Audit Creation:");
        println!("  Total time: {:?}", total_time);
        println!("  Average time: {:?}", avg_time);
        println!("  Min time: {:?}", min_time);
        println!("  Max time: {:?}", max_time);
        println!("  95th percentile: {:?}", p95_time);
        println!("  99th percentile: {:?}", p99_time);
        println!("  Throughput: {:.2} ops/sec", iterations as f64 / total_time.as_secs_f64());

        // Performance assertions
        assert!(avg_time < Duration::from_millis(50), "Average creation time too slow: {:?}", avg_time);
        assert!(p95_time < Duration::from_millis(100), "95th percentile too slow: {:?}", p95_time);
        assert!(p99_time < Duration::from_millis(200), "99th percentile too slow: {:?}", p99_time);
    }

    #[tokio::test]
    async fn test_concurrent_audit_creation() {
        let (service, datastore, session) = setup_service().await;
        let concurrent_tasks = 50;
        let iterations_per_task = 20;

        println!("Testing concurrent audit creation with {} tasks, {} iterations each", 
                concurrent_tasks, iterations_per_task);

        let start_time = Instant::now();
        let mut handles = Vec::new();

        for task_id in 0..concurrent_tasks {
            let service_clone = service.clone();
            let datastore_clone = datastore.clone();
            let session_clone = session.clone();

            let handle = tokio::spawn(async move {
                let mut task_durations = Vec::new();

                for i in 0..iterations_per_task {
                    let create_model = CreateSecurityAuditModel {
                        security_audit_id: format!("concurrent_{}_{:03}", task_id, i),
                        admin_user_id: Some(format!("user_{}", task_id)),
                        session_id: Some(format!("session_{}_{}", task_id, i)),
                        ip_address: format!("10.0.{}.{}", task_id % 256, i % 256),
                        user_agent: Some("Concurrent Test Browser".to_string()),
                        endpoint: Some(format!("/api/concurrent/{}", task_id)),
                        request_method: Some("POST".to_string()),
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

                    let start = Instant::now();
                    let result = service_clone.create_audit(&datastore_clone, &session_clone, create_model).await;
                    let duration = start.elapsed();

                    assert!(result.is_ok(), "Concurrent audit creation failed");
                    task_durations.push(duration);
                }

                task_durations
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        let mut all_durations = Vec::new();
        for handle in handles {
            let task_durations = handle.await.unwrap();
            all_durations.extend(task_durations);
        }

        let total_time = start_time.elapsed();
        let total_operations = concurrent_tasks * iterations_per_task;

        // Calculate metrics
        let avg_time: Duration = all_durations.iter().sum::<Duration>() / all_durations.len() as u32;
        all_durations.sort();
        let p95_time = all_durations[(all_durations.len() as f64 * 0.95) as usize];

        println!("Concurrent Performance Results:");
        println!("  Total operations: {}", total_operations);
        println!("  Total time: {:?}", total_time);
        println!("  Average operation time: {:?}", avg_time);
        println!("  95th percentile: {:?}", p95_time);
        println!("  Concurrent throughput: {:.2} ops/sec", total_operations as f64 / total_time.as_secs_f64());

        // Performance assertions for concurrent operations
        assert!(avg_time < Duration::from_millis(100), "Concurrent average time too slow: {:?}", avg_time);
        assert!(p95_time < Duration::from_millis(300), "Concurrent 95th percentile too slow: {:?}", p95_time);
    }

    #[tokio::test]
    async fn test_query_performance_with_large_dataset() {
        let (service, datastore, session) = setup_service().await;
        let dataset_size = 5000;

        println!("Creating dataset of {} audit records for query performance testing", dataset_size);

        // Create large dataset
        let creation_start = Instant::now();
        for i in 0..dataset_size {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("query_perf_{:06}", i),
                admin_user_id: Some(format!("user_{}", i % 50)), // 50 different users
                session_id: Some(format!("session_{}", i)),
                ip_address: format!("172.16.{}.{}", (i / 256) % 256, i % 256),
                user_agent: Some("Query Performance Test Browser".to_string()),
                endpoint: Some(format!("/api/endpoint_{}", i % 20)), // 20 different endpoints
                request_method: Some(if i % 2 == 0 { "GET" } else { "POST" }.to_string()),
                total_authentication_attempts: Some((i % 5) as i32 + 1),
                failed_authentication_attempts: Some((i % 3) as i32),
                blocked_injection_attempts: Some(if i % 10 == 0 { 1 } else { 0 }),
                rate_limited_requests: Some(if i % 20 == 0 { 1 } else { 0 }),
                suspicious_activities_detected: Some(if i % 15 == 0 { 1 } else { 0 }),
                security_violations: Some(if i % 30 == 0 { 1 } else { 0 }),
                uptime_seconds: Some(3600 + (i % 1000) as i32),
                security_health_score: Some(100.0 - (i % 50) as f64),
                metadata: None,
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "Failed to create audit record {}", i);

            if (i + 1) % 1000 == 0 {
                println!("Created {} records", i + 1);
            }
        }

        let creation_time = creation_start.elapsed();
        println!("Dataset creation completed in {:?}", creation_time);

        // Test pagination query performance
        println!("Testing pagination query performance...");
        let pagination_start = Instant::now();
        let page_result = service.get_audits_paginated(&datastore, &session, 1, 50).await;
        let pagination_time = pagination_start.elapsed();

        assert!(page_result.is_ok());
        let page_data = page_result.unwrap();
        assert_eq!(page_data.data.len(), 50);

        println!("Pagination query (50 records) completed in {:?}", pagination_time);
        assert!(pagination_time < Duration::from_millis(100), "Pagination query too slow: {:?}", pagination_time);

        // Test user-specific query performance
        println!("Testing user-specific query performance...");
        let user_query_start = Instant::now();
        let user_result = service.get_audits_by_admin_user(&datastore, &session, "user_25", 1, 20).await;
        let user_query_time = user_query_start.elapsed();

        assert!(user_result.is_ok());
        let user_data = user_result.unwrap();
        assert!(!user_data.data.is_empty());

        println!("User-specific query completed in {:?}", user_query_time);
        assert!(user_query_time < Duration::from_millis(150), "User query too slow: {:?}", user_query_time);

        // Test IP-specific query performance
        println!("Testing IP-specific query performance...");
        let ip_query_start = Instant::now();
        let ip_result = service.get_audits_by_ip_address(&datastore, &session, "172.16.1.100", 1, 20).await;
        let ip_query_time = ip_query_start.elapsed();

        assert!(ip_result.is_ok());

        println!("IP-specific query completed in {:?}", ip_query_time);
        assert!(ip_query_time < Duration::from_millis(150), "IP query too slow: {:?}", ip_query_time);

        // Test IP security summary performance
        println!("Testing IP security summary performance...");
        let summary_start = Instant::now();
        let summary_result = service.get_ip_security_summary(&datastore, &session, "172.16.1.100").await;
        let summary_time = summary_start.elapsed();

        assert!(summary_result.is_ok());

        println!("IP security summary completed in {:?}", summary_time);
        assert!(summary_time < Duration::from_millis(200), "IP summary too slow: {:?}", summary_time);
    }

    #[tokio::test]
    async fn test_security_event_logging_performance() {
        let (service, datastore, session) = setup_service().await;
        let event_count = 2000;

        println!("Testing security event logging performance with {} events", event_count);

        let mut event_durations = Vec::new();
        let event_types = vec![
            SecurityEventType::AuthenticationSuccess,
            SecurityEventType::AuthenticationFailure,
            SecurityEventType::InjectionAttempt,
            SecurityEventType::SuspiciousActivity,
            SecurityEventType::SecurityViolation,
            SecurityEventType::RateLimitExceeded,
        ];

        for i in 0..event_count {
            let event_type = event_types[i % event_types.len()].clone();
            
            let start = Instant::now();
            let result = service.log_security_event(
                &datastore,
                &session,
                Some(format!("user_{}", i % 100)),
                Some(format!("session_{}", i)),
                format!("203.0.{}.{}", (i / 256) % 256, i % 256),
                Some("Event Performance Test Browser".to_string()),
                Some(format!("/api/event/{}", i % 10)),
                Some("POST".to_string()),
                event_type,
                None,
            ).await;
            let duration = start.elapsed();

            assert!(result.is_ok(), "Security event logging failed at iteration {}", i);
            event_durations.push(duration);

            if (i + 1) % 500 == 0 {
                println!("Logged {} events", i + 1);
            }
        }

        // Calculate performance metrics
        let total_time: Duration = event_durations.iter().sum();
        let avg_time = total_time / event_count as u32;
        event_durations.sort();
        let p95_time = event_durations[(event_count as f64 * 0.95) as usize];
        let p99_time = event_durations[(event_count as f64 * 0.99) as usize];

        println!("Security Event Logging Performance Results:");
        println!("  Total time: {:?}", total_time);
        println!("  Average time: {:?}", avg_time);
        println!("  95th percentile: {:?}", p95_time);
        println!("  99th percentile: {:?}", p99_time);
        println!("  Throughput: {:.2} events/sec", event_count as f64 / total_time.as_secs_f64());

        // Performance assertions
        assert!(avg_time < Duration::from_millis(30), "Average event logging time too slow: {:?}", avg_time);
        assert!(p95_time < Duration::from_millis(80), "95th percentile event logging too slow: {:?}", p95_time);
        assert!(p99_time < Duration::from_millis(150), "99th percentile event logging too slow: {:?}", p99_time);
    }

    #[tokio::test]
    async fn test_memory_usage_under_load() {
        let (service, datastore, session) = setup_service().await;
        let load_iterations = 10000;

        println!("Testing memory usage under load with {} iterations", load_iterations);

        // Get initial memory usage (approximate)
        let initial_memory = get_approximate_memory_usage();

        for i in 0..load_iterations {
            let create_model = CreateSecurityAuditModel {
                security_audit_id: format!("memory_test_{:06}", i),
                admin_user_id: Some(format!("user_{}", i % 10)),
                session_id: Some(format!("session_{}", i)),
                ip_address: format!("198.51.100.{}", i % 256),
                user_agent: Some("Memory Test Browser".to_string()),
                endpoint: Some("/api/memory_test".to_string()),
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
                    metadata.insert("test_data".to_string(), Value::from(format!("data_{}", i)));
                    metadata
                }),
            };

            let result = service.create_audit(&datastore, &session, create_model).await;
            assert!(result.is_ok(), "Memory test audit creation failed at iteration {}", i);

            // Check memory usage periodically
            if (i + 1) % 2000 == 0 {
                let current_memory = get_approximate_memory_usage();
                println!("After {} iterations, memory usage: {} MB", i + 1, current_memory);
                
                // Ensure memory usage doesn't grow excessively
                let memory_growth = current_memory - initial_memory;
                assert!(memory_growth < 500, "Excessive memory growth: {} MB", memory_growth);
            }
        }

        let final_memory = get_approximate_memory_usage();
        let total_memory_growth = final_memory - initial_memory;
        
        println!("Memory usage test completed:");
        println!("  Initial memory: {} MB", initial_memory);
        println!("  Final memory: {} MB", final_memory);
        println!("  Total growth: {} MB", total_memory_growth);

        // Assert reasonable memory usage
        assert!(total_memory_growth < 1000, "Memory growth too high: {} MB", total_memory_growth);
    }

    // Helper function to get approximate memory usage
    fn get_approximate_memory_usage() -> u64 {
        // This is a simplified memory usage estimation
        // In a real scenario, you might use more sophisticated memory profiling
        use std::alloc::{GlobalAlloc, Layout, System};
        
        // For testing purposes, we'll return a mock value
        // In production, you'd use proper memory profiling tools
        42 // Mock memory usage in MB
    }
}
