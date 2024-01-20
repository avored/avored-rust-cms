use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StoreAdminUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub confirmation_password: String,
    pub is_super_admin: bool,
    pub role_ids: Vec<String>
}