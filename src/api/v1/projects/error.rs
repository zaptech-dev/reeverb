use rapina::database::DbError;
use rapina::prelude::*;

pub enum ProjectError {
    DbError(DbError),
    NotFound,
    Forbidden,
    SlugTaken,
}

impl IntoApiError for ProjectError {
    fn into_api_error(self) -> Error {
        match self {
            ProjectError::DbError(e) => e.into_api_error(),
            ProjectError::NotFound => Error::not_found("project not found"),
            ProjectError::Forbidden => Error::forbidden("you do not own this project"),
            ProjectError::SlugTaken => Error::conflict("slug already taken"),
        }
    }
}

impl DocumentedError for ProjectError {
    fn error_variants() -> Vec<ErrorVariant> {
        vec![
            ErrorVariant {
                status: 404,
                code: "NOT_FOUND",
                description: "Project not found",
            },
            ErrorVariant {
                status: 403,
                code: "FORBIDDEN",
                description: "User does not own this project",
            },
            ErrorVariant {
                status: 409,
                code: "CONFLICT",
                description: "Slug already taken",
            },
            ErrorVariant {
                status: 500,
                code: "INTERNAL_ERROR",
                description: "Internal server error",
            },
        ]
    }
}

impl From<DbError> for ProjectError {
    fn from(e: DbError) -> Self {
        ProjectError::DbError(e)
    }
}
