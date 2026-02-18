use rapina::migration::prelude::*;
use rapina::sea_orm_migration;

use super::m20260218_000002_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ImportSources::Table)
                    .col(
                        ColumnDef::new(ImportSources::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ImportSources::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(ImportSources::Platform)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ImportSources::Credentials).json_binary())
                    .col(
                        ColumnDef::new(ImportSources::AutoSync)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ImportSources::LastSyncedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ImportSources::SyncIntervalHours)
                            .integer()
                            .not_null()
                            .default(24),
                    )
                    .col(
                        ColumnDef::new(ImportSources::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ImportSources::Table, ImportSources::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ImportSources::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ImportSources {
    Table,
    Id,
    ProjectId,
    Platform,
    Credentials,
    AutoSync,
    LastSyncedAt,
    SyncIntervalHours,
    CreatedAt,
}
