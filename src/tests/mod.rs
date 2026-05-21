use leptos::prelude::*;

#[cfg(test)]
pub mod reactivity {
    use super::*;

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_signal_basic_creation() {
        let _rt = test_runtime();
        let (value, _set_value) = signal(42);
        assert_eq!(value.get(), 42);
    }

    #[test]
    fn test_signal_update() {
        let _rt = test_runtime();
        let (value, set_value) = signal(String::from("initial"));

        set_value.update(|v| v.push_str(" - updated"));
        assert_eq!(value.get(), "initial - updated");
    }

    #[test]
    fn test_signal_derive() {
        let _rt = test_runtime();
        let (count, set_count) = signal(0);

        let double = move || count.get() * 2;

        assert_eq!(double(), 0);
        set_count.set(5);
        assert_eq!(double(), 10);
    }

    #[test]
    fn test_memo_reactive() {
        let _rt = test_runtime();
        let (items, set_items) = signal(vec![1, 2, 3]);

        let memo_count = move || items.get().len();

        assert_eq!(memo_count(), 3);
        set_items.set(vec![1, 2, 3, 4, 5]);
        assert_eq!(memo_count(), 5);
    }

    #[test]
    fn test_rw_signal_write() {
        let _rt = test_runtime();
        let state = RwSignal::new(10);

        assert_eq!(state.get_untracked(), 10);
        state.update(|v| *v = *v + 5);
        assert_eq!(state.get_untracked(), 15);
    }

    #[test]
    fn test_read_signal_get() {
        let _rt = test_runtime();
        let (value, _) = signal(100);

        assert_eq!(value.get(), 100);
    }

    #[test]
    fn test_effect_reactive_execution() {
        let _rt = test_runtime();
        let (counter, _set_counter) = signal(0);

        let value = counter.get();
        assert_eq!(value, 0);
    }

    #[test]
    fn test_derived_signal_from_multiple_sources() {
        let _rt = test_runtime();
        let (a, set_a) = signal(10);
        let (b, set_b) = signal(20);

        let sum = move || a.get() + b.get();

        assert_eq!(sum(), 30);
        set_a.set(15);
        assert_eq!(sum(), 35);
        set_b.set(25);
        assert_eq!(sum(), 40);
    }

    #[test]
    fn test_condition_reactive() {
        let _rt = test_runtime();
        let (visible, set_visible) = signal(true);

        let display = move || if visible.get() { "block" } else { "none" };

        assert_eq!(display(), "block");
        set_visible.set(false);
        assert_eq!(display(), "none");
    }

    #[test]
    fn test_vec_signal_reactivity() {
        let _rt = test_runtime();
        let (items, set_items) = signal(vec![1, 2, 3]);

        assert_eq!(items.get().len(), 3);

        set_items.update(|v| v.push(4));
        assert_eq!(items.get().len(), 4);

        set_items.set(vec![]);
        assert_eq!(items.get().len(), 0);
    }

    #[test]
    fn test_option_signal_reactivity() {
        let _rt = test_runtime();
        let (selected, set_selected) = signal::<Option<String>>(None);

        assert!(selected.get().is_none());

        set_selected.set(Some("item1".to_string()));
        assert!(selected.get().is_some());
        assert_eq!(selected.get().unwrap(), "item1");

        set_selected.set(None);
        assert!(selected.get().is_none());
    }
}

