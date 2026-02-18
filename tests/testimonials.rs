use rapina::auth::{AuthMiddleware, PublicRoutes};
use rapina::database::DatabaseConfig;
use rapina::prelude::*;
use rapina::testing::TestClient;
use serde_json::json;
use tokio::sync::OnceCell;
use uuid::Uuid;

use reeverb::api::v1::auth;
use reeverb::api::v1::projects;
use reeverb::api::v1::testimonials;
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
        .group("/api/v1/projects", projects::routes())
        .group("/api/v1/projects", testimonials::project_routes())
        .group("/api/v1/testimonials", testimonials::routes());

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

async fn create_project(client: &TestClient, token: &str) -> String {
    let slug = unique_slug();
    let res = client
        .post("/api/v1/projects")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "Test Project", "slug": slug }))
        .send()
        .await;

    let body: serde_json::Value = res.json();
    body["id"].as_str().unwrap().to_string()
}

async fn create_test_testimonial(client: &TestClient, token: &str, project_pid: &str) -> String {
    let res = client
        .post(&format!("/api/v1/projects/{}/testimonials", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "author_name": "Jane Doe",
            "content": "Great product!",
            "rating": 5,
            "author_email": "jane@example.com"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    let body: serde_json::Value = res.json();
    body["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn create_testimonial_returns_201() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    let res = client
        .post(&format!("/api/v1/projects/{}/testimonials", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "author_name": "Jane Doe",
            "content": "Amazing service!",
            "rating": 5,
            "author_email": "jane@example.com",
            "author_title": "CTO",
            "author_company": "Acme Inc"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: serde_json::Value = res.json();
    assert_eq!(body["author_name"], "Jane Doe");
    assert_eq!(body["content"], "Amazing service!");
    assert_eq!(body["rating"], 5);
    assert_eq!(body["type"], "text");
    assert_eq!(body["project_id"], project_pid);
    assert_eq!(body["is_approved"], false);
    assert_eq!(body["is_featured"], false);
    assert!(body["id"].is_string());
    assert!(body["created_at"].is_string());
}

#[tokio::test]
async fn list_testimonials_for_project() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .get(&format!("/api/v1/projects/{}/testimonials", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: Vec<serde_json::Value> = res.json();
    assert!(!body.is_empty());
    assert_eq!(body[0]["author_name"], "Jane Doe");
}

#[tokio::test]
async fn list_testimonials_with_approved_filter() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .get(&format!(
            "/api/v1/projects/{}/testimonials?is_approved=true",
            project_pid
        ))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: Vec<serde_json::Value> = res.json();
    assert!(body.is_empty());
}

#[tokio::test]
async fn get_testimonial_by_pid() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .get(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert_eq!(body["id"], testimonial_pid);
    assert_eq!(body["author_name"], "Jane Doe");
    assert_eq!(body["project_id"], project_pid);
}

#[tokio::test]
async fn update_testimonial_partial() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .put(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "author_name": "John Smith" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert_eq!(body["author_name"], "John Smith");
    assert_eq!(body["content"], "Great product!");
}

#[tokio::test]
async fn delete_testimonial_returns_204() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .delete(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = client
        .get(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn approve_toggle() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .post(&format!("/api/v1/testimonials/{}/approve", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body: serde_json::Value = res.json();
    assert_eq!(body["is_approved"], true);

    let res = client
        .post(&format!("/api/v1/testimonials/{}/approve", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body: serde_json::Value = res.json();
    assert_eq!(body["is_approved"], false);
}

#[tokio::test]
async fn feature_toggle() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;

    let res = client
        .post(&format!("/api/v1/testimonials/{}/feature", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body: serde_json::Value = res.json();
    assert_eq!(body["is_featured"], true);

    let res = client
        .post(&format!("/api/v1/testimonials/{}/feature", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body: serde_json::Value = res.json();
    assert_eq!(body["is_featured"], false);
}

#[tokio::test]
async fn ownership_enforcement_returns_403() {
    let client = setup().await;
    let token_owner = register_and_get_token(&client).await;
    let token_other = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token_owner).await;
    let testimonial_pid = create_test_testimonial(&client, &token_owner, &project_pid).await;

    let res = client
        .get(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let res = client
        .put(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .json(&json!({ "author_name": "Hacked" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let res = client
        .delete(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let res = client
        .get(&format!("/api/v1/projects/{}/testimonials", project_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn testimonials_without_token_returns_401() {
    let client = setup().await;

    let res = client.get("/api/v1/testimonials/some-id").send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
