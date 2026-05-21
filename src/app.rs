use crate::services::audio::AudioViewModel;
use crate::services::{NavigationService, RegistryService, TodoService};
use crate::ui::pages::accordion::AccordionDemo;
use crate::ui::pages::audio_player::AudioPlayerDemoView;
use crate::ui::pages::audio_recorder::AudioRecorderView;
use crate::ui::pages::calendar::Calendar;
use crate::ui::pages::dashboard::Dashboard;
use crate::ui::pages::drawer::DrawerDemo;
use crate::ui::pages::ffi_demo::FfiDemo;
use crate::ui::pages::image_viewer::ImageViewer;
use crate::ui::pages::json_todo::JsonTodoDemo;
use crate::ui::pages::markdown_demo::MarkdownDemo;
use crate::ui::pages::microphone::MicrophoneDemo;
use crate::ui::pages::table_demo::TableDemo;
use crate::ui::pages::tabs::TabsDemo;
use crate::ui::pages::theme_demo::ThemeDemo;
use crate::ui::pages::thirdparty::{LeafletDemo, MathJaxDemo, MermaidDemo};
use crate::ui::pages::toast_demo::ToastDemo;
use crate::ui::pages::todo_demo::TodoDemo;
use crate::ui::pages::tree_view::TreeViewDemo;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const PAGES: &[(&str, &str, &str)] = &[
    ("Dashboard", "dashboard", "Components"),
    ("Accordion", "accordion", "Components"),
    ("Tabs", "tabs", "Components"),
    ("Drawer", "drawer", "Components"),
    ("Tree View", "tree_view", "Components"),
    ("Table", "table_demo", "Components"),
    ("Calendar", "calendar", "Components"),
    ("Image Viewer", "image_viewer", "Components"),
    ("Theme Demo", "theme_demo", "Components"),
    ("Toast", "toast_demo", "Components"),
    ("FFI Demo", "ffi_demo", "Demos"),
    ("Todo Demo", "todo_demo", "Demos"),
    ("JSON Todo", "json_todo", "Demos"),
    ("Markdown", "markdown_demo", "Demos"),
    ("Leaflet", "leaflet", "3rd Party"),
    ("MathJax", "mathjax", "3rd Party"),
    ("Mermaid", "mermaid", "3rd Party"),
    ("Audio Player", "audio_player", "Media"),
    ("Audio Recorder", "audio_recorder", "Media"),
    ("Microphone", "microphone", "Media"),
];

