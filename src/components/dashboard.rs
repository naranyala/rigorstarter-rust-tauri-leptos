use leptos::prelude::*;
use crate::models::RegistryItem;

#[component]
pub fn Dashboard(
    items: ReadSignal<Vec<RegistryItem>>,
    set_active_demo: Callback<Option<String>>,
) -> impl IntoView {
    let pinned = move || items.get().iter().filter(|i| i.status == "pinned").cloned().collect::<Vec<_>>();
    let in_dev = move || items.get().iter().filter(|i| i.status == "in-development").cloned().collect::<Vec<_>>();
    let archives = move || items.get().iter().filter(|i| i.status == "archives").cloned().collect::<Vec<_>>();

    let render_all_list = move || {
        let all_items = items.get();
        view! {
            <div class="dashboard-section">
                <h3 class="section-title">"All Components & Utilities"</h3>
                <div class="all-items-list">
                    {all_items.into_iter().map(|item| {
                        let id = item.id.clone();
                        let name = item.name.clone();
                        view! {
                            <a class="all-item-link" on:click=move |_| set_active_demo.run(Some(id.clone()))>
                                {name}
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        }
    };

    let render_cards = move |list: Vec<RegistryItem>, category_label: String, status_class: String| {
        view! {
            <div class="dashboard-section">
                <h3 class="section-title">{category_label}</h3>
                <div class="dashboard-grid">
                    {list.into_iter().map(|item| {
                        let id = item.id.clone();
                        view! {
                            <div class="dashboard-card" on:click=move |_| set_active_demo.run(Some(id.clone()))>
                                <div class="card-header">
                                    <span class="card-category">{item.category}</span>
                                    <span class=format!("card-status {}", status_class)>{item.status}</span>
                                </div>
                                <div class="card-body">
                                    <span class="card-name">{item.name}</span>
                                    <span class="card-action">"Explore →"</span>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        }
    };

    view! {
        <div class="dashboard-welcome">
            <div class="welcome-header">
                <h1>"Welcome to the Component Library"</h1>
                <p>"A collection of reusable UI components and system utilities built with Rust, Tauri and Leptos."</p>
            </div>

            <div class="dashboard-content">
                {move || {
                    let mut content = Vec::new();
                    content.push(render_cards(pinned(), "Featured & Pinned".to_string(), "status-pinned".to_string()).into_any());
                    content.push(render_cards(in_dev(), "Coming Soon / In Dev".to_string(), "status-dev".to_string()).into_any());
                    content.push(render_cards(archives(), "Archived".to_string(), "status-archive".to_string()).into_any());
                    content.push(render_all_list().into_any());
                    content.into_view().into_any()
                }}
            </div>
        </div>
    }
}
