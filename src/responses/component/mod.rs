use crate::models::component_model::ComponentModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PutComponentIdentifierResponse {
    pub component: ComponentModel,
}
