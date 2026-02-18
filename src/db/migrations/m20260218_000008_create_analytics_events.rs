use rapina::migration::prelude::*;
use rapina::sea_orm_migration;

use super::m20260218_000006_create_widgets::Widgets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AnalyticsEvents::Table)
                    .col(
                        ColumnDef::new(AnalyticsEvents::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AnalyticsEvents::WidgetId).uuid().not_null())
                    .col(
                        ColumnDef::new(AnalyticsEvents::EventType)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(AnalyticsEvents::Referrer).text())
                    .col(ColumnDef::new(AnalyticsEvents::UserAgent).text())
                    .col(ColumnDef::new(AnalyticsEvents::IpHash).string_len(64))
                    .col(ColumnDef::new(AnalyticsEvents::Country).string_len(2))
                    .col(
                        ColumnDef::new(AnalyticsEvents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnalyticsEvents::Table, AnalyticsEvents::WidgetId)
                            .to(Widgets::Table, Widgets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_analytics_widget")
                    .table(AnalyticsEvents::Table)
                    .col(AnalyticsEvents::WidgetId)
                    .col(AnalyticsEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_analytics_type")
                    .table(AnalyticsEvents::Table)
                    .col(AnalyticsEvents::EventType)
                    .col(AnalyticsEvents::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AnalyticsEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AnalyticsEvents {
    Table,
    Id,
    WidgetId,
    EventType,
    Referrer,
    UserAgent,
    IpHash,
    Country,
    CreatedAt,
}
