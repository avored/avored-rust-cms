use crate::core::application::dtos::email_template_dto::{
    EmailTemplatePaginationResponse, PaginateEmailTemplateCommand,
};
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::repositories::EmailTemplateRepository;
use crate::error::Result;

#[derive(Clone)]
pub struct EmailTemplateUseCase<R>
where
    R: EmailTemplateRepository,
{
    repository: R,
}

impl<R> EmailTemplateUseCase<R>
where
    R: EmailTemplateRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn paginate(
        &self,
        query: PaginateEmailTemplateCommand,
    ) -> Result<EmailTemplatePaginationResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let (data, total) = self.repository.paginate(page, page_size).await?;

        Ok(EmailTemplatePaginationResponse {
            data,
            total,
            page,
            page_size,
        })
    }
}
