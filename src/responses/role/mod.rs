use serde::Serialize;
use crate::models::role_model::RoleModel;

#[derive(Serialize)]
pub struct PutRoleIdentifierResponse {
    pub role: RoleModel
}