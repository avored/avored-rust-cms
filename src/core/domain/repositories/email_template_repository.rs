use crate::core::domain::entities::email_template::EmailTemplateModel;
use crate::error::Result;

#[async_trait::async_trait]
pub trait EmailTemplateRepository: Send + Sync {
    async fn create(
        &self,
        name: String,
        subject: String,
        body_html: String,
        body_plain: Option<String>,
    ) -> Result<EmailTemplateModel>;

    async fn find_by_id(&self, id: &str) -> Result<EmailTemplateModel>;

    async fn update(
        &self,
        id: &str,
        name: String,
        subject: String,
        body_html: String,
        body_plain: Option<String>,
    ) -> Result<EmailTemplateModel>;

    async fn paginate(&self, page: u64, page_size: u64) -> Result<(Vec<EmailTemplateModel>, u64)>;
}
