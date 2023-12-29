use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateAdminUserRequest {
    pub full_name: String,
    pub is_super_admin: bool,
}
