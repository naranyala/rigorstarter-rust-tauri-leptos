use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let sidebar_service = use_context::<crate::services::SidebarService>()
        .expect("SidebarService should be provided");

    view! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <h2>"Details"</h2>
                <button class="close-btn" on:click=move |_| sidebar_service.toggle_sidebar()>
                    "✕"
                </button>
            </div>
            <div class="sidebar-content">
                <p>"This is a reactive right-sidebar. You can put details, settings, or metadata here."</p>
            </div>
        </aside>
    }
}
