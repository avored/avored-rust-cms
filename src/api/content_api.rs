use crate::api::proto::content::content_server::Content;
use crate::api::proto::content::{
    CollectionAllRequest, CollectionAllResponse, ContentPaginateRequest, ContentPaginateResponse,
    DeleteContentRequest, DeleteContentResponse, GetCollectionRequest, GetCollectionResponse,
    GetContentRequest, GetContentResponse, PutContentIdentifierRequest,
    PutContentIdentifierResponse, StoreCollectionRequest, StoreCollectionResponse,
    StoreContentRequest, StoreContentResponse, UpdateCollectionRequest, UpdateCollectionResponse,
    UpdateContentRequest, UpdateContentResponse,
};
use crate::avored_state::AvoRedState;
use crate::error::Error::Tonic;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

/// AvoRed Content API
pub struct ContentApi {
    /// The AvoRed state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Content for ContentApi {
    async fn collection_all(
        &self,
        request: Request<CollectionAllRequest>,
    ) -> Result<Response<CollectionAllResponse>, Status> {
        println!("->> {:<12} - collection_all", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("collection_all"),
            )
            .await?;

        match self
            .state
            .content_service
            .collection_all(&self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn content_paginate(
        &self,
        request: Request<ContentPaginateRequest>,
    ) -> Result<Response<ContentPaginateResponse>, Status> {
        println!("->> {:<12} - content_paginate", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("content_paginate"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .content_service
            .content_paginate(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn store_content(
        &self,
        request: Request<StoreContentRequest>,
    ) -> Result<Response<StoreContentResponse>, Status> {
        println!("->> {:<12} - store_content", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("store_content"),
            )
            .await?;

        let req = request.into_inner();
        req.validate(&self.state).await?;

        match self
            .state
            .content_service
            .store_content(req, claims.email, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentResponse>, Status> {
        println!("->> {:<12} - get_content", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_content"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .content_service
            .get_content(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentResponse>, Status> {
        println!("->> {:<12} - update_content", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("update_content"),
            )
            .await?;

        let req = request.into_inner();
        req.validate().await?;

        match self
            .state
            .content_service
            .update_content(&self.state.db, req, claims.email)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn put_content_identifier(
        &self,
        request: Request<PutContentIdentifierRequest>,
    ) -> Result<Response<PutContentIdentifierResponse>, Status> {
        println!(
            "->> {:<12} - put_content_identifier",
            "gRPC_Content_Api_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("put_content_identifier"),
            )
            .await?;

        let req = request.into_inner();
        req.validate(&self.state).await?;

        match self
            .state
            .content_service
            .put_content_identifier(&self.state.db, req, claims.email)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_collection(
        &self,
        request: Request<GetCollectionRequest>,
    ) -> Result<Response<GetCollectionResponse>, Status> {
        println!("->> {:<12} - get_collection", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_collection"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .content_service
            .get_collection(&self.state.db, req)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
    async fn store_collection(
        &self,
        request: Request<StoreCollectionRequest>,
    ) -> Result<Response<StoreCollectionResponse>, Status> {
        println!("->> {:<12} - store_collection", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("store_collection"),
            )
            .await?;

        let req = request.into_inner();
        req.validate(&self.state).await?;

        match self
            .state
            .content_service
            .store_collection(&self.state.db, req, &logged_in_user.email)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_collection(
        &self,
        request: Request<UpdateCollectionRequest>,
    ) -> Result<Response<UpdateCollectionResponse>, Status> {
        println!("->> {:<12} - update_collection", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("update_collection"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .content_service
            .update_collection(&self.state.db, req, &logged_in_user.email)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn delete_content(
        &self,
        request: Request<DeleteContentRequest>,
    ) -> Result<Response<DeleteContentResponse>, Status> {
        println!("->> {:<12} - delete_content", "gRPC_Content_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_content"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .content_service
            .delete_content(&self.state.db, &req.content_id, &req.content_type)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
