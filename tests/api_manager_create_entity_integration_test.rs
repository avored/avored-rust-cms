#![recursion_limit = "256"]

//! Integration tests for `POST /api/v1/entities`.
//!
//! Exercises the use-case layer (in-memory SurrealDB) directly:
//! - Successful dynamic record creation with assigned ID and returned record
//! - Validation of unknown entity type (404)
//! - Validation of attribute schemas: unknown attributes, wrong types (400)
//! - Support for multiple data types (string, integer, boolean, date, json)
//! - Ensuring /api/entities behavior is not broken

use avored_rust_cms::{
    core::{
        application::use_cases::EntityUseCase,
        domain::
            entities::entity::{StorableEntity, StorableEntityAttribute}
        ,
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
                    name: "Published".to_string(),
                    identifier: "is_published".to_string(),
                    data_type: "boolean".to_string(),
                    field_type: "checkbox".to_string(),
                },
                StorableEntityAttribute {
                    name: "Publish Date".to_string(),
                    identifier: "publish_date".to_string(),
                    data_type: "date".to_string(),
                    field_type: "date".to_string(),
                },
                StorableEntityAttribute {
                    name: "Metadata".to_string(),
                    identifier: "metadata".to_string(),
                    data_type: "json".to_string(),
                    field_type: "textarea".to_string(),
                },
            ],
        })
        .await
        .expect("seeding test entity should succeed");

    (use_case, entity.identifier)
}

#[tokio::test]
async fn test_create_entity_record_success() {
    let (use_case, identifier) = seed_test_entity("Articles", "test_articles").await;

    let mut attrs = serde_json::Map::new();
    attrs.insert(
        "title".to_string(),
        serde_json::Value::String("First Post".to_string()),
    );
    attrs.insert(
        "views".to_string(),
        serde_json::Value::Number(serde_json::Number::from(42)),
    );
    attrs.insert("is_published".to_string(), serde_json::Value::Bool(true));

    let created = use_case
        .create_entity_record(&identifier, attrs, "author@example.com")
        .await
        .expect("create record should succeed");

    assert!(created.is_object());
    let obj = created.as_object().unwrap();

    // ID should exist
    assert!(obj.get("id").is_some(), "persisted record must have an id");
    // Standard timestamp and author fields
    assert_eq!(
        obj.get("created_by").and_then(|v| v.as_str()),
        Some("author@example.com")
    );
    assert_eq!(
        obj.get("title").and_then(|v| v.as_str()),
        Some("First Post")
    );
    assert_eq!(
        obj.get("views").and_then(|v| v.as_u64()),
        Some(42)
    );
    assert_eq!(
        obj.get("is_published").and_then(|v| v.as_bool()),
        Some(true)
    );
}

#[tokio::test]
async fn test_create_entity_record_unknown_entity_type() {
    let repo = test_entity_repository().await;
    let use_case = EntityUseCase::new(repo);

    let attrs = serde_json::Map::new();
    let result = use_case
        .create_entity_record("non_existent_entity", attrs, "test@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => {
            assert!(msg.contains("non_existent_entity"));
        }
        other => panic!("expected NotFound error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_create_entity_record_unknown_attribute() {
    let (use_case, identifier) = seed_test_entity("Books", "test_books").await;

    let mut attrs = serde_json::Map::new();
    attrs.insert(
        "title".to_string(),
        serde_json::Value::String("Rust in Action".to_string()),
    );
    attrs.insert(
        "unknown_column".to_string(),
        serde_json::Value::String("bad".to_string()),
    );

    let result = use_case
        .create_entity_record(&identifier, attrs, "author@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::BadRequest(err_resp) => {
            assert!(!err_resp.status);
            assert!(err_resp.errors.iter().any(|e| e.key == "unknown_column"));
        }
        other => panic!("expected BadRequest error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_create_entity_record_invalid_attribute_type() {
    let (use_case, identifier) = seed_test_entity("Products", "test_products").await;

    let mut attrs = serde_json::Map::new();
    // views is defined as integer, pass a string
    attrs.insert(
        "views".to_string(),
        serde_json::Value::String("not-a-number".to_string()),
    );

    let result = use_case
        .create_entity_record(&identifier, attrs, "author@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::BadRequest(err_resp) => {
            assert!(!err_resp.status);
            assert!(err_resp.errors.iter().any(|e| e.key == "views"));
        }
        other => panic!("expected BadRequest error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_create_entity_record_supported_json_types() {
    let (use_case, identifier) = seed_test_entity("Complex", "test_complex").await;

    let mut meta = serde_json::Map::new();
    meta.insert("tag".to_string(), serde_json::Value::String("tech".to_string()));
    meta.insert(
        "sub_items".to_string(),
        serde_json::Value::Array(vec![
            serde_json::Value::from(1),
            serde_json::Value::from(2),
        ]),
    );

    let mut attrs = serde_json::Map::new();
    attrs.insert("title".to_string(), serde_json::Value::String("Rich Content".to_string()));
    attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(100)));
    attrs.insert("is_published".to_string(), serde_json::Value::Bool(false));
    attrs.insert("publish_date".to_string(), serde_json::Value::String("2026-10-04T00:00:00Z".to_string()));
    attrs.insert("metadata".to_string(), serde_json::Value::Object(meta));

    let created = use_case
        .create_entity_record(&identifier, attrs, "admin@example.com")
        .await
        .expect("creating record with multiple json types should succeed");

    let obj = created.as_object().unwrap();
    assert_eq!(obj.get("views").and_then(|v| v.as_u64()), Some(100));
    assert_eq!(obj.get("is_published").and_then(|v| v.as_bool()), Some(false));
    assert!(obj.get("metadata").unwrap().is_object());
}
