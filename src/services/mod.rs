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

pub mod todo;
pub use todo::TodoService;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_navigation_service_initial_state() {
        let _rt = setup_runtime();
        let nav = NavigationService::new();
        assert!(nav.active_demo.get().is_none());
    }

    #[test]
    fn test_navigation_service_navigate_to_some() {
        let _rt = setup_runtime();
        let nav = NavigationService::new();
        nav.navigate_to(Some("accordion".to_string()));
        assert_eq!(nav.active_demo.get().unwrap(), "accordion");
    }

    #[test]
    fn test_navigation_service_navigate_to_none() {
        let _rt = setup_runtime();
        let nav = NavigationService::new();
        nav.navigate_to(Some("drawer".to_string()));
        nav.navigate_to(None);
        assert!(nav.active_demo.get().is_none());
    }

    #[test]
    fn test_navigation_service_multiple_navigations() {
        let _rt = setup_runtime();
        let nav = NavigationService::new();
        for id in &["a", "b", "c", "d", "e"] {
            nav.navigate_to(Some((*id).to_string()));
            assert_eq!(nav.active_demo.get().unwrap(), *id);
        }
    }

    #[test]
    fn test_search_service_initial_state() {
        let _rt = setup_runtime();
        let search = SearchService::new();
        assert!(search.query.get().is_empty());
        assert!(!search.is_open.get());
    }

    #[test]
    fn test_search_service_toggle_open() {
        let _rt = setup_runtime();
        let search = SearchService::new();
        assert!(!search.is_open.get());
        search.toggle_search();
        assert!(search.is_open.get());
        search.toggle_search();
        assert!(!search.is_open.get());
    }

    #[test]
    fn test_search_service_close_resets_query() {
        let _rt = setup_runtime();
        let search = SearchService::new();
        search.set_query.set("test query".to_string());
        search.toggle_search();
        assert!(search.is_open.get());
        search.close_search();
        assert!(!search.is_open.get());
        assert!(search.query.get().is_empty());
    }

    #[test]
    fn test_search_service_query_updates() {
        let _rt = setup_runtime();
        let search = SearchService::new();
        search.set_query.set("Accordion".to_string());
        assert_eq!(search.query.get(), "Accordion");
        search.set_query.set(String::new());
        assert!(search.query.get().is_empty());
    }

    #[test]
    fn test_search_service_toggle_multiple_times() {
        let _rt = setup_runtime();
        let search = SearchService::new();
        for i in 0..10 {
            search.toggle_search();
            assert_eq!(
                search.is_open.get(),
                i % 2 == 0,
                "Failed at toggle {}",
                i + 1
            );
        }
    }

    #[test]
    fn test_theme_service_initial_state() {
        let _rt = setup_runtime();
        let theme = ThemeService::new();
        assert!(!theme.is_dark_mode.get());
    }

    #[test]
    fn test_theme_service_toggle() {
        let _rt = setup_runtime();
        let theme = ThemeService::new();
        theme.toggle_theme();
        assert!(theme.is_dark_mode.get());
        theme.toggle_theme();
        assert!(!theme.is_dark_mode.get());
    }

    #[test]
    fn test_theme_service_toggle_many() {
        let _rt = setup_runtime();
        let theme = ThemeService::new();
        for i in 0..100 {
            theme.toggle_theme();
            assert_eq!(
                theme.is_dark_mode.get(),
                i % 2 == 0,
                "Failed at toggle {}",
                i + 1
            );
        }
    }

    #[test]
    fn test_registry_service_initial_state() {
        let _rt = setup_runtime();
        let reg = RegistryService::new();
        assert!(reg.items.get().is_empty());
        assert!(reg.is_loading.get());
    }

    #[test]
    fn test_registry_service_set_items() {
        let _rt = setup_runtime();
        let reg = RegistryService::new();
        let items = vec![RegistryItem {
            name: "Test".into(),
            id: "t1".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        }];
        reg.set_items.set(items.clone());
        assert_eq!(reg.items.get().len(), 1);
        assert_eq!(reg.items.get()[0].name, "Test");
    }

    #[test]
    fn test_registry_service_loading_state() {
        let _rt = setup_runtime();
        let reg = RegistryService::new();
        assert!(reg.is_loading.get());
        reg.set_loading.set(false);
        assert!(!reg.is_loading.get());
        reg.set_loading.set(true);
        assert!(reg.is_loading.get());
    }
}
