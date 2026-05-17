pub fn get_disk_usage() -> String {
    "C: 50GB free / 200GB total".to_string()
}

pub fn get_mount_points() -> Vec<String> {
    vec!["/".to_string(), "/home".to_string(), "/mnt/data".to_string()]
}
