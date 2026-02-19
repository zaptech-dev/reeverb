use gloo_storage::Storage;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Clone, Serialize, Deserialize)]
struct Project {
    pid: String,
    name: String,
    slug: String,
}

fn logout() {
    let _ = gloo_storage::LocalStorage::raw().remove_item("token");
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/login");
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let token_exists = gloo_storage::LocalStorage::raw()
        .get_item("token")
        .ok()
        .flatten()
        .is_some();

    if !token_exists {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("/login");
        }
        return view! { <div></div> }.into_any();
    }

    let (projects, set_projects) = signal(None::<Result<Vec<Project>, String>>);
    let (loading, set_loading) = signal(true);

    let fetch_projects = move || {
        set_loading.set(true);
        leptos::task::spawn_local(async move {
            let result = api::get::<Vec<Project>>("/api/v1/projects").await;
            set_projects.set(Some(result));
            set_loading.set(false);
        });
    };

    fetch_projects();

    let (show_form, set_show_form) = signal(false);
    let (new_name, set_new_name) = signal(String::new());
    let (new_slug, set_new_slug) = signal(String::new());
    let (form_error, set_form_error) = signal(Option::<String>::None);

    let on_create = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_form_error.set(None);

        let name = new_name.get_untracked();
        let slug = new_slug.get_untracked();

        leptos::task::spawn_local(async move {
            #[derive(serde::Serialize)]
            struct CreateProject {
                name: String,
                slug: String,
            }

            match api::post::<Project, _>("/api/v1/projects", &CreateProject { name, slug }).await {
                Ok(_) => {
                    set_show_form.set(false);
                    set_new_name.set(String::new());
                    set_new_slug.set(String::new());
                    // Refetch projects
                    let result = api::get::<Vec<Project>>("/api/v1/projects").await;
                    set_projects.set(Some(result));
                }
                Err(e) => set_form_error.set(Some(e)),
            }
        });
    };

    view! {
        <div style="min-height: 100vh; background: var(--color-bg);">
            <header style="background: var(--color-surface); border-bottom: 1px solid var(--color-border); padding: 16px 0;">
                <div class="container" style="display: flex; justify-content: space-between; align-items: center;">
                    <h1 style="font-size: 1.25rem; font-weight: 700;">"Reeverb"</h1>
                    <button
                        class="btn"
                        style="color: var(--color-text-secondary); background: none; border: 1px solid var(--color-border);"
                        on:click=move |_| logout()
                    >
                        "Sign out"
                    </button>
                </div>
            </header>

            <main class="container" style="padding-top: 32px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
                    <h2 style="font-size: 1.5rem; font-weight: 600;">"Projects"</h2>
                    <button
                        class="btn btn-primary"
                        on:click=move |_| set_show_form.update(|v| *v = !*v)
                    >
                        {move || if show_form.get() { "Cancel" } else { "New project" }}
                    </button>
                </div>

                {move || show_form.get().then(|| view! {
                    <div class="card" style="margin-bottom: 24px;">
                        {move || form_error.get().map(|e| view! {
                            <div class="error-message" style="margin-bottom: 16px;">{e}</div>
                        })}
                        <form on:submit=on_create style="display: flex; gap: 12px; align-items: end;">
                            <div style="flex: 1;">
                                <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 6px;">"Name"</label>
                                <input
                                    class="input"
                                    placeholder="My Project"
                                    required=true
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| set_new_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div style="flex: 1;">
                                <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 6px;">"Slug"</label>
                                <input
                                    class="input"
                                    placeholder="my-project"
                                    required=true
                                    prop:value=move || new_slug.get()
                                    on:input=move |ev| set_new_slug.set(event_target_value(&ev))
                                />
                            </div>
                            <button type="submit" class="btn btn-primary">"Create"</button>
                        </form>
                    </div>
                })}

                {move || {
                    if loading.get() && projects.get().is_none() {
                        return view! {
                            <p style="color: var(--color-text-secondary);">"Loading projects..."</p>
                        }.into_any();
                    }

                    match projects.get() {
                        Some(Ok(list)) if list.is_empty() => view! {
                            <div class="card" style="text-align: center; padding: 48px;">
                                <p style="color: var(--color-text-secondary);">"No projects yet. Create your first one."</p>
                            </div>
                        }.into_any(),
                        Some(Ok(list)) => view! {
                            <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px;">
                                {list.into_iter().map(|project| view! {
                                    <div class="card" style="cursor: pointer; transition: border-color 0.15s ease;">
                                        <h3 style="font-weight: 600; margin-bottom: 4px;">{project.name}</h3>
                                        <p style="color: var(--color-text-secondary); font-size: 0.875rem; font-family: var(--font-mono);">
                                            {project.slug}
                                        </p>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <div class="error-message">{e}</div>
                        }.into_any(),
                        None => view! {
                            <p style="color: var(--color-text-secondary);">"Loading projects..."</p>
                        }.into_any(),
                    }
                }}
            </main>
        </div>
    }
    .into_any()
}
