use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: usize,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    pub toasts: RwSignal<Vec<Toast>>,
}

impl ToastContext {
    pub fn notify(&self, message: &str, toast_type: ToastType) {
        let id = js_sys::Math::random() as usize;
        let new_toast = Toast {
            id,
            message: message.to_string(),
            toast_type,
        };

        self.toasts.update(|toasts| toasts.push(new_toast));

        // Auto-remove after 3 seconds
        let toasts_clone = self.toasts;
        set_timeout(
            move || {
                toasts_clone.update(|toasts| {
                    toasts.retain(|t| t.id != id);
                });
            },
            Duration::from_secs(3),
        );
    }
}

#[component]
fn ToastList() -> impl IntoView {
    let toast_ctx = use_context::<ToastContext>().expect("ToastContext should be provided");

    view! {
        <div class="toast-container">
            <For
                each=move || toast_ctx.toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let id = toast.id;
                    let toast_ctx_inner = toast_ctx;
                    let toast_class = match toast.toast_type {
                        ToastType::Success => "toast-success",
                        ToastType::Error => "toast-error",
                        ToastType::Info => "toast-info",
                    };
                    view! {
                        <div class=format!("toast-item {}", toast_class)>
                            {toast.message}
                            <span
                                class="toast-close"
                                on:click=move |_| {
                                    toast_ctx_inner.toasts.update(|toasts| {
                                        toasts.retain(|t| t.id != id);
                                    });
                                }
                            >
                                "✕"
                            </span>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
fn NotifyButton(label: String, toast_type: ToastType) -> impl IntoView {
    let toast_ctx = use_context::<ToastContext>().expect("ToastContext should be provided");

    let btn_class = match toast_type {
        ToastType::Success => "toast-btn-success",
        ToastType::Error => "toast-btn-error",
        ToastType::Info => "toast-btn-info",
    };

    let msg = match toast_type {
        ToastType::Success => "Operation successful!",
        ToastType::Error => "Something went wrong!",
        ToastType::Info => "Here is some information.",
    };

    view! {
        <button
            class=format!("toast-btn {}", btn_class)
            on:click=move |_| toast_ctx.notify(msg, toast_type.clone())
        >
            {label}
        </button>
    }
}

#[component]
pub fn ToastDemo() -> impl IntoView {
    let toasts = RwSignal::new(Vec::<Toast>::new());
    let toast_ctx = ToastContext { toasts };

    provide_context(toast_ctx);

    view! {
        <div class="toast-demo">
            <h2>"Context Pattern: Toast Notifications"</h2>
            <div class="toast-demo-container">
                <p>"Click the buttons below to trigger notifications from different components via context."</p>
                <div class="toast-btn-group">
                    <NotifyButton label="Success".to_string() toast_type=ToastType::Success />
                    <NotifyButton label="Error".to_string() toast_type=ToastType::Error />
                    <NotifyButton label="Info".to_string() toast_type=ToastType::Info />
                </div>
                <div class="deep-component-demo">
                    <DeeplyNestedNotifier />
                </div>
            </div>
            <ToastList />
        </div>
    }
}

#[component]
fn DeeplyNestedNotifier() -> impl IntoView {
    view! {
        <div class="deep-notifier-container">
            <p>"I am a deeply nested component."</p>
            <NotifyButton label="Notify from Deep".to_string() toast_type=ToastType::Info />
        </div>
    }
}
