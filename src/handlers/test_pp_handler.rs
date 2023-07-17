use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};

use serde::Deserialize;
use serde_json::json;
use std::{fmt::Debug, sync::Arc};
use validator::Validate;

use crate::routes::AppState;

pub async fn test_pp_handler(
    app_state: State<Arc<AppState>>,
    Form(input): Form<NameInput>,
) -> impl IntoResponse {
    let validation_result = input.validate();
    if validation_result.is_err() {
        // @todo flash message here
        return Redirect::to("/admin/login").into_response();
    }

    let data = json!({});

    let handlebars = &app_state.handlebars;

    let html = handlebars
        .render("test-pp", &data)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct NameInput {
    #[validate(length(min = 5, message = "minimum 5 character is required"))]
    pub name: String,
}
