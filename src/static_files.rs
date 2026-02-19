use rapina::context::RequestContext;
use rapina::http::{Response, StatusCode};
use rapina::hyper::Request;
use rapina::hyper::body::Incoming;
use rapina::middleware::{BoxFuture, Middleware, Next};
use rapina::response::BoxBody;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "dist/"]
struct DashboardAssets;

fn mime_for(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    }
}

fn cache_header(path: &str) -> &'static str {
    if path.ends_with(".wasm") || path.ends_with(".js") || path.ends_with(".css") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    }
}

fn serve_file(path: &str) -> Response<BoxBody> {
    use rapina::response::IntoResponse;

    let path = path.trim_start_matches('/');

    if let Some(file) = DashboardAssets::get(path) {
        return Response::builder()
            .status(StatusCode::OK)
            .header("content-type", mime_for(path))
            .header("cache-control", cache_header(path))
            .body(BoxBody::new(file.data.into_owned().into()))
            .unwrap();
    }

    // SPA fallback: serve index.html
    if let Some(index) = DashboardAssets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/html; charset=utf-8")
            .header("cache-control", "no-cache")
            .body(BoxBody::new(index.data.into_owned().into()))
            .unwrap();
    }

    StatusCode::NOT_FOUND.into_response()
}

fn is_api_path(path: &str) -> bool {
    path.starts_with("/api/") || path == "/health" || path.starts_with("/__rapina")
}

pub struct DashboardMiddleware;

impl Middleware for DashboardMiddleware {
    fn handle<'a>(
        &'a self,
        req: Request<Incoming>,
        _ctx: &'a RequestContext,
        next: Next<'a>,
    ) -> BoxFuture<'a, Response<BoxBody>> {
        Box::pin(async move {
            let path = req.uri().path().to_string();

            if is_api_path(&path) {
                return next.run(req).await;
            }

            serve_file(&path)
        })
    }
}
