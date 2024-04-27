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