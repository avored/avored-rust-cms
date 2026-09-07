use axum::{extract::Path, extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::attribute_dto::AttributeResponse;
use crate::error::Result;

pub async fn fetch_attribute_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AttributeResponse>> {
    let attribute_model = state.attribute_use_case.get_by_id(&id).await?;

    Ok(Json(attribute_model.into()))
    
}
