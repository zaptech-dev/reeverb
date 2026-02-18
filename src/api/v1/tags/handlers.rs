use rapina::database::{Db, DbError};
use rapina::prelude::*;
use rapina::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::db::entities::project::{Column as ProjectColumn, Entity as Project};
use crate::db::entities::tag::{ActiveModel, Column, Entity as Tag};
use crate::db::entities::testimonial::{Column as TestimonialColumn, Entity as Testimonial};
use crate::db::entities::testimonial_tag::{
    ActiveModel as TestimonialTagActiveModel, Column as TestimonialTagColumn,
    Entity as TestimonialTag,
};
use crate::db::entities::user::{Column as UserColumn, Entity as User};

use super::dto::{
    CreateTagRequest, SetTestimonialTagsRequest, TagResponse, TestimonialTagsResponse,
    UpdateTagRequest,
};
use super::error::TagError;

fn to_response(tag: crate::db::entities::tag::Model, project_pid: &Uuid) -> TagResponse {
    TagResponse {
        id: tag.pid.to_string(),
        project_id: project_pid.to_string(),
        name: tag.name,
        color: tag.color,
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

pub fn to_tag_responses(
    tags: Vec<crate::db::entities::tag::Model>,
    project_pid: &Uuid,
) -> Vec<TagResponse> {
    tags.into_iter()
        .map(|t| to_response(t, project_pid))
        .collect()
}

pub async fn load_tags_for_testimonial(
    db: &Db,
    testimonial_id: i32,
    project_pid: &Uuid,
) -> Result<Vec<TagResponse>> {
    let links = TestimonialTag::find()
        .filter(TestimonialTagColumn::TestimonialId.eq(testimonial_id))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    if links.is_empty() {
        return Ok(vec![]);
    }

    let tag_ids: Vec<i32> = links.iter().map(|l| l.tag_id).collect();
    let tags = Tag::find()
        .filter(Column::Id.is_in(tag_ids))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    Ok(to_tag_responses(tags, project_pid))
}

pub async fn load_tags_for_testimonials(
    db: &Db,
    testimonial_ids: &[i32],
    project_pid: &Uuid,
) -> Result<std::collections::HashMap<i32, Vec<TagResponse>>> {
    use std::collections::HashMap;

    if testimonial_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let links = TestimonialTag::find()
        .filter(TestimonialTagColumn::TestimonialId.is_in(testimonial_ids.to_vec()))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    if links.is_empty() {
        return Ok(HashMap::new());
    }

    let tag_ids: Vec<i32> = links.iter().map(|l| l.tag_id).collect();
    let tags = Tag::find()
        .filter(Column::Id.is_in(tag_ids))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    let tag_map: HashMap<i32, crate::db::entities::tag::Model> =
        tags.into_iter().map(|t| (t.id, t)).collect();

    let mut result: HashMap<i32, Vec<TagResponse>> = HashMap::new();
    for link in links {
        if let Some(tag) = tag_map.get(&link.tag_id) {
            result
                .entry(link.testimonial_id)
                .or_default()
                .push(to_response(tag.clone(), project_pid));
        }
    }

    Ok(result)
}

#[get("/api/v1/projects/:id/tags")]
#[errors(TagError)]
pub async fn list_tags(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<Vec<TagResponse>>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner()).map_err(|_| TagError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(ProjectColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TagError::Forbidden.into_api_error());
    }

    let tags = Tag::find()
        .filter(Column::ProjectId.eq(project.id))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    let response = to_tag_responses(tags, &project.pid);
    Ok(Json(response))
}

#[post("/api/v1/projects/:id/tags")]
#[errors(TagError)]
pub async fn create_tag(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<TagResponse>)> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner()).map_err(|_| TagError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(ProjectColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TagError::Forbidden.into_api_error());
    }

    let req = body.into_inner();

    let existing = Tag::find()
        .filter(Column::ProjectId.eq(project.id))
        .filter(Column::Name.eq(&req.name))
        .one(db.conn())
        .await
        .map_err(DbError)?;

    if existing.is_some() {
        return Err(TagError::NameTaken.into_api_error());
    }

    let new_tag = ActiveModel {
        pid: Set(Uuid::new_v4()),
        project_id: Set(project.id),
        name: Set(req.name),
        color: Set(req.color),
        ..Default::default()
    };

    let tag = new_tag.insert(db.conn()).await.map_err(DbError)?;

    Ok((StatusCode::CREATED, Json(to_response(tag, &project.pid))))
}

