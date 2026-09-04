use crate::core::application::dtos::entity_dto::{
    CreateEntityCommand, EntityPaginationResponse, EntityResponse, PaginateEntityCommand, UpdateEntityCommand,
};
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse};
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

    pub async fn create(&self, command: CreateEntityCommand) -> Result<EntityResponse> {
        // Check uniqueness of identifier
        if let Some(_) = self.repository.find_by_identifier(&command.identifier).await? {
            let error_response = ErrorResponse {
                status: false,
                errors: vec![ErrorMessageResponse {
                    key: "identifier".to_string(),
                    message: format!("Identifier '{}' already exists", command.identifier),
                }],
            };
            return Err(Error::BadRequest(error_response));
        }

        let storable = command.to_storable();
        let entity = self.repository.create(storable).await?;
        Ok(entity.into())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<EntityResponse>> {
        let entity = self.repository.find_by_id(id).await?;
        Ok(entity.map(Into::into))
    }

    pub async fn paginate(&self, query: PaginateEntityCommand) -> Result<EntityPaginationResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let entities = self.repository.paginate(page, page_size).await?;
        let modal_count = self.repository.count().await?;
        let data = entities.into_iter().map(Into::into).collect();

        Ok(EntityPaginationResponse { data, total: modal_count.total })
    }

    pub async fn update(&self, id: &str, command: UpdateEntityCommand) -> Result<EntityResponse> {
        // Verify existence
        let existing = self.repository.find_by_id(id).await?;
        if existing.is_none() {
            return Err(Error::Generic(format!("Entity with id '{}' not found", id)));
        }

        // Verify identifier uniqueness if changed
        if let Some(by_identifier) = self.repository.find_by_identifier(&command.identifier).await? {
            let existing_id = existing.unwrap().id;
            if by_identifier.id != existing_id {
                let error_response = ErrorResponse {
                    status: false,
                    errors: vec![ErrorMessageResponse {
                        key: "identifier".to_string(),
                        message: format!("Identifier '{}' is already in use", command.identifier),
                    }],
                };
                return Err(Error::BadRequest(error_response));
            }
        }

        let storable = command.to_storable();
        let updated = self.repository.update(id, storable).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let existing = self.repository.find_by_id(id).await?;
        if existing.is_none() {
            return Err(Error::Generic(format!("Entity with id '{}' not found", id)));
        }

        self.repository.delete(id).await
    }
}
