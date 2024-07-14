use juniper::{EmptyMutation, EmptySubscription, RootNode};
use crate::error::Result as AvoredResult;
use crate::query::AvoRedQuery;

pub struct AvoRedGraphqlContext {}

impl juniper::Context for AvoRedGraphqlContext{}

// impl AvoRedGraphqlContext {
//     pub async fn new() -> AvoredResult<AvoRedGraphqlContext> {
//         Ok(AvoRedGraphqlContext {})
//     }
// }

pub type AvoRedGraphqlSchema = RootNode<'static, AvoRedQuery, EmptyMutation, EmptySubscription>;

pub struct AvoRedGraphqlProvider {
    pub schema: AvoRedGraphqlSchema,
    pub context: AvoRedGraphqlContext
}

impl AvoRedGraphqlProvider {
    pub async fn register() -> AvoredResult<AvoRedGraphqlProvider> {

        let context = AvoRedGraphqlContext{};
        let schema = AvoRedGraphqlSchema::new(AvoRedQuery, EmptyMutation::new(), EmptySubscription::new());

        Ok(AvoRedGraphqlProvider {schema, context})
    }
}

