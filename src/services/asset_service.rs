use crate::api::proto::asset::asset_paginate_response::{AssetPaginateData, AssetPagination};
use crate::api::proto::asset::{
    AssetPaginateRequest, AssetPaginateResponse, CreateFolderRequest, CreateFolderResponse,
    DeleteAssetRequest, DeleteAssetResponse, DeleteFolderRequest, DeleteFolderResponse,
    RenameAssetRequest, RenameAssetResponse,
};
use crate::error::Error;
use crate::models::asset_model::{
    AssetModel, CreatableAssetModel, FolderTypeMetaData, MetaDataType,
};
use crate::{
    error::Result, providers::avored_database_provider::DB,
    repositories::asset_repository::AssetRepository, PER_PAGE,
};
use tokio::fs;
use tonic::Status;

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
        req: AssetPaginateRequest,
        (datastore, database_session): &DB,
    ) -> Result<AssetPaginateResponse> {
        let asset_model_count = self
            .asset_repository
            .get_total_count(datastore, database_session, "".to_string())
            .await?;

        let per_page: i64 = PER_PAGE as i64;
        let current_page = req.page.unwrap_or(0);
        let order = req.order.unwrap_or_default();

        let start = current_page * per_page;
        let mut order_column = "id";
        let mut order_type = "desc";
        if !order.is_empty() {
            let mut parts = order.split(':');
            if parts.clone().count() == 2 {
                order_column = parts.clone().nth(0).unwrap_or("");
                order_type = parts.nth(1).unwrap_or("");
            }
        }

        let assets = self
            .asset_repository
            .paginate(
                datastore,
                database_session,
                start,
                "".to_string(),
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        let mut grpc_assets = vec![];
        assets.iter().for_each(|asset| {
            let model: crate::api::proto::asset::AssetModel = asset.clone().try_into().unwrap();

            grpc_assets.push(model);
        });

        let pagination = AssetPagination {
            total: asset_model_count.total,
        };
        let paginate_data = AssetPaginateData {
            pagination: Option::from(pagination),
            data: grpc_assets,
        };

        let res = AssetPaginateResponse {
            status: true,
            data: Option::from(paginate_data),
        };

        Ok(res)
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
        creatable_asset_model: CreatableAssetModel,
    ) -> Result<AssetModel> {
        self.asset_repository
            .create_asset(datastore, database_session, creatable_asset_model)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        asset_id: &str,
    ) -> Result<AssetModel> {
        self.asset_repository
            .find_by_id(datastore, database_session, asset_id)
            .await
    }

    pub async fn create_asset_folder(
        &self,
        db: &DB,
        req: CreateFolderRequest,
        logged_in_user: &str,
    ) -> Result<CreateFolderResponse> {
        let (datastore, database_session) = db;

        let name = req.name;
        let parent_id = req.parent_id.unwrap_or_default();

        let mut full_path = format!("public/upload/{}", name.clone());

        if !parent_id.is_empty() {
            let asset = self.find_by_id(db, &parent_id).await?;

            full_path = format!("{}/{}", asset.new_path, name.clone());
        }
        fs::create_dir_all(&format!("./{}", full_path.clone())).await?;

        // @todo fix this default color????
        let color = String::from("text-gray-400");

        let creatable_asset_model = CreatableAssetModel {
            logged_in_username: logged_in_user.to_string(),
            parent_id,
            name: name.clone(),
            asset_type: "FOLDER".to_string(),
            metadata: MetaDataType {
                file_meta_data: Default::default(),
                folder_meta_data: FolderTypeMetaData { color },
            },
        };

        let asset_model = self
            .asset_repository
            .create_asset_folder(datastore, database_session, creatable_asset_model)
            .await?;

        let grpc_asset_model: crate::api::proto::asset::AssetModel = asset_model.try_into()?;

        let res = CreateFolderResponse {
            status: true,
            data: Option::from(grpc_asset_model),
        };

        Ok(res)
    }

    pub async fn delete_asset(
        &self,
        (datastore, database_session): &DB,
        request: DeleteAssetRequest,
    ) -> Result<DeleteAssetResponse> {
        let asset_model = self
            .asset_repository
            .find_by_id(datastore, database_session, &request.asset_id)
            .await?;
        let asset_path = format!("./{path}", path = asset_model.new_path);

        if fs::try_exists(&asset_path).await? {
            fs::remove_file(asset_path).await?;

            let result = self
                .asset_repository
                .delete_by_id(datastore, database_session, &request.asset_id)
                .await?;
            let res = DeleteAssetResponse { status: result };

            return Ok(res);
        }

        Err(Error::Tonic(Box::new(Status::internal("Unable to delete asset"))))
    }

    pub async fn delete_folder(
        &self,
        (datastore, database_session): &DB,
        request: DeleteFolderRequest,
    ) -> Result<DeleteFolderResponse> {
        let asset_model = self
            .asset_repository
            .find_by_id(datastore, database_session, &request.folder_id)
            .await?;
        let folder_path = format!("./{path}", path = asset_model.new_path);

        if fs::try_exists(&folder_path).await? {
            fs::remove_dir(&folder_path).await?;

            let result = self
                .asset_repository
                .delete_by_id(datastore, database_session, &request.folder_id)
                .await?;
            let res = DeleteFolderResponse { status: result };

            return Ok(res);
        }

        Err(Error::Tonic(Box::new(Status::internal("Unable to delete folder"))))
    }

    pub async fn rename_asset(
        &self,
        (datastore, database_session): &DB,
        request: RenameAssetRequest,
        logged_in_username: &str,
    ) -> Result<RenameAssetResponse> {
        let asset_model = self
            .asset_repository
            .find_by_id(datastore, database_session, &request.asset_id)
            .await?;
        let old_asset_path = format!(".{}", asset_model.new_path);
        let new_asset_path = format!("/public/upload/{}", &request.name);

        if fs::try_exists(&old_asset_path).await? {
            fs::rename(&old_asset_path, &format!(".{new_asset_path}")).await?;
            let updated_asset_model = self.asset_repository
                .update_asset_path(
                    datastore,
                    database_session,
                    &request.name,
                    &request.asset_id,
                    logged_in_username,
                )
                .await?;
            let updated_asset_model: crate::api::proto::asset::AssetModel =
                updated_asset_model.try_into()?;

            let response = RenameAssetResponse {
                status: true,
                data: Some(updated_asset_model),
            };

            return Ok(response);
        }

        Err(Error::Tonic(Box::new(Status::internal("Unable to rename asset"))))
    }
}
