use leptos::prelude::*;

#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <div style="padding: 20px; max-width: 1200px; margin: 0 auto; font-family: sans-serif;">
            {children()}
        </div>
    }
}

#[component]
pub fn Surface(
    #[prop(optional)] outline: bool,
    #[prop(optional)] padding: bool,
    children: Children,
) -> impl IntoView {
    let style = format!(
        "border: {}; padding: {}; background: var(--surface-color, #fff); border-radius: 8px;",
        if outline { "1px solid #ccc" } else { "none" },
        if padding { "16px" } else { "0px" }
    );
    view! {
        <div style=style>
            {children()}
        </div>
    }
}
