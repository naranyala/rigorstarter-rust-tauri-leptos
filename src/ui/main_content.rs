use crate::services::{NavigationService, RegistryService, TodoService};
use crate::ui::codeblock::CodeBlockView;
use crate::ui::lightbox::Lightbox;
use crate::ui::pages::accordion::AccordionDemo;
use crate::ui::pages::audio_player::AudioPlayerDemoView;
use crate::ui::pages::audio_recorder::AudioRecorderView;
use crate::ui::pages::calendar::Calendar;
use crate::ui::pages::dashboard::Dashboard;
use crate::ui::pages::drawer::DrawerDemo;
use crate::ui::pages::image_viewer::ImageViewer;
use crate::ui::pages::json_todo::JsonTodoDemo;
use crate::ui::pages::markdown_demo::MarkdownDemo;
use crate::ui::pages::microphone::MicrophoneDemo;
use crate::ui::pages::table_demo::TableDemo;
use crate::ui::pages::theme_demo::ThemeDemo;
use crate::ui::pages::toast_demo::ToastDemo;
use crate::ui::pages::todo_demo::TodoDemo;
use crate::ui::pages::tree_view::TreeViewDemo;
use crate::ui::utility_code_block::UtilityCodeBlock;
use leptos::prelude::*;

#[component]
pub fn MainContent() -> impl IntoView {
    let registry_service =
        use_context::<RegistryService>().expect("RegistryService should be provided");
    let nav_service =
        use_context::<NavigationService>().expect("NavigationService should be provided");
    let todo_service = use_context::<TodoService>().expect("TodoService should be provided");

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

            // 3. Audio Player Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("audio_player") { "block" } else { "none" }>
                <AudioPlayerDemoView />
            </div>

            // 4. Audio Recorder Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("audio_recorder") { "block" } else { "none" }>
                <AudioRecorderView />
            </div>

            // 5. Calendar Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("calendar") { "block" } else { "none" }>
                <Calendar />
            </div>

            // 6. Code Block Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("codeblock") { "block" } else { "none" }>
                <CodeBlockView />
            </div>

            // 7. Drawer Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("drawer") { "block" } else { "none" }>
                <DrawerDemo />
            </div>

            // 8. Image Viewer Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("image_viewer") { "block" } else { "none" }>
                <ImageViewer />
            </div>

            // 9. Lightbox Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("lightbox") { "block" } else { "none" }>
                <Lightbox />
            </div>

            // 10. Microphone Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("microphone") { "block" } else { "none" }>
                <MicrophoneDemo />
            </div>

            // 11. Tabs Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("tabs") { "block" } else { "none" }>
                <crate::ui::pages::tabs::TabsDemo />
            </div>

            // 12. Modal Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("modal") { "block" } else { "none" }>
                <crate::ui::modal::ModalDemo />
            </div>

            // 13. Theme Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("theme") { "block" } else { "none" }>
                <ThemeDemo />
            </div>

            // 14. Toast Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("toast") { "block" } else { "none" }>
                <ToastDemo />
            </div>

            // 15. Todo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("todo") { "block" } else { "none" }>
                {move || {
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

            // 16. Tree View Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("tree_view") { "block" } else { "none" }>
                <TreeViewDemo />
            </div>

            // 17. JSON Todo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("json_todo") { "block" } else { "none" }>
                <JsonTodoDemo />
            </div>

            // 18. Table Demo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("table_demo") { "block" } else { "none" }>
                <TableDemo />
            </div>

            // 19. Markdown Demo Slot
            <div style:display=move || if nav_service.active_demo.get().as_deref() == Some("markdown_demo") { "block" } else { "none" }>
                <MarkdownDemo />
            </div>

            // 20. Dynamic Utility Slot
            <div style:display=move || {
                let active = nav_service.active_demo.get();
                if active.is_some() && active.as_deref() != Some("accordion") && active.as_deref() != Some("audio_player") && active.as_deref() != Some("audio_recorder") && active.as_deref() != Some("calendar") && active.as_deref() != Some("codeblock") && active.as_deref() != Some("drawer") && active.as_deref() != Some("image_viewer") && active.as_deref() != Some("lightbox") && active.as_deref() != Some("microphone") && active.as_deref() != Some("tabs") && active.as_deref() != Some("modal") && active.as_deref() != Some("theme") && active.as_deref() != Some("toast") && active.as_deref() != Some("todo") && active.as_deref() != Some("json_todo") && active.as_deref() != Some("tree_view") && active.as_deref() != Some("table_demo") && active.as_deref() != Some("markdown_demo") {
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
