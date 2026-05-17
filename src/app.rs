use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
fn AccordionItem(
    title: String,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="accordion-item">
            <button class="accordion-header" on:click=move |_| set_is_open.update(|v| *v = !*v)>
                {title}
                <span class="accordion-icon">{move || if is_open.get() { "−" } else { "+" }}</span>
            </button>
            <div class="accordion-content" style:display=move || if is_open.get() { "block" } else { "none" }>
                <div class="accordion-body">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <div class="accordion-demo">
            <h2>"Accordion Demo"</h2>
            <AccordionItem title="Section 1".to_string()>
                <p>"This is the content for section 1. It can contain any Leptos view!"</p>
            </AccordionItem>
            <AccordionItem title="Section 2".to_string()>
                <p>"Section 2 content goes here. You can put more text, images, or even other components."</p>
            </AccordionItem>
            <AccordionItem title="Section 3".to_string()>
                <p>"And finally, section 3. The state is managed locally within each AccordionItem."</p>
            </AccordionItem>
        </div>
    }
}

#[component]
pub fn DrawerDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="drawer-demo">
            <h2>"Drawer Component Demo"</h2>
            <p>"Click the button below to open the sliding panel from the bottom."</p>
            
            <button class="open-drawer-btn" on:click=move |_| set_is_open.set(true)>
                "Open Drawer"
            </button>

            <div 
                class="drawer-overlay" 
                style:display=move || if is_open.get() { "flex" } else { "none" }
                on:click=move |_| set_is_open.set(false)
            >
                <div 
                    class="drawer-panel" 
                    on:click=|ev| ev.stop_propagation()
                    class:open=move || is_open.get()
                >
                    <div class="drawer-header">
                        <h3>"Settings Panel"</h3>
                        <button class="close-drawer-btn" on:click=move |_| set_is_open.set(false)>"✕"</button>
                    </div>
                    <div class="drawer-body">
                        <p>"This is a sliding up panel (drawer). It's great for mobile-first designs or secondary settings!"</p>
                        <div class="drawer-option">
                            <label>"Enable Notifications"</label>
                            <input type="checkbox" />
                        </div>
                        <div class="drawer-option">
                            <label>"Dark Mode"</label>
                            <input type="checkbox" checked />
                        </div>
                        <button class="save-btn" on:click=move |_| set_is_open.set(false)>"Save Changes"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}


#[component]
fn DevelopmentPlaceholder(name: String) -> impl IntoView {
    view! {
        <div class="placeholder">
            <h1>{name} " Component"</h1>
            <p>"This component is still in development. Please check back later!"</p>
        </div>
    }
}

