use std::sync::Arc;
use std::vec;

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::{AdminUser, AdminUserPaginate};
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn admin_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    // let datastore = &state.datastore;
    // let database_session = &state.database_session;

    // let sql = "DELETE admin_users where full_name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let sql = "CREATE admin_users SET full_name = 'Purvesh';";

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

    let admin_user_paginate  = match state
        .admin_user_repository
        .no_of_record(&state.datastore, &state.database_session)
        .await
    {
        Ok(count) => count,
        Err(_) => AdminUserPaginate::empty_admin_user_paginate(),
    };

    let admin_users = state
        .admin_user_repository
        .paginate(&state.datastore, &state.database_session, 0)
        .await;

    let admin_users = match admin_users {
        Ok(data) => data,
        Err(_) => panic!("no data found error"),
    };

    let mut view_model = AdminHandlerViewModel::new();
    view_model.admin_users = admin_users;

    view_model.logged_in_user = logged_in_user;
    view_model.admin_user_paginate = admin_user_paginate;

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("dashboard", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}

#[derive(Serialize)]
pub struct AdminHandlerViewModel {
    logged_in_user: AdminUser,
    validation_email_message: String,
    validation_password_message: String,
    admin_users: Vec<AdminUser>,
    admin_user_paginate: AdminUserPaginate
}

impl AdminHandlerViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        AdminHandlerViewModel {
            logged_in_user,
            validation_email_message: String::from(""),
            validation_password_message: String::from(""),
            admin_users: vec![],
            admin_user_paginate: AdminUserPaginate::empty_admin_user_paginate()
        }
    }
}
