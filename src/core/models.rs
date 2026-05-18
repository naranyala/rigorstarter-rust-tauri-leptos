use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryItem {
    pub name: String,
    pub id: String,
    pub category: String,
    pub status: String,
    pub line_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonTodo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[cfg(test)]
mod tree_node_tests {
    use super::*;

    #[test]
    fn test_tree_node_serde_roundtrip() {
        let node = TreeNode {
            id: "root".into(),
            label: "Root".into(),
            children: vec![
                TreeNode {
                    id: "child1".into(),
                    label: "Child 1".into(),
                    children: vec![],
                },
                TreeNode {
                    id: "child2".into(),
                    label: "Child 2".into(),
                    children: vec![TreeNode {
                        id: "grandchild1".into(),
                        label: "Grandchild 1".into(),
                        children: vec![],
                    }],
                },
            ],
        };
        let json = serde_json::to_string(&node).unwrap();
        let deserialized: TreeNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, deserialized);
    }

    #[test]
    fn test_tree_node_empty_children() {
        let node = TreeNode {
            id: "empty".into(),
            label: "Empty".into(),
            children: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        let deserialized: TreeNode = serde_json::from_str(&json).unwrap();
        assert!(deserialized.children.is_empty());
    }
}

#[cfg(test)]
mod todo_tests {
    use super::*;

    #[test]
    fn test_todo_item_serde_roundtrip() {
        let item = TodoItem {
            id: 1,
            title: "Test todo".into(),
            completed: false,
            created_at: "2025-01-01 12:00:00".into(),
        };
        let json = serde_json::to_string(&item).unwrap();
        let deserialized: TodoItem = serde_json::from_str(&json).unwrap();
        assert_eq!(item.id, deserialized.id);
        assert_eq!(item.title, deserialized.title);
        assert_eq!(item.completed, deserialized.completed);
    }

    #[test]
    fn test_todo_item_completed_serialization() {
        let item = TodoItem {
            id: 2,
            title: "Done".into(),
            completed: true,
            created_at: "2025-06-01".into(),
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], 2);
        assert_eq!(json["title"], "Done");
        assert_eq!(json["completed"], true);
    }

    #[test]
    fn test_todo_item_clone() {
        let item = TodoItem {
            id: 3,
            title: "Clone".into(),
            completed: true,
            created_at: "now".into(),
        };
        assert_eq!(item, item.clone());
    }
}

#[cfg(test)]
mod json_todo_tests {
    use super::*;

    #[test]
    fn test_json_todo_serde_roundtrip() {
        let item = JsonTodo {
            id: 1,
            title: "Test".into(),
            completed: false,
        };
        let json = serde_json::to_string(&item).unwrap();
        let deserialized: JsonTodo = serde_json::from_str(&json).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_json_todo_json_structure() {
        let item = JsonTodo {
            id: 42,
            title: "Buy milk".into(),
            completed: true,
        };
        let value = serde_json::to_value(&item).unwrap();
        assert_eq!(value["id"], 42);
        assert_eq!(value["title"], "Buy milk");
        assert_eq!(value["completed"], true);
    }

    #[test]
    fn test_json_todo_clone_equal() {
        let item = JsonTodo {
            id: 7,
            title: "Clone me".into(),
            completed: false,
        };
        assert_eq!(item, item.clone());
    }

    #[test]
    fn test_json_todo_empty_title() {
        let item = JsonTodo {
            id: 0,
            title: String::new(),
            completed: false,
        };
        assert!(item.title.is_empty());
    }
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
