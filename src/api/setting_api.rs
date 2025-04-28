use std::sync::Arc;
use tonic::{async_trait, Response, Status};
use crate::api::proto::setting::{GetSettingRequest, GetSettingResponse};
use crate::api::proto::setting::setting_server::Setting;
use crate::avored_state::AvoRedState;

pub struct SettingApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Setting for SettingApi {
    async fn get_setting(
        &self, 
        _request: tonic::Request<GetSettingRequest>
    ) -> Result<Response<GetSettingResponse>, tonic::Status> {
        match self.
            state.
            setting_service.
            get_setting(
                &self.state.db
            ).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string()))
        }
    }
}