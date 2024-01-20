use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

use crate::error::{Error, Result};
use crate::models::asset_model::{CreatableAssetModel, AssetModel};
use crate::models::ModelCount;
use crate::PER_PAGE;

use super::into_iter_objects;
const ASSET_TABLE: &str = "assets";

#[derive(Clone)]
pub struct AssetRepository {}

impl AssetRepository {
    pub fn new() -> Self {
        AssetRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
    ) -> Result<Vec<AssetModel>> {
        let sql = "SELECT * FROM type::table($table) LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("table".into(), ASSET_TABLE.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut asset_list: Vec<AssetModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let asset_object = object?;

            let asset_model: Result<AssetModel> = asset_object.try_into();
            asset_list.push(asset_model?);
        }
        Ok(asset_list)
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM assets GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let total_count = match result_object {
            Ok(obj) => obj.try_into(),
            Err(_) => Ok(ModelCount::default()),
        };

        total_count
    }
    //
    // pub async fn find_by_id(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     asset_id: String,
    // ) -> Result<AssetModel> {
    //     let sql =
    //         "SELECT * FROM type::thing($table, $id);";
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), asset_id.into()),
    //         ("table".into(), "assets".into()),
    //     ]
    //         .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     // println!("RESULT_OBJECT: {result_object:?}");
    //     let asset_model: Result<AssetModel> = result_object?.try_into();
    //
    //     asset_model
    // }

    pub async fn create_asset(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_asset_model: CreatableAssetModel,
    ) -> Result<AssetModel> {

        println!("REPO MODEL: {creatable_asset_model:?}");

        let sql = "CREATE assets CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("file_name".into(), creatable_asset_model.file_name.into()),
            ("file_path".into(), creatable_asset_model.file_path.into(),),
            ("file_size".into(), creatable_asset_model.file_size.into()),
            ("created_by".into(), creatable_asset_model.logged_in_username.clone().into(),),
            ("file_size".into(), creatable_asset_model.file_size.into()),
            ("file_type".into(), creatable_asset_model.file_type.into()),
            ("information".into(), creatable_asset_model.information.into()),
            ("updated_by".into(), creatable_asset_model.logged_in_username.into(),),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
            .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::CreateModelError("cannot create assets record")),
        };
        let asset_model: Result<AssetModel> = result_object?.try_into();

        asset_model
    }

    // pub async fn update_asset(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     updatable_admin_user: UpdatableAssetModel,
    // ) -> Result<AssetModel> {
    //     let sql = "
    //         UPDATE type::thing($table, $id) MERGE {
    //             name: $name,
    //             identifier: $identifier,
    //             content: $content,
    //             updated_by: $logged_in_user_name,
    //             updated_at: time::now()
    //         };";
    //
    //     let vars = BTreeMap::from([
    //         ("name".into(), updatable_admin_user.name.into()),
    //         ("identifier".into(), updatable_admin_user.identifier.into()),
    //         ("content".into(), updatable_admin_user.content.into()),
    //         (
    //             "logged_in_user_name".into(),
    //             updatable_admin_user.logged_in_username.into(),
    //         ),
    //         ("id".into(), updatable_admin_user.id.into()),
    //         ("table".into(), "assets".into()),
    //     ]);
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let asset_model: Result<AssetModel> = result_object?.try_into();
    //
    //     asset_model
    // }

}
