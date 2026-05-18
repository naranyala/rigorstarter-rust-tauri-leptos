#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_file_line_count_exists() {
        let paths = vec!["src-tauri/src/lib.rs".to_string(), "src/lib.rs".to_string()];

        let mut found = false;
        for path in paths {
            if get_file_line_count(path).is_ok() {
                found = true;
                break;
            }
        }
        assert!(found, "Should have found lib.rs in one of the common paths");
    }

    #[test]
    fn test_get_file_line_count_missing() {
        let count = get_file_line_count("non_existent_file.rs".to_string());
        assert!(count.is_err());
    }

    #[test]
    fn test_get_utility_source_valid() {
        let result = get_utility_source("network");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_utility_source_path_traversal() {
        let bad_inputs = vec![
            "../main",
            "etc/passwd",
            "utils/../../Cargo.toml",
            "C:/Windows/system32",
        ];
        for input in bad_inputs {
            let result = get_utility_source(input);
            assert!(result.is_err(), "Input {} should have been blocked", input);
        }
    }

    #[test]
    fn test_get_utility_source_missing() {
        let result = get_utility_source("ghost_utility");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_registry_consistency() {
        let registry = get_registry().unwrap();
        assert!(!registry.is_empty());
        for item in registry {
            if item.category == "utility" {
                assert!(
                    get_utility_source(&item.id).is_ok(),
                    "Utility {} should exist on disk",
                    item.id
                );
            }
        }
    }

    #[test]
    fn test_get_registry_line_counts() {
        let registry = get_registry().unwrap();
        for item in registry {
            if item.category == "utility" {
                assert!(
                    item.line_count > 0,
                    "Utility {} should have a line count > 0",
                    item.name
                );
            } else {
                assert_eq!(
                    item.line_count, 0,
                    "Component {} should have line count 0",
                    item.name
                );
            }
        }
    }

    #[test]
    fn test_greet_success() {
        let res = greet("World");
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Hello, World"));
    }

    #[test]
    fn test_greet_empty_name() {
        let res = greet("   ");
        assert!(res.is_err());
    }

    #[test]
    fn test_greet_forbidden_word() {
        let res = greet("error");
        assert!(res.is_err());
    }

    #[test]
    fn test_get_system_info() {
        let res = get_system_info();
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val["hostname"].as_str().is_some());
        assert!(val["distro"].as_str().is_some());
        assert!(val["paths"]["config"].as_str().is_some());
    }

    #[test]
    fn test_get_system_status() {
        let res = get_system_status();
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val["system"].is_object());
        assert!(val["resources"].is_object());
        assert!(val["session"].is_object());
        assert!(val["network"].is_array());
        assert!(val["storage"].is_object() || val["storage"].is_null());
    }

    #[test]
    fn test_get_file_line_count_empty_vs_nonempty() {
        let paths = vec!["src-tauri/src/lib.rs".to_string(), "src/lib.rs".to_string()];
        let mut found = false;
        for path in paths {
            if let Ok(count) = get_file_line_count(path) {
                assert!(count > 0, "lib.rs should have more than 0 lines");
                found = true;
                break;
            }
        }
        assert!(found, "Should have found lib.rs in one of the common paths");
    }

    #[test]
    fn test_get_registry_structure() {
        let registry = get_registry().unwrap();
        assert!(!registry.is_empty());
        for item in &registry {
            assert!(!item.name.is_empty(), "Item name should not be empty");
            assert!(!item.id.is_empty(), "Item id should not be empty");
            assert!(
                !item.category.is_empty(),
                "Item category should not be empty"
            );
            assert!(!item.status.is_empty(), "Item status should not be empty");
        }
    }

    #[test]
    fn test_greet_whitespace_variants() {
        assert!(greet(" ").is_err());
        assert!(greet("\t").is_err());
        assert!(greet("\n").is_err());
        assert!(greet("").is_err());
    }

    #[test]
    fn test_greet_forbidden_word_variants() {
        assert!(greet("error").is_err());
        assert!(greet("ERROR").is_err());
        assert!(greet("Error").is_err());
    }

    #[test]
    fn test_greet_special_characters() {
        let res = greet("Alice & Bob");
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Alice & Bob"));
    }

    #[test]
    fn test_get_utility_source_invalid_variants() {
        let bad = vec!["../secret", "utils/../Cargo.toml", "a/b", "..\\windows"];
        for input in bad {
            assert!(
                get_utility_source(input).is_err(),
                "Should reject: {}",
                input
            );
        }
    }

    #[test]
    fn test_get_utility_source_valid_known() {
        for name in &[
            "network",
            "storage",
            "process",
            "env_vars",
            "disk_usage",
            "system",
        ] {
            assert!(
                get_utility_source(name).is_ok(),
                "Known utility {} should be found",
                name
            );
        }
    }

    #[test]
    fn test_log_message_output() {
        // Just verify it doesn't panic
        log_message("test message".to_string());
        log_message(String::new());
        log_message("special chars: @!#$%^&*()".to_string());
    }
}
