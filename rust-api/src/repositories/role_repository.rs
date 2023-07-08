use entity::{admin_users, admin_users_roles, roles};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, LoaderTrait, PaginatorTrait,
    QueryFilter, Set,
};

use crate::responses::roless_paginate_response::RolessPaginateResponse;

pub struct RoleRepository {}

impl RoleRepository {
    pub fn new() -> RoleRepository {
        RoleRepository { }
    }

    pub async fn paginate(&self, conn: sea_orm::DatabaseConnection, per_page: u64, current_page: u64) -> RolessPaginateResponse {
        let roles_paginate_list = roles::Entity::find()
            .paginate(&conn, per_page)
            .fetch_page(current_page)
            .await
            .unwrap();

        RolessPaginateResponse::transform_into_response(roles_paginate_list)
    }
}
