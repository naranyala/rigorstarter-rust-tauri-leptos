use crate::core::models::FrontendError;
use leptos::prelude::*;

#[component]
pub fn ResultView(
    loading: bool,
    result: Result<String, FrontendError>,
    children: Children,
) -> impl IntoView {
    if loading {
        view! { <div class="placeholder">"Component is still in development..."</div> }.into_any()
    } else {
        match result {
            Ok(_) => children().into_any(),
            Err(e) => view! { <div class="error-banner">
                <strong>"Error: "</strong> {e.to_string()}
            </div> }
            .into_any(),
        }
    }
}
