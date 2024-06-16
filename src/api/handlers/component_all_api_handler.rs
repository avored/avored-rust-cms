use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::component_model::ComponentModel;

pub async fn component_all_api_handler(
    state: State<Arc<AvoRedState>>
) -> Result<Json<Vec<ComponentModel>>> {
    println!("->> {:<12} - component_all_api_handler", "HANDLER");


    Ok(Json(state.component_service.all(&state.db).await?))
}
