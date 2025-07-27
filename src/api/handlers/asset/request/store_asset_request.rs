use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// This struct is used to handle requests for storing assets.
pub struct StoreAssetRequest {
    /// Optional parent asset identifier.
    pub parent_id: Option<String>,
}
