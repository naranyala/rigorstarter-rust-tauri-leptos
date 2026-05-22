use crate::ui::stdlib::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::stdlib::components::card::Card;
use crate::ui::stdlib::forms::*;
use leptos::prelude::*;

#[derive(Clone, Debug, Default)]
struct UserProfile {
    name: String,
    email: String,
    age: f64,
    role: String,
    agree_terms: bool,
}

#[derive(Clone, Debug, Default)]
struct UserProfileErrors {
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

#[component]
pub fn AdvancedFormDemo() -> impl IntoView {
    let (profile, set_profile) = signal(UserProfile {
        name: String::new(),
        email: String::new(),
        age: 18.0,
        role: "developer".to_string(),
        agree_terms: false,
    });

    let (errors, set_errors) = signal(UserProfileErrors::default());
    let (is_submitted, set_is_submitted) = signal(false);

    let validate = move || {
        let p = profile.get();
        let mut e = UserProfileErrors::default();
        let mut valid = true;

        if let Err(msg) = Validators::required(&p.name) {
            e.name = Some(msg);
            valid = false;
        } else if let Err(msg) = Validators::min_length(&p.name, 3) {
            e.name = Some(msg);
            valid = false;
        }

        if let Err(msg) = Validators::required(&p.email) {
            e.email = Some(msg);
            valid = false;
        } else if let Err(msg) = Validators::email(&p.email) {
            e.email = Some(msg);
            valid = false;
        }

        if let Err(msg) = Validators::range(p.age, 13.0, 120.0) {
            e.age = Some(msg);
            valid = false;
        }

        set_errors.set(e);
        valid
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if validate() {
            set_is_submitted.set(true);
            leptos::logging::log!("Profile submitted: {:?}", profile.get());
        } else {
            set_is_submitted.set(false);
        }
    };

    // Memos for values
    let name_val = Memo::new(move |_| profile.get().name);
    let email_val = Memo::new(move |_| profile.get().email);
    let age_val = Memo::new(move |_| profile.get().age);
    let role_val = Memo::new(move |_| profile.get().role);
    let agree_val = Memo::new(move |_| profile.get().agree_terms);

    // Memos for errors
    let name_err = Memo::new(move |_| errors.get().name);
    let email_err = Memo::new(move |_| errors.get().email);
    let age_err = Memo::new(move |_| errors.get().age);
    let role_err = Memo::new(move |_| None::<String>);

    let roles = vec![
        ("developer".to_string(), "Developer".to_string()),
        ("designer".to_string(), "Designer".to_string()),
        ("manager".to_string(), "Manager".to_string()),
        ("other".to_string(), "Other".to_string()),
    ];

    view! {
        <div class="advanced-form-demo" style="padding: 2rem; max-width: 600px; margin: 0 auto;">
            <div style="margin-bottom: 2rem;">
                <h2>"Advanced Form Stdlib Demo"</h2>
                <p style="color: var(--text-muted);">"A comprehensive demonstration of the Form stdlib including primitives, composites, and centralized validation."</p>
            </div>

            <Card class="form-container".to_string()>
                <form on:submit=on_submit style="display: flex; flex-direction: column; gap: 1rem;">

                    <FormField label="Full Name".to_string() error=name_err>
                        <TextInput
                            value=name_val
                            on_change=Callback::new(move |v| set_profile.update(|p| p.name = v))
                            placeholder="Enter your full name"
                            input_type="text"
                        />
                    </FormField>

                    <FormField label="Email Address".to_string() error=email_err>
                        <TextInput
                            value=email_val
                            on_change=Callback::new(move |v| set_profile.update(|p| p.email = v))
                            placeholder="you@example.com"
                            input_type="email"
                        />
                    </FormField>

                    <FormField label="Age".to_string() error=age_err>
                        <NumberInput
                            value=age_val
                            on_change=Callback::new(move |v| set_profile.update(|p| p.age = v))
                            placeholder="18"
                        />
                    </FormField>

                    <FormField label="Primary Role".to_string() error=role_err>
                        <SelectInput
                            value=role_val
                            on_change=Callback::new(move |v| set_profile.update(|p| p.role = v))
                            options=roles
                        />
                    </FormField>

                    <div style="display: flex; align-items: center; gap: 0.75rem; margin: 1rem 0;">
                        <CheckboxInput
                            value=agree_val
                            on_change=Callback::new(move |v| set_profile.update(|p| p.agree_terms = v))
                        />
                        <span style="font-size: 0.9rem;">"I agree to the terms and conditions"</span>
                    </div>

                    <Button
                        variant=ButtonVariant::Primary
                        size=ButtonSize::Medium
                        disabled=false
                        on_click=Callback::new(move |_| {})
                        class="submit-btn".to_string()
                        button_type=Some("submit".to_string())
                    >
                        "Complete Registration"
                    </Button>
                </form>
            </Card>

            <Show when=move || is_submitted.get()>
                <div style="margin-top: 1.5rem; padding: 1rem; background: var(--success-bg, #e6fffa); color: #2d3748; border-radius: 8px; border: 1px solid #b2f5ea; text-align: center;">
                    "✅ Profile successfully validated and submitted!"
                </div>
            </Show>
        </div>
    }
}
