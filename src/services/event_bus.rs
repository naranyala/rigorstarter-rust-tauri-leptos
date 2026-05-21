use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppEvent {
    Error(String),
    Info(String),
    Notification(String),
}

pub struct EventBus;

impl EventBus {
    pub async fn publish<T: Serialize>(event_name: &str, payload: T) -> Result<(), JsValue> {
        let js_payload = serde_wasm_bindgen::to_value(&payload)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
        extern "C" {
            pub fn emit(event: &str, payload: JsValue);
        }

        emit(event_name, js_payload);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn subscribe<T: for<'de> Deserialize<'de> + 'static>(
        event_name: &str,
        callback: Rc<dyn Fn(T)>,
    ) -> Result<JsValue, JsValue> {
        let callback_clone = callback.clone();

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(payload) = serde_wasm_bindgen::from_value::<T>(event) {
                callback_clone(payload);
            }
        }) as Box<dyn FnMut(JsValue)>);

        let callback_ref = closure.as_ref().unchecked_ref();

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
        extern "C" {
            pub async fn listen(
                event: &str,
                callback: &js_sys::Function,
            ) -> js_sys::Promise<JsValue>;
        }

        let promise = listen(event_name, callback_ref).await;

        closure.forget();

        Ok(promise.into())
    }
}
