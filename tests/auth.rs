use std::sync::Once;

use rapina::auth::{AuthMiddleware, PublicRoutes};
use rapina::database::DatabaseConfig;
use rapina::prelude::*;
use rapina::testing::TestClient;
use serde_json::json;
use uuid::Uuid;

use reeverb::api::v1::auth;
use reeverb::db::migrations::Migrator;

static MIGRATIONS: Once = Once::new();

fn database_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests")
}

async fn run_migrations_once() {
    let needs_migration = std::sync::Arc::new(tokio::sync::Notify::new());
    let done = needs_migration.clone();

    let mut ran = false;
    MIGRATIONS.call_once(|| {
        ran = true;
    });

    if ran {
        let url = database_url();
        let config = DatabaseConfig::new(&url);
        let conn = config.connect().await.expect("failed to connect");

        use rapina::sea_orm_migration::MigratorTrait;
        Migrator::up(&conn, None)
            .await
            .expect("failed to run migrations");

        done.notify_waiters();
    }
}

async fn setup() -> TestClient {
    run_migrations_once().await;

    let auth_config = AuthConfig::new("test-secret", 3600);

    let mut public_routes = PublicRoutes::new();
    for (method, path) in auth::PUBLIC_ROUTES {
        public_routes.add(method, path);
    }

    let auth_middleware = AuthMiddleware::with_public_routes(auth_config.clone(), public_routes);

    let router = Router::new().group("/api/v1/auth", auth::routes());

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

#[tokio::test]
async fn register_returns_token_and_user() {
    let client = setup().await;
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

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert!(body["token"].is_string());
    assert!(body["expires_in"].is_number());
    assert_eq!(body["user"]["email"], email);
    assert_eq!(body["user"]["name"], "Test User");
}

#[tokio::test]
async fn register_duplicate_email_returns_conflict() {
    let client = setup().await;
    let email = unique_email();

    client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .await;

    let res = client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "other"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn login_returns_token() {
    let client = setup().await;
    let email = unique_email();

    client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .await;

    let res = client
        .post("/api/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body: serde_json::Value = res.json();
    assert!(body["token"].is_string());
    assert_eq!(body["user"]["email"], email);
}

#[tokio::test]
async fn login_wrong_password_returns_unauthorized() {
    let client = setup().await;
    let email = unique_email();

    client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123"
        }))
        .send()
        .await;

    let res = client
        .post("/api/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": "wrong"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_nonexistent_email_returns_unauthorized() {
    let client = setup().await;

    let res = client
        .post("/api/v1/auth/login")
        .json(&json!({
            "email": "nobody@example.com",
            "password": "password123"
        }))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_with_valid_token_returns_user() {
    let client = setup().await;
    let email = unique_email();

    let res = client
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": email,
            "password": "password123",
            "name": "Auth User"
        }))
        .send()
        .await;

    let body: serde_json::Value = res.json();
    let token = body["token"].as_str().unwrap();

    let res = client
        .get("/api/v1/auth/me")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let user: serde_json::Value = res.json();
    assert_eq!(user["email"], email);
    assert_eq!(user["name"], "Auth User");
}

#[tokio::test]
async fn me_without_token_returns_unauthorized() {
    let client = setup().await;

    let res = client.get("/api/v1/auth/me").send().await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_with_invalid_token_returns_unauthorized() {
    let client = setup().await;

    let res = client
        .get("/api/v1/auth/me")
        .header("Authorization", "Bearer invalid.token.here")
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
