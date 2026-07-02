use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub no_logo: bool,

    #[serde(default)]
    pub no_colors: bool,

    #[serde(default)]
    pub compact: bool,

    #[serde(default)]
    pub logo: String,

    #[serde(default = "default_separator")]
    pub separator: String,
}

fn default_separator() -> String {
    "·".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            no_logo: false,
            no_colors: false,
            compact: false,
            logo: String::new(),
            separator: default_separator(),
        }
    }
}

pub fn config_path() -> Option<PathBuf> {
    let mut path = dirs::config_dir()?;
    path.push("systeminfo");
    path.push("config.toml");
    Some(path)
}

pub fn load() -> Config {
    let Some(path) = config_path() else {
        debug!("Could not determine config directory, using defaults");
        return Config::default();
    };

    match fs::read_to_string(&path) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                warn!("Failed to parse config file: {}. Using defaults.", e);
                Config::default()
            }
        },
        Err(_) => {
            debug!("No config file at {}, using defaults", path.display());
            Config::default()
        }
    }
}

pub fn write_default_config() -> std::io::Result<PathBuf> {
    let path = config_path().expect("could not determine config directory");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let starter = r#"# ─── fetchy configuration ─────────────────────────────────────────
# CLI flags always override these values.

# Hide the ASCII logo
no_logo = false

# Plain text output (no colors)
no_colors = false

# Compact mode — fewer fields
compact = false

# Force a logo: arch, ubuntu, debian, fedora, manjaro, opensuse,
# void, nixos, gentoo, mint, pop, endeavouros, generic
logo = ""

# Label separator (default: middle dot)
separator = "·"
"#;

    fs::write(&path, starter)?;
    Ok(path)
}