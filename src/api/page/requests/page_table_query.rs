use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PageTableQuery {
    pub page: Option<i64>,
}
