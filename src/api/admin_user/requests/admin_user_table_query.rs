use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AdminUserTableQuery {
    pub page: Option<i64>,
}
