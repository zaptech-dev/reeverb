use rapina::auth::{AuthMiddleware, PublicRoutes};
use rapina::database::DatabaseConfig;
use rapina::prelude::*;
use rapina::testing::TestClient;
use serde_json::json;
use tokio::sync::OnceCell;
use uuid::Uuid;

use reeverb::api::v1::auth;
use reeverb::api::v1::projects;
use reeverb::db::migrations::Migrator;

static MIGRATIONS: OnceCell<()> = OnceCell::const_new();

fn database_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests")
}

async fn run_migrations_once() {
    MIGRATIONS
        .get_or_init(|| async {
            let config = DatabaseConfig::new(database_url());
            let conn = config.connect().await.expect("failed to connect");

            use rapina::sea_orm_migration::MigratorTrait;
            Migrator::up(&conn, None)
                .await
                .expect("failed to run migrations");
        })
        .await;
}

async fn setup() -> TestClient {
    run_migrations_once().await;

    let auth_config = AuthConfig::new("test-secret", 3600);

    let mut public_routes = PublicRoutes::new();
    for (method, path) in auth::PUBLIC_ROUTES {
        public_routes.add(method, path);
    }

    let auth_middleware = AuthMiddleware::with_public_routes(auth_config.clone(), public_routes);

    let router = Router::new()
        .group("/api/v1/auth", auth::routes())
        .group("/api/v1/projects", projects::routes());

    let app = Rapina::new()
        .with_introspection(false)
        .state(auth_config)
        .middleware(auth_middleware)
        .with_database(DatabaseConfig::new(database_url()))
        .await
        .expect("failed to connect to test database")
        .router(router);

    TestClient::new(app).await
}

fn unique_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

async fn register_and_get_token(client: &TestClient) -> String {
    let email = unique_email();
    let res = client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123",
            "name": "Test User"
        }))
        .send()
        .await;

    let body: serde_json::Value = res.json();
    body["token"].as_str().unwrap().to_string()
}

fn unique_slug() -> String {
    format!("project-{}", Uuid::new_v4())
}

#[tokio::test]
async fn create_project_returns_201() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let slug = unique_slug();

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": "My Project",
            "slug": slug,
            "website_url": "https://example.com"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "My Project");
    assert_eq!(body["slug"], slug);
    assert_eq!(body["website_url"], "https://example.com");
    assert!(body["id"].is_string());
    assert!(body["created_at"].is_string());
}

#[tokio::test]
async fn list_projects_returns_only_own() {
    let client = setup().await;
    let token_a = register_and_get_token(&client).await;
    let token_b = register_and_get_token(&client).await;

    let slug_a = unique_slug();
    let slug_b = unique_slug();

    client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token_a))
        .json(&json!({ "name": "Project A", "slug": slug_a }))
        .send()
        .await;

    client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token_b))
        .json(&json!({ "name": "Project B", "slug": slug_b }))
        .send()
        .await;

    let res = client
        .get("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token_a))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: Vec<serde_json::Value> = res.json();
    assert!(body.iter().all(|p| p["slug"] != slug_b));
    assert!(body.iter().any(|p| p["slug"] == slug_a));
}

#[tokio::test]
async fn get_project_by_pid() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let slug = unique_slug();

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "Get Test", "slug": slug }))
        .send()
        .await;

    let created: serde_json::Value = res.json();
    let pid = created["id"].as_str().unwrap();

    let res = client
        .get(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert_eq!(body["id"], pid);
    assert_eq!(body["name"], "Get Test");
}

#[tokio::test]
async fn update_project_partial() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let slug = unique_slug();

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "Before", "slug": slug }))
        .send()
        .await;

    let created: serde_json::Value = res.json();
    let pid = created["id"].as_str().unwrap();

    let res = client
        .put(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "After" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "After");
    assert_eq!(body["slug"], slug);
}

#[tokio::test]
async fn delete_project_returns_204() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let slug = unique_slug();

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "To Delete", "slug": slug }))
        .send()
        .await;

    let created: serde_json::Value = res.json();
    let pid = created["id"].as_str().unwrap();

    let res = client
        .delete(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = client
        .get(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn slug_uniqueness_returns_409() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let slug = unique_slug();

    client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "First", "slug": slug }))
        .send()
        .await;

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "Second", "slug": slug }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn ownership_enforcement_returns_403() {
    let client = setup().await;
    let token_owner = register_and_get_token(&client).await;
    let token_other = register_and_get_token(&client).await;
    let slug = unique_slug();

    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token_owner))
        .json(&json!({ "name": "Private", "slug": slug }))
        .send()
        .await;

    let created: serde_json::Value = res.json();
    let pid = created["id"].as_str().unwrap();

    let res = client
        .get(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let res = client
        .put(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .json(&json!({ "name": "Hacked" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let res = client
        .delete(&format!("/api/v1/projects/{}", pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn projects_without_token_returns_401() {
    let client = setup().await;

    let res = client.get("/api/v1/projects").send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
