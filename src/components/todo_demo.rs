use crate::core::models::TodoItem;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[component]
pub fn TodoDemo() -> impl IntoView {
    let (todos, set_todos) = signal(Vec::<TodoItem>::new());
    let (new_title, set_new_title) = signal(String::new());

    spawn_local(async move {
        let result = invoke("list_todos", JsValue::NULL).await;
        if let Ok(val) = result {
            if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                set_todos.set(items);
            }
        }
    });

    let add_todo = move || {
        let title = new_title.get();
        if title.trim().is_empty() {
            return;
        }
        spawn_local(async move {
            let args =
                serde_wasm_bindgen::to_value(&serde_json::json!({ "title": title })).unwrap();
            let _ = invoke("add_todo", args).await;
            let result = invoke("list_todos", JsValue::NULL).await;
            if let Ok(val) = result {
                if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                    set_todos.set(items);
                }
            }
            set_new_title.set(String::new());
        });
    };

    view! {
        <div class="todo-demo">
            <h2>"Todo List"</h2>
            <div class="todo-input-row">
                <input
                    type="text"
                    placeholder="Add a new todo..."
                    value=new_title
                    on:input=move |ev| set_new_title.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" { add_todo(); }
                    }
                />
                <button on:click=move |_| add_todo()>"Add"</button>
            </div>
            <div class="todo-list">
                {move || {
                    let items = todos.get();
                    if items.is_empty() {
                        view! { <p class="todo-empty">"No todos yet. Add one above!"</p> }.into_any()
                    } else {
                        items.into_iter().map(|todo| {
                            let id = todo.id;
                            let toggle_id = id;
                            let delete_id = id;
                            view! {
                                <div class="todo-item" class:completed=todo.completed>
                                    <input
                                        type="checkbox"
                                        checked=todo.completed
                                        on:click=move |_| {
                                            spawn_local(async move {
                                                let args = serde_wasm_bindgen::to_value(
                                                    &serde_json::json!({ "id": toggle_id })
                                                ).unwrap();
                                                let _ = invoke("toggle_todo", args).await;
                                                let result = invoke("list_todos", JsValue::NULL).await;
                                                if let Ok(val) = result {
                                                    if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                                                        set_todos.set(items);
                                                    }
                                                }
                                            });
                                        }
                                    />
                                    <span class="todo-title">{todo.title}</span>
                                    <span class="todo-date">{todo.created_at}</span>
                                    <button
                                        class="todo-delete"
                                        on:click=move |_| {
                                            spawn_local(async move {
                                                let args = serde_wasm_bindgen::to_value(
                                                    &serde_json::json!({ "id": delete_id })
                                                ).unwrap();
                                                let _ = invoke("delete_todo", args).await;
                                                let result = invoke("list_todos", JsValue::NULL).await;
                                                if let Ok(val) = result {
                                                    if let Ok(items) = serde_wasm_bindgen::from_value::<Vec<TodoItem>>(val) {
                                                        set_todos.set(items);
                                                    }
                                                }
                                            });
                                        }
                                    >"✕"</button>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}
