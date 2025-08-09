use crate::api::proto::setting::GetSettingResponse;
use crate::api::proto::setting::{
    SettingModel as SettingModelGrpc, StoreSettingRequest, StoreSettingResponse,
};
use crate::models::setting_model::UpdatableSettingModel;
use crate::providers::avored_database_provider::DB;
use crate::{error::Result, repositories::setting_repository::SettingRepository};

/// Setting service 
pub struct SettingService {
    setting_repository: SettingRepository,
}

impl SettingService {
    /// Creates a new instance of `SettingService`
    pub fn new(setting_repository: SettingRepository) -> Result<Self> {
        Ok(Self { setting_repository })
    }

    /// get settings
    pub async fn get_setting(
        &self,
        (datastore, database_session): &DB,
    ) -> Result<GetSettingResponse> {
        let models = self
            .setting_repository
            .all(datastore, database_session)
            .await?;

        let mut setting_grpc_models = vec![];
        for model in models {
            let setting_grpc_model: SettingModelGrpc = model.try_into()?;
            setting_grpc_models.push(setting_grpc_model);
        }

        let res = GetSettingResponse {
            status: true,
            data: setting_grpc_models,
        };

        Ok(res)
    }

    /// store settings
    pub async fn store_setting(
        &self,
        (datastore, database_session): &DB,
        request: StoreSettingRequest,
        email: String,
    ) -> Result<StoreSettingResponse> {
        for setting in request.data {
            let updatable_setting_model = UpdatableSettingModel {
                id: setting.id,
                value: setting.value,
                logged_in_username: email.clone(),
            };
            self.setting_repository
                .update_setting(datastore, database_session, updatable_setting_model)
                .await?;
        }

        let res = StoreSettingResponse { status: true };

        Ok(res)
    }
}

