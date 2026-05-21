use leptos::prelude::*;

#[component]
pub fn TabsDemo() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0);

    let tabs = vec![("General", "⚙️"), ("Advanced", "🚀"), ("Security", "🛡️")];

    let contents = move || match active_tab.get() {
        0 => view! {
            <div class="tab-panel">
                <h3>"General Settings"</h3>
                <p>"Configure the basic behavior of your application."</p>
                <div class="tab-form-group">
                    <label>"Application Name"</label>
                    <input type="text" value="Component Library" readonly />
                </div>
                <div class="tab-form-group">
                    <label>"Theme Mode"</label>
                    <select readonly>
                        <option selected>"Light"</option>
                        <option>"Dark"</option>
                        <option>"System"</option>
                    </select>
                </div>
            </div>
        }
        .into_any(),
        1 => view! {
            <div class="tab-panel">
                <h3>"Performance Tuning"</h3>
                <p>"Optimize the internal engine for high-load scenarios."</p>
                <div class="tab-form-group">
                    <label>"Cache TTL (seconds)"</label>
                    <input type="number" value="3600" readonly />
                </div>
                <div class="tab-form-group">
                    <label>"Max Concurrent Workers"</label>
                    <input type="number" value="4" readonly />
                </div>
            </div>
        }
        .into_any(),
        _ => view! {
            <div class="tab-panel">
                <h3>"Security & Access"</h3>
                <p>"Manage authentication and permission tokens."</p>
                <div class="tab-form-group">
                    <label>"API Key"</label>
                    <input type="password" value="••••••••••••" readonly />
                </div>
                <div class="tab-form-group">
                    <label>"Two-Factor Authentication"</label>
                    <div class="switch-container">
                        <span class="switch-label">"Enabled"</span>
                        <div class="switch"></div>
                    </div>
                </div>
            </div>
        }
        .into_any(),
    };

    view! {
        <div class="tabs-demo">
            <h2>"Tabs Demo"</h2>
            <div class="tabs-container">
                <div class="tabs-header">
                    {tabs.into_iter().enumerate().map(|(idx, (name, icon))| {
                        view! {
                            <button
                                class=move || if active_tab.get() == idx { "tab-btn active" } else { "tab-btn" }
                                on:click=move |_| set_active_tab.set(idx)
                            >
                                <span class="tab-icon">{icon}</span>
                                {name}
                            </button>
                        }
                    }).collect_view()}
                </div>
                <div class="tabs-content">
                    {contents}
                </div>
            </div>
        </div>
    }
}
