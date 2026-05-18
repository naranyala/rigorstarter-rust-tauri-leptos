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
        log_message("test message".to_string());
        log_message(String::new());
        log_message("special chars: @!#$%^&*()".to_string());
    }

    // --- UNICODE EDGE CASES ---

    #[test]
    fn test_greet_unicode() {
        let res = greet("日本語");
        assert!(res.is_ok(), "Japanese should be accepted");
        assert!(res.unwrap().contains("日本語"));

        let res = greet("😊🎉");
        assert!(res.is_ok(), "Emoji should be accepted");

        let res = greet("\u{200D}"); // zero-width joiner
                                     // ZWJ is not whitespace, so it passes through
        if let Ok(msg) = &res {
            assert!(msg.contains("\u{200D}"));
        }

        let res = greet("Löwe 老虎 Räder");
        assert!(res.is_ok(), "Mixed unicode should be accepted");

        let res = greet("\u{202E}"); // RTL override (not whitespace)
        if let Ok(msg) = &res {
            assert!(msg.contains("\u{202E}"));
        }
    }

    #[test]
    fn test_get_utility_source_unicode_name() {
        let res = get_utility_source("ネットワーク");
        assert!(res.is_err(), "Unicode utility name should not exist");
    }

    #[test]
    fn test_get_file_line_count_unicode_path() {
        let result = get_file_line_count("αβγ/不存在.rs".to_string());
        assert!(result.is_err(), "Unicode non-existent path should error");
    }

    // --- INJECTION / MALICIOUS INPUT ---

    #[test]
    fn test_greet_null_byte() {
        let res = greet("Alice\0Bob");
        assert!(res.is_ok(), "Null byte should be treated as part of string");
    }

    #[test]
    fn test_greet_very_long_name() {
        let long = "A".repeat(100_000);
        let res = greet(&long);
        assert!(res.is_ok(), "Very long name should be accepted");
        let msg = res.unwrap();
        assert!(msg.len() > 100_000);
    }

    #[test]
    fn test_greet_html_injection() {
        let res = greet("<script>alert('xss')</script>");
        assert!(res.is_ok());
        // The response should include the name as-is (no sanitization is expected)
        assert!(res.unwrap().contains("<script>"));
    }

    #[test]
    fn test_get_utility_source_all_whitespace() {
        let res = get_utility_source("   ");
        assert!(
            res.is_err(),
            "Whitespace-only name should be rejected or not found"
        );
    }

    #[test]
    fn test_get_utility_source_very_long_name() {
        // Path construction with very long string should not panic
        let long = "a".repeat(10_000);
        let res = get_utility_source(&long);
        assert!(res.is_err(), "Very long name should not exist");
    }

    // --- REGISTRY EDGE CASES ---

    #[test]
    fn test_get_registry_ids_are_unique() {
        let registry = get_registry().unwrap();
        let mut ids = std::collections::HashSet::new();
        for item in &registry {
            assert!(ids.insert(&item.id), "Duplicate id found: {}", item.id);
        }
    }

    #[test]
    fn test_get_registry_categories() {
        let registry = get_registry().unwrap();
        let categories: std::collections::HashSet<&str> =
            registry.iter().map(|i| i.category.as_str()).collect();
        for cat in &categories {
            assert!(
                *cat == "component" || *cat == "utility",
                "Unexpected category: {}",
                cat
            );
        }
    }

    // --- LINE COUNT BOUNDARY ---

    #[test]
    fn test_get_file_line_count_symlink_not_followed() {
        // Just verify that non-existent symlink path errors
        let result = get_file_line_count("/nonexistent_link.rs".to_string());
        assert!(result.is_err());
    }

    // --- SYSTEM INFO EDGE CASES ---

    #[test]
    fn test_get_system_info_paths_not_empty() {
        let res = get_system_info().unwrap();
        let config = res["paths"]["config"].as_str().unwrap_or("");
        let data = res["paths"]["data"].as_str().unwrap_or("");
        let cache = res["paths"]["cache"].as_str().unwrap_or("");
        assert!(!config.is_empty(), "Config path should not be empty");
        assert!(!data.is_empty(), "Data path should not be empty");
        assert!(!cache.is_empty(), "Cache path should not be empty");
    }

    #[test]
    fn test_get_system_status_all_fields_present() {
        let res = get_system_status().unwrap();
        assert!(res["system"]["hostname"].as_str().unwrap_or("").len() > 0);
        assert!(res["system"]["kernel"].as_str().unwrap_or("").len() > 0);
        assert!(res["session"]["desktop_env"].as_str().unwrap_or("").len() > 0);
    }

    // --- CONCURRENCY SAFETY (lightweight) ---

    #[test]
    fn test_greet_concurrent_calls() {
        let mut handles = Vec::new();
        for i in 0..10 {
            handles.push(std::thread::spawn(move || {
                let name = format!("Thread{}", i);
                greet(&name)
            }));
        }
        for (i, handle) in handles.into_iter().enumerate() {
            let res = handle.join().unwrap();
            assert!(res.is_ok(), "Thread {} should succeed", i);
            assert!(res.unwrap().contains(&format!("Thread{}", i)));
        }
    }
}
