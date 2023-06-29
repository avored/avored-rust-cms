use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateAdminUserRequest {
    pub name: String,
    pub email: String,
    pub password: String
}
