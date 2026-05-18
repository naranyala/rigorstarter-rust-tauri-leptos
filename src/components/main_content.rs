use crate::components::accordion::AccordionDemo;
use crate::components::dashboard::Dashboard;
use crate::components::drawer::DrawerDemo;
use crate::components::json_todo_demo::JsonTodoDemo;
use crate::components::markdown_demo::MarkdownDemo;
use crate::components::table_demo::TableDemo;
use crate::components::theme_demo::ThemeDemo;
use crate::components::toast_demo::ToastDemo;
use crate::components::todo_demo::TodoDemo;
use crate::components::tree_view_demo::TreeViewDemo;
use crate::components::utility_code_block::UtilityCodeBlock;
use crate::services::{NavigationService, RegistryService, TodoService};
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

            // 8. Todo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("todo") { "block" } else { "none" }>
                {move || {
                    let todo_service = use_context::<TodoService>().expect("TodoService should be provided");
                    view! {
                        <TodoDemo
                            items=todo_service.items
                            on_add=Callback::new(move |title| todo_service.add_todo(title))
                            on_toggle=Callback::new(move |id| todo_service.toggle_todo(id))
                            on_delete=Callback::new(move |id| todo_service.delete_todo(id))
                        />
                    }
                }}
            </div>

            // 9. Tree View Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("tree_view") { "block" } else { "none" }>
                <TreeViewDemo />
            </div>

            // 10. JSON Todo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("json_todo") { "block" } else { "none" }>
                <JsonTodoDemo />
            </div>

            // 11. Table Demo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("table_demo") { "block" } else { "none" }>
                <TableDemo />
            </div>

            // 12. Markdown Demo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("markdown_demo") { "block" } else { "none" }>
                <MarkdownDemo />
            </div>

            // 13. Dynamic Utility Slot
            <div style:display=move || {
                let active = nav_service.active_demo.get();
                if active.is_some() && active.as_deref() != Some("accordion") && active.as_deref() != Some("drawer") && active.as_deref() != Some("tabs") && active.as_deref() != Some("modal") && active.as_deref() != Some("theme") && active.as_deref() != Some("toast") && active.as_deref() != Some("todo") && active.as_deref() != Some("json_todo") && active.as_deref() != Some("tree_view") && active.as_deref() != Some("table_demo") && active.as_deref() != Some("markdown_demo") {
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
