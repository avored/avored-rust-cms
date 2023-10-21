use crate::{
    error::Result,
    providers::avored_database_provider::DB,
    repositories::asset_repository::AssetRepository,
};
use crate::models::asset_model::{CreatableAssetModel, AssetModel};

pub struct AssetService {
    asset_repository: AssetRepository,
}

impl AssetService {
    pub fn new(asset_repository: AssetRepository) -> Result<Self> {
        Ok(AssetService { asset_repository })
    }
}
impl AssetService {
    // pub async fn paginate(
    //     &self,
    //     (datastore, database_session): &DB,
    //     current_asset: i64,
    // ) -> Result<AssetPagination> {
    //     let start = (current_asset - 1) * PER_PAGE;
    //     let to = start + PER_PAGE;
    //
    //     let admin_user_count = self
    //         .asset_repository
    //         .get_total_count(datastore, database_session)
    //         .await?;
    //
    //     let mut has_next_asset = false;
    //     if admin_user_count.total > to {
    //         has_next_asset = true;
    //     };
    //     let mut has_previous_asset = false;
    //     if current_asset > 1 {
    //         has_previous_asset = true;
    //     };
    //
    //     let pagination = Pagination {
    //         total: admin_user_count.total,
    //         per_asset: PER_PAGE,
    //         current_asset,
    //         from: (start + 1),
    //         to,
    //         has_previous_asset,
    //         next_asset_number: (current_asset + 1),
    //         has_next_asset,
    //         previous_asset_number: (current_asset - 1),
    //     };
    //
    //     let assets = self
    //         .asset_repository
    //         .paginate(datastore, database_session, start)
    //         .await?;
    //
    //     // println!("{:?}", assets);
    //
    //     Ok(AssetPagination {
    //         data: assets,
    //         pagination,
    //     })
    // }

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

    // pub async fn update_asset(
    //     &self,
    //     (datastore, database_session): &DB,
    //     updatable_asset_model: UpdatableAssetModel,
    // ) -> Result<AssetModel> {
    //     self.asset_repository
    //         .update_asset(datastore, database_session, updatable_asset_model)
    //         .await
    // }
}
