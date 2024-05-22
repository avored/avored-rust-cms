use axum::Json;
use utoipa::OpenApi;
use utoipa::openapi::Server;
use crate::error::Result;
use crate::api::handlers::admin_user::admin_user_login_api_handler::LoginResponseData;
use crate::models::admin_user_model::AdminUserModel;
use crate::models::role_model::RoleModel;
use crate::api::handlers::admin_user::request::authenticate_admin_user_request::AuthenticateAdminUserRequest;

pub async fn openapi_api_handler() -> Result<Json<utoipa::openapi::OpenApi>> {
    println!("->> {:<12} - openapi_api_handler", "HANDLER");
    let mut json = ApiDoc::openapi();
    json.info.title = String::from("AvoRed Cms");
    let server = Server::new("http://localhost:8080");

    json.servers = Some(vec![server]);

    Ok(Json(json))
}


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::admin_user::admin_user_login_api_handler::admin_user_login_api_handler
    ),
    components(
        schemas(
            LoginResponseData,
            AdminUserModel,
            RoleModel,
            AuthenticateAdminUserRequest
        )
    )
)]
struct ApiDoc;
