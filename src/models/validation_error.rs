use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorMessage {
    pub key: String,
    pub message: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: bool,
    pub errors: Vec<ErrorMessage>
}