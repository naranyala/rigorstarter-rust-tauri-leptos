use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<AppTheme>,
}

impl ThemeContext {
    pub fn toggle(&self) {
        self.theme.update(|t| {
            *t = match *t {
                AppTheme::Light => AppTheme::Dark,
                AppTheme::Dark => AppTheme::Light,
            }
        });
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext should be provided");

    view! {
        <button
            class="theme-toggle-btn"
            on:click=move |_| theme_ctx.toggle()
        >
            {move || if theme_ctx.theme.get() == AppTheme::Light { "🌙 Switch to Dark" } else { "☀️ Switch to Light" }}
        </button>
    }
}

#[component]
fn ThemeContent() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext should be provided");

    let theme_class = move || {
        if theme_ctx.theme.get() == AppTheme::Light {
            "theme-light"
        } else {
            "theme-dark"
        }
    };

    view! {
        <div class=theme_class>
            <div class="theme-card">
                <h3>"Theme-Aware Component"</h3>
                <p>"This component reads the theme from the global context and updates its style automatically without prop drilling."</p>
            </div>
            <div class="theme-card">
                <h3>"Deeply Nested Component"</h3>
                <p>"I am even deeper in the tree, but I still know what the theme is!"</p>
                <NestedComponent />
            </div>
        </div>
    }
}

#[component]
fn NestedComponent() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext should be provided");

    view! {
        <div class="nested-theme-info">
            {move || format!("Current theme is: {}", if theme_ctx.theme.get() == AppTheme::Light { "Light" } else { "Dark" })}
        </div>
    }
}

#[component]
pub fn ThemeDemo() -> impl IntoView {
    let theme = RwSignal::new(AppTheme::Light);
    let theme_ctx = ThemeContext { theme };

    provide_context(theme_ctx);

    let theme_class = move || {
        if theme.get() == AppTheme::Light {
            "theme-light"
        } else {
            "theme-dark"
        }
    };

    view! {
        <div class="theme-demo">
            <h2>"Context Pattern: Theme Switcher"</h2>
            <div class=move || format!("theme-demo-container {}", theme_class())>
                <ThemeToggle />
                <ThemeContent />
            </div>
        </div>
    }
}
