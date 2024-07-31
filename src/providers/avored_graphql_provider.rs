use juniper::{EmptyMutation, EmptySubscription, RootNode};
use crate::avored_state::AvoRedState;
use crate::query::AvoRedQuery;

pub type AvoRedGraphqlSchema = RootNode<'static, AvoRedQuery, EmptyMutation<AvoRedState>, EmptySubscription<AvoRedState>>;
