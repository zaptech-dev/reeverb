use rapina::migration::prelude::*;
use rapina::sea_orm_migration;

use super::m20260218_000001_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .col(
                        ColumnDef::new(Projects::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Projects::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Projects::Slug)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Projects::LogoUrl).text())
                    .col(ColumnDef::new(Projects::WebsiteUrl).text())
                    .col(
                        ColumnDef::new(Projects::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Projects::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Projects::Table, Projects::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Projects {
    Table,
    Id,
    UserId,
    Name,
    Slug,
    LogoUrl,
    WebsiteUrl,
    CreatedAt,
    UpdatedAt,
}
