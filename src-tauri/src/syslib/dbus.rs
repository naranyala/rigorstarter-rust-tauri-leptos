use std::error::Error;
use zbus::zvariant::Value;
use zbus::{Connection, Proxy};

pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
}

pub async fn send_notification(note: Notification) -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;

    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
    )
    .await?;

    // Notify returns a u32 (the notification ID)
    let _: Result<u32, _> = proxy
        .call(
            "Notify",
            &(
                "RigorStarter",
                0u32,
                note.icon.as_deref().unwrap_or(""),
                note.title,
                note.body,
                &[] as &[String],
                &std::collections::HashMap::<String, Value>::new(),
                5000i32,
            ),
        )
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_struct_basic() {
        let note = Notification {
            title: "Test Title".into(),
            body: "Test Body".into(),
            icon: None,
        };
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.body, "Test Body");
        assert!(note.icon.is_none());
    }

    #[test]
    fn test_notification_with_icon() {
        let note = Notification {
            title: "Alert".into(),
            body: "Something happened".into(),
            icon: Some("dialog-warning".into()),
        };
        assert_eq!(note.icon.unwrap(), "dialog-warning");
    }

    #[test]
    fn test_notification_empty_fields() {
        let note = Notification {
            title: String::new(),
            body: String::new(),
            icon: None,
        };
        assert!(note.title.is_empty());
        assert!(note.body.is_empty());
    }

    #[test]
    fn test_notification_clone() {
        let note = Notification {
            title: "Clone".into(),
            body: "Test".into(),
            icon: Some("info".into()),
        };
        let cloned = Notification {
            title: note.title.clone(),
            body: note.body.clone(),
            icon: note.icon.clone(),
        };
        assert_eq!(note.title, cloned.title);
        assert_eq!(note.body, cloned.body);
        assert_eq!(note.icon, cloned.icon);
    }
}
