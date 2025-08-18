pub fn detect_package_count() -> Option<String> {
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
