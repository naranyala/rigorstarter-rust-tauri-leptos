use crate::core::logic::filter_registry;
use crate::services::{NavigationService, RegistryService, SearchService};
use leptos::prelude::*;

#[component]
pub fn SearchOverlay() -> impl IntoView {
    let registry_service =
        use_context::<RegistryService>().expect("RegistryService should be provided");
    let nav_service =
        use_context::<NavigationService>().expect("NavigationService should be provided");
    let search_service = use_context::<SearchService>().expect("SearchService should be provided");

    let filtered_demos = move || {
        filter_registry(
            &registry_service.items.get(),
            &search_service.query.get(),
            "component",
        )
    };
    let filtered_utils = move || {
        filter_registry(
            &registry_service.items.get(),
            &search_service.query.get(),
            "utility",
        )
    };

    view! {
        <div class="search-overlay" style:display=move || if search_service.is_open.get() { "flex" } else { "none" }>
            <div class="search-container">
                <div class="search-input-wrapper">
                    <input
                        type="text"
                        placeholder="Search components or utilities..."
                        on:input=move |ev| search_service.set_query.set(event_target_value(&ev))
                    />
                    <button class="close-search" on:click=move |_| search_service.close_search()>"✕"</button>
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
                                            nav_service.navigate_to(Some(id.clone()));
                                            search_service.close_search();
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
                                            nav_service.navigate_to(Some(id.clone()));
                                            search_service.close_search();
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
