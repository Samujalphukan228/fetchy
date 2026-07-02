use log::debug;
use std::process::Command;

/// Tries each known package manager and returns a count string.
pub fn package_count() -> String {
    let mut counts: Vec<String> = Vec::new();

    // Arch / Manjaro
    if let Some(count) = count_via("pacman", &["-Qq"]) {
        debug!("pacman: {} packages", count);
        counts.push(format!("{} (pacman)", count));
    }

    // Debian / Ubuntu
    if let Some(count) = count_via("dpkg-query", &["-f", ".\n", "-W"]) {
        debug!("dpkg: {} packages", count);
        counts.push(format!("{} (dpkg)", count));
    }

    // Fedora / RHEL
    if let Some(count) = count_via("rpm", &["-qa"]) {
        debug!("rpm: {} packages", count);
        counts.push(format!("{} (rpm)", count));
    }

    // Flatpak
    if let Some(count) = count_via("flatpak", &["list"]) {
        if count > 0 {
            debug!("flatpak: {} packages", count);
            counts.push(format!("{} (flatpak)", count));
        }
    }

    // Snap
    if let Some(count) = count_via("snap", &["list"]) {
        let adjusted = count.saturating_sub(1); // header line
        if adjusted > 0 {
            debug!("snap: {} packages", adjusted);
            counts.push(format!("{} (snap)", adjusted));
        }
    }

    if counts.is_empty() {
        "Unknown".to_string()
    } else {
        counts.join(", ")
    }
}

/// Runs `command args` and counts non-empty lines of stdout.
fn count_via(command: &str, args: &[&str]) -> Option<usize> {
    let output = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(stdout.lines().filter(|line| !line.trim().is_empty()).count())
}