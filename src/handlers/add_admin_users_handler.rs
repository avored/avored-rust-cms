use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::ReadableSession;
use entity::roles;
use serde_derive::Serialize;
use std::sync::Arc;

use crate::{
    repositories::admin_user_repository::AdminUser,
    routes::{establish_connection, AppState},
};

pub async fn add_admin_users_handler(
    app_state: State<Arc<AppState>>,
    session: ReadableSession,
) -> impl IntoResponse {
    let handlebars = &app_state.handlebars;

    let logged_in_user: AdminUser = session.get("logged_in_user").unwrap();
   
    let connection = establish_connection().await;
    let role_options = match app_state.role_repository.all(connection).await {
        Ok(roles) => roles,
        Err(_) => {
            println!("there is an issue while fetching all roles for create admin user");
            Vec::new()
        }
    };

     let data: AddAdminUserViewModel = AddAdminUserViewModel {
        logged_in_user,
        role_options,
    };


    let html = handlebars
        .render("admin-users/add-admin-user", &data)
        .unwrap();

    Html(html).into_response()
}

#[derive(Serialize)]
struct AddAdminUserViewModel {
    logged_in_user: AdminUser,
    role_options: Vec<roles::Model>,
}
