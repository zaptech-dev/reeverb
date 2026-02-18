use rapina::database::DatabaseConfig;
use rapina::middleware::RequestLogMiddleware;
use rapina::prelude::*;
use rapina::schemars;

mod migrations;

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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let router = Router::new().get("/health", health);

    Rapina::new()
        .with_tracing(TracingConfig::new())
        .openapi("Reeverb API", env!("CARGO_PKG_VERSION"))
        .middleware(RequestLogMiddleware::new())
        .with_database(DatabaseConfig::new(&database_url))
        .await?
        .run_migrations::<migrations::Migrator>()
        .await?
        .router(router)
        .listen("0.0.0.0:3000")
        .await
}
