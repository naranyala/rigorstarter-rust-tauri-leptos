use crate::models::RegistryItem;
use leptos::prelude::*;

#[component]
pub fn Dashboard(
    items: ReadSignal<Vec<RegistryItem>>,
    set_active_demo: Callback<Option<String>>,
) -> impl IntoView {
    let pinned = move || {
        items
            .get()
            .iter()
            .filter(|i| i.status == "pinned")
            .cloned()
            .collect::<Vec<_>>()
    };
    let in_dev = move || {
        items
            .get()
            .iter()
            .filter(|i| i.status == "in-development")
            .cloned()
            .collect::<Vec<_>>()
    };
    let archives = move || {
        items
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
                            <a class="simple-item-link" on:click=move |_| set_active_demo.run(Some(id.clone()))>
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
