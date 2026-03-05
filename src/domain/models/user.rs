use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}
