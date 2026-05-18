#![allow(dead_code)]

mod db;
mod errors;
#[cfg(test)]
mod lib_tests;
mod menu;
mod syslib;
mod utils;

use db::TodoItem;
use errors::AppError;
use menu::setup_main_menu;
use std::fs;
use std::sync::Mutex;
use syslib::{
    get_disk_usage, get_local_ips, get_session_info, get_system_metrics, send_notification,
    Notification, SystemInfo, XdgPaths,
};
use tauri::Manager;

pub struct Database {
    pub conn: Mutex<rusqlite::Connection>,
}

// --- Todo Commands ---

#[tauri::command]
fn add_todo(title: String, state: tauri::State<'_, Database>) -> Result<TodoItem, AppError> {
    let conn = state
        .conn
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    db::insert(&conn, &title).map_err(|e| AppError::Internal(e))
}

#[tauri::command]
fn list_todos(state: tauri::State<'_, Database>) -> Result<Vec<TodoItem>, AppError> {
    let conn = state
        .conn
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    db::list_all(&conn).map_err(|e| AppError::Internal(e))
}

#[tauri::command]
fn toggle_todo(id: i64, state: tauri::State<'_, Database>) -> Result<TodoItem, AppError> {
    let conn = state
        .conn
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    db::toggle(&conn, id).map_err(|e| AppError::Internal(e))
}

#[tauri::command]
fn delete_todo(id: i64, state: tauri::State<'_, Database>) -> Result<(), AppError> {
    let conn = state
        .conn
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    db::delete(&conn, id).map_err(|e| AppError::Internal(e))
}

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
        ("Todo List", "todo", "component", "pinned"),
        ("Tree View", "tree_view", "component", "pinned"),
        ("JSON Todo", "json_todo", "component", "pinned"),
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

#[tauri::command]
async fn open_file_dialog(app: tauri::AppHandle) -> Result<Option<String>, AppError> {
    use tauri_plugin_dialog::DialogExt;
    let file_path = app
        .dialog()
        .file()
        .set_title("Select a file")
        .blocking_pick_file();
    Ok(file_path.map(|p| p.to_string()))
}

#[tauri::command]
async fn open_directory_dialog(app: tauri::AppHandle) -> Result<Option<String>, AppError> {
    use tauri_plugin_dialog::DialogExt;
    let dir_path = app
        .dialog()
        .file()
        .set_title("Select a directory")
        .blocking_pick_folder();
    Ok(dir_path.map(|p| p.to_string()))
}

#[tauri::command]
fn show_message_dialog(
    app: tauri::AppHandle,
    title: String,
    message: String,
) -> Result<(), AppError> {
    use tauri_plugin_dialog::DialogExt;
    app.dialog().message(message).title(title).show(|_| {});
    Ok(())
}

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize SQLite database
            let app_data = app.path().app_data_dir().map_err(|e| Box::new(e))?;
            std::fs::create_dir_all(&app_data)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let db_path = app_data.join("todos.db");
            let conn = db::open(&db_path).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                    as Box<dyn std::error::Error>
            })?;
            db::migrate(&conn).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                    as Box<dyn std::error::Error>
            })?;
            app.manage(Database {
                conn: Mutex::new(conn),
            });

            setup_main_menu(app.handle())?;

            // 1. Create Menu Items
            let show_window = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let quit_app = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            // 2. Build the Menu
            let menu = Menu::with_items(app, &[&show_window, &quit_app])?;

            // 3. Build the Tray Icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)
                .unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_utility_source,
            get_registry,
            log_message,
            get_system_info,
            get_system_status,
            notify_user,
            open_file_dialog,
            open_directory_dialog,
            show_message_dialog,
            add_todo,
            list_todos,
            toggle_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
