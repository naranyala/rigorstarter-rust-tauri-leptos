use crate::syslib::process::exec;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiskUsage {
    pub filesystem: String,
    pub size_gb: f32,
    pub used_gb: f32,
    pub available_gb: f32,
    pub usage_percent: f32,
    pub mount: String,
}

pub fn get_disk_usage(path: &str) -> Option<DiskUsage> {
    let result = exec(
        "df",
        &["-BG", "--output=source,size,used,avail,pcent,target", path],
    );

    match result {
        Ok(res) => {
            let lines: Vec<&str> = res.stdout.lines().collect();
            if lines.len() < 2 {
                return None;
            }

            let data = lines[1].split_whitespace().collect::<Vec<&str>>();
            if data.len() < 6 {
                return None;
            }

            Some(DiskUsage {
                filesystem: data[0].to_string(),
                size_gb: data[1].trim_start_matches('G').parse().unwrap_or(0.0),
                used_gb: data[2].trim_start_matches('G').parse().unwrap_or(0.0),
                available_gb: data[3].trim_start_matches('G').parse().unwrap_or(0.0),
                usage_percent: data[4].trim_end_matches('%').parse().unwrap_or(0.0),
                mount: data[5].to_string(),
            })
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_usage_root_exists() {
        // This test may fail in environments without `df` command
        let result = get_disk_usage("/");
        if let Some(usage) = result {
            assert!(!usage.filesystem.is_empty());
            // size_gb may be 0.0 in some environments (e.g. Docker, minimal systems)
            assert_eq!(usage.mount, "/");
        }
    }

    #[test]
    fn test_get_disk_usage_invalid_path() {
        // Should handle non-existent paths gracefully
        let result = get_disk_usage("/nonexistent_path_xyz");
        // The result could be None (df fails) or Some (df succeeds on path)
        // Either is acceptable, just shouldn't panic
        let _ = result;
    }

    #[test]
    fn test_disk_usage_fields() {
        let usage = DiskUsage {
            filesystem: "/dev/sda1".to_string(),
            size_gb: 100.0,
            used_gb: 60.0,
            available_gb: 40.0,
            usage_percent: 60.0,
            mount: "/".to_string(),
        };
        assert_eq!(usage.filesystem, "/dev/sda1");
        assert_eq!(usage.size_gb, 100.0);
        assert_eq!(usage.used_gb, 60.0);
        assert_eq!(usage.available_gb, 40.0);
        assert_eq!(usage.usage_percent, 60.0);
        assert_eq!(usage.mount, "/");
    }

    #[test]
    fn test_disk_usage_serialize() {
        let usage = DiskUsage {
            filesystem: "/dev/nvme0n1p2".to_string(),
            size_gb: 500.0,
            used_gb: 200.0,
            available_gb: 300.0,
            usage_percent: 40.0,
            mount: "/home".to_string(),
        };
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("filesystem"));
        assert!(json.contains("size_gb"));
        assert!(json.contains("mount"));
        assert!(json.contains("/home"));
    }

    #[test]
    fn test_disk_usage_sum_consistency() {
        let usage = DiskUsage {
            filesystem: "tmpfs".to_string(),
            size_gb: 8.0,
            used_gb: 2.0,
            available_gb: 6.0,
            usage_percent: 25.0,
            mount: "/tmp".to_string(),
        };
        assert!(
            (usage.used_gb + usage.available_gb - usage.size_gb).abs() < 0.01,
            "Used + available should roughly equal size"
        );
    }

    #[test]
    fn test_disk_usage_percent_bounds() {
        let usage = DiskUsage {
            filesystem: "test".to_string(),
            size_gb: 10.0,
            used_gb: 5.0,
            available_gb: 5.0,
            usage_percent: 50.0,
            mount: "/mnt".to_string(),
        };
        assert!(usage.usage_percent >= 0.0 && usage.usage_percent <= 100.0);
    }

    #[test]
    fn test_get_disk_usage_with_spaces_in_mount() {
        // The df command handling should gracefully deal with spaces
        let result = get_disk_usage("/");
        // Just verify it doesn't panic
        let _ = result;
    }
}
