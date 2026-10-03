use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/* #region Query DTO */

/// Query parameters for `GET /api/v1/entities`.
///
/// Required: `entity_type` — the identifier of an entity type (e.g. `"blog_posts"`).
/// Optional: `page` (1-based, default 1), `limit` (default 20, max 100).
/// Any additional query key that does not match a reserved name is treated as an
/// attribute equality filter: `?title=Hello&views=10`.
///
/// Reserved names (not treated as filters): `entity_type`, `page`, `limit`.
#[derive(Debug, Deserialize)]
pub struct ListEntitiesQuery {
    pub entity_type: String,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    /// Catch-all for attribute filters.  Axum's `Query` extractor flattens
    /// extra keys into a `HashMap` via `#[serde(flatten)]`.
    #[serde(flatten)]
    pub filters: HashMap<String, String>,
}

/* #endregion */

/* #region Response DTOs */

/// A single dynamic record returned by the list endpoint.
///
/// `id` and `entity_type` are promoted to the top level; all other stored
/// fields are folded into the flat `attributes` object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRecordResponse {
    pub id: String,
    pub entity_type: String,
    pub attributes: serde_json::Map<String, serde_json::Value>,
}

/// Paginated envelope returned by `GET /api/v1/entities`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEntitiesResponse {
    pub data: Vec<EntityRecordResponse>,
    pub page: u64,
    pub limit: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

/* #endregion */

/* #region Use-case command */

/// Internal command passed from the handler to the use case.
#[derive(Debug, Clone, Default)]
pub struct ListEntitiesCommand {
    /// Resolved entity-type identifier (i.e. the SurrealDB table name).
    pub table_name: String,
    /// Human-readable entity type identifier as supplied by the caller.
    pub entity_type: String,
    pub page: u64,
    pub limit: u64,
    /// Attribute equality filters: key → value string.
    pub filters: HashMap<String, String>,
}

/* #endregion */
