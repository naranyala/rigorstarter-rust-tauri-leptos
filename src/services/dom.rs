#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use web_sys::{FileList, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

#[allow(dead_code)]
pub fn event_target_value(ev: &leptos::ev::Event) -> String {
    ev.target()
        .and_then(|t| {
            t.dyn_into::<web_sys::HtmlElement>().ok().and_then(|el| {
                if let Ok(input) = el.clone().dyn_into::<HtmlInputElement>() {
                    Some(input.value())
                } else if let Ok(select) = el.clone().dyn_into::<HtmlSelectElement>() {
                    Some(select.value())
                } else if let Ok(textarea) = el.clone().dyn_into::<HtmlTextAreaElement>() {
                    Some(textarea.value())
                } else {
                    el.text_content()
                }
            })
        })
        .unwrap_or_default()
}

pub fn event_target_checked(ev: &leptos::ev::Event) -> bool {
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or(false)
}

pub fn event_target_files(ev: &leptos::ev::Event) -> Option<FileList> {
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .and_then(|input| input.files())
}

pub fn event_target_value_as_i32(ev: &leptos::ev::Event) -> i32 {
    event_target_value(ev).parse().unwrap_or(0)
}

pub fn event_target_value_as_f64(ev: &leptos::ev::Event) -> f64 {
    event_target_value(ev).parse().unwrap_or(0.0)
}

pub fn event_target_value_as_bool(ev: &leptos::ev::Event) -> bool {
    let v = event_target_value(ev).to_lowercase();
    matches!(v.as_str(), "true" | "1" | "yes" | "on")
}

pub fn focus_element(el: &web_sys::HtmlElement) {
    let _ = el.focus();
}

pub fn blur_element(el: &web_sys::HtmlElement) {
    let _ = el.blur();
}

pub fn scroll_into_view(el: &web_sys::HtmlElement) {
    el.scroll_into_view();
}

pub fn scroll_into_view_smooth(el: &web_sys::HtmlElement) {
    el.scroll_into_view_with_bool(true);
}

pub fn set_element_html(el: &web_sys::HtmlElement, html: &str) {
    el.set_inner_html(html);
}

pub fn get_element_text(el: &web_sys::HtmlElement) -> String {
    el.text_content().unwrap_or_default()
}

pub fn query_selector(selector: &str) -> Option<web_sys::Element> {
    web_sys::window()?
        .document()?
        .query_selector(selector)
        .ok()
        .flatten()
}

pub fn get_element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(id)
        .map(|el| el.unchecked_into())
}

pub fn add_class(el: &web_sys::HtmlElement, class: &str) {
    let _ = el.class_list().add_1(class);
}

pub fn remove_class(el: &web_sys::HtmlElement, class: &str) {
    let _ = el.class_list().remove_1(class);
}

pub fn toggle_class(el: &web_sys::HtmlElement, class: &str) {
    if el.class_list().contains(class) {
        let _ = el.class_list().remove_1(class);
    } else {
        let _ = el.class_list().add_1(class);
    }
}

pub fn has_class(el: &web_sys::HtmlElement, class: &str) -> bool {
    el.class_list().contains(class)
}

pub fn set_data_attribute(el: &web_sys::HtmlElement, key: &str, value: &str) {
    let _ = el.set_attribute(&format!("data-{}", key), value);
}

pub fn get_data_attribute(el: &web_sys::HtmlElement, key: &str) -> Option<String> {
    el.get_attribute(&format!("data-{}", key))
}

pub fn get_viewport_size() -> (u32, u32) {
    let window = web_sys::window().unwrap();
    (
        window
            .inner_width()
            .ok()
            .map(|v| v.as_f64().unwrap_or(0.0) as u32)
            .unwrap_or(0),
        window
            .inner_height()
            .ok()
            .map(|v| v.as_f64().unwrap_or(0.0) as u32)
            .unwrap_or(0),
    )
}

pub fn get_scroll_position() -> (u32, u32) {
    let window = web_sys::window().unwrap();
    let scroll_x = window.scroll_x().ok().map(|v| v as u32).unwrap_or(0);
    let scroll_y = window.scroll_y().ok().map(|v| v as u32).unwrap_or(0);
    (scroll_x, scroll_y)
}

pub fn copy_to_clipboard(text: &str) {
    if let Some(clipboard) = web_sys::window().map(|w| w.navigator().clipboard()) {
        let _ = clipboard.write_text(text);
    }
}

pub fn request_animation_frame(callback: impl FnMut() + 'static) {
    let window = web_sys::window().unwrap();
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
    let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
    closure.forget();
}
