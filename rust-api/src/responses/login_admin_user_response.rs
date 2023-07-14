use serde::Serialize;

#[derive(Serialize)]
pub struct LoginAdminUserResponse {
    pub success: bool,
    pub message: String,
}
