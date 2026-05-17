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
