use chrono::NaiveDateTime;
use entity::roles;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RoleResponseModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize)]
pub struct RolessPaginateResponse {
    pub results: Vec<RoleResponseModel>,
}

impl RolessPaginateResponse {
    pub fn transform_into_response(roles_models: Vec<roles::Model>) -> RolessPaginateResponse {
        let mut roles_vec = Vec::new();

        for role_model in roles_models {
            let role_response = RoleResponseModel {
                id: role_model.id,
                name: role_model.name,
                description: role_model.description,
                created_at: role_model.created_at,
                updated_at: role_model.updated_at,
                created_by: role_model.created_by,
                updated_by: role_model.updated_by,
            };

            roles_vec.push(role_response)
        }

        RolessPaginateResponse { results: roles_vec }
    }
}
