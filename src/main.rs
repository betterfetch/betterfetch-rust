mod args;
mod ascii;
mod packages;
mod system;
mod utils;

use args::Args;
use ascii::print_ascii_art;
use clap::Parser;
use colored::*;
use packages::detect_package_count;
use system::SystemInfo;

fn main() {
    let args = Args::parse();

    let sys = SystemInfo::gather();

    if !args.no_ascii {
        print_ascii_art(&sys.os);
    }

    println!("{} {}", "user".bright_blue(), sys.username);
    println!("{} {}", "host".bright_blue(), sys.hostname);
    println!("{} {}", "os".bright_blue(), sys.distro_line);
    println!("{} {}", "kernel".bright_blue(), sys.kernel);
    println!("{} {}", "uptime".bright_blue(), sys.uptime);
    println!(
        "{} {} ({} cores)",
        "cpu".bright_blue(),
        sys.cpu_name,
        sys.cpu_cores
    );
    println!(
        "{} {} MB / {} MB",
        "memory".bright_blue(),
        sys.used_mem_mb,
        sys.total_mem_mb
    );
    println!("{} {}", "disk".bright_blue(), sys.disk_line);

    let packages = detect_package_count().unwrap_or_else(|| "unknown".into());
    println!("{} {}", "packages".bright_blue(), packages);
}
