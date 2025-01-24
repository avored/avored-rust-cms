use crate::models::role_model::RoleModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PutRoleIdentifierResponse {
    pub role: RoleModel,
}
