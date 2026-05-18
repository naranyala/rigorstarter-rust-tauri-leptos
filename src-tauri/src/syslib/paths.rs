use std::env;
use std::path::PathBuf;

pub struct XdgPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub home_dir: PathBuf,
}

impl XdgPaths {
    pub fn new(app_name: &str) -> Self {
        let home = env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/home/user")); // Fallback

        let config_dir = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".config"))
            .join(app_name);

        let data_dir = env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".local/share"))
            .join(app_name);

        let cache_dir = env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".cache"))
            .join(app_name);

        Self {
            config_dir,
            data_dir,
            cache_dir,
            home_dir: home,
        }
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn clear_xdg_env() {
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_CACHE_HOME");
    }

    #[test]
    fn test_xdg_paths_defaults() {
        clear_xdg_env();
        let paths = XdgPaths::new("test-app");
        assert!(paths
            .config_dir
            .to_string_lossy()
            .contains(".config/test-app"));
        assert!(paths
            .data_dir
            .to_string_lossy()
            .contains(".local/share/test-app"));
        assert!(paths
            .cache_dir
            .to_string_lossy()
            .contains(".cache/test-app"));
    }

    #[test]
    fn test_xdg_paths_custom() {
        env::set_var("XDG_CONFIG_HOME", "/tmp/custom_config");
        let paths = XdgPaths::new("test-app");
        assert_eq!(
            paths.config_dir,
            PathBuf::from("/tmp/custom_config/test-app")
        );
    }

    #[test]
    fn test_xdg_paths_empty_env_vars() {
        clear_xdg_env();
        env::set_var("XDG_CONFIG_HOME", "");
        env::set_var("XDG_DATA_HOME", "");
        env::set_var("XDG_CACHE_HOME", "");
        let paths = XdgPaths::new("app");
        assert!(!paths.config_dir.to_string_lossy().is_empty());
        assert!(!paths.data_dir.to_string_lossy().is_empty());
        assert!(!paths.cache_dir.to_string_lossy().is_empty());
    }

    #[test]
    fn test_xdg_paths_home_fallback() {
        clear_xdg_env();
        let original_home = env::var("HOME").ok();
        env::remove_var("HOME");
        let paths = XdgPaths::new("app");
        assert_eq!(paths.home_dir, PathBuf::from("/home/user"));
        if let Some(h) = original_home {
            env::set_var("HOME", h);
        }
    }

    #[test]
    fn test_xdg_paths_app_name_with_spaces() {
        clear_xdg_env();
        let paths = XdgPaths::new("my test app");
        assert!(paths.config_dir.to_string_lossy().contains("my test app"));
    }

    #[test]
    fn test_xdg_paths_app_name_empty() {
        clear_xdg_env();
        let paths = XdgPaths::new("");
        assert!(paths.config_dir.to_string_lossy().ends_with("/.config/"));
    }

    #[test]
    fn test_ensure_dirs_creates_directories() {
        clear_xdg_env();
        let tmp = env::temp_dir().join(format!("rigor_xdg_test_{}", std::process::id()));
        env::set_var("XDG_CONFIG_HOME", tmp.join("config").to_str().unwrap());
        env::set_var("XDG_DATA_HOME", tmp.join("data").to_str().unwrap());
        env::set_var("XDG_CACHE_HOME", tmp.join("cache").to_str().unwrap());

        let paths = XdgPaths::new("test-app");
        let result = paths.ensure_dirs();
        assert!(result.is_ok(), "ensure_dirs should succeed");

        assert!(paths.config_dir.exists(), "config dir should exist");
        assert!(paths.data_dir.exists(), "data dir should exist");
        assert!(paths.cache_dir.exists(), "cache dir should exist");

        // Cleanup
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_ensure_dirs_idempotent() {
        clear_xdg_env();
        let tmp = env::temp_dir().join(format!("rigor_xdg_idem_{}", std::process::id()));
        env::set_var("XDG_CONFIG_HOME", tmp.join("config").to_str().unwrap());
        env::set_var("XDG_DATA_HOME", tmp.join("data").to_str().unwrap());
        env::set_var("XDG_CACHE_HOME", tmp.join("cache").to_str().unwrap());

        let paths = XdgPaths::new("idem-app");
        assert!(paths.ensure_dirs().is_ok());
        assert!(
            paths.ensure_dirs().is_ok(),
            "Second ensure_dirs should also succeed"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_xdg_paths_all_custom() {
        clear_xdg_env();
        env::set_var("XDG_CONFIG_HOME", "/custom/cfg");
        env::set_var("XDG_DATA_HOME", "/custom/data");
        env::set_var("XDG_CACHE_HOME", "/custom/cache");
        let paths = XdgPaths::new("myapp");
        assert_eq!(paths.config_dir, PathBuf::from("/custom/cfg/myapp"));
        assert_eq!(paths.data_dir, PathBuf::from("/custom/data/myapp"));
        assert_eq!(paths.cache_dir, PathBuf::from("/custom/cache/myapp"));
    }
}
