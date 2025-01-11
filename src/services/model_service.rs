use crate::models::model_model::{ModelPagination, PutModelIdentifierModel, UpdatableModelModel};
use crate::models::{ModelCount, Pagination};
use crate::{
    error::Result,
    models::model_model::{CreatableModel, ModelModel},
    providers::avored_database_provider::DB,
    repositories::model_repository::ModelRepository,
    PER_PAGE,
};

pub struct ModelService {
    model_repository: ModelRepository,
}

impl ModelService {
    pub fn new(model_repository: ModelRepository) -> Result<Self> {
        Ok(ModelService { model_repository })
    }
}
impl ModelService {
    pub async fn create_model(
        &self,
        (datastore, database_session): &DB,
        creatable_model_model: CreatableModel,
    ) -> Result<ModelModel> {
        self.model_repository
            .create_model(datastore, database_session, creatable_model_model)
            .await
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
        order: String,
    ) -> Result<ModelPagination> {
        let start = current_page * PER_PAGE;
        let to = start + PER_PAGE;

        let model_count = self
            .model_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if model_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: model_count.total,
            per_page: PER_PAGE,
            current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let mut order_column = "id";
        let mut order_type = "ASC";
        let mut parts = order.split(':');
        if parts.clone().count() == 2 {
            order_column = parts.clone().nth(0).unwrap_or("");
            order_type = parts.nth(1).unwrap_or("");
        }

        let paginated_models = self
            .model_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        Ok(ModelPagination {
            data: paginated_models,
            pagination,
        })
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<ModelModel> {
        self.model_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn update_model_identifier(
        &self,
        (datastore, database_session): &DB,
        put_model_identifier_model: PutModelIdentifierModel,
    ) -> Result<ModelModel> {
        self.model_repository
            .update_model_identifier(datastore, database_session, put_model_identifier_model)
            .await
    }

    pub async fn count_of_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: String,
    ) -> Result<ModelCount> {
        self.model_repository
            .count_of_identifier(datastore, database_session, identifier)
            .await
    }

    pub async fn update_model(
        &self,
        (datastore, database_session): &DB,
        updatable_model_model: UpdatableModelModel,
    ) -> Result<ModelModel> {
        self.model_repository
            .update_model(datastore, database_session, updatable_model_model)
            .await
    }
}
