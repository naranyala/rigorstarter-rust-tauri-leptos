use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[component]
pub fn Stack(
    #[prop(into)] direction: MaybeInto<StackDirection>,
    #[prop(into)] gap: MaybeInto<String>,
    children: Children,
) -> impl IntoView {
    let direction = direction.into();
    let gap = gap.into();
    
    let flex_dir = match direction {
        StackDirection::Vertical => "column",
        StackDirection::Horizontal => "row",
    };

    view! {
        <div 
            style=move || format!("display: flex; flex-direction: {}; gap: {};")
            flex_dir
            gap
        >
            {children()}
        </div>
    }
}
