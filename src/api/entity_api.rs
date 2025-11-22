use crate::api::proto::entity::entity_paginate_response::{EntityPaginateData, EntityPagination};
use crate::api::proto::entity::entty_service_server::EnttyService;
use crate::api::proto::entity::{EntityPaginateRequest, EntityPaginateResponse, StoreEntityRequest, StoreEntityResponse};
use crate::avored_state::AvoRedState;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Response, Status};
use tonic::Request;
use crate::error::Error::Tonic;


/// `AvoRed` Setting API
pub struct EntityApi {
    /// The `AvoRed` state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl EnttyService for EntityApi {
    async fn store_entity(
        &self,
        request: Request<StoreEntityRequest>,
    ) -> Result<Response<StoreEntityResponse>, Status> {
        println!("->> {:<12} - store_entity", "gRPC_Entity_Service");

        
        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        let req = request.into_inner();

        let res = self
            .state
            .entity_service.store(
                req,
                logged_in_user.email,
                &self.state.db,
            ).await?;
        let res = Response::new(res);

        Ok(res)
    }

    async fn paginate(
        &self,
        request: Request<EntityPaginateRequest>,
    ) -> Result<Response<EntityPaginateResponse>, Status> {
        println!("->> {:<12} - paginate", "gRPC_Entity_Service");
        
         let claims = request.get_token_claim()?;

        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("paginate_entity"),
            )
            .await?;

        let req = request.into_inner();
        let page = req.page.unwrap_or_default();
        let order = req.order.unwrap_or_default();

        match self
            .state
            .entity_service
            .paginate(page, order, &self.state.db)
            .await
        {
            Ok(entity_paginate_data) => {
                let pagination = EntityPagination {
                    total: entity_paginate_data.0.total,
                };

                let paginate_data = EntityPaginateData {
                    pagination: Option::from(pagination),
                    data: entity_paginate_data.1,
                };

                let entity_paginate_response = EntityPaginateResponse {
                    status: true,
                    data: Option::from(paginate_data),
                };

                Ok(Response::new(entity_paginate_response))
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
