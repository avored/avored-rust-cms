use serde_json::json;

use crate::core::application::dtos::collection_dto::{
    CollectionPaginationResponse, PaginateCollectionCommand,
};
use crate::core::application::dtos::entity_dto::PaginateEntityCommand;
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::entity::{
    StorableEntityAttribute, UpdableIdentifierEntity, UpdatableEntity,
};
use crate::core::domain::entities::{AttributeModel, EntityModel, StorableEntity};
use crate::core::domain::repositories::EntityRepository;
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct EntityUseCase<R>
where
    R: EntityRepository,
{
    repository: R,
}

impl<R> EntityUseCase<R>
where
    R: EntityRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel> {
        let attributes = storable_entity.attributes.clone();
        let logged_in_user = storable_entity.logged_in_user_email.clone();
        let mut entity_model = self.repository.create(storable_entity).await?;

        for attribute in attributes {
            let attrobute_model = self
                .repository
                .create_attribute(attribute, entity_model.id.clone(), logged_in_user.clone())
                .await?;

            entity_model.attributes.push(attrobute_model);
        }


        self.repository
            .create_collection_table(&entity_model.identifier)
            .await?;


        Ok(entity_model)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<EntityModel> {
        self.repository.find_by_id(id).await
    }

    pub async fn paginate(&self, query: PaginateEntityCommand) -> Result<(Vec<EntityModel>, u64)> {
        let page: u64 = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let entities = self.repository.paginate(page, page_size).await?;
        let modal_count = self.repository.count().await?;

        Ok((entities, modal_count.total))
    }

    pub async fn update(&self, id: &str, updatable_entity: UpdatableEntity) -> Result<EntityModel> {
        let mut updated = self.repository.update(id, updatable_entity.clone()).await?;
        let logged_in_user = updatable_entity.logged_in_user_email.clone();

        for attribute in updatable_entity.attributes {
            let attrobute_model : AttributeModel;

            if attribute.is_new {
                let storable_entity_attribute = StorableEntityAttribute {
                    name: attribute.name,
                    identifier: attribute.identifier,
                    data_type: attribute.data_type,
                    field_type: attribute.field_type,
                };
                attrobute_model = self
                    .repository
                    .create_attribute(
                        storable_entity_attribute,
                        id.to_string(),
                        logged_in_user.clone(),
                    )
                    .await?;
            } else {
                attrobute_model = self
                    .repository
                    .update_attribute(
                        attribute,
                        logged_in_user.clone(),
                    )
                    .await?;
            }

            updated.attributes.push(attrobute_model);
        }

        Ok(updated)
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = self.repository.delete(id).await?;

        self.repository.delete_attribute(id).await?;
        // if result

        Ok(result)
    }

    pub async fn get_by_identifier(&self, identifier: &str) -> Result<EntityModel> {
        self.repository.find_by_identifier(identifier).await
    }

    pub async fn entity_count_by_identifier(&self, identifier: &str) -> Result<u64> {
        let entity = self.repository.find_by_identifier(identifier).await;
        match entity {
            Ok(_) => Ok(1),
            Err(Error::NotFound(_)) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub async fn list_options(&self) -> Result<Vec<EntityModel>> {
        self.repository.list_options().await
    }

    pub async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierEntity,
    ) -> Result<EntityModel> {
        self.repository
            .update_identifier(id, updatable_identifier)
            .await
    }

    pub async fn paginate_collection(
        &self,
        entity_id: &str,
        query: PaginateCollectionCommand,
    ) -> Result<CollectionPaginationResponse> {
        let entity = self.repository.find_by_id(entity_id).await?;
        let page: u64 = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

        let (mut data, total) = self
            .repository
            .paginate_collection(&entity.identifier, page, page_size)
            .await?;

        let test = json!({"id": "id", "name": "Admin", "email": "admin@avored.com"});

        data.push(test);


        Ok(CollectionPaginationResponse {
            entity,
            data,
            total,
        })
    }
}
