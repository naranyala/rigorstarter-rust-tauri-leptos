use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryItem {
    pub name: String,
    pub id: String,
    pub category: String,
    pub status: String,
    pub line_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_item_serde_roundtrip() {
        let item = RegistryItem {
            name: "Accordion".into(),
            id: "accordion".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 42,
        };
        let json = serde_json::to_string(&item).unwrap();
        let deserialized: RegistryItem = serde_json::from_str(&json).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_registry_item_serde_json_structure() {
        let item = RegistryItem {
            name: "Test".into(),
            id: "t1".into(),
            category: "utility".into(),
            status: "dev".into(),
            line_count: 100,
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["name"], "Test");
        assert_eq!(json["id"], "t1");
        assert_eq!(json["category"], "utility");
        assert_eq!(json["status"], "dev");
        assert_eq!(json["line_count"], 100);
    }

    #[test]
    fn test_registry_item_clone_equal() {
        let item = RegistryItem {
            name: "Clone".into(),
            id: "c1".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        };
        assert_eq!(item, item.clone());
    }

    #[test]
    fn test_registry_item_not_equal() {
        let a = RegistryItem {
            name: "A".into(),
            id: "a".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        };
        let b = RegistryItem {
            name: "B".into(),
            id: "b".into(),
            category: "utility".into(),
            status: "dev".into(),
            line_count: 20,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_registry_item_zero_line_count() {
        let item = RegistryItem {
            name: "Empty".into(),
            id: "e1".into(),
            category: "component".into(),
            status: "new".into(),
            line_count: 0,
        };
        assert_eq!(item.line_count, 0);
    }

    #[test]
    fn test_registry_item_empty_strings() {
        let item = RegistryItem {
            name: String::new(),
            id: String::new(),
            category: String::new(),
            status: String::new(),
            line_count: 0,
        };
        assert!(item.name.is_empty());
        assert!(item.id.is_empty());
    }
}
