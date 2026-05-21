use crate::services::audio::AudioViewModel;
use crate::services::{
    AppFileSystem, ErrorService, NavigationService, RegistryService, SearchService, SidebarService,
    ThemeService, TodoService,
};
use crate::ui::error_toast::ErrorToast;
use crate::ui::main_content::MainContent;
use crate::ui::navbar::Navbar;
use crate::ui::search::SearchOverlay;
use crate::ui::sidebar::Sidebar;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn App() -> impl IntoView {
    console_error_panic_hook::set_once();

    let registry_service = RegistryService::new();
    let nav_service = NavigationService::new();
    let search_service = SearchService::new();
    let theme_service = ThemeService::new();
    let todo_service = TodoService::new();
    let sidebar_service = SidebarService::new();
    let error_service = ErrorService::new();
    let fs_service = AppFileSystem;
    let audio_vm = AudioViewModel::new();

    provide_context(registry_service);
    provide_context(nav_service);
    provide_context(search_service);
    provide_context(theme_service);
    provide_context(todo_service);
    provide_context(sidebar_service);
    provide_context(error_service);
    provide_context(Arc::new(fs_service));
    provide_context(audio_vm);

    registry_service.load_registry();
    todo_service.load_todos();

    let theme_service_effect = theme_service;
    Effect::new(move |_| {
        let _is_dark = theme_service_effect.is_dark_mode.get();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let class_list = body.class_list();
                    if theme_service_effect.is_dark_mode.get() {
                        let _ = class_list.add_1("dark");
                    } else {
                        let _ = class_list.remove_1("dark");
                    }
                }
            }
        }
    });

    view! {
        <div class="container" class:sidebar-open=move || sidebar_service.is_open.get()>
            <Navbar
                on_navigate=Callback::new(move |id| nav_service.navigate_to(id))
                on_search_toggle=Callback::new(move |_| search_service.toggle_search())
                on_theme_toggle=Callback::new(move |_| theme_service.toggle_theme())
                on_sidebar_toggle=Callback::new(move |_| sidebar_service.toggle_sidebar())
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

            <Sidebar />
            <ErrorToast />
        </div>
    }
}
