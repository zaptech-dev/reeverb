use rapina::database::DbError;
use rapina::prelude::*;

pub enum AuthError {
    DbError(DbError),
    InvalidCredentials,
    EmailTaken,
    HashError(String),
}

impl IntoApiError for AuthError {
    fn into_api_error(self) -> Error {
        match self {
            AuthError::DbError(e) => e.into_api_error(),
            AuthError::InvalidCredentials => Error::unauthorized("invalid credentials"),
            AuthError::EmailTaken => Error::conflict("email already registered"),
            AuthError::HashError(msg) => Error::internal(msg),
        }
    }
}

impl DocumentedError for AuthError {
    fn error_variants() -> Vec<ErrorVariant> {
        vec![
            ErrorVariant {
                status: 401,
                code: "UNAUTHORIZED",
                description: "Invalid email or password",
            },
            ErrorVariant {
                status: 409,
                code: "CONFLICT",
                description: "Email already registered",
            },
            ErrorVariant {
                status: 500,
                code: "INTERNAL_ERROR",
                description: "Internal server error",
            },
        ]
    }
}

impl From<DbError> for AuthError {
    fn from(e: DbError) -> Self {
        AuthError::DbError(e)
    }
}
