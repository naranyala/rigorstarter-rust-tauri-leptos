use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_percent: f32,
}

#[derive(Debug, Serialize)]
pub struct CpuInfo {
    pub global_usage: f32,
    pub per_core_usage: Vec<f32>,
    pub load_avg: (f32, f32, f32),
}

pub fn get_system_metrics() -> (MemoryInfo, CpuInfo) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_mem = sys.total_memory();
    let available_mem = sys.available_memory();

    let mem_info = MemoryInfo {
        total_kb: total_mem,
        available_kb: available_mem,
        used_percent: if total_mem > 0 {
            ((total_mem - available_mem) as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        },
    };

    let load = System::load_average();

    let cpu_info = CpuInfo {
        global_usage: sys.global_cpu_usage(),
        per_core_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        load_avg: (load.one as f32, load.five as f32, load.fifteen as f32),
    };

    (mem_info, cpu_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_metrics_returns_data() {
        let (mem, cpu) = get_system_metrics();
        assert!(mem.total_kb > 0, "Total memory should be > 0");
        assert!(
            mem.available_kb > 0 || mem.total_kb > 0,
            "Available or total memory should be > 0"
        );
        assert!(
            !cpu.per_core_usage.is_empty(),
            "Should have at least one CPU core"
        );
    }

    #[test]
    fn test_memory_used_percent_in_range() {
        let (mem, _) = get_system_metrics();
        assert!(
            mem.used_percent >= 0.0 && mem.used_percent <= 100.0,
            "Used memory percent should be between 0 and 100, got {}",
            mem.used_percent
        );
    }

    #[test]
    fn test_memory_available_less_than_total() {
        let (mem, _) = get_system_metrics();
        assert!(
            mem.available_kb <= mem.total_kb || mem.total_kb == 0,
            "Available memory should not exceed total memory"
        );
    }

    #[test]
    fn test_cpu_global_usage_in_range() {
        let (_, cpu) = get_system_metrics();
        assert!(
            cpu.global_usage >= 0.0 && cpu.global_usage <= 100.0,
            "Global CPU usage should be between 0 and 100, got {}",
            cpu.global_usage
        );
    }

    #[test]
    fn test_cpu_per_core_usage_in_range() {
        let (_, cpu) = get_system_metrics();
        for usage in &cpu.per_core_usage {
            assert!(
                *usage >= 0.0 && *usage <= 100.0,
                "Per-core CPU usage should be between 0 and 100, got {}",
                usage
            );
        }
    }

    #[test]
    fn test_cpu_load_avg_non_negative() {
        let (_, cpu) = get_system_metrics();
        assert!(cpu.load_avg.0 >= 0.0, "1-min load avg should be >= 0");
        assert!(cpu.load_avg.1 >= 0.0, "5-min load avg should be >= 0");
        assert!(cpu.load_avg.2 >= 0.0, "15-min load avg should be >= 0");
    }

    #[test]
    fn test_memory_info_serialize() {
        let mem = MemoryInfo {
            total_kb: 16_000_000,
            available_kb: 8_000_000,
            used_percent: 50.0,
        };
        let json = serde_json::to_string(&mem).unwrap();
        assert!(json.contains("total_kb"));
        assert!(json.contains("available_kb"));
        assert!(json.contains("used_percent"));
    }

    #[test]
    fn test_cpu_info_serialize() {
        let cpu = CpuInfo {
            global_usage: 25.0,
            per_core_usage: vec![20.0, 30.0],
            load_avg: (1.5, 1.0, 0.8),
        };
        let json = serde_json::to_string(&cpu).unwrap();
        assert!(json.contains("global_usage"));
        assert!(json.contains("per_core_usage"));
        assert!(json.contains("load_avg"));
    }
}
