use crate::app::ActivePage;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct SystemStatus {
    pub system: serde_json::Value,
    pub resources: serde_json::Value,
    pub session: String,
    pub network: serde_json::Value,
    pub storage: serde_json::Value,
}

#[component]
pub fn WelcomeDashboard() -> impl IntoView {
    let active_page = use_context::<ActivePage>().expect("ActivePage not provided");

    let (system_status, set_system_status) = signal::<Option<SystemStatus>>(None);
    let (is_loading, set_is_loading) = signal(true);

    Effect::new(move |_| {
        spawn_local(async move {
            set_is_loading.set(true);
            let promise = invoke("get_system_status", JsValue::NULL);
            let result = JsFuture::from(promise).await;
            match result {
                Ok(val) => match serde_wasm_bindgen::from_value::<SystemStatus>(val.clone()) {
                    Ok(status) => set_system_status.set(Some(status)),
                    Err(e) => {
                        leptos::logging::error!(
                            "Failed to deserialize system status: {}. Value: {:?}",
                            e,
                            val
                        );
                        set_system_status.set(None);
                    }
                },
                Err(e) => {
                    leptos::logging::error!("Tauri invoke error: {:?}", e);
                    set_system_status.set(None);
                }
            }
            set_is_loading.set(false);
        });
    });

    let rows = Memo::new(move |_| {
        system_status.get().map(|status| {
            let mut r = Vec::new();
            if let Some(sys) = status.system.as_object() {
                for (k, v) in sys {
                    let val = v
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| v.to_string());
                    r.push((k.clone(), val));
                }
            }
            if let Some(res) = status.resources.as_object() {
                for (k, v) in res {
                    let val = v
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| v.to_string());
                    r.push((k.clone(), val));
                }
            }
            r.push(("Session".to_string(), status.session));
            r.push(("Network".to_string(), status.network.to_string()));
            r.push(("Storage".to_string(), status.storage.to_string()));
            r
        })
    });

    view! {
        <div style:display=move || if active_page.get().is_none() || active_page.get() == Some("welcome") { "block" } else { "none" }>
            <div class="welcome-container">
                <div class="welcome-header">
                    <h1 class="welcome-title">"System Information"</h1>
                </div>

                <div class="welcome-content">
                    {move || if is_loading.get() {
                        view! { <div class="loading-container"><p>"Loading system information..."</p></div> }.into_any()
                    } else {
                        match rows.get() {
                            Some(rows_list) => view! {
                                <div class="table-container">
                                    <table class="data-table">
                                        <thead>
                                            <tr>
                                                <th>"Property"</th>
                                                <th>"Value"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <For
                                                each=move || rows_list.clone()
                                                key=|row| row.0.clone()
                                                children=move |(prop, val)| {
                                                    view! {
                                                        <tr>
                                                            <td>{prop}</td>
                                                            <td>{val}</td>
                                                        </tr>
                                                    }
                                                }
                                            />
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any(),
                            None => view! {
                                <div class="table-container">
                                    <p>"Failed to load system information."</p>
                                </div>
                            }.into_any(),
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
