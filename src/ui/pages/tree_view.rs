use crate::core::models::TreeNode;
use leptos::prelude::*;

#[component]
fn TreeNodeView(node: TreeNode) -> AnyView {
    let (is_expanded, set_is_expanded) = signal(true);
    let label = node.label;
    let has_children = !node.children.is_empty();
    let (children, _) = signal(node.children);

    view! {
        <div class="tree-node">
            <div
                class="tree-node-header"
                on:click=move |_| {
                    if has_children {
                        set_is_expanded.update(|v| *v = !*v);
                    }
                }
            >
                <span class="tree-node-icon">
                    {move || {
                        if !has_children {
                            "".to_string()
                        } else if is_expanded.get() {
                            "▼".to_string()
                        } else {
                            "▶".to_string()
                        }
                    }}
                </span>
                <span class="tree-node-label">{label}</span>
            </div>
            <Show
                when=move || is_expanded.get()
                fallback=|| view! { <div class="tree-node-empty-space" /> }.into_any()
            >
                <div class="tree-node-children">
                    {move || children.get().into_iter().map(|child| view! { <TreeNodeView node=child /> }).collect_view()}
                </div>
            </Show>
        </div>
    }.into_any()
}

#[component]
pub fn TreeViewDemo() -> impl IntoView {
    let demo_data = TreeNode {
        id: "root".into(),
        label: "Root".into(),
        children: vec![
            TreeNode {
                id: "c1".into(),
                label: "Documents".into(),
                children: vec![
                    TreeNode {
                        id: "c1-1".into(),
                        label: "Work".into(),
                        children: vec![
                            TreeNode {
                                id: "c1-1-1".into(),
                                label: "Project_A.pdf".into(),
                                children: vec![],
                            },
                            TreeNode {
                                id: "c1-1-2".into(),
                                label: "Budget.xlsx".into(),
                                children: vec![],
                            },
                        ],
                    },
                    TreeNode {
                        id: "c1-2".into(),
                        label: "Personal".into(),
                        children: vec![],
                    },
                ],
            },
            TreeNode {
                id: "c2".into(),
                label: "Images".into(),
                children: vec![
                    TreeNode {
                        id: "c2-1".into(),
                        label: "Vacation".into(),
                        children: vec![],
                    },
                    TreeNode {
                        id: "c2-2".into(),
                        label: "Profile.png".into(),
                        children: vec![],
                    },
                ],
            },
            TreeNode {
                id: "c3".into(),
                label: "System".into(),
                children: vec![],
            },
        ],
    };

    view! {
        <div class="tree-view-demo">
            <h2>"Tree View Demo"</h2>
            <p class="tree-view-subtitle">"Recursive hierarchy, expanded by default"</p>
            <div class="tree-view-container">
                <TreeNodeView node=demo_data />
            </div>
        </div>
    }
}
