use std::sync::Mutex;

use log::trace;
use once_cell::sync::Lazy;
use sbbw_exec::Params;
use serde::Serialize;
use sysinfo::{
    CpuExt, CpuRefreshKind, DiskExt, NetworkExt, NetworksExt, RefreshKind, System, SystemExt,
    UserExt,
};
use tao::window::Window;
use wry::http::status::StatusCode;

use super::SbbwResponse;

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));

#[derive(Serialize, Clone)]
struct SbbwDisk {
    pub name: String,
    pub total_space: u64,
    pub free_space: u64,
    pub is_removable: bool,
    pub mount_point: String,
    pub file_system: String,
}

pub fn disks(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_disks();
    trace!("Request disk list");

    let disks = sys
        .disks()
        .iter()
        .map(|d| SbbwDisk {
            name: d.name().to_str().unwrap_or_default().to_string(),
            total_space: d.total_space(),
            free_space: d.available_space(),
            mount_point: d.mount_point().to_str().unwrap_or_default().to_string(),
            is_removable: d.is_removable(),
            file_system: String::from_utf8(d.file_system().to_owned()).unwrap_or_default(),
        })
        .collect::<Vec<SbbwDisk>>();

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&disks).unwrap_or_default();

    res
}

#[derive(Serialize, Clone)]
struct SbbwNetwork {
    pub name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packets_transmitted: u64,
    pub total_packets_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
}

pub fn network(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_networks();
    trace!("Request Network interface list");

    let nets = sys
        .networks()
        .iter()
        .map(|n| SbbwNetwork {
            name: n.0.to_string(),
            received: n.1.received(),
            total_received: n.1.total_received(),
            transmitted: n.1.transmitted(),
            total_transmitted: n.1.total_transmitted(),
            packets_received: n.1.packets_received(),
            total_packets_received: n.1.total_packets_received(),
            packets_transmitted: n.1.packets_transmitted(),
            total_packets_transmitted: n.1.total_packets_transmitted(),
            errors_on_received: n.1.errors_on_received(),
            total_errors_on_received: n.1.total_errors_on_received(),
            errors_on_transmitted: n.1.errors_on_transmitted(),
            total_errors_on_transmitted: n.1.total_errors_on_transmitted(),
        })
        .collect::<Vec<SbbwNetwork>>();

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&nets).unwrap_or_default();

    res
}

#[derive(Serialize, Clone)]
struct SbbwInfoUser {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub groups: Vec<String>,
}

#[derive(Serialize, Clone)]
struct SbbwInfo {
    pub hostname: String,
    pub uptime: u64,
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub users: Vec<SbbwInfoUser>,
}

pub fn info(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_users_list();
    trace!("Request Info");

    let info = SbbwInfo {
        hostname: sys.host_name().unwrap_or_default(),
        uptime: sys.uptime(),
        kernel_version: sys.kernel_version().unwrap_or_default(),
        os_version: sys.os_version().unwrap_or_default(),
        long_os_version: sys.long_os_version().unwrap_or_default(),
        users: sys
            .users()
            .iter()
            .map(|u| SbbwInfoUser {
                id: u.id().to_string(),
                group_id: u.group_id().to_string(),
                name: u.name().to_string(),
                groups: u.groups().to_vec(),
            })
            .collect::<Vec<SbbwInfoUser>>(),
    };

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&info).unwrap_or_default();

    res
}

#[derive(Serialize, Clone)]
struct SbbwMemory {
    pub total: u64,
    pub free: u64,
    pub aviable: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub swap_used: u64,
}

pub fn memory(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();
    trace!("Request Memory(Ram) Data");

    let memory = SbbwMemory {
        total: sys.total_memory(),
        free: sys.free_memory(),
        aviable: sys.available_memory(),
        used: sys.used_memory(),
        swap_total: sys.total_swap(),
        swap_free: sys.free_swap(),
        swap_used: sys.used_swap(),
    };

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&memory).unwrap_or_default();

    res
}

#[derive(Serialize, Clone)]
struct SbbwCpu {
    pub cpu_usage: f32,
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
}

pub fn cpu(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_cpu();
    trace!("Request Cpu data");

    let cpus = sys
        .cpus()
        .iter()
        .map(|c| SbbwCpu {
            cpu_usage: c.cpu_usage(),
            name: c.name().to_string(),
            vendor_id: c.vendor_id().to_string(),
            brand: c.brand().to_string(),
            frequency: c.frequency(),
        })
        .collect::<Vec<SbbwCpu>>();

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&cpus).unwrap_or_default();

    res
}
