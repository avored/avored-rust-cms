use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ErrorMessageResponse {
    pub key: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ApiErrorResponse {
    pub status: bool,
    pub errors: Vec<ErrorMessageResponse>,
}

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ApiErrorMessageResponse {
    pub status: bool,
    pub message: String,
}
