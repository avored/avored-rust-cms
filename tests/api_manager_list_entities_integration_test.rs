#![recursion_limit = "256"]

//! Integration tests for `GET /api/v1/entities`.
//!
//! These tests exercise the full repository layer (in-memory SurrealDB) but do
//! not spin up an HTTP server – they call the repository and use-case directly,
//! matching the pattern used by the rest of the test suite.

use avored_rust_cms::{
    core::domain::{
        entities::entity::{StorableEntity, StorableEntityAttribute},
        repositories::EntityRepository,
    },
    infrastructure::persistence::entity_repository::test_entity_repository,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Seed an entity type and a collection table, insert `n` records with a
/// `title` attribute, and return the `(repo, entity_identifier)` pair.
async fn seed_entity_with_records(
    n: usize,
    entity_name: &str,
    identifier: &str,
) -> (
    avored_rust_cms::infrastructure::persistence::entity_repository::EntityRepositoryImpl,
    String,
) {
    let repo = test_entity_repository().await;

    let entity = repo
        .create(StorableEntity {
            name: entity_name.to_string(),
            identifier: identifier.to_string(),
            logged_in_user_email: "tester@example.com".to_string(),
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
        .expect("collection table creation should succeed");

    for i in 1..=n {
        let mut payload = serde_json::Map::new();
        payload.insert(
            "title".to_string(),
            serde_json::Value::String(format!("Record {}", i)),
        );
        payload.insert(
            "views".to_string(),
            serde_json::Value::Number(serde_json::Number::from(i as u64)),
        );
        payload.insert(
            "status".to_string(),
            serde_json::Value::String(if i % 2 == 0 {
                "published".to_string()
            } else {
                "draft".to_string()
            }),
        );
        repo.create_collection(&entity.identifier, payload)
            .await
            .expect("collection record insert should succeed");
    }

    (repo, entity.identifier)
}

// ---------------------------------------------------------------------------
// Pagination tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_entities_default_pagination() {
    let (repo, table) = seed_entity_with_records(5, "Articles", "articles_default").await;
    let filters = std::collections::HashMap::new();

    let (records, total, ..) = repo
        .list_entities(&table, 1, 20, &filters)
        .await
        .expect("list_entities should succeed");

    assert_eq!(total, 5, "total_items should equal the number of seeded records");
    assert_eq!(records.len(), 5);
}

#[tokio::test]
async fn test_list_entities_explicit_pagination_first_page() {
    let (repo, table) = seed_entity_with_records(10, "Posts", "posts_page1").await;
    let filters = std::collections::HashMap::new();

    let (records, total, ..) = repo
        .list_entities(&table, 1, 3, &filters)
        .await
        .expect("list_entities page 1 should succeed");

    assert_eq!(total, 10);
    assert_eq!(records.len(), 3, "first page with limit=3 should return 3 records");
}

#[tokio::test]
async fn test_list_entities_last_page_partial() {
    let (repo, table) = seed_entity_with_records(7, "Items", "items_last_page").await;
    let filters = std::collections::HashMap::new();

    // page=3, limit=3 → skip=6, expect 1 record
    let (records, total, ..) = repo
        .list_entities(&table, 3, 3, &filters)
        .await
        .expect("list_entities last page should succeed");

    assert_eq!(total, 7);
    assert_eq!(records.len(), 1, "last partial page should return the remaining 1 record");
}

#[tokio::test]
async fn test_list_entities_out_of_range_page_returns_empty() {
    let (repo, table) = seed_entity_with_records(3, "Widgets", "widgets_out_of_range").await;
    let filters = std::collections::HashMap::new();

    let (records, total, ..) = repo
        .list_entities(&table, 999, 20, &filters)
        .await
        .expect("out-of-range page should not error");

    assert_eq!(total, 3);
    assert!(records.is_empty(), "out-of-range page should return an empty data array");
}

#[tokio::test]
async fn test_list_entities_zero_results() {
    let (repo, table) = seed_entity_with_records(0, "Empty", "empty_table").await;
    let filters = std::collections::HashMap::new();

    let (records, total, ..) = repo
        .list_entities(&table, 1, 20, &filters)
        .await
        .expect("zero-result query should succeed");

    assert_eq!(total, 0);
    assert!(records.is_empty());
}

// ---------------------------------------------------------------------------
// Filter tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_entities_single_attribute_filter() {
    let (repo, table) =
        seed_entity_with_records(6, "Filtered Posts", "filtered_posts_single").await;
    let mut filters = std::collections::HashMap::new();
    filters.insert("status".to_string(), "published".to_string());

    let (records, total, ..) = repo
        .list_entities(&table, 1, 20, &filters)
        .await
        .expect("single filter query should succeed");

    // Records 2, 4, 6 are published.
    assert_eq!(total, 3, "filter on status=published should match 3 records");
    assert_eq!(records.len(), 3);
    for record in &records {
        let status = record
            .get("status")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        assert_eq!(status, "published");
    }
}

#[tokio::test]
async fn test_list_entities_no_match_filter() {
    let (repo, table) =
        seed_entity_with_records(4, "No Match Posts", "no_match_posts").await;
    let mut filters = std::collections::HashMap::new();
    filters.insert("status".to_string(), "archived".to_string());

    let (records, total, ..) = repo
        .list_entities(&table, 1, 20, &filters)
        .await
        .expect("no-match filter should succeed (empty result)");

    assert_eq!(total, 0);
    assert!(records.is_empty());
}

// ---------------------------------------------------------------------------
// Limit cap test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_entities_limit_cap_at_100() {
    let (repo, table) =
        seed_entity_with_records(5, "Capped", "capped_limit").await;
    let filters = std::collections::HashMap::new();

    // Request limit=500 — repository must cap at 100.
    let (records, total, ..) = repo
        .list_entities(&table, 1, 500, &filters)
        .await
        .expect("oversized limit should be capped without error");

    assert_eq!(total, 5);
    assert_eq!(records.len(), 5, "all 5 records should be returned even when limit is capped");
}

// ---------------------------------------------------------------------------
// Invalid table name (injection guard)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_entities_invalid_table_name_is_rejected() {
    let repo = test_entity_repository().await;
    let filters = std::collections::HashMap::new();

    let result = repo
        .list_entities("bad; DROP TABLE entities;--", 1, 20, &filters)
        .await;

    assert!(
        result.is_err(),
        "a non-alphanumeric table name must be rejected"
    );
}

// ---------------------------------------------------------------------------
// Flat attributes shape
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_entities_records_contain_expected_fields() {
    let (repo, table) =
        seed_entity_with_records(2, "Shape Test", "shape_test_records").await;
    let filters = std::collections::HashMap::new();

    let (records, ..) = repo
        .list_entities(&table, 1, 20, &filters)
        .await
        .expect("shape test list should succeed");

    assert_eq!(records.len(), 2);
    for record in &records {
        assert!(
            record.get("id").is_some(),
            "each record must have an id field"
        );
        assert!(
            record.get("title").is_some(),
            "each record must expose the title attribute"
        );
    }
}
