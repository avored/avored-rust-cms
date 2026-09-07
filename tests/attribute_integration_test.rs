#![recursion_limit = "256"]

use avored_rust_cms::core::domain::entities::user::TokenClaims;
use avored_rust_cms::interfaces::api::attribute::paginate_attribute_handler::paginate_attributes_handler;
use avored_rust_cms::{
    avored_state::test_avored_state,
    core::domain::{entities::attribute::StorableAttribute, repositories::AttributeRepository},
    infrastructure::{
        middleware::auth_middleware, persistence::attribute_repository::test_attribute_repository,
    },
    interfaces::api::attribute::{
            create_attribute_handler::create_attribute_handler,
            delete_attribute_handler::delete_attribute_handler,
            fetch_attribute_handler::fetch_attribute_handler,
            update_attribute_handler::update_attribute_handler,
        },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    middleware,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use tower::ServiceExt;

#[tokio::test]
async fn test_attribute_repository_crud_lifecycle() {
    let repo = test_attribute_repository().await;

    let empty_count = repo.count().await.expect("count failed");
    assert_eq!(empty_count.total, 0);

    // 1. Create
    let storable = StorableAttribute {
        entity_id: "test_entity".to_string(),
        name: "Page Attribute".to_string(),
        identifier: "page".to_string(),
        logged_in_user_email: "test@example.com".to_string(),
        data_type: "string".to_string(),
        field_type: "text".to_string(),
    };
    let created = repo
        .create(storable)
        .await
        .expect("create attribute failed");
    assert_eq!(created.name, "Page Attribute");
    assert_eq!(created.identifier, "page");
    assert_eq!(created.deleted_at, None);

    // 2. Find by ID
    let id = &created.id.trim_start_matches("attributes:").to_string();
    let found = repo.find_by_id(id).await.expect("find failed");
    assert_eq!(found.identifier, "page");

    // 3. Find by identifier
    let found_ident = repo
        .find_by_identifier("page")
        .await
        .expect("find by identifier failed");
    assert!(found_ident.identifier == "page");

    // 4. Paginate
    let list = repo.paginate(0, 10).await.expect("paginate failed");
    assert_eq!(list.len(), 1);

    // 5. Update
    let updated = repo
        .update(
            &created.id,
            StorableAttribute {
                name: "Updated Page".to_string(),
                identifier: "page_v2".to_string(),
                logged_in_user_email: "test@example.com".to_string(),
                entity_id: "test_entity".to_string(),
                data_type: "string".to_string(),
                field_type: "text".to_string(),
            },
        )
        .await
        .expect("update failed");
    assert_eq!(updated.name, "Updated Page");
    assert_eq!(updated.identifier, "page_v2");

    // 6. Soft Delete
    let deleted = repo.delete(id).await.expect("delete failed");
    assert!(deleted);

    //@todo fix 7. Verify not found after soft delete
    let after_delete = repo.find_by_id(id).await;
    assert!(after_delete.is_err()); // Should return an error since the attribute is soft deleted

    let list_after = repo.paginate(0, 10).await.expect("paginate failed");
    assert_eq!(list_after.len(), 0);
}

#[tokio::test]
async fn test_attribute_rest_api_endpoints() {
    let state = test_avored_state().await;

    let now = Utc::now().timestamp() as usize;
    let token = encode(
        &Header::default(),
        &TokenClaims {
            sub: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            iat: now,
            exp: now + 3600,
        },
        &EncodingKey::from_secret(state.config.jwt_secret_key.as_bytes()),
    )
    .unwrap();

    let protected_routes = Router::new()
        .route(
            "/api/attributes",
            post(create_attribute_handler).get(paginate_attributes_handler),
        )
        .route(
            "/api/attributes/{id}",
            get(fetch_attribute_handler)
                .put(update_attribute_handler)
                .delete(delete_attribute_handler),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware::check_auth,
        ));
    let app = protected_routes.with_state(state);

    // 1. GET /api/attribute with no records
    let empty_list_req = Request::builder()
        .method("GET")
        .uri("/api/attributes")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(empty_list_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let empty_list_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(empty_list_json["total"], 0);
    assert_eq!(empty_list_json["data"].as_array().unwrap().len(), 0);

    // 2. POST /api/attribute
    let create_req = Request::builder()
        .method("POST")
        .uri("/api/attributes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"Product Attribute","identifier":"product", "entity_id": "test_entity_id", "data_type": "text", "field_type": "text"}"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(create_req).await.unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    if status != StatusCode::CREATED {
        eprintln!(
            "Response status: {}, body: {}",
            status,
            String::from_utf8_lossy(&body)
        );
    }
    assert_eq!(status, StatusCode::CREATED);

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["name"], "Product Attribute");
    assert_eq!(json["identifier"], "product");
    let attribute_id = json["id"]
        .as_str()
        .unwrap()
        .to_string()
        .trim_start_matches("attributes:")
        .to_string();

    // 3. GET /api/attribute
    let list_req = Request::builder()
        .method("GET")
        .uri("/api/attributes")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(list_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let list_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(list_json["total"], 1);

    // 4. GET /api/attribute/{id}
    let fetch_req = Request::builder()
        .method("GET")
        .uri(format!("/api/attributes/{}", attribute_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(fetch_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. PUT /api/attribute/{id}
    let update_req = Request::builder()
        .method("PUT")
        .uri(format!("/api/attributes/{}", attribute_id))
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::from(
            r#"{"name":"Product Updated","identifier":"product_updated","entity_id":"test_entity_id","data_type":"text","field_type":"text"}"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(update_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let update_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(update_json["name"], "Product Updated");

    // 6. DELETE /api/attribute/{id}
    let delete_req = Request::builder()
        .method("DELETE")
        .uri(format!("/api/attributes/{}", attribute_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(delete_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 6. Verify GET /api/attribute/{id} returns 500 after soft delete
    let fetch_deleted_req = Request::builder()
        .method("GET")
        .uri(format!("/api/attributes/{}", attribute_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(fetch_deleted_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
