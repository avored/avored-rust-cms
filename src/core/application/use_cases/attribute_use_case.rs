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
}
