use crate::services::error_service::{ErrorService, ErrorSeverity};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[allow(dead_code)]
pub async fn invoke_with_error_handling<T>(
    cmd: &str,
    args: JsValue,
    error_service: ErrorService,
) -> Result<T, String>
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    match invoke(cmd, args).await {
        Ok(val) => {
            serde_wasm_bindgen::from_value(val).map_err(|e| format!("Deserialize error: {:?}", e))
        }
        Err(e) => {
            let msg = format!("Tauri command '{}' failed: {:?}", cmd, e);
            error_service.push(msg.clone(), ErrorSeverity::Error);
            Err(msg)
        }
    }
}
