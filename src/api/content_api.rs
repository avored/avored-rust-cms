use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::content::{CollectionAllRequest, CollectionAllResponse};
use crate::avored_state::AvoRedState;
use crate::api::proto::content::content_server::Content;
use crate::error::Error::TonicError;

pub struct ContentApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Content for ContentApi {
    async fn collection_all(
        &self, 
        _request: Request<CollectionAllRequest>
    ) -> Result<Response<CollectionAllResponse>, Status> {
      
        match self.
            state.
            content_service.
            collection_all(&self.state.db).await {
                Ok(reply) => {
                    let res = Response::new(reply);
    
                    Ok(res)
                },
                Err(e) => match e {
                    TonicError(status) => Err(status),
                    _ => Err(Status::internal(e.to_string()))
                }
            }
    }
}