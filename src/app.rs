use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

static UTILITIES: &[(&str, &str)] = &[
    ("Network", "network"),
    ("System", "system"),
    ("Storage", "storage"),
    ("Process", "process"),
];

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (is_search_open, set_is_search_open) = signal(false);
    let (active_demo, set_active_demo) = signal(Option::<String>::None);

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    let _ = loader.remove();
                }
            }
        }
    });

    let filtered_demos = move || {
        let query = search_query.get().to_lowercase();
        vec![
            ("Accordion", "accordion"),
            ("Drawer", "drawer"),
        ].into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query))
            .collect::<Vec<_>>()
    };

    let filtered_utils = move || {
        let query = search_query.get().to_lowercase();
        UTILITIES.iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <div class="container">
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
                                    let component_results = filtered.into_iter().map(|(name, id)| {
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

            <main style="padding: 2rem; text-align: center;">
                {move || match active_demo.get().as_deref() {
                    Some("accordion") => view! { <AccordionDemo /> }.into_any(),
                    Some("drawer") => view! { <DrawerDemo /> }.into_any(),
                    Some(id) => {
                        if let Some((_, util_id)) = UTILITIES.iter().find(|(_, d)| *d == id) {
                            view! { <UtilityCodeBlock name=util_id.to_string() /> }.into_any()
                        } else {
                            view! { <div class="error-msg">"Item not found"</div> }.into_any()
                        }
                    },
                    None => {
                        let component_count = 2;
                        let util_count = UTILITIES.len();
                        view! { 
                            <div class="placeholder">
                                <h1>"Welcome to the Component Library"</h1>
                                <p>"We currently have " {component_count} " components and " {util_count} " system utilities available. Click the search button in the navbar to explore them!"</p>
                            </div> 
                        }.into_any()
                    },
                }}
            </main>
        </div>
    }
}

#[component]
fn ResultView(
    loading: bool,
    result: Result<View<()>, String>,
    children: Children,
) -> impl IntoView {
    if loading {
        view! { <div class="placeholder">"Loading..."</div> }
    } else {
        match result {
            Ok(val) => val,
            Err(e) => view! { <div class="error-msg" style="padding: 1rem; border: 1px solid #f5c6cb; background: #f8d7da; color: #721c24; border-radius: 4px;">
                <strong>"Error: "</strong> {e}
            </div> }.into_view(),
        }
    }
}

#[component]
fn UtilityCodeBlock(name: String) -> impl IntoView {
    let (state, set_state) = signal(Result::<View<()>, String>::Err("Initial state".to_string()));
    let (is_loading, set_is_loading) = signal(true);

    let name_for_spawn = name.clone();
    spawn_local(async move {
        set_is_loading.set(true);
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "utility": name_for_spawn })).unwrap();
        let result = invoke("get_utility_source", args).await;
        
        match result {
            Ok(val) => {
                if let Some(content) = val.as_string() {
                    set_state.set(Ok(view! { 
                        <div class="code-block">
                            <pre><code>{content}</code></pre>
                        </div> 
                    }.into_view()));
                } else {
                    set_state.set(Err("Could not parse source code as string".to_string()));
                }
            },
            Err(e) => {
                set_state.set(Err(format!("Backend error: {:?}", e)));
            }
        }
        set_is_loading.set(false);
    });

    view! {
        <div class="utility-container">
            <h2>{name} " System Utility"</h2>
            <ResultView 
                loading=is_loading.get() 
                result=state.get()
            >
                {move || match state.get() {
                    Ok(_) => view! { <div /> }.into_any(),
                    Err(_) => view! { <div /> }.into_any(),
                }}
            </ResultView>
        </div>
    }
}

    }
}

#[component]
fn UtilityCodeBlock(name: String) -> impl IntoView {
    let (state, set_state) = signal(Result::<View, String>::Err("Initial state".to_string()));
    let (is_loading, set_is_loading) = signal(true);

    let name_for_spawn = name.clone();
    spawn_local(async move {
        set_is_loading.set(true);
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "utility": name_for_spawn })).unwrap();
        let result = invoke("get_utility_source", args).await;
        
        match result {
            Ok(val) => {
                if let Some(content) = val.as_string() {
                    set_state.set(Ok(view! { 
                        <div class="code-block">
                            <pre><code>{content}</code></pre>
                        </div> 
                    }.into_any()));
                } else {
                    set_state.set(Err("Could not parse source code as string".to_string()));
                }
            },
            Err(e) => {
                set_state.set(Err(format!("Backend error: {:?}", e)));
            }
        }
        set_is_loading.set(false);
    });

    view! {
        <div class="utility-container">
            <h2>{name} " System Utility"</h2>
            <ResultView 
                loading=is_loading.get() 
                result=state.get()
            >
                {move || match state.get() {
                    Ok(_) => view! { <div /> }.into_any(),
                    Err(_) => view! { <div /> }.into_any(),
                }}
            </ResultView>
        </div>
    }
}

#[component]
fn AccordionItem(title: String, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    view! {
        <div class="accordion-item">
            <button class="accordion-header" on:click=move |_| set_is_open.update(|v| *v = !*v)>
                {title}
                <span class="accordion-icon">{move || if is_open.get() { "−" } else { "+" }}</span>
            </button>
            <div style:display=move || if is_open.get() { "block" } else { "none" }>
                <div class="accordion-body">{children()}</div>
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
                <p>"This is the content for section 1."</p>
            </AccordionItem>
            <AccordionItem title="Section 2".to_string()>
                <p>"This is the content for section 2."</p>
            </AccordionItem>
        </div>
    }
}

#[component]
pub fn DrawerDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    view! {
        <div class="drawer-demo">
            <h2>"Drawer Demo"</h2>
            <button class="open-drawer-btn" on:click=move |_| set_is_open.set(true)>"Open Drawer"</button>
            <div class="drawer-overlay" style:display=move || if is_open.get() { "flex" } else { "none" } on:click=move |_| set_is_open.set(false)>
                <div class="drawer-panel" on:click=|ev| ev.stop_propagation() class:open=move || is_open.get()>
                    <div class="drawer-header">
                        <h3>"Settings"</h3>
                        <button class="close-drawer-btn" on:click=move |_| set_is_open.set(false)>"✕"</button>
                    </div>
                    <div class="drawer-body"><p>"Drawer content goes here."</p></div>
                </div>
            </div>
        </div>
    }
}
