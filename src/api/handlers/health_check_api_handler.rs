use axum::Json;
use serde::Serialize;
use crate::error::Result;

pub async fn health_check_api_handler() -> Result<Json<ResponseData>> {
    println!("->> {:<12} - health_check_api_handler", "HANDLER");
    let response = ResponseData {
        status: true,
        data: String::from("ok")
    };

    Ok(Json(response))
}

#[derive(Serialize)]
pub struct ResponseData {
    status: bool,
    data: String
}




#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::{json, Value};
    use tower::ServiceExt;
    use crate::api::handlers::health_check_api_handler::ResponseData;
    use crate::api::rest_api_routes::tests::{get_axum_app, send_get_request};
    use crate::error::Result;

    #[tokio::test]
    async fn test_health_check_api_handler() -> Result<()>
    {
        let app = get_axum_app().await.unwrap();

        let response = app.oneshot(send_get_request("/api/health-check")).await.unwrap();

        let dummy_res = ResponseData {
            status: true,
            data: String::from("ok")
        };
        assert_eq!(response.status(), StatusCode::OK);

        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(&dummy_res));
        Ok(())
    }
}