use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::AdminUsers, m20230705_082531_create_roles_table::Roles};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUsersRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUsersRoles::AdminUserId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AdminUsersRoles::RoleId)
                            .uuid()
                            .not_null()
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-admin_users_roles")
                            .col(AdminUsersRoles::AdminUserId)
                            .col(AdminUsersRoles::RoleId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-admin_users_roles-admin-users-id")
                            .from(AdminUsersRoles::Table, AdminUsersRoles::AdminUserId)
                            .to(AdminUsers::Table, AdminUsers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-admin_users_roles-role-id")
                            .from(AdminUsersRoles::Table, AdminUsersRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(AdminUsersRoles::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum AdminUsersRoles {
    Table,
    AdminUserId,
    RoleId,
}