#[cfg(test)]
pub mod services {
    use super::*;
    use crate::core::models::RegistryItem;
    use crate::core::models::TodoItem;
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
        TodoService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_registry_service_initial_state() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        assert!(registry.items.get().is_empty());
        assert!(registry.is_loading.get());
    }

    #[test]
    fn test_registry_service_set_items() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        let items = vec![
            RegistryItem {
                name: "Test".into(),
                id: "t1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Demo".into(),
                id: "d1".into(),
                category: "utility".into(),
                status: "in-development".into(),
                line_count: 20,
            },
        ];

        registry.set_items.set(items.clone());
        assert_eq!(registry.items.get().len(), 2);
    }

    #[test]
    fn test_registry_service_loading_state() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        assert!(registry.is_loading.get());
        registry.set_loading.set(false);
        assert!(!registry.is_loading.get());
    }

    #[test]
    fn test_registry_service_filter_by_status() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        let items = vec![
            RegistryItem {
                name: "Pinned1".into(),
                id: "p1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Dev1".into(),
                id: "d1".into(),
                category: "component".into(),
                status: "in-development".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "Archived1".into(),
                id: "a1".into(),
                category: "component".into(),
                status: "archives".into(),
                line_count: 30,
            },
        ];

        registry.set_items.set(items);

        let all_items = registry.items.get();
        let pinned: Vec<_> = all_items.iter().filter(|i| i.status == "pinned").collect();
        assert_eq!(pinned.len(), 1);

        let dev: Vec<_> = all_items
            .iter()
            .filter(|i| i.status == "in-development")
            .collect();
        assert_eq!(dev.len(), 1);
    }

    #[test]
    fn test_registry_service_filter_by_category() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        let items = vec![
            RegistryItem {
                name: "Component1".into(),
                id: "c1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Utility1".into(),
                id: "u1".into(),
                category: "utility".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];

        registry.set_items.set(items);

        let all_items = registry.items.get();
        let components: Vec<_> = all_items
            .iter()
            .filter(|i| i.category == "component")
            .collect();
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_navigation_service_initial_state() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        assert!(nav.active_demo.get().is_none());
    }

    #[test]
    fn test_navigation_service_navigate_to() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        nav.navigate_to(Some("accordion".to_string()));
        assert!(nav.active_demo.get().is_some());
        assert_eq!(nav.active_demo.get().unwrap(), "accordion");
    }

    #[test]
    fn test_navigation_service_navigate_to_none() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        nav.navigate_to(Some("drawer".to_string()));
        assert!(nav.active_demo.get().is_some());

        nav.navigate_to(None);
        assert!(nav.active_demo.get().is_none());
    }

    #[test]
    fn test_navigation_service_multiple_navigations() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        let ids = vec!["accordion", "drawer", "modal", "theme"];
        for id in ids {
            nav.navigate_to(Some(id.to_string()));
            assert!(nav.active_demo.get().is_some());
        }
    }

    #[test]
    fn test_search_service_initial_state() {
        let _rt = test_runtime();
        let search = SearchService::new();

        assert!(search.query.get().is_empty());
        assert!(!search.is_open.get());
    }

    #[test]
    fn test_search_service_set_query() {
        let _rt = test_runtime();
        let search = SearchService::new();

        search.set_query.set("test query".to_string());
        assert_eq!(search.query.get(), "test query");
    }

    #[test]
    fn test_search_service_toggle_open() {
        let _rt = test_runtime();
        let search = SearchService::new();

        assert!(!search.is_open.get());
        search.toggle_search();
        assert!(search.is_open.get());
        search.toggle_search();
        assert!(!search.is_open.get());
    }

    #[test]
    fn test_search_service_close() {
        let _rt = test_runtime();
        let search = SearchService::new();

        search.set_query.set("some query".to_string());
        search.set_open.set(true);

        search.close_search();

        assert!(!search.is_open.get());
        assert!(search.query.get().is_empty());
    }

    #[test]
    fn test_theme_service_initial_state() {
        let _rt = test_runtime();
        let theme = ThemeService::new();

        assert!(!theme.is_dark_mode.get());
    }

    #[test]
    fn test_theme_service_toggle() {
        let _rt = test_runtime();
        let theme = ThemeService::new();

        assert!(!theme.is_dark_mode.get());
        theme.toggle_theme();
        assert!(theme.is_dark_mode.get());
        theme.toggle_theme();
        assert!(!theme.is_dark_mode.get());
    }

    #[test]
    fn test_theme_service_set_dark_mode() {
        let _rt = test_runtime();
        let theme = ThemeService::new();

        theme.set_dark_mode.set(true);
        assert!(theme.is_dark_mode.get());

        theme.set_dark_mode.set(false);
        assert!(!theme.is_dark_mode.get());
    }

    #[test]
    fn test_sidebar_service_initial_state() {
        let _rt = test_runtime();
        let sidebar = SidebarService::new();

        assert!(!sidebar.is_open.get());
    }

    #[test]
    fn test_sidebar_service_toggle() {
        let _rt = test_runtime();
        let sidebar = SidebarService::new();

        assert!(!sidebar.is_open.get());
        sidebar.toggle_sidebar();
        assert!(sidebar.is_open.get());
        sidebar.toggle_sidebar();
        assert!(!sidebar.is_open.get());
    }

    #[test]
    fn test_sidebar_service_set_open() {
        let _rt = test_runtime();
        let sidebar = SidebarService::new();

        sidebar.set_is_open.set(true);
        assert!(sidebar.is_open.get());
    }

    #[test]
    fn test_todo_service_initial_state() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        assert!(todo.items.get().is_empty());
    }

    #[test]
    fn test_todo_service_set_items() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        let items = vec![
            TodoItem {
                id: 1,
                title: "Task 1".to_string(),
                completed: false,
                created_at: "2025-01-01".to_string(),
            },
            TodoItem {
                id: 2,
                title: "Task 2".to_string(),
                completed: true,
                created_at: "2025-01-02".to_string(),
            },
        ];

        todo.set_items.set(items);
        assert_eq!(todo.items.get().len(), 2);
    }

    #[test]
    fn test_todo_service_add_item() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        todo.set_items.set(vec![]);

        let mut items = todo.items.get();
        items.push(TodoItem {
            id: 1,
            title: "New Task".to_string(),
            completed: false,
            created_at: "2025-01-01".to_string(),
        });
        todo.set_items.set(items);

        assert_eq!(todo.items.get().len(), 1);
    }

    #[test]
    fn test_todo_service_toggle_completed() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        let items = vec![TodoItem {
            id: 1,
            title: "Task".to_string(),
            completed: false,
            created_at: "2025-01-01".to_string(),
        }];
        todo.set_items.set(items);

        let mut items = todo.items.get();
        if let Some(item) = items.first_mut() {
            item.completed = true;
        }
        todo.set_items.set(items);

        assert!(todo.items.get()[0].completed);
    }

    #[test]
    fn test_todo_service_delete_item() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        let items = vec![
            TodoItem {
                id: 1,
                title: "Task 1".to_string(),
                completed: false,
                created_at: "2025-01-01".to_string(),
            },
            TodoItem {
                id: 2,
                title: "Task 2".to_string(),
                completed: false,
                created_at: "2025-01-02".to_string(),
            },
        ];
        todo.set_items.set(items);

        let filtered: Vec<_> = todo.items.get().into_iter().filter(|i| i.id != 1).collect();
        todo.set_items.set(filtered);

        assert_eq!(todo.items.get().len(), 1);
    }
}

