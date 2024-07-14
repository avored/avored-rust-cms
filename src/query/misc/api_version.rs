use juniper::graphql_object;
use crate::query::AvoRedQuery;

#[graphql_object]
impl AvoRedQuery {
    fn api_version() -> &'static str {
        "1.0"
    }
}