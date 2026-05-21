use std::path::{Path, PathBuf};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct AppFileSystem;

impl AppFileSystem {
    pub async fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, String> {
        let path_str = path.to_string_lossy().to_string();

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        extern "C" {
            #[wasm_bindgen(js_name = invoke)]
            fn invoke_list_dir(cmd: &str, args: JsValue) -> js_sys::Promise;
        }

        let args = serde_json::json!({ "path": path_str });
        let promise = invoke_list_dir(
            "list_directory",
            serde_wasm_bindgen::to_value(&args).unwrap(),
        );
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;

        match result {
            Ok(val) if !val.is_undefined() && !val.is_null() => {
                serde_wasm_bindgen::from_value(val).map_err(|e| e.to_string())
            }
            _ => Err("No data returned from Tauri".to_string()),
        }
    }

    pub async fn read_file(&self, path: &Path) -> Result<Vec<u8>, String> {
        let path_str = path.to_string_lossy().to_string();

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        extern "C" {
            #[wasm_bindgen(js_name = invoke)]
            fn invoke_read_file(cmd: &str, args: JsValue) -> js_sys::Promise;
        }

        let args = serde_json::json!({ "path": path_str });
        let promise = invoke_read_file("read_file", serde_wasm_bindgen::to_value(&args).unwrap());
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;

        match result {
            Ok(val) if !val.is_undefined() && !val.is_null() => {
                serde_wasm_bindgen::from_value(val).map_err(|e| e.to_string())
            }
            _ => Err("No data returned from Tauri".to_string()),
        }
    }

    pub async fn create_dir_all(&self, path: &Path) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        extern "C" {
            #[wasm_bindgen(js_name = invoke)]
            fn invoke_create_dir(cmd: &str, args: JsValue) -> js_sys::Promise;
        }

        let args = serde_json::json!({ "path": path_str });
        let promise = invoke_create_dir(
            "create_dir_all",
            serde_wasm_bindgen::to_value(&args).unwrap(),
        );
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;

        if result.is_ok() {
            Ok(())
        } else {
            Err(format!("{:?}", result.err()))
        }
    }
}
