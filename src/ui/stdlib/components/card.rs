use leptos::prelude::*;
use crate::ui::stdlib::theme::Theme;

#[component]
pub fn Card(
    #[prop(into)] class: MaybeInto<String>,
    children: Children,
) -> impl IntoView {
    let class = class.into();
    
    view! {
        <div 
            class=move || format!("card-container {}", class)
            style="background: var(--secondary-bg); border: 1px solid var(--border-color); border-radius: 0.75rem; padding: 1.5rem; box-shadow: var(--shadow-md);"
        >
            {children()}
        </div>
    }
}