#[component]
fn UtilityCodeBlock(name: String) -> impl IntoView {
    let code = create_resource(
        move || name.clone(),
        |name| async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "utility": name })).unwrap();
            let result = invoke("get_utility_source", args).await;
            if let Ok(val) = result {
                val.as_string().unwrap_or_else(|| "Could not parse source code".to_string())
            } else {
                "Error loading source code".to_string()
            }
        },
    );

    view! {
        <div class="utility-container">
            <h2>{name} " System Utility"</h2>
            <div class="code-block">
                <pre><code>{move || code.get().unwrap_or("Loading code...").to_string()}</code></pre>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (is_search_open, set_is_search_open) = signal(false);
    let (active_demo, set_active_demo) = signal(Option::<String>::None);

    let demos: Vec<(&'static str, &'static str, Box<dyn Fn() -> View>)> = vec![
        ("Accordion", "accordion", Box::new(|| view! { <AccordionDemo /> }.into_any())),
        ("Drawer", "drawer", Box::new(|| view! { <DrawerDemo /> }.into_any())),
        ("Table", "table", Box::new(|| view! { <DevelopmentPlaceholder name="Table".to_string() /> }.into_any())),
        ("Modal", "modal", Box::new(|| view! { <DevelopmentPlaceholder name="Modal".to_string() /> }.into_any())),
        ("Toast", "toast", Box::new(|| view! { <DevelopmentPlaceholder name="Toast".to_string() /> }.into_any())),
        ("Form", "form", Box::new(|| view! { <DevelopmentPlaceholder name="Form".to_string() /> }.into_any())),
    ];

    let utilities: Vec<(&'static str, &'static str)> = vec![
        ("Network", "network"),
        ("System", "system"),
    ];

    let filtered_demos = move || {
        let query = search_query.get().to_lowercase();
        demos.iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    };

    let filtered_utils = move || {
        let query = search_query.get().to_lowercase();
        utilities.iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <nav class="navbar">
            <button class="nav-brand-btn" on:click=move |_| set_active_demo.set(None)>"Component Library"</button>
            <button class="search-toggle" on:click=move |_| set_is_search_open.update(|v| *v = !*v)>
                "🔍 Search"
            </button>
        </nav>

        <div class="search-overlay" style:display=move || if is_search_open.get() { "flex" } else { "none" }>
            <div class="search-container">
                <div class="search-input-wrapper">
                    <input 
                        type="text" 
                        placeholder="Search components or utilities..." 
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                    <button class="close-search" on:click=move |_| set_is_search_open.set(false)>"✕"</button>
                </div>
                <div class="search-results">
                    {move || {
                        let filtered = filtered_demos();
                        let utils = filtered_utils();
                        
                        if filtered.is_empty() && utils.is_empty() {
                            view! { <div class="no-results">"No results found"</div> }.into_any()
                        } else {
                            let mut results = Vec::new();
                            
                            if !filtered.is_empty() {
                                results.push(view! { <div class="search-section-title">"Components"</div> }.into_any());
                                let component_results = filtered.into_iter().map(|(name, id, _)| {
                                    let id = id.to_string();
                                    view! {
                                        <div class="search-item" on:click=move |_| {
                                            set_active_demo.set(Some(id.clone()));
                                            set_is_search_open.set(false);
                                            set_search_query.set(String::new());
                                        }>
                                            <span class="item-name">{name}</span>
                                            <span class="item-action">"View →"</span>
                                        </div>
                                    }.into_any()
                                }).collect::<Vec<_>>();
                                results.extend(component_results);
                            }

                            if !utils.is_empty() {
                                results.push(view! { <div class="search-section-title">"System Utilities"</div> }.into_any());
                                let util_results = utils.into_iter().map(|(name, id)| {
                                    let id = id.to_string();
                                    view! {
                                        <div class="search-item" on:click=move |_| {
                                            set_active_demo.set(Some(id.clone()));
                                            set_is_search_open.set(false);
                                            set_search_query.set(String::new());
                                        }>
                                            <span class="item-name">{name}</span>
                                            <span class="item-action">"Source →"</span>
                                        </div>
                                    }.into_any()
                                }).collect::<Vec<_>>();
                                results.extend(util_results);
                            }

                            results.into_view().into_any()
                        }
                    }}
                </div>
            </div>
        </div>

        <main class="container">
            {move || match active_demo.get().as_deref() {
                Some(id) => {
                    if let Some((_, demo_id, render)) = demos.iter().find(|(_, d, _)| *d == id) {
                        render()
                    } else if let Some((_, util_id)) = utilities.iter().find(|(_, d)| *d == id) {
                        view! { <UtilityCodeBlock name=util_id.to_string() /> }.into_any()
                    } else {
                        view! { <div class="error-msg">"Item not found"</div> }.into_any()
                    }
                },
                None => {
                    let count = demos.len();
                    let util_count = utilities.len();
                    view! { 
                        <div class="placeholder">
                            <h1>"Welcome to the Component Library"</h1>
                            <p>"We currently have {count} components and {util_count} system utilities available. Click the search button in the navbar to explore them!"</p>
                        </div> 
                    }.into_any()
                },
            }}
        </main>
    }
}

