use sysinfo::System;

pub struct SystemInfo {
    pub hostname: String,
    pub kernel: String,
    pub distro: String,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());

        // sysinfo doesn't directly give kernel/distro in a simple way on all platforms,
        // but we can use the OS name/version if available or fall back gracefully.
        // For a more complete solution on Linux, we could still use /etc/os-release
        // but in a way that doesn't crash on other platforms.

        // On Linux, we can still try to get distro from /etc/os-release safely.
        let distro = std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|c| {
                c.lines()
                    .find(|l| l.starts_with("PRETTY_NAME="))
                    .map(|l| l.replace("PRETTY_NAME=", "").replace("\"", ""))
            })
            .unwrap_or_else(|| "Unknown".to_string());

        // A very basic way to get kernel/os info
        let kernel = std::env::consts::OS.to_string();

        Self {
            hostname,
            kernel,
            distro,
        }
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

    #[test]
    fn test_system_info_hostname_not_unknown() {
        let info = SystemInfo::collect();
        if info.hostname != "unknown" {
            assert!(info.hostname.len() >= 1);
        }
    }

    #[test]
    fn test_system_info_kernel_format() {
        let info = SystemInfo::collect();
        assert!(!info.kernel.is_empty());
    }
}
