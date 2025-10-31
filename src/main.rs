mod args;
mod ascii;
mod config;
mod packages;
mod system;
mod utils;

use args::Args;
use ascii::print_ascii_art;
use clap::Parser;
use colored::*;
use config::Config;
use packages::detect_package_count;
use system::SystemInfo;

fn main() {
    let config: Config = Config::load();
    let args: Args = Args::parse();
    let sys: SystemInfo = SystemInfo::gather();

    // ========== ASCII ART ==========
    if !args.no_ascii {
        if let Some(ref art) = config.ascii_art {
            println!("{}", art);
        } else {
            print_ascii_art(&sys.os);
        }
    }

    // ========== MODULES ==========
    // fallback to your default order if config doesn't define
    let modules_to_render = config.modules.clone().unwrap_or_else(|| {
        vec![
            "user".into(),
            "host".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "packages".into(),
        ]
    });

    for module in modules_to_render {
        match module.as_str() {
            "user" => println!("{} {}", color_title("user", &config), sys.username),
            "host" => println!("{} {}", color_title("host", &config), sys.hostname),
            "os" => println!("{} {}", color_title("os", &config), sys.distro_line),
            "kernel" => println!("{} {}", color_title("kernel", &config), sys.kernel),
            "uptime" => println!("{} {}", color_title("uptime", &config), sys.uptime),
            // FIXME: See the src/system.rs file
            "cpu" => println!(
                "{} {} ({}cores)",
                color_title("cpu", &config),
                sys.cpu_name,
                sys.cpu_cores
            ),

            "memory" => println!(
                "{} {} MB / {} MB",
                color_title("memory", &config),
                sys.used_mem_mb,
                sys.total_mem_mb
            ),
            "disk" => println!("{} {}", color_title("disk", &config), sys.disk_line),
            "packages" => {
                let packages = detect_package_count().unwrap_or_else(|| "unknown".into());
                println!("{} {}", color_title("packages", &config), packages);
            }
            other => eprintln!("Unknown module in config: {}", other),
        }
    }
}

/// Apply color settings from config, fallback to bright_blue
fn color_title(title: &str, config: &Config) -> String {
    if let Some(colors) = &config.colors {
        if let Some(ref color) = colors.title {
            // try to apply configured color (basic names like "red", "green")
            return title.color(color.as_str()).to_string();
        }
    }
    title.bright_blue().to_string()
}
