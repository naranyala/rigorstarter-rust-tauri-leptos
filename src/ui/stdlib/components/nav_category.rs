use crate::ui::page_registry::PageInfo;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn NavCategory(
    cat: &'static str,
    pages: Vec<&'static PageInfo>,
    search: RwSignal<String>,
    active_page: RwSignal<Option<&'static str>>,
    sidebar_open: RwSignal<bool>,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(true);

    let pages_for_memo = pages.clone();
    let visible_count = Memo::new(move |_| {
        let query = search.get().to_lowercase();
        pages_for_memo
            .iter()
            .filter(|page| {
                query.is_empty()
                    || page.name.to_lowercase().contains(&query)
                    || page.category.to_lowercase().contains(&query)
                    || page.desc.to_lowercase().contains(&query)
            })
            .count()
    });

    view! {
        <div style="margin-bottom: 1rem;">
            <div
                style="cursor: pointer; display: flex; align-items: center; font-size: 0.65rem; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 0.4rem; padding-left: 0.25rem;"
                on:click=move |_| set_is_expanded.update(|v| *v = !*v)
            >
                <span style="margin-right: 4px; font-size: 0.8rem; width: 14px; display: inline-block; text-align: center;">
                    {move || if is_expanded.get() { "▾" } else { "▸" }}
                </span>
                {move || format!("{} ({})", cat, visible_count.get())}
            </div>
            <div style=move || {
                let display = if is_expanded.get() { "flex" } else { "none" };
                format!("display: {}; flex-direction: column; gap: 2px;", display)
            }>
                {pages.iter().map(|page| {
                    let name = page.name;
                    let page_id = page.id;
                    let cat = page.category;
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
                                let sop = sidebar_open;
                                let closure = Closure::once_into_js(move || {
                                    ap.set(Some(pid));
                                    sop.set(false);
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
}
