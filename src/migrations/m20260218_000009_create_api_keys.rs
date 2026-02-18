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
                    .table(ApiKeys::Table)
                    .col(
                        ColumnDef::new(ApiKeys::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKeys::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(ApiKeys::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::KeyHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::KeyPrefix)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::Scopes)
                            .json_binary()
                            .not_null()
                            .default("[\"read\"]"),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::LastUsedAt)
                            .timestamp_with_time_zone(),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::ExpiresAt)
                            .timestamp_with_time_zone(),
                    )
                    .col(
                        ColumnDef::new(ApiKeys::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ApiKeys::Table, ApiKeys::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ApiKeys {
    Table,
    Id,
    UserId,
    Name,
    KeyHash,
    KeyPrefix,
    Scopes,
    LastUsedAt,
    ExpiresAt,
    CreatedAt,
}
