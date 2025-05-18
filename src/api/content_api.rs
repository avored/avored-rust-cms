use crate::api::proto::content::content_server::Content;
use crate::api::proto::content::{CollectionAllRequest, CollectionAllResponse, ContentPaginateRequest, ContentPaginateResponse, GetCollectionRequest, GetCollectionResponse, GetContentRequest, GetContentResponse, PutContentIdentifierRequest, PutContentIdentifierResponse, StoreCollectionRequest, StoreCollectionResponse, StoreContentRequest, StoreContentResponse, UpdateCollectionRequest, UpdateCollectionResponse, UpdateContentRequest, UpdateContentResponse};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::models::token_claim_model::TokenClaims;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

pub struct ContentApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Content for ContentApi {
    async fn collection_all(
        &self,
        request: Request<CollectionAllRequest>,
    ) -> Result<Response<CollectionAllResponse>, Status> {
        println!("->> {:<12} - collection_all", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("content_paginate"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn content_paginate(
        &self,
        request: Request<ContentPaginateRequest>,
    ) -> Result<Response<ContentPaginateResponse>, Status> {
        println!("->> {:<12} - content_paginate", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("content_paginate"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn store_content(
        &self,
        request: Request<StoreContentRequest>,
    ) -> Result<Response<StoreContentResponse>, Status> {
        println!("->> {:<12} - store_content", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("store_content"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentResponse>, Status> {
        println!("->> {:<12} - get_content", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("get_content"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentResponse>, Status> {
        println!("->> {:<12} - update_content", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("update_content"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
        println!("request {:#?}", req);
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
                TonicError(status) => Err(status),
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

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("put_content_identifier"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_collection(
        &self,
        request: Request<GetCollectionRequest>,
    ) -> Result<Response<GetCollectionResponse>, Status> {
        println!("->> {:<12} - get_collection", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("get_collection"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
    async fn store_collection(
        &self,
        request: Request<StoreCollectionRequest>,
    ) -> Result<Response<StoreCollectionResponse>, Status> {
        println!("->> {:<12} - store_collection", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user.clone(), String::from("store_collection"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_collection(
        &self,
        request: Request<UpdateCollectionRequest>,
    ) -> Result<Response<UpdateCollectionResponse>, Status> {
        println!("->> {:<12} - update_collection", "gRPC_Content_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user.clone(), String::from("update_collection"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
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
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
