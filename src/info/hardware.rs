use std::path::Path;
use sysinfo::{Disks, System};

#[derive(Clone)]
pub struct Usage {
    pub used: f64,
    pub total: f64,
    pub unit: &'static str,
}

impl Usage {
    pub fn percent(&self) -> f64 {
        if self.total <= 0.0 {
            0.0
        } else {
            (self.used / self.total) * 100.0
        }
    }

    pub fn display(&self) -> String {
        if self.total <= 0.0 {
            return "Unknown".to_string();
        }
        format!(
            "{:.1} / {:.1} {} ({:.0}%)",
            self.used,
            self.total,
            self.unit,
            self.percent()
        )
    }

    pub fn compact(&self) -> String {
        if self.total <= 0.0 {
            return "Unknown".to_string();
        }
        format!("{:.1} / {:.1} {}", self.used, self.total, self.unit)
    }
}

/// CPU model name with core count summary.
pub fn cpu_name(sys: &System) -> String {
    let brand = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    let cores = sys.cpus().len();
    if cores > 0 {
        format!("{} ({} cores)", brand, cores)
    } else {
        brand
    }
}

pub fn cpu_load_usage(sys: &System) -> Usage {
    let cpus = sys.cpus();
    let avg = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };
    Usage {
        used: avg as f64,
        total: 100.0,
        unit: "%",
    }
}

/// Memory usage in GiB.
pub fn memory_usage(sys: &System) -> Usage {
    let used = sys.used_memory() as f64 / 1_073_741_824.0;
    let total = sys.total_memory() as f64 / 1_073_741_824.0;
    Usage {
        used,
        total,
        unit: "GiB",
    }
}

/// Swap usage in GiB.
pub fn swap_usage(sys: &System) -> Usage {
    let used = sys.used_swap() as f64 / 1_073_741_824.0;
    let total = sys.total_swap() as f64 / 1_073_741_824.0;
    Usage {
        used,
        total,
        unit: "GiB",
    }
}

/// Running process count.
pub fn process_count(sys: &System) -> usize {
    sys.processes().len()
}

/// Returns disk usage for the root "/" filesystem.
pub fn disk_usage_root() -> Usage {
    disk_usage_at("/")
}

/// Home partition usage when on a separate mount from /.
pub fn disk_usage_home() -> Option<(String, Usage)> {
    let home = dirs::home_dir()?;
    let mount = find_mount_for_path(&home)?;
    if mount == "/" {
        return None;
    }
    let usage = disk_usage_at(&mount);
    if usage.total <= 0.0 {
        return None;
    }
    Some((mount, usage))
}

fn find_mount_for_path(path: &Path) -> Option<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut best: Option<(String, usize)> = None;

    for disk in disks.list() {
        let mp = disk.mount_point();
        if path.starts_with(mp) {
            let len = mp.as_os_str().len();
            if best.as_ref().map(|(_, l)| len > *l).unwrap_or(true) {
                best = Some((mp.to_string_lossy().to_string(), len));
            }
        }
    }

    best.map(|(mount, _)| mount)
}

fn disk_usage_at(path: &str) -> Usage {
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy();
        if mount == path {
            let total = disk.total_space() as f64 / 1_073_741_824.0;
            let available = disk.available_space() as f64 / 1_073_741_824.0;
            let used = total - available;
            return Usage {
                used: used.max(0.0),
                total,
                unit: "GiB",
            };
        }
    }

    Usage {
        used: 0.0,
        total: 0.0,
        unit: "GiB",
    }
}

/// Best-effort CPU temperature from thermal zones.
pub fn temperature() -> Option<String> {
    let zones = std::fs::read_dir("/sys/class/thermal").ok()?;
    let mut temps: Vec<String> = Vec::new();

    for entry in zones.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("thermal_zone") {
            continue;
        }
        let type_path = entry.path().join("type");
        let temp_path = entry.path().join("temp");
        let zone_type = std::fs::read_to_string(&type_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let raw = std::fs::read_to_string(&temp_path).ok()?;
        let milli: i64 = raw.trim().parse().ok()?;
        let celsius = milli as f64 / 1000.0;
        if celsius < 1.0 || celsius > 110.0 {
            continue;
        }
        let label = if zone_type.is_empty() {
            "cpu".to_string()
        } else {
            zone_type
        };
        if temps.iter().any(|t| t.starts_with(&format!("{label} "))) {
            continue;
        }
        temps.push(format!("{label} {:.0}°C", celsius));
        if temps.len() >= 3 {
            break;
        }
    }

    if temps.is_empty() {
        None
    } else {
        Some(temps.join(", "))
    }
}