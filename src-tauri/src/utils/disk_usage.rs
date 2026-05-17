use std::process::Command;

pub fn get_disk_usage() -> String {
    // Simple implementation using 'df -h' for Unix-like systems
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output();

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to execute df command".to_string(),
    }
}