#[cfg(test)]
pub mod context {
    use super::*;
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_provide_and_use_context() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        provide_context(registry);

        let retrieved: RegistryService = use_context().expect("Should have context");
        assert!(retrieved.items.get().is_empty());
    }

    #[test]
    fn test_context_reactivity_shared() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        provide_context(nav);

        let nav1: NavigationService = use_context().expect("Context 1");
        let nav2: NavigationService = use_context().expect("Context 2");

        nav1.navigate_to(Some("test".to_string()));

        assert!(nav2.active_demo.get().is_some());
    }

    #[test]
    fn test_multiple_services_context() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();
        let theme = ThemeService::new();
        let sidebar = SidebarService::new();

        provide_context(registry);
        provide_context(nav);
        provide_context(theme);
        provide_context(sidebar);

        let retrieved_registry: RegistryService = use_context().expect("Registry");
        let retrieved_nav: NavigationService = use_context().expect("Nav");
        let retrieved_theme: ThemeService = use_context().expect("Theme");
        let retrieved_sidebar: SidebarService = use_context().expect("Sidebar");

        assert!(retrieved_registry.items.get().is_empty());
        assert!(retrieved_nav.active_demo.get().is_none());
        assert!(!retrieved_theme.is_dark_mode.get());
        assert!(!retrieved_sidebar.is_open.get());
    }

    #[test]
    fn test_search_with_registry_context() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let search = SearchService::new();

        provide_context(registry);
        provide_context(search);

        let _reg: RegistryService = use_context().expect("Registry");
        let search_ctx: SearchService = use_context().expect("Search");

        search_ctx.set_query.set("test".to_string());

        let query = search_ctx.query.get();
        assert_eq!(query, "test");
    }
}

#[cfg(test)]
pub mod workflows {
    use super::*;
    use crate::core::models::{RegistryItem, TodoItem};
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
        TodoService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_complete_navigation_flow() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();

        provide_context(registry);
        provide_context(nav);

        let items = vec![
            RegistryItem {
                name: "Accordion".into(),
                id: "accordion".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 100,
            },
            RegistryItem {
                name: "Drawer".into(),
                id: "drawer".into(),
                category: "component".into(),
                status: "in-development".into(),
                line_count: 200,
            },
        ];
        registry.set_items.set(items);

        let reg_ctx: RegistryService = use_context().expect("Registry");
        let nav_ctx: NavigationService = use_context().expect("Nav");

        nav_ctx.navigate_to(Some("accordion".to_string()));

        let active_id = nav_ctx.active_demo.get();
        assert!(active_id.is_some());
        assert_eq!(active_id.unwrap(), "accordion");

        let all_items = reg_ctx.items.get();
        let found_item = all_items.iter().find(|i| i.id == "accordion");
        assert!(found_item.is_some());
        assert_eq!(found_item.unwrap().name, "Accordion");

