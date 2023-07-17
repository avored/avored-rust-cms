use chrono::NaiveDateTime;
use entity::roles;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

impl RoleResponse {
    pub fn transform_into_response(role_model: roles::Model) -> RoleResponse {
        RoleResponse {
            id: role_model.id,
            name: role_model.name,
            description: role_model.description,
            created_at: role_model.created_at,
            updated_at: role_model.updated_at,
            created_by: role_model.created_by,
            updated_by: role_model.updated_by,
        }
    }
}
