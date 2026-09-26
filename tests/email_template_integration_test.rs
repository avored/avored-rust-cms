use avored_rust_cms::core::application::dtos::email_template_dto::CreateEmailTemplateCommand;

#[tokio::test]
async fn test_create_email_template_command_requires_name_and_subject() {
    let command = CreateEmailTemplateCommand {
        name: "".to_string(),
        subject: "".to_string(),
        body_html: Some("<h1>Hello</h1>".to_string()),
        body_plain: Some("Hello".to_string()),
    };

    let result = command.validate("en").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_email_template_command_accepts_valid_payload() {
    let command = CreateEmailTemplateCommand {
        name: "Welcome Campaign".to_string(),
        subject: "Welcome to our store".to_string(),
        body_html: Some("<h1>Hello {{ user.first_name }}</h1>".to_string()),
        body_plain: Some("Hello {{ user.first_name }}".to_string()),
    };

    let result = command.validate("en").await;
    assert!(result.is_ok());
}
