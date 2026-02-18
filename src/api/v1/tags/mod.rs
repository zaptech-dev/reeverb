pub mod dto;
pub mod error;
pub mod handlers;

use handlers::*;
use rapina::prelude::*;

pub fn project_routes() -> Router {
    Router::new()
        .get("/:id/tags", list_tags)
        .post("/:id/tags", create_tag)
}

pub fn routes() -> Router {
    Router::new()
        .put("/:id", update_tag)
        .delete("/:id", delete_tag)
}

pub fn testimonial_tag_routes() -> Router {
    Router::new().put("/:id/tags", set_testimonial_tags)
}
