use std::sync::Arc;
use std::vec;

use axum::extract::State;
use axum::response::{IntoResponse, Html};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn admin_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession
) -> impl IntoResponse {
    let counter = match session.get("counter") {
        Some(count) => count,
        None => 0
    };

    println!("{counter}");

    session.insert("counter", counter + 1)
        .expect("cant store counter into session");
    // let datastore = &state.datastore;
    // let database_session = &state.database_session;

    // let sql = "DELETE admin_users where name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let sql = "CREATE admin_users SET name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };
    // let sql = "SELECT * FROM admin_users;";

    // let responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let response = responses
    //     .into_iter()
    //     .next()
    //     .expect("error while retriving the first response");

    // let result = response.result.expect("first result comes with error");

    // let array: Array = W(result).try_into().expect("sdfds");
    // let objects: Result<Vec<Object>> = array.into_iter().map(|value| W(value).try_into()).collect();
    // let objects = match objects {
    //     Ok(obj) => obj,
    //     Err(_) => panic!("no data"),
    // };

    // let test: Result<Vec<AdminUser>> = objects.into_iter().map(|o| o.try_into()).collect();

    // let admin_users = match test {
    //     Ok(data) => data,
    //     Err(_) => panic!("some errror"),
    // };

    let admin_users = state
        .admin_user_repository
        .paginate(&state.datastore, &state.database_session)
        .await;

     let admin_users = match admin_users {
        Ok(data) => data,
        Err(_) => panic!("no data found error"),
    };

    let mut view_model = GetAdminHandlerViewModel::new();
    view_model.admin_users = admin_users;

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("dashboard", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}


#[derive(Serialize)]
pub struct GetAdminHandlerViewModel {
    validation_email_message: String,
    validation_password_message: String,
    admin_users: Vec<AdminUser>
}

impl GetAdminHandlerViewModel {
    fn new() -> Self {
        GetAdminHandlerViewModel {
            validation_email_message: String::from(""),
            validation_password_message: String::from(""),
            admin_users: vec![],
        }
    }
}