#[put("/api/v1/tags/:id")]
#[errors(TagError)]
pub async fn update_tag(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<UpdateTagRequest>,
) -> Result<Json<TagResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner()).map_err(|_| TagError::NotFound.into_api_error())?;

    let tag = Tag::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    let project = Project::find_by_id(tag.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TagError::Forbidden.into_api_error());
    }

    let req = body.into_inner();

    if let Some(ref name) = req.name {
        let name_taken = Tag::find()
            .filter(Column::ProjectId.eq(project.id))
            .filter(Column::Name.eq(name))
            .filter(Column::Id.ne(tag.id))
            .one(db.conn())
            .await
            .map_err(DbError)?;

        if name_taken.is_some() {
            return Err(TagError::NameTaken.into_api_error());
        }
    }

    let mut active: ActiveModel = tag.into();

    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(color) = req.color {
        active.color = Set(Some(color));
    }

    let updated = active.update(db.conn()).await.map_err(DbError)?;

    Ok(Json(to_response(updated, &project.pid)))
}

#[delete("/api/v1/tags/:id")]
#[errors(TagError)]
pub async fn delete_tag(id: Path<String>, db: Db, current_user: CurrentUser) -> Result<StatusCode> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner()).map_err(|_| TagError::NotFound.into_api_error())?;

    let tag = Tag::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    let project = Project::find_by_id(tag.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TagError::Forbidden.into_api_error());
    }

    Tag::delete_by_id(tag.id)
        .exec(db.conn())
        .await
        .map_err(DbError)?;

    Ok(StatusCode::NO_CONTENT)
}

#[put("/api/v1/testimonials/:id/tags")]
#[errors(TagError)]
pub async fn set_testimonial_tags(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<SetTestimonialTagsRequest>,
) -> Result<Json<TestimonialTagsResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid = Uuid::parse_str(&id.into_inner()).map_err(|_| TagError::NotFound.into_api_error())?;

    let testimonial = Testimonial::find()
        .filter(TestimonialColumn::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    let project = Project::find_by_id(testimonial.project_id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| TagError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(TagError::Forbidden.into_api_error());
    }

    let req = body.into_inner();

    // Resolve tag pids to internal ids, verify all belong to this project
    let mut tag_models = Vec::new();
    for tag_pid_str in &req.tag_ids {
        let tag_pid =
            Uuid::parse_str(tag_pid_str).map_err(|_| TagError::NotFound.into_api_error())?;

        let tag = Tag::find()
            .filter(Column::Pid.eq(tag_pid))
            .one(db.conn())
            .await
            .map_err(DbError)?
            .ok_or_else(|| TagError::NotFound.into_api_error())?;

        if tag.project_id != project.id {
            return Err(TagError::Forbidden.into_api_error());
        }

        tag_models.push(tag);
    }

    // Delete existing testimonial_tags
    TestimonialTag::delete_many()
        .filter(TestimonialTagColumn::TestimonialId.eq(testimonial.id))
        .exec(db.conn())
        .await
        .map_err(DbError)?;

    // Insert new ones
    for tag in &tag_models {
        let link = TestimonialTagActiveModel {
            testimonial_id: Set(testimonial.id),
            tag_id: Set(tag.id),
        };
        link.insert(db.conn()).await.map_err(DbError)?;
    }

    let response_tags = to_tag_responses(tag_models, &project.pid);
    Ok(Json(TestimonialTagsResponse {
        tags: response_tags,
    }))
}
