use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn WebWorkerDemo() -> impl IntoView {
    let (result, set_result) = signal(String::from("No calculation yet."));

    let run_calculation = move |_| {
        set_result.set(String::from("Calculating..."));
        spawn_local(async move {
            // Simulate heavy work in a "worker" (actually just an async block here for demo)
            // In a real app, you'd use a dedicated Web Worker.
            let mut sum = 0;
            for i in 0..10_000_000 {
                sum += i;
            }
            set_result.set(format!("Calculation result: {}", sum));
        });
    };

    view! {
        <div class="web-worker-demo-container">
            <h2>"Web Worker Demo"</h2>
            <p>"This demo simulates heavy computation using an async block to avoid blocking the main thread."</p>
            <button class="btn-primary" on:click=run_calculation>
                "Run Heavy Calculation"
            </button>
            <p class="result-text">{move || result.get()}</p>
        </div>
    }
}
