use crate::ui::stdlib::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::stdlib::components::card::Card;
use crate::ui::stdlib::hooks::use_storage::use_storage;
use crate::ui::stdlib::utils::{event_target_checked, event_target_value};
use leptos::prelude::*;

#[component]
pub fn PreferencesDemo() -> impl IntoView {
    // Persistence via use_storage
    let username = use_storage("user_name", "Guest".to_string());
    let notify_enabled = use_storage("notifications_enabled", true);
    let theme_pref = use_storage("theme_preference", "dark".to_string());

    view! {
        <div class="preferences-demo" style="padding: 2rem; max-width: 600px; margin: 0 auto;">
            <div style="margin-bottom: 2rem;">
                <h2>"User Preferences"</h2>
                <p style="color: var(--text-muted);">
                    "This demo showcases the use_storage hook. Changes are persisted to localStorage in real-time."
                </p>
            </div>

            <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                <Card class="pref-section".to_string()>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <h3 style="margin: 0; font-size: 1rem;">"Account"</h3>
                        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                            <label style="font-size: 0.8rem; color: var(--text-muted);">"Display Name"</label>
                            <input
                                type="text"
                                prop:value=move || username.get()
                                on:input=move |ev| username.set(event_target_value(&ev))
                                style="padding: 0.6rem; border-radius: 4px; border: 1px solid var(--border-color); background: var(--bg-color); color: var(--text-main);"
                            />
                        </div>
                    </div>
                </Card>

                <Card class="pref-section".to_string()>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <h3 style="margin: 0; font-size: 1rem;">"Notifications"</h3>
                        <div style="display: flex; align-items: center; gap: 0.5rem;">
                            <input
                                type="checkbox"
                                prop:checked=move || notify_enabled.get()
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    notify_enabled.set(checked);
                                }
                            />
                            <span style="font-size: 0.9rem;">"Enable Desktop Notifications"</span>
                        </div>
                    </div>
                </Card>

                <Card class="pref-section".to_string()>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <h3 style="margin: 0; font-size: 1rem;">"Appearance"</h3>
                        <div style="display: flex; gap: 0.5rem;">
                            <Button
                                variant=ButtonVariant::Primary
                                size=ButtonSize::Small
                                disabled=false
                                on_click=Callback::new(move |_| theme_pref.set("light".to_string()))
                                class="theme-btn".to_string()
                                button_type=None
                            >
                                "Light"
                            </Button>
                            <Button
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::Small
                                disabled=false
                                on_click=Callback::new(move |_| theme_pref.set("dark".to_string()))
                                class="theme-btn".to_string()
                                button_type=None
                            >
                                "Dark"
                            </Button>
                        </div>
                        <div style="font-size: 0.8rem; color: var(--text-muted);">
                            "Selected: " {move || theme_pref.get()}
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    }
}
