use crate::avored_state::AvoRedState;
use crate::query::AvoRedQuery;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub type AvoRedGraphqlSchema =
    RootNode<'static, AvoRedQuery, EmptyMutation<AvoRedState>, EmptySubscription<AvoRedState>>;
