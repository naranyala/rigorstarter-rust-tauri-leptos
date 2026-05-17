use std::fs;

pub struct SystemInfo {
    pub hostname: String,
    pub kernel: String,
    pub distro: String,
}

impl SystemInfo {
    pub fn collect() -> Self {
        Self {
            hostname: Self::read_file("/proc/sys/kernel/hostname")
                .unwrap_or_else(|| "unknown".to_string()),
            kernel: Self::read_file("/proc/sys/kernel/osrelease")
                .unwrap_or_else(|| "unknown".to_string()),
            distro: Self::read_file("/etc/os-release")
                .map(|c| {
                    c.lines()
                        .find(|l| l.starts_with("PRETTY_NAME="))
                        .map(|l| l.replace("PRETTY_NAME=", "").replace("\"", ""))
                        .unwrap_or_else(|| "Linux".to_string())
                })
                .unwrap_or_else(|| "Linux".to_string()),
        }
    }

    fn read_file(path: &str) -> Option<String> {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_collect() {
        let info = SystemInfo::collect();
        assert!(!info.hostname.is_empty());
        assert!(!info.kernel.is_empty());
        assert!(!info.distro.is_empty());
    }
}
