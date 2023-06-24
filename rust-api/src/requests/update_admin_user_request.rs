use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateAdminUsersRequest {
    pub email: String
}
