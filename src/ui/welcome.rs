use crate::app::ActivePage;
use crate::ui::page_registry::{grouped_pages, PAGES};
use leptos::prelude::*;

#[component]
pub fn WelcomeDashboard() -> impl IntoView {
    let active_page = use_context::<ActivePage>().expect("ActivePage not provided");
    let groups = grouped_pages();
    let total = PAGES.len();

    view! {
        <div style:display=move || if active_page.get().is_none() || active_page.get() == Some("welcome") { "block" } else { "none" }>
            <div class="welcome-container">
                <div class="welcome-header">
                    <h1 class="welcome-title">"Component Exploration"</h1>
                    <p class="welcome-subtitle">{format!("Discover {} high-performance Rust-Leptos components", total)}</p>
                </div>

                <div class="welcome-grid">
                    {groups.into_iter().map(|(cat, pages)| {
                        let count = pages.len();
                        let first_page_id = pages.first().map(|p| p.id).unwrap_or("");
                        view! {
                            <div class="welcome-card" on:click=move |_| {
                                active_page.set(Some(first_page_id));
                            }>
                                <div class="welcome-card-header">
                                    <span class="welcome-card-name">{cat}</span>
                                    <span style="background: var(--primary); color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.75rem;">
                                        {count}
                                    </span>
                                </div>
                                 <div class="welcome-card-desc">
                                     {format!("{} components available", count)}
                                 </div>
                                 <div class="welcome-card-footer"></div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
