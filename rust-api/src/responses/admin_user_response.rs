use chrono::NaiveDateTime;
use serde_derive::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
