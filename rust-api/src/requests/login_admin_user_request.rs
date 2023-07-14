use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginAdminUserRequest {
    pub email: String,
    pub password: String,
}
