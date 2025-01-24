use crate::error::Result;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn testing_api_handler(
    Json(payload): Json<TestingRequest>,
) -> Result<Json<ResponseData>> {
    println!("->> {:<12} - testing_api_handler", "HANDLER");

    println!("->> {:<12} - {:?}", "Payload", payload.name);

    let response = ResponseData {
        status: true,
        data: String::from("ok"),
    };

    Ok(Json(response))
}

#[derive(Deserialize, Debug)]
pub struct TestingRequest {
    name: String,
    // field_data: TestFieldData
}

//
// #[derive(Deserialize, Debug)]
// pub enum TestFieldData {
//     SelectFieldData {
//         select_field_option: TestingFieldOption
//     },
//     None
// }
//
//
// #[derive(Deserialize, Debug)]
// pub struct TestingFieldOption {
//     pub label: String,
//     pub value: String
// }

#[derive(Serialize)]
pub struct ResponseData {
    status: bool,
    data: String,
}
