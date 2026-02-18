use rapina::database::DbError;
use rapina::prelude::*;

pub enum TestimonialError {
    DbError(DbError),
    NotFound,
    Forbidden,
}

impl IntoApiError for TestimonialError {
    fn into_api_error(self) -> Error {
        match self {
            TestimonialError::DbError(e) => e.into_api_error(),
            TestimonialError::NotFound => Error::not_found("testimonial not found"),
            TestimonialError::Forbidden => Error::forbidden("you do not own this project"),
        }
    }
}

impl DocumentedError for TestimonialError {
    fn error_variants() -> Vec<ErrorVariant> {
        vec![
            ErrorVariant {
                status: 404,
                code: "NOT_FOUND",
                description: "Testimonial not found",
            },
            ErrorVariant {
                status: 403,
                code: "FORBIDDEN",
                description: "User does not own this project",
            },
            ErrorVariant {
                status: 500,
                code: "INTERNAL_ERROR",
                description: "Internal server error",
            },
        ]
    }
}

impl From<DbError> for TestimonialError {
    fn from(e: DbError) -> Self {
        TestimonialError::DbError(e)
    }
}
