use crate::models::RegistryItem;

pub fn filter_registry(registry: &[RegistryItem], query: &str, category: &str) -> Vec<(String, String)> {
    let query_lower = query.to_lowercase();
    registry.iter()
        .filter(|item| item.category == category && item.name.to_lowercase().contains(&query_lower))
        .map(|item| (item.name.clone(), item.id.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_registry() -> Vec<RegistryItem> {
        vec![
            RegistryItem { name: "Accordion".into(), id: "acc".into(), category: "component".into(), status: "pinned".into(), line_count: 10 },
            RegistryItem { name: "Drawer".into(), id: "draw".into(), category: "component".into(), status: "dev".into(), line_count: 20 },
            RegistryItem { name: "Network".into(), id: "net".into(), category: "utility".into(), status: "pinned".into(), line_count: 30 },
        ]
    }

    #[test]
    fn test_filter_components_success() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "Acc", "component");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, "Accordion");
    }

    #[test]
    fn test_filter_case_insensitive() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "ACCORDION", "component");
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_filter_no_results() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "Unknown", "component");
        assert!(res.is_empty());
    }

    #[test]
    fn test_filter_wrong_category() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "Network", "component"); // Network is a utility
        assert!(res.is_empty());
    }
}
