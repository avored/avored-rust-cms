use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ModelTableRequest {
    pub page: Option<i64>,
    pub order: Option<String>,
}
