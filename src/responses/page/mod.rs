use serde::Serialize;
use crate::models::page_model::NewPageModel;
use crate::responses::ApiResponse;
use crate::error::Result;

#[derive(Serialize)]
pub struct PutPageIdentifierResponse {
    pub page: NewPageModel
}


#[derive(Serialize)]
pub struct FetchPageCmsResponse {
    pub page_model: NewPageModel
}

impl NewPageModel {
    pub fn convert_to_response(&self) -> Result<ApiResponse<FetchPageCmsResponse>>
    {
        Ok(ApiResponse {
            status: true,
            data: FetchPageCmsResponse {
                page_model: self.to_owned()
            }
        })
    }
}