use crate::core::models::RegistryItem;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[derive(Clone, Copy)]
pub struct RegistryService {
    pub items: ReadSignal<Vec<RegistryItem>>,
    pub set_items: WriteSignal<Vec<RegistryItem>>,
    pub is_loading: ReadSignal<bool>,
    pub set_loading: WriteSignal<bool>,
}

impl RegistryService {
    pub fn new() -> Self {
        let (items, set_items) = signal(Vec::<RegistryItem>::new());
        let (is_loading, set_loading) = signal(true);

        Self {
            items,
            set_items,
            is_loading,
            set_loading,
        }
    }

    pub fn load_registry(&self) {
        let set_items = self.set_items;
        let set_loading = self.set_loading;

        spawn_local(async move {
            let result = invoke("get_registry", JsValue::NULL).await;
            match result {
                Ok(val) => {
                    if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<RegistryItem>>(val) {
                        set_items.set(items);
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Failed to fetch registry: {:?}", e);
                }
            }
            set_loading.set(false);
        });
    }
}

#[derive(Clone, Copy)]
pub struct NavigationService {
    pub active_demo: ReadSignal<Option<String>>,
    pub set_active_demo: WriteSignal<Option<String>>,
}

impl NavigationService {
    pub fn new() -> Self {
        let (active_demo, set_active_demo) = signal(Option::<String>::None);
        Self {
            active_demo,
            set_active_demo,
        }
    }

    pub fn navigate_to(&self, id: Option<String>) {
        self.set_active_demo.set(id);
    }
}

#[derive(Clone, Copy)]
pub struct SearchService {
    pub query: ReadSignal<String>,
    pub set_query: WriteSignal<String>,
    pub is_open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
}

impl SearchService {
    pub fn new() -> Self {
        let (query, set_query) = signal(String::new());
        let (is_open, set_open) = signal(false);
        Self {
            query,
            set_query,
            is_open,
            set_open,
        }
    }

    pub fn toggle_search(&self) {
        self.set_open.update(|v| *v = !*v);
    }

    pub fn close_search(&self) {
        self.set_open.set(false);
        self.set_query.set(String::new());
    }
}

#[derive(Clone, Copy)]
pub struct ThemeService {
    pub is_dark_mode: ReadSignal<bool>,
    pub set_dark_mode: WriteSignal<bool>,
}

impl ThemeService {
    pub fn new() -> Self {
        let (is_dark_mode, set_dark_mode) = signal(false);
        Self {
            is_dark_mode,
            set_dark_mode,
        }
    }

    pub fn toggle_theme(&self) {
        self.set_dark_mode.update(|v| *v = !*v);
    }
}
