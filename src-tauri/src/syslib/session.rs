use std::env;

#[derive(Debug, serde::Serialize)]
pub enum WindowingSystem {
    Wayland,
    X11,
    Unknown,
}

#[derive(Debug, serde::Serialize)]
pub struct SessionInfo {
    pub desktop_env: String,
    pub windowing_system: WindowingSystem,
    pub session_id: Option<String>,
}

pub fn get_session_info() -> SessionInfo {
    let desktop_env = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| env::var("DESKTOP_SESSION").unwrap_or_else(|_| "Unknown".to_string()));

    let windowing_system = if env::var("WAYLAND_DISPLAY").is_ok() {
        WindowingSystem::Wayland
    } else if env::var("DISPLAY").is_ok() {
        WindowingSystem::X11
    } else {
        WindowingSystem::Unknown
    };

    let session_id = env::var("XDG_SESSION_ID").ok();

    SessionInfo {
        desktop_env,
        windowing_system,
        session_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_info_consistency() {
        let info = get_session_info();
        assert!(!info.desktop_env.is_empty());
        match info.windowing_system {
            WindowingSystem::Wayland | WindowingSystem::X11 | WindowingSystem::Unknown => {}
        }
    }

    #[test]
    fn test_session_info_fields_present() {
        let info = get_session_info();
        // desktop_env should always be set (defaults to "Unknown" if no env vars)
        assert!(!info.desktop_env.is_empty());
        // session_id may be None
        if let Some(ref sid) = info.session_id {
            assert!(!sid.is_empty());
        }
    }

    #[test]
    fn test_windowing_system_debug_display() {
        let variants = [
            WindowingSystem::Wayland,
            WindowingSystem::X11,
            WindowingSystem::Unknown,
        ];
        for v in &variants {
            let debug = format!("{:?}", v);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_session_info_serialization() {
        let info = SessionInfo {
            desktop_env: "GNOME".into(),
            windowing_system: WindowingSystem::Wayland,
            session_id: Some("42".into()),
        };
        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["desktop_env"], "GNOME");
        assert_eq!(json["windowing_system"], "Wayland");
        assert_eq!(json["session_id"], "42");
    }

    #[test]
    fn test_session_info_serialization_none_id() {
        let info = SessionInfo {
            desktop_env: "KDE".into(),
            windowing_system: WindowingSystem::X11,
            session_id: None,
        };
        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["desktop_env"], "KDE");
        assert_eq!(json["windowing_system"], "X11");
        assert!(json["session_id"].is_null());
    }
}
