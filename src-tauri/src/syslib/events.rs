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
