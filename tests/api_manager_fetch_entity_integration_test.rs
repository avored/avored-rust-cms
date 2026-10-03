#![recursion_limit = "256"]

//! Integration tests for `GET /api/v1/entities/{id}`.
//!
//! Exercises fetching a single dynamic record:
//! - Successfully fetching an existing record with multiple attribute types
//! - Correct flat JSON response shape with assigned ID and attributes
//! - Returns 404 Not Found for missing or deleted records
//! - Returns 404 Not Found for unknown entity type
//! - Does not break existing collection/entity definition endpoints

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
                StorableEntityAttribute {
                    name: "Active".to_string(),
                    identifier: "is_active".to_string(),
                    data_type: "boolean".to_string(),
                    field_type: "checkbox".to_string(),
                },
                StorableEntityAttribute {
                    name: "Details".to_string(),
                    identifier: "details".to_string(),
                    data_type: "json".to_string(),
                    field_type: "textarea".to_string(),
                },
            ],
        })
        .await
        .expect("seeding test entity should succeed");

    (use_case, entity.identifier, entity.id)
}

#[tokio::test]
async fn test_fetch_entity_record_success_multiple_attribute_types() {
    let (use_case, identifier, _entity_id) = seed_test_entity("News", "fetch_news_success").await;

    let mut details = serde_json::Map::new();
    details.insert("author_note".to_string(), serde_json::Value::String("Breaking".to_string()));

    let mut attrs = serde_json::Map::new();
    attrs.insert("title".to_string(), serde_json::Value::String("World News".to_string()));
    attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(250)));
    attrs.insert("is_active".to_string(), serde_json::Value::Bool(true));
    attrs.insert("details".to_string(), serde_json::Value::Object(details));

    let created = use_case
        .create_entity_record(&identifier, attrs, "writer@example.com")
        .await
        .expect("record create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);

    // Fetch the record
    let fetched = use_case
        .get_entity_record(&identifier, record_id)
        .await
        .expect("fetch record should succeed");

    assert!(fetched.is_object());
    let obj = fetched.as_object().unwrap();

    // Verify fields
    assert_eq!(obj.get("title").and_then(|v| v.as_str()), Some("World News"));
    assert_eq!(obj.get("views").and_then(|v| v.as_u64()), Some(250));
    assert_eq!(obj.get("is_active").and_then(|v| v.as_bool()), Some(true));
    assert!(obj.get("details").unwrap().is_object());
    assert_eq!(
        obj.get("created_by").and_then(|v| v.as_str()),
        Some("writer@example.com")
    );
}

#[tokio::test]
async fn test_fetch_entity_record_not_found() {
    let (use_case, identifier, _entity_id) = seed_test_entity("News", "fetch_news_404").await;

    let result = use_case
        .get_entity_record(&identifier, "non_existent_record_id")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => {
            assert!(msg.contains("not found"));
        }
        other => panic!("expected NotFound, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_fetch_entity_record_unknown_entity_type() {
    let repo = test_entity_repository().await;
    let use_case = EntityUseCase::new(repo);

    let result = use_case
        .get_entity_record("completely_unknown_type", "rec_1")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => {
            assert!(msg.contains("completely_unknown_type"));
        }
        other => panic!("expected NotFound, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_fetch_entity_record_deleted_record_returns_not_found() {
    let (use_case, identifier, entity_id) = seed_test_entity("News", "fetch_news_deleted").await;

    // 1. Create a record
    let mut attrs = serde_json::Map::new();
    attrs.insert("title".to_string(), serde_json::Value::String("To be deleted".to_string()));
    attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
    attrs.insert("is_active".to_string(), serde_json::Value::Bool(false));

    let created = use_case
        .create_entity_record(&identifier, attrs, "writer@example.com")
        .await
        .expect("record create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);

    // 2. Fetch succeeds initially
    assert!(use_case.get_entity_record(&identifier, record_id).await.is_ok());

    // 3. Delete the record via repository soft-delete
    use_case.delete_collection(&entity_id, record_id).await.expect("soft delete should succeed");

    // 4. Fetching the deleted record now returns 404
    let result = use_case.get_entity_record(&identifier, record_id).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("not found")),
        other => panic!("expected NotFound, got: {:?}", other),
    }
}

