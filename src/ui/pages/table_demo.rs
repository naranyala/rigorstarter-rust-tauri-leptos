use crate::core::models::User;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum SortDirection {
    Asc,
    Desc,
}

#[component]
pub fn TableDemo() -> impl IntoView {
    let initial_data = vec![
        User {
            id: 1,
            name: "Alice Johnson".into(),
            email: "alice@example.com".into(),
            role: "Admin".into(),
        },
        User {
            id: 2,
            name: "Bob Smith".into(),
            email: "bob@example.com".into(),
            role: "User".into(),
        },
        User {
            id: 3,
            name: "Charlie Brown".into(),
            email: "charlie@example.com".into(),
            role: "Editor".into(),
        },
        User {
            id: 4,
            name: "Diana Prince".into(),
            email: "diana@example.com".into(),
            role: "User".into(),
        },
        User {
            id: 5,
            name: "Edward Norton".into(),
            email: "edward@example.com".into(),
            role: "Admin".into(),
        },
        User {
            id: 6,
            name: "Fiona Gallagher".into(),
            email: "fiona@example.com".into(),
            role: "Editor".into(),
        },
    ];

    let (data, _set_data) = signal(initial_data);
    let (sort_col, set_sort_col) = signal(None::<(String, SortDirection)>);

    let sorted_data = Memo::new(move |_| {
        let mut items = data.get();
        if let Some((col, direction)) = sort_col.get() {
            items.sort_by(|a, b| {
                let cmp = match col.as_str() {
                    "id" => a.id.cmp(&b.id),
                    "name" => a.name.cmp(&b.name),
                    "email" => a.email.cmp(&b.email),
                    "role" => a.role.cmp(&b.role),
                    _ => std::cmp::Ordering::Equal,
                };
                match direction {
                    SortDirection::Asc => cmp,
                    SortDirection::Desc => cmp.reverse(),
                }
            });
        }
        items
    });

    let toggle_sort = move |col: &str| {
        set_sort_col.update(|current| {
            if let Some((c, dir)) = current.as_ref() {
                if c == col {
                    *current = match dir {
                        SortDirection::Asc => Some((col.to_string(), SortDirection::Desc)),
                        SortDirection::Desc => None,
                    };
                } else {
                    *current = Some((col.to_string(), SortDirection::Asc));
                }
            } else {
                *current = Some((col.to_string(), SortDirection::Asc));
            }
        });
    };

    view! {
        <div class="table-demo">
            <h2>"Sortable Data Table"</h2>
            <p class="table-demo-subtitle">"Click headers to sort data by column"</p>

            <div class="table-container">
                <table class="data-table">
                    <thead>
                        <tr>
                            <th on:click=move |_| toggle_sort("id") class=move || if sort_col.get().is_some_and(|(c, d)| c == "id" && matches!(d, SortDirection::Asc)) || sort_col.get().is_some_and(|(c, d)| c == "id" && matches!(d, SortDirection::Desc)) { "sortable" } else { "" }>
                                "ID"
                                <span class="sort-icon">{move || if sort_col.get().is_some_and(|(c, d)| c == "id" && matches!(d, SortDirection::Asc)) { " ▴" } else if sort_col.get().is_some_and(|(c, d)| c == "id" && matches!(d, SortDirection::Desc)) { " ▾" } else { "" }}</span>
                            </th>
                            <th on:click=move |_| toggle_sort("name") class=move || if sort_col.get().is_some_and(|(c, d)| c == "name" && matches!(d, SortDirection::Asc)) || sort_col.get().is_some_and(|(c, d)| c == "name" && matches!(d, SortDirection::Desc)) { "sortable" } else { "" }>
                                "Name"
                                <span class="sort-icon">{move || if sort_col.get().is_some_and(|(c, d)| c == "name" && matches!(d, SortDirection::Asc)) { " ▴" } else if sort_col.get().is_some_and(|(c, d)| c == "name" && matches!(d, SortDirection::Desc)) { " ▾" } else { "" }}</span>
                            </th>
                            <th on:click=move |_| toggle_sort("email") class=move || if sort_col.get().is_some_and(|(c, d)| c == "email" && matches!(d, SortDirection::Asc)) || sort_col.get().is_some_and(|(c, d)| c == "email" && matches!(d, SortDirection::Desc)) { "sortable" } else { "" }>
                                "Email"
                                <span class="sort-icon">{move || if sort_col.get().is_some_and(|(c, d)| c == "email" && matches!(d, SortDirection::Asc)) { " ▴" } else if sort_col.get().is_some_and(|(c, d)| c == "email" && matches!(d, SortDirection::Desc)) { " ▾" } else { "" }}</span>
                            </th>
                            <th on:click=move |_| toggle_sort("role") class=move || if sort_col.get().is_some_and(|(c, d)| c == "role" && matches!(d, SortDirection::Asc)) || sort_col.get().is_some_and(|(c, d)| c == "role" && matches!(d, SortDirection::Desc)) { "sortable" } else { "" }>
                                "Role"
                                <span class="sort-icon">{move || if sort_col.get().is_some_and(|(c, d)| c == "role" && matches!(d, SortDirection::Asc)) { " ▴" } else if sort_col.get().is_some_and(|(c, d)| c == "role" && matches!(d, SortDirection::Desc)) { " ▾" } else { "" }}</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || sorted_data.get()
                            key=|user| user.id
                            children=move |user| {
                                view! {
                                    <tr>
                                        <td>{user.id}</td>
                                        <td>{user.name}</td>
                                        <td>{user.email}</td>
                                        <td><span class="role-badge">{user.role}</span></td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
        </div>
    }
}
