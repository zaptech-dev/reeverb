use rapina::database::DatabaseConfig;
use rapina::middleware::RequestLogMiddleware;
use rapina::prelude::*;
use rapina::schemars;

use reeverb::api::v1::auth;
use reeverb::api::v1::projects;
use reeverb::api::v1::tags;
use reeverb::api::v1::testimonials;

#[derive(Clone, Config)]
struct AppConfig {
    #[env = "DATABASE_URL"]
    database_url: String,

    #[env = "HOST"]
    #[default = "0.0.0.0"]
    host: String,

    #[env = "PORT"]
    #[default = "3000"]
    port: u16,
}

#[derive(Serialize, JsonSchema)]
struct HealthResponse {
    status: String,
    version: String,
}

#[public]
#[get("/health")]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env().expect("missing required config");
    let auth_config = AuthConfig::from_env().expect("JWT_SECRET must be set");

    let router = Router::new()
        .get("/health", health)
        .group("/api/v1/auth", auth::routes())
        .group("/api/v1/projects", projects::routes())
        .group("/api/v1/projects", testimonials::project_routes())
        .group("/api/v1/projects", tags::project_routes())
        .group("/api/v1/tags", tags::routes())
        .group("/api/v1/testimonials", testimonials::routes())
        .group("/api/v1/testimonials", tags::testimonial_tag_routes());

    let mut app = Rapina::new()
        .with_tracing(TracingConfig::new())
        .with_auth(auth_config.clone())
        .public_route("GET", "/health");

    for (method, path) in auth::PUBLIC_ROUTES {
        app = app.public_route(method, path);
    }

    let addr = format!("{}:{}", config.host, config.port);

    app.openapi("Reeverb API", env!("CARGO_PKG_VERSION"))
        .middleware(RequestLogMiddleware::new())
        .state(auth_config)
        .with_database(DatabaseConfig::new(&config.database_url))
        .await?
        .run_migrations::<reeverb::db::migrations::Migrator>()
        .await?
        .router(router)
        .listen(&addr)
        .await
}
