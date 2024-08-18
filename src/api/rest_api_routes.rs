use std::sync::Arc;
use axum::{middleware, routing::get, Extension, Router};
use axum::routing::{delete, on, post, put, MethodFilter};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use juniper::{EmptyMutation, EmptySubscription};
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use tower_http::cors::CorsLayer;
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
    asset::store_asset_api_handler::store_asset_api_handler,
    cms::fetch_page_cms_api_handler::fetch_page_cms_api_handler,
    component::component_table_api_handler::component_table_api_handler,
    component::fetch_component_api_handler::fetch_component_api_handler,
    component::store_component_api_handler::store_component_api_handler,
    component::update_component_api_handler::update_component_api_handler,
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
    misc::health_check_api_handler::health_check_api_handler,
    misc::openapi_api_handler::openapi_api_handler,
    component::component_all_api_handler::component_all_api_handler,
    component::put_component_identifier_api_handler::put_component_identifier_api_handler,
    model::fetch_model_api_handler::fetch_model_api_handler,
    model::model_table_api_handler::model_table_api_handler,
    model::put_model_identifier_api_handler::put_model_identifier_api_handler,
    model::store_model_api_handler::store_model_api_handler,
    model::update_model_api_handler::update_model_api_handler,
    asset::create_folder_api_handler::create_folder_api_handler,
    asset::delete_folder_api_handler::delete_folder_api_handler,
    asset::rename_asset_api_handler::rename_asset_api_handler,
};

use crate::api::handlers::graphql::graphql_api_handler::graphql_api_handler;
use crate::providers::avored_graphql_provider::AvoRedGraphqlSchema;
use crate::query::AvoRedQuery;

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {

    let react_admin_url = &state.config.react_admin_app_url;
    let react_front_url = &state.config.react_frontend_app_url;

    let origins = [
        react_admin_url.parse().unwrap(),
        react_front_url.parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
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
        .route("/api/rename-asset/:asset_id", post(rename_asset_api_handler))
        .route("/api/create-folder", post(create_folder_api_handler))
        .route("/api/delete-folder/:asset_id", delete(delete_folder_api_handler))
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
        .route("/api/model", get(model_table_api_handler))
        .route("/api/model", post(store_model_api_handler))
        .route("/api/model/:model_id", put(update_model_api_handler))
        .route("/api/model/:model_id", get(fetch_model_api_handler))
        .route("/api/put-model-identifier/:model_id", put(put_model_identifier_api_handler))
        .route("/api/page", get(page_table_api_handler))
        .route("/api/page", post(store_page_api_handler))
        .route("/api/page/:page_id", put(update_page_api_handler))
        .route("/api/page/:page_id", get(fetch_page_api_handler))
        .route("/api/put-page-identifier/:page_id", put(put_page_identifier_api_handler))
        .route("/api/component-all", get(component_all_api_handler))
        .route("/api/openapi.json", get(openapi_api_handler))
        .route("/api/setting", get(setting_all_api_handler))
        .route("/api/setting", post(update_setting_all_api_handler))
        // .route("/test", get(test_handler))
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
        .route("/cms/page/:page_id", get(fetch_page_cms_api_handler))
        .with_state(state)
        .layer(cors)
        .layer(Extension(Arc::new(schema)))
}


// pub async fn test_handler (
//     state: State<Arc<AvoRedState>>,
// ) -> Result<impl IntoResponse> {
//     println!("Test call");
//
//     let sql = "
//             CREATE persons content { data_type: 'TEXT', data: 'test_data' };
//     ";

    // let sql = "
    //         CREATE persons content { data_type: 'INT', data: 1 };
    // ";

    // let (ds, ses) = &state.db;
    //
    // let responses = ds.execute(sql, ses, None).await?;
    //
    // let result_object_option = crate::repositories::into_iter_objects(responses)?.next();
    // let result_object = match result_object_option {
    //     Some(object) => object,
    //     None => Err(Error::Generic("no record found".to_string())),
    // };
    // let admin_user_model: Result<PersonModel> = result_object?.try_into();
    // println!("Response: {:?}", admin_user_model);

    // Ok(Json(admin_user_model))
// }

//
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct PersonModel {
//     pub id: String,
//     pub data_type: String,
//     pub data: AvoRedDataType
// }
//
// #[derive(Serialize, Debug, Deserialize, Clone)]
// pub enum  AvoRedDataType {
//     AvoredString(String),
//     Int64(i64)
// }
//
// impl Default for AvoRedDataType {
//     fn default() -> AvoRedDataType {
//         AvoRedDataType::AvoredString("".to_string())
//     }
// }

// impl TryFrom<Object> for PersonModel {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<PersonModel> {
//         let id = val.get("id").get_id()?;
//         let data_type = val.get("data_type").get_string()?;
//
//         let data = match data_type.as_str() {
//             "TEXT" => {
//                 let string_val = val.get("data").get_string()?;
//
//                 let a: AvoRedDataType = AvoRedDataType::AvoredString(string_val);
//                 a
//             },
//             "INT" => {
//                 let val = val.get("data").get_int()?;
//                 let a: AvoRedDataType = AvoRedDataType::Int64(val);
//
//                 a
//             },
//             _ => AvoRedDataType::default()
//         };
//
//         // let data = val.get("data").get_string()?;
//         Ok(PersonModel {
//             id,
//             data_type,
//             data
//         })
//     }
// }


#[cfg(test)]
pub mod tests {
    use std::env;
    use std::sync::Arc;
    use axum::body::Body;
    use axum::http::{self, header, Request};
    use axum::Router;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use crate::avored_state::AvoRedState;
    use crate::error::Result;
    use crate::models::admin_user_model::AdminUserModel;
    use crate::models::token_claim_model::TokenClaims;

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

    pub fn get_auth_token(state: Arc<AvoRedState>) -> Result<String> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let admin_user_model = AdminUserModel::default();

        let claims: TokenClaims = TokenClaims {
            sub: admin_user_model.clone().id,
            name: admin_user_model.clone().full_name,
            email:admin_user_model.clone().email,
            admin_user_model: admin_user_model.clone(),
            exp,
            iat,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.config.jwt_secret_key.as_ref()),
        ).unwrap())
    }

    pub async fn get_axum_app() -> Result<(Router, Arc<AvoRedState>)>
    {
        env::set_var("AVORED_DATABASE_NAMESPACE", "public_test");
        env::set_var("AVORED_DATABASE_NAME", "avored_cms_test");
        env::set_var("AVORED_DATABASE_FOLDER_NAME", "memory");


        env::set_var("AVORED_PASSWORD_SALT", "UnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTest");

        env::set_var("AVORED_JWT_SECRET", "UnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTestUnitTest");
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