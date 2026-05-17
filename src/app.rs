use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::RegistryItem;
use crate::components::navbar::Navbar;
use crate::components::search::SearchOverlay;
use crate::components::main_content::MainContent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[component]
pub fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    let (search_query, set_search_query) = signal(String::new());
    let (is_search_open, set_is_search_open) = signal(false);
    let (active_demo, set_active_demo) = signal(Option::<String>::None);
    let (registry, set_registry) = signal(Vec::<RegistryItem>::new());
    let (is_loading_registry, set_is_loading_registry) = signal(true);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    let _ = loader.remove();
                }
            }
        }
    });

    spawn_local(async move {
        let result = invoke("get_registry", JsValue::NULL).await;
        match result {
            Ok(val) => {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<RegistryItem>>(val) {
                    set_registry.set(items);
                }
            },
            Err(e) => {
                leptos::logging::error!("Failed to fetch registry: {:?}", e);
            }
        }
        set_is_loading_registry.set(false);
    });

    view! {
        <div class="container">
            <Navbar 
                on_brand_click=Callback::new(move |_| set_active_demo.set(None))
                on_search_toggle=Callback::new(move |_| set_is_search_open.update(|v| *v = !*v))
            />

            <SearchOverlay 
                is_open=is_search_open
                set_is_open=set_is_search_open
                search_query=search_query
                set_search_query=set_search_query
                registry=registry
                set_active_demo=Callback::new(move |id| set_active_demo.set(id))
            />

            <ErrorBoundary fallback=move |_| view! { <div class="error-msg">"A critical error occurred in the main content. Please try selecting another component."</div> }>
                <MainContent 
                    active_demo=active_demo
                    set_active_demo=Callback::new(move |id| set_active_demo.set(id))
                    registry=registry
                    is_loading_registry=is_loading_registry
                />
            </ErrorBoundary>
        </div>
    }
}
