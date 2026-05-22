use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[prop(into)] variant: ButtonVariant,
    #[prop(into)] size: ButtonSize,
    #[prop(into)] disabled: bool,
    #[prop(into)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    #[prop(into)] class: String,
    #[prop(into)] button_type: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Ghost => "btn-ghost",
        ButtonVariant::Danger => "btn-danger",
    };

    let size_class = match size {
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "btn-md",
        ButtonSize::Large => "btn-lg",
    };

    let b_type = button_type.unwrap_or_else(|| "button".to_string());

    view! {
        <button
            type=b_type
            class=move || format!("btn {} {} {}", variant_class, size_class, class)
            disabled=disabled
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
