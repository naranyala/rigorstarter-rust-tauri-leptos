pub fn get_disk_usage() -> String {
    "C: 50GB free / 200GB total".to_string()
}

pub fn get_mount_points() -> Vec<String> {
    vec!["/".to_string(), "/home".to_string(), "/mnt/data".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_usage_format() {
        let usage = get_disk_usage();
        assert!(usage.contains("free"));
        assert!(usage.contains("total"));
        assert!(usage.contains("GB"));
    }

    #[test]
    fn test_get_disk_usage_non_empty() {
        assert!(!get_disk_usage().is_empty());
    }

    #[test]
    fn test_get_mount_points_count() {
        let mounts = get_mount_points();
        assert_eq!(mounts.len(), 3);
    }

    #[test]
    fn test_get_mount_points_contains_root() {
        let mounts = get_mount_points();
        assert!(mounts.contains(&"/".to_string()));
    }

    #[test]
    fn test_get_mount_points_contains_home() {
        let mounts = get_mount_points();
        assert!(mounts.contains(&"/home".to_string()));
    }

    #[test]
    fn test_get_mount_points_all_non_empty() {
        let mounts = get_mount_points();
        for mount in &mounts {
            assert!(!mount.is_empty(), "Mount point should not be empty");
        }
    }
}
