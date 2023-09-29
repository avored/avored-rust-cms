use crate::{
    error::Result,
    models::field_model::{CreatableFieldModel, FieldModel},
    providers::avored_database_provider::DB,
    repositories::field_repository::FieldRepository,
};

pub struct FieldService {
    field_repository: FieldRepository,
}

impl FieldService {
    pub fn new(field_repository: FieldRepository) -> Result<Self> {
        Ok(FieldService { field_repository })
    }
}
impl FieldService {
    // pub async fn paginate(
    //     &self,
    //     (datastore, database_session): &DB,
    //     current_page: i64,
    // ) -> Result<FieldPagination> {
    //     let start = (current_page - 1) * PER_PAGE;
    //     let to = start + PER_PAGE;
    //
    //     let field_count = self
    //         .field_repository
    //         .get_total_count(datastore, database_session)
    //         .await?;
    //
    //     let mut has_next_page = false;
    //     if field_count.total > to {
    //         has_next_page = true;
    //     };
    //     let mut has_previous_page = false;
    //     if current_page > 1 {
    //         has_previous_page = true;
    //     };
    //
    //     let pagination = Pagination {
    //         total: field_count.total,
    //         per_page: PER_PAGE,
    //         current_page: current_page,
    //         from: (start + 1),
    //         to,
    //         has_previous_page,
    //         next_page_number: (current_page + 1),
    //         has_next_page,
    //         previous_page_number: (current_page - 1),
    //     };
    //
    //     let fields = self
    //         .field_repository
    //         .paginate(datastore, database_session, start)
    //         .await?;
    //
    //     Ok(FieldPagination {
    //         data: fields,
    //         pagination,
    //     })
    // }

    pub async fn create_field(
        &self,
        (datastore, database_session): &DB,
        creatable_field_model: CreatableFieldModel,
    ) -> Result<FieldModel> {
        // Think of how we going to do a DB transaction
        // create field
        // create field
        // attach field with field

        self.field_repository
            .create_field(datastore, database_session, creatable_field_model)
            .await
    }

    // pub async fn find_by_id(
    //     &self,
    //     (datastore, database_session): &DB,
    //     id: String,
    // ) -> Result<FieldModel> {
    //     self.field_repository
    //         .find_by_id(datastore, database_session, id)
    //         .await
    // }
    //
    // pub async fn update_field(
    //     &self,
    //     (datastore, database_session): &DB,
    //     updatable_field_model: UpdatableFieldModel,
    // ) -> Result<FieldModel> {
    //     self.field_repository
    //         .update_field(datastore, database_session, updatable_field_model)
    //         .await
    // }
    //
    // pub async fn delete_field(
    //     &self,
    //     (datastore, database_session): &DB,
    //     field_id: String,
    // ) -> Result<bool> {
    //     self.field_repository
    //         .delete_field(datastore, database_session, field_id)
    //         .await
    // }
}
