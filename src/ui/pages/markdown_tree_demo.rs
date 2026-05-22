use crate::core::models::TreeNode;
use leptos::prelude::*;
use pulldown_cmark::{html, Parser};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

#[component]
fn MarkdownTreeNodeView(node: TreeNode, on_select: Callback<String>) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(true);
    let label = node.label;
    let id = node.id.clone();
    let has_children = !node.children.is_empty();
    let (children_read, _set_children) = signal(node.children);

    view! {
        <div class="tree-node">
            <div
                class="tree-node-header"
                on:click=move |_| {
                    if has_children {
                        set_is_expanded.update(|v| *v = !*v);
                    }
                    on_select.run(id.clone());
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
                    {move || children_read.get().into_iter().map(|child| {
                        view! { <MarkdownTreeNodeView node=child.clone() on_select=on_select /> }
                    }).collect_view()}
                </div>
            </Show>
        </div>
    }
    .into_any()
}

#[component]
pub fn MarkdownTreeDemo() -> impl IntoView {
    // Mock document tree
    let demo_tree = TreeNode {
        id: "root".into(),
        label: "Project Documentation".into(),
        children: vec![
            TreeNode {
                id: "intro".into(),
                label: "Introduction".into(),
                children: vec![],
            },
            TreeNode {
                id: "guides".into(),
                label: "User Guides".into(),
                children: vec![
                    TreeNode {
                        id: "setup".into(),
                        label: "Getting Started".into(),
                        children: vec![],
                    },
                    TreeNode {
                        id: "advanced".into(),
                        label: "Advanced Usage".into(),
                        children: vec![],
                    },
                ],
            },
            TreeNode {
                id: "api".into(),
                label: "API Reference".into(),
                children: vec![
                    TreeNode {
                        id: "core".into(),
                        label: "Core API".into(),
                        children: vec![],
                    },
                    TreeNode {
                        id: "plugins".into(),
                        label: "Plugins API".into(),
                        children: vec![],
                    },
                ],
            },
        ],
    };

    let (selected_id, set_selected_id) = signal("root".to_string());
    let (rendered_html, set_rendered_html) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    // Simulate fetching content from a file or API
    let fetch_content = move |id: String| {
        let set_rendered_html = set_rendered_html;
        let set_is_loading = set_is_loading;

        spawn_local(async move {
            set_is_loading.set(true);

            // Simulation of async loading (e.g. from a file)
            // In a real app, this would be: invoke("read_markdown_file", json!({ "path": id }))
            let content_map: HashMap<&'static str, &'static str> = [
                ("root", "# Project Documentation\n\nSelect a file from the sidebar to begin."),
                ("intro", "# Introduction\n\nWelcome to the documentation. This project is built with **Leptos** and **Tauri**."),
                ("guides", "# User Guides\n\nPlease select a specific guide from the sub-menu."),
                ("setup", "# Getting Started\n\n1. Install Rust\n2. Run `cargo build`\n3. Enjoy!"),
                ("advanced", "# Advanced Usage\n\nExplore the deep internals of our system..."),
                ("api", "# API Reference\n\nDetailed documentation of our public API."),
                ("core", "# Core API\n\n`pub fn init()`: Initializes the core system."),
                ("plugins", "# Plugins API\n\nLearn how to write your own plugins."),
            ].into_iter().collect();

            let content = content_map
                .get(id.as_str())
                .unwrap_or(&"# No Content\n\nNo content found for this node.");

            let parser = Parser::new(content);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            set_rendered_html.set(html_output);
            set_is_loading.set(false);
        });
    };

    // Initial load
    Effect::new(move |_| {
        let id = selected_id.get();
        fetch_content(id);
    });

    view! {
        <div class="markdown-tree-demo" style="display: flex; height: 100%; gap: 20px;">
            <div class="tree-sidebar" style="width: 300px; border-right: 1px solid var(--border-color); padding: 1rem; overflow-y: auto;">
                <h3>"Documentation"</h3>
                <div class="tree-container">
                    <MarkdownTreeNodeView
                        node=demo_tree
                        on_select=Callback::new(move |id: String| {
                            set_selected_id.set(id.clone());
                            fetch_content(id);
                        })
                    />
                </div>
            </div>
            <div class="markdown-content" style="flex: 1; padding: 1rem; overflow-y: auto; position: relative;">
                {move || if is_loading.get() {
                    view! { <div class="loading-overlay"><p>"Loading document..."</p></div> }.into_any()
                } else {
                    view! { <div class="markdown-view" inner_html=rendered_html.get() /> }.into_any()
                }}
            </div>
        </div>
    }
}
