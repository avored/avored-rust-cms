use crate::core::domain::entities::email_template::EmailTemplateModel;
use crate::error::Result;

#[async_trait::async_trait]
pub trait EmailTemplateRepository: Send + Sync {
    async fn paginate(&self, page: u64, page_size: u64) -> Result<(Vec<EmailTemplateModel>, u64)>;
}
