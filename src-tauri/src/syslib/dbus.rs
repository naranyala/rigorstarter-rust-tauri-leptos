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
