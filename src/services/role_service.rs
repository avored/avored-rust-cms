use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use crate::models::role_model::RoleModel;
use crate::repositories::role_repository::RoleRepository;
use crate::error::Result;

pub struct RoleService {}

impl RoleService {
    pub fn new() -> RoleService {
        RoleService {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64
    ) -> Result<Vec<RoleModel>> {
        let role_repository = RoleRepository::new();
        role_repository.paginate(datastore, database_session, start).await
    }

}
