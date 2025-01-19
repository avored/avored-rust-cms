use crate::models::content_model::{ContentModel, CreatableContentModel};
use crate::providers::avored_database_provider::DB;
use crate::repositories::content_repository::ContentRepository;
use crate::error::Result;

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub(crate) async fn create_content(
        &self,
        (datastore, database_session): &DB,
        creatable_page_model: CreatableContentModel,
    ) -> Result<ContentModel> {
        self.content_repository
            .create_content(datastore, database_session, creatable_page_model)
            .await
    }
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Result<Self> {
        Ok(ContentService { content_repository })
    }
}