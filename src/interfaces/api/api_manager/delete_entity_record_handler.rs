use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::api_manager_dto::DeleteEntityRecordQuery,
    core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse},
    error::{Error, Result},
};

/// `DELETE /api/v1/entities/{id}`
///
/// Deletes (soft-deletes) a dynamic record and its associated values.
///
/// Returns `204 No Content` with an empty body on success.
/// Returns `404 Not Found` if the record does not exist or has already been deleted.
pub async fn delete_entity_record_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<DeleteEntityRecordQuery>,
) -> Result<StatusCode> {
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

    state
        .entity_use_case
        .delete_entity_record(&entity_type, &record_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
