use crate::core::logic::filter_registry;
use leptos::prelude::*;

#[component]
pub fn SearchOverlay(
    items: ReadSignal<Vec<crate::core::models::RegistryItem>>,
    query: ReadSignal<String>,
    is_open: ReadSignal<bool>,
    on_query_change: Callback<String>,
    on_close: Callback<()>,
    on_item_select: Callback<String>,
) -> impl IntoView {
    let filtered_demos = move || filter_registry(&items.get(), &query.get(), "component");
    let filtered_utils = move || filter_registry(&items.get(), &query.get(), "utility");

    view! {
        <div class="search-overlay" style:display=move || if is_open.get() { "flex" } else { "none" }>
            <div class="search-container">
                <div class="search-input-wrapper">
                    <input
                        type="text"
                        placeholder="Search components or utilities..."
                        on:input=move |ev| on_query_change.run(event_target_value(&ev))
                    />
                    <button class="close-search" on:click=move |_| on_close.run(())>"✕"</button>
                </div>
                <div class="search-results">
                    {move || {
                        let filtered = filtered_demos();
                        let utils = filtered_utils();

                        if filtered.is_empty() && utils.is_empty() {
                            view! { <div class="no-results">"No results found"</div> }.into_any()
                        } else {
                            let mut results = Vec::new();

                            if !filtered.is_empty() {
                                results.push(view! { <div class="search-section-title">"Components"</div> }.into_any());
                                let component_results = filtered.into_iter().map(|(name, id)| {
                                    let id = id.to_string();
                                    view! {
                                        <div class="search-item" on:click=move |_| {
                                            on_item_select.run(id.clone());
                                            on_close.run(());
                                        }>
                                            <span class="item-name">{name}</span>
                                            <span class="item-action">"View →"</span>
                                        </div>
                                    }.into_any()
                                }).collect::<Vec<_>>();
                                results.extend(component_results);
                            }

                            if !utils.is_empty() {
                                results.push(view! { <div class="search-section-title">"System Utilities"</div> }.into_any());
                                let util_results = utils.into_iter().map(|(name, id)| {
                                    let id = id.to_string();
                                    view! {
                                        <div class="search-item" on:click=move |_| {
                                            on_item_select.run(id.clone());
                                            on_close.run(());
                                        }>
                                            <span class="item-name">{name}</span>
                                            <span class="item-action">"Source →"</span>
                                        </div>
                                    }.into_any()
                                }).collect::<Vec<_>>();
                                results.extend(util_results);
                            }

                            results.into_view().into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
