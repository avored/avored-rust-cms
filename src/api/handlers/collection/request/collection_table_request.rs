use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CollectionTableRequest {
    pub page: Option<i64>,
    pub order: Option<String>,
}
