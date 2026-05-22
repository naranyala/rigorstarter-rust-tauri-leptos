use crate::ui::stdlib::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::stdlib::components::card::Card;
use crate::ui::stdlib::utils::event_target_value;
use leptos::prelude::*;

#[derive(Clone, Debug)]
struct FormState {
    username: String,
    email: String,
    password: String,
}

#[derive(Clone, Debug)]
struct FormErrors {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[component]
pub fn FormDemo() -> impl IntoView {
    let (form_state, set_form_state) = signal(FormState {
        username: String::new(),
        email: String::new(),
        password: String::new(),
    });
    let (errors, set_errors) = signal(FormErrors {
        username: None,
        email: None,
        password: None,
    });
    let (is_submitted, set_is_submitted) = signal(false);

    let validate = move |_| {
        let state = form_state.get();
        let mut new_errors = FormErrors {
            username: None,
            email: None,
            password: None,
        };
        let mut is_valid = true;

        // Username validation
        if state.username.trim().is_empty() {
            new_errors.username = Some("Username is required".to_string());
            is_valid = false;
        } else if state.username.len() < 3 {
            new_errors.username = Some("Username must be at least 3 characters".to_string());
            is_valid = false;
        }

        // Email validation
        if state.email.trim().is_empty() {
            new_errors.email = Some("Email is required".to_string());
            is_valid = false;
        } else if !state.email.contains('@') {
            new_errors.email = Some("Invalid email address".to_string());
            is_valid = false;
        }

        // Password validation
        if state.password.is_empty() {
            new_errors.password = Some("Password is required".to_string());
            is_valid = false;
        } else if state.password.len() < 8 {
            new_errors.password = Some("Password must be at least 8 characters".to_string());
            is_valid = false;
        }

        set_errors.set(new_errors);
        is_valid
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if validate(()) {
            set_is_submitted.set(true);
            leptos::logging::log!("Form submitted successfully: {:?}", form_state.get());
        } else {
            set_is_submitted.set(false);
            leptos::logging::log!("Form submission failed due to validation errors");
        }
    };

    view! {
        <div class="form-demo" style="padding: 2rem; max-width: 500px; margin: 0 auto;">
            <div style="margin-bottom: 2rem;">
                <h2>"Form Validation Demo"</h2>
                <p style="color: var(--text-muted);">"A demonstration of reactive form validation and error handling."</p>
            </div>

            <Card class="form-card".to_string()>
                <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 1.5rem;">
                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        <label style="font-size: 0.9rem; font-weight: 500;">"Username"</label>
                        <input
                            type="text"
                            prop:value=move || form_state.get().username
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_form_state.update(|s| s.username = val);
                            }
                            style=move || format!("padding: 0.6rem; border-radius: 4px; border: 1px solid {}; background: var(--bg-color); color: var(--text-main);",
                                if errors.get().username.is_some() { "var(--error-color, #ff4d4f)" } else { "var(--border-color)" })
                        />
                        <Show when=move || errors.get().username.is_some()>
                            <span style="color: var(--error-color, #ff4d4f); font-size: 0.75rem; margin-top: 0.2rem;">
                                {move || errors.get().username}
                            </span>
                        </Show>
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        <label style="font-size: 0.9rem; font-weight: 500;">"Email"</label>
                        <input
                            type="email"
                            prop:value=move || form_state.get().email
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_form_state.update(|s| s.email = val);
                            }
                            style=move || format!("padding: 0.6rem; border-radius: 4px; border: 1px solid {}; background: var(--bg-color); color: var(--text-main);",
                                if errors.get().email.is_some() { "var(--error-color, #ff4d4f)" } else { "var(--border-color)" })
                        />
                        <Show when=move || errors.get().email.is_some()>
                            <span style="color: var(--error-color, #ff4d4f); font-size: 0.75rem; margin-top: 0.2rem;">
                                {move || errors.get().email}
                            </span>
                        </Show>
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        <label style="font-size: 0.9rem; font-weight: 500;">"Password"</label>
                        <input
                            type="password"
                            prop:value=move || form_state.get().password
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_form_state.update(|s| s.password = val);
                            }
                            style=move || format!("padding: 0.6rem; border-radius: 4px; border: 1px solid {}; background: var(--bg-color); color: var(--text-main);",
                                if errors.get().password.is_some() { "var(--error-color, #ff4d4f)" } else { "var(--border-color)" })
                        />
                        <Show when=move || errors.get().password.is_some()>
                            <span style="color: var(--error-color, #ff4d4f); font-size: 0.75rem; margin-top: 0.2rem;">
                                {move || errors.get().password}
                            </span>
                        </Show>
                    </div>

                    <Button
                        variant=ButtonVariant::Primary
                        size=ButtonSize::Medium
                        disabled=false
                        on_click=Callback::new(move |_| {})
                        class="submit-btn".to_string()
                        button_type=Some("submit".to_string())
                    >
                        "Submit Form"
                    </Button>
                </form>
            </Card>

            <Show when=move || is_submitted.get()>
                <div style="margin-top: 1.5rem; padding: 1rem; background: var(--success-bg, #e6fffa); color: #2d3748; border-radius: 8px; border: 1px solid #b2f5ea; text-align: center;">
                    "✅ Form submitted successfully!"
                </div>
            </Show>
        </div>
    }
}
