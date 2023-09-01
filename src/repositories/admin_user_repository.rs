use std::collections::BTreeMap;

use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Array, Object};

use crate::error::Result;
use crate::models::admin_user_model::{
    AdminUser, AdminUserPaginate, CreatableAdminUser, UpdatableAdminUser,
};
use crate::models::{W, ModelCount};
use crate::PER_PAGE;

pub struct AdminUserRepository {}

impl AdminUserRepository {
    pub fn new() -> AdminUserRepository {
        AdminUserRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
    ) -> Result<Vec<AdminUser>> {
        let sql = "SELECT * FROM admin_users LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), false)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response.result.expect(
            "there is an issue with receiving the response result of surreal db query response",
        );

        let array: Array = W(result)
            .try_into()
            .expect("there is an issue while converting query result into an array");
        let objects: Result<Vec<Object>> =
            array.into_iter().map(|value| W(value).try_into()).collect();
        let objects = match objects {
            Ok(obj) => obj,
            Err(_) => {
                let objects: Vec<Object> = vec![];

                objects
            }
        };

        let result_admin_users: Result<Vec<AdminUser>> =
            objects.into_iter().map(|o| o.try_into()).collect();

        let admin_users = match result_admin_users {
            Ok(data) => data,
            Err(_) => panic!("issue while converting an vector of objects into admin_user struct"),
        };

        Ok(admin_users)
    }

    pub async fn no_of_record(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<AdminUserPaginate> {
        let sql = "SELECT count() FROM admin_users GROUP ALL;";

        let responses = match datastore.execute(sql, &database_session, None, false).await {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();

        let result_admin_users_count: Result<Object> = W(result).try_into();

        let admin_users_count: Result<AdminUserPaginate> = match result_admin_users_count {
            Ok(data) => data.try_into(),
            Err(_) => Ok(AdminUserPaginate::empty_admin_user_paginate()),
        };

        admin_users_count
    }

    pub async fn find_by_email(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: String,
    ) -> Result<AdminUser> {
        let sql = "SELECT * FROM admin_users where email=$email;";
        let vars = BTreeMap::from([("email".into(), email.into())]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), false)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();

        let result_admin_users: Result<Object> = W(result).try_into();

        let admin_user: Result<AdminUser> = match result_admin_users {
            Ok(data) => data.try_into(),
            Err(_) => Ok(AdminUser::empty_admin_user()),
        };

        admin_user
    }

    pub async fn has_email_address_taken(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: String,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM admin_users WHERE email=$email;";
        let vars = BTreeMap::from([("email".into(), email.into())]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), true)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();
        println!("Result: {:?}", result);
        let result_object: Result<Object> = W(result).try_into();

        let admin_users_count: Result<ModelCount> = match result_object {
            Ok(data) => data.try_into(),
            Err(_) => Ok(ModelCount::new()),
        };

        // println!("{:?}", admin_users_count);

        admin_users_count
    }

    pub async fn update_admin_user(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatableAdminUser,
    ) -> Result<AdminUser> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                full_name: $full_name,
                profile_image: $profile_image,
                is_super_admin: $is_super_admin,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("full_name".into(), updatable_admin_user.full_name.into()),
            (
                "logged_in_user_name".into(),
                updatable_admin_user.logged_in_username.into(),
            ),
            (
                "profile_image".into(),
                updatable_admin_user.profile_image.into(),
            ),
            (
                "is_super_admin".into(),
                updatable_admin_user.is_super_admin.into(),
            ),
            ("id".into(), updatable_admin_user.id.into()),
            ("table".into(), "admin_users".into()),
        ]);

        let responses = match datastore
            .execute(sql, database_session, Some(vars), false)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();

        let result_admin_users: Result<Object> = W(result).try_into();

        let admin_user: Result<AdminUser> = match result_admin_users {
            Ok(data) => data.try_into(),
            Err(_) => Ok(AdminUser::empty_admin_user()),
        };

        admin_user
    }
    
    pub async fn create_admin_user(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_admin_user: CreatableAdminUser,
    ) -> Result<AdminUser> {
        let sql = "
            CREATE admin_users CONTENT {
                full_name: $full_name,
                email: $email,
                password: $password,
                profile_image: $profile_image,
                is_super_admin: $is_super_admin,
                created_by: $logged_in_user_name,
                updated_by: $logged_in_user_name,
                created_at: time::now(),
                updated_at: time::now()
            };
        ";

        let vars = BTreeMap::from([
            ("full_name".into(), creatable_admin_user.full_name.into()),
            ("email".into(), creatable_admin_user.email.into()),
            ("password".into(), creatable_admin_user.password.into()),
            (
                "profile_image".into(),
                creatable_admin_user.profile_image.into(),
            ),
            (
                "is_super_admin".into(),
                creatable_admin_user.is_super_admin.into(),
            ),
            (
                "logged_in_user_name".into(),
                creatable_admin_user.logged_in_username.into(),
            ),
        ]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), false)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();

        let result_admin_users: Result<Object> = W(result).try_into();

        let admin_user: Result<AdminUser> = match result_admin_users {
            Ok(data) => data.try_into(),
            Err(_) => Ok(AdminUser::empty_admin_user()),
        };

        admin_user
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: String,
    ) -> Result<AdminUser> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars = BTreeMap::from([
            ("table".into(), "admin_users".into()),
            ("id".into(), id.into()),
        ]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), false)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();

        let result_admin_users: Result<Object> = W(result).try_into();

        let admin_user: Result<AdminUser> = match result_admin_users {
            Ok(data) => data.try_into(),
            Err(_) => Ok(AdminUser::empty_admin_user()),
        };

        admin_user
    }
}
