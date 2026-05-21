use tauri::{
    menu::{Menu, MenuItem, Submenu},
    AppHandle, Emitter, Manager,
};

pub fn setup_main_menu(app: &AppHandle) -> tauri::Result<()> {
    // 1. Create Menu Items
    let new_item = MenuItem::with_id(app, "new", "New Project", true, Some("CmdOrCtrl+N"))?;
    let open_item = MenuItem::with_id(app, "open", "Open...", true, Some("CmdOrCtrl+O"))?;
    let save_item = MenuItem::with_id(app, "save", "Save", true, Some("CmdOrCtrl+S"))?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, Some("Cmd+Q"))?;

    let undo_item = MenuItem::with_id(app, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?;
    let redo_item = MenuItem::with_id(app, "redo", "Redo", true, Some("CmdOrCtrl+Y"))?;

    let theme_item = MenuItem::with_id(
        app,
        "toggle_theme",
        "Toggle Dark/Light Mode",
        true,
        Some("CmdOrCtrl+T"),
    )?;
    let about_item = MenuItem::with_id(app, "about", "About", true, None::<&str>)?;

    // 2. Create Submenus
    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[&new_item, &open_item, &save_item, &quit_item],
    )?;
    let edit_menu = Submenu::with_items(app, "Edit", true, &[&undo_item, &redo_item])?;
    let view_menu = Submenu::with_items(app, "View", true, &[&theme_item])?;
    let help_menu = Submenu::with_items(app, "Help", true, &[&about_item])?;

    // 3. Build the Main Menu
    let menu = Menu::with_items(app, &[&file_menu, &edit_menu, &view_menu, &help_menu])?;

    // 4. Set the menu on the main window
    if let Some(window) = app.get_webview_window("main") {
        window.set_menu(menu)?;
    }

    // 5. Register Event Handlers
    app.on_menu_event(move |app, event| match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "toggle_theme" => {
            let _ = app.emit("menu-toggle-theme", ());
        }
        "about" => {
            let _ = app.emit("menu-about", ());
        }
        _ => {
            println!("Menu item clicked: {:?}", event.id);
        }
    });

    Ok(())
}
