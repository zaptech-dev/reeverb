use rapina::database::DbError;
use rapina::prelude::*;

pub enum TagError {
    DbError(DbError),
    NotFound,
    Forbidden,
    NameTaken,
}

impl IntoApiError for TagError {
    fn into_api_error(self) -> Error {
        match self {
            TagError::DbError(e) => e.into_api_error(),
            TagError::NotFound => Error::not_found("tag not found"),
            TagError::Forbidden => Error::forbidden("you do not own this project"),
            TagError::NameTaken => {
                Error::conflict("a tag with this name already exists in this project")
            }
        }
    }
}

impl DocumentedError for TagError {
    fn error_variants() -> Vec<ErrorVariant> {
        vec![
            ErrorVariant {
                status: 404,
                code: "NOT_FOUND",
                description: "Tag not found",
            },
            ErrorVariant {
                status: 403,
                code: "FORBIDDEN",
                description: "User does not own this project",
            },
            ErrorVariant {
                status: 409,
                code: "CONFLICT",
                description: "Tag name already exists in this project",
            },
            ErrorVariant {
                status: 500,
                code: "INTERNAL_ERROR",
                description: "Internal server error",
            },
        ]
    }
}

impl From<DbError> for TagError {
    fn from(e: DbError) -> Self {
        TagError::DbError(e)
    }
}
