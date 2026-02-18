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
                    .table(Testimonials::Table)
                    .col(
                        ColumnDef::new(Testimonials::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Testimonials::ProjectId).uuid().not_null())
                    // Content
                    .col(
                        ColumnDef::new(Testimonials::Type)
                            .string_len(20)
                            .not_null()
                            .default("text"),
                    )
                    .col(ColumnDef::new(Testimonials::Content).text())
                    .col(ColumnDef::new(Testimonials::Rating).small_integer())
                    // Author
                    .col(
                        ColumnDef::new(Testimonials::AuthorName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Testimonials::AuthorEmail).string_len(255))
                    .col(ColumnDef::new(Testimonials::AuthorTitle).string_len(255))
                    .col(ColumnDef::new(Testimonials::AuthorAvatarUrl).text())
                    .col(ColumnDef::new(Testimonials::AuthorCompany).string_len(255))
                    .col(ColumnDef::new(Testimonials::AuthorUrl).text())
                    // Media
                    .col(ColumnDef::new(Testimonials::VideoUrl).text())
                    .col(ColumnDef::new(Testimonials::VideoThumbnailUrl).text())
                    .col(ColumnDef::new(Testimonials::VideoDurationSeconds).integer())
                    .col(ColumnDef::new(Testimonials::Transcription).text())
                    // Source
                    .col(
                        ColumnDef::new(Testimonials::Source)
                            .string_len(50)
                            .default("form"),
                    )
                    .col(ColumnDef::new(Testimonials::SourcePlatform).string_len(50))
                    .col(ColumnDef::new(Testimonials::SourceUrl).text())
                    .col(ColumnDef::new(Testimonials::SourceId).string_len(255))
                    // Metadata
                    .col(ColumnDef::new(Testimonials::Sentiment).string_len(20))
                    .col(ColumnDef::new(Testimonials::SentimentScore).float())
                    .col(ColumnDef::new(Testimonials::Language).string_len(10))
                    .col(
                        ColumnDef::new(Testimonials::IsApproved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Testimonials::IsFeatured)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Testimonials::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Testimonials::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Testimonials::Table, Testimonials::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_testimonials_project")
                    .table(Testimonials::Table)
                    .col(Testimonials::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_testimonials_approved")
                    .table(Testimonials::Table)
                    .col(Testimonials::ProjectId)
                    .col(Testimonials::IsApproved)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_testimonials_featured")
                    .table(Testimonials::Table)
                    .col(Testimonials::ProjectId)
                    .col(Testimonials::IsFeatured)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_testimonials_source")
                    .table(Testimonials::Table)
                    .col(Testimonials::SourcePlatform)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_testimonials_sentiment")
                    .table(Testimonials::Table)
                    .col(Testimonials::Sentiment)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Testimonials::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Testimonials {
    Table,
    Id,
    ProjectId,
    Type,
    Content,
    Rating,
    AuthorName,
    AuthorEmail,
    AuthorTitle,
    AuthorAvatarUrl,
    AuthorCompany,
    AuthorUrl,
    VideoUrl,
    VideoThumbnailUrl,
    VideoDurationSeconds,
    Transcription,
    Source,
    SourcePlatform,
    SourceUrl,
    SourceId,
    Sentiment,
    SentimentScore,
    Language,
    IsApproved,
    IsFeatured,
    CreatedAt,
    UpdatedAt,
}
