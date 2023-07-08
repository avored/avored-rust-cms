use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RolesRequest {
    pub current_page: u64,
    pub per_page: u64,
}
