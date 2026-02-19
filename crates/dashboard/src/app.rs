use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages::{DashboardPage, HomePage, LoginPage, NotFoundPage, SignupPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFoundPage>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/login") view=LoginPage />
                <Route path=path!("/signup") view=SignupPage />
                <Route path=path!("/dashboard") view=DashboardPage />
            </Routes>
        </Router>
    }
}
