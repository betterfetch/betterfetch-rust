use clap::Parser;
use colored::*;
use hostname::get as get_hostname;
use os_info::Info;
use regex::Regex;
use std::fs;
use std::path::Path;
use sysinfo::{Disks, System};
use users::{get_current_uid, get_user_by_uid};

#[derive(Parser, Debug)]
#[command(name = "betterfetch", about = "System fetcher written in Rust (Linux)")]
struct Args {
    /// Hide ASCII art
    #[arg(short, long)]
    no_ascii: bool,
}

/// The main function of the program.
///
/// Parses command line arguments using `clap` and retrieves system information
/// using `sysinfo` and `os_info`. Prints various system information such as
/// username, hostname, operating system, kernel, uptime, CPU, memory, disk
/// usage, and package count.
fn main() {
    let args = Args::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    // OS info
    let os = os_info::get();
    let distro_line = format!("{} {}", os.os_type(), os.version());

    // Hostname & user
    let hostname = get_hostname()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".into());
    let uid = get_current_uid();
    let username = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".into());

    // Kernel
    let kernel = read_first_line("/proc/version")
        .unwrap_or_else(|| "kernel info unavailable".into());

    // Uptime
    let uptime = read_uptime()
        .map(format_duration)
        .unwrap_or_else(|| "unknown".into());

    // CPU & memory
    let cpu_name = sys.global_cpu_info().brand().to_string();
    let cpu_cores = sys.physical_core_count().unwrap_or(0);
    let total_mem_mb = sys.total_memory() / 1024;
    let used_mem_mb = sys.used_memory() / 1024;

    // Disk info
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

    // Package count
    let packages = detect_package_count().unwrap_or_else(|| "unknown".into());

    // Output
    if !args.no_ascii {
        print_ascii_art(&os);
    }

    println!("{} {}", "user".bright_blue(), username);
    println!("{} {}", "host".bright_blue(), hostname);
    println!("{} {}", "os".bright_blue(), distro_line);
    println!("{} {}", "kernel".bright_blue(), kernel);
    println!("{} {}", "uptime".bright_blue(), uptime);
    println!("{} {} ({} cores)", "cpu".bright_blue(), cpu_name, cpu_cores);
    println!(
        "{} {} MB / {} MB",
        "memory".bright_blue(),
        used_mem_mb,
        total_mem_mb
    );
    println!("{} {}", "disk".bright_blue(), disk_line);
    println!("{} {}", "packages".bright_blue(), packages);
}

fn read_first_line<P: AsRef<Path>>(p: P) -> Option<String> {
    fs::read_to_string(p).ok()?.lines().next().map(|l| l.to_string())
}

fn read_uptime() -> Option<u64> {
    let s = fs::read_to_string("/proc/uptime").ok()?;
    let re = Regex::new(r"^([0-9]+)").ok()?;
    re.captures(&s)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u64>().ok())
}

fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

fn print_ascii_art(os: &Info) {
    let name = os.os_type().to_string().to_lowercase();
    if name.contains("ubuntu") {
        println!("{}", UBUNTU_ASCII);
    } else if name.contains("debian") {
        println!("{}", DEBIAN_ASCII);
    } else if name.contains("arch") {
        println!("{}", ARCH_ASCII);
    } else {
        println!("{}", GENERIC_ASCII);
    }
}

/// Detects the number of installed packages.
///
/// This function checks for the existence of common package managers and
/// counts the number of installed packages using their respective commands.
/// The function returns an `Option<String>`, where the string contains the
/// package count followed by the package manager name. If no package manager
/// is found or the command fails, `None` is returned.
fn detect_package_count() -> Option<String> {
    if which::which("dpkg-query").is_ok() {
        if let Ok(out) = std::process::Command::new("sh")
            .arg("-c")
            .arg("dpkg-query -f '${binary:Package}\\n' -W | wc -l")
            .output()
        {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return Some(format!("{} (dpkg)", s.trim()));
                }
            }
        }
    }
    if which::which("rpm").is_ok() {
        if let Ok(out) = std::process::Command::new("rpm").arg("-qa").output() {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    let cnt = s.lines().count();
                    return Some(format!("{} (rpm)", cnt));
                }
            }
        }
    }
    if which::which("pacman").is_ok() {
        if let Ok(out) = std::process::Command::new("sh")
            .arg("-c")
            .arg("pacman -Q | wc -l")
            .output()
        {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return Some(format!("{} (pacman)", s.trim()));
                }
            }
        }
    }
    if which::which("flatpak").is_ok() {
        if let Ok(out) = std::process::Command::new("sh")
            .arg("-c")
            .arg("flatpak list --app | wc -l")
            .output()
        {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return Some(format!("{} (flatpak)", s.trim()));
                }
            }
        }
    }
    None
}

// ASCII art for different distributions
// These are used to display a nice logo based on the detected OS type.
// The ASCII art is displayed only if the `--no-ascii` flag is not set
// when running the program.
const GENERIC_ASCII: &str = r#"
 _          _   _             __      _       _     
| |__   ___| |_| |_ ___ _ __ / _| ___| |_ ___| |__  
| '_ \ / _ \ __| __/ _ \ '__| |_ / _ \ __/ __| '_ \ 
| |_) |  __/ |_| ||  __/ |  |  _|  __/ || (__| | | |
|_.__/ \___|\__|\__\___|_|  |_|  \___|\__\___|_| |_|                                                
"#;

const UBUNTU_ASCII: &str = r#"
 _   _ _                 _         
| | | | |__  _   _ _ __ | |_ _   _ 
| | | | '_ \| | | | '_ \| __| | | |
| |_| | |_) | |_| | | | | |_| |_| |
 \___/|_.__/ \__,_|_| |_|\__|\__,_|
                                
"#;

const DEBIAN_ASCII: &str = r#"
 ____       _     _             
|  _ \  ___| |__ (_) __ _ _ __  
| | | |/ _ \ '_ \| |/ _` | '_ \ 
| |_| |  __/ |_) | | (_| | | | |
|____/ \___|_.__/|_|\__,_|_| |_|
                                
"#;

const ARCH_ASCII: &str = r#"
    _             _     
   / \   _ __ ___| |__  
  / _ \ | '__/ __| '_ \ 
 / ___ \| | | (__| | | |
/_/   \_\_|  \___|_| |_|

"#;
