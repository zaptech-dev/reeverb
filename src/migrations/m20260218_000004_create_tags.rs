use rapina::migration::prelude::*;
use rapina::sea_orm_migration;

use super::m20260218_000002_create_projects::Projects;
use super::m20260218_000003_create_testimonials::Testimonials;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .col(ColumnDef::new(Tags::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tags::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(Tags::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Tags::Color).string_len(7))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tags::Table, Tags::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tags_project_name")
                    .table(Tags::Table)
                    .col(Tags::ProjectId)
                    .col(Tags::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Join table
        manager
            .create_table(
                Table::create()
                    .table(TestimonialTags::Table)
                    .col(
                        ColumnDef::new(TestimonialTags::TestimonialId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TestimonialTags::TagId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(TestimonialTags::TestimonialId)
                            .col(TestimonialTags::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TestimonialTags::Table, TestimonialTags::TestimonialId)
                            .to(Testimonials::Table, Testimonials::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TestimonialTags::Table, TestimonialTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestimonialTags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tags {
    Table,
    Id,
    ProjectId,
    Name,
    Color,
}

#[derive(DeriveIden)]
pub enum TestimonialTags {
    Table,
    TestimonialId,
    TagId,
}
