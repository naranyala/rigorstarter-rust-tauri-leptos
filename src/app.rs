use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[component]
pub fn App() -> impl IntoView {
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

    view! {
        <div class="container">
            <nav class="navbar">
                <button class="nav-brand-btn" on:click=move |_| set_active_demo.set(None)>"Component Library"</button>
                <button on:click=move |_| set_active_demo.set(Some("accordion".to_string()))>"Accordion"</button>
                <button on:click=move |_| set_active_demo.set(Some("drawer".to_string()))>"Drawer"</button>
                <button on:click=move |_| set_active_demo.set(Some("network".to_string()))>"Network"</button>
            </nav>

            <main style="padding: 2rem; text-align: center;">
                {move || match active_demo.get().as_deref() {
                    Some("accordion") => view! { <AccordionDemo /> }.into_any(),
                    Some("drawer") => view! { <DrawerDemo /> }.into_any(),
                    Some("network") => view! { <UtilityCodeBlock name="network".to_string() /> }.into_any(),
                    Some(_) => view! { <div>"Coming Soon..."</div> }.into_any(),
                    None => view! { <h1>"Welcome to the Component Library"</h1> }.into_any(),
                }}
            </main>
        </div>
    }
}

#[component]
fn UtilityCodeBlock(name: String) -> impl IntoView {
    let (code, set_code) = signal("Loading code...".to_string());

    let name_for_spawn = name.clone();
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "utility": name_for_spawn })).unwrap();
        let result = invoke("get_utility_source", args).await;
        if let Ok(val) = result {
            set_code.set(val.as_string().unwrap_or_else(|| "Could not parse source code".to_string()));
        } else {
            set_code.set("Error loading source code".to_string());
        }
    });

    view! {
        <div class="utility-container">
            <h2>{name} " System Utility"</h2>
            <div class="code-block">
                <pre><code>{move || code.get()}</code></pre>
            </div>
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
