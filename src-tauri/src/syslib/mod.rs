pub mod dbus;
pub mod events;
pub mod ffi;
pub mod network;
pub mod paths;
pub mod process;
pub mod resources;
pub mod session;
pub mod stdlib;
pub mod storage;
pub mod system;

// Re-export for easier access
pub use dbus::{send_notification, Notification};
pub use network::get_local_ips;
pub use paths::XdgPaths;
pub use resources::get_system_metrics;
pub use session::get_session_info;
pub use storage::get_disk_usage;
pub use system::SystemInfo;
