use axum::{response::IntoResponse, Json, Extension};
use serde_json::json;

use crate::{repositories::admin_user_repository::AdminUser};

pub async fn home_handler (
    Extension(current_user): Extension<AdminUser>,
) -> impl IntoResponse { 
    println!("extension: {:?}", current_user);
    Json(json!(current_user)).into_response()
}
