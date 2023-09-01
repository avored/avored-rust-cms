use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use crate::models::ModelCount;
use crate::models::role_model::{RoleModel, CreatableRole, UpdatableRole};
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

    pub async fn has_identifier_taken(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: String
    ) -> Result<ModelCount> {
        let role_repository = RoleRepository::new();
        role_repository.has_identifier_taken(datastore, database_session, identifier).await
    }

    pub async fn create_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_role: CreatableRole
    ) -> Result<RoleModel> {
        let role_repository = RoleRepository::new();
        role_repository.create_role(datastore, database_session, createable_role).await
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: String
    ) -> Result<RoleModel> {
        let role_repository = RoleRepository::new();
        role_repository.find_by_id(datastore, database_session, id).await
    }

    pub async fn update_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_role: UpdatableRole
    ) -> Result<RoleModel> {
        let role_repository = RoleRepository::new();
        role_repository.update_role(datastore, database_session, updatable_role).await
    }
}
