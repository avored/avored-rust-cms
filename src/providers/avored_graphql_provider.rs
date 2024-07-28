use juniper::{EmptyMutation, EmptySubscription, RootNode};
use crate::avored_state::AvoRedState;
use crate::error::Result as AvoredResult;
use crate::query::AvoRedQuery;


#[derive(Debug)]
pub struct Context {}

impl juniper::Context for Context{}

impl Context {
    pub async fn new() -> AvoredResult<Context> {
        Ok(Context {})
    }
}

pub type AvoRedGraphqlSchema = RootNode<'static, AvoRedQuery, EmptyMutation<AvoRedState>, EmptySubscription<AvoRedState>>;

// pub struct AvoRedGraphqlProvider {
    // pub schema: AvoRedGraphqlSchema,
    // pub context: AvoRedGraphqlContext
// }

// impl AvoRedGraphqlProvider {
//     pub async fn register() -> AvoredResult<AvoRedGraphqlProvider> {
//
//         // let context = AvoRedGraphqlContext{};
//         // let schema = AvoRedGraphqlSchema::new(AvoRedQuery, EmptyMutation::new(), EmptySubscription::new());
//
//         Ok(AvoRedGraphqlProvider {})
//     }
// }

