use crate::components::result_view::ResultView;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[component]
pub fn UtilityCodeBlock(name: String, id: String) -> impl IntoView {
    let (state, set_state) = signal(Result::<String, String>::Err("Initial state".to_string()));
    let (is_loading, set_is_loading) = signal(true);

    let id_for_spawn = id.clone();
    spawn_local(async move {
        set_is_loading.set(true);
        let args =
            serde_wasm_bindgen::to_value(&serde_json::json!({ "utility": id_for_spawn })).unwrap();
        let result = invoke("get_utility_source", args).await;

        match result {
            Ok(val) => {
                if let Some(content) = val.as_string() {
                    set_state.set(Ok(content));
                } else {
                    set_state.set(Err("Could not parse source code as string".to_string()));
                }
            }
            Err(e) => {
                set_state.set(Err(format!("Backend error: {:?}", e)));
            }
        }
        set_is_loading.set(false);
    });

    view! {
        <div class="utility-container">
            <h2>{name} " System Utility"</h2>
            {move || view! {
                <ResultView
                    loading=is_loading.get()
                    result=state.get()
                >
                    {move || match state.get() {
                        Ok(code) => view! {
                            <div class="code-block">
                                <pre class="language-rust"><code>{code}</code></pre>
                            </div>
                        }.into_any(),
                        Err(_) => view! { <div /> }.into_any(),
                    }}
                </ResultView>
            }.into_any()}
        </div>
    }
}
