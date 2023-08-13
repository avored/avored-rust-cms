use std::collections::HashMap;
use std::sync::Arc;
use axum::Form;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect};
use serde::Deserialize;
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::store_component_request::StoreComponentRequest;

pub async fn store_component_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
    Form(payload): Form<StoreComponentRequest>,
) -> impl IntoResponse {
    // let logged_in_user = match session.get("logged_in_user") {
    //     Some(logged_in_user) => logged_in_user,
    //     None => AdminUser::empty_admin_user(),
    // };
   
    // let mut view_model = CreatePageHandlerViewModel::new();

    println!("Payload: {:?}", payload.test);
    
    let json: HashMap<String, FieldType>= serde_json::from_str(&payload.test).unwrap();
    
    println!("JSON: {:?}", json);
    // view_model.logged_in_user = logged_in_user;

    // let handlebars = &state.handlebars;

    // let html = handlebars
    //     .render("component/create-component", &view_model)
    //     .expect("there is an issue while loading the admin template");

    // Html(html).into_response()

    Redirect::to("/admin/create-component").into_response()
    // Json(admin_users).into_response()
}

#[derive(Deserialize, Debug, Clone)]
pub struct FieldType {
    pub field_name: String,
    pub field_type: String,
    pub field_value: String,
}
