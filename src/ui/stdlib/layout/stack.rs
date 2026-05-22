use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[component]
pub fn Stack(
    #[prop(into)] direction: Signal<StackDirection>,
    #[prop(into)] gap: Signal<String>,
    children: Children,
) -> impl IntoView {
    let flex_dir = move || match direction.get() {
        StackDirection::Vertical => "column",
        StackDirection::Horizontal => "row",
    };

    let gap_val = move || gap.get();

    view! {
        <div
            style=move || format!("display: flex; flex-direction: {}; gap: {};", flex_dir(), gap_val())
        >
            {children()}
        </div>
    }
}
