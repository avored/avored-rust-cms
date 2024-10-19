use std::sync::Arc;

use crate::models::role_model::RoleModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;

pub async fn fetch_role_api_handler (
    AxumPath(role_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_role_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_role"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let role_model = state
        .role_service
        .find_by_id(&state.db, role_id)
        .await?;
    let response = FetchPageResponse {
        status: true,
        role_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub role_model: RoleModel
}


#[cfg(test)]
mod tests {
    use tower::ServiceExt;
    use crate::api::rest_api_routes::tests::{get_axum_app, get_login_response, send_get_request, setup_avored_db};
    use crate::models::role_model::RolePagination;

    #[tokio::test]
    async fn test_fetch_role_api_handler() -> crate::error::Result<()>
    {
        let (app, _state) = get_axum_app().await?;
        setup_avored_db(app.clone()).await;
        let logged_in_user_response = get_login_response(app.clone()).await?;

        let token = logged_in_user_response.data;
        let response = app.oneshot(send_get_request("/api/role", token)).await.unwrap();
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();

        let body_str: String = String::from_utf8(body.to_vec())
            .expect("Failed to convert body to string");

        let body: RolePagination = serde_json::from_str(&body_str)
            .expect("Failed to parse JSON");

        assert_eq!(body.data.len() , 1);

        Ok(())
    }
}