use crate::ui::stdlib::utils::{event_target_checked, event_target_value};
use leptos::prelude::*;

#[component]
pub fn TextInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] placeholder: String,
    #[prop(into)] input_type: String,
) -> impl IntoView {
    view! {
        <input
            type=input_type
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| on_change.run(event_target_value(&ev))
            class="stdlib-input"
            style="padding: 0.6rem; border-radius: 4px; border: 1px solid var(--border-color); background: var(--bg-color); color: var(--text-main); width: 100%; box-sizing: border-box;"
        />
    }
}

#[component]
pub fn NumberInput(
    #[prop(into)] value: Signal<f64>,
    #[prop(into)] on_change: Callback<f64>,
    #[prop(into)] placeholder: String,
) -> impl IntoView {
    view! {
        <input
            type="number"
            placeholder=placeholder
            prop:value=move || value.get().to_string()
            on:input=move |ev| {
                let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                on_change.run(val);
            }
            class="stdlib-input"
            style="padding: 0.6rem; border-radius: 4px; border: 1px solid var(--border-color); background: var(--bg-color); color: var(--text-main); width: 100%; box-sizing: border-box;"
        />
    }
}

#[component]
pub fn SelectInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Vec<(String, String)>, // (Value, Label)
) -> impl IntoView {
    view! {
        <select
            prop:value=move || value.get()
            on:change=move |ev| {
                let val = event_target_value(&ev);
                on_change.run(val);
            }
            class="stdlib-input"
            style="padding: 0.6rem; border-radius: 4px; border: 1px solid var(--border-color); background: var(--bg-color); color: var(--text-main); width: 100%; box-sizing: border-box;"
        >
            {options.into_iter().map(|(val, label)| {
                view! { <option value=val>{label}</option> }
            }).collect_view()}
        </select>
    }
}

#[component]
pub fn CheckboxInput(
    #[prop(into)] value: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            prop:checked=move || value.get()
            on:change=move |ev| {
                on_change.run(event_target_checked(&ev));
            }
            style="width: 1.2rem; height: 1.2rem; cursor: pointer;"
        />
    }
}
