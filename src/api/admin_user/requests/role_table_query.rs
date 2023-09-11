use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoleTableQuery {
    pub page: Option<i64>,
}
