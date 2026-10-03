use axum::{
    extract::State,
    http::StatusCode,
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::api_manager_dto::{CreateEntityRecordRequest, EntityRecordResponse},
    core::domain::entities::{
        error_message::{ErrorMessageResponse, ErrorResponse},
        user::TokenClaims,
    },
    error::{Error, Result},
};

/// `POST /api/v1/entities`
///
/// Creates a new dynamic record for the supplied entity type and returns the
/// complete persisted record with a `201 Created` status.
///
/// # Request body
/// ```json
/// {
///   "entity_type": "blog_posts",
///   "attributes": { "title": "Hello", "views": 10 }
/// }
/// ```
///
/// # Error responses
/// - `400 Bad Request` — missing or blank `entity_type`, or empty attributes key
/// - `404 Not Found`  — unknown entity type (propagated from use case)
/// - `500`            — persistence failure
pub async fn create_entity_record_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Json(payload): Json<CreateEntityRecordRequest>,
) -> Result<(StatusCode, Json<EntityRecordResponse>)> {
    // --- Input validation ---

    let entity_type = payload.entity_type.trim().to_string();
    if entity_type.is_empty() {
        return Err(Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "entity_type".to_string(),
                message: "entity_type is required".to_string(),
            }],
        }));
    }

    // Validate that attribute keys are non-empty strings.
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

    // --- Delegate to use case ---

    let raw = state
        .entity_use_case
        .create_entity_record(&entity_type, payload.attributes, &logged_in_user.email)
        .await?;

    // --- Shape response ---

    let response = shape_record(raw, &entity_type);

    Ok((StatusCode::CREATED, Json(response)))
}

/// Convert a raw SurrealDB JSON record into the flat-attributes response DTO.
///
/// `id` is the bare record key (colon-stripped); internal bookkeeping fields
/// are excluded from `attributes`.
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