        nav_ctx.navigate_to(None);
        assert!(nav_ctx.active_demo.get().is_none());
    }

    #[test]
    fn test_theme_toggle_workflow() {
        let _rt = test_runtime();

        let theme = ThemeService::new();
        provide_context(theme);

        let theme_ctx: ThemeService = use_context().expect("Theme");

        assert!(!theme_ctx.is_dark_mode.get());

        theme_ctx.toggle_theme();
        assert!(theme_ctx.is_dark_mode.get());

        theme_ctx.toggle_theme();
        assert!(!theme_ctx.is_dark_mode.get());

        theme_ctx.set_dark_mode.set(true);
        assert!(theme_ctx.is_dark_mode.get());
    }

    #[test]
    fn test_sidebar_toggle_workflow() {
        let _rt = test_runtime();

        let sidebar = SidebarService::new();
        provide_context(sidebar);

        let sidebar_ctx: SidebarService = use_context().expect("Sidebar");

        assert!(!sidebar_ctx.is_open.get());

        sidebar_ctx.toggle_sidebar();
        assert!(sidebar_ctx.is_open.get());

        sidebar_ctx.set_is_open.set(false);
        assert!(!sidebar_ctx.is_open.get());
    }

    #[test]
    fn test_search_workflow() {
        let _rt = test_runtime();

        let search = SearchService::new();

        search.set_query.set("test".to_string());

        assert_eq!(search.query.get_untracked(), "test");

        search.set_query.set("acc".to_string());
        let query = search.query.get_untracked();

        let items = vec![
            RegistryItem {
                name: "Accordion".into(),
                id: "acc".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Modal".into(),
                id: "mod".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "Network".into(),
                id: "net".into(),
                category: "utility".into(),
                status: "pinned".into(),
                line_count: 30,
            },
        ];

        let filtered: Vec<_> = items
            .iter()
            .filter(|i| i.name.to_lowercase().contains(&query.to_lowercase()))
            .collect();

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Accordion");
    }

    #[test]
    fn test_todo_crud_workflow() {
        let _rt = test_runtime();

        let todo = TodoService::new();
        provide_context(todo);

        let todo_ctx: TodoService = use_context().expect("Todo");

        assert!(todo_ctx.items.get().is_empty());

        todo_ctx.set_items.set(vec![TodoItem {
            id: 1,
            title: "Task 1".into(),
            completed: false,
            created_at: "2025-01-01".into(),
        }]);
        assert_eq!(todo_ctx.items.get().len(), 1);

        let mut items = todo_ctx.items.get();
        if let Some(t) = items.first_mut() {
            t.completed = true;
        }
        todo_ctx.set_items.set(items);
        assert!(todo_ctx.items.get()[0].completed);

        todo_ctx.set_items.set(vec![]);
        assert!(todo_ctx.items.get().is_empty());
    }

    #[test]
    fn test_complete_app_state_flow() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();
        let search = SearchService::new();
        let theme = ThemeService::new();
        let sidebar = SidebarService::new();
        let todo = TodoService::new();

        provide_context(registry);
        provide_context(nav);
        provide_context(search);
        provide_context(theme);
        provide_context(sidebar);
        provide_context(todo);

        let reg: RegistryService = use_context().expect("Registry");
        let nav_svc: NavigationService = use_context().expect("Nav");
        let search_svc: SearchService = use_context().expect("Search");
        let theme_svc: ThemeService = use_context().expect("Theme");
        let sidebar_svc: SidebarService = use_context().expect("Sidebar");
        let todo_svc: TodoService = use_context().expect("Todo");

        reg.set_items.set(vec![RegistryItem {
            name: "Component".into(),
            id: "comp".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 50,
        }]);

        nav_svc.navigate_to(Some("comp".to_string()));
        search_svc.set_open.set(true);
        theme_svc.toggle_theme();
        sidebar_svc.toggle_sidebar();

        todo_svc.set_items.set(vec![TodoItem {
            id: 1,
            title: "Test".into(),
            completed: false,
            created_at: "2025-01-01".into(),
        }]);

        assert_eq!(reg.items.get().len(), 1);
        assert!(nav_svc.active_demo.get().is_some());
        assert!(search_svc.is_open.get());
        assert!(theme_svc.is_dark_mode.get());
        assert!(sidebar_svc.is_open.get());
        assert_eq!(todo_svc.items.get().len(), 1);
    }
}

#[cfg(test)]
pub mod edge_cases {
    use super::*;
    use crate::core::models::RegistryItem;

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_empty_registry() {
        let _rt = test_runtime();
        let registry = crate::services::RegistryService::new();

        registry.set_items.set(vec![]);

        assert!(registry.items.get().is_empty());
    }

    #[test]
    fn test_large_registry() {
        let _rt = test_runtime();
        let registry = crate::services::RegistryService::new();

        let items: Vec<_> = (0..1000)
            .map(|i| RegistryItem {
                name: format!("Item {}", i),
                id: format!("item_{}", i),
                category: if i % 2 == 0 {
                    "component".into()
                } else {
                    "utility".into()
                },
                status: if i % 3 == 0 {
                    "pinned".into()
                } else {
                    "in-development".into()
                },
                line_count: i as usize,
            })
            .collect();

        registry.set_items.set(items);

        assert_eq!(registry.items.get().len(), 1000);
    }

