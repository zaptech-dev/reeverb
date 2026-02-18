use rapina::database::{Db, DbError};
use rapina::prelude::*;
use rapina::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::api::v1::tags::dto::TagResponse;
use crate::api::v1::tags::handlers::{load_tags_for_testimonial, load_tags_for_testimonials};
use crate::db::entities::project::{Column as ProjectColumn, Entity as Project};
use crate::db::entities::testimonial::{ActiveModel, Column, Entity as Testimonial};
use crate::db::entities::user::{Column as UserColumn, Entity as User};

use super::dto::{
    CreateTestimonialRequest, ListTestimonialsQuery, TestimonialResponse, UpdateTestimonialRequest,
};
use super::error::TestimonialError;

fn to_response(
    t: crate::db::entities::testimonial::Model,
    project_pid: &Uuid,
    tags: Vec<TagResponse>,
) -> TestimonialResponse {
    TestimonialResponse {
        id: t.pid.to_string(),
        project_id: project_pid.to_string(),
        testimonial_type: t.testimonial_type,
        content: t.content,
        rating: t.rating,
        author_name: t.author_name,
        author_email: t.author_email,
        author_title: t.author_title,
        author_avatar_url: t.author_avatar_url,
        author_company: t.author_company,
        author_url: t.author_url,
        video_url: t.video_url,
        video_thumbnail_url: t.video_thumbnail_url,
        video_duration_seconds: t.video_duration_seconds,
        transcription: t.transcription,
        source: t.source,
        source_platform: t.source_platform,
        source_url: t.source_url,
        source_id: t.source_id,
        sentiment: t.sentiment,
        sentiment_score: t.sentiment_score,
        language: t.language,
        is_approved: t.is_approved,
        is_featured: t.is_featured,
        tags,
        created_at: t.created_at.to_rfc3339(),
        updated_at: t.updated_at.to_rfc3339(),
    }
}

async fn resolve_user_id(db: &Db, current_user: &CurrentUser) -> Result<i32> {
    let pid = Uuid::parse_str(&current_user.id)
        .map_err(|_| Error::unauthorized("invalid user id in token"))?;

    let user = User::find()
        .filter(UserColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::unauthorized("user not found"))?;

    Ok(user.id)
}

#[get("/api/v1/projects/:id/testimonials")]
#[errors(TestimonialError)]
pub async fn list_testimonials(
    id: Path<String>,
    query: Query<ListTestimonialsQuery>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<Vec<TestimonialResponse>>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(ProjectColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let params = query.into_inner();
    let mut q = Testimonial::find().filter(Column::ProjectId.eq(project.id));

    if let Some(approved) = params.is_approved {
        q = q.filter(Column::IsApproved.eq(approved));
    }
    if let Some(featured) = params.is_featured {
        q = q.filter(Column::IsFeatured.eq(featured));
    }

    let testimonials = q.all(db.conn()).await.map_err(DbError)?;

    let testimonial_ids: Vec<i32> = testimonials.iter().map(|t| t.id).collect();
    let mut tags_map = load_tags_for_testimonials(&db, &testimonial_ids, &project.pid).await?;

    let response: Vec<TestimonialResponse> = testimonials
        .into_iter()
        .map(|t| {
            let tags = tags_map.remove(&t.id).unwrap_or_default();
            to_response(t, &project.pid, tags)
        })
        .collect();

    Ok(Json(response))
}

#[post("/api/v1/projects/:id/testimonials")]
#[errors(TestimonialError)]
pub async fn create_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<CreateTestimonialRequest>,
) -> Result<(StatusCode, Json<TestimonialResponse>)> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(ProjectColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let req = body.into_inner();

    let new_testimonial = ActiveModel {
        pid: Set(Uuid::new_v4()),
        project_id: Set(project.id),
        testimonial_type: Set(req.testimonial_type.unwrap_or_else(|| "text".to_string())),
        content: Set(req.content),
        rating: Set(req.rating),
        author_name: Set(req.author_name),
        author_email: Set(req.author_email),
        author_title: Set(req.author_title),
        author_avatar_url: Set(req.author_avatar_url),
        author_company: Set(req.author_company),
        author_url: Set(req.author_url),
        video_url: Set(req.video_url),
        video_thumbnail_url: Set(req.video_thumbnail_url),
        video_duration_seconds: Set(req.video_duration_seconds),
        transcription: Set(req.transcription),
        source: Set(req.source),
        source_platform: Set(req.source_platform),
        source_url: Set(req.source_url),
        source_id: Set(req.source_id),
        sentiment: Set(req.sentiment),
        sentiment_score: Set(req.sentiment_score),
        language: Set(req.language),
        ..Default::default()
    };

    let testimonial = new_testimonial.insert(db.conn()).await.map_err(DbError)?;

    Ok((
        StatusCode::CREATED,
        Json(to_response(testimonial, &project.pid, vec![])),
    ))
}

