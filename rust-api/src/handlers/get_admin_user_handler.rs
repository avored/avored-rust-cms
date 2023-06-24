use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::{State, Path}};
use uuid::Uuid;

use crate::routes::AppState;

pub async fn get_admin_user_handler(
        app_state : State<Arc<AppState>>,
        Path(admin_user_id): Path<String>
    ) -> impl IntoResponse {

        let admin_user_uuid: Uuid = Uuid::parse_str(&admin_user_id).unwrap();
        let admin_user = app_state.admin_user_repository.find_by_uuid(admin_user_uuid);

        Json(admin_user).into_response()
}

