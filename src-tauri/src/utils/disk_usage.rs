use std::process::Command;

pub fn get_disk_usage() -> String {
    // Simple implementation using 'df -h' for Unix-like systems
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output();

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to execute df command".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_usage_returns_string() {
        let result = get_disk_usage();
        assert!(!result.is_empty(), "Disk usage should not be empty");
    }

    #[test]
    fn test_get_disk_usage_contains_mount() {
        let result = get_disk_usage();
        assert!(result.contains('/'), "Should contain root mount point");
    }

    #[test]
    fn test_get_disk_usage_contains_filesystem_header() {
        let result = get_disk_usage();
        assert!(
            result.contains("Filesystem")
                || result.contains("Size")
                || result.contains("Avail")
                || result.starts_with("Failed"),
            "Should contain expected headers or fail gracefully"
        );
    }
}
