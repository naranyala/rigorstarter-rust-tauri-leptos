mod utils;
mod errors;

use errors::AppError;
use std::fs;

#[tauri::command]
fn get_utility_source(utility: &str) -> Result<String, AppError> {
    if utility.contains("..") || utility.contains('/') || utility.contains('\\') {
        return Err(AppError::InvalidArgument("Invalid utility name".to_string()));
    }
    
    let path = format!("src-tauri/src/utils/{}.rs", utility);
    fs::read_to_string(path).map_err(AppError::from)
}

#[tauri::command]
fn greet(name: &str) -> Result<String, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::InvalidArgument("Name cannot be empty!".to_string()));
    }
    if name.to_lowercase() == "error" {
        return Err(AppError::Internal("You entered the forbidden word 'error'!".to_string()));
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
