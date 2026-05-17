// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;
use std::fs;

#[tauri::command]
fn get_utility_source(utility: &str) -> Result<String, String> {
    let path = format!("src-tauri/src/utils/{}.rs", utility);
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("Name cannot be empty!".to_string());
    }
    if name.to_lowercase() == "error" {
        return Err("You entered the forbidden word 'error'!".to_string());
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_utility_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
