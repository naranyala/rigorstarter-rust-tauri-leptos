use leptos::prelude::*;
use pulldown_cmark::{html, Parser};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarkdownMode {
    Edit,
    View,
}

#[component]
pub fn MarkdownDemo() -> impl IntoView {
    let (mode, set_mode) = signal(MarkdownMode::Edit);
    let (content, set_content) = signal("## Hello Markdown!\n\nStart typing here...".to_string());

    let rendered_html = Memo::new(move |_| {
        let text = content.get();
        let parser = Parser::new(&text);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    });

    view! {
        <div class="markdown-demo">
            <div class="markdown-demo-header">
                <h2>"Markdown Editor"</h2>
                <div class="markdown-demo-controls">
                    <button
                        class=move || if mode.get() == MarkdownMode::Edit { "active" } else { "" }
                        on:click=move |_| set_mode.set(MarkdownMode::Edit)
                    >
                        "Edit"
                    </button>
                    <button
                        class=move || if mode.get() == MarkdownMode::View { "active" } else { "" }
                        on:click=move |_| set_mode.set(MarkdownMode::View)
                    >
                        "View"
                    </button>
                </div>
            </div>

            <div class="markdown-demo-body">
                <Show
                    when=move || mode.get() == MarkdownMode::Edit
                    fallback=move || view! { <div class="markdown-view" inner_html=rendered_html.get() /> }.into_any()
                >
                    <textarea
                        class="markdown-editor"
                        on:input=move |ev| set_content.set(event_target_value(&ev))
                        prop:value=content
                    />
                </Show>
            </div>
        </div>
    }
}
