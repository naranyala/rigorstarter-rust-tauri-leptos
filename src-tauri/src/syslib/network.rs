use crate::syslib::process::exec;
use serde::Serialize;
use std::net::IpAddr;

#[derive(Debug, Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub status: String,
}

pub fn get_local_ips() -> Vec<NetworkInterface> {
    let result = exec("ip", &["-o", "addr", "show"]);

    match result {
        Ok(res) => res
            .stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    return None;
                }

                let name = parts[1].trim_end_matches(':').to_string();
                let ip_part = parts[3].split('/').next()?;
                let ip = ip_part.parse::<IpAddr>().ok();

                Some(NetworkInterface {
                    name,
                    ip,
                    status: "up".to_string(),
                })
            })
            .collect(),
        Err(_) => vec![],
    }
}
