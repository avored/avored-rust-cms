use juniper::graphql_object;
use crate::providers::avored_graphql_provider::Context;
use crate::query::AvoRedQuery;

#[graphql_object]
#[graphql(context = Context)]
impl AvoRedQuery {
    pub async fn api_version(context: &Context,) -> &'static str {
        println!("ctx : {:?}", context);
        "1.0"
    }
}