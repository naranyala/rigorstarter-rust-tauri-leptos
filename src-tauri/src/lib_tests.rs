#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_file_line_count_exists() {
        // Test with a known existing file (e.g., lib.rs itself)
        let count = get_file_line_count("src-tauri/src/lib.rs".to_string());
        assert!(count.is_ok());
        assert!(count.unwrap() > 0);
    }

    #[test]
    fn test_get_file_line_count_missing() {
        let count = get_file_line_count("non_existent_file.rs".to_string());
        assert!(count.is_err());
    }

    #[test]
    fn test_get_utility_source_valid() {
        // Assuming network.rs exists
        let result = get_utility_source("network");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_utility_source_path_traversal() {
        let bad_inputs = vec!["../main", "etc/passwd", "utils/../../Cargo.toml", "C:/Windows/system32"];
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
        for item in registry {
            // Ensure each item can actually be opened
            if item.category == "utility" {
                assert!(get_utility_source(&item.id).is_ok(), "Utility {} should exist on disk", item.id);
            }
        }
    }
}
