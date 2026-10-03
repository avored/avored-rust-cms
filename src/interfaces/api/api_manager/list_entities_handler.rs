use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::api_manager_dto::{
        EntityRecordResponse, ListEntitiesQuery, ListEntitiesResponse,
    },
    core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse},
    error::{Error, Result},
};

/// Reserved query-parameter keys that must never be treated as attribute filters.
const RESERVED_KEYS: &[&str] = &["entity_type", "page", "limit"];

/// `GET /api/v1/entities?entity_type={type}&page={p}&limit={l}[&attr=value…]`
///
/// Returns a paginated list of dynamic records belonging to the supplied entity
/// type.  Any query parameter other than the three reserved ones is interpreted
/// as an attribute equality filter.
pub async fn list_entities_handler(
    State(state): State<AppState>,
    Query(mut query): Query<ListEntitiesQuery>,
) -> Result<Json<ListEntitiesResponse>> {
    // --- Input validation ---

    let entity_type = query.entity_type.trim().to_string();
    if entity_type.is_empty() {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "entity_type".to_string(),
                message: "entity_type is required".to_string(),
            }],
        }));
    }

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    if page == 0 {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "page".to_string(),
                message: "page must be greater than 0".to_string(),
            }],
        }));
    }

    if limit == 0 || limit > 100 {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "limit".to_string(),
                message: "limit must be between 1 and 100".to_string(),
            }],
        }));
    }

    // Strip reserved keys from the flat filter map so attribute filters are clean.
    for key in RESERVED_KEYS {
        query.filters.remove(*key);
    }

    // --- Delegate to use case ---

    let (records, total_items, total_pages, resolved_limit) = state
        .entity_use_case
        .list_entities(&entity_type, page, limit, query.filters)
        .await?;

    // --- Shape response ---

    let data: Vec<EntityRecordResponse> = records
        .into_iter()
        .map(|record| shape_record(record, &entity_type))
        .collect();

    let response = ListEntitiesResponse {
        data,
        page,
        limit: resolved_limit,
        total_items,
        total_pages,
    };

    Ok(Json(response))
}

/// Convert a raw SurrealDB JSON record into the flat-attributes response DTO.
///
/// `id` and `entity_type` are promoted to the envelope; every other field goes
/// into `attributes`.  Internal bookkeeping fields (`created_at`, `created_by`,
/// `updated_at`, `updated_by`, `deleted_at`, `deleted_by`) are excluded from
/// `attributes` to keep the surface clean.
fn shape_record(record: serde_json::Value, entity_type: &str) -> EntityRecordResponse {
    let obj = match record {
        serde_json::Value::Object(m) => m,
        other => {
            // Fallback: wrap scalar in an empty response.
            return EntityRecordResponse {
                id: other.to_string(),
                entity_type: entity_type.to_string(),
                attributes: serde_json::Map::new(),
            };
        }
    };

    let id = obj
        .get("id")
        .and_then(serde_json::Value::as_str)
        .map(extract_record_id_part)
        .unwrap_or_default();

    const EXCLUDED: &[&str] = &[
        "id",
        "created_at",
        "created_by",
        "updated_at",
        "updated_by",
        "deleted_at",
        "deleted_by",
    ];

    let mut attributes = serde_json::Map::new();
    for (key, value) in obj {
        if !EXCLUDED.contains(&key.as_str()) {
            attributes.insert(key, value);
        }
    }

    EntityRecordResponse {
        id,
        entity_type: entity_type.to_string(),
        attributes,
    }
}

/// Extract the record part from a SurrealDB `"table:key"` id string.
///
/// e.g. `"blog_posts:abc123"` → `"abc123"`.
fn extract_record_id_part(full_id: &str) -> String {
    full_id
        .split_once(':')
        .map(|(_, key)| key.to_string())
        .unwrap_or_else(|| full_id.to_string())
}
