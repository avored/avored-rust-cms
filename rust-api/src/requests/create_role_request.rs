use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>
}
