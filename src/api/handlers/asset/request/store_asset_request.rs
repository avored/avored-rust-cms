use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StoreAssetRequest {
    pub parent_id: Option<String>
}