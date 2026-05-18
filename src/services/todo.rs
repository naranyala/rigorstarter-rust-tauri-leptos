use crate::core::models::TodoItem;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[derive(Clone, Copy)]
pub struct TodoService {
    pub items: ReadSignal<Vec<TodoItem>>,
    pub set_items: WriteSignal<Vec<TodoItem>>,
}

impl TodoService {
    pub fn new() -> Self {
        let (items, set_items) = signal(Vec::<TodoItem>::new());
        Self { items, set_items }
    }

    pub fn load_todos(&self) {
        let set_items = self.set_items;
        spawn_local(async move {
            let result = invoke("list_todos", JsValue::NULL).await;
            if let Ok(val) = result {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                    set_items.set(items);
                }
            }
        });
    }

    pub fn add_todo(&self, title: String) {
        let set_items = self.set_items;
        spawn_local(async move {
            let args =
                serde_wasm_bindgen::to_value(&serde_json::json!({ "title": title })).unwrap();
            let _ = invoke("add_todo", args).await;

            // Refresh list
            let result = invoke("list_todos", JsValue::NULL).await;
            if let Ok(val) = result {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                    set_items.set(items);
                }
            }
        });
    }

    pub fn toggle_todo(&self, id: i64) {
        let set_items = self.set_items;
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "id": id })).unwrap();
            let _ = invoke("toggle_todo", args).await;

            let result = invoke("list_todos", JsValue::NULL).await;
            if let Ok(val) = result {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                    set_items.set(items);
                }
            }
        });
    }

    pub fn delete_todo(&self, id: i64) {
        let set_items = self.set_items;
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "id": id })).unwrap();
            let _ = invoke("delete_todo", args).await;

            let result = invoke("list_todos", JsValue::NULL).await;
            if let Ok(val) = result {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                    set_items.set(items);
                }
            }
        });
    }
}
