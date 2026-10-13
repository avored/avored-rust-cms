use serde::{Deserialize, Serialize};

/// Represents a stored API Key record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyModel {
    pub id: String,
    pub name: String,
    /// First 8 chars of the plain secret (safe to display).
    pub key_prefix: String,
    /// SHA-256 hash of the plain secret (stored; never returned after creation).
    pub key_hash: String,
    /// JSON array of permitted v1 endpoints + HTTP methods.
    pub permissions: Vec<ApiKeyPermission>,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: String,
    pub updated_by: String,
}

/// A single permission entry: endpoint pattern + HTTP verb.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyPermission {
    pub endpoint: String, // e.g. "/api/v1/entities"
    pub method: String,   // e.g. "GET" | "POST" | "PATCH" | "DELETE"
}

/// Used when creating a new API key.
pub struct StorableApiKey {
    pub name: String,
    pub key_prefix: String,
    pub key_hash: String,
    pub permissions: Vec<ApiKeyPermission>,
    pub created_by: String,
}

/// Used when updating permissions/name on an existing key.
pub struct UpdatableApiKey {
    pub name: String,
    pub permissions: Vec<ApiKeyPermission>,
    pub updated_by: String,
}
