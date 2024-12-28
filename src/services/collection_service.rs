use crate::error::Result;
use crate::models::collection_model::{
    CollectionModel, CollectionPagination, CreatableCollection, UpdatableCollection,
};
use crate::models::Pagination;
use crate::providers::avored_database_provider::DB;
use crate::repositories::collection_repository::CollectionRepository;
use crate::PER_PAGE;

pub struct CollectionService {
    collection_repository: CollectionRepository,
}

impl CollectionService {
    pub fn new(collection_repository: CollectionRepository) -> Result<Self> {
        Ok(CollectionService {
            collection_repository,
        })
    }
}
impl CollectionService {
    pub async fn create_collection(
        &self,
        (datastore, database_session): &DB,
        creatable_collection_collection: CreatableCollection,
    ) -> Result<CollectionModel> {
        self.collection_repository
            .create_collection(datastore, database_session, creatable_collection_collection)
            .await
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
        order: String,
    ) -> Result<CollectionPagination> {
        let start = current_page * PER_PAGE;
        let to = start + PER_PAGE;

        let collection_count = self
            .collection_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if collection_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: collection_count.total,
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

        let paginated_collections = self
            .collection_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        Ok(CollectionPagination {
            data: paginated_collections,
            pagination,
        })
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<CollectionModel> {
        self.collection_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    // pub async fn update_collection_identifier(
    //     &self,
    //     (datastore, database_session): &DB,
    //     put_collection_identifier_collection: PutCollectionIdentifierCollection
    // ) -> Result<CollectionCollection> {
    //     self.collection_repository
    //         .update_collection_identifier(datastore, database_session, put_collection_identifier_collection)
    //         .await
    // }

    // pub async fn count_of_identifier(
    //     &self,
    //     (datastore, database_session): &DB,
    //     identifier: String,
    // ) -> Result<CollectionCount> {
    //     self.collection_repository
    //         .count_of_identifier(datastore, database_session, identifier)
    //         .await
    // }

    pub async fn update_collection(
        &self,
        (datastore, database_session): &DB,
        updatable_collection_collection: UpdatableCollection,
    ) -> Result<CollectionModel> {
        self.collection_repository
            .update_collection(datastore, database_session, updatable_collection_collection)
            .await
    }
}
