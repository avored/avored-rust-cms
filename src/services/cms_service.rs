use crate::error::Result;
use crate::api::proto::cms::{GetCmsContentRequest, GetCmsContentResponse};
use crate::api::proto::content::ContentModel;
use crate::providers::avored_database_provider::DB;
use crate::repositories::content_repository::ContentRepository;

pub struct CmsService {
    content_repository: ContentRepository,
}

impl CmsService {
    pub fn new(content_repository: ContentRepository) -> Result<Self> {
        Ok(CmsService {
            content_repository
        })
    }
}

impl CmsService {
    // pub async fn sent_contact_us_email(
    //     &self,
    //     template: &AvoRedTemplateProvider,
    //     payload: SentContactUsEmailRequest,
    // ) -> Result<bool> {
    //     let from_address = String::from("info@avored.com");
    //     let to_address = String::from("ind.purvesh@gmail.com");
    //     let email_subject = String::from("Contact us message");
    //     let sent_contact_email_message_body =
    //         template.handlebars.render("contact-us-email", &payload)?;
    // 
    //     let email = Message::builder()
    //         .from(from_address.parse()?)
    //         .to(to_address.parse()?)
    //         .subject(email_subject)
    //         .multipart(
    //             MultiPart::alternative().singlepart(
    //                 SinglePart::builder()
    //                     .header(header::ContentType::TEXT_HTML)
    //                     .body(sent_contact_email_message_body),
    //             ),
    //         )?;
    // 
    //     // Send the email
    //     match template.mailer.send(email).await {
    //         Ok(_) => Ok(true),
    //         Err(err) => {
    //             error!("there is an issue with sending an email via smtp: {err:?}");
    //             Err(Error::Generic(String::from("error while sending an email")))
    //         }
    //     }
    // }

    pub async fn get_cms_content(
        &self,
        request: GetCmsContentRequest,
        (datastore, database_session): &DB
    ) -> Result<GetCmsContentResponse> {

        let content_model = self
            .content_repository
            .find_by_id(datastore, database_session, &request.content_type, &request.content_id)
            .await?;
        let grpc_model: ContentModel = content_model.try_into()?;
        
        let response = GetCmsContentResponse {
            status: true,
            data: Some(grpc_model),
        };

        Ok(response)
    }
}
