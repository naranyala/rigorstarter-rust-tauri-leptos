use crate::components::accordion::AccordionDemo;
use crate::components::dashboard::Dashboard;
use crate::components::drawer::DrawerDemo;
use crate::components::theme_demo::ThemeDemo;
use crate::components::toast_demo::ToastDemo;
use crate::components::utility_code_block::UtilityCodeBlock;
use crate::services::{NavigationService, RegistryService};
use leptos::prelude::*;

#[component]
pub fn MainContent() -> impl IntoView {
    let registry_service =
        use_context::<RegistryService>().expect("RegistryService should be provided");
    let nav_service =
        use_context::<NavigationService>().expect("NavigationService should be provided");

    // Memoize the current item to prevent re-searching the registry on every render
    let current_item = Memo::new(move |_| {
        nav_service.active_demo.get().as_ref().and_then(|id| {
            registry_service
                .items
                .get()
                .iter()
                .find(|i| i.id == *id)
                .cloned()
        })
    });

    view! {
        <div class="main-content-wrapper">
            // 1. Dashboard Slot
            <div style:display=move || if nav_service.active_demo.get().is_none() { "block" } else { "none" }>
                <Show
                    when=move || registry_service.is_loading.get()
                    fallback=move || view! { <Dashboard /> }.into_any()
                >
                    <div class="placeholder">"Loading dashboard..."</div>
                </Show>
            </div>

            // 2. Accordion Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("accordion") { "block" } else { "none" }>
                <AccordionDemo />
            </div>

            // 3. Drawer Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("drawer") { "block" } else { "none" }>
                <DrawerDemo />
            </div>

            // 4. Tabs Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("tabs") { "block" } else { "none" }>
                <crate::components::tabs::TabsDemo />
            </div>

            // 5. Modal Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("modal") { "block" } else { "none" }>
                <crate::components::modal::ModalDemo />
            </div>

            // 6. Theme Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("theme") { "block" } else { "none" }>
                <ThemeDemo />
            </div>

            // 7. Toast Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("toast") { "block" } else { "none" }>
                <ToastDemo />
            </div>

            // 8. Dynamic Utility Slot
            <div style:display=move || {
                let active = nav_service.active_demo.get();
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
