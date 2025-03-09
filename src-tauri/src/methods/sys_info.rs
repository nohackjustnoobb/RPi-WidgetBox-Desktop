use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::Serialize;
use sysinfo::{Components, DiskKind, Disks, Networks, System};

static mut SYS: Lazy<Arc<Mutex<System>>> = Lazy::new(|| Arc::new(Mutex::new(System::new_all())));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MemoryInfo {
    total: u64,
    used: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CPUInfo {
    usage: f32,
    name: String,
    brand: String,
    vendor_id: String,
    frequency: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    boot_time: u64,
    cpus: Vec<CPUInfo>,
    memory: MemoryInfo,
    swap: MemoryInfo,
    host_name: Option<String>,
    os_version: Option<String>,
    long_os_version: Option<String>,
    system_name: Option<String>,
    kernel_version: Option<String>,
    total_proc: u64,
}

#[tauri::command]
pub fn get_sys_info() -> SystemInfo {
    let mut sys = unsafe { SYS.lock().unwrap() };
    sys.refresh_all();

    SystemInfo {
        boot_time: System::boot_time(),
        cpus: sys
            .cpus()
            .iter()
            .map(|c| CPUInfo {
                usage: c.cpu_usage(),
                name: c.name().to_string(),
                brand: c.brand().to_string(),
                vendor_id: c.vendor_id().to_string(),
                frequency: c.frequency(),
            })
            .collect(),
        memory: MemoryInfo {
            total: sys.total_memory(),
            used: sys.used_memory(),
        },
        swap: MemoryInfo {
            total: sys.total_swap(),
            used: sys.used_swap(),
        },
        host_name: System::host_name(),
        system_name: System::name(),
        os_version: System::os_version(),
        long_os_version: System::long_os_version(),
        kernel_version: System::kernel_version(),
        total_proc: sys.processes().len() as u64,
    }
}

static mut DISKS: Lazy<Arc<Mutex<Disks>>> =
    Lazy::new(|| Arc::new(Mutex::new(Disks::new_with_refreshed_list())));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    kind: String,
    name: Option<String>,
    file_system: Option<String>,
    mount_point: Option<String>,
    total_space: u64,
    available_space: u64,
}

#[tauri::command]
pub fn get_disks_info() -> Vec<DiskInfo> {
    let mut disks = unsafe { DISKS.lock().unwrap() };
    disks.refresh(true);

    disks
        .list()
        .iter()
        .map(|info| DiskInfo {
            kind: match info.kind() {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                _ => "OTHERS".to_string(),
            },
            name: info.name().to_str().map(|s| s.to_string()),
            file_system: info.file_system().to_str().map(|s| s.to_string()),
            mount_point: info.mount_point().to_str().map(|s| s.to_string()),
            total_space: info.total_space(),
            available_space: info.available_space(),
        })
        .collect()
}

static mut NETWORKS: Lazy<Arc<Mutex<Networks>>> =
    Lazy::new(|| Arc::new(Mutex::new(Networks::new_with_refreshed_list())));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    name: String,
    total_received: u64,
    total_transmitted: u64,
    total_packets_received: u64,
    total_packets_transmitted: u64,
    total_errors_on_received: u64,
    total_errors_on_transmitted: u64,
    mac_address: String,
    ip_networks: Vec<String>,
    mtu: u64,
}

#[tauri::command]
pub fn get_networks_info() -> Vec<NetworkInfo> {
    let mut networks = unsafe { NETWORKS.lock().unwrap() };
    networks.refresh(true);

    networks
        .list()
        .iter()
        .map(|(name, info)| NetworkInfo {
            name: name.clone(),
            total_received: info.total_received(),
            total_transmitted: info.total_transmitted(),
            total_packets_received: info.total_packets_received(),
            total_packets_transmitted: info.total_packets_transmitted(),
            total_errors_on_received: info.total_errors_on_received(),
            total_errors_on_transmitted: info.total_errors_on_transmitted(),
            mac_address: info.mac_address().to_string(),
            ip_networks: info
                .ip_networks()
                .iter()
                .map(|i| i.addr.to_string())
                .collect(),
            mtu: info.mtu(),
        })
        .collect()
}

static mut COMPONENTS: Lazy<Arc<Mutex<Components>>> =
    Lazy::new(|| Arc::new(Mutex::new(Components::new_with_refreshed_list())));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
    label: String,
    temperature: Option<f32>,
    max: Option<f32>,
    critical: Option<f32>,
}

#[tauri::command]
pub fn get_components_info() -> Vec<ComponentInfo> {
    let mut components = unsafe { COMPONENTS.lock().unwrap() };
    components.refresh(true);

    components
        .list()
        .iter()
        .map(|info| ComponentInfo {
            label: info.label().to_string(),
            temperature: info.temperature(),
            max: info.max(),
            critical: info.critical(),
        })
        .collect()
}
