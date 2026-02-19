use gloo_storage::Storage;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_loading.set(true);

        let email_val = email.get_untracked();
        let password_val = password.get_untracked();

        leptos::task::spawn_local(async move {
            let body = LoginRequest {
                email: email_val,
                password: password_val,
            };

            match api::post::<LoginResponse, _>("/api/v1/auth/login", &body).await {
                Ok(resp) => {
                    let _ = gloo_storage::LocalStorage::raw().set_item("token", &resp.token);
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href("/dashboard");
                    }
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div style="display: flex; align-items: center; justify-content: center; min-height: 100vh; background: var(--color-bg);">
            <div class="card" style="width: 100%; max-width: 400px;">
                <div style="text-align: center; margin-bottom: 32px;">
                    <h1 style="font-size: 1.5rem; font-weight: 700;">"Reeverb"</h1>
                    <p style="color: var(--color-text-secondary); margin-top: 4px;">"Sign in to your account"</p>
                </div>

                {move || error.get().map(|e| view! {
                    <div class="error-message" style="margin-bottom: 16px;">{e}</div>
                })}

                <form on:submit=on_submit>
                    <div style="margin-bottom: 16px;">
                        <label for="email" style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 6px;">"Email"</label>
                        <input
                            id="email"
                            type="email"
                            class="input"
                            placeholder="you@example.com"
                            required=true
                            prop:value=move || email.get()
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                    </div>

                    <div style="margin-bottom: 24px;">
                        <label for="password" style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 6px;">"Password"</label>
                        <input
                            id="password"
                            type="password"
                            class="input"
                            placeholder="Your password"
                            required=true
                            prop:value=move || password.get()
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    <button
                        type="submit"
                        class="btn btn-primary"
                        style="width: 100%;"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Signing in..." } else { "Sign in" }}
                    </button>
                </form>

                <p style="text-align: center; margin-top: 20px; font-size: 0.875rem; color: var(--color-text-secondary);">
                    "Don't have an account? "
                    <a href="/signup">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}
