use leptos::prelude::*;

#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(into)] error: Signal<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="form-field" style="display: flex; flex-direction: column; gap: 0.4rem; margin-bottom: 1rem;">
            <label style="font-size: 0.9rem; font-weight: 500; color: var(--text-main);">
                {label}
            </label>
            <div class="field-input-container">
                {children()}
            </div>
            <Show when=move || error.get().is_some()>
                <span class="form-error" style="color: var(--error-color, #ff4d4f); font-size: 0.75rem; margin-top: 0.2rem;">
                    {move || error.get().unwrap_or_default()}
                </span>
            </Show>
        </div>
    }
}
