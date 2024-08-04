use serde::Serialize;
use crate::models::page_model::PageModel;
use crate::responses::ApiResponse;
use crate::error::Result;

#[derive(Serialize)]
pub struct PutPageIdentifierResponse {
    pub page: PageModel
}


#[derive(Serialize)]
pub struct FetchPageCmsResponse {
    pub page_model: PageModel
}

impl PageModel {
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