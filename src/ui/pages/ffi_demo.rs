use crate::ui::layout::PageLayout;
use leptos::prelude::*;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;

async fn invoke_cmd<T: serde::de::DeserializeOwned>(_cmd: &str, _args: serde_json::Value) -> T {
    // This is a placeholder. In a real Tauri app with a Rust frontend,
    // you'd use a JS bridge or a crate like tauri-sys.
    panic!("Tauri invoke not implemented for Rust frontend yet");
}

#[component]
pub fn FfiDemo() -> impl IntoView {
    let (num_a, set_num_a) = signal(10);
    let (num_b, set_num_b) = signal(20);
    let (add_result, set_add_result) = signal(String::new());

    let (name, set_name) = signal("Rustacean".to_string());
    let (greet_result, set_greet_result) = signal(String::new());

    let (text, set_text) = signal("hello ffi world".to_string());
    let (upper_result, set_upper_result) = signal(String::new());

    let (callback_val, set_callback_val) = signal(42);
    let (callback_result, set_callback_result) = signal(String::new());

    let run_add = move |_| {
        spawn_local(async move {
            let res: i32 =
                invoke_cmd("ffi_add", json!({ "a": num_a.get(), "b": num_b.get() })).await;
            set_add_result.set(res.to_string());
        });
    };

    let run_greet = move |_| {
        spawn_local(async move {
            let res: String = invoke_cmd("ffi_greet", json!({ "name": name.get() })).await;
            set_greet_result.set(res);
        });
    };

    let run_upper = move |_| {
        spawn_local(async move {
            let res: String = invoke_cmd("ffi_uppercase", json!({ "text": text.get() })).await;
            set_upper_result.set(res);
        });
    };

    let run_callback = move |_| {
        spawn_local(async move {
            let res: String =
                invoke_cmd("ffi_run_callback", json!({ "value": callback_val.get() })).await;
            set_callback_result.set(res);
        });
    };

    view! {
        <PageLayout>
            <h1>"C / Rust FFI Demo"</h1>
            <p style="color: var(--text-secondary); margin-bottom: 2rem;">"This page demonstrates calling C functions from Rust via Tauri."</p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px;">
                // Basic Math
                <div style="padding: 20px; border: 1px solid var(--border-color); border-radius: 12px; background: var(--surface-color);">
                    <h3>"1. Basic Math (Ints)"</h3>
                    <div style="display: flex; gap: 10px; margin-bottom: 10px;">
                        <input type="number" value=num_a on:input=move |ev| set_num_a.set(event_target_value(&ev).parse().unwrap_or(0)) />
                        <input type="number" value=num_b on:input=move |ev| set_num_b.set(event_target_value(&ev).parse().unwrap_or(0)) />
                    </div>
                    <button class="btn-primary" on:click=run_add>"Add in C"</button>
                    <p>"Result: " {move || add_result.get()}</p>
                </div>

                // String Handling
                <div style="padding: 20px; border: 1px solid var(--border-color); border-radius: 12px; background: var(--surface-color);">
                    <h3>"2. String Allocation"</h3>
                    <input type="text" value=name on:input=move |ev| set_name.set(event_target_value(&ev)) style="width: 100%; margin-bottom: 10px;" />
                    <button class="btn-primary" on:click=run_greet>"Greet from C"</button>
                    <p>"Result: " {move || greet_result.get()}</p>
                </div>

                // Buffer manipulation
                <div style="padding: 20px; border: 1px solid var(--border-color); border-radius: 12px; background: var(--surface-color);">
                    <h3>"3. Buffer Modification"</h3>
                    <input type="text" value=text on:input=move |ev| set_text.set(event_target_value(&ev)) style="width: 100%; margin-bottom: 10px;" />
                    <button class="btn-primary" on:click=run_upper>"Uppercase in C"</button>
                    <p>"Result: " {move || upper_result.get()}</p>
                </div>

                // Callbacks
                <div style="padding: 20px; border: 1px solid var(--border-color); border-radius: 12px; background: var(--surface-color);">
                    <h3>"4. C $\to$ Rust Callback"</h3>
                    <input type="number" value=callback_val on:input=move |ev| set_callback_val.set(event_target_value(&ev).parse().unwrap_or(0)) style="width: 100%; margin-bottom: 10px;" />
                    <button class="btn-primary" on:click=run_callback>"Run Callback"</button>
                    <p>"Status: " {move || callback_result.get()}</p>
                    <small style="color: var(--text-secondary);">"Check the terminal for the callback log!"</small>
                </div>
            </div>
        </PageLayout>
    }
}
