use crate::models::model_model::ModelModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PutModelIdentifierResponse {
    pub model: ModelModel,
}
