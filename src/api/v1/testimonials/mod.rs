pub mod dto;
pub mod error;
pub mod handlers;

use handlers::*;
use rapina::prelude::*;

pub fn project_routes() -> Router {
    Router::new()
        .get("/:id/testimonials", list_testimonials)
        .post("/:id/testimonials", create_testimonial)
}

pub fn routes() -> Router {
    Router::new()
        .get("/:id", get_testimonial)
        .put("/:id", update_testimonial)
        .delete("/:id", delete_testimonial)
        .post("/:id/approve", approve_testimonial)
        .post("/:id/feature", feature_testimonial)
}
