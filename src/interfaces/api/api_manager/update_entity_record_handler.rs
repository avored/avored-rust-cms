use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::api_manager_dto::{
        EntityRecordResponse, UpdateEntityRecordQuery, UpdateEntityRecordRequest,
    },
    core::domain::entities::{
        error_message::{ErrorMessageResponse, ErrorResponse},
        user::TokenClaims,
    },
    error::{Error, Result},
};

/// `PATCH /api/v1/entities/{id}`
///
/// Partially updates dynamic record attributes and returns the complete
/// post-update record.
///
/// # Request shape
/// Body:
/// ```json
/// {
///   "entity_type": "blog_posts", // optional if specified via query param
///   "attributes": { "title": "Updated Title" }
/// }
/// ```
/// Query: `?entity_type=blog_posts` (optional if specified in body)
///
/// # Status codes
/// - `200 OK`           — successful update
/// - `400 Bad Request`  — missing entity_type, blank attribute key, or invalid type/unknown attribute
/// - `404 Not Found`    — unknown entity type or record not found
/// - `500`              — persistence failure
pub async fn update_entity_record_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    Query(query): Query<UpdateEntityRecordQuery>,
    Json(payload): Json<UpdateEntityRecordRequest>,
) -> Result<(StatusCode, Json<EntityRecordResponse>)> {
    // Resolve entity_type from body or query param
    let entity_type = payload
        .entity_type
        .or(query.entity_type)
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if entity_type.is_empty() {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "entity_type".to_string(),
                message: "entity_type is required in body or query param".to_string(),
            }],
        }));
    }

    // Validate that attribute keys are non-empty strings
    for key in payload.attributes.keys() {
        if key.trim().is_empty() {
            return Err(Error::BadRequest(ErrorResponse {
                status: false,
                errors: vec![ErrorMessageResponse {
                    key: "attributes".to_string(),
                    message: "attribute keys must not be blank".to_string(),
                }],
            }));
        }
    }

    let record_id = id.trim_start_matches(&format!("{}:", entity_type));
    let record_id = record_id.split_once(':').map(|(_, k)| k).unwrap_or(record_id);

    let raw = state
        .entity_use_case
        .update_entity_record(
            &entity_type,
            record_id,
            payload.attributes,
            &logged_in_user.email,
        )
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
