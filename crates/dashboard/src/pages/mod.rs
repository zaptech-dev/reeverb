mod dashboard;
mod home;
mod login;
mod signup;

pub use dashboard::DashboardPage;
pub use home::HomePage;
pub use login::LoginPage;
pub use signup::SignupPage;

use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; min-height: 100vh;">
            <div style="text-align: center;">
                <h1 style="font-size: 3rem; font-weight: 700; color: var(--color-text);">"404"</h1>
                <p style="color: var(--color-text-secondary); margin-top: 8px;">"Page not found"</p>
                <a href="/dashboard" style="display: inline-block; margin-top: 16px;">"Back to dashboard"</a>
            </div>
        </div>
    }
}
