use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use handlebars::Handlebars;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};


/// avored template provider
pub struct AvoRedTemplateProvider {

    /// handlebar
    pub handlebars: Handlebars<'static>,

    /// async smtp transport
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl AvoRedTemplateProvider {

    /// register avored template provider
    pub async fn register(config: AvoRedConfigProvider) -> Result<Self> {
        let mut reg = Handlebars::new();
        reg.register_template_file("forgot-password", "./resources/mail/forgot-password.hbs")?;
        reg.register_template_file("contact-us-email", "./resources/mail/contact-us-email.hbs")?;

        let creds = Credentials::new(config.smtp_username, config.smtp_password);

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
                .unwrap()
                .port(config.smtp_port)
                .credentials(creds)
                .build();

        Ok(Self {
            handlebars: reg,
            mailer,
        })
    }
}
