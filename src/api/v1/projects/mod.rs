pub mod dto;
pub mod error;
pub mod handlers;

use handlers::*;
use rapina::prelude::*;

pub fn routes() -> Router {
    Router::new()
        .get("/", list_projects)
        .post("/", create_project)
        .get("/:id", get_project)
        .put("/:id", update_project)
        .delete("/:id", delete_project)
}
