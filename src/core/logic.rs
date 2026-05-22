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

#[allow(dead_code)]
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
    fn test_filter_whitespace_query() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "   ", "component");
        // "   ".to_lowercase().contains("") is true.
        // Current implementation treats whitespace as a search term.
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_filter_extremely_long_query() {
        let reg = mock_registry();
        let res = filter_registry(&reg, &"a".repeat(1000), "component");
        assert!(res.is_empty());
    }

    #[test]
    fn test_filter_large_dataset_performance() {
        let mut reg = Vec::new();
        for i in 0..10000 {
            reg.push(RegistryItem {
                name: format!("Item {}", i),
                id: format!("id_{}", i),
                category: "component".into(),
                status: "pinned".into(),
                line_count: i,
            });
        }
        let res = filter_registry(&reg, "Item 9999", "component");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, "Item 9999");
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
    fn test_filter_empty_category() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "Acc", "");
        assert!(res.is_empty());
    }

    #[test]
    fn test_filter_category_no_query() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "", "component");
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_filter_partial_match() {
        let reg = mock_registry();
        let res = filter_registry(&reg, "ordion", "component");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, "Accordion");
    }

    #[test]
    fn test_filter_special_characters() {
        let reg = vec![RegistryItem {
            name: "Component (Special) @!".into(),
            id: "spec".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        }];
        let res = filter_registry(&reg, "@!", "component");
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_filter_numeric_query() {
        let reg = vec![RegistryItem {
            name: "Component 123".into(),
            id: "num".into(),
            category: "component".into(),
            status: "pinned".into(),
            line_count: 10,
        }];
        let res = filter_registry(&reg, "123", "component");
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_sort_single_item() {
        let reg = vec![RegistryItem {
            name: "One".into(),
            id: "o1".into(),
            category: "component".into(),
            status: "archives".into(),
            line_count: 10,
        }];
        let sorted = sort_registry_by_status(&reg);
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].status, "archives");
    }

    #[test]
    fn test_sort_unsorted_input() {
        let reg = vec![
            RegistryItem {
                name: "A".into(),
                id: "a".into(),
                category: "component".into(),
                status: "archives".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "B".into(),
                id: "b".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "C".into(),
                id: "c".into(),
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
    fn test_sort_reversed_input() {
        let reg = vec![
            RegistryItem {
                name: "A".into(),
                id: "a".into(),
                category: "component".into(),
                status: "archives".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "B".into(),
                id: "b".into(),
                category: "component".into(),
                status: "in-development".into(),
                line_count: 20,
            },
            RegistryItem {
                name: "C".into(),
                id: "c".into(),
                category: "component".into(),
                status: "pinned".into(),
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
                id: "u".into(),
                category: "component".into(),
                status: "mystery".into(),
                line_count: 10,
            },
            RegistryItem {
                name: "Pinned".into(),
                id: "p".into(),
                category: "component".into(),
                status: "pinned".into(),
                line_count: 20,
            },
        ];
        let sorted = sort_registry_by_status(&reg);
        // unknown (3) should be last
        assert_eq!(sorted[0].status, "pinned");
        assert_eq!(sorted[1].status, "mystery");
    }
}
