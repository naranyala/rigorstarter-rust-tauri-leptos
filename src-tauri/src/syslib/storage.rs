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
