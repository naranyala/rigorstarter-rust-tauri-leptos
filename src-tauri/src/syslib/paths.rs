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

    #[test]
    fn test_xdg_paths_defaults() {
        // Clear env to test defaults
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_CACHE_HOME");

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
}
