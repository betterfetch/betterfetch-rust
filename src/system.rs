use hostname::get as get_hostname;
use os_info::{self, Info};
use std::path::Path;
use sysinfo::{Disks, System};
use users::{get_current_uid, get_user_by_uid};

use crate::utils::{read_first_line, read_uptime, format_duration};

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
            .unwrap_or_else(|| "kernel info unavailable".into());

        let uptime = read_uptime()
            .map(format_duration)
            .unwrap_or_else(|| "unknown".into());

        let cpu_name = sys.global_cpu_info().brand().to_string();
        let cpu_cores = sys.physical_core_count().unwrap_or(0);
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
        }
    }
}
