use std::sync::Arc;
use tonic::{Response, Status, async_trait};

use crate::{api::proto::web_event::{WebEventRequest, WebEventResponse, web_event_server::WebEvent}, avored_state::AvoRedState};



/// `AvoRed` Setting API
pub struct WebEventApi {
    /// The `AvoRed` state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl WebEvent for WebEventApi {
    async fn send_event(
        &self,
        request: tonic::Request<WebEventRequest>,
    ) -> Result<Response<WebEventResponse>, Status> {
        println!("->> {:<12} - sent_event", "gRPC_Webhook_Service");

        let req = request.into_inner();
        println!("Received webhook event: {req:?}");
        
        
        let response = self
            .state
            .web_event_service
            .send_event(req)
            .await?;
        
        let res = Response::new(response);

        Ok(res)
    }
}