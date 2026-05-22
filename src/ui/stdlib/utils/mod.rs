use wasm_bindgen::JsCast;
use web_sys::Event;

pub fn event_target_value(ev: &Event) -> String {
    ev.target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

pub fn event_target_checked(ev: &Event) -> bool {
    ev.target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or(false)
}
