use std::sync::Arc;
use tonic::{async_trait, Response, Status};
use crate::api::proto::setting::{GetSettingRequest, GetSettingResponse, StoreSettingRequest, StoreSettingResponse};
use crate::api::proto::setting::setting_server::Setting;
use crate::avored_state::AvoRedState;
use crate::models::token_claim_model::TokenClaims;

pub struct SettingApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Setting for SettingApi {
    async fn get_setting(
        &self, 
        _request: tonic::Request<GetSettingRequest>
    ) -> Result<Response<GetSettingResponse>, tonic::Status> {
        println!("->> {:<12} - get_setting", "GRPC_SERVICE");
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

    async fn store_setting(
        &self,
        request: tonic::Request<StoreSettingRequest>
    ) -> Result<Response<StoreSettingResponse>, tonic::Status> {
        println!("->> {:<12} - store_setting", "GRPC_SERVICE");
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
        match self.
            state.
            setting_service.
            store_setting(
                &self.state.db,
                req,
                claims.email
            ).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string()))
        }
    }
}