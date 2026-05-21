use leptos::prelude::*;

#[derive(PartialEq, Clone)]
struct Token {
    text: String,
    color: &'static str,
}

fn highlight_rust(code: &str) -> Vec<Token> {
    let keywords = [
        "fn", "let", "mut", "pub", "use", "mod", "struct", "enum", "impl", "trait", "match", "if",
        "else", "loop", "while", "for", "in", "return", "break", "continue", "async", "await",
        "dyn", "type", "where", "crate", "super", "self", "Some", "None",
    ];

    let mut tokens = Vec::new();
    let mut current_word = String::new();

    for c in code.chars() {
        if c.is_alphanumeric() || c == '_' {
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                let color = if keywords.contains(&current_word.as_str()) {
                    "#c678dd"
                } else {
                    "#abb2bf"
                };
                tokens.push(Token {
                    text: current_word.clone(),
                    color,
                });
                current_word.clear();
            }
            tokens.push(Token {
                text: c.to_string(),
                color: "#abb2bf",
            });
        }
    }
    if !current_word.is_empty() {
        let color = if keywords.contains(&current_word.as_str()) {
            "#c678dd"
        } else {
            "#abb2bf"
        };
        tokens.push(Token {
            text: current_word.clone(),
            color,
        });
    }
    tokens
}

#[component]
pub fn CodeBlockView() -> impl IntoView {
    let code = r#"fn main() {
    let mut x = 10;
    if x > 5 {
        println!("Hello, Leptos!");
    } else {
        return;
    }
}

pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            age: 0,
        }
    }
}
"#;

    let tokens = highlight_rust(code);

    view! {
        <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;height:100vh;background-color:#282c34;color:#abb2bf;font-family:'Fira Code','Courier New',monospace;padding:20px;">
            <h2 style="color:white;margin-bottom:20px;font-family:sans-serif;">
                "Basic Code Highlighting Demo"
            </h2>

            <div style="background:#21252b;padding:20px;border-radius:10px;box-shadow:0 10px 30px rgba(0,0,0,0.5);border:1px solid #3e4451;line-height:1.5;white-space:pre-wrap;overflow-x:auto;max-width:800px;width:100%;">
                {tokens.into_iter().map(|token| view! {
                    <span style={format!("color: {}", token.color)}>{token.text}</span>
                }).collect::<Vec<_>>()}
            </div>

            <button
                style="margin-top:20px;padding:10px 20px;cursor:pointer;background:#61afef;border:none;border-radius:5px;color:#282c34;font-weight:bold;"
                on:click=move |_| {
                    // In a real app, integrate with the clipboard API
                    leptos::logging::log!("Copied to clipboard!");
                }
            >
                "Copy Code"
            </button>
        </div>
    }
}
