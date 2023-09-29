use crate::models::field_model::FieldModel;
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
    ) -> Result<ComponentPagination> {
        let start = (current_page - 1) * PER_PAGE;
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
            current_page: current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let components = self
            .component_repository
            .paginate(datastore, database_session, start)
            .await?;

        Ok(ComponentPagination {
            data: components,
            pagination,
        })
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

    pub async fn delete_component(
        &self,
        (datastore, database_session): &DB,
        component_id: String,
    ) -> Result<bool> {
        self.component_repository
            .delete_component(datastore, database_session, component_id)
            .await
    }

    pub async fn attach_component_with_field(
        &self,
        (datastore, database_session): &DB,
        component_model: ComponentModel,
        field_model: FieldModel,
        logged_in_username: String,
    ) -> Result<bool> {
        self.component_repository
            .attach_component_with_field(
                datastore,
                database_session,
                component_model,
                field_model,
                logged_in_username,
            )
            .await
    }
}