/// Renders all pages statically; the active page is shown via `style:display`.
/// This avoids Leptos 0.7's `{move || ...}` reactive-block view-reconciliation
/// initialization issue that causes reactive updates to not propagate on first render.
#[component]
fn PageContent(
    active_page: RwSignal<Option<&'static str>>,
    todo_service: TodoService,
    search: RwSignal<String>,
) -> impl IntoView {
    let render_page = move |id: &'static str| match id {
        "dashboard" => view! { <Dashboard /> }.into_any(),
        "accordion" => view! { <AccordionDemo /> }.into_any(),
        "tabs" => view! { <TabsDemo /> }.into_any(),
        "drawer" => view! { <DrawerDemo /> }.into_any(),
        "tree_view" => view! { <TreeViewDemo /> }.into_any(),
        "table_demo" => view! { <TableDemo /> }.into_any(),
        "calendar" => view! { <Calendar /> }.into_any(),
        "image_viewer" => view! { <ImageViewer /> }.into_any(),
        "theme_demo" => view! { <ThemeDemo /> }.into_any(),
        "toast_demo" => view! { <ToastDemo /> }.into_any(),
        "ffi_demo" => view! { <FfiDemo /> }.into_any(),
        "json_todo" => view! { <JsonTodoDemo /> }.into_any(),
        "markdown_demo" => view! { <MarkdownDemo /> }.into_any(),
        "leaflet" => view! { <LeafletDemo /> }.into_any(),
        "mathjax" => view! { <MathJaxDemo /> }.into_any(),
        "mermaid" => view! { <MermaidDemo /> }.into_any(),
        "audio_player" => view! { <AudioPlayerDemoView /> }.into_any(),
        "audio_recorder" => view! { <AudioRecorderView /> }.into_any(),
        "microphone" => view! { <MicrophoneDemo /> }.into_any(),
        "todo_demo" => view! {
            <TodoDemo
                items=todo_service.items
                on_add=Callback::new(move |title| todo_service.add_todo(title))
                on_toggle=Callback::new(move |id| todo_service.toggle_todo(id))
                on_delete=Callback::new(move |id| todo_service.delete_todo(id))
            />
        }
        .into_any(),
        _ => view! { <div>"Not Found"</div> }.into_any(),
    };

    view! {
        <main class="main-content">
            {move || {
                let active = active_page.get();
                if let Some(id) = active {
                    view! {
                        <div class="page-wrapper">
                            {render_page(id)}
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="empty-state">"Select a page from the sidebar"</div> }.into_any()
                }
            }}
            <span
                style=move || {
                    let q = search.get().to_lowercase();
                    let display = if q.is_empty() || PAGES.iter().any(|(n, _, c)| n.to_lowercase().contains(&q) || c.to_lowercase().contains(&q)) {
                        "none"
                    } else {
                        "block"
                    };
                    format!("display: {}; color: var(--text-muted); font-size: 0.8rem; padding: 0.5rem;", display)
                }
            >
                "No results found"
            </span>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let active_page = RwSignal::new(None::<&'static str>);
    let search = RwSignal::new(String::new());
    let is_dark = RwSignal::new(false);

    Effect::new(move |_| {
        let dark = is_dark.get();
        leptos::logging::log!("theme effect: dark={}", dark);
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.class_list().toggle_with_force("dark", dark);
        }
    });

    Effect::new(move |_| {
        let _ = active_page.get();
        if let Some(window) = web_sys::window() {
            if let Some(body) = window.document().and_then(|d| d.body()) {
                let _ = body.offset_height();
            }
        }
    });

    // Force a synchronous reflow and DOM paint on mount to wake up the browser rendering pipeline.
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    loader.remove();
                }

                let w = window.clone();
                let ap = active_page;
                let closure = Closure::once_into_js(move || {
                    if let Some(body) = w.document().and_then(|d| d.body()) {
                        let _ = body.offset_height();
                        if let Ok(event) = web_sys::Event::new("resize") {
                            let _ = w.dispatch_event(&event);
                        }
                    }
                    ap.update(|_| {});
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.unchecked_ref(),
                    100,
                );
            }
        }
    });

    // Keyboard shortcut Ctrl+Shift+R to force page reload for debugging
    if let Some(w) = web_sys::window() {
        let w_for_handler = w.clone();
        let handler =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if ev.ctrl_key() && ev.shift_key() && ev.key() == "R" {
                    ev.prevent_default();
                    leptos::logging::log!("Ctrl+Shift+R: reloading page");
                    let _ = w_for_handler.location().reload();
                }
            });
        let _ = w.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        std::mem::forget(handler);
    }

    let registry_service = RegistryService::new();
    let nav_service = NavigationService::new();
    let todo_service = TodoService::new();
    let audio_vm = AudioViewModel::new();

    provide_context(registry_service);
    provide_context(nav_service);
    provide_context(todo_service);
    provide_context(audio_vm);

    registry_service.load_registry();

    // Pre-compute page groupings once (static).
    let groups = {
        #[allow(clippy::type_complexity)]
        let mut groups: Vec<(&str, Vec<(&str, &str, &str)>)> = Vec::new();
        for p in PAGES {
            let cat = p.2;
            if let Some((_, list)) = groups.iter_mut().find(|(c, _)| *c == cat) {
                list.push(*p);
            } else {
                groups.push((cat, vec![*p]));
            }
        }
        groups
    };

    view! {
        <div class="app-container">
            <nav class="sidebar">
                <div class="sidebar-brand" style="cursor: pointer;" on:click=move |_| {
                    search.set(String::new());
                    active_page.set(None);
                }>
                    "RigorStarter"
                </div>
                <div class="sidebar-content">
                    <input
                        type="text"
                        placeholder="Search pages..."
                        style="width: 100%; padding: 0.6rem 0.75rem; border: 1px solid var(--border-color); border-radius: 0.5rem; background: var(--secondary-bg); color: var(--text-main); font-size: 0.9rem; outline: none; box-sizing: border-box;"
                        on:input=move |ev| {
                            search.set(event_target_value(&ev));
                        }
                    />

                    <div class="sidebar-nav">
                        // Render all pages statically; filter via reactive style:display.
                        // This avoids the {move || ...} reactive-block approach which has a
                        // Leptos 0.7 view-reconciliation initialization issue on first render.
                        {groups.iter().map(|(cat, group)| {
                            view! {
                                <div style="margin-bottom: 1rem;">
                                    <div style="font-size: 0.65rem; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 0.4rem; padding-left: 0.25rem;">
                                        {*cat}
                                    </div>
                                    <div style="display: flex; flex-direction: column; gap: 2px;">
                                         {group.iter().map(|(name, page_id, _)| {
                                             let name = *name;
                                             let page_id = *page_id;
                                             let cat = *cat;
                                             let q = search;

                                             view! {
                                                 <button
                                                     class="page-list-item"
                                                     style=move || {
                                                         let query = q.get().to_lowercase();
                                                         let display = if query.is_empty() || name.to_lowercase().contains(&query) || cat.to_lowercase().contains(&query) {
                                                             "block"
                                                         } else {
                                                             "none"
                                                         };
                                                         format!("text-align: left; width: 100%; display: {};", display)
                                                     }
                                                     on:click=move |_| {
                                                         leptos::logging::log!("Sidebar click: navigating to: {}", page_id);
                                                         let ap = active_page;
                                                         let pid = page_id;
                                                         let closure = Closure::once_into_js(move || {
                                                             ap.set(Some(pid));
                                                         });
                                                         let _ = web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref());
                                                     }
                                                 >
                                                     <span style="font-weight: 500;">{name}</span>
                                                 </button>
                                             }
                                         }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
                <div class="sidebar-footer">
                    <button class="btn-icon" on:click=move |_| {
                        leptos::logging::log!("Theme toggle clicked");
                        is_dark.update(|d| *d = !*d);
                    }>
                        {move || if is_dark.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </nav>

            <PageContent active_page todo_service search />
        </div>
    }
}
