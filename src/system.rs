use hostname::get as get_hostname;
use os_info::{self, Info};
use std::fs;
use std::path::Path;
use sysinfo::{Disks, System};
use users::{get_current_uid, get_user_by_uid};

use crate::battery::get_battery_info;
use crate::utils::{format_duration, read_first_line, read_uptime};

pub struct SystemInfo {
    pub os: Info,
    pub distro_line: String,
    pub hostname: String,
    pub username: String,
    pub kernel: String,
    pub uptime: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub total_mem_mb: u64,
    pub used_mem_mb: u64,
    pub disk_line: String,
    pub battery_info: Option<String>,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os = os_info::get();
        let distro_line = format!("{} {}", os.os_type(), os.version());

        let hostname = get_hostname()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".into());
        let uid = get_current_uid();
        let username = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into());

        let kernel = read_first_line("/proc/version")
            .and_then(|s| s.split_whitespace().nth(2).map(|v| v.to_string()))
            .unwrap_or_else(|| "kernel info unavailable".into());

        let uptime = read_uptime()
            .map(format_duration)
            .unwrap_or_else(|| "unknown".into());
        let cpu_name = {
            let brand = sys.global_cpu_info().brand().to_string();
            if brand.is_empty() {
                fs::read_to_string("/proc/cpuinfo")
                    .ok()
                    .and_then(|s| {
                        s.lines()
                            .find(|l| l.starts_with("model name"))
                            .and_then(|l| l.split(':').nth(1))
                            .map(|s| s.trim().to_string())
                    })
                    .unwrap_or_else(|| "Unknown CPU".to_string())
            } else {
                brand
            }
        };
        let cpu_cores = sys.cpus().len();
        let total_mem_mb = sys.total_memory() / 1024;
        let used_mem_mb = sys.used_memory() / 1024;

        let mut disk_line = String::from("No disk info");
        let disks = Disks::new_with_refreshed_list();
        if let Some(d) = disks.iter().find(|d| d.mount_point() == Path::new("/")) {
            let total = d.total_space() / 1024 / 1024;
            let avail = d.available_space() / 1024 / 1024;
            disk_line = format!("{}/{} MB free", avail, total);
        } else if let Some(d0) = disks.iter().next() {
            let total = d0.total_space() / 1024 / 1024;
            let avail = d0.available_space() / 1024 / 1024;
            disk_line = format!("{}/{} MB free (first disk)", avail, total);
        }

        SystemInfo {
            os,
            distro_line,
            hostname,
            username,
            kernel,
            uptime,
            cpu_name,
            cpu_cores,
            total_mem_mb,
            used_mem_mb,
            disk_line,
            battery_info: get_battery_info(),
        }
    }
}
