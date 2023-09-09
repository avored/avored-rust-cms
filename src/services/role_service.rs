use crate::{
    error::Result, models::role_model::{RoleModel, CreatableRole, UpdatableRoleModel}, providers::avored_database_provider::DB,
    repositories::role_repository::RoleRepository,
};

pub struct RoleService {
    role_repository: RoleRepository,
}

impl RoleService {
    pub fn new(role_repository: RoleRepository) -> Result<Self> {
        Ok(RoleService { role_repository })
    }
}
impl RoleService {
    // pub async fn all_admin_users(
    //     &self,
    //     (datastore, database_session): &DB,
    // ) -> Result<Vec<AdminUserModel>> {
    //     let sql = "SELECT * FROM admin_users";

    //     let responses = datastore.execute(sql, database_session, None).await?;

    //     // println!("{:?}", responses);
    //     let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

    //     for object in into_iter_objects(responses)? {
    //         let admin_user_object = object?;

    //         let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
    //         admin_user_list.push(admin_user_model?);
    //     }
    //     Ok(admin_user_list)
    // }

    // pub async fn find_by_id(
    //     &self,
    //     (datastore, database_session): &DB,
    //     id: String,
    // ) -> Result<RoleModel> {
    //     self.role_repository
    //         .find_by_id(datastore, database_session, id)
    //         .await
    // }

    // pub async fn update_role(
    //     &self,
    //     (datastore, database_session): &DB,
    //     updateable_role_model: UpdatableAdminUserModel,
    // ) -> Result<AdminUserModel> {
    //     self.admin_user_repository
    //         .update_admin_user(datastore, database_session, updateable_admin_user_model)
    //         .await
    // }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        start: i64,
    ) -> Result<Vec<RoleModel>> {
        self.role_repository
            .paginate(datastore, database_session, start)
            .await
    }

    pub async fn create_role(
        &self,
        (datastore, database_session): &DB,
        creatable_role_model: CreatableRole,
    ) -> Result<RoleModel> {
        self.role_repository
            .create_role(datastore, database_session, creatable_role_model)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<RoleModel> {
        self.role_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn update_role(
        &self,
        (datastore, database_session): &DB,
        updatable_role_model: UpdatableRoleModel,
    ) -> Result<RoleModel> {
        self.role_repository
            .update_role(datastore, database_session, updatable_role_model)
            .await
    }


}
