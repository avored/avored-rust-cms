use crate::PER_PAGE;
use crate::api::proto::entity::{ DeleteEntityRequest, DeleteEntityResponse, EntityModel as GrpcEntityModel, GetEntityRequest, PutEntityIdentifierRequest, PutEntityIdentifierResponse, StoreEntityRequest, StoreEntityResponse, UpdateEntityRequest, UpdateEntityResponse};
use crate::models::ModelCount;
use crate::models::entity_model::{CreatableEntityModel, PutEntityIdentifierModel, UpdatableEntityModel};
use crate::providers::avored_database_provider::DB;
use crate::repositories::entity_repository::EntityRepository;
use crate::error::Result;

/// admin user service
pub struct EntityService {
    entity_repository: EntityRepository,
}


impl EntityService {
    /// new instance for admin user service
    pub const fn new(
        entity_repository: EntityRepository,
    ) -> Result<Self> {
        Ok(Self {
            entity_repository,
        })
    }


    /// paginate admin user 
    pub async fn paginate(
        &self,
        page: i64,
        order: String,
        (datastore, database_session): &DB,
    ) -> Result<(
        ModelCount,
        Vec<crate::api::proto::entity::EntityModel>,
    )> {
        let entity_model_count = self
            .entity_repository
            .get_total_count(datastore, database_session)
            .await?;

        let per_page: i64 = PER_PAGE;
        let current_page = page;

        let start = current_page * per_page;
        let mut order_column = "id";
        let mut order_type = "desc";
        if !order.is_empty() {
            let mut parts = order.split(':');
            if parts.clone().count() == 2 {
                order_column = parts.clone().nth(0).unwrap_or("");
                order_type = parts.nth(1).unwrap_or("");
            }
        }

        let entities = self
            .entity_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        let mut grpc_entities = vec![];
        for entity in &entities {
            let model: crate::api::proto::entity::EntityModel =
                entity.clone().try_into().unwrap();
            grpc_entities.push(model);
        }

        Ok((entity_model_count, grpc_entities))
    }


     /// store entity
    pub async fn store(
        &self,
        req: StoreEntityRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<StoreEntityResponse> {
        

        let created_entity_model = CreatableEntityModel {
            name: req.name,
            identifier: req.identifier,
            logged_in_username,
        };


        let entity_model = self
            .entity_repository
            .create_entity(datastore, database_session, created_entity_model)
            .await?;

        let model: GrpcEntityModel = entity_model.try_into().unwrap();

        let res = StoreEntityResponse {
            status: true,
            data: Option::from(model),
        };
        Ok(res)
    }


    /// find entity by id
    pub async fn find_entity_by_id(
        &self,
        request: GetEntityRequest,
        (datastore, database_session): &DB,
    ) -> Result<GrpcEntityModel> {
        let entity_model = self
            .entity_repository
            .find_by_id(datastore, database_session, &request.entity_id)
            .await?;

        let model: GrpcEntityModel = entity_model.try_into().unwrap();

        Ok(model)
    }

    /// update entity
    pub async fn update_entity(
        &self,
        req: UpdateEntityRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<UpdateEntityResponse> {
        let updatable_model = UpdatableEntityModel {
            id: req.entity_id,
            name: req.name,
            logged_in_username: logged_in_username.clone(),
        };

        let model = self
            .entity_repository
            .update_entity(datastore, database_session, updatable_model.clone())
            .await?;

        let model: GrpcEntityModel = model.try_into().unwrap();

        let res = UpdateEntityResponse {
            status: true,
            data: Option::from(model),
        };
        Ok(res)
    }

    /// put entity identifier
    pub async fn put_entity_identifier(
        &self,
        req: PutEntityIdentifierRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<PutEntityIdentifierResponse> {
        let updatable_entity_model = PutEntityIdentifierModel {
            id: req.entity_id,
            identifier: req.identifier,
            logged_in_username: logged_in_username.clone(),
        };

        let model = self
            .entity_repository
            .update_entity_identifier(datastore, database_session, updatable_entity_model)
            .await?;

        let model: GrpcEntityModel = model.try_into().unwrap();

        let res = PutEntityIdentifierResponse {
            status: true,
            data: Option::from(model),
        };
        Ok(res)
    }

    /// count of role identifier
    pub async fn count_of_entity_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: &str,
    ) -> Result<ModelCount> {
        self.entity_repository
            .count_of_identifier(datastore, database_session, identifier)
            .await
    }


    /// delete entity
    pub async fn delete_entity(
        &self,
        request: DeleteEntityRequest,
        (datastore, database_session): &DB,
    ) -> Result<DeleteEntityResponse> {
        let delete_status = self
            .entity_repository
            .delete_entity(datastore, database_session, &request.entity_id)
            .await?;

        let response = DeleteEntityResponse {
            status: delete_status,
        };

        Ok(response)
    }

}