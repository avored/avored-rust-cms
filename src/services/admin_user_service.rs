use std::collections::BTreeMap;

use surrealdb::{
    dbs::Response,
    sql::{Datetime, Object, Value},
};

use crate::{
    error::{Error, Result},
    models::{
        admin_user_model::{
            AdminUserModel, AdminUserPagination, CreatableAdminUser, UpdatableAdminUserModel,
        },
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::admin_user_repository::AdminUserRepository,
    PER_PAGE,
};

pub struct AdminUserService {
    admin_user_repository: AdminUserRepository,
}

impl AdminUserService {
    pub fn new(admin_user_repository: AdminUserRepository) -> Result<Self> {
        Ok(AdminUserService {
            admin_user_repository,
        })
    }
}
impl AdminUserService {
    // pub async fn all_admin_users(
    //     &self,
    //     (datastore, database_session): &DB,
    // ) -> Result<Vec<AdminUserModel>> {
    //     let sql = "SELECT * FROM admin_users";

    //     let responses = datastore.execute(sql, database_session, None).await?;

    //     let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

    //     for object in into_iter_objects(responses)? {
    //         let admin_user_object = object?;

    //         let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
    //         admin_user_list.push(admin_user_model?);
    //     }
    //     Ok(admin_user_list)
    // }

    pub async fn find_by_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_email(datastore, database_session, email)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn update_admin_user(
        &self,
        (datastore, database_session): &DB,
        updateable_admin_user_model: UpdatableAdminUserModel,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .update_admin_user(datastore, database_session, updateable_admin_user_model)
            .await
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
    ) -> Result<AdminUserPagination> {
        let start = (current_page - 1) * PER_PAGE;
        let to = start + PER_PAGE;

        let admin_user_count = self
            .admin_user_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if admin_user_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: admin_user_count.total,
            per_page: PER_PAGE,
            current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let sql = "SELECT * FROM admin_users LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let admin_user_object = object?;

            let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
            admin_user_list.push(admin_user_model?);
        }

        let admin_user_paginate = AdminUserPagination {
            data: admin_user_list,
            pagination,
        };

        Ok(admin_user_paginate)
    }

    // pub async fn delete_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     admin_user_id: String,
    // ) -> Result<bool> {
    //     self.admin_user_repository
    //         .delete_admin_user(datastore, database_session, admin_user_id)
    //         .await
    // }

    pub async fn create_admin_user(
        &self,
        (ds, ses): &DB,
        creatable_admin_user_model: CreatableAdminUser,
    ) -> Result<AdminUserModel> {
        let sql = "CREATE admin_users CONTENT $data";

        let data: BTreeMap<String, Value> = [
            (
                "full_name".into(),
                creatable_admin_user_model.full_name.into(),
            ),
            ("email".into(), creatable_admin_user_model.email.into()),
            (
                "password".into(),
                creatable_admin_user_model.password.into(),
            ),
            (
                "profile_image".into(),
                creatable_admin_user_model.profile_image.into(),
            ),
            (
                "is_super_admin".into(),
                creatable_admin_user_model.is_super_admin.into(),
            ),
            (
                "created_by".into(),
                creatable_admin_user_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_admin_user_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = ds.execute(sql, ses, Some(vars)).await?;

        let result_object_option = into_iter_objects(ress)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

        admin_user_model
    }
}

fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let response = responses
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()?;

    match response {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(Error::Generic("empty object")),
            });

            Ok(it)
        }
        _ => Err(Error::Generic("No Record found")),
    }
}
