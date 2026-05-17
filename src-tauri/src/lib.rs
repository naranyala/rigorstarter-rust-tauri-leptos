mod errors;
#[cfg(test)]
mod lib_tests;
mod syslib;
mod utils;

use errors::AppError;
use std::fs;
use syslib::{
    get_disk_usage, get_local_ips, get_session_info, get_system_metrics, send_notification,
    Notification, SystemInfo, XdgPaths,
};

fn get_file_line_count(path: String) -> Result<usize, AppError> {
    fs::read_to_string(path)
        .map(|s| s.lines().count())
        .map_err(|e| AppError::NotFound(e.to_string()))
}

#[derive(serde::Serialize)]
pub struct RegistryItem {
    pub name: String,
    pub id: String,
    pub category: String,
    pub status: String,
    pub line_count: usize,
}

#[tauri::command]
fn get_registry() -> Result<Vec<RegistryItem>, AppError> {
    let items = vec![
        ("Accordion", "accordion", "component", "pinned"),
        ("Drawer", "drawer", "component", "in-development"),
        ("Tabs", "tabs", "component", "pinned"),
        ("Modal", "modal", "component", "in-development"),
        ("Network", "network", "utility", "pinned"),
        ("System", "system", "utility", "archives"),
        ("Storage", "storage", "utility", "in-development"),
        ("Process", "process", "utility", "pinned"),
        ("Disk Usage", "disk_usage", "utility", "pinned"),
        ("Env Vars", "env_vars", "utility", "archives"),
    ];

    let mut registry = Vec::new();
    for (name, id, category, status) in items {
        let line_count = if category == "utility" {
            let paths = vec![
                format!("src-tauri/src/utils/{}.rs", id),
                format!("src/utils/{}.rs", id),
            ];

            paths
                .into_iter()
                .find_map(|path| fs::read_to_string(path).ok())
                .map(|s| s.lines().count())
                .unwrap_or(0)
        } else {
            0 // Components are frontend, line count not easily available here
        };

        registry.push(RegistryItem {
            name: name.to_string(),
            id: id.to_string(),
            category: category.to_string(),
            status: status.to_string(),
            line_count,
        });
    }

    Ok(registry)
}

#[tauri::command]
fn get_utility_source(utility: &str) -> Result<String, AppError> {
    if utility.contains("..") || utility.contains('/') || utility.contains('\\') {
        return Err(AppError::InvalidArgument(
            "Invalid utility name".to_string(),
        ));
    }

    let paths = vec![
        format!("src-tauri/src/utils/{}.rs", utility),
        format!("src/utils/{}.rs", utility),
    ];

    for path in paths {
        if let Ok(content) = fs::read_to_string(&path) {
            return Ok(content);
        }
    }

    Err(AppError::NotFound(format!(
        "Utility source for '{}' not found",
        utility
    )))
}

#[tauri::command]
fn log_message(message: String) {
    println!("[FRONTEND LOG] {}", message);
}

#[tauri::command]
async fn notify_user(title: String, body: String) -> Result<(), AppError> {
    let note = Notification {
        title,
        body,
        icon: None,
    };

    send_notification(note)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
fn get_system_status() -> Result<serde_json::Value, AppError> {
    let info = SystemInfo::collect();
    let (mem, cpu) = get_system_metrics();
    let session = get_session_info();
    let nets = get_local_ips();
    let disk = get_disk_usage("/");

    Ok(serde_json::json!({
        "system": {
            "hostname": info.hostname,
            "kernel": info.kernel,
            "distro": info.distro,
        },
        "resources": {
            "memory": mem,
            "cpu": cpu,
        },
        "session": session,
        "network": nets,
        "storage": disk,
    }))
}

#[tauri::command]
fn get_system_info() -> Result<serde_json::Value, AppError> {
    let info = SystemInfo::collect();
    let paths = XdgPaths::new("rigorstarter");

    Ok(serde_json::json!({
        "hostname": info.hostname,
        "kernel": info.kernel,
        "distro": info.distro,
        "paths": {
            "config": paths.config_dir,
            "data": paths.data_dir,
            "cache": paths.cache_dir,
        }
    }))
}

#[tauri::command]
fn greet(name: &str) -> Result<String, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::InvalidArgument(
            "Name cannot be empty!".to_string(),
        ));
    }
    if name.to_lowercase() == "error" {
        return Err(AppError::Internal(
            "You entered the forbidden word 'error'!".to_string(),
        ));
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_utility_source,
            get_registry,
            log_message,
            get_system_info,
            get_system_status,
            notify_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
