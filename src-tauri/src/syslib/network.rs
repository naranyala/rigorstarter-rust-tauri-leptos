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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ips_returns_vec() {
        let result = get_local_ips();
        // The result may be empty if 'ip' command is not available (e.g., containers)
        // but it should never panic
        assert!(result.is_empty() || result.iter().all(|iface| !iface.name.is_empty()));
    }

    #[test]
    fn test_network_interface_fields() {
        let iface = NetworkInterface {
            name: "eth0".to_string(),
            ip: Some("192.168.1.1".parse().unwrap()),
            status: "up".to_string(),
        };
        assert_eq!(iface.name, "eth0");
        assert!(iface.ip.is_some());
        assert_eq!(iface.status, "up");
    }

    #[test]
    fn test_network_interface_no_ip() {
        let iface = NetworkInterface {
            name: "lo".to_string(),
            ip: None,
            status: "down".to_string(),
        };
        assert!(iface.ip.is_none());
    }

    #[test]
    fn test_network_interface_serialize() {
        let iface = NetworkInterface {
            name: "wlan0".to_string(),
            ip: Some("10.0.0.1".parse().unwrap()),
            status: "up".to_string(),
        };
        let json = serde_json::to_string(&iface).unwrap();
        assert!(json.contains("wlan0"));
        assert!(json.contains("10.0.0.1"));
        assert!(json.contains("up"));
    }

    #[test]
    fn test_network_interface_ipv6() {
        let iface = NetworkInterface {
            name: "eth0".to_string(),
            ip: Some("fe80::1".parse().unwrap()),
            status: "up".to_string(),
        };
        assert_eq!(iface.ip.unwrap().to_string(), "fe80::1");
    }
}
