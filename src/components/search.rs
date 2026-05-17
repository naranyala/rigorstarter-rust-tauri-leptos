use crate::logic::filter_registry;
use crate::models::RegistryItem;
use leptos::prelude::*;

#[component]
pub fn SearchOverlay(
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
    registry: ReadSignal<Vec<RegistryItem>>,
    set_active_demo: Callback<Option<String>>,
) -> impl IntoView {
    let filtered_demos = move || filter_registry(&registry.get(), &search_query.get(), "component");

    let filtered_utils = move || filter_registry(&registry.get(), &search_query.get(), "utility");

    view! {
        <div class="search-overlay" style:display=move || if is_open.get() { "flex" } else { "none" }>
            <div class="search-container">
                <div class="search-input-wrapper">
                    <input
                        type="text"
                        placeholder="Search components or utilities..."
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                    <button class="close-search" on:click=move |_| set_is_open.set(false)>"✕"</button>
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
                                            set_active_demo.run(Some(id.clone()));
                                            set_is_open.set(false);
                                            set_search_query.set(String::new());
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
                                            set_active_demo.run(Some(id.clone()));
                                            set_is_open.set(false);
                                            set_search_query.set(String::new());
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
