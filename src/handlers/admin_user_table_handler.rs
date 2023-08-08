use std::sync::Arc;
use std::vec;

use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};

use crate::PER_PAGE;
use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::{AdminUser, AdminUserPaginate};
use crate::providers::avored_session_provider::AvoRedSession;

#[derive(Deserialize, Debug)]
pub struct AdminUsersRequest {
    pub current_page: Option<u64>,
    pub per_page: Option<u64>,
}

pub async fn admin_user_table_handler(
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<AdminUsersRequest>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    println!(" QUERY : {:?}", query_param);
    let current_page: i64 = match query_param.current_page {
        Some(current_page) => current_page.try_into().unwrap(),
        None => 1,
    };
    let per_page: i64 = match query_param.per_page {
        Some(per_page) => per_page.try_into().unwrap(),
        None => PER_PAGE.try_into().unwrap(),
    };
    let from = ((current_page - 1) * per_page) + 1;
    let to = from + per_page - 1;
    let start = from - 1;

    println!("{}", current_page);
    // let mut previous_page: i64 = 0;
   

    let admin_users = state
        .admin_user_repository
        .paginate(&state.datastore, &state.database_session, start)
        .await;

    let admin_users = match admin_users {
        Ok(data) => data,
        Err(_) => panic!("no data found error"),
    };

    let mut admin_user_paginate = match state
        .admin_user_repository
        .no_of_record(&state.datastore, &state.database_session)
        .await
    {
        Ok(count) => count,
        Err(_) => AdminUserPaginate::empty_admin_user_paginate(),
    };

    admin_user_paginate.from = from;
    admin_user_paginate.to = to;
    

    if current_page > 1 {
        println!("i am inside prev page");
        admin_user_paginate.has_previous_page = true;
        admin_user_paginate.previous_page = current_page - 1;
    }

    if to < admin_user_paginate.count {
        admin_user_paginate.has_next_page = true;
        let next_page  = current_page + 1;
        println!("i am inside next page {}", (current_page + 1));
        admin_user_paginate.next_page= next_page;
    }

    let mut view_model = AdminUserTableHandlerViewModel::new();
    view_model.admin_users = admin_users;

    view_model.logged_in_user = logged_in_user;
    view_model.admin_user_paginate = admin_user_paginate;

    println!("{:?} from: {} to: {} current_page: {}, per_page {}", view_model.admin_user_paginate, from, to, current_page, per_page);

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/admin-user-table", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}

#[derive(Serialize)]
pub struct AdminUserTableHandlerViewModel {
    logged_in_user: AdminUser,
    admin_users: Vec<AdminUser>,
    admin_user_paginate: AdminUserPaginate,
}

impl AdminUserTableHandlerViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        AdminUserTableHandlerViewModel {
            logged_in_user,
            admin_users: vec![],
            admin_user_paginate: AdminUserPaginate::empty_admin_user_paginate(),
        }
    }
}
