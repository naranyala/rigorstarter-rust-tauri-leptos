use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[derive(Debug, Clone, Copy)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub fn use_window_size() -> Signal<WindowSize> {
    let (size, set_size) = signal(WindowSize {
        width: 1024,
        height: 768,
    });

    let window_obj = window().expect("no global window found");

    let on_resize = Closure::wrap(Box::new(move || {
        let w = window()
            .and_then(|w| w.inner_width().ok().and_then(|v| v.as_f64()))
            .unwrap_or(1024.0);
        let h = window()
            .and_then(|w| w.inner_height().ok().and_then(|v| v.as_f64()))
            .unwrap_or(768.0);
        set_size.set(WindowSize {
            width: w as u32,
            height: h as u32,
        });
    }) as Box<dyn FnMut()>);

    window_obj
        .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
        .expect("failed to add resize listener");

    on_resize.forget();

    size.into()
}
