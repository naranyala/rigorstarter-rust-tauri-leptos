use crate::services::audio::AudioViewModel;
use crate::services::{NavigationService, RegistryService};
use crate::ui::stdlib::components::nav_category::NavCategory;
use crate::ui::stdlib::utils::event_target_value;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub type ActivePage = RwSignal<Option<&'static str>>;

use crate::ui::page_registry::PAGES;

/// Renders all pages statically; the active page is shown via `style:display`.
/// This avoids Leptos 0.7's `{move || ...}` reactive-block view-reconciliation
/// initialization issue that causes reactive updates to not propagate on first render.
#[component]
fn PageContent(
    active_page: RwSignal<Option<&'static str>>,
    search: RwSignal<String>,
) -> impl IntoView {
    let render_page = move |id: &'static str| {
        PAGES
            .iter()
            .find(|page| page.info.id == id)
            .map(|page| (page.component)())
            .unwrap_or_else(|| view! { <div>"Not Found"</div> }.into_any())
    };

    view! {
        <main class="main-content">
            {move || {
                let active = active_page.get();
                if let Some(id) = active {
                    view! {
                        <div class="page-wrapper">
                            {render_page(id)}
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="empty-state">"Select a page from the sidebar"</div> }.into_any()
                }
            }}
            <span
                style=move || {
                    let q = search.get().to_lowercase();
                    let display = if q.is_empty() || PAGES.iter().any(|page| page.info.name.to_lowercase().contains(&q) || page.info.category.to_lowercase().contains(&q)) {
                        "none"
                    } else {
                        "block"
                    };
                    format!("display: {}; color: var(--text-muted); font-size: 0.8rem; padding: 0.5rem;", display)
                }
            >
                "No results found"
            </span>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let active_page = RwSignal::new(Some("welcome"));
    let search = RwSignal::new(String::new());
    let is_dark = RwSignal::new(false);
    let sidebar_open = RwSignal::new(false);

    provide_context(active_page);

    Effect::new(move |_| {
        let dark = is_dark.get();
        leptos::logging::log!("theme effect: dark={}", dark);
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.class_list().toggle_with_force("dark", dark);
        }
    });

    Effect::new(move |_| {
        let _ = active_page.get();
        if let Some(window) = web_sys::window() {
            if let Some(body) = window.document().and_then(|d| d.body()) {
                let _ = body.offset_height();
            }
        }
    });

    // Force a synchronous reflow and DOM paint on mount to wake up the browser rendering pipeline.
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    loader.remove();
                }

                let w = window.clone();
                let ap = active_page;
                let closure = Closure::once_into_js(move || {
                    if let Some(body) = w.document().and_then(|d| d.body()) {
                        let _ = body.offset_height();
                        if let Ok(event) = web_sys::Event::new("resize") {
                            let _ = w.dispatch_event(&event);
                        }
                    }
                    ap.update(|_| {});
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.unchecked_ref(),
                    100,
                );
            }
        }
    });

    // Keyboard shortcut Ctrl+Shift+R to force page reload for debugging
    if let Some(w) = web_sys::window() {
        let w_for_handler = w.clone();
        let handler =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if ev.ctrl_key() && ev.shift_key() && ev.key() == "R" {
                    ev.prevent_default();
                    leptos::logging::log!("Ctrl+Shift+R: reloading page");
                    let _ = w_for_handler.location().reload();
                }
            });
        let _ = w.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        std::mem::forget(handler);
    }

    let registry_service = RegistryService::new();
    let nav_service = NavigationService::new();
    let audio_vm = AudioViewModel::new();

    provide_context(registry_service);
    provide_context(nav_service);
    provide_context(audio_vm);

    registry_service.load_registry();

    // Pre-compute page groupings once (static).
    let groups = crate::ui::page_registry::grouped_pages();

    view! {
        <div class="app-container" class:sidebar-open=move || sidebar_open.get()>
            <div class="sidebar-overlay" on:click=move |_| sidebar_open.set(false) />
            <nav class="sidebar">
                <div style="display: flex; align-items: center; justify-content: space-between; padding-right: 1rem;">
                    <div class="sidebar-brand" style="cursor: pointer;" on:click=move |_| {
                        search.set(String::new());
                        active_page.set(Some("welcome"));
                    }>
                        "RigorStarter"
                    </div>
                    <button class="btn-icon" style="font-size: 1.2rem; background: none; border: none; cursor: pointer;" on:click=move |_| {
                        leptos::logging::log!("Theme toggle clicked");
                        is_dark.update(|d| *d = !*d);
                    }>
                        {move || if is_dark.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
                <div style="padding: 0 1rem 1.5rem 1rem;">
                    <div style="position: relative; display: flex; align-items: center;">
                        <input
                            type="text"
                            placeholder="Search pages..."
                            style="width: 100%; padding: 0.6rem 2.5rem 0.6rem 0.75rem; border: 1px solid var(--border-color); border-radius: 0.5rem; background: var(--secondary-bg); color: var(--text-main); font-size: 0.9rem; outline: none; box-sizing: border-box;"
                            on:input=move |ev| {
                                search.set(event_target_value(&ev));
                            }
                        />
                        {move || if search.get().is_empty() {
                            view! { <div style="display: none;">"✕"</div> }.into_any()
                        } else {
                            view! {
                                <span
                                    style="position: absolute; right: 0.75rem; cursor: pointer; color: var(--text-muted); font-size: 0.8rem; user-select: none;"
                                    on:click=move |_| {
                                        search.set(String::new());
                                    }
                                >
                                    "✕"
                                </span>
                            }.into_any()
                        }}
                    </div>
                </div>
                <div class="sidebar-content">
                    <div class="sidebar-nav">
                        {groups.iter().map(|(cat, pages)| {
                            view! { <NavCategory cat=*cat pages=pages.clone() search=search active_page=active_page sidebar_open=sidebar_open /> }
                        }).collect_view()}
                    </div>
                </div>
            </nav>

            <div class="mobile-header">
                <button class="hamburger-menu" on:click=move |_| sidebar_open.update(|v| *v = !*v)>
                    "☰"
                </button>
                <div class="mobile-brand">"RigorStarter"</div>
                <button class="btn-icon" on:click=move |_| {
                    is_dark.update(|d| *d = !*d);
                }>
                    {move || if is_dark.get() { "☀️" } else { "🌙" }}
                </button>
            </div>

            <PageContent active_page search />
        </div>
    }
}
