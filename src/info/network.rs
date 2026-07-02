use std::fs;
use std::process::Command;

pub struct NetworkInfo {
    pub local_ip: String,
    pub interface: String,
}

pub fn network_info() -> NetworkInfo {
    via_ip_route()
        .or_else(via_ip_addr)
        .or_else(via_proc_net)
        .unwrap_or(NetworkInfo {
            local_ip: "Unknown".to_string(),
            interface: "—".to_string(),
        })
}

fn via_ip_route() -> Option<NetworkInfo> {
    let output = Command::new("ip")
        .args(["route", "get", "1.1.1.1"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut iface = None;
    let mut ip = None;
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if *part == "dev" {
            iface = parts.get(i + 1).map(|s| s.to_string());
        }
        if *part == "src" {
            ip = parts.get(i + 1).map(|s| s.to_string());
        }
    }

    Some(NetworkInfo {
        local_ip: ip?,
        interface: iface.unwrap_or_else(|| "—".to_string()),
    })
}

fn via_ip_addr() -> Option<NetworkInfo> {
    let output = Command::new("ip")
        .args(["-4", "-o", "addr", "show", "scope", "global", "up"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let iface = parts.first()?.to_string();
    let ip = parts.get(3)?.split('/').next()?.to_string();

    Some(NetworkInfo {
        local_ip: ip,
        interface: iface,
    })
}

fn via_proc_net() -> Option<NetworkInfo> {
    let routes = fs::read_to_string("/proc/net/route").ok()?;
    let mut best_iface = None;
    let mut best_metric = u32::MAX;

    for line in routes.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 8 {
            continue;
        }
        let iface = cols[0];
        let dest = cols[1];
        let metric = cols[6].parse::<u32>().unwrap_or(u32::MAX);
        if dest == "00000000" && iface != "lo" && metric < best_metric {
            best_metric = metric;
            best_iface = Some(iface.to_string());
        }
    }

    let iface = best_iface?;
    let ip = iface_address(&iface)?;
    Some(NetworkInfo {
        local_ip: ip,
        interface: iface,
    })
}

fn iface_address(iface: &str) -> Option<String> {
    let output = Command::new("ip")
        .args(["-4", "-o", "addr", "show", "dev", iface])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(rest) = line.split_whitespace().nth(3) {
            return Some(rest.split('/').next()?.to_string());
        }
    }
    None
}