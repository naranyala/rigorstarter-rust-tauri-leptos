use crate::components::main_content::MainContent;
use crate::components::navbar::Navbar;
use crate::components::search::SearchOverlay;
use crate::services::{
    NavigationService, RegistryService, SearchService, ThemeService, TodoService,
};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    console_error_panic_hook::set_once();

    // Dependency Injection: Initialize Services
    let registry_service = RegistryService::new();
    let nav_service = NavigationService::new();
    let search_service = SearchService::new();
    let theme_service = ThemeService::new();
    let todo_service = TodoService::new();

    // Provide services to the component tree via Context
    provide_context(registry_service);
    provide_context(nav_service);
    provide_context(search_service);
    provide_context(theme_service);
    provide_context(todo_service);

    // Initial data load
    registry_service.load_registry();
    todo_service.load_todos();

    // Theme Effect: Sync service state with DOM body class
    Effect::new(move |_| {
        let is_dark = theme_service.is_dark_mode.get();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    if is_dark {
                        body.class_list().add_1("dark").unwrap();
                    } else {
                        body.class_list().remove_1("dark").unwrap();
                    }
                }
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    loader.remove();
                }
            }
        }
    });

    view! {
        <div class="container">
            <Navbar
                on_navigate=Callback::new(move |id| nav_service.navigate_to(id))
                on_search_toggle=Callback::new(move |_| search_service.toggle_search())
                on_theme_toggle=Callback::new(move |_| theme_service.toggle_theme())
                is_dark_mode=theme_service.is_dark_mode
            />

            <SearchOverlay
                items=registry_service.items
                query=search_service.query
                is_open=search_service.is_open
                on_query_change=Callback::new(move |q| search_service.set_query.set(q))
                on_close=Callback::new(move |_| search_service.close_search())
                on_item_select=Callback::new(move |id| {
                    nav_service.navigate_to(Some(id));
                    search_service.close_search();
                })
            />

            <ErrorBoundary fallback=move |_| view! { <div class="error-msg">"A critical error occurred in the main content. Please try selecting another component."</div> }>
                <MainContent />
            </ErrorBoundary>
        </div>
    }
}
