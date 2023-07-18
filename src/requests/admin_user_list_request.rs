use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AdminUsersRequest {
    pub current_page: Option<u64>,
    pub per_page: Option<u64>,
}
