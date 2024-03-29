use crate::{
    error::Result,
    models::{
        admin_user_model::{
            AdminUserModel, AdminUserPagination, UpdatableAdminUserModel,
        },
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::admin_user_repository::AdminUserRepository,
    PER_PAGE,
};
use crate::models::admin_user_model::CreatableAdminUserModel;
use crate::repositories::role_repository::RoleRepository;

pub struct AdminUserService {
    admin_user_repository: AdminUserRepository,
    role_repository: RoleRepository
}

impl AdminUserService {
    pub fn new(admin_user_repository: AdminUserRepository, role_repository: RoleRepository) -> Result<Self> {
        Ok(AdminUserService {
            admin_user_repository,
            role_repository

        })
    }
}
impl AdminUserService {
    // pub async fn all_admin_users(
    //     &self,
    //     (datastore, database_session): &DB,
    // ) -> Result<Vec<AdminUserModel>> {
    //     let sql = "SELECT * FROM admin_users";

    //     let responses = datastore.execute(sql, database_session, None).await?;

    //     let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

    //     for object in into_iter_objects(responses)? {
    //         let admin_user_object = object?;

    //         let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
    //         admin_user_list.push(admin_user_model?);
    //     }
    //     Ok(admin_user_list)
    // }

    pub async fn find_by_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_email(datastore, database_session, email)
            .await
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn update_admin_user(
        &self,
        (datastore, database_session): &DB,
        updatable_admin_user_model: UpdatableAdminUserModel,
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self.admin_user_repository
            .update_admin_user(datastore, database_session, updatable_admin_user_model.clone())
            .await?;

        for role_id in updatable_admin_user_model.clone().role_ids {
            self.admin_user_repository
                .detach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_id)
                .await?;
        }

        for role_id in updatable_admin_user_model.role_ids {
            let role_model = self.role_repository.find_by_id(datastore, database_session, role_id).await?;
            self.admin_user_repository
                .attach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_model.clone().id)
                .await?;

            admin_user_model.roles.push(role_model);
        }

        Ok(admin_user_model)
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
    ) -> Result<AdminUserPagination> {
        let start = (current_page - 1) * PER_PAGE;
        let to = start + PER_PAGE;

        let admin_user_model_count = self
            .admin_user_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if admin_user_model_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 1 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: admin_user_model_count.total,
            per_page: PER_PAGE,
            current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let admin_users = self
            .admin_user_repository
            .paginate(datastore, database_session, start)
            .await?;

        Ok(AdminUserPagination {
            data: admin_users,
            pagination,
        })
    }

    // pub async fn delete_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     admin_user_id: String,
    // ) -> Result<bool> {
    //     self.admin_user_repository
    //         .delete_admin_user(datastore, database_session, admin_user_id)
    //         .await
    // }

    pub async fn create_admin_user(
        &self,
        (datastore, database_session): &DB,
        creatable_admin_user_model: CreatableAdminUserModel,
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self.admin_user_repository
            .create_admin_user(datastore, database_session, creatable_admin_user_model.clone())
            .await?;
        for role_id in creatable_admin_user_model.role_ids {
            let role_model = self.role_repository.find_by_id(datastore, database_session, role_id).await?;
            self.admin_user_repository
                .attach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_model.clone().id)
                .await?;

            admin_user_model.roles.push(role_model);
        }

        Ok(admin_user_model)
    }
}

// fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
//     let response = responses
//         .into_iter()
//         .next()
//         .map(|rp| rp.result)
//         .transpose()?;
//
//     match response {
//         Some(Value::Array(arr)) => {
//             let it = arr.into_iter().map(|v| match v {
//                 Value::Object(object) => Ok(object),
//                 _ => Err(Error::Generic("empty object")),
//             });
//
//             Ok(it)
//         }
//         _ => Err(Error::Generic("No Record found")),
//     }
// }
