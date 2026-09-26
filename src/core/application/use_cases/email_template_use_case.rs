use crate::core::application::dtos::email_template_dto::{
    CreateEmailTemplateCommand, EmailTemplatePaginationResponse, PaginateEmailTemplateCommand,
};
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::EmailTemplateModel;
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

    pub async fn create(&self, command: CreateEmailTemplateCommand) -> Result<EmailTemplateModel> {
        command.validate("en").await?;

        let body_html = command.body_html.unwrap_or_default();
        let body_plain = command
            .body_plain
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        self.repository
            .create(
                command.name.trim().to_string(),
                command.subject.trim().to_string(),
                body_html,
                body_plain,
            )
            .await
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
