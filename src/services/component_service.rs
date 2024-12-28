use crate::models::component_model::PutComponentIdentifierModel;
use crate::models::ModelCount;
use crate::{
    error::Result,
    models::{
        component_model::{
            ComponentModel, ComponentPagination, CreatableComponent, UpdatableComponentModel,
        },
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::component_repository::ComponentRepository,
    PER_PAGE,
};

pub struct ComponentService {
    component_repository: ComponentRepository,
}

impl ComponentService {
    pub fn new(component_repository: ComponentRepository) -> Result<Self> {
        Ok(ComponentService {
            component_repository,
        })
    }
}
impl ComponentService {
    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
        order: String,
    ) -> Result<ComponentPagination> {
        let start = current_page * PER_PAGE;
        let to = start + PER_PAGE;

        let component_count = self
            .component_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if component_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: component_count.total,
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

        let components = self
            .component_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        Ok(ComponentPagination {
            data: components,
            pagination,
        })
    }
    pub async fn all(&self, (datastore, database_session): &DB) -> Result<Vec<ComponentModel>> {
        self.component_repository
            .all(datastore, database_session)
            .await
    }
    pub async fn create_component(
        &self,
        (datastore, database_session): &DB,
        creatable_component_model: CreatableComponent,
    ) -> Result<ComponentModel> {
        self.component_repository
            .create_component(datastore, database_session, creatable_component_model)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<ComponentModel> {
        self.component_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn update_component(
        &self,
        (datastore, database_session): &DB,
        updatable_component_model: UpdatableComponentModel,
    ) -> Result<ComponentModel> {
        self.component_repository
            .update_component(datastore, database_session, updatable_component_model)
            .await
    }

    pub async fn count_of_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: String,
    ) -> Result<ModelCount> {
        self.component_repository
            .count_of_identifier(datastore, database_session, identifier)
            .await
    }

    pub async fn update_component_identifier(
        &self,
        (datastore, database_session): &DB,
        put_component_identifier_model: PutComponentIdentifierModel,
    ) -> Result<ComponentModel> {
        self.component_repository
            .update_component_identifier(
                datastore,
                database_session,
                put_component_identifier_model,
            )
            .await
    }
}
