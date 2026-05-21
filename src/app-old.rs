use crate::services::audio::AudioViewModel;
use crate::services::{NavigationService, RegistryService, TodoService};
use crate::ui::page_renderer::PageRenderer;
use crate::ui::welcome::WelcomeDashboard;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let is_dark = RwSignal::new(true);
    let active_page = RwSignal::new(None::<&'static str>);

    Effect::new(move |_| {
        let dark = is_dark.get();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let class_list = body.class_list();
                    if dark {
                        let _ = class_list.add_1("dark");
                    } else {
                        let _ = class_list.remove_1("dark");
                    }
                }
            }
        }
    });

    let registry_service = RegistryService::new();
    let nav_service = NavigationService::new();
    let todo_service = TodoService::new();
    let audio_vm = AudioViewModel::new();

    provide_context(registry_service);
    provide_context(nav_service);
    provide_context(todo_service);
    provide_context(audio_vm);

    registry_service.load_registry();

    let on_navigate = Callback::new(move |id: &'static str| active_page.set(Some(id)));
    let on_back = Callback::new(move |_| active_page.set(None));

    view! {
        <div>
            <nav class="navbar">
                <button
                    class="navbar-brand"
                    style="background: none; border: none; cursor: pointer;"
                    on:click=move |_| on_back.run(())
                >
                    "RigorStarter"
                </button>
                <div class="navbar-actions" style="display: flex; gap: 0.5rem; align-items: center;">
                    <button class="btn-icon" on:click=move |_| is_dark.update(|d| *d = !*d)>
                        {move || if is_dark.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </nav>

            {move || match active_page.get() {
                None => view! { <WelcomeDashboard on_navigate /> }.into_any(),
                Some(id) => view! { <PageRenderer id on_back /> }.into_any(),
            }}
        </div>
    }
}