#[get("/api/v1/testimonials/:id")]
#[errors(TestimonialError)]
pub async fn get_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<TestimonialResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let tags = load_tags_for_testimonial(&db, testimonial.id, &project.pid).await?;
    Ok(Json(to_response(testimonial, &project.pid, tags)))
}

#[put("/api/v1/testimonials/:id")]
#[errors(TestimonialError)]
pub async fn update_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<UpdateTestimonialRequest>,
) -> Result<Json<TestimonialResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let req = body.into_inner();
    let mut active: ActiveModel = testimonial.into();

    if let Some(testimonial_type) = req.testimonial_type {
        active.testimonial_type = Set(testimonial_type);
    }
    if let Some(content) = req.content {
        active.content = Set(Some(content));
    }
    if let Some(rating) = req.rating {
        active.rating = Set(Some(rating));
    }
    if let Some(author_name) = req.author_name {
        active.author_name = Set(author_name);
    }
    if let Some(author_email) = req.author_email {
        active.author_email = Set(Some(author_email));
    }
    if let Some(author_title) = req.author_title {
        active.author_title = Set(Some(author_title));
    }
    if let Some(author_avatar_url) = req.author_avatar_url {
        active.author_avatar_url = Set(Some(author_avatar_url));
    }
    if let Some(author_company) = req.author_company {
        active.author_company = Set(Some(author_company));
    }
    if let Some(author_url) = req.author_url {
        active.author_url = Set(Some(author_url));
    }
    if let Some(video_url) = req.video_url {
        active.video_url = Set(Some(video_url));
    }
    if let Some(video_thumbnail_url) = req.video_thumbnail_url {
        active.video_thumbnail_url = Set(Some(video_thumbnail_url));
    }
    if let Some(video_duration_seconds) = req.video_duration_seconds {
        active.video_duration_seconds = Set(Some(video_duration_seconds));
    }
    if let Some(transcription) = req.transcription {
        active.transcription = Set(Some(transcription));
    }
    if let Some(source) = req.source {
        active.source = Set(Some(source));
    }
    if let Some(source_platform) = req.source_platform {
        active.source_platform = Set(Some(source_platform));
    }
    if let Some(source_url) = req.source_url {
        active.source_url = Set(Some(source_url));
    }
    if let Some(source_id) = req.source_id {
        active.source_id = Set(Some(source_id));
    }
    if let Some(sentiment) = req.sentiment {
        active.sentiment = Set(Some(sentiment));
    }
    if let Some(sentiment_score) = req.sentiment_score {
        active.sentiment_score = Set(Some(sentiment_score));
    }
    if let Some(language) = req.language {
        active.language = Set(Some(language));
    }
    if let Some(is_approved) = req.is_approved {
        active.is_approved = Set(is_approved);
    }
    if let Some(is_featured) = req.is_featured {
        active.is_featured = Set(is_featured);
    }

    let updated = active.update(db.conn()).await.map_err(DbError)?;

    let tags = load_tags_for_testimonial(&db, updated.id, &project.pid).await?;
    Ok(Json(to_response(updated, &project.pid, tags)))
}

#[delete("/api/v1/testimonials/:id")]
#[errors(TestimonialError)]
pub async fn delete_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<StatusCode> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    Testimonial::delete_by_id(testimonial.id)
        .exec(db.conn())
        .await
        .map_err(DbError)?;

    Ok(StatusCode::NO_CONTENT)
}

#[post("/api/v1/testimonials/:id/approve")]
#[errors(TestimonialError)]
pub async fn approve_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<TestimonialResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let new_value = !testimonial.is_approved;
    let testimonial_id = testimonial.id;
    let mut active: ActiveModel = testimonial.into();
    active.is_approved = Set(new_value);

    let updated = active.update(db.conn()).await.map_err(DbError)?;

    let tags = load_tags_for_testimonial(&db, testimonial_id, &project.pid).await?;
    Ok(Json(to_response(updated, &project.pid, tags)))
}

#[post("/api/v1/testimonials/:id/feature")]
#[errors(TestimonialError)]
pub async fn feature_testimonial(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<TestimonialResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner())
        .map_err(|_| TestimonialError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TestimonialError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TestimonialError::Forbidden.into_api_error());
    }

    let new_value = !testimonial.is_featured;
    let testimonial_id = testimonial.id;
    let mut active: ActiveModel = testimonial.into();
    active.is_featured = Set(new_value);

    let updated = active.update(db.conn()).await.map_err(DbError)?;

    let tags = load_tags_for_testimonial(&db, testimonial_id, &project.pid).await?;
    Ok(Json(to_response(updated, &project.pid, tags)))
}