    #[test]
    fn test_special_characters_in_names() {
        let _rt = test_runtime();
        let registry = crate::services::RegistryService::new();

        let items = vec![
            RegistryItem {
                name: "Item with <special> chars".into(),
                id: "special_1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Item with 'quotes'".into(),
                id: "special_2".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];

        registry.set_items.set(items);

        assert_eq!(registry.items.get().len(), 2);
    }

    #[test]
    fn test_unicode_content() {
        let _rt = test_runtime();
        let registry = crate::services::RegistryService::new();

        let items = vec![
            RegistryItem {
                name: "组件".into(),
                id: "chinese".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Компонент".into(),
                id: "russian".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];

        registry.set_items.set(items);

        assert_eq!(registry.items.get().len(), 2);
    }

    #[test]
    fn test_concurrent_state_updates() {
        let _rt = test_runtime();
        let (a, set_a) = signal(0);
        let (b, set_b) = signal(0);

        set_a.set(1);
        set_b.set(2);
        set_a.set(3);
        set_b.set(4);

        assert_eq!(a.get(), 3);
        assert_eq!(b.get(), 4);
    }

    #[test]
    fn test_derived_state_after_updates() {
        let _rt = test_runtime();
        let (count, set_count) = signal(0);

        let doubled = move || count.get() * 2;
        let tripled = move || count.get() * 3;

        set_count.set(5);
        assert_eq!(doubled(), 10);
        assert_eq!(tripled(), 15);

        set_count.set(10);
        assert_eq!(doubled(), 20);
        assert_eq!(tripled(), 30);
    }
}

#[cfg(test)]
pub mod model_edge_cases {
    use super::*;
    use crate::core::models::{FrontendError, JsonTodo, TodoItem, TreeNode, User};

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_frontend_error_all_variants() {
        let not_found = FrontendError::NotFound("file.txt".to_string());
        assert_eq!(not_found.to_string(), "File not found: file.txt");

        let permission = FrontendError::PermissionDenied("denied".to_string());
        assert_eq!(permission.to_string(), "Permission denied: denied");

        let internal = FrontendError::Internal("error".to_string());
        assert_eq!(internal.to_string(), "Internal error: error");

        let invalid = FrontendError::InvalidArgument("bad input".to_string());
        assert_eq!(invalid.to_string(), "Invalid argument: bad input");
    }

    #[test]
    fn test_frontend_error_clone() {
        let err = FrontendError::NotFound("test".to_string());
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }

    #[test]
    fn test_tree_node_recursive() {
        let tree = TreeNode {
            id: "root".into(),
            label: "Root".into(),
            children: vec![
                TreeNode {
                    id: "child1".into(),
                    label: "Child 1".into(),
                    children: vec![
                        TreeNode {
                            id: "grand1".into(),
                            label: "Grand 1".into(),
                            children: vec![],
                        },
                        TreeNode {
                            id: "grand2".into(),
                            label: "Grand 2".into(),
                            children: vec![],
                        },
                    ],
                },
                TreeNode {
                    id: "child2".into(),
                    label: "Child 2".into(),
                    children: vec![],
                },
            ],
        };

        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.children[0].children.len(), 2);
        assert_eq!(tree.children[1].children.len(), 0);
    }

    #[test]
    fn test_tree_node_clone() {
        let node = TreeNode {
            id: "test".into(),
            label: "Test".into(),
            children: vec![],
        };
        let cloned = node.clone();
        assert_eq!(node.id, cloned.id);
        assert_eq!(node.label, cloned.label);
    }

    #[test]
    fn test_json_todo_variants() {
        let todo1 = JsonTodo {
            id: 1,
            title: "Task 1".into(),
            completed: false,
        };
        let todo2 = JsonTodo {
            id: 2,
            title: "Task 2".into(),
            completed: true,
        };

        assert!(!todo1.completed);
        assert!(todo2.completed);
    }

    #[test]
    fn test_json_todo_serde() {
        let todo = JsonTodo {
            id: 42,
            title: "Test".into(),
            completed: true,
        };
        let json = serde_json::to_string(&todo).unwrap();
        let parsed: JsonTodo = serde_json::from_str(&json).unwrap();
        assert_eq!(todo.id, parsed.id);
        assert_eq!(todo.title, parsed.title);
        assert_eq!(todo.completed, parsed.completed);
    }

    #[test]
    fn test_todo_item_all_fields() {
        let item = TodoItem {
            id: 1,
            title: "Test task".into(),
            completed: false,
            created_at: "2025-01-15 10:30:00".into(),
        };

        assert_eq!(item.id, 1);
        assert_eq!(item.title, "Test task");
        assert!(!item.completed);
        assert_eq!(item.created_at, "2025-01-15 10:30:00");
    }

    #[test]
    fn test_user_all_fields() {
        let user = User {
            id: 1,
            name: "John Doe".into(),
            email: "john@example.com".into(),
            role: "admin".into(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.role, "admin");
    }

    #[test]
    fn test_user_serde() {
        let user = User {
            id: 5,
            name: "Jane".into(),
            email: "jane@test.com".into(),
            role: "user".into(),
        };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user.name, parsed.name);
    }

    #[test]
    fn test_registry_item_max_values() {
        let item = crate::core::models::RegistryItem {
            name: "A".repeat(1000),
            id: "B".repeat(1000),
            category: "C".repeat(100),
            status: "D".repeat(100),
            line_count: usize::MAX,
        };

        assert_eq!(item.name.len(), 1000);
        assert_eq!(item.id.len(), 1000);
        assert_eq!(item.line_count, usize::MAX);
    }
}

#[cfg(test)]
pub mod service_state_transitions {
    use super::*;
    use crate::core::models::{RegistryItem, TodoItem};
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
        TodoService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_navigation_state_transitions() {
        let _rt = test_runtime();
        let nav = NavigationService::new();

        assert!(nav.active_demo.get().is_none());

        nav.navigate_to(Some("a".to_string()));
        assert!(nav.active_demo.get().is_some());
        assert_eq!(nav.active_demo.get().unwrap(), "a");

        nav.navigate_to(Some("b".to_string()));
        assert_eq!(nav.active_demo.get().unwrap(), "b");

        nav.navigate_to(None);
        assert!(nav.active_demo.get().is_none());

        nav.navigate_to(Some("c".to_string()));
        nav.navigate_to(None);
        assert!(nav.active_demo.get().is_none());
    }

