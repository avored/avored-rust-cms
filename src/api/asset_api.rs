use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::asset::asset_server::Asset;
use crate::api::proto::asset::{AssetPaginateRequest, AssetPaginateResponse, CreateFolderRequest, CreateFolderResponse, DeleteAssetRequest, DeleteAssetResponse, DeleteFolderRequest, DeleteFolderResponse, RenameAssetRequest, RenameAssetResponse};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;

pub struct AssetApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Asset for AssetApi {
    async fn paginate(
        &self,
        request: Request<AssetPaginateRequest>
    ) -> Result<Response<AssetPaginateResponse>, Status>
    {
        println!("->> {:<12} - paginate_asset", "gRPC_Asset_Api_Service");
        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("paginate_asset"),
            )
            .await?;

        let req = request.into_inner();


        match self.
            state.
            asset_service.
            paginate(
                req,
                &self.state.db
            ).await {
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

    async fn create_folder(
        &self,
        request: Request<CreateFolderRequest>
    ) -> Result<Response<CreateFolderResponse>, Status>
    {
        println!("->> {:<12} - create_folder", "gRPC_Asset_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("create_folder"),
            )
            .await?;

        let req = request.into_inner();

        match self.
            state.
            asset_service.
            create_asset_folder(
                &self.state.db,
                req,
                &logged_in_user.email
            ).await {
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

    async fn delete_asset(
        &self,
        request: Request<DeleteAssetRequest>
    ) -> Result<Response<DeleteAssetResponse>, Status>
    {
        println!("->> {:<12} - delete_asset", "gRPC_Asset_Api_Service");
        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("asset_delete"),
            )
            .await?;

        let req = request.into_inner();

        match self.
            state.
            asset_service.
            delete_asset(
                &self.state.db,
                req
            ).await {
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
    
    async fn delete_folder(
        &self,
        request: Request<DeleteFolderRequest>
    ) -> Result<Response<DeleteFolderResponse>, Status>
    {
        println!("->> {:<12} - delete_folder", "gRPC_Asset_Api_Service");
        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_folder"),
            )
            .await?;

        let req = request.into_inner();

        match self.
            state.
            asset_service.
            delete_folder(
                &self.state.db,
                req
            ).await {
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

    async fn rename_asset(
        &self,
        request: Request<RenameAssetRequest>
    ) -> Result<Response<RenameAssetResponse>, Status>
    {
        println!("->> {:<12} - rename_asset", "gRPC_Asset_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("rename_asset"),
            )
            .await?;

        let req = request.into_inner();

        match self.
            state.
            asset_service.
            rename_asset(
                &self.state.db,
                req,
                &claims.email
            ).await {
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
