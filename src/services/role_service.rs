use crate::models::role_model::{PutRoleIdentifierModel, RoleOptionModel};
use crate::models::ModelCount;
use crate::{
    error::Result,
    models::{
        role_model::{CreatableRole, RoleModel, RolePagination, UpdatableRoleModel},
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::role_repository::RoleRepository,
    PER_PAGE,
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
    pub async fn all(&self, (datastore, database_session): &DB) -> Result<Vec<RoleOptionModel>> {
        let roles = self
            .role_repository
            .all(datastore, database_session)
            .await?;
        let mut role_options: Vec<RoleOptionModel> = vec![];

        for role in roles {
            let role_option_model = RoleOptionModel {
                value: role.id,
                label: role.name,
            };
            role_options.push(role_option_model);
        }

        Ok(role_options)
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
        order: String,
    ) -> Result<RolePagination> {
        let start = current_page * PER_PAGE;
        let to = start + PER_PAGE;

        let admin_user_count = self
            .role_repository
            .get_total_count(datastore, database_session)
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

        let roles = self
            .role_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        Ok(RolePagination {
            data: roles,
            pagination,
        })
    }

    pub async fn count_of_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: String,
    ) -> Result<ModelCount> {
        self.role_repository
            .count_of_identifier(datastore, database_session, identifier)
            .await
    }

    pub async fn update_role_identifier(
        &self,
        (datastore, database_session): &DB,
        put_role_identifier_model: PutRoleIdentifierModel,
    ) -> Result<RoleModel> {
        self.role_repository
            .update_role_identifier(datastore, database_session, put_role_identifier_model)
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
