use rapina::database::{Db, DbError};
use rapina::prelude::*;
use rapina::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::db::entities::project::{ActiveModel, Column, Entity as Project};
use crate::db::entities::user::{Column as UserColumn, Entity as User};

use super::dto::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use super::error::ProjectError;

fn to_response(p: crate::db::entities::project::Model) -> ProjectResponse {
    ProjectResponse {
        id: p.pid.to_string(),
        name: p.name,
        slug: p.slug,
        logo_url: p.logo_url,
        website_url: p.website_url,
        created_at: p.created_at.to_rfc3339(),
        updated_at: p.updated_at.to_rfc3339(),
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

#[get("/api/v1/projects")]
#[errors(ProjectError)]
pub async fn list_projects(
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<Vec<ProjectResponse>>> {
    let user_id = resolve_user_id(&db, &current_user).await?;

    let projects = Project::find()
        .filter(Column::UserId.eq(user_id))
        .all(db.conn())
        .await
        .map_err(DbError)?;

    let response: Vec<ProjectResponse> = projects.into_iter().map(to_response).collect();
    Ok(Json(response))
}

#[post("/api/v1/projects")]
#[errors(ProjectError)]
pub async fn create_project(
    db: Db,
    current_user: CurrentUser,
    body: Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ProjectResponse>)> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let req = body.into_inner();

    let existing = Project::find()
        .filter(Column::Slug.eq(&req.slug))
        .one(db.conn())
        .await
        .map_err(DbError)?;

    if existing.is_some() {
        return Err(ProjectError::SlugTaken.into_api_error());
    }

    let pid = Uuid::new_v4();

    let new_project = ActiveModel {
        pid: Set(pid),
        user_id: Set(user_id),
        name: Set(req.name),
        slug: Set(req.slug),
        logo_url: Set(req.logo_url),
        website_url: Set(req.website_url),
        ..Default::default()
    };

    let project = new_project.insert(db.conn()).await.map_err(DbError)?;

    Ok((StatusCode::CREATED, Json(to_response(project))))
}

#[get("/api/v1/projects/:id")]
#[errors(ProjectError)]
pub async fn get_project(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<Json<ProjectResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid =
        Uuid::parse_str(&id.into_inner()).map_err(|_| ProjectError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| ProjectError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(ProjectError::Forbidden.into_api_error());
    }

    Ok(Json(to_response(project)))
}

#[put("/api/v1/projects/:id")]
#[errors(ProjectError)]
pub async fn update_project(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
    body: Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid =
        Uuid::parse_str(&id.into_inner()).map_err(|_| ProjectError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| ProjectError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(ProjectError::Forbidden.into_api_error());
    }

    let req = body.into_inner();

    if let Some(ref slug) = req.slug {
        let slug_taken = Project::find()
            .filter(Column::Slug.eq(slug))
            .filter(Column::Id.ne(project.id))
            .one(db.conn())
            .await
            .map_err(DbError)?;

        if slug_taken.is_some() {
            return Err(ProjectError::SlugTaken.into_api_error());
        }
    }

    let mut active: ActiveModel = project.into();

    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(slug) = req.slug {
        active.slug = Set(slug);
    }
    if let Some(logo_url) = req.logo_url {
        active.logo_url = Set(Some(logo_url));
    }
    if let Some(website_url) = req.website_url {
        active.website_url = Set(Some(website_url));
    }

    let updated = active.update(db.conn()).await.map_err(DbError)?;

    Ok(Json(to_response(updated)))
}

#[delete("/api/v1/projects/:id")]
#[errors(ProjectError)]
pub async fn delete_project(
    id: Path<String>,
    db: Db,
    current_user: CurrentUser,
) -> Result<StatusCode> {
    let user_id = resolve_user_id(&db, &current_user).await?;
    let pid =
        Uuid::parse_str(&id.into_inner()).map_err(|_| ProjectError::NotFound.into_api_error())?;

    let project = Project::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| ProjectError::NotFound.into_api_error())?;

    if project.user_id != user_id {
        return Err(ProjectError::Forbidden.into_api_error());
    }

    Project::delete_by_id(project.id)
        .exec(db.conn())
        .await
        .map_err(DbError)?;

    Ok(StatusCode::NO_CONTENT)
}
