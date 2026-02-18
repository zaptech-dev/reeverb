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
                    .table(Widgets::Table)
                    .col(
                        ColumnDef::new(Widgets::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Widgets::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(Widgets::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    // Type
                    .col(
                        ColumnDef::new(Widgets::WidgetType)
                            .string_len(50)
                            .not_null(),
                    )
                    // Filtering
                    .col(ColumnDef::new(Widgets::TagFilter).json_binary())
                    .col(ColumnDef::new(Widgets::MinRating).small_integer())
                    .col(
                        ColumnDef::new(Widgets::FeaturedOnly)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Widgets::MaxTestimonials)
                            .integer()
                            .not_null()
                            .default(20),
                    )
                    // Styling
                    .col(
                        ColumnDef::new(Widgets::Theme)
                            .string_len(20)
                            .not_null()
                            .default("light"),
                    )
                    .col(
                        ColumnDef::new(Widgets::AccentColor)
                            .string_len(7)
                            .not_null()
                            .default("#6366f1"),
                    )
                    .col(
                        ColumnDef::new(Widgets::BorderRadius)
                            .integer()
                            .not_null()
                            .default(8),
                    )
                    .col(ColumnDef::new(Widgets::FontFamily).string_len(100))
                    .col(ColumnDef::new(Widgets::CustomCss).text())
                    // Behavior
                    .col(
                        ColumnDef::new(Widgets::Autoplay)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Widgets::AutoplaySpeed)
                            .integer()
                            .not_null()
                            .default(5000),
                    )
                    .col(
                        ColumnDef::new(Widgets::ShowRating)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Widgets::ShowAvatar)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Widgets::ShowDate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Widgets::ShowSource)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Stats
                    .col(
                        ColumnDef::new(Widgets::ViewCount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Widgets::ClickCount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Widgets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Widgets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Widgets::Table, Widgets::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_widgets_project")
                    .table(Widgets::Table)
                    .col(Widgets::ProjectId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Widgets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Widgets {
    Table,
    Id,
    ProjectId,
    Name,
    WidgetType,
    TagFilter,
    MinRating,
    FeaturedOnly,
    MaxTestimonials,
    Theme,
    AccentColor,
    BorderRadius,
    FontFamily,
    CustomCss,
    Autoplay,
    AutoplaySpeed,
    ShowRating,
    ShowAvatar,
    ShowDate,
    ShowSource,
    ViewCount,
    ClickCount,
    CreatedAt,
    UpdatedAt,
}
