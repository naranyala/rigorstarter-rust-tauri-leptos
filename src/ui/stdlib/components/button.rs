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
    #[prop(into)] variant: MaybeInto<ButtonVariant>,
    #[prop(into)] size: MaybeInto<ButtonSize>,
    #[prop(into)] disabled: MaybeInto<bool>,
    #[prop(into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into)] class: MaybeInto<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.into();
    let size = size.into();
    let disabled = disabled.into();
    let class = class.into();

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

    view! {
        <button
            class=move || format!("btn {} {} {}", variant_class, size_class, class)
            disabled=disabled
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.call(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
