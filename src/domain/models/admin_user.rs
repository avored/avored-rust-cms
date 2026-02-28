use serde::{Deserialize, Serialize};



// id


#[derive(Debug, Serialize, Deserialize)]
pub struct StorableAdminUser {
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub email: String,
    pub password_hash: String,
    pub logged_in_user: String,
}
