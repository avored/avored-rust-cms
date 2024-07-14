use std::sync::Arc;
use axum::Extension;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use crate::providers::avored_graphql_provider::{AvoRedGraphqlSchema, Context};

pub async fn graphql_api_handler(
    Extension(schema): Extension<Arc<AvoRedGraphqlSchema>>,
    Extension(ctx): Extension<Arc<Context>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    println!("->> {:<12} - graphql_api_handler", "HANDLER");
    let res  = request.execute(&*schema, &ctx).await;
    JuniperResponse(res)
}

