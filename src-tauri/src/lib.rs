#![allow(dead_code)]

mod db;
mod errors;
#[cfg(test)]
mod lib_tests;
mod menu;
mod registry;
mod syslib;
mod utils;
mod utils_data;

use db::TodoItem;
use errors::AppError;
use menu::setup_main_menu;
use registry::{RegistryItem, REGISTRY_ITEMS};
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

#[tauri::command]
fn get_registry() -> Result<Vec<RegistryItem>, AppError> {
    let registry = REGISTRY_ITEMS
        .iter()
        .map(|(name, id, category, status)| {
            let line_count = if *category == "utility" {
                utils_data::get_utility_line_count(id)
            } else {
                0
            };

            RegistryItem {
                name: name.to_string(),
                id: id.to_string(),
                category: category.to_string(),
                status: status.to_string(),
                line_count,
            }
        })
        .collect();

    Ok(registry)
}

#[tauri::command]
fn get_utility_source(utility: &str) -> Result<String, AppError> {
    if utility.contains("..") || utility.contains('/') || utility.contains('\\') {
        return Err(AppError::InvalidArgument(
            "Invalid utility name".to_string(),
        ));
    }

    utils_data::get_utility_source(utility)
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::NotFound(format!("Utility source for '{}' not found", utility)))
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            initialize_database(app)?;
            setup_main_menu(app.handle())?;
            setup_tray(app)?;
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

fn initialize_database(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_data = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data)?;
    let db_path = app_data.join("todos.db");
    let conn = db::open(&db_path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    db::migrate(&conn).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    app.manage(Database {
        conn: Mutex::new(conn),
    });
    Ok(())
}

fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::TrayIconBuilder;

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
}
