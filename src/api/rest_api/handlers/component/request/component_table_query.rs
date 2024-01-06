use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ComponentTableQuery {
    pub page: Option<i64>,
}
