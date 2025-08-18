use regex::Regex;
use std::fs;
use std::path::Path;

pub fn read_first_line<P: AsRef<Path>>(p: P) -> Option<String> {
    fs::read_to_string(p).ok()?.lines().next().map(|l| l.to_string())
}

pub fn read_uptime() -> Option<u64> {
    let s = fs::read_to_string("/proc/uptime").ok()?;
    let re = Regex::new(r"^([0-9]+)").ok()?;
    re.captures(&s)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u64>().ok())
}

pub fn format_duration(secs: u64) -> String {
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
