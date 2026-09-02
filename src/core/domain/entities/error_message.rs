use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ErrorMessageResponse {
    pub key: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ErrorResponse {
    pub status: bool,
    pub errors: Vec<ErrorMessageResponse>,
}


