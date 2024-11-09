use lettre::{AsyncTransport, Message};
use lettre::message::{header, MultiPart, SinglePart};
use tracing::error;
use crate::api::handlers::cms::sent_contact_us_email_handler::SentContactUsEmailRequest;
use crate::error::Result;
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::error::Error;

pub struct CmsService {}


impl CmsService {
    pub fn new() -> Result<Self> {
        Ok(CmsService { })
    }
}

impl CmsService {
    pub async fn sent_contact_us_email (
        &self,
        template: &AvoRedTemplateProvider,
        payload: SentContactUsEmailRequest
    ) -> Result<bool> {
        let from_address = String::from("info@avored.com");
        let to_address = String::from("ind.purvesh@gmail.com");
        let email_subject = String::from("Contact us message sent");
        let sent_contact_email_message_body = template.handlebars.render("contact-us-email", &payload)?;

        let email = Message::builder()
            .from(from_address.parse()?)
            .to(to_address.parse()?)
            .subject(email_subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(sent_contact_email_message_body),
                    ),
            )?;

        // Send the email
        match template.mailer.send(email).await {
            Ok(_) => Ok(true),
            Err(err) =>  {
                error!("there is an issue with sending an email via smtp: {err:?}");
                Err(Error::Generic(String::from("error while sending an email")))
            },
        }
    }
}