#![recursion_limit = "256"]

use avored_rust_cms::{
    core::domain::{
        entities::entity::{StorableEntity, StorableEntityAttribute},
        repositories::EntityRepository,
    },
    infrastructure::persistence::entity_repository::test_entity_repository,
};

#[tokio::test]
async fn test_collection_record_fetch_and_update_round_trip() {
    let repo = test_entity_repository().await;

    let entity = repo
        .create(StorableEntity {
            name: "Blog Post".to_string(),
            identifier: "blog_posts".to_string(),
            logged_in_user_email: "test@example.com".to_string(),
            attributes: vec![StorableEntityAttribute {
                name: "Title".to_string(),
                identifier: "title".to_string(),
                data_type: "string".to_string(),
                field_type: "text".to_string(),
            }],
        })
        .await
        .expect("entity create should succeed");

    repo.create_collection_table(&entity.identifier)
        .await
        .expect("collection table should be created");

    let mut payload = serde_json::Map::new();
    payload.insert("title".to_string(), serde_json::Value::String("Initial title".to_string()));
    payload.insert("status".to_string(), serde_json::Value::String("draft".to_string()));

    let created = repo
        .create_collection(&entity.identifier, payload)
        .await
        .expect("collection record should be created");

    let record_id = created
        .get("id")
        .and_then(serde_json::Value::as_str)
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .to_string();

    let fetched = repo
        .fetch_collection_by_id(&entity.identifier, &record_id)
        .await
        .expect("collection fetch should succeed");

    assert_eq!(fetched.get("title").and_then(serde_json::Value::as_str), Some("Initial title"));

    let mut update = serde_json::Map::new();
    update.insert("title".to_string(), serde_json::Value::String("Updated title".to_string()));
    update.insert("status".to_string(), serde_json::Value::String("published".to_string()));

    let updated = repo
        .update_collection_by_id(&entity.identifier, &record_id, update)
        .await
        .expect("collection update should succeed");

    assert_eq!(updated.get("title").and_then(serde_json::Value::as_str), Some("Updated title"));
    assert_eq!(updated.get("status").and_then(serde_json::Value::as_str), Some("published"));

    let deleted = repo
        .delete_collection_by_id(&entity.identifier, &record_id)
        .await
        .expect("collection delete should succeed");

    assert!(deleted);
    assert!(repo.fetch_collection_by_id(&entity.identifier, &record_id).await.is_err());
}
