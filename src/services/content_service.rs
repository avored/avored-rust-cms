use crate::models::content_model::{ContentModel, ContentPagination, CreatableContentModel, PutContentIdentifierModel, UpdatableContentModel};
use crate::providers::avored_database_provider::DB;
use crate::repositories::content_repository::ContentRepository;
use crate::error::Result;
use crate::models::{ModelCount, Pagination};
use crate::PER_PAGE;

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {

    pub async fn update_content(
        &self,
        (datastore, database_session): &DB,
        updatable_page_model: UpdatableContentModel,
    ) -> Result<ContentModel> {
        self.content_repository
            .update_content(datastore, database_session, updatable_page_model)
            .await
    }

    pub(crate) async fn count_of_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: &str,
        collection_type: &str
    ) -> Result<ModelCount> {
        self.content_repository
            .count_of_identifier(datastore, database_session, collection_type, identifier)
            .await
    }

    pub(crate) async fn update_content_identifier(
        &self,
        (datastore, database_session): &DB,
        put_content_identifier_model: PutContentIdentifierModel,
    ) -> Result<ContentModel> {
        self.content_repository
            .update_content_identifier(datastore, database_session, put_content_identifier_model)
            .await
    }


    pub(crate) async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        content_type: String,
        id: &str,
    ) -> Result<ContentModel> {
        self.content_repository
            .find_by_id(datastore, database_session, content_type, id)
            .await
    }
    pub(crate) async fn paginate(
        &self,
        (datastore, database_session): &DB,
        content_type: &str,
        current_page: i64,
        order: String,
    ) -> Result<ContentPagination> {
            let start = current_page * PER_PAGE;
            let to = start + PER_PAGE;

            let admin_user_count = self
                .content_repository
                .get_total_count(datastore, database_session, content_type)
                .await?;

            let mut has_next_page = false;
            if admin_user_count.total > to {
                has_next_page = true;
            };
            let mut has_previous_page = false;
            if current_page > 1 {
                has_previous_page = true;
            };

            let pagination = Pagination {
                total: admin_user_count.total,
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
            let content = self
                .content_repository
                .paginate(
                    datastore,
                    database_session,
                    content_type,
                    start,
                    order_column.to_string(),
                    order_type.to_string(),
                )
                .await?;

            Ok(ContentPagination {
                data: content,
                pagination,
            })
        }

    pub(crate) async fn create_content(
        &self,
        (datastore, database_session): &DB,
        creatable_page_model: CreatableContentModel,
    ) -> Result<ContentModel> {
        self.content_repository
            .create_content(datastore, database_session, creatable_page_model)
            .await
    }

    pub fn new(content_repository: ContentRepository) -> Result<Self> {
        Ok(ContentService { content_repository })
    }
}


