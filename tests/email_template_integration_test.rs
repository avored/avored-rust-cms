#![recursion_limit = "512"]

use avored_rust_cms::core::application::dtos::email_template_dto::CreateEmailTemplateCommand;
use avored_rust_cms::avored_state::test_avored_state;

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

#[tokio::test]
async fn test_email_template_repository_find_by_id_returns_record() {
    let state = test_avored_state().await;
    let created = state
        .email_template_use_case
        .create(CreateEmailTemplateCommand {
            name: "Welcome Campaign".to_string(),
            subject: "Welcome to our store".to_string(),
            body_html: Some("<h1>Hello</h1>".to_string()),
            body_plain: Some("Hello".to_string()),
        })
        .await
        .expect("create email template should succeed");

    let found = state
        .email_template_use_case
        .get_by_id(&created.id)
        .await
        .expect("fetch by id should succeed");

    assert_eq!(found.name, "Welcome Campaign");
    assert_eq!(found.subject, "Welcome to our store");
}

#[tokio::test]
async fn test_email_template_repository_find_by_id_returns_not_found() {
    let state = test_avored_state().await;

    let result = state.email_template_use_case.get_by_id("email_templates:missing").await;
    assert!(result.is_err());
}
