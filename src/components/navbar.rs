use crate::services::{NavigationService, SearchService, ThemeService};
use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let nav_service =
        use_context::<NavigationService>().expect("NavigationService should be provided");
    let search_service = use_context::<SearchService>().expect("SearchService should be provided");
    let theme_service = use_context::<ThemeService>().expect("ThemeService should be provided");

    view! {
        <nav class="navbar">
            <button class="nav-brand-btn" on:click=move |_| nav_service.navigate_to(None)>
                "Component Library"
            </button>
            <div class="nav-actions">
                <button class="search-toggle" on:click=move |_| search_service.toggle_search()>
                    "🔍 Search"
                </button>
                <button class="theme-toggle" on:click=move |_| theme_service.toggle_theme()>
                    {move || if theme_service.is_dark_mode.get() { "☀️" } else { "🌙" }}
                </button>
            </div>
        </nav>
    }
}
