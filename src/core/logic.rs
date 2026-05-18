use crate::core::models::RegistryItem;

pub fn filter_registry(
    registry: &[RegistryItem],
    query: &str,
    category: &str,
) -> Vec<(String, String)> {
    let query_lower = query.to_lowercase();
    registry
        .iter()
        .filter(|item| item.category == category && item.name.to_lowercase().contains(&query_lower))
        .map(|item| (item.name.clone(), item.id.clone()))
        .collect()
}

pub fn sort_registry_by_status(registry: &[RegistryItem]) -> Vec<RegistryItem> {
    let mut sorted = registry.to_vec();
    sorted.sort_by_key(|item| match item.status.as_str() {
        "pinned" => 0,
        "in-development" => 1,
        "archives" => 2,
        _ => 3,
    });
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_registry() -> Vec<RegistryItem> {
        vec![
            RegistryItem {
                name: "Accordion".into(),
                id: "acc".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Drawer".into(),
                id: "draw".into(),
                category: "component".into(),
                status: "dev".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "Network".into(),
                id: "net".into(),
                category: "utility".into(),
                status: "pinned".into(),
                line_count: 30,
            },
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
    fn test_filter_multiple_results() {
        let reg = vec![
            RegistryItem {
                name: "Accordion".into(),
                id: "acc".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Accent".into(),
                id: "acc2".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];
        let res = filter_registry(&reg, "Acc", "component");
        assert_eq!(res.len(), 2);
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

    #[test]
    fn test_filter_empty_registry() {
        let reg = vec![];
        let res = filter_registry(&reg, "Acc", "component");
        assert!(res.is_empty());
    }

    #[test]
    fn test_filter_empty_query() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "", "component");
        // Should return all components
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_sort_by_status() {
        let reg = vec![
            RegistryItem {
                name: "Archive1".into(),
                id: "a1".into(),
                category: "component".into(),
                status: "archives".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Pinned1".into(),
                id: "p1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "Dev1".into(),
                id: "d1".into(),
                category: "component".into(),
                status: "in-development".into(),
                line_count: 30,
            },
        ];
        let sorted = sort_registry_by_status(&reg);
        assert_eq!(sorted[0].status, "pinned");
        assert_eq!(sorted[1].status, "in-development");
        assert_eq!(sorted[2].status, "archives");
    }

    #[test]
    fn test_sort_unknown_status() {
        let reg = vec![
            RegistryItem {
                name: "Unknown".into(),
                id: "u1".into(),
                category: "component".into(),
                status: "weird".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Pinned".into(),
                id: "p1".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];
        let sorted = sort_registry_by_status(&reg);
        assert_eq!(sorted[0].status, "pinned");
        assert_eq!(sorted[1].status, "weird");
    }
}
