use juniper::{EmptyMutation, EmptySubscription, RootNode};
use crate::error::Result as AvoredResult;
use crate::query::AvoRedQuery;


#[derive(Debug)]
pub struct Context {
    pub id: String
}

impl juniper::Context for Context{}

impl Context {
    pub async fn new() -> AvoredResult<Context> {
        Ok(Context {id: String::from("test gql")})
    }
}

pub type AvoRedGraphqlSchema = RootNode<'static, AvoRedQuery, EmptyMutation<Context>, EmptySubscription<Context>>;

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

