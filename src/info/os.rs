use log::debug;
use std::fs;

/// Reads /etc/os-release and pulls out the distro's pretty name.
pub fn distro_name() -> String {
    let pretty = read_os_release_field("PRETTY_NAME").unwrap_or_else(|| "Unknown Linux".to_string());
    let version = read_os_release_field("VERSION_ID").unwrap_or_default();
    if version.is_empty() {
        pretty
    } else if pretty.contains(&version) {
        pretty
    } else {
        format!("{} ({})", pretty, version)
    }
}

/// Reads the short distro ID (e.g. "arch", "ubuntu", "fedora").
pub fn distro_id() -> String {
    read_os_release_field("ID").unwrap_or_else(|| "linux".to_string())
}

/// Helper: reads a specific key from /etc/os-release.
fn read_os_release_field(key: &str) -> Option<String> {
    let contents = fs::read_to_string("/etc/os-release").ok()?;
    let prefix = format!("{}=", key);

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix(&prefix) {
            let cleaned = value.trim_matches('"').to_string();
            debug!("{}={}", key, cleaned);
            return Some(cleaned);
        }
    }

    debug!("{} not found in /etc/os-release", key);
    None
}

/// Kernel version, e.g. "6.8.0-31-generic"
pub fn kernel_version() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "Unknown".to_string())
        .trim()
        .to_string()
}

/// Hostname of the machine
pub fn hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname"))
        .unwrap_or_else(|_| "Unknown".to_string())
        .trim()
        .to_string()
}

/// Machine / board name from DMI when available.
pub fn machine_model() -> String {
    for path in [
        "/sys/class/dmi/id/product_name",
        "/sys/devices/virtual/dmi/id/product_name",
    ] {
        if let Ok(raw) = fs::read_to_string(path) {
            let name = raw.trim().to_string();
            if !name.is_empty() && name != "To be filled by O.E.M." {
                return name;
            }
        }
    }
    "Unknown".to_string()
}

/// CPU architecture from the running binary.
pub fn architecture() -> String {
    std::env::consts::ARCH.to_string()
}

/// Init system (PID 1 comm).
pub fn init_system() -> String {
    fs::read_to_string("/proc/1/comm")
        .map(|s| s.trim().trim_end_matches('\n').to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Uptime in a human-readable format.
pub fn uptime() -> String {
    let raw = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let seconds: f64 = raw
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let total_secs = seconds as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;

    match (days, hours, minutes) {
        (0, 0, m) => format!("{}m", m),
        (0, h, m) => format!("{}h {}m", h, m),
        (d, h, m) => format!("{}d {}h {}m", d, h, m),
    }
}