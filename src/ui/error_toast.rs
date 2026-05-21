use crate::services::{ErrorService, ErrorSeverity};
use leptos::prelude::*;

#[component]
pub fn ErrorToast() -> impl IntoView {
    let error_service = use_context::<ErrorService>().expect("ErrorService missing");
    let errors = error_service.errors;

    view! {
        <div style="position:fixed; bottom:20px; right:20px; z-index:9999; display:flex; flex-direction:column; gap:10px; width:300px;">
            {move || {
                let err_list = errors.get();
                let items: Vec<_> = err_list.into_iter().map(|err| {
                    let id = err.id;
                    let msg = err.message.clone();
                    let severity = err.severity.clone();
                    let es = error_service.clone();
                    let severity_color = match severity {
                        ErrorSeverity::Info => "var(--accent-color)",
                        ErrorSeverity::Warning => "#ffcc00",
                        ErrorSeverity::Error => "#ff4444",
                        ErrorSeverity::Critical => "#aa0000",
                    };
                    view! {
                        <div style=format!("background:var(--surface-color); border-left: 5px solid {}; padding:12px; border-radius:4px; box-shadow:0 4px 12px rgba(0,0,0,0.3); color:var(--text-primary); animation: slideIn 0.3s ease-out;", severity_color)>
                            <div style="display:flex; justify-content:space-between; align-items:start; gap:10px;">
                                <span style="font-size:0.85rem;">{msg}</span>
                                <button
                                    on:click=move |_| es.remove(id)
                                    style="background:none; border:none; color:var(--text-secondary); cursor:pointer; font-size:1rem; line-height:1;"
                                >
                                    "✕"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                }).collect();
                items.into_iter().collect_view()
            }}
        </div>
    }
}
