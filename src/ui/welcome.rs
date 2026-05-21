use crate::app::ActivePage;
use crate::ui::page_registry::{grouped_pages, PAGES};
use leptos::prelude::*;

#[component]
pub fn WelcomeDashboard() -> impl IntoView {
    let active_page = use_context::<ActivePage>().expect("ActivePage not provided");
    let groups = grouped_pages();
    let total = PAGES.len();

    view! {
        <div style:display=move || if active_page.0.get().is_none() { "block" } else { "none" }>
            <div class="page-content" style="max-width: 640px; margin: 0 auto; padding: 2rem 1.5rem;">
                <div style="margin-bottom: 1.5rem;">
                    <h1 style="font-size: 1.25rem; font-weight: 700; color: var(--text-main); margin: 0 0 0.25rem;">"Page Index"</h1>
                    <p style="font-size: 0.8rem; color: var(--text-muted); margin: 0;">{total.to_string() + " pages available"}</p>
                </div>

                {groups.into_iter().map(|(cat, pages)| {
                    view! {
                        <div style="margin-bottom: 1.25rem;">
                            <div style="font-size: 0.7rem; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 0.4rem; padding-left: 0.25rem;">
                                {cat}
                                <span style="font-weight: 400; margin-left: 0.3rem;">{"("}{pages.len().to_string()}{")"}</span>
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 2px;">
                                {pages.into_iter().map(|p| {
                                    let id = p.id;
                                    view! {
                                        <button
                                            class="page-list-item"
                                            on:click=move |_| {
                                                leptos::logging::log!("navigating to: {}", id);
                                                active_page.0.set(Some(id));
                                            }
                                        >
                                            <span style="font-weight: 500;">{p.name}</span>
                                            <span class="page-list-desc">{p.desc}</span>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
