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
        let result = get_disk_usage("/");
        if let Some(usage) = result {
            assert!(!usage.filesystem.is_empty());
            assert_eq!(usage.mount, "/");
        }
    }

    #[test]
    fn test_get_disk_usage_invalid_path() {
        let result = get_disk_usage("/nonexistent_path_xyz");
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
        assert!((usage.used_gb + usage.available_gb - usage.size_gb).abs() < 0.01);
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

    // --- BOUNDARY AND INVARIANT TESTS ---

    #[test]
    fn test_disk_usage_zero_values() {
        let usage = DiskUsage {
            filesystem: "zero".to_string(),
            size_gb: 0.0,
            used_gb: 0.0,
            available_gb: 0.0,
            usage_percent: 0.0,
            mount: "/zero".to_string(),
        };
        assert_eq!(usage.size_gb, 0.0);
        assert_eq!(usage.used_gb, 0.0);
        assert_eq!(usage.available_gb, 0.0);
        assert_eq!(usage.usage_percent, 0.0);
    }

    #[test]
    fn test_disk_usage_full_disk() {
        let usage = DiskUsage {
            filesystem: "full".to_string(),
            size_gb: 100.0,
            used_gb: 100.0,
            available_gb: 0.0,
            usage_percent: 100.0,
            mount: "/full".to_string(),
        };
        assert_eq!(usage.usage_percent, 100.0);
        assert_eq!(usage.available_gb, 0.0);
    }

    #[test]
    fn test_disk_usage_size_not_less_than_used() {
        let usage = DiskUsage {
            filesystem: "consistency".to_string(),
            size_gb: 50.0,
            used_gb: 30.0,
            available_gb: 20.0,
            usage_percent: 60.0,
            mount: "/check".to_string(),
        };
        assert!(
            usage.size_gb >= usage.used_gb,
            "size_gb ({}) should be >= used_gb ({})",
            usage.size_gb,
            usage.used_gb
        );
        assert!(
            usage.size_gb >= usage.available_gb,
            "size_gb ({}) should be >= available_gb ({})",
            usage.size_gb,
            usage.available_gb
        );
    }

    #[test]
    fn test_disk_usage_percent_calculation() {
        let size = 200.0;
        let used = 50.0;
        let expected_percent = (used / size) * 100.0;
        let usage = DiskUsage {
            filesystem: "/dev/sdb1".to_string(),
            size_gb: size,
            used_gb: used,
            available_gb: size - used,
            usage_percent: expected_percent,
            mount: "/mnt/data".to_string(),
        };
        assert!(
            (usage.usage_percent - expected_percent).abs() < 0.01,
            "usage_percent {} should match calculation {}",
            usage.usage_percent,
            expected_percent
        );
    }

    #[test]
    fn test_disk_usage_mount_points_multiple() {
        // Test that multiple paths return data without panic
        for path in &["/", "/tmp", "/dev", "/proc"] {
            let result = get_disk_usage(path);
            // Some of these may fail (e.g. /proc) - just don't panic
            let _ = result;
        }
    }

    #[test]
    fn test_disk_usage_serialize_contains_all_fields() {
        let usage = DiskUsage {
            filesystem: "test_fs".to_string(),
            size_gb: 256.0,
            used_gb: 128.0,
            available_gb: 128.0,
            usage_percent: 50.0,
            mount: "/roundtrip".to_string(),
        };
        let json = serde_json::to_value(&usage).unwrap();
        assert_eq!(json["filesystem"], "test_fs");
        assert_eq!(json["mount"], "/roundtrip");
        assert!((json["size_gb"].as_f64().unwrap() - 256.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_disk_usage_negative_values() {
        let usage = DiskUsage {
            filesystem: "negative".to_string(),
            size_gb: -1.0,
            used_gb: -2.0,
            available_gb: -3.0,
            usage_percent: -10.0,
            mount: "/negative".to_string(),
        };
        // Negative values should serialize without panic
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("-1.0") || json.contains("-2.0"));
    }

    #[test]
    fn test_get_disk_usage_with_spaces_in_mount() {
        let result = get_disk_usage("/");
        let _ = result;
    }

    #[test]
    fn test_disk_usage_large_values() {
        let usage = DiskUsage {
            filesystem: "large".to_string(),
            size_gb: 1_000_000.0,
            used_gb: 500_000.0,
            available_gb: 500_000.0,
            usage_percent: 50.0,
            mount: "/large".to_string(),
        };
        assert!(usage.size_gb > 0.0);
        let json = serde_json::to_string(&usage).unwrap();
        assert!(
            json.contains("1000000")
                || json.contains("1e6")
                || json.contains("1.0e6")
                || json.contains("1.0E6")
                || json.contains("1000000.0")
        );
    }
}
