use crate::api::proto::setting::setting_server::Setting;
use crate::api::proto::setting::{
    GetSettingRequest, GetSettingResponse, StoreSettingRequest, StoreSettingResponse,
};
use crate::avored_state::AvoRedState;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Response, Status};

/// AvoRed Setting API
pub struct SettingApi {
    /// The AvoRed state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Setting for SettingApi {
    async fn get_setting(
        &self,
        request: tonic::Request<GetSettingRequest>,
    ) -> Result<Response<GetSettingResponse>, Status> {
        println!("->> {:<12} - get_setting", "gRPC_Setting_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_setting"),
            )
            .await?;

        match self.state.setting_service.get_setting(&self.state.db).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn store_setting(
        &self,
        request: tonic::Request<StoreSettingRequest>,
    ) -> Result<Response<StoreSettingResponse>, Status> {
        println!("->> {:<12} - store_setting", "gRPC_Setting_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("store_setting"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .setting_service
            .store_setting(&self.state.db, req, claims.email)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
