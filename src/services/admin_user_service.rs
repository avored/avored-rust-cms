use std::collections::BTreeMap;

use surrealdb::{
    dbs::Response,
    sql::{Datetime, Object, Value},
};

use crate::{
    error::{Error, Result},
    models::admin_user_model::{AdminUserModel, CreatableAdminUser, UpdatableAdminUserModel},
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

    //     // println!("{:?}", responses);
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
        start: i64,
    ) -> Result<Vec<AdminUserModel>> {
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
        Ok(admin_user_list)
    }

    pub async fn create_admin_user(
        &self,
        (ds, ses): &DB,
        creatable_admin_user_model: CreatableAdminUser,
    ) -> Result<String> {
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

        into_iter_objects(ress)?
            .next()
            .transpose()?
            .and_then(|obj| obj.get("id").map(|id| id.to_string()))
            .ok_or_else(|| Error::Generic("no record"))
    }
}

fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let response = responses
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()?;

    // println!("{:?}", response);

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
