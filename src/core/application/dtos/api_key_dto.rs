use serde::{Deserialize, Serialize};

use crate::core::domain::entities::api_key::ApiKeyPermission;

/// POST /api/api-keys  request body
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<ApiKeyPermission>,
}

/// PUT /api/api-keys/{id}  request body
#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<ApiKeyPermission>,
}

/// Response envelope for a single key
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<ApiKeyPermission>,
    pub created_at: String,
    pub updated_at: String,
    /// Only populated in the POST response; None on all subsequent reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_secret: Option<String>,
}

/// Paginated list response
#[derive(Debug, Serialize)]
pub struct ListApiKeysResponse {
    pub data: Vec<ApiKeyResponse>,
    pub page: u64,
    pub limit: u64,
    pub total_items: u64,
    pub total_pages: u64,
}
