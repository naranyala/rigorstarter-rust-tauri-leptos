use leptos::prelude::*;

#[component]
pub fn ResultView(
    loading: bool,
    result: Result<String, String>,
    children: Children,
) -> impl IntoView {
    if loading {
        view! { <div class="placeholder">"Component is still in development..."</div> }.into_any()
    } else {
        match result {
            Ok(_) => children().into_any(),
            Err(e) => view! { <div class="error-msg" style="padding: 1rem; border: 1px solid #f5c6cb; background: #f8d7da; color: #721c24; border-radius: 4px;">
                <strong>"Error: "</strong> {e}
            </div> }.into_any(),
        }
    }
}
