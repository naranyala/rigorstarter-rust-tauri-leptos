use crate::core::models::TodoItem;
use leptos::prelude::*;

#[component]
pub fn TodoDemo(
    items: ReadSignal<Vec<TodoItem>>,
    on_add: Callback<String>,
    on_toggle: Callback<i64>,
    on_delete: Callback<i64>,
) -> impl IntoView {
    let (new_title, set_new_title) = signal(String::new());

    let handle_add = move |_| {
        let title = new_title.get();
        if title.trim().is_empty() {
            return;
        }
        on_add.run(title);
        set_new_title.set(String::new());
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
                        if ev.key() == "Enter" { handle_add(()); }
                    }
                />
                <button on:click=move |_| handle_add(())>"Add"</button>
            </div>
            <div class="todo-list">
                {move || {
                    let items_list = items.get();
                    if items_list.is_empty() {
                        view! { <p class="todo-empty">"No todos yet. Add one above!"</p> }.into_any()
                    } else {
                        items_list.into_iter().map(|todo| {
                            let id = todo.id;
                            let toggle_id = id;
                            let delete_id = id;
                            view! {
                                <div class="todo-item" class:completed=todo.completed>
                                    <input
                                        type="checkbox"
                                        checked=todo.completed
                                        on:click=move |_| {
                                            on_toggle.run(toggle_id);
                                        }
                                    />
                                    <span class="todo-title">{todo.title}</span>
                                    <span class="todo-date">{todo.created_at}</span>
                                    <button
                                        class="todo-delete"
                                        on:click=move |_| {
                                            on_delete.run(delete_id);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_todo_add_logic() {
        let _rt = setup_runtime();
        let (new_title, set_new_title) = signal(String::new());
        let added_titles = RwSignal::new(Vec::<String>::new());

        let on_add = Callback::new(move |title: String| {
            added_titles.update(|list| list.push(title));
        });

        let handle_add = move || {
            let title = new_title.get();
            if title.trim().is_empty() {
                return;
            }
            on_add.run(title);
            set_new_title.set(String::new());
        };

        // Test valid add
        set_new_title.set("Test Todo".to_string());
        handle_add();
        assert_eq!(added_titles.get().len(), 1);
        assert_eq!(added_titles.get()[0], "Test Todo");
        assert!(new_title.get().is_empty());

        // Test empty add
        set_new_title.set("  ".to_string());
        handle_add();
        assert_eq!(added_titles.get().len(), 1);
    }
}
