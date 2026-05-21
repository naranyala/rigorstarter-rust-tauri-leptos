use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct RegistryItem {
    pub name: String,
    pub id: String,
    pub category: String,
    pub status: String,
    pub line_count: usize,
}

pub const REGISTRY_ITEMS: &[(&str, &str, &str, &str)] = &[
    ("Accordion", "accordion", "component", "pinned"),
    ("Audio Player", "audio_player", "component", "pinned"),
    ("Audio Recorder", "audio_recorder", "component", "pinned"),
    ("Calendar", "calendar", "component", "pinned"),
    ("Code Block", "codeblock", "component", "pinned"),
    ("Drawer", "drawer", "component", "in-development"),
    ("Image Viewer", "image_viewer", "component", "pinned"),
    ("Lightbox", "lightbox", "component", "pinned"),
    ("Microphone", "microphone", "component", "pinned"),
    ("Tabs", "tabs", "component", "pinned"),
    ("Modal", "modal", "component", "in-development"),
    ("Todo List", "todo", "component", "pinned"),
    ("Tree View", "tree_view", "component", "pinned"),
    ("JSON Todo", "json_todo", "component", "pinned"),
    ("Table Demo", "table_demo", "component", "pinned"),
    ("Markdown Demo", "markdown_demo", "component", "pinned"),
    ("Theme Demo", "theme_demo", "component", "pinned"),
    ("Toast Demo", "toast_demo", "component", "pinned"),
    ("Network", "network", "utility", "pinned"),
    ("System", "system", "utility", "archives"),
    ("Storage", "storage", "utility", "in-development"),
    ("Process", "process", "utility", "pinned"),
    ("Disk Usage", "disk_usage", "utility", "pinned"),
    ("Env Vars", "env_vars", "utility", "archives"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_items_count() {
        assert_eq!(REGISTRY_ITEMS.len(), 24);
    }

    #[test]
    fn test_registry_items_content() {
        let first = REGISTRY_ITEMS[0];
        assert_eq!(first.0, "Accordion");
        assert_eq!(first.1, "accordion");
        assert_eq!(first.2, "component");
        assert_eq!(first.3, "pinned");
    }

    #[test]
    fn test_registry_item_serialization() {
        let item = RegistryItem {
            name: "Test".into(),
            id: "t1".into(),
            category: "comp".into(),
            status: "p".into(),
            line_count: 5,
        };
        let serialized = serde_json::to_string(&item).unwrap();
        assert!(serialized.contains(r#""name":"Test""#));
        assert!(serialized.contains(r#""line_count":5"#));
    }
}
