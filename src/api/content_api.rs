use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::content::{CollectionAllRequest, CollectionAllResponse, ContentPaginateRequest, ContentPaginateResponse, GetContentRequest, GetContentResponse, PutContentIdentifierRequest, PutContentIdentifierResponse, StoreContentRequest, StoreContentResponse, UpdateContentRequest, UpdateContentResponse};
use crate::avored_state::AvoRedState;
use crate::api::proto::content::content_server::Content;
use crate::error::Error::TonicError;
use crate::models::token_claim_model::TokenClaims;

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
    
    async fn content_paginate(
        &self, 
        request: Request<ContentPaginateRequest>
    ) -> Result<Response<ContentPaginateResponse>, Status> {
        let req = request.into_inner();
        match self.
            state.
            content_service.
            content_paginate(req, &self.state.db).await {
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

    async fn store_content(
        &self,
        request: Request<StoreContentRequest>,
    ) -> Result<Response<StoreContentResponse>, Status> {

        let claim = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
        match self.
            state.
            content_service.
            store_content(
                req,
                claim.email,
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

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentResponse>, Status> {

        // let claim = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
        match self.
            state.
            content_service.
            get_content(
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

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentResponse>, Status> {
        let claim = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
        match self.
            state.
            content_service.
            update_content(
                &self.state.db,
                req,
                claim.email,
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

    async fn put_content_identifier(
        &self,
        request: Request<PutContentIdentifierRequest>,
    ) -> Result<Response<PutContentIdentifierResponse>, Status> {
        let claim = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
        match self.
            state.
            content_service.
            put_content_identifier(
                &self.state.db,
                req,
                claim.email,
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