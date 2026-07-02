use std::process::Command;

pub fn resolution() -> String {
    hyprland_monitors()
        .or_else(sway_outputs)
        .or_else(wlr_randr)
        .or_else(xrandr_current)
        .unwrap_or_else(|| "Unknown".to_string())
}

fn hyprland_monitors() -> Option<String> {
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_err() {
        return None;
    }
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return hyprctl_plain();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_hypr_json(&stdout)
}

fn hyprctl_plain() -> Option<String> {
    let output = Command::new("hyprctl")
        .args(["monitors"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut sizes = Vec::new();
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("res: ") {
            sizes.push(rest.trim().to_string());
        }
    }
    if sizes.is_empty() {
        None
    } else {
        Some(sizes.join(", "))
    }
}

fn parse_hypr_json(json: &str) -> Option<String> {
    let mut sizes = Vec::new();
    for line in json.lines() {
        if let Some(idx) = line.find("\"width\"") {
            let w = extract_json_number(line, idx + 8)?;
            // crude: look ahead for height on same or next chunk
            if let Some(h_idx) = line.find("\"height\"") {
                let h = extract_json_number(line, h_idx + 9)?;
                sizes.push(format!("{}x{}", w, h));
            }
        }
    }
    if sizes.is_empty() {
        None
    } else {
        Some(sizes.join(", "))
    }
}

fn extract_json_number(line: &str, start: usize) -> Option<u32> {
    let slice = line.get(start..)?;
    let digits: String = slice.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn sway_outputs() -> Option<String> {
    if std::env::var("SWAYSOCK").is_err() {
        return None;
    }
    let output = Command::new("swaymsg")
        .args(["-t", "get_outputs"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut sizes = Vec::new();
    for line in stdout.lines() {
        if let Some(rest) = line.trim().strip_prefix("\"current_mode\":") {
            if let (Some(w), Some(h)) = (
                extract_json_number(rest, rest.find("\"width\"").map(|i| i + 8).unwrap_or(0)),
                extract_json_number(rest, rest.find("\"height\"").map(|i| i + 9).unwrap_or(0)),
            ) {
                if w > 0 && h > 0 {
                    sizes.push(format!("{}x{}", w, h));
                }
            }
        }
    }
    if sizes.is_empty() {
        None
    } else {
        Some(sizes.join(", "))
    }
}

fn wlr_randr() -> Option<String> {
    let output = Command::new("wlr-randr")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_mode_lines(&String::from_utf8_lossy(&output.stdout))
}

fn xrandr_current() -> Option<String> {
    let output = Command::new("xrandr")
        .args(["--current"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_mode_lines(&String::from_utf8_lossy(&output.stdout))
}

fn parse_mode_lines(text: &str) -> Option<String> {
    let mut sizes = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.contains('*') || trimmed.contains('+') {
            if let Some(mode) = trimmed.split_whitespace().nth(0) {
                if mode.contains('x') {
                    sizes.push(mode.to_string());
                }
            }
        }
    }
    if sizes.is_empty() {
        None
    } else {
        Some(sizes.join(", "))
    }
}