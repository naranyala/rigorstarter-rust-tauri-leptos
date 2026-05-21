use leptos::prelude::*;

#[component]
pub fn ModalDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="modal-demo">
            <h2>"Modal Demo"</h2>
            <div class="modal-trigger-container">
                <button class="open-modal-btn" on:click=move |_| set_is_open.set(true)>"Manage Profile"</button>
            </div>

            <Show when=move || is_open.get()>
                <div class="modal-overlay" on:click=move |_| set_is_open.set(false)>
                    <div class="modal-container" on:click=|ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <div class="modal-title-group">
                                <span class="modal-icon">"👤"</span>
                                <h3>"User Profile Settings"</h3>
                            </div>
                            <button class="close-modal-btn" on:click=move |_| set_is_open.set(false)>"✕"</button>
                        </div>
                        <div class="modal-body">
                            <div class="profile-section">
                                <div class="profile-info">
                                    <div class="avatar">"JD"</div>
                                    <div class="info-text">
                                        <span class="full-name">"Jane Doe"</span>
                                        <span class="email">"jane.doe@example.com"</span>
                                    </div>
                                </div>
                                <div class="profile-form">
                                    <div class="field">
                                        <label>"Username"</label>
                                        <input type="text" value="janedoe_dev" readonly />
                                    </div>
                                    <div class="field">
                                        <label>"Role"</label>
                                        <input type="text" value="Lead Architect" readonly />
                                    </div>
                                </div>
                            </div>
                            <div class="modal-warning">
                                <span class="warning-icon">"⚠️"</span>
                                <p>"Changes made here will be synced across all your devices instantly."</p>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button class="btn-secondary" on:click=move |_| set_is_open.set(false)>"Discard"</button>
                            <button class="btn-primary" on:click=move |_| {
                                set_is_open.set(false);
                            }>"Save Changes"</button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
