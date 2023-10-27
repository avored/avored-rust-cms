use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AssetTableQuery {
    pub page: Option<i64>,
}
