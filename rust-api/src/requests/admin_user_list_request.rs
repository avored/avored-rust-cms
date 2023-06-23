use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AdminUsersRequest {
    pub current_page: i64,
    pub per_page: i64
}
