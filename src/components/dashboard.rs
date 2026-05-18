use crate::core::models::RegistryItem;
use crate::services::{NavigationService, RegistryService};
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let registry_service =
        use_context::<RegistryService>().expect("RegistryService should be provided");
    let nav_service =
        use_context::<NavigationService>().expect("NavigationService should be provided");

    let pinned = move || {
        registry_service
            .items
            .get()
            .iter()
            .filter(|i| i.status == "pinned")
            .cloned()
            .collect::<Vec<_>>()
    };
    let in_dev = move || {
        registry_service
            .items
            .get()
            .iter()
            .filter(|i| i.status == "in-development")
            .cloned()
            .collect::<Vec<_>>()
    };
    let archives = move || {
        registry_service
            .items
            .get()
            .iter()
            .filter(|i| i.status == "archives")
            .cloned()
            .collect::<Vec<_>>()
    };

    let render_simple_list = move |list: Vec<RegistryItem>, category_label: String| {
        view! {
            <div class="dashboard-section">
                <h3 class="section-title">{category_label}</h3>
                <div class="simple-items-list">
                    {list.into_iter().map(|item| {
                        let id = item.id.clone();
                        let name = item.name.clone();
                        view! {
                            <a class="simple-item-link" on:click=move |_| nav_service.navigate_to(Some(id.clone()))>
                                {name}
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        }
    };

    view! {
        <div class="dashboard-welcome">
            <div class="welcome-header">
                <h1>"Component Library"</h1>
                <p>"A collection of reusable UI components and system utilities."</p>
            </div>

            <div class="dashboard-content">
                {move || {
                    vec![
                        render_simple_list(pinned(), "Featured & Pinned".to_string()).into_any(),
                        render_simple_list(in_dev(), "Coming Soon".to_string()).into_any(),
                        render_simple_list(archives(), "Archived".to_string()).into_any(),
                    ].into_view().into_any()
                }}
            </div>
        </div>
    }
}
