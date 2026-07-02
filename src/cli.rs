use clap::Parser;

/// systeminfo — a fast, friendly system info tool for your terminal.
#[derive(Parser, Debug)]
#[command(
    name = "systeminfo",
    version,
    about = "A fast, friendly system info tool for your terminal",
    long_about = "Fetchy displays system information alongside a colorful ASCII logo.\n\
                  It auto-detects your Linux distribution and picks the matching art.\n\
                  Customize output with flags or a config file."
)]
pub struct Cli {
    /// Hide the ASCII logo and only show info lines
    #[arg(long)]
    pub no_logo: bool,

    /// Disable colored output (useful for piping to a file)
    #[arg(long)]
    pub no_colors: bool,

    /// Compact output — fewer fields, single section
    #[arg(long)]
    pub compact: bool,

    /// Force a specific logo instead of auto-detecting your distro
    #[arg(long, value_name = "DISTRO")]
    pub logo: Option<String>,

    /// Write a starter config file to ~/.config/systeminfo/config.toml and exit
    #[arg(long)]
    pub init_config: bool,

    /// List all available logos and exit
    #[arg(long)]
    pub list_logos: bool,

    /// Print output as JSON (for scripting)
    #[arg(long)]
    pub json: bool,
}