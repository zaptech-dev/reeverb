use rapina::database::{Db, DbError};
use rapina::prelude::*;
use rapina::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::db::entities::user::{ActiveModel, Column, Entity as User};

use super::dto::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use super::error::AuthError;

#[public]
#[post("/api/v1/auth/register")]
#[errors(AuthError)]
pub async fn register(
    db: Db,
    auth: State<AuthConfig>,
    body: Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let req = body.into_inner();
    let auth_config = auth.into_inner();

    let existing = User::find()
        .filter(Column::Email.eq(&req.email))
        .one(db.conn())
        .await
        .map_err(DbError)?;

    if existing.is_some() {
        return Err(AuthError::EmailTaken.into_api_error());
    }

    let password_hash =
        bcrypt::hash(&req.password, 12).map_err(|e| AuthError::HashError(e.to_string()))?;

    let pid = Uuid::new_v4();

    let new_user = ActiveModel {
        pid: Set(pid),
        email: Set(req.email.clone()),
        password_hash: Set(Some(password_hash)),
        name: Set(req.name.clone()),
        avatar_url: Set(None),
        oauth_provider: Set(None),
        oauth_id: Set(None),
        ..Default::default()
    };

    new_user.insert(db.conn()).await.map_err(DbError)?;

    let token = auth_config.create_token(pid.to_string())?;

    Ok(Json(AuthResponse {
        token,
        expires_in: auth_config.expiration(),
        user: UserResponse {
            id: pid.to_string(),
            email: req.email,
            name: req.name,
            avatar_url: None,
        },
    }))
}

#[public]
#[post("/api/v1/auth/login")]
#[errors(AuthError)]
pub async fn login(
    db: Db,
    auth: State<AuthConfig>,
    body: Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let req = body.into_inner();
    let auth_config = auth.into_inner();

    let user = User::find()
        .filter(Column::Email.eq(&req.email))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| AuthError::InvalidCredentials.into_api_error())?;

    let password_hash = user
        .password_hash
        .as_deref()
        .ok_or_else(|| AuthError::InvalidCredentials.into_api_error())?;

    let valid =
        bcrypt::verify(&req.password, password_hash).map_err(|_| AuthError::InvalidCredentials)?;

    if !valid {
        return Err(AuthError::InvalidCredentials.into_api_error());
    }

    let token = auth_config.create_token(user.pid.to_string())?;

    Ok(Json(AuthResponse {
        token,
        expires_in: auth_config.expiration(),
        user: UserResponse {
            id: user.pid.to_string(),
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
        },
    }))
}

#[get("/api/v1/auth/me")]
#[errors(AuthError)]
pub async fn me(db: Db, current_user: CurrentUser) -> Result<Json<UserResponse>> {
    let pid = Uuid::parse_str(&current_user.id)
        .map_err(|_| Error::unauthorized("invalid user id in token"))?;

    let user = User::find()
        .filter(Column::Pid.eq(pid))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found("user not found"))?;

    Ok(Json(UserResponse {
        id: user.pid.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
    }))
}
