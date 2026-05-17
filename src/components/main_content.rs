use crate::components::accordion::AccordionDemo;
use crate::components::dashboard::Dashboard;
use crate::components::drawer::DrawerDemo;
use crate::components::theme_demo::ThemeDemo;
use crate::components::toast_demo::ToastDemo;
use crate::components::utility_code_block::UtilityCodeBlock;
use crate::models::RegistryItem;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[component]
pub fn MainContent(
    active_demo: ReadSignal<Option<String>>,
    set_active_demo: Callback<Option<String>>,
    registry: ReadSignal<Vec<RegistryItem>>,
    is_loading_registry: ReadSignal<bool>,
) -> impl IntoView {
    // Memoize the current item to prevent re-searching the registry on every render
    let current_item = Memo::new(move |_| {
        active_demo
            .get()
            .as_ref()
            .and_then(|id| registry.get().iter().find(|i| i.id == *id).cloned())
    });

    view! {
        <div style="padding: 2rem; text-align: center;">
            // 1. Dashboard Slot
            <div style:display=move || if active_demo.get().is_none() { "block" } else { "none" }>
                <Show
                    when=move || is_loading_registry.get()
                    fallback=move || view! { <Dashboard items=registry set_active_demo=set_active_demo /> }.into_any()
                >
                    <div class="placeholder">"Loading dashboard..."</div>
                </Show>
            </div>

            // 2. Accordion Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("accordion") { "block" } else { "none" }>
                <AccordionDemo />
            </div>

            // 3. Drawer Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("drawer") { "block" } else { "none" }>
                <DrawerDemo />
            </div>

            // 4. Tabs Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("tabs") { "block" } else { "none" }>
                <crate::components::tabs::TabsDemo />
            </div>

            // 5. Modal Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("modal") { "block" } else { "none" }>
                <crate::components::modal::ModalDemo />
            </div>

            // 6. Theme Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("theme") { "block" } else { "none" }>
                <ThemeDemo />
            </div>

            // 7. Toast Slot
            <div style:display=move || if active_demo.get().as_deref() == Some("toast") { "block" } else { "none" }>
                <ToastDemo />
            </div>

            // 8. Dynamic Utility Slot
            <div style:display=move || {
                let active = active_demo.get();
                if active.is_some() && active.as_deref() != Some("accordion") && active.as_deref() != Some("drawer") && active.as_deref() != Some("tabs") && active.as_deref() != Some("modal") && active.as_deref() != Some("theme") && active.as_deref() != Some("toast") {
                    "block"
                } else {
                    "none"
                }
            }>
                 {move || {
                    if let Some(item) = current_item.get() {
                        view! { <UtilityCodeBlock name=item.name.clone() id=item.id.clone() /> }.into_any()
                    } else {
                        view! { <div class="error-msg">"Item not found"</div> }.into_any()
                    }
                 }}
            </div>
        </div>
    }
}
