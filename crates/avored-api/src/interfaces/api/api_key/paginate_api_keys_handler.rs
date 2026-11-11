use axum::extract::{Query, State};
use axum::Json;

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::ListApiKeysResponse;
use crate::core::application::dtos::pagination_dto::{PaginatedResponse, PaginationQuery};
use crate::error::Result;
use crate::interfaces::api::api_key::api_key_model_to_response;

pub async fn paginate_api_keys_handler(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<ListApiKeysResponse>> {
    let page = query.current_page();
    let per_page = query.per_page();

    let (models, total) = state.api_key_use_case.list(page, per_page).await?;

    let data = models.into_iter().map(api_key_model_to_response).collect();
    Ok(Json(PaginatedResponse {
        data,
        pagination: crate::core::application::dtos::pagination_dto::Pagination::new(
            total, per_page, page,
        ),
    }))
}
