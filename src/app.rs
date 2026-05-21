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
    _tick: RwSignal<u32>,
) -> impl IntoView {
    // Map of page IDs to NodeRefs
    let mut refs = std::collections::HashMap::new();
    
    // We'll define the pages and their IDs
    let page_ids = [
        "dashboard", "accordion", "tabs", "drawer", "tree_view", "table_demo",
        "calendar", "image_viewer", "theme_demo", "toast_demo", "ffi_demo",
        "json_todo", "markdown_demo", "leaflet", "mathjax", "mermaid",
        "audio_player", "audio_recorder", "microphone", "todo_demo",
    ];

    // Create a NodeRef and an Effect for each page
    let page_elements = page_ids.into_iter().map(|id| {
        let node_ref = NodeRef::<leptos::html::Div>::new();
        let node_ref_clone = node_ref.clone();
        
        Effect::new(move |_| {
            if let Some(el) = node_ref_clone.get() {
                let display = if active_page.get() == Some(id) { "block" } else { "none" };
                let _ = el.style().set_property("display", display);
            }
        });
        (id, node_ref)
    }).collect::<Vec<_>>();

    // Empty state ref
    let empty_ref = NodeRef::<leptos::html::Div>::new();
    let empty_ref_clone = empty_ref.clone();
    Effect::new(move |_| {
        if let Some(el) = empty_ref_clone.get() {
            let display = if active_page.get().is_none() { "block" } else { "none" };
            let _ = el.style().set_property("display", display);
        }
    });

    view! {
        <main class="main-content">
            <div node_ref=empty_ref class="empty-state">"Select a page from the sidebar"</div>
            <div node_ref=page_elements[0].1 style="display: none;"><Dashboard /></div>
            <div node_ref=page_elements[1].1 style="display: none;"><AccordionDemo /></div>
            <div node_ref=page_elements[2].1 style="display: none;"><TabsDemo /></div>
            <div node_ref=page_elements[3].1 style="display: none;"><DrawerDemo /></div>
            <div node_ref=page_elements[4].1 style="display: none;"><TreeViewDemo /></div>
            <div node_ref=page_elements[5].1 style="display: none;"><TableDemo /></div>
            <div node_ref=page_elements[6].1 style="display: none;"><Calendar /></div>
            <div node_ref=page_elements[7].1 style="display: none;"><ImageViewer /></div>
            <div node_ref=page_elements[8].1 style="display: none;"><ThemeDemo /></div>
            <div node_ref=page_elements[9].1 style="display: none;"><ToastDemo /></div>
            <div node_ref=page_elements[10].1 style="display: none;"><FfiDemo /></div>
            <div node_ref=page_elements[11].1 style="display: none;"><JsonTodoDemo /></div>
            <div node_ref=page_elements[12].1 style="display: none;"><MarkdownDemo /></div>
            <div node_ref=page_elements[13].1 style="display: none;"><LeafletDemo /></div>
            <div node_ref=page_elements[14].1 style="display: none;"><MathJaxDemo /></div>
            <div node_ref=page_elements[15].1 style="display: none;"><MermaidDemo /></div>
            <div node_ref=page_elements[16].1 style="display: none;"><AudioPlayerDemoView /></div>
            <div node_ref=page_elements[17].1 style="display: none;"><AudioRecorderView /></div>
            <div node_ref=page_elements[18].1 style="display: none;"><MicrophoneDemo /></div>
            <div node_ref=page_elements[19].1 style="display: none;">
                <TodoDemo
                    items=todo_service.items
                    on_add=Callback::new(move |title| todo_service.add_todo(title))
                    on_toggle=Callback::new(move |id| todo_service.toggle_todo(id))
                    on_delete=Callback::new(move |id| todo_service.delete_todo(id))
                />
            </div>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let active_page = RwSignal::new(None::<&'static str>);
    let search = RwSignal::new(String::new());
    let is_dark = RwSignal::new(false);
    let tick = RwSignal::new(0u32);

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

    // Force a synchronous reflow and DOM paint on mount to wake up the browser rendering pipeline.
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(loader) = document.get_element_by_id("app-loader") {
                    loader.remove();
                }

                let w = window.clone();
                let ap = active_page;
                let t = tick;
                let closure = Closure::once_into_js(move || {
                    if let Some(body) = w.document().and_then(|d| d.body()) {
                        let _ = body.offset_height();
                        if let Ok(event) = web_sys::Event::new("resize") {
                            let _ = w.dispatch_event(&event);
                        }
                    }
                    ap.update(|_| {});
                    t.update(|val| *val += 1);
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
                                             
                                             let node_ref = NodeRef::<leptos::html::Button>::new();
                                             let node_ref_clone = node_ref.clone();
                                             Effect::new(move |_| {
                                                 if let Some(el) = node_ref_clone.get() {
                                                     let query = q.get().to_lowercase();
                                                     let display = if query.is_empty() || name.to_lowercase().contains(&query) || cat.to_lowercase().contains(&query) {
                                                         "block"
                                                     } else {
                                                         "none"
                                                     };
                                                     let _ = el.style().set_property("display", display);
                                                 }
                                             });

                                             view! {
                                                 <button
                                                     node_ref=node_ref
                                                     class="page-list-item"
                                                     style="text-align: left; width: 100%;"
                                                     on:click=move |_| {
                                                         leptos::logging::log!("navigating to: {}", page_id);
                                                         active_page.set(Some(page_id));
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
                        // "No results found" message - shown only when search has no matches.
                        <span
                            style:display=move || {
                                let _ = tick.get();
                                let q = search.get().to_lowercase();
                                if q.is_empty() { "none" } else {
                                    let has_match = PAGES.iter().any(|(n, _, c)| n.to_lowercase().contains(&q) || c.to_lowercase().contains(&q));
                                    if has_match { "none" } else { "block" }
                                }
                            }
                            style="color: var(--text-muted); font-size: 0.8rem; padding: 0.5rem;"
                        >
                            "No results found"
                        </span>
                    </div>
                </div>
                <div class="sidebar-footer">
                    <button class="btn-icon" on:click=move |_| is_dark.update(|d| *d = !*d)>
                        {move || if is_dark.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </nav>

            <PageContent active_page todo_service tick />
        </div>
    }
}
