use rapina::auth::{AuthMiddleware, PublicRoutes};
use rapina::database::DatabaseConfig;
use rapina::prelude::*;
use rapina::testing::TestClient;
use serde_json::json;
use tokio::sync::OnceCell;
use uuid::Uuid;

use reeverb::api::v1::auth;
use reeverb::api::v1::projects;
use reeverb::api::v1::tags;
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
        .group("/api/v1/projects", tags::project_routes())
        .group("/api/v1/tags", tags::routes())
        .group("/api/v1/testimonials", testimonials::routes())
        .group("/api/v1/testimonials", tags::testimonial_tag_routes());

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

async fn create_test_tag(
    client: &TestClient,
    token: &str,
    project_pid: &str,
    name: &str,
    color: Option<&str>,
) -> String {
    let mut payload = json!({ "name": name });
    if let Some(c) = color {
        payload["color"] = json!(c);
    }

    let res = client
        .post(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    let body: serde_json::Value = res.json();
    body["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn create_tag_returns_201() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    let res = client
        .post(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "bug", "color": "#ff0000" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "bug");
    assert_eq!(body["color"], "#ff0000");
    assert_eq!(body["project_id"], project_pid);
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn create_duplicate_name_returns_409() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    create_test_tag(&client, &token, &project_pid, "duplicate", None).await;

    let res = client
        .post(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "duplicate" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn list_tags_for_project() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    create_test_tag(&client, &token, &project_pid, "feature", Some("#00ff00")).await;

    let res = client
        .get(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: Vec<serde_json::Value> = res.json();
    assert!(!body.is_empty());
    let found = body.iter().any(|t| t["name"] == "feature");
    assert!(found);
}

#[tokio::test]
async fn update_tag_partial() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let tag_pid = create_test_tag(&client, &token, &project_pid, "old-name", Some("#000000")).await;

    let res = client
        .put(&format!("/api/v1/tags/{}", tag_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "color": "#ffffff" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "old-name");
    assert_eq!(body["color"], "#ffffff");
}

#[tokio::test]
async fn update_tag_name_to_existing_returns_409() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;

    create_test_tag(&client, &token, &project_pid, "taken-name", None).await;
    let tag_pid = create_test_tag(&client, &token, &project_pid, "other-name", None).await;

    let res = client
        .put(&format!("/api/v1/tags/{}", tag_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": "taken-name" }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn delete_tag_returns_204() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let tag_pid = create_test_tag(&client, &token, &project_pid, "to-delete", None).await;

    let res = client
        .delete(&format!("/api/v1/tags/{}", tag_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify it's gone from list
    let res = client
        .get(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    let body: Vec<serde_json::Value> = res.json();
    let found = body.iter().any(|t| t["id"] == tag_pid);
    assert!(!found);
}

#[tokio::test]
async fn set_testimonial_tags() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;
    let tag1 = create_test_tag(&client, &token, &project_pid, "tag-a", None).await;
    let tag2 = create_test_tag(&client, &token, &project_pid, "tag-b", None).await;

    let res = client
        .put(&format!("/api/v1/testimonials/{}/tags", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "tag_ids": [tag1, tag2] }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    let tags = body["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn set_testimonial_tags_empty_clears() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;
    let tag1 = create_test_tag(&client, &token, &project_pid, "clear-me", None).await;

    // Set one tag
    client
        .put(&format!("/api/v1/testimonials/{}/tags", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "tag_ids": [tag1] }))
        .send()
        .await;

    // Clear all tags
    let res = client
        .put(&format!("/api/v1/testimonials/{}/tags", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "tag_ids": [] }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    let tags = body["tags"].as_array().unwrap();
    assert!(tags.is_empty());
}

#[tokio::test]
async fn set_tags_from_different_project_fails() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project1 = create_project(&client, &token).await;
    let project2 = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project1).await;
    let foreign_tag = create_test_tag(&client, &token, &project2, "foreign", None).await;

    let res = client
        .put(&format!("/api/v1/testimonials/{}/tags", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "tag_ids": [foreign_tag] }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn testimonial_get_includes_tags() {
    let client = setup().await;
    let token = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token).await;
    let testimonial_pid = create_test_testimonial(&client, &token, &project_pid).await;
    let tag_pid = create_test_tag(&client, &token, &project_pid, "included", Some("#abcdef")).await;

    client
        .put(&format!("/api/v1/testimonials/{}/tags", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "tag_ids": [tag_pid] }))
        .send()
        .await;

    let res = client
        .get(&format!("/api/v1/testimonials/{}", testimonial_pid))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    let tags = body["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0]["name"], "included");
    assert_eq!(tags[0]["color"], "#abcdef");
}

#[tokio::test]
async fn ownership_enforcement_returns_403() {
    let client = setup().await;
    let token_owner = register_and_get_token(&client).await;
    let token_other = register_and_get_token(&client).await;
    let project_pid = create_project(&client, &token_owner).await;
    let tag_pid = create_test_tag(&client, &token_owner, &project_pid, "owned", None).await;

    // List tags
    let res = client
        .get(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // Create tag
    let res = client
        .post(&format!("/api/v1/projects/{}/tags", project_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .json(&json!({ "name": "hacked" }))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // Update tag
    let res = client
        .put(&format!("/api/v1/tags/{}", tag_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .json(&json!({ "name": "hacked" }))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // Delete tag
    let res = client
        .delete(&format!("/api/v1/tags/{}", tag_pid))
        .header("Authorization", &format!("Bearer {}", token_other))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn tags_without_token_returns_401() {
    let client = setup().await;

    let res = client.get("/api/v1/tags/some-id").send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
