use lettre::message::{header, MessageBuilder, MultiPart, SinglePart};
use lettre::Message;


/// Trait for building email messages
pub trait EmailMessageBuilder {
    /// Builds an email message with the provided parameters.
    fn build_email_message(
        &self,
        from_address: &str,
        to_address: &str,
        email_subject: &str,
        forgot_password_email_content: String,
    ) -> crate::error::Result<Message>;
}

impl EmailMessageBuilder for MessageBuilder {
    fn build_email_message(
        &self,
        from_address: &str,
        to_address: &str,
        email_subject: &str,
        forgot_password_email_content: String,
    ) -> crate::error::Result<Message> {
        let message = Message::builder()
            .from(from_address.parse()?)
            .to(to_address.parse()?)
            .subject(email_subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(forgot_password_email_content),
                ),
            )?;
        Ok(message)
    }
}
