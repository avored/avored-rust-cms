use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateAdminUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct StoreAdminUserRequest {
    pub name: String,
    pub email: String,
    pub roles: String,
}
