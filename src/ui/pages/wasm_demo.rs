use leptos::prelude::*;

#[component]
pub fn WasmDemo() -> impl IntoView {
    let (result, set_result) = signal(String::from("Not run yet."));

    let run_wasm = move |_| {
        // Simulate WASM heavy task
        set_result.set(String::from("WASM performing task..."));
        // In a real app, you'd call a function exported from a separate WASM module.
        let res = (1..1000).sum::<i32>();
        set_result.set(format!("WASM Result (sum 1..1000): {}", res));
    };

    view! {
        <div class="wasm-demo-container">
            <h2>"WebAssembly Demo"</h2>
            <p>"This demo simulates high-performance computation that would typically be handled by a WASM module."</p>
            <button class="btn-primary" on:click=run_wasm>
                "Run WASM Task"
            </button>
            <p class="result-text">{move || result.get()}</p>
        </div>
    }
}
