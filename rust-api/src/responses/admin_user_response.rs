use chrono::NaiveDateTime;
use serde_derive::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
