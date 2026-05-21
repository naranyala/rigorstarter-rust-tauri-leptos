use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct AppFileSystem;

impl AppFileSystem {
    pub fn new() -> Self {
        Self
    }

    pub async fn list_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(path)? {
            files.push(entry?.path());
        }
        Ok(files)
    }

    pub async fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    pub async fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    pub async fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    pub async fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    pub async fn write_file(&self, path: &Path, content: &[u8]) -> io::Result<()> {
        std::fs::write(path, content)
    }

    pub async fn get_metadata(&self, path: &Path) -> io::Result<std::fs::Metadata> {
        std::fs::metadata(path)
    }

    pub fn get_app_data_dir(&self, app_name: &str) -> PathBuf {
        dirs::data_dir()
            .map(|p| p.join(app_name))
            .unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            })
    }

    pub fn normalize_path(&self, path: &str) -> Option<PathBuf> {
        std::fs::canonicalize(path).ok()
    }

    pub fn last_path_file() -> String {
        dirs::config_dir()
            .map(|p| {
                p.join("whispercpp-gui/last_path.txt")
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or_else(|| "last_path.txt".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_system() {
        let fs = AppFileSystem::new();
        let temp_dir = std::env::temp_dir().join("fs_test");
        let _ = std::fs::create_dir_all(&temp_dir);

        let file_path = temp_dir.join("test.txt");
        let content = b"hello world".to_vec();

        fs.write_file(&file_path, &content).await.unwrap();
        assert!(fs.exists(&file_path).await);
        assert_eq!(fs.read_file(&file_path).await.unwrap(), content);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
