use std::path::Path;
use crate::{error::Result, PER_PAGE, providers::avored_database_provider::DB, repositories::asset_repository::AssetRepository};
use crate::models::asset_model::{AssetPagination, CreatableAssetModelNew, MetaDataType, NewAssetModel};
use crate::models::Pagination;
use crate::models::token_claim_model::LoggedInUser;

pub struct AssetService {
    asset_repository: AssetRepository,
}

impl AssetService {
    pub fn new(asset_repository: AssetRepository) -> Result<Self> {
        Ok(AssetService { asset_repository })
    }
}
impl AssetService {
    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
    ) -> Result<AssetPagination> {
        let start = (current_page - 1) * PER_PAGE;
        let to = start + PER_PAGE;

        let asset_model_count = self
            .asset_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if asset_model_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: asset_model_count.total,
            per_page: PER_PAGE,
            current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let assets = self
            .asset_repository
            .paginate(datastore, database_session, start)
            .await?;

        Ok(AssetPagination {
            data: assets,
            pagination,
        })
    }

    // pub async fn find_by_id(
    //     &self,
    //     (datastore, database_session): &DB,
    //     id: String,
    // ) -> Result<AssetModel> {
    //     self.asset_repository
    //         .find_by_id(datastore, database_session, id)
    //         .await
    // }

    pub async fn create_asset(
        &self,
        (datastore, database_session): &DB,
        creatable_asset_model: CreatableAssetModelNew,
    ) -> Result<NewAssetModel> {
        self.asset_repository
            .create_asset(datastore, database_session, creatable_asset_model)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        asset_id: &str
    ) -> Result<NewAssetModel> {
        self.asset_repository
            .find_by_id(datastore, database_session, asset_id)
            .await
    }

    pub async fn delete_by_id(
        &self,
        (datastore, database_session): &DB,
        asset_id: &str,
    ) -> Result<bool> {
        self.asset_repository
            .delete_by_id(datastore, database_session, &asset_id)
            .await
    }

    pub async fn create_asset_folder(
        &self,
        (datastore, database_session): &DB,
        name: String,
        logged_in_user: LoggedInUser
    ) -> Result<NewAssetModel> {

        let full_path = Path::new("public").join("upload").join(name.clone());
        // @todo createa folder in file system here...
        let _create_dir = tokio::fs::create_dir_all(full_path).await?;

        // @todo if we have a parent_id then use the path from parent_id to build a new path
        let relative_path = format!("/public/upload/{}", name);
        let color= String::from("text-gray-400");

        let creatable_asset_model = CreatableAssetModelNew {
            logged_in_username: logged_in_user.email,
            parent_id: "".to_string(),
            name: name.clone(),
            path: relative_path,
            asset_type: "FOLDER".to_string(),
            metadata: MetaDataType::FolderTypeMetaData {color},
        };

        self.asset_repository
            .create_asset_folder(datastore, database_session, creatable_asset_model)
            .await
    }

    pub async fn update_asset_path(
        &self,
        (datastore, database_session): &DB,
        name: &str,
        new_path: &str,
        asset_id: &str,
        logged_in_username: &str
    ) -> Result<NewAssetModel> {
        self.asset_repository
            .update_asset_path(datastore, database_session, name, new_path, asset_id, logged_in_username)
            .await
    }
}
