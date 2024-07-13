use serde::Serialize;
use crate::models::component_model::ComponentModel;

#[derive(Serialize)]
pub struct PutComponentIdentifierResponse {
    pub component: ComponentModel
}