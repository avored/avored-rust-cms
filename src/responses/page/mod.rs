use serde::Serialize;
use crate::models::page_model::PageModel;

#[derive(Serialize)]
pub struct PutPageIdentifierResponse {
    pub page: PageModel
}