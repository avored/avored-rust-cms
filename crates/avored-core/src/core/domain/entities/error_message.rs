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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ApiErrorMessageResponse {
    pub status: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
