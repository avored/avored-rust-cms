use std::sync::Arc;

use axum::{routing::get, Router, middleware, Extension};
use axum::routing::{MethodFilter, on, post, put};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::header::HeaderValue;
use juniper::{EmptyMutation, EmptySubscription};
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use tower_http::cors::CorsLayer;
use crate::api::handlers::{
    page::page_table_api_handler::page_table_api_handler,
    admin_user::admin_user_login_api_handler::admin_user_login_api_handler,
    component_all_api_handler::component_all_api_handler,
    health_check_api_handler::health_check_api_handler,
    page::store_page_api_handler::store_page_api_handler,
    page::update_page_api_handler::update_page_api_handler,
    page::fetch_page_api_handler::fetch_page_api_handler,
    admin_user::admin_user_table_api_handler::admin_user_table_api_handler,
    admin_user::store_admin_user_api_handler::store_admin_user_api_handler,
    admin_user::update_admin_user_api_handler::update_admin_user_api_handler,
    admin_user::fetch_admin_user_api_handler::fetch_admin_user_api_handler,
    role::role_table_api_handler::role_table_api_handler,
    role::fetch_role_api_handler::fetch_role_api_handler,
    role::store_role_api_handler::store_role_api_handler,
    role::update_role_api_handler::update_role_api_handler,
    role::role_option_api_handler::role_option_api_handler,
    asset::asset_table_api_handler::asset_table_api_handler,
    asset::store_asset_api_handler::store_asset_api_handler,
    component::component_table_api_handler::component_table_api_handler,
    component::store_component_api_handler::store_component_api_handler,
    component::fetch_component_api_handler::fetch_component_api_handler,
    component::update_component_api_handler::update_component_api_handler,
    setup::post_setup_avored_handler::post_setup_avored_handler,
    admin_user::logged_in_user_api_handler::logged_in_user_api_handler,
    admin_user::admin_user_forgot_password_api_handler::admin_user_forgot_password_api_handler,
    admin_user::admin_user_reset_password_api_handler::admin_user_reset_password_api_handler,
    openapi_api_handler::openapi_api_handler,
    setting::setting_all_api_handler::setting_all_api_handler,
    setting::update_setting_all_api_handler::update_setting_all_api_handler,
    admin_user::change_password_api_handler::change_password_api_handler,
    role::put_role_identifier_api_handler::put_role_identifier_api_handler,
    page::put_page_identifier_api_handler::put_page_identifier_api_handler
};
use crate::api::handlers::component::put_component_identifier_api_handler::put_component_identifier_api_handler;
use crate::api::handlers::graphql::graphql_api_handler::graphql_api_handler;
use crate::providers::avored_graphql_provider::{Context, AvoRedGraphqlSchema};
use crate::query::AvoRedQuery;

pub fn rest_api_routes(state: Arc<AvoRedState>, ctx: Arc<Context>) -> Router {

    let front_end_app_url = &state.config.front_end_app_url;

    println!("FRONT END CORS ALLOWED URL: {front_end_app_url}");
    let cors_layer = CorsLayer::new()
        .allow_origin(front_end_app_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);

    let schema = AvoRedGraphqlSchema::new(
        AvoRedQuery,
        EmptyMutation::new(),
        EmptySubscription::new()
    );

    Router::new()
        .route("/api/component", get(component_table_api_handler))
        .route("/api/component", post(store_component_api_handler))
        .route("/api/component/:component_id", get(fetch_component_api_handler))
        .route("/api/component/:component_id", put(update_component_api_handler))
        .route("/api/put-component-identifier/:page_id", put(put_component_identifier_api_handler))
        .route("/api/asset", get(asset_table_api_handler))
        .route("/api/asset", post(store_asset_api_handler))
        .route("/api/role-options", get(role_option_api_handler))
        .route("/api/role", get(role_table_api_handler))
        .route("/api/role", post(store_role_api_handler))
        .route("/api/role/:role_id", get(fetch_role_api_handler))
        .route("/api/put-role-identifier/:role_id", put(put_role_identifier_api_handler))
        .route("/api/role/:role_id", put(update_role_api_handler))
        .route("/api/admin-user", get(admin_user_table_api_handler))
        .route("/api/admin-user", post(store_admin_user_api_handler))
        .route("/api/change-password", post(change_password_api_handler))
        .route("/api/admin-user/:admin_user_id", put(update_admin_user_api_handler))
        .route("/api/logged-in-user", get(logged_in_user_api_handler))
        .route("/api/admin-user/:admin_user_id", get(fetch_admin_user_api_handler))
        .route("/api/page", get(page_table_api_handler))
        .route("/api/page", post(store_page_api_handler))
        .route("/api/page/:page_id", put(update_page_api_handler))
        .route("/api/page/:page_id", get(fetch_page_api_handler))
        .route("/api/put-page-identifier/:page_id", put(put_page_identifier_api_handler))
        .route("/api/component-all", get(component_all_api_handler))
        .route("/api/openapi.json", get(openapi_api_handler))
        .route("/api/setting", get(setting_all_api_handler))
        .route("/api/setting", post(update_setting_all_api_handler))
        .route("/graphql", on(
            MethodFilter::GET.or(MethodFilter::POST),
            graphql_api_handler,
        ),)
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .route("/api/health-check", get(health_check_api_handler))
        .route("/api/setup", post(post_setup_avored_handler))
        .route("/api/login", post(admin_user_login_api_handler))
        .route("/api/reset-password", post(admin_user_reset_password_api_handler))
        .route("/api/forgot-password", post(admin_user_forgot_password_api_handler))
        .with_state(state)
        .layer(cors_layer)
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(ctx))
}



#[cfg(test)]
pub mod tests {
    use std::env;
    use std::sync::Arc;
    use axum::{body::Body, http::Request, Router};
    use crate::api::rest_api_routes::rest_api_routes;
    use crate::avored_state::AvoRedState;
    use crate::error::Result;

    pub fn send_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }

    pub fn send_post_request(uri: &str, body: Body) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .header("content-type", "application/json")
            .method("POST")
            .body(body).unwrap()
    }

    pub async fn get_axum_app() -> Result<Router>
    {
        env::set_var("AVORED_DATABASE_NAMESPACE", "public_test");
        env::set_var("AVORED_DATABASE_NAME", "avored_cms_test");
        env::set_var("AVORED_DATABASE_FOLDER_NAME", "memory");

        let state = Arc::new(AvoRedState::new().await?);

        let app = rest_api_routes(state.clone());

       Ok(app)
    }
}