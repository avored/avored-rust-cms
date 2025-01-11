use crate::api::handlers::cms::all_pages_cms_api_handler::all_pages_cms_api_handler;
use crate::api::handlers::cms::sent_contact_us_email_handler::sent_contact_us_email_handler;
use crate::api::handlers::collection::collection_table_api_handler::collection_table_api_handler;
use crate::api::handlers::collection::fetch_collection_api_handler::fetch_collection_api_handler;
use crate::api::handlers::collection::store_collection_api_handler::store_collection_api_handler;
use crate::api::handlers::collection::update_collection_api_handler::update_collection_api_handler;
use crate::api::handlers::graphql::graphql_api_handler::graphql_api_handler;
use crate::api::handlers::misc::delete_demo_data_api_handler::delete_demo_data_api_handler;
use crate::api::handlers::misc::install_demo_data_api_handler::install_demo_data_api_handler;
use crate::api::handlers::misc::testing_api_handler::testing_api_handler;
use crate::api::handlers::{
    admin_user::admin_user_forgot_password_api_handler::admin_user_forgot_password_api_handler,
    admin_user::admin_user_login_api_handler::admin_user_login_api_handler,
    admin_user::admin_user_reset_password_api_handler::admin_user_reset_password_api_handler,
    admin_user::admin_user_table_api_handler::admin_user_table_api_handler,
    admin_user::change_password_api_handler::change_password_api_handler,
    admin_user::fetch_admin_user_api_handler::fetch_admin_user_api_handler,
    admin_user::logged_in_user_api_handler::logged_in_user_api_handler,
    admin_user::store_admin_user_api_handler::store_admin_user_api_handler,
    admin_user::update_admin_user_api_handler::update_admin_user_api_handler,
    asset::asset_table_api_handler::asset_table_api_handler,
    asset::create_folder_api_handler::create_folder_api_handler,
    asset::delete_asset_api_handler::delete_asset_api_handler,
    asset::delete_folder_api_handler::delete_folder_api_handler,
    asset::rename_asset_api_handler::rename_asset_api_handler,
    asset::store_asset_api_handler::store_asset_api_handler,
    cms::fetch_page_cms_api_handler::fetch_page_cms_api_handler,
    component::component_all_api_handler::component_all_api_handler,
    component::component_table_api_handler::component_table_api_handler,
    component::fetch_component_api_handler::fetch_component_api_handler,
    component::put_component_identifier_api_handler::put_component_identifier_api_handler,
    component::store_component_api_handler::store_component_api_handler,
    component::update_component_api_handler::update_component_api_handler,
    misc::health_check_api_handler::health_check_api_handler,
    misc::openapi_api_handler::openapi_api_handler,
    model::fetch_model_api_handler::fetch_model_api_handler,
    model::model_table_api_handler::model_table_api_handler,
    model::put_model_identifier_api_handler::put_model_identifier_api_handler,
    model::store_model_api_handler::store_model_api_handler,
    model::update_model_api_handler::update_model_api_handler,
    page::delete_page_handler::delete_page_handler,
    page::fetch_page_api_handler::fetch_page_api_handler,
    page::page_table_api_handler::page_table_api_handler,
    page::put_page_identifier_api_handler::put_page_identifier_api_handler,
    page::store_page_api_handler::store_page_api_handler,
    page::update_page_api_handler::update_page_api_handler,
    role::fetch_role_api_handler::fetch_role_api_handler,
    role::put_role_identifier_api_handler::put_role_identifier_api_handler,
    role::role_option_api_handler::role_option_api_handler,
    role::role_table_api_handler::role_table_api_handler,
    role::store_role_api_handler::store_role_api_handler,
    role::update_role_api_handler::update_role_api_handler,
    setting::setting_all_api_handler::setting_all_api_handler,
    setting::update_setting_all_api_handler::update_setting_all_api_handler,
    setup::post_setup_avored_handler::post_setup_avored_handler,
};
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use crate::middleware::validate_cms_authentication::validate_cms_authentication;
use crate::providers::avored_graphql_provider::AvoRedGraphqlSchema;
use crate::query::AvoRedQuery;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::routing::{delete, on, post, put, MethodFilter};
use axum::{middleware, routing::get, Extension, Router};
use juniper::{EmptyMutation, EmptySubscription};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::api::handlers::collection::collection_all_api_handler::collection_all_api_handler;
use crate::api::handlers::collection::put_collection_identifier_api_handler::put_collection_identifier_api_handler;

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .merge(admin_api_routes(state.clone()))
        .merge(cms_api_routes(state.clone()))
}