    #[test]
    fn test_theme_multiple_toggles() {
        let _rt = test_runtime();
        let theme = ThemeService::new();

        theme.toggle_theme();
        assert!(theme.is_dark_mode.get_untracked());

        theme.toggle_theme();
        assert!(!theme.is_dark_mode.get_untracked());

        theme.toggle_theme();
        assert!(theme.is_dark_mode.get_untracked());

        theme.toggle_theme();
        assert!(!theme.is_dark_mode.get_untracked());
    }

    #[test]
    fn test_search_query_state_machine() {
        let _rt = test_runtime();
        let search = SearchService::new();

        assert!(!search.is_open.get());
        assert!(search.query.get().is_empty());

        search.toggle_search();
        assert!(search.is_open.get());

        search.set_query.set("test".to_string());
        assert_eq!(search.query.get(), "test");

        search.close_search();
        assert!(!search.is_open.get());
        assert!(search.query.get().is_empty());

        search.set_query.set("another".to_string());
        search.toggle_search();
        assert!(search.is_open.get());
        assert_eq!(search.query.get(), "another");
    }

    #[test]
    fn test_sidebar_rapid_toggles() {
        let _rt = test_runtime();
        let sidebar = SidebarService::new();

        sidebar.toggle_sidebar();
        assert!(sidebar.is_open.get_untracked());

        sidebar.toggle_sidebar();
        assert!(!sidebar.is_open.get_untracked());

        sidebar.toggle_sidebar();
        assert!(sidebar.is_open.get_untracked());
    }

    #[test]
    fn test_registry_items_mutation_sequence() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        registry.set_items.set(vec![]);
        assert!(registry.items.get().is_empty());

        registry.set_items.set(vec![RegistryItem {
            name: "Test".into(),
            id: "t1".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        }]);
        assert_eq!(registry.items.get().len(), 1);

        let mut items = registry.items.get();
        items.push(RegistryItem {
            name: "Test2".into(),
            id: "t2".into(),
            category: "utility".into(),
            status: "in-development".into(),
            line_count: 20,
        });
        registry.set_items.set(items);
        assert_eq!(registry.items.get().len(), 2);

        registry.set_items.set(vec![]);
        assert!(registry.items.get().is_empty());
    }

    #[test]
    fn test_todo_state_sequence() {
        let _rt = test_runtime();
        let todo = TodoService::new();

        todo.set_items.set(vec![]);
        assert!(todo.items.get().is_empty());

        todo.set_items.set(vec![TodoItem {
            id: 1,
            title: "Task 1".into(),
            completed: false,
            created_at: "2025-01-01".into(),
        }]);
        assert_eq!(todo.items.get().len(), 1);

        let mut items = todo.items.get();
        items[0].completed = true;
        todo.set_items.set(items);
        assert!(todo.items.get()[0].completed);

        todo.set_items.set(vec![]);
        assert!(todo.items.get().is_empty());
    }

    #[test]
    fn test_loading_state_transitions() {
        let _rt = test_runtime();
        let registry = RegistryService::new();

        assert!(registry.is_loading.get());

        registry.set_loading.set(false);
        assert!(!registry.is_loading.get());

        registry.set_loading.set(true);
        assert!(registry.is_loading.get());

        registry.set_loading.set(false);
        assert!(!registry.is_loading.get());
    }

    #[test]
    fn test_all_services_isolated() {
        let _rt = test_runtime();

        let nav1 = NavigationService::new();
        let nav2 = NavigationService::new();

        nav1.navigate_to(Some("test".to_string()));

        assert!(nav1.active_demo.get().is_some());
        assert!(nav2.active_demo.get().is_none());
    }
}

#[cfg(test)]
pub mod component_state_edge_cases {
    use super::*;
    use crate::core::models::{RegistryItem, TodoItem};
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
        TodoService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_nested_context_hierarchy() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();

        provide_context(registry);

        let ctx1: RegistryService = use_context().expect("Level 1");

        provide_context(nav);

        let ctx2_registry: RegistryService = use_context().expect("Level 2 registry");
        let ctx2_nav: NavigationService = use_context().expect("Level 2 nav");

