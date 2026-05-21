use leptos::prelude::*;

#[component]
pub fn Navbar(
    on_navigate: Callback<Option<String>>,
    on_search_toggle: Callback<()>,
    on_theme_toggle: Callback<()>,
    on_sidebar_toggle: Callback<()>,
    is_dark_mode: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <nav class="navbar">
            <button class="nav-brand-btn" on:click=move |_| on_navigate.run(None)>
                "Component Library"
            </button>
            <div class="nav-actions">
                <button class="search-toggle" on:click=move |_| on_search_toggle.run(())>
                    "🔍 Search"
                </button>
                <button class="sidebar-toggle" on:click=move |_| on_sidebar_toggle.run(())>
                    "🛠️ Details"
                </button>
                <button class="theme-toggle" on:click=move |_| on_theme_toggle.run(())>
                    {move || if is_dark_mode.get() { "☀️" } else { "🌙" }}
                </button>
            </div>
        </nav>
    }
}
