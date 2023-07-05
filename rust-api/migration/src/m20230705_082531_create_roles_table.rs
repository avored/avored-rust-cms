use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::Description).text().null())
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .col(ColumnDef::new(Roles::CreatedBy).string().default(Value::String(Some(Box::new(String::from("CLI"))))))
                    .col(ColumnDef::new(Roles::UpdatedBy).string().default(Value::String(Some(Box::new(String::from("CLI"))))))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Roles {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy
}
