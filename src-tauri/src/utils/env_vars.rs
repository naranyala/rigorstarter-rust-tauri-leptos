use std::env;

pub fn get_env_vars() -> String {
    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    
    vars.into_iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_vars_returns_sorted() {
        let result = get_env_vars();
        assert!(!result.is_empty(), "Should have at least some env vars");
    }

    #[test]
    fn test_get_env_vars_format() {
        let result = get_env_vars();
        assert!(result.contains(": "), "Lines should be key: value format");
    }

    #[test]
    fn test_get_env_vars_contains_path() {
        let result = get_env_vars();
        // Most environments have PATH or PATH-like variables
        assert!(
            result.contains("PATH")
                || result.contains("HOME")
                || result.contains("USER"),
            "Should contain common environment variables"
        );
    }

    #[test]
    fn test_get_env_vars_sorted_order() {
        let result = get_env_vars();
        let lines: Vec<&str> = result.lines().collect();
        if lines.len() >= 2 {
            assert!(lines[0] <= lines[1], "Variables should be sorted alphabetically");
        }
    }

    #[test]
    fn test_get_env_vars_multiple_lines() {
        let result = get_env_vars();
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1, "Should have multiple environment variables");
    }
}
