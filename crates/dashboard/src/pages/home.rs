use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div style="min-height: 100vh; background: var(--color-bg);">
            <header style="padding: 20px 0;">
                <div class="container" style="display: flex; justify-content: space-between; align-items: center;">
                    <a href="/" style="font-size: 1.25rem; font-weight: 700; color: var(--color-text); text-decoration: none;">"Reeverb"</a>
                    <nav style="display: flex; gap: 16px; align-items: center;">
                        <a href="/login" style="font-size: 0.875rem; color: var(--color-text-secondary);">"Sign in"</a>
                        <a href="/signup" class="btn btn-primary" style="font-size: 0.875rem; padding: 8px 16px;">"Get started"</a>
                    </nav>
                </div>
            </header>

            <main class="container" style="padding-top: 80px; padding-bottom: 80px; text-align: center; max-width: 640px;">
                <h1 style="font-size: 3rem; font-weight: 800; line-height: 1.1; letter-spacing: -0.02em;">
                    "Social proof that "
                    <span style="color: var(--color-primary);">"resonates"</span>
                </h1>
                <p style="margin-top: 20px; font-size: 1.125rem; color: var(--color-text-secondary); line-height: 1.7;">
                    "Collect, manage, and display testimonials. One platform for all your social proof."
                </p>
                <div style="margin-top: 40px; display: flex; gap: 12px; justify-content: center;">
                    <a href="/signup" class="btn btn-primary" style="padding: 12px 28px; font-size: 1rem;">"Start for free"</a>
                </div>
            </main>

            <footer style="border-top: 1px solid var(--color-border); padding: 24px 0;">
                <div class="container" style="text-align: center;">
                    <p style="font-size: 0.8rem; color: var(--color-text-secondary);">"Open source. MIT licensed."</p>
                </div>
            </footer>
        </div>
    }
}