// Ideally cms routes will have all the frontend api calls in future more api will end points will be added
fn cms_api_routes(state: Arc<AvoRedState>) -> Router {
    let cors = get_cors_urls(state.clone());
    Router::new()
        .route("/cms/page/:page_id", get(fetch_page_cms_api_handler))
        .route("/cms/page", get(all_pages_cms_api_handler))
        .route(
            "/cms/sent-contact-us-email",
            post(sent_contact_us_email_handler),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            validate_cms_authentication,
        ))
        .with_state(state)
        .layer(cors)
}

fn admin_api_routes(state: Arc<AvoRedState>) -> Router {
    let cors = get_cors_urls(state.clone());
    let schema =
        AvoRedGraphqlSchema::new(AvoRedQuery, EmptyMutation::new(), EmptySubscription::new());

    Router::new()
        .route("/api/component", get(component_table_api_handler))
        .route("/api/component", post(store_component_api_handler))
        .route(
            "/api/component/:component_id",
            get(fetch_component_api_handler),
        )
        .route(
            "/api/component/:component_id",
            put(update_component_api_handler),
        )
        .route(
            "/api/put-component-identifier/:page_id",
            put(put_component_identifier_api_handler),
        )
        .route("/api/asset", get(asset_table_api_handler))
        .route("/api/asset", post(store_asset_api_handler))
        .route(
            "/api/rename-asset/:asset_id",
            post(rename_asset_api_handler),
        )
        .route("/api/create-folder", post(create_folder_api_handler))
        .route(
            "/api/delete-folder/:asset_id",
            delete(delete_folder_api_handler),
        )
        .route(
            "/api/delete-asset/:asset_id",
            delete(delete_asset_api_handler),
        )
        .route("/api/role-options", get(role_option_api_handler))
        .route("/api/role", get(role_table_api_handler))
        .route("/api/role", post(store_role_api_handler))
        .route("/api/role/:role_id", get(fetch_role_api_handler))
        .route(
            "/api/put-role-identifier/:role_id",
            put(put_role_identifier_api_handler),
        )
        .route("/api/role/:role_id", put(update_role_api_handler))
        .route("/api/admin-user", get(admin_user_table_api_handler))
        .route("/api/admin-user", post(store_admin_user_api_handler))
        .route("/api/change-password", post(change_password_api_handler))
        .route(
            "/api/admin-user/:admin_user_id",
            put(update_admin_user_api_handler),
        )
        .route("/api/logged-in-user", get(logged_in_user_api_handler))
        .route(
            "/api/admin-user/:admin_user_id",
            get(fetch_admin_user_api_handler),
        )
        .route("/api/collection", get(collection_table_api_handler))
        .route("/api/collection", post(store_collection_api_handler))
        .route(
            "/api/collection/:collection_id",
            get(fetch_collection_api_handler),
        )
        .route(
            "/api/collection/:collection_id",
            put(update_collection_api_handler),
        )
        .route(
            "/api/put-collection-identifier/:collection_id",
            put(put_collection_identifier_api_handler),
        )
        .route("/api/collection-all", get(collection_all_api_handler))
        .route("/api/model", get(model_table_api_handler))
        .route("/api/model", post(store_model_api_handler))
        .route("/api/model/:model_id", put(update_model_api_handler))
        .route("/api/model/:model_id", get(fetch_model_api_handler))
        .route(
            "/api/put-model-identifier/:model_id",
            put(put_model_identifier_api_handler),
        )
        .route("/api/page", get(page_table_api_handler))
        .route("/api/page", post(store_page_api_handler))
        .route("/api/page/:page_id", put(update_page_api_handler))
        .route("/api/page/:page_id", get(fetch_page_api_handler))
        .route("/api/page/:page_id", delete(delete_page_handler))
        .route(
            "/api/put-page-identifier/:page_id",
            put(put_page_identifier_api_handler),
        )
        .route("/api/component-all", get(component_all_api_handler))
        .route("/api/openapi.json", get(openapi_api_handler))
        .route("/api/setting", get(setting_all_api_handler))
        .route("/api/setting", post(update_setting_all_api_handler))
        .route(
            "/api/install-demo-data",
            post(install_demo_data_api_handler),
        )
        .route("/api/delete-demo-data", post(delete_demo_data_api_handler))
        // .route("/test", get(test_handler))
        .route(
            "/graphql",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                graphql_api_handler,
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .route("/api/health-check", get(health_check_api_handler))
        .route("/api/setup", post(post_setup_avored_handler))
        .route("/api/login", post(admin_user_login_api_handler))
        .route("/api/testing", post(testing_api_handler))
        .route(
            "/api/reset-password",
            post(admin_user_reset_password_api_handler),
        )
        .route(
            "/api/forgot-password",
            post(admin_user_forgot_password_api_handler),
        )
        .with_state(state)
        .layer(cors)
        .layer(Extension(Arc::new(schema)))
}

fn get_cors_urls(state: Arc<AvoRedState>) -> CorsLayer {
    let mut origins: Vec<HeaderValue> = vec![];
    for origin in &state.config.cors_allowed_app_url {
        origins.push(HeaderValue::from_str(origin).unwrap());
    }

    CorsLayer::new()
        .allow_origin(origins)
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
}

#[cfg(test)]
pub mod tests {
    use crate::api::handlers::admin_user::admin_user_login_api_handler::LoginResponseData;
    use crate::api::handlers::setup::post_setup_avored_handler::SetupViewModel;
    use crate::avored_state::AvoRedState;
    use crate::error::Result;
    use crate::models::admin_user_model::AdminUserModel;
    use crate::models::token_claim_model::TokenClaims;
    use axum::body::Body;
    use axum::http::{self, header, Request, StatusCode};
    use axum::Router;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::env;
    use std::sync::Arc;
    use tower::ServiceExt;

    use super::rest_api_routes;

    pub fn send_get_request(uri: &str, token: String) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json")
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }

    pub fn send_post_request(uri: &str, body: Body) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .method("POST")
            .body(body)
            .unwrap()
    }

    pub async fn setup_avored_db(app: Router) {
        let payload = Body::from(
            r#"{
                    "email": "admin@admin.com",
                    "password": "admin123"
                }"#,
        );
        let expected_response = SetupViewModel { status: true };
        let response = app
            .oneshot(send_post_request("/api/setup", payload))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();

        let body_str = String::from_utf8(body.to_vec()).expect("Failed to convert body to string");

        let body: SetupViewModel = serde_json::from_str(&body_str).expect("Failed to parse JSON");
        assert_eq!(body, expected_response);
    }

    pub async fn get_login_response(app: Router) -> Result<LoginResponseData> {
        let payload = Body::from(
            r#"{
                    "email": "admin@admin.com",
                    "password": "admin123"
                }"#,
        );

        let response = app
            .oneshot(send_post_request("/api/login", payload))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();

        let body_str: String =
            String::from_utf8(body.to_vec()).expect("Failed to convert body to string");

        let body: LoginResponseData =
            serde_json::from_str(&body_str).expect("Failed to parse JSON");

        Ok(body)
    }

    pub fn get_auth_token(state: Arc<AvoRedState>) -> Result<String> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let admin_user_model = AdminUserModel::default();

        let claims: TokenClaims = TokenClaims {
            sub: admin_user_model.clone().id,
            name: admin_user_model.clone().full_name,
            email: admin_user_model.clone().email,
            admin_user_model: admin_user_model.clone(),
            exp,
            iat,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.config.jwt_secret_key.as_ref()),
        )
        .unwrap())
    }

    pub async fn get_axum_app() -> Result<(Router, Arc<AvoRedState>)> {
        env::set_var("AVORED_DATABASE_NAMESPACE", "public_test");
        env::set_var("AVORED_DATABASE_NAME", "avored_cms_test");
        env::set_var("AVORED_DATABASE_FOLDER_NAME", "memory");

        env::set_var(
            "AVORED_PASSWORD_SALT",
            "UnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTest",
        );

        env::set_var(
            "AVORED_JWT_SECRET",
            "UnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTest",
        );
        env::set_var("AVORED_JWT_EXPIRED_IN", "60");
        env::set_var("AVORED_JWT_MAXAGE", "60");

        env::set_var("AVORED_REACT_FRONTEND_APP_URL", "http://localhost:5173");
        env::set_var("AVORED_REACT_ADMIN_APP_URL", "http://localhost:3000");
        env::set_var("AVORED_BACK_END_APP_URL", "http://localhost:8080");

        env::set_var("SMTP_HOST", "http://smtp.url");
        env::set_var("SMTP_USERNAME", "smtp_username");
        env::set_var("SMTP_PASSWORD", "smtp_password");
        env::set_var("SMTP_PORT", "587");

        let state = Arc::new(AvoRedState::new().await?);

        let app = rest_api_routes(state.clone());

        Ok((app, state))
    }
}
