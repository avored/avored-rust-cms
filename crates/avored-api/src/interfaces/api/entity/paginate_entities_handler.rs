use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{EntityPaginationResponse, PaginateEntityCommand};
use crate::core::application::dtos::pagination_dto::{Pagination, PaginationQuery};
use crate::error::Result;
use axum::extract::Query;
use axum::{extract::State, Json};

pub async fn paginate_entities_handler(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<EntityPaginationResponse>> {
    let page = query.current_page();
    let per_page = query.per_page();
    let command = PaginateEntityCommand {
        page: Some(page),
        page_size: Some(per_page),
    };
    let (data, total) = state.entity_use_case.paginate(command).await?;
    let response = EntityPaginationResponse {
        data: data.into_iter().map(Into::into).collect(),
        pagination: Pagination::new(total, per_page, page),
    };
    Ok(Json(response))
}
