use crate::models::content_model::{ContentModel, CreatableContentModel};
use crate::providers::avored_database_provider::DB;
use crate::repositories::content_repository::ContentRepository;
use crate::error::Result;

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub(crate) async fn create_content(&self, _p0: &DB, _p1: CreatableContentModel) -> Result<ContentModel> {
        let model = ContentModel::default();

        Ok(model)
    }
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Result<Self> {
        Ok(ContentService { content_repository })
    }
}