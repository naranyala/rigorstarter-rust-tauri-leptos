use crate::app::ActivePage;
use crate::ui::pages::accordion::AccordionDemo;
use crate::ui::pages::audio_player::AudioPlayerDemoView;
use crate::ui::pages::audio_recorder::AudioRecorderView;
use crate::ui::pages::calendar::Calendar;
use crate::ui::pages::drawer::DrawerDemo;
use crate::ui::pages::ffi_demo::FfiDemo;
use crate::ui::pages::image_viewer::ImageViewer;
use crate::ui::pages::json_todo::JsonTodoDemo;
use crate::ui::pages::markdown_demo::MarkdownDemo;
use crate::ui::pages::microphone::MicrophoneDemo;
use crate::ui::pages::table_demo::TableDemo;
use crate::ui::pages::tabs::TabsDemo;
use crate::ui::pages::theme_demo::ThemeDemo;
use crate::ui::pages::toast_demo::ToastDemo;
use crate::ui::pages::todo_demo::TodoDemo;
use crate::ui::pages::tree_view::TreeViewDemo;
use leptos::prelude::*;

#[component]
pub fn PageRenderer() -> impl IntoView {
    let active_page = use_context::<ActivePage>().expect("ActivePage not provided");

    view! {
        <div style:display=move || if active_page.get().is_some() { "block" } else { "none" }>
            <div class="page-content">
                <div style="padding: 1rem 2rem 0;">
                    <button
                        class="btn-secondary"
                        style="font-size: 0.85rem; padding: 0.35rem 0.9rem;"
                        on:click=move |_| {
                            leptos::logging::log!("going back to main view");
                            active_page.set(None);
                        }
                    >
                        "← Back to Main"
                    </button>
                </div>
                {move || match active_page.get() {
                    Some("accordion") => view! { <AccordionDemo /> }.into_any(),
                    Some("tabs") => view! { <TabsDemo /> }.into_any(),
                    Some("drawer") => view! { <DrawerDemo /> }.into_any(),
                    Some("tree_view") => view! { <TreeViewDemo /> }.into_any(),
                    Some("table_demo") => view! { <TableDemo /> }.into_any(),
                    Some("calendar") => view! { <Calendar /> }.into_any(),
                    Some("image_viewer") => view! { <ImageViewer /> }.into_any(),
                    Some("theme_demo") => view! { <ThemeDemo /> }.into_any(),
                    Some("toast_demo") => view! { <ToastDemo /> }.into_any(),
                    Some("ffi_demo") => view! { <FfiDemo /> }.into_any(),
                    Some("json_todo") => view! { <JsonTodoDemo /> }.into_any(),
                    Some("markdown_demo") => view! { <MarkdownDemo /> }.into_any(),
                    Some("audio_player") => view! { <AudioPlayerDemoView /> }.into_any(),
                    Some("audio_recorder") => view! { <AudioRecorderView /> }.into_any(),
                    Some("microphone") => view! { <MicrophoneDemo /> }.into_any(),
                    Some("todo_demo") => view! { <TodoDemo /> }.into_any(),
                    _ => view! { <div class="page-not-found"><h2>"Page not found"</h2></div> }.into_any(),
                }}
            </div>
        </div>
    }
}
