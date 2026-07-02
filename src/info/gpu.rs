use log::debug;
use std::process::Command;

/// Detects GPU using lspci.
pub fn gpu_name() -> String {
    let output = match Command::new("lspci")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            debug!("lspci not found");
            return "Unknown (lspci not found)".to_string();
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpus: Vec<String> = Vec::new();

    for line in stdout.lines() {
        if line.contains("VGA compatible controller") || line.contains("3D controller") {
            if let Some(name) = extract_gpu_name(line) {
                debug!("Found GPU: {}", name);
                gpus.push(name);
            }
        }
    }

    if gpus.is_empty() {
        "Unknown".to_string()
    } else {
        gpus.join(", ")
    }
}

/// Extracts the GPU name from a lspci line.
fn extract_gpu_name(line: &str) -> Option<String> {
    // Line format: "01:00.0 VGA compatible controller: NVIDIA Corporation ..."
    let after_type = line.split(": ").skip(1).collect::<Vec<&str>>().join(": ");

    if after_type.is_empty() {
        return None;
    }

    // Remove revision info like "(rev 06)"
    let cleaned = if let Some(idx) = after_type.rfind(" (rev") {
        &after_type[..idx]
    } else {
        &after_type
    };

    Some(cleaned.trim().to_string())
}