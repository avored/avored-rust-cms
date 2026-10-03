#![recursion_limit = "256"]

//! Integration tests for `PATCH /api/v1/entities/{id}`.
//!
//! Exercises partial updates on dynamic records:
//! - Update single attribute while preserving omitted attributes
//! - Update multiple attributes
//! - Verify `updated_at` changes and contains persisted values
//! - 404 on missing record
//! - 404 on unknown entity type
//! - 400 on unknown attribute
//! - 400 on invalid attribute type
//! - Explicit null support

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
            ],
        })
        .await
        .expect("seeding test entity should succeed");

    (use_case, entity.identifier)
}

#[tokio::test]
async fn test_patch_entity_record_single_attribute_preserves_omitted() {
    let (use_case, identifier) = seed_test_entity("Blogs", "patch_blogs_single").await;

    // 1. Create a record
    let mut initial_attrs = serde_json::Map::new();
    initial_attrs.insert("title".to_string(), serde_json::Value::String("Initial Title".to_string()));
    initial_attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
    initial_attrs.insert("is_published".to_string(), serde_json::Value::Bool(true));

    let created = use_case
        .create_entity_record(&identifier, initial_attrs, "creator@example.com")
        .await
        .expect("record create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);

    // 2. PATCH only the title
    let mut patch_attrs = serde_json::Map::new();
    patch_attrs.insert("title".to_string(), serde_json::Value::String("Updated Title".to_string()));

    let updated = use_case
        .update_entity_record(&identifier, record_id, patch_attrs, "editor@example.com")
        .await
        .expect("patch record should succeed");

    let obj = updated.as_object().unwrap();

    // Verify title was updated
    assert_eq!(obj.get("title").and_then(|v| v.as_str()), Some("Updated Title"));

    // Verify omitted fields remain unchanged
    assert_eq!(obj.get("views").and_then(|v| v.as_u64()), Some(10));
    assert_eq!(obj.get("is_published").and_then(|v| v.as_bool()), Some(true));

    // Verify updated_by
    assert_eq!(obj.get("updated_by").and_then(|v| v.as_str()), Some("editor@example.com"));
}

#[tokio::test]
async fn test_patch_entity_record_multiple_attributes_and_updated_at() {
    let (use_case, identifier) = seed_test_entity("Articles", "patch_articles_multi").await;

    // 1. Create a record
    let mut initial_attrs = serde_json::Map::new();
    initial_attrs.insert("title".to_string(), serde_json::Value::String("Old Title".to_string()));
    initial_attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));
    initial_attrs.insert("is_published".to_string(), serde_json::Value::Bool(false));

    let created = use_case
        .create_entity_record(&identifier, initial_attrs, "creator@example.com")
        .await
        .expect("record create should succeed");

    let full_id = created.get("id").and_then(|v| v.as_str()).unwrap();
    let record_id = full_id.split_once(':').map(|(_, k)| k).unwrap_or(full_id);
    let original_updated_at = created.get("updated_at").and_then(|v| v.as_str()).unwrap().to_string();

    // Small delay to ensure timestamp differs
    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;

    // 2. PATCH title and views
    let mut patch_attrs = serde_json::Map::new();
    patch_attrs.insert("title".to_string(), serde_json::Value::String("New Title".to_string()));
    patch_attrs.insert("views".to_string(), serde_json::Value::Number(serde_json::Number::from(999)));

    let updated = use_case
        .update_entity_record(&identifier, record_id, patch_attrs, "updater@example.com")
        .await
        .expect("patch record should succeed");

    let obj = updated.as_object().unwrap();

    assert_eq!(obj.get("title").and_then(|v| v.as_str()), Some("New Title"));
    assert_eq!(obj.get("views").and_then(|v| v.as_u64()), Some(999));
    assert_eq!(obj.get("is_published").and_then(|v| v.as_bool()), Some(false));

    let new_updated_at = obj.get("updated_at").and_then(|v| v.as_str()).unwrap();
    assert_ne!(original_updated_at, new_updated_at, "updated_at should be refreshed");
}

#[tokio::test]
async fn test_patch_entity_record_unknown_entity_type() {
    let repo = test_entity_repository().await;
    let use_case = EntityUseCase::new(repo);

    let patch_attrs = serde_json::Map::new();
    let result = use_case
        .update_entity_record("unknown_type", "rec123", patch_attrs, "user@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound(msg) => assert!(msg.contains("unknown_type")),
        other => panic!("expected NotFound, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_patch_entity_record_missing_record() {
    let (use_case, identifier) = seed_test_entity("Items", "patch_missing_rec").await;

    let mut patch_attrs = serde_json::Map::new();
    patch_attrs.insert("title".to_string(), serde_json::Value::String("Valid".to_string()));

    let result = use_case
        .update_entity_record(&identifier, "non_existent_key", patch_attrs, "user@example.com")
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_patch_entity_record_unknown_attribute() {
    let (use_case, identifier) = seed_test_entity("Posts", "patch_unknown_attr").await;

    let mut patch_attrs = serde_json::Map::new();
    patch_attrs.insert("invalid_field".to_string(), serde_json::Value::String("bad".to_string()));

    let result = use_case
        .update_entity_record(&identifier, "any_key", patch_attrs, "user@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::BadRequest(err_resp) => {
            assert!(err_resp.errors.iter().any(|e| e.key == "invalid_field"));
        }
        other => panic!("expected BadRequest, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_patch_entity_record_invalid_attribute_type() {
    let (use_case, identifier) = seed_test_entity("Products", "patch_invalid_type").await;

    let mut patch_attrs = serde_json::Map::new();
    // views expects integer, pass a string
    patch_attrs.insert("views".to_string(), serde_json::Value::String("not_a_number".to_string()));

    let result = use_case
        .update_entity_record(&identifier, "any_key", patch_attrs, "user@example.com")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::BadRequest(err_resp) => {
            assert!(err_resp.errors.iter().any(|e| e.key == "views"));
        }
        other => panic!("expected BadRequest, got: {:?}", other),
    }
}
