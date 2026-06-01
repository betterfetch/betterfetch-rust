mod args;
mod ascii;
mod battery;
mod config;
mod packages;
mod system;
mod utils;

use args::Args;
use ascii::get_ascii_art;
use clap::Parser;
use colored::*;
use config::Config;
use packages::detect_package_count;
use system::SystemInfo;

/// The main endpoint of the project, applys configs and displays the system info
fn main() {
    let config: Config = Config::load();
    let args: Args = Args::parse();
    let sys: SystemInfo = SystemInfo::gather();

    // ========== COLLECT INFO LINES ==========
    let mut info_lines: Vec<String> = Vec::new();

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
            "battery".into(),
            "disk".into(),
            "packages".into(),
        ]
    });

    for module in modules_to_render {
        match module.as_str() {
            "user" => info_lines.push(format!("{} {}", color_title("user", &config), sys.username)),
            "host" => info_lines.push(format!("{} {}", color_title("host", &config), sys.hostname)),
            "os" => info_lines.push(format!(
                "{} {}",
                color_title("os", &config),
                sys.distro_line
            )),
            "kernel" => {
                info_lines.push(format!("{} {}", color_title("kernel", &config), sys.kernel))
            }
            "uptime" => {
                info_lines.push(format!("{} {}", color_title("uptime", &config), sys.uptime))
            }
            "cpu" => info_lines.push(format!(
                "{} {} ({} cores)",
                color_title("cpu", &config),
                sys.cpu_name,
                sys.cpu_cores
            )),
            "memory" => info_lines.push(format!(
                "{} {} MB / {} MB",
                color_title("memory", &config),
                sys.used_mem_mb,
                sys.total_mem_mb
            )),
            "battery" => {
                if let Some(ref battery) = sys.battery_info {
                    info_lines.push(format!("{} {}", color_title("battery", &config), battery));
                }
            }
            "disk" => info_lines.push(format!(
                "{} {}",
                color_title("disk", &config),
                sys.disk_line
            )),
            "packages" => {
                let packages = detect_package_count().unwrap_or_else(|| "unknown".into());
                info_lines.push(format!("{} {}", color_title("packages", &config), packages));
            }
            other => eprintln!("Unknown module in config: {}", other),
        }
    }

    // ========== GET ASCII ART ==========
    let ascii_art = if !args.no_ascii {
        if let Some(ref art) = config.ascii_art {
            art.as_str()
        } else {
            get_ascii_art(&sys.os)
        }
    } else {
        ""
    };

    let ascii_lines: Vec<&str> = ascii_art.lines().collect();

    // ========== CALCULATE MAX WIDTH OF ASCII ==========
    let ascii_width = ascii_lines
        .iter()
        .map(|line| strip_ansi_codes(line).len())
        .max()
        .unwrap_or(0);

    // ========== DISPLAY SIDE-BY-SIDE ==========
    let max_lines = ascii_lines.len().max(info_lines.len());
    let spacing = 4; // spaces between ASCII and info

    for i in 0..max_lines {
        let ascii_part = if i < ascii_lines.len() {
            ascii_lines[i]
        } else {
            ""
        };

        let info_part = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        // Calculate padding needed
        let ascii_visible_len = strip_ansi_codes(ascii_part).len();
        let padding = ascii_width.saturating_sub(ascii_visible_len) + spacing;

        println!(
            "{}{:padding$}{}",
            ascii_part,
            "",
            info_part,
            padding = padding
        );
    }
}

/// Apply color settings from config, fallback to bright_blue
fn color_title(title: &str, config: &Config) -> String {
    if let Some(colors) = &config.colors
        && let Some(ref color) = colors.title
    {
        // try to apply configured color (basic names like "red", "green")
        return title.color(color.as_str()).to_string();
    }
    title.bright_blue().to_string()
}

/// Strip ANSI escape codes to calculate visible width
fn strip_ansi_codes(s: &str) -> String {
    // Simple ANSI stripping - removes escape sequences
    let mut result = String::new();
    let mut in_escape = false;

    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape && ch == 'm' {
            in_escape = false;
        } else if !in_escape {
            result.push(ch);
        }
    }

    result
}
