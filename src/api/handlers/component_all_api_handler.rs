use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::component_model::ComponentModel;

pub async fn component_all_api_handler(
    state: State<Arc<AvoRedState>>
) -> Result<Json<Vec<ComponentModel>>> {
    println!("->> {:<12} - component_all_api_handler", "HANDLER");

    Ok(Json(state.component_service.all(&state.db).await?))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::StatusCode;
    use serde_json::{json, Value};
    use tower::ServiceExt;
    use crate::api::handlers::setup::post_setup_avored_handler::SetupViewModel;
    use crate::api::rest_api_routes::tests::{get_axum_app, send_post_request};
    use crate::error::Result;

    #[tokio::test]
    async fn test_component_all_api_handler() -> Result<()>
    {
        let app = get_axum_app().await.unwrap();
        let body = Body::from(
        r#"{
                    "email": "admin@admin.com",
                    "password": "admin123"

                }"#,
        );

        //@todo check for component-all-api with success response
        let response = app.oneshot(send_post_request("/api/setup", body)).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let dummy_res = SetupViewModel {
            status: true
        };
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(&dummy_res));

        Ok(())
    }
}