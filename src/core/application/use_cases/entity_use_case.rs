use crate::core::application::dtos::entity_dto::PaginateEntityCommand;
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::{EntityModel, StorableEntity};
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
        self.repository.create(storable_entity).await
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

    pub async fn update(&self, id: &str, storable_entity: StorableEntity) -> Result<EntityModel> {
        let updated = self.repository.update(id, storable_entity).await?;
        Ok(updated)
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        self.repository.delete(id).await
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
}
