use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

use crate::error::{Error, Result};
use crate::models::asset_model::{AssetModel, CreatableAssetModel};
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
        parent_id: String,
    ) -> Result<Vec<AssetModel>> {
        let sql = "SELECT * FROM type::table($table) WHERE parent_id=$parent_id LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("table".into(), ASSET_TABLE.into()),
            ("parent_id".into(), parent_id.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut asset_list: Vec<AssetModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let asset_object = object?;

            let mut asset_model: AssetModel = asset_object.try_into()?;

            let mut new_path = asset_model.name.clone();
            let mut t = asset_model.clone();

            // Not sure do we need to add some kind a loop limit to prevent infinite loop??
            while !t.parent_id.is_empty() {
                let parent_model = self
                    .find_by_id(datastore, database_session, &t.parent_id)
                    .await?;
                t = parent_model.clone();
                new_path = format!("{}/{}", parent_model.name, new_path);
            }

            new_path = format!("/{}/{}", "public/upload", new_path);
            asset_model.new_path = new_path;

            asset_list.push(asset_model);
        }
        Ok(asset_list)
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        parent_id: String,
    ) -> Result<ModelCount> {
        let sql = format!(
            "SELECT count() FROM assets WHERE parent_id = '{}' GROUP ALL;",
            parent_id
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        match result_object {
            Ok(obj) => obj.try_into(),
            Err(_) => Ok(ModelCount::default()),
        }
    }

    pub async fn create_asset(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_asset_model: CreatableAssetModel,
    ) -> Result<AssetModel> {
        let sql = "CREATE assets CONTENT $data";
        let meta = creatable_asset_model.metadata.get_file_metadata();
        let metadata: BTreeMap<String, Value> =
            [("file_type".into(), meta.file_type.into())].into();

        let data: BTreeMap<String, Value> = [
            ("name".into(), creatable_asset_model.name.into()),
            ("parent_id".into(), creatable_asset_model.parent_id.into()),
            ("asset_type".into(), creatable_asset_model.asset_type.into()),
            ("metadata".into(), metadata.into()),
            (
                "created_by".into(),
                creatable_asset_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_asset_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::CreateModel(
                "cannot create assets record".to_string(),
            )),
        };
        let asset_model: Result<AssetModel> = result_object?.try_into();

        asset_model
    }

    pub async fn create_asset_folder(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_asset_model: CreatableAssetModel,
    ) -> Result<AssetModel> {
        let sql = "CREATE assets CONTENT $data";
        let meta = creatable_asset_model.metadata.get_folder_metadata();
        let metadata: BTreeMap<String, Value> = [("color".into(), meta.color.into())].into();

        let data: BTreeMap<String, Value> = [
            ("name".into(), creatable_asset_model.name.into()),
            ("asset_type".into(), creatable_asset_model.asset_type.into()),
            ("parent_id".into(), creatable_asset_model.parent_id.into()),
            ("metadata".into(), metadata.into()),
            (
                "created_by".into(),
                creatable_asset_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_asset_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::CreateModel(
                "cannot create assets record".to_string(),
            )),
        };
        let asset_model: Result<AssetModel> = result_object?.try_into();

        asset_model
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        asset_id: &str,
    ) -> Result<AssetModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), asset_id.into()),
            ("table".into(), ASSET_TABLE.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let asset_object = result_object?;

        let mut asset_model: AssetModel = asset_object.try_into()?;

        let mut new_path = asset_model.name.clone();
        let mut t = asset_model.clone();

        // Not sure do we need to add some kind a loop limit to prevent infinite loop??
        while !t.parent_id.is_empty() {
            let parent_model =
                Box::pin(self.find_by_id(datastore, database_session, &t.parent_id)).await?;

            t = parent_model.clone();
            new_path = format!("{}/{}", parent_model.name, new_path);
        }

        new_path = format!("/{}/{}", "public/upload", new_path);
        asset_model.new_path = new_path;

        Ok(asset_model)
    }

    pub async fn delete_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        asset_id: &str,
    ) -> Result<bool> {
        let sql = "DELETE type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), asset_id.into()),
            ("table".into(), ASSET_TABLE.into()),
        ]
        .into();

        // find a way to get a response from query
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let response = responses.into_iter().next().map(|rp| rp.output());
        let query_result = match response {
            Some(object) => object.is_ok(),
            None => false,
        };

        Ok(query_result)
    }

    pub async fn update_asset_path(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        name: &str,
        asset_id: &str,
        logged_in_username: &str,
    ) -> Result<AssetModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                path: $path,
                name: $name,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("name".into(), name.into()),
            ("logged_in_user_name".into(), logged_in_username.into()),
            ("id".into(), asset_id.into()),
            ("table".into(), ASSET_TABLE.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let asset_model: Result<AssetModel> = result_object?.try_into();

        asset_model
    }
}
