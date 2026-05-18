mod app;
mod components;
mod core;
mod services;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    // Remove loader immediately before mounting
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(loader) = document.get_element_by_id("app-loader") {
                loader.remove();
            }
        }
    }

    mount_to_body(|| {
        view! {
            <ErrorBoundary
                fallback=move |errors| {
                    view! {
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: sans-serif; color: #d32f2f; text-align: center; padding: 2rem;">
                            <h1>"Application Error"</h1>
                            <p>"Something went wrong while rendering the application."</p>
                            <div style="background: #fff0f0; border: 1px solid #ffcccc; padding: 1rem; border-radius: 8px; max-width: 600px; overflow-x: auto; text-align: left; font-family: monospace; font-size: 0.8rem; margin: 1rem 0;">
                                {move || errors.get().clone().into_iter().map(|(_, e)| view! { <div>{e.to_string()}</div> }).collect_view()}
                            </div>
                            <button
                                on:click=move |_| {
                                    web_sys::window().unwrap().location().reload().unwrap();
                                }
                                style="padding: 0.5rem 1rem; cursor: pointer; background: #d32f2f; color: white; border: none; border-radius: 4px;"
                            >
                                "Reload Page"
                            </button>
                        </div>
                    }
                }
            >
                <App/>
            </ErrorBoundary>
        }
    })
}
