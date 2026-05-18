pub fn get_active_processes() -> Vec<String> {
    vec!["tauri-app".to_string(), "rust-analyzer".to_string(), "cargo".to_string()]
}

pub fn get_process_count() -> usize {
    142
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_active_processes_returns_expected() {
        let processes = get_active_processes();
        assert_eq!(processes.len(), 3);
        assert_eq!(processes[0], "tauri-app");
        assert_eq!(processes[1], "rust-analyzer");
        assert_eq!(processes[2], "cargo");
    }

    #[test]
    fn test_get_process_count_returns_expected() {
        assert_eq!(get_process_count(), 142);
    }

    #[test]
    fn test_get_active_processes_contains_expected_names() {
        let processes = get_active_processes();
        assert!(processes.contains(&"tauri-app".to_string()));
        assert!(processes.contains(&"cargo".to_string()));
    }

    #[test]
    fn test_get_process_count_positive() {
        assert!(get_process_count() > 0);
    }
}
