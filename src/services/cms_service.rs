use std::net::SocketAddr;

use crate::api::proto::cms::{
    GetCmsContentRequest, GetCmsContentResponse, SentContactFormRequest, SentContactFormResponse,
};
use crate::api::proto::content::ContentModel;
use crate::error::{Error, Result};
use crate::extensions::email_message_builder::EmailMessageBuilder;
use crate::models::visitor_log_model::CreatableVisitorLogModel;
use crate::providers::avored_database_provider::DB;
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::content_repository::ContentRepository;
use crate::repositories::visitor_log_repository::VisitorLogRepository;
use lettre::{AsyncTransport, Message};
use serde::{Deserialize, Serialize};
use tracing::log::error;

/// cms service
pub struct CmsService {
    content_repository: ContentRepository,
    visitor_log_repository: VisitorLogRepository
}

impl CmsService {
    /// new instance for cms service
    pub const fn new(content_repository: ContentRepository, visitor_log_repository: VisitorLogRepository) -> Result<Self> {
        Ok(Self { content_repository, visitor_log_repository })
    }
}

impl CmsService {

    /// sent contact form
    pub async fn sent_contact_form(
        &self,
        template: &AvoRedTemplateProvider,
        request: SentContactFormRequest,
    ) -> Result<SentContactFormResponse> {
        let from_address = std::env::var("AVORED_CONTACT_FROM_EMAIL")
            .unwrap_or_else(|_| "info@avored.com".to_string());
        let to_address = std::env::var("AVORED_CONTACT_TO_EMAIL")
            .unwrap_or_else(|_| "admin@avored.com".to_string());
        let email_subject = std::env::var("AVORED_CONTACT_EMAIL_SUBJECT")
            .unwrap_or_else(|_| "Contact us message".to_string());

        let payload = SentContactUsEmailRequest {
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            message: request.message,
            phone: request.phone,
        };

        let sent_contact_email_message_body =
            template.handlebars.render("contact-us-email", &payload)?;

        let email_message = Message::builder().build_email_message(
            &from_address,
            &to_address,
            &email_subject,
            sent_contact_email_message_body,
        )?;

        // Send the email
        match template.mailer.send(email_message).await {
            Ok(_) => {
                let response = SentContactFormResponse { status: true };

                Ok(response)
            }
            Err(err) => {
                error!("there is an issue with sending an email via smtp: {err:?}");
                Err(Error::Generic(String::from("error while sending an email")))
            }
        }
    }

    /// get cms content
    pub async fn get_cms_content(
        &self,
        request: GetCmsContentRequest,
        remote_addr: Option<SocketAddr>,
        (datastore, database_session): &DB,
    ) -> Result<GetCmsContentResponse> {

        let content_model = self
            .content_repository
            .find_by_identifier(
                datastore,
                database_session,
                &request.content_type,
                &request.content_identifier,
            )
            .await?;
        let grpc_model: ContentModel = content_model.try_into()?;

        let ip_address = match remote_addr {
            Some(ip) => ip.ip().to_string(),
            None => String::from("")
        };

        let creatable_visitor_log = CreatableVisitorLogModel {
            content_type: request.content_type.clone(),
            content_id: grpc_model.id.clone(),
            ip_address: Some(ip_address)
        };

        self.visitor_log_repository.create(
            datastore,
            database_session,
            creatable_visitor_log
        ).await?;

        let response = GetCmsContentResponse {
            status: true,
            data: Some(grpc_model),
        };

        Ok(response)
    }
}
/// sent contact us email request
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SentContactUsEmailRequest {
    /// email
    pub email: String,
    /// first name
    pub first_name: String,
    /// last name
    pub last_name: String,
    /// message
    pub message: String,
    /// phone
    pub phone: String,
}
