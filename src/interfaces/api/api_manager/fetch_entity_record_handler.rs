use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::api_manager_dto::{EntityRecordResponse, FetchEntityRecordQuery},
    core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse},
    error::{Error, Result},
};

/// `GET /api/v1/entities/{id}`
///
/// Retrieves a single dynamic record by ID and returns its flat attributes.
///
/// # Query parameters
/// - `entity_type` (optional if `id` is formatted as `"entity_type:record_key"`).
///
/// # Status codes
/// - `200 OK`           — successful retrieval
/// - `400 Bad Request`  — missing entity_type context or malformed ID
/// - `404 Not Found`    — unknown entity type or record not found
/// - `500`              — persistence failure
pub async fn fetch_entity_record_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<FetchEntityRecordQuery>,
) -> Result<(StatusCode, Json<EntityRecordResponse>)> {
    // Resolve entity_type from query param or parse from `{table}:{key}` id
    let (entity_type, record_id) = if let Some(et) = query.entity_type {
        let clean_et = et.trim().to_string();
        let rec_id = id.trim_start_matches(&format!("{}:", clean_et));
        let rec_id = rec_id.split_once(':').map(|(_, k)| k).unwrap_or(rec_id);
        (clean_et, rec_id.to_string())
    } else if let Some((table, key)) = id.split_once(':') {
        (table.trim().to_string(), key.trim().to_string())
    } else {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "entity_type".to_string(),
                message: "entity_type query parameter is required when id does not contain entity prefix"
                    .to_string(),
            }],
        }));
    };

    if entity_type.is_empty() {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "entity_type".to_string(),
                message: "entity_type must not be blank".to_string(),
            }],
        }));
    }

    if record_id.is_empty() {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "id".to_string(),
                message: "record id must not be blank".to_string(),
            }],
        }));
    }

    let raw = state
        .entity_use_case
        .get_entity_record(&entity_type, &record_id)
        .await?;

    let response = shape_record(raw, &entity_type);

    Ok((StatusCode::OK, Json(response)))
}

/// Convert a raw SurrealDB JSON record into the flat-attributes response DTO.
fn shape_record(record: serde_json::Value, entity_type: &str) -> EntityRecordResponse {
    let obj = match record {
        serde_json::Value::Object(m) => m,
        other => {
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
fn extract_record_id_part(full_id: &str) -> String {
    full_id
        .split_once(':')
        .map(|(_, key)| key.to_string())
        .unwrap_or_else(|| full_id.to_string())
}
