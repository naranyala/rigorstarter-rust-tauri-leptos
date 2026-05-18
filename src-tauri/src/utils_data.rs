pub fn get_utility_source(utility: &str) -> Option<&'static str> {
    match utility {
        "network" => Some(include_str!("utils/network.rs")),
        "system" => Some(include_str!("utils/system.rs")),
        "storage" => Some(include_str!("utils/storage.rs")),
        "process" => Some(include_str!("utils/process.rs")),
        "disk_usage" => Some(include_str!("utils/disk_usage.rs")),
        "env_vars" => Some(include_str!("utils/env_vars.rs")),
        _ => None,
    }
}

pub fn get_utility_line_count(utility: &str) -> usize {
    get_utility_source(utility)
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_utility_source_known() {
        let source = get_utility_source("network");
        assert!(source.is_some());
        assert!(source.unwrap().contains("pub fn"));
    }

    #[test]
    fn test_get_utility_source_unknown() {
        let source = get_utility_source("non_existent");
        assert!(source.is_none());
    }

    #[test]
    fn test_get_utility_line_count_known() {
        let count = get_utility_line_count("network");
        assert!(count > 0);
    }

    #[test]
    fn test_get_utility_line_count_unknown() {
        let count = get_utility_line_count("non_existent");
        assert_eq!(count, 0);
    }
}
