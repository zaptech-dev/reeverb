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
                    .table(Forms::Table)
                    .col(
                        ColumnDef::new(Forms::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Forms::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(Forms::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Forms::Slug)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    // Config
                    .col(ColumnDef::new(Forms::Headline).text())
                    .col(ColumnDef::new(Forms::Description).text())
                    .col(
                        ColumnDef::new(Forms::Questions)
                            .json_binary()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Forms::AllowVideo)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Forms::AllowText)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Forms::RequireRating)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Branding
                    .col(ColumnDef::new(Forms::LogoUrl).text())
                    .col(
                        ColumnDef::new(Forms::AccentColor)
                            .string_len(7)
                            .not_null()
                            .default("#6366f1"),
                    )
                    .col(
                        ColumnDef::new(Forms::BackgroundColor)
                            .string_len(7)
                            .not_null()
                            .default("#ffffff"),
                    )
                    // Thank you page
                    .col(
                        ColumnDef::new(Forms::ThankYouTitle)
                            .text()
                            .default("Thank you!"),
                    )
                    .col(ColumnDef::new(Forms::ThankYouMessage).text())
                    .col(
                        ColumnDef::new(Forms::ThankYouCtaText).string_len(255),
                    )
                    .col(ColumnDef::new(Forms::ThankYouCtaUrl).text())
                    // Incentives
                    .col(
                        ColumnDef::new(Forms::IncentiveEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Forms::IncentiveDescription).text())
                    // Sharing
                    .col(
                        ColumnDef::new(Forms::ShareEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Forms::ShareMessage).text())
                    // Status
                    .col(
                        ColumnDef::new(Forms::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Forms::SubmissionCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Forms::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Forms::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Forms::Table, Forms::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_forms_slug")
                    .table(Forms::Table)
                    .col(Forms::Slug)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Forms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Forms {
    Table,
    Id,
    ProjectId,
    Name,
    Slug,
    Headline,
    Description,
    Questions,
    AllowVideo,
    AllowText,
    RequireRating,
    LogoUrl,
    AccentColor,
    BackgroundColor,
    ThankYouTitle,
    ThankYouMessage,
    ThankYouCtaText,
    ThankYouCtaUrl,
    IncentiveEnabled,
    IncentiveDescription,
    ShareEnabled,
    ShareMessage,
    IsActive,
    SubmissionCount,
    CreatedAt,
    UpdatedAt,
}
