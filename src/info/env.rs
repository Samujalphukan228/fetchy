use std::env;
use std::process::Command;

/// Reads the SHELL env var and returns just the binary name.
pub fn shell() -> String {
    let name = env::var("SHELL")
        .ok()
        .and_then(|path| path.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "Unknown".to_string());

    shell_version(&name)
        .map(|v| format!("{} {}", name, v))
        .unwrap_or(name)
}

fn shell_version(shell: &str) -> Option<String> {
    let flag = match shell {
        "bash" => "--version",
        "zsh" => "--version",
        "fish" => "--version",
        "nu" | "nushell" => "--version",
        _ => return None,
    };
    let output = Command::new(shell)
        .arg(flag)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    let line = String::from_utf8_lossy(&output.stdout).lines().next()?.to_string();
    line.split_whitespace().nth(3).map(String::from)
}

/// Detects the active terminal emulator.
pub fn terminal() -> String {
    if let Some(name) = detect_named_terminal() {
        return name;
    }

    if let Ok(term) = env::var("TERM_PROGRAM") {
        let version = env::var("TERM_PROGRAM_VERSION").unwrap_or_default();
        if version.is_empty() {
            return term;
        }
        return format!("{} {}", term, version);
    }

    if let Ok(term) = env::var("TERMINAL") {
        return term;
    }

    env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
}

fn detect_named_terminal() -> Option<String> {
    if env::var("KITTY_WINDOW_ID").is_ok() {
        return version_of("kitty", &["+kitten", "show-version"]).or(Some("kitty".into()));
    }
    if env::var("ALACRITTY_WINDOW_ID").is_ok() || env::var("ALACRITTY_LOG").is_ok() {
        return version_of("alacritty", &["-V"]).or(Some("alacritty".into()));
    }
    if env::var("WEZTERM_EXECUTABLE").is_ok() {
        return version_of("wezterm", &["--version"]).or(Some("wezterm".into()));
    }
    if env::var("FOOT_LOG").is_ok() || env::var("FOOT_PID").is_ok() {
        return version_of("foot", &["--version"]).or(Some("foot".into()));
    }
    if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return Some("ghostty".to_string());
    }
    if env::var("KONSOLE_VERSION").is_ok() {
        return env::var("KONSOLE_VERSION")
            .ok()
            .map(|v| format!("konsole {}", v));
    }
    if env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        return Some("gnome-terminal".to_string());
    }
    if env::var("VTE_VERSION").is_ok() {
        return Some("vte-based".to_string());
    }
    None
}

fn version_of(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    let line = String::from_utf8_lossy(&output.stdout).lines().next()?.trim().to_string();
    if line.is_empty() {
        None
    } else {
        Some(line)
    }
}

/// Current username
pub fn username() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Desktop environment, window manager, or compositor.
pub fn desktop_environment() -> String {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return wm_version("Hyprland", "hyprctl", &["version"]);
    }
    if env::var("SWAYSOCK").is_ok() {
        return wm_version("Sway", "swaymsg", &["-t", "get_version"]);
    }
    if env::var("I3SOCK").is_ok() {
        return "i3".to_string();
    }
    if env::var("WAYLAND_DISPLAY").is_ok() {
        if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
            if !de.is_empty() {
                return de;
            }
        }
    }
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        if !de.is_empty() {
            return de;
        }
    }
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        if !de.is_empty() {
            return de;
        }
    }
    if env::var("DISPLAY").is_ok() {
        return "X11".to_string();
    }
    "Unknown".to_string()
}

fn wm_version(name: &str, cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output();
    if let Ok(o) = output {
        if o.status.success() {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let line = stdout.lines().next().unwrap_or("").trim();
            let ver = line
                .split_whitespace()
                .find(|w| w.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
                .unwrap_or(line);
            if !ver.is_empty() {
                return format!("{name} {ver}");
            }
        }
    }
    name.to_string()
}

/// GTK / icon theme when available.
pub fn theme_info() -> String {
    let gtk = env::var("GTK_THEME")
        .or_else(|_| gsettings_get("org.gnome.desktop.interface", "gtk-theme"))
        .unwrap_or_default();
    let icons = env::var("ICON_THEME")
        .or_else(|_| gsettings_get("org.gnome.desktop.interface", "icon-theme"))
        .unwrap_or_default();

    match (gtk.is_empty(), icons.is_empty()) {
        (false, false) => format!("{} / {}", gtk, icons),
        (false, true) => gtk,
        (true, false) => format!("icons: {}", icons),
        (true, true) => "Unknown".to_string(),
    }
}

fn gsettings_get(schema: &str, key: &str) -> Result<String, std::env::VarError> {
    let output = Command::new("gsettings")
        .args(["get", schema, key])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .map_err(|_| std::env::VarError::NotPresent)?;
    if !output.status.success() {
        return Err(std::env::VarError::NotPresent);
    }
    let value = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\'')
        .to_string();
    if value.is_empty() {
        Err(std::env::VarError::NotPresent)
    } else {
        Ok(value)
    }
}

/// System locale, e.g. en_US.UTF-8
pub fn locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LC_MESSAGES"))
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "Unknown".to_string())
}