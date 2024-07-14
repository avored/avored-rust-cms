use std::sync::Arc;
use axum::extract::State;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use crate::avored_state::AvoRedState;

pub async fn graphql_api_handler(
    state: State<Arc<AvoRedState>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    println!("->> {:<12} - graphql_api_handler", "HANDLER");
    let schematest = state.clone();

    JuniperResponse(request.execute(&schematest.schema, &()).await)

}