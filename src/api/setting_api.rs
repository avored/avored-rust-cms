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
        request: tonic::Request<GetSettingRequest>
    ) -> Result<Response<GetSettingResponse>, tonic::Status> {
        println!("->> {:<12} - get_setting", "gRPC_Setting_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("get_setting"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
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
        println!("->> {:<12} - store_setting", "gRPC_Setting_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("store_setting"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
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