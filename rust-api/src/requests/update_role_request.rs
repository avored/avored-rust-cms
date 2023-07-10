use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}
