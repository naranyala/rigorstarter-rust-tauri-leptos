use crate::core::models::JsonTodo;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[component]
pub fn JsonTodoDemo() -> impl IntoView {
    let (todos, set_todos) = signal(Vec::<JsonTodo>::new());
    let (new_title, set_new_title) = signal(String::new());
    let next_id = RwSignal::new(1u64);

    spawn_local(async move {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(Some(json)) = storage.get_item("json_todos") {
                if let Ok(items) = serde_json::from_str::<Vec<JsonTodo>>(&json) {
                    set_todos.set(items.clone());
                    let max_id = items.iter().map(|t| t.id).max().unwrap_or(0);
                    next_id.set(max_id + 1);
                }
            }
        }
    });

    let save = move |items: Vec<JsonTodo>| {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(json) = serde_json::to_string(&items) {
                let _ = storage.set_item("json_todos", &json);
            }
        }
    };

    let add_todo = move || {
        let title = new_title.get();
        let trimmed = title.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        let id = next_id.get();
        next_id.set(id + 1);
        let mut items = todos.get();
        items.push(JsonTodo {
            id,
            title: trimmed,
            completed: false,
        });
        save(items.clone());
        set_todos.set(items);
        set_new_title.set(String::new());
    };

    let toggle_todo = move |id: u64| {
        let mut items = todos.get();
        if let Some(todo) = items.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        save(items.clone());
        set_todos.set(items);
    };

    let delete_todo = move |id: u64| {
        let mut items = todos.get();
        items.retain(|t| t.id != id);
        save(items.clone());
        set_todos.set(items);
    };

    let json_preview = move || {
        let items = todos.get();
        serde_json::to_string_pretty(&items).unwrap_or_default()
    };

    view! {
        <div class="json-todo-demo">
            <h2>"JSON Todo List"</h2>
            <p class="json-todo-subtitle">"Stored in localStorage as JSON — no backend needed"</p>

            <div class="json-todo-input-row">
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

            <div class="json-todo-list">
                {move || {
                    let items = todos.get();
                    if items.is_empty() {
                        view! { <p class="json-todo-empty">"No todos yet. Add one above!"</p> }.into_any()
                    } else {
                        items.into_iter().map(|todo| {
                            let tid = todo.id;
                            view! {
                                <div class="json-todo-item" class:completed=todo.completed>
                                    <input
                                        type="checkbox"
                                        checked=todo.completed
                                        on:click=move |_| toggle_todo(tid)
                                    />
                                    <span class="json-todo-title">{todo.title}</span>
                                    <span class="json-todo-id">"#"({todo.id})</span>
                                    <button
                                        class="json-todo-delete"
                                        on:click=move |_| delete_todo(tid)
                                    >"✕"</button>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>

            <div class="json-todo-preview">
                <h3>"JSON Preview"</h3>
                <pre>{json_preview}</pre>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_todo_serde_storage_format() {
        let items = vec![
            JsonTodo {
                id: 1,
                title: "A".into(),
                completed: false,
            },
            JsonTodo {
                id: 2,
                title: "B".into(),
                completed: true,
            },
        ];
        let json = serde_json::to_string(&items).unwrap();
        let deserialized: Vec<JsonTodo> = serde_json::from_str(&json).unwrap();
        assert_eq!(items, deserialized);
    }

    #[test]
    fn test_json_todo_empty_list_serialization() {
        let items: Vec<JsonTodo> = vec![];
        let json = serde_json::to_string(&items).unwrap();
        assert_eq!(json, "[]");
        let deserialized: Vec<JsonTodo> = serde_json::from_str(&json).unwrap();
        assert!(deserialized.is_empty());
    }

    #[test]
    fn test_json_preview_formatting() {
        let items = vec![JsonTodo {
            id: 1,
            title: "Test".into(),
            completed: false,
        }];
        let pretty = serde_json::to_string_pretty(&items).unwrap();
        assert!(pretty.contains("\"id\": 1"));
        assert!(pretty.contains("\"title\": \"Test\""));
        assert!(pretty.contains("\"completed\": false"));
    }
}
