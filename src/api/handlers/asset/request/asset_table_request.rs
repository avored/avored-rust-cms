use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AssetTableRequest {
    pub page: Option<i64>,
    // pub order: Option<String>,
    pub parent_id: Option<String>,
}
