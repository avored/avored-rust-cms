use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PageTableRequest {
    pub page: Option<i64>,
    pub order: Option<String>,
}
