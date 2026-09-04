#![recursion_limit = "256"]

use avored_rust_cms::{
    avored_state::test_avored_state,
    core::domain::{
        entities::entity::StorableEntity,
        repositories::EntityRepository,
    },
    infrastructure::persistence::entity_repository::test_entity_repository,
    interfaces::api::entity::{
        create_entity_handler, delete_entity_handler, fetch_entity_handler,
        paginate_entities_handler, update_entity_handler,
    },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::{get, post},
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_entity_repository_crud_lifecycle() {
    let repo = test_entity_repository().await;

    let empty_count = repo.count().await.expect("count failed");
    assert_eq!(empty_count.total, 0);

    // 1. Create
    let storable = StorableEntity {
        name: "Page Entity".to_string(),
        identifier: "page".to_string(),
        data_type: "PAGE".to_string(),
    };
    let created = repo.create(storable).await.expect("create entity failed");
    assert_eq!(created.name, "Page Entity");
    assert_eq!(created.identifier, "page");
    assert_eq!(created.deleted_at, None);

    // 2. Find by ID
    let found = repo.find_by_id(&created.id).await.expect("find failed");
    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier, "page");

    // 3. Find by identifier
    let found_ident = repo.find_by_identifier("page").await.expect("find by identifier failed");
    assert!(found_ident.is_some());

    // 4. Paginate
    let list = repo.paginate(0, 10).await.expect("paginate failed");
    assert_eq!(list.len(), 1);

    // 5. Update
    let updated = repo
        .update(
            &created.id,
            StorableEntity {
                name: "Updated Page".to_string(),
                identifier: "page_v2".to_string(),
                data_type: "PAGE".to_string(),
            },
        )
        .await
        .expect("update failed");
    assert_eq!(updated.name, "Updated Page");
    assert_eq!(updated.identifier, "page_v2");

    // 6. Soft Delete
    let deleted = repo.delete(&created.id).await.expect("delete failed");
    assert!(deleted);

    // 7. Verify not found after soft delete
    let after_delete = repo.find_by_id(&created.id).await.expect("find failed");
    assert!(after_delete.is_none());

    let list_after = repo.paginate(0, 10).await.expect("paginate failed");
    assert_eq!(list_after.len(), 0);
}

#[tokio::test]
async fn test_entity_rest_api_endpoints() {
    let state = test_avored_state().await;

    let app = Router::new()
        .route("/api/entity", post(create_entity_handler).get(paginate_entities_handler))
        .route("/api/entity/{id}", get(fetch_entity_handler).put(update_entity_handler).delete(delete_entity_handler))
        .with_state(state);

    // 1. GET /api/entity with no records
    let empty_list_req = Request::builder()
        .method("GET")
        .uri("/api/entity")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(empty_list_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let empty_list_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(empty_list_json["total"], 0);
    assert_eq!(empty_list_json["data"].as_array().unwrap().len(), 0);

    // 2. POST /api/entity
    let create_req = Request::builder()
        .method("POST")
        .uri("/api/entity")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"Product Entity","identifier":"product","data_type":"PRODUCT"}"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(create_req).await.unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    if status != StatusCode::CREATED {
        eprintln!("Response status: {}, body: {}", status, String::from_utf8_lossy(&body));
    }
    assert_eq!(status, StatusCode::CREATED);

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["name"], "Product Entity");
    assert_eq!(json["identifier"], "product");
    let entity_id = json["id"].as_str().unwrap().to_string();

    // 3. GET /api/entity
    let list_req = Request::builder()
        .method("GET")
        .uri("/api/entity")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(list_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let list_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(list_json["total"], 1);

    // 4. GET /api/entity/{id}
    let fetch_req = Request::builder()
        .method("GET")
        .uri(format!("/api/entity/{}", entity_id))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(fetch_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. PUT /api/entity/{id}
    let update_req = Request::builder()
        .method("PUT")
        .uri(format!("/api/entity/{}", entity_id))
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"Product Updated","identifier":"product_updated","data_type":"PRODUCT"}"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(update_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let update_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(update_json["name"], "Product Updated");

    // 6. DELETE /api/entity/{id}
    let delete_req = Request::builder()
        .method("DELETE")
        .uri(format!("/api/entity/{}", entity_id))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(delete_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 6. Verify GET /api/entity/{id} returns 404 after soft delete
    let fetch_deleted_req = Request::builder()
        .method("GET")
        .uri(format!("/api/entity/{}", entity_id))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(fetch_deleted_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
