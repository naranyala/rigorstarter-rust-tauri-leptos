use notify::Config;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch_config_file<F>(path: &Path, callback: F) -> notify::Result<RecommendedWatcher>
where
    F: Fn(Event) + Send + 'static,
{
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    watcher.watch(path, RecursiveMode::NonRecursive)?;

    std::thread::spawn(move || {
        for event in rx {
            callback(event);
        }
    });

    Ok(watcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_watch_config_file_returns_watcher() {
        let dir = std::env::temp_dir().join(format!("rigorstater_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let test_file = dir.join("test_config.toml");
        fs::write(&test_file, "key = value\n").unwrap();

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        let callback = move |event: Event| {
            events_clone.lock().unwrap().push(event);
        };

        let result = watch_config_file(&test_file, callback);
        assert!(result.is_ok());

        // Give the watcher a moment to initialize
        std::thread::sleep(Duration::from_millis(100));

        // Modify the file to trigger an event
        fs::write(&test_file, "key = modified\n").unwrap();
        std::thread::sleep(Duration::from_millis(500));

        // Drop the watcher to stop it
        drop(result);

        // Cleanup
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_watch_config_file_invalid_path() {
        let result = watch_config_file(
            Path::new("/nonexistent/path/that/does/not/exist/config.toml"),
            |_event: Event| {},
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_watch_config_file_callback_type() {
        // Verify the function accepts different callback types
        fn static_callback(_event: Event) {}
        let fn_ptr: fn(Event) = static_callback;
        let dir = std::env::temp_dir().join(format!("rigorstater_cb_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let test_file = dir.join("cb_test.toml");
        fs::write(&test_file, "data\n").unwrap();
        let result = watch_config_file(&test_file, fn_ptr);
        assert!(result.is_ok());
        drop(result);
        let _ = fs::remove_dir_all(&dir);
    }
}