        assert!(ctx1.items.get().is_empty());
        assert!(ctx2_registry.items.get().is_empty());
        assert!(ctx2_nav.active_demo.get().is_none());
    }

    #[test]
    fn test_multiple_context_types() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();
        let theme = ThemeService::new();
        let sidebar = SidebarService::new();
        let search = SearchService::new();

        provide_context(registry);
        provide_context(nav);
        provide_context(theme);
        provide_context(sidebar);
        provide_context(search);

        let r: RegistryService = use_context().expect("Registry");
        let n: NavigationService = use_context().expect("Nav");
        let t: ThemeService = use_context().expect("Theme");
        let s: SidebarService = use_context().expect("Sidebar");
        let sc: SearchService = use_context().expect("Search");

        assert!(r.items.get().is_empty());
        assert!(n.active_demo.get().is_none());
        assert!(!t.is_dark_mode.get());
        assert!(!s.is_open.get());
        assert!(sc.query.get().is_empty());
    }

    #[test]
    fn test_context_override() {
        let _rt = test_runtime();

        let registry1 = RegistryService::new();
        provide_context(registry1);

        let before: RegistryService = use_context().expect("Before");

        let registry2 = RegistryService::new();
        registry2.set_items.set(vec![RegistryItem {
            name: "Overridden".into(),
            id: "ov".into(),
            category: "test".into(),
            status: "pinned".into(),
            line_count: 10,
        }]);
        provide_context(registry2);

        let after: RegistryService = use_context().expect("After");

        assert!(before.items.get().is_empty());
        assert_eq!(after.items.get().len(), 1);
    }

    #[test]
    fn test_service_with_initial_data() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        registry.set_items.set(vec![
            RegistryItem {
                name: "A".into(),
                id: "a".into(),
                category: "c".into(),
                status: "p".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "B".into(),
                id: "b".into(),
                category: "c".into(),
                status: "p".into(),
                line_count: 20,
            },
        ]);

        provide_context(registry);

        let ctx: RegistryService = use_context().expect("Service");

        let items = ctx.items.get();
        assert_eq!(items.len(), 2);

        let item = items.iter().find(|i| i.id == "a").expect("Found");
        assert_eq!(item.name, "A");
    }

    #[test]
    fn test_todo_with_multiple_items() {
        let _rt = test_runtime();

        let todo = TodoService::new();
        todo.set_items.set(vec![
            TodoItem {
                id: 1,
                title: "Task 1".into(),
                completed: false,
                created_at: "2025-01-01".into(),
            },
            TodoItem {
                id: 2,
                title: "Task 2".into(),
                completed: true,
                created_at: "2025-01-02".into(),
            },
            TodoItem {
                id: 3,
                title: "Task 3".into(),
                completed: false,
                created_at: "2025-01-03".into(),
            },
        ]);

        provide_context(todo);

        let ctx: TodoService = use_context().expect("Todo");

        assert_eq!(ctx.items.get().len(), 3);

        let all_items = ctx.items.get();
        let completed: Vec<_> = all_items.iter().filter(|t| t.completed).cloned().collect();
        assert_eq!(completed.len(), 1);

        let pending: Vec<_> = all_items.iter().filter(|t| !t.completed).cloned().collect();
        assert_eq!(pending.len(), 2);
    }
}

#[cfg(test)]
pub mod integration_edge_cases {
    use super::*;
    use crate::core::models::{RegistryItem, TodoItem};
    use crate::services::{
        NavigationService, RegistryService, SearchService, SidebarService, ThemeService,
        TodoService,
    };

    pub fn test_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_full_user_session_workflow() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();
        let theme = ThemeService::new();
        let sidebar = SidebarService::new();
        let search = SearchService::new();
        let todo = TodoService::new();

        provide_context(registry);
        provide_context(nav);
        provide_context(theme);
        provide_context(sidebar);
        provide_context(search);
        provide_context(todo);

        let reg: RegistryService = use_context().expect("R");
        let n: NavigationService = use_context().expect("N");
        let t: ThemeService = use_context().expect("T");
        let s: SidebarService = use_context().expect("S");
        let sc: SearchService = use_context().expect("SC");
        let td: TodoService = use_context().expect("TD");

        reg.set_items.set(vec![
            RegistryItem {
                name: "Home".into(),
                id: "home".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 100,
            },
            RegistryItem {
                name: "Settings".into(),
                id: "settings".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 200,
            },
        ]);

        n.navigate_to(Some("home".to_string()));
        t.toggle_theme();
        s.toggle_sidebar();
        sc.set_open.set(true);
        sc.set_query.set("home".to_string());
        td.set_items.set(vec![TodoItem {
            id: 1,
            title: "Setup".into(),
            completed: true,
            created_at: "2025-01-01".into(),
        }]);

