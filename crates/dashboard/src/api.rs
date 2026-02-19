use gloo_net::http::{Request, RequestBuilder};
use gloo_storage::Storage;
use serde::de::DeserializeOwned;

fn get_token() -> Option<String> {
    gloo_storage::LocalStorage::raw()
        .get_item("token")
        .ok()
        .flatten()
}

fn clear_token_and_redirect() {
    let _ = gloo_storage::LocalStorage::raw().remove_item("token");
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/login");
    }
}

fn with_auth(builder: RequestBuilder) -> RequestBuilder {
    match get_token() {
        Some(token) => builder.header("Authorization", &format!("Bearer {}", token)),
        None => builder,
    }
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let resp = with_auth(Request::get(path))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        clear_token_and_redirect();
        return Err("Unauthorized".into());
    }

    if !resp.ok() {
        return Err(format!("Request failed: {}", resp.status()));
    }

    resp.json::<T>().await.map_err(|e| e.to_string())
}

pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    let resp = with_auth(Request::post(path))
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        clear_token_and_redirect();
        return Err("Unauthorized".into());
    }

    if !resp.ok() {
        return Err(format!("Request failed: {}", resp.status()));
    }

    resp.json::<T>().await.map_err(|e| e.to_string())
}
