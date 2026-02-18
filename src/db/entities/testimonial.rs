use rapina::sea_orm;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "testimonials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub project_id: i32,
    #[sea_orm(column_name = "type")]
    pub testimonial_type: String,
    pub content: Option<String>,
    pub rating: Option<i16>,
    pub author_name: String,
    pub author_email: Option<String>,
    pub author_title: Option<String>,
    pub author_avatar_url: Option<String>,
    pub author_company: Option<String>,
    pub author_url: Option<String>,
    pub video_url: Option<String>,
    pub video_thumbnail_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub transcription: Option<String>,
    pub source: Option<String>,
    pub source_platform: Option<String>,
    pub source_url: Option<String>,
    pub source_id: Option<String>,
    pub sentiment: Option<String>,
    pub sentiment_score: Option<f32>,
    pub language: Option<String>,
    pub is_approved: bool,
    pub is_featured: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
