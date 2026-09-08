use crate::core::application::dtos::attribute_dto::PaginateAttributeCommand;
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::attribute::UpdableIdentifierAttribute;
use crate::core::domain::entities::{AttributeModel, StorableAttribute};
use crate::core::domain::repositories::AttributeRepository;
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct AttributeUseCase<R>
where
    R: AttributeRepository,
{
    repository: R,
}

impl<R> AttributeUseCase<R>
where
    R: AttributeRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn attribute_count_by_identifier(&self, identifier: &str) -> Result<u64> {
        let entity = self.repository.find_by_identifier(identifier).await;
        match entity {
            Ok(_) => Ok(1),
            Err(Error::NotFound(_)) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub async fn create(&self, storable_model: StorableAttribute) -> Result<AttributeModel> {
        let model = self.repository.create(storable_model).await?;
        Ok(model)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<AttributeModel> {
        self.repository.find_by_id(id).await
    }

    pub async fn paginate(
        &self,
        query: PaginateAttributeCommand,
    ) -> Result<(Vec<AttributeModel>, u64)> {
        let page = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let paginate_models = self.repository.paginate(page, page_size).await?;
        let modal_count = self.repository.count().await?;
        // let data = entities.into_iter().map(Into::into).collect();

        Ok((paginate_models, modal_count.total))
    }

    pub async fn update(
        &self,
        id: &str,
        storable_entity: StorableAttribute,
    ) -> Result<AttributeModel> {
        self.repository.update(id, storable_entity).await
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        self.repository.delete(id).await
    }

    pub async fn get_by_identifier(&self, identifier: &str) -> Result<AttributeModel> {
        let entity = self.repository.find_by_identifier(identifier).await?;
        Ok(entity)
    }

    pub async fn list_options(&self) -> Result<Vec<AttributeModel>> {
        self.repository.list_options().await
    }

    pub async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierAttribute,
    ) -> Result<AttributeModel> {
        self.repository.update_identifier(id, updatable_identifier).await
    }

}
