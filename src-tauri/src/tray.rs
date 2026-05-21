use anyhow::Result;
use tray_icon::menu::{Menu, MenuItem};
use tray_icon::TrayIconBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    ShowApp,
    Quit,
}

pub struct TrayManager;

#[allow(clippy::new_without_default)]
impl TrayManager {
    pub fn new() -> Result<(Self, tray_icon::TrayIcon)> {
        let menu = Menu::new();

        let show_item = MenuItem::with_id("show_app", "Show Application", true, None);
        let quit_item = MenuItem::with_id("quit", "Quit", false, None);

        let _ = menu.append(&show_item);
        let _ = menu.append(&quit_item);

        let icon_data = vec![0u8; 4];

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("WhisperCPP GUI")
            .with_icon(tray_icon::Icon::from_rgba(icon_data, 1, 1).map_err(|e| anyhow::anyhow!(e))?)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build tray icon: {}", e))?;

        Ok((Self, tray_icon))
    }

    pub fn handle_event(&self, event: tray_icon::menu::MenuEvent) -> Option<TrayAction> {
        match event.id.as_ref() {
            "show_app" => Some(TrayAction::ShowApp),
            "quit" => Some(TrayAction::Quit),
            _ => None,
        }
    }
}
