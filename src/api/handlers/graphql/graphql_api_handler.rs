use crate::avored_state::AvoRedState;
use crate::providers::avored_graphql_provider::AvoRedGraphqlSchema;
use axum::extract::State;
use axum::Extension;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use std::sync::Arc;

pub async fn graphql_api_handler(
    Extension(schema): Extension<Arc<AvoRedGraphqlSchema>>,
    ctx: State<Arc<AvoRedState>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    println!("->> {:<12} - graphql_api_handler", "HANDLER");
    let res = request.execute(&*schema, &ctx).await;
    JuniperResponse(res)
}
