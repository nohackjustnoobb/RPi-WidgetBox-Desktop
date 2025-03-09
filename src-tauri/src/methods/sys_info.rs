use serde::Serialize;
use sys_info::{
    boottime, cpu_num, cpu_speed, disk_info, hostname, loadavg, mem_info, os_release, os_type,
    proc_total,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    boot_time: Option<u64>,
    cpu_num: Option<u32>,
    cpu_speed: Option<u64>,
    disk_total: Option<u64>,
    disk_free: Option<u64>,
    memory_total: Option<u64>,
    memory_free: Option<u64>,
    hostname: Option<String>,
    avg_load: Option<f64>,
    os_release: Option<String>,
    os_type: Option<String>,
    proc_total: Option<u64>,
}

#[tauri::command]
pub fn get_sys_info() -> SystemInfo {
    let disk_info = disk_info().ok();
    let mem_info = mem_info().ok();

    return SystemInfo {
        boot_time: boottime().ok().map(|t| t.tv_sec as u64),
        cpu_num: cpu_num().ok(),
        cpu_speed: cpu_speed().ok(),
        disk_total: disk_info.as_ref().map(|i| i.total),
        disk_free: disk_info.map(|i| i.free),
        memory_total: mem_info.as_ref().map(|i| i.total),
        memory_free: mem_info.map(|i| i.free),
        hostname: hostname().ok(),
        avg_load: loadavg().ok().map(|i| i.one),
        os_release: os_release().ok(),
        os_type: os_type().ok(),
        proc_total: proc_total().ok(),
    };
}
