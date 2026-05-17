use leptos::prelude::*;

#[component]
pub fn Navbar(on_brand_click: Callback<()>, on_search_toggle: Callback<()>) -> impl IntoView {
    view! {
        <nav class="navbar">
            <button class="nav-brand-btn" on:click=move |_| on_brand_click.run(())>"Component Library"</button>
            <button class="search-toggle" on:click=move |_| on_search_toggle.run(())>
                "🔍 Search"
            </button>
        </nav>
    }
}