        assert_eq!(reg.items.get().len(), 2);
        assert!(n.active_demo.get().is_some());
        assert!(t.is_dark_mode.get());
        assert!(s.is_open.get());
        assert!(sc.is_open.get());
        assert!(!sc.query.get().is_empty());
        assert_eq!(td.items.get().len(), 1);
    }

    #[test]
    fn test_state_persistence_during_navigation() {
        let _rt = test_runtime();

        let nav = NavigationService::new();
        let theme = ThemeService::new();

        provide_context(nav);
        provide_context(theme);

        let n: NavigationService = use_context().expect("Nav");
        let t: ThemeService = use_context().expect("Theme");

        t.toggle_theme();
        n.navigate_to(Some("page1".to_string()));

        assert!(t.is_dark_mode.get());
        assert_eq!(n.active_demo.get().unwrap(), "page1");

        n.navigate_to(Some("page2".to_string()));

        assert!(t.is_dark_mode.get());
        assert_eq!(n.active_demo.get().unwrap(), "page2");

        n.navigate_to(None);

        assert!(t.is_dark_mode.get());
        assert!(n.active_demo.get().is_none());
    }

    #[test]
    fn test_search_filter_navigation_integration() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let search = SearchService::new();
        let nav = NavigationService::new();

        provide_context(registry);
        provide_context(search);
        provide_context(nav);

        let reg: RegistryService = use_context().expect("R");
        let sc: SearchService = use_context().expect("S");
        let n: NavigationService = use_context().expect("N");

        reg.set_items.set(vec![
            RegistryItem {
                name: "Button".into(),
                id: "button".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Input".into(),
                id: "input".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "Output".into(),
                id: "output".into(),
                category: "utility".into(),
                status: "pinned".into(),
                line_count: 30,
            },
        ]);

        sc.set_query.set("but".to_string());

        let items = reg.items.get();
        let filtered: Vec<_> = items
            .iter()
            .filter(|i| {
                i.name
                    .to_lowercase()
                    .contains(&sc.query.get().to_lowercase())
            })
            .collect();

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Button");

        n.navigate_to(Some(filtered[0].id.clone()));

        assert_eq!(n.active_demo.get().unwrap(), "button");
    }

    #[test]
    fn test_theme_sidebar_search_combo() {
        let _rt = test_runtime();

        let theme = ThemeService::new();
        let sidebar = SidebarService::new();
        let search = SearchService::new();

        provide_context(theme);
        provide_context(sidebar);
        provide_context(search);

        let t: ThemeService = use_context().expect("T");
        let s: SidebarService = use_context().expect("S");
        let sc: SearchService = use_context().expect("SC");

        t.toggle_theme();
        s.toggle_sidebar();
        sc.toggle_search();

        sc.set_query.set("test".to_string());

        assert!(t.is_dark_mode.get());
        assert!(s.is_open.get());
        assert!(sc.is_open.get());
        assert_eq!(sc.query.get(), "test");
    }

    #[test]
    fn test_registry_filter_sort_pagination_simulation() {
        let _rt = test_runtime();

        let registry = RegistryService::new();

        let items: Vec<_> = (0..50)
            .map(|i| RegistryItem {
                name: format!("Item {}", i),
                id: format!("item_{}", i),
                category: if i % 2 == 0 {
                    "component".into()
                } else {
                    "utility".into()
                },
                status: if i % 3 == 0 {
                    "pinned".into()
                } else {
                    "in-development".into()
                },
                line_count: i as usize,
            })
            .collect();

        registry.set_items.set(items);

        let all = registry.items.get();

        let page_size = 10;
        let page = 0;
        let paged: Vec<_> = all
            .iter()
            .skip(page * page_size)
            .take(page_size)
            .cloned()
            .collect();

        assert_eq!(paged.len(), 10);

        let components: Vec<_> = all
            .iter()
            .filter(|i| i.category == "component")
            .cloned()
            .collect();
        assert_eq!(components.len(), 25);

        let pinned: Vec<_> = all
            .iter()
            .filter(|i| i.status == "pinned")
            .cloned()
            .collect();
        assert_eq!(pinned.len(), 17);
    }

    #[test]
    fn test_context_lifecycle() {
        let _rt = test_runtime();

        let result_before = use_context::<RegistryService>();
        assert!(result_before.is_none());

        {
            let registry = RegistryService::new();
            provide_context(registry);
        }

        let result_after = use_context::<RegistryService>();
        assert!(result_after.is_some());
    }

    #[test]
    fn test_service_identity_after_updates() {
        let _rt = test_runtime();

        let nav1 = NavigationService::new();
        let nav2 = nav1;

        nav1.navigate_to(Some("test".to_string()));

        assert!(nav1.active_demo.get().is_some());
        assert!(nav2.active_demo.get().is_some());
    }

    #[test]
    fn test_all_services_copy_semantics() {
        let _rt = test_runtime();

        let registry = RegistryService::new();
        let nav = NavigationService::new();
        let search = SearchService::new();
        let theme = ThemeService::new();
        let sidebar = SidebarService::new();
        let todo = TodoService::new();

        let reg_copy = registry;
        let nav_copy = nav;
        let search_copy = search;
        let theme_copy = theme;
        let sidebar_copy = sidebar;
        let todo_copy = todo;

        reg_copy.set_items.set(vec![]);
        nav_copy.navigate_to(Some("x".to_string()));
        search_copy.set_open.set(true);
        theme_copy.toggle_theme();
        sidebar_copy.toggle_sidebar();
        todo_copy.set_items.set(vec![]);

        assert!(registry.items.get().is_empty());
        assert!(nav.active_demo.get().is_some());
        assert!(search.is_open.get());
        assert!(theme.is_dark_mode.get());
        assert!(sidebar.is_open.get());
        assert!(todo.items.get().is_empty());
    }
}
