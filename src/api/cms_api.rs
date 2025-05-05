use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::cms::cms_server::Cms;
use crate::api::proto::cms::{GetCmsContentRequest, GetCmsContentResponse};
use crate::avored_state::AvoRedState;

pub struct CmsApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Cms for CmsApi {
    async fn get_cms_content(
        &self, 
        request: Request<GetCmsContentRequest>
    ) -> Result<Response<GetCmsContentResponse>, Status> {
        
        let req = request.into_inner();
      
        match self.
            state.
            cms_service.
            get_cms_content(
                req,
                &self.state.db
            ).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string()))
        }

    }
}