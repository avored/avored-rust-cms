use serde::Serialize;
use crate::models::model_model::ModelModel;

#[derive(Serialize)]
pub struct PutModelIdentifierResponse {
    pub model: ModelModel
}