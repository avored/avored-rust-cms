use std::sync::Arc;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::{Deserialize, Serialize};

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn create_component_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let mut view_model = CreatePageHandlerViewModel::new();
    view_model.logged_in_user = logged_in_user;
    let handlebars = &state.handlebars;

    let html = handlebars
        .render("component/create-component", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Address {
    postcode: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct QueryTest {
    address: Vec<Address>,
}

#[derive(Serialize)]
pub struct CreatePageHandlerViewModel {
    logged_in_user: AdminUser,
    test: String
}

impl CreatePageHandlerViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        CreatePageHandlerViewModel {
            logged_in_user,
            test: String::from("TEST VALUE")
        }
    }
}
