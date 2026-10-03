#![recursion_limit = "256"]

//! Integration tests for `DELETE /api/v1/entities/{id}`.
//!
//! Exercises soft-deletion of dynamic records:
//! - Deleting an existing record returns 204 No Content
//! - Deleted record is no longer returned by fetch or list
//! - Subsequent delete on the same record returns 404 Not Found
//! - Deleting a non-existent record returns 404 Not Found
//! - Unknown entity type returns 404 Not Found
//! - Existing collection delete behavior remains intact

use avored_rust_cms::{
    core::{
        application::use_cases::EntityUseCase,
        domain::entities::entity::{StorableEntity, StorableEntityAttribute},
    },
    error::Error,
    infrastructure::persistence::entity_repository::test_entity_repository,
};

/// Helper to seed an entity with diverse attributes and initialize its collection table.
async fn seed_test_entity(
    name: &str,
    identifier: &str,
) -> (
    EntityUseCase<avored_rust_cms::infrastructure::persistence::entity_repository::EntityRepositoryImpl>,
    String,
    String,
) {
    let repo = test_entity_repository().await;
    let use_case = EntityUseCase::new(repo);

    let entity = use_case
        .create(StorableEntity {
            name: name.to_string(),
            identifier: identifier.to_string(),
            logged_in_user_email: "admin@example.com".to_string(),
            attributes: vec![
                StorableEntityAttribute {
                    name: "Title".to_string(),
                    identifier: "title".to_string(),
                    data_type: "string".to_string(),
                    field_type: "text".to_string(),
                },
                StorableEntityAttribute {
                    name: "Views".to_string(),
                    identifier: "views".to_string(),
                    data_type: "integer".to_string(),
                    field_type: "number".to_string(),
                },
            ],
        })
        .await
        .expect("seeding test entity should succeed");

    (use_case, entity.identifier, entity.id)
}

#[tokio::test]
async fn test_delete_entity_record_success_and_inaccessible() {
    let (use_case, identifier, _entity_id) = seed_test_entity("Articles", "del_articles_success").await;

    // 1. Create a record
    let mut attrs = serde_json::Map::new();
    attrs.insert("title".to_string(), serde_json::Value::String("Active Article".to_string()));
    attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(50)));

    let created = use_case
        .create_entity_record(&identifier, attrs, "author@example.com")
        .await
        .expect("record create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);

    // Verify it is accessible initially
    assert!(use_case.get_entity_record(&identifier, record_id).await.is_ok());

    // 2. Delete the record
    let deleted = use_case
        .delete_entity_record(&identifier, record_id)
        .await
        .expect("delete should succeed");

    assert!(deleted, "delete operation should return true");

    // 3. Verify record is no longer returned by fetch
    let fetch_result = use_case.get_entity_record(&identifier, record_id).await;
    assert!(fetch_result.is_err());
    match fetch_result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("not found")),
        other => panic!("expected NotFound, got: {:?}", other),
    }

    // 4. Verify record is not returned in list
    let filters = std::collections::HashMap::new();
    let (records, total, ..) = use_case
        .list_entities(&identifier, 1, 20, filters)
        .await
        .expect("list should succeed");

    assert_eq!(total, 0, "total records should be 0 after delete");
    assert_eq!(records.len(), 0);
}

#[tokio::test]
async fn test_delete_entity_record_repeated_returns_not_found() {
    let (use_case, identifier, _entity_id) = seed_test_entity("Items", "del_items_repeat").await;

    let mut attrs = serde_json::Map::new();
    attrs.insert("title".to_string(), serde_json::Value::String("Item".to_string()));
    let created = use_case
        .create_entity_record(&identifier, attrs, "author@example.com")
        .await
        .expect("create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);

    // First delete succeeds
    assert!(use_case.delete_entity_record(&identifier, record_id).await.is_ok());

    // Repeated delete fails with NotFound
    let result = use_case.delete_entity_record(&identifier, record_id).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("not found")),
        other => panic!("expected NotFound on repeated delete, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_delete_entity_record_missing_record() {
    let (use_case, identifier, _entity_id) = seed_test_entity("Products", "del_products_404").await;

    let result = use_case
        .delete_entity_record(&identifier, "non_existent_key")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("not found")),
        other => panic!("expected NotFound, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_delete_entity_record_unknown_entity_type() {
    let repo = test_entity_repository().await;
    let use_case = EntityUseCase::new(repo);

    let result = use_case
        .delete_entity_record("totally_unknown_type", "rec_123")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("totally_unknown_type")),
        other => panic!("expected NotFound, got: {:?}", other),
    }
}
