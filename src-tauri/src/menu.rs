use tauri::{
    menu::{Menu, MenuItem, Submenu},
    AppHandle, Manager,
};

pub fn setup_main_menu(app: &AppHandle) -> tauri::Result<()> {
    // 1. Create Menu Items
    let new_item = MenuItem::with_id(app, "new", "New Project", true, None::<&str>)?;
    let open_item = MenuItem::with_id(app, "open", "Open...", true, None::<&str>)?;
    let save_item = MenuItem::with_id(app, "save", "Save", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let undo_item = MenuItem::with_id(app, "undo", "Undo", true, None::<&str>)?;
    let redo_item = MenuItem::with_id(app, "redo", "Redo", true, None::<&str>)?;

    let theme_item = MenuItem::with_id(
        app,
        "toggle_theme",
        "Toggle Dark/Light Mode",
        true,
        None::<&str>,
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
    app.on_menu_event(move |app, event| {
        match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "toggle_theme" => {
                // Here we would trigger the theme service
                // For now, we can use a custom event or call a command
                println!("Menu: Toggle Theme triggered");
                // In a real implementation, you'd call your theme service logic here
            }
            "about" => {
                println!("Menu: About clicked");
            }
            _ => {
                println!("Menu item clicked: {:?}", event.id);
            }
        }
    });

    Ok(())
}
