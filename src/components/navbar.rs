use leptos::prelude::*;

#[component]
pub fn Navbar(
    on_brand_click: Callback<()>,
    on_search_toggle: Callback<()>,
    on_theme_toggle: Callback<()>,
    is_dark_mode: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <nav class="navbar">
            <button class="nav-brand-btn" on:click=move |_| on_brand_click.run(())>"Component Library"</button>
            <div class="nav-actions">
                <button class="search-toggle" on:click=move |_| on_search_toggle.run(())>
                    "🔍 Search"
                </button>
                <button class="theme-toggle" on:click=move |_| on_theme_toggle.run(())>
                    {move || if is_dark_mode.get() { "☀️" } else { "🌙" }}
                </button>
            </div>
        </nav>
    }
}
