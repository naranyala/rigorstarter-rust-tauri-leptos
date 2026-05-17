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
}
