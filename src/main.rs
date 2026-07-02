mod cli;
mod config;
mod info;
mod logos;
mod render;
mod theme;
mod tui;

use clap::Parser;
use cli::Cli;
use info::{battery, display, env, gpu, hardware, network, os, packages};
use log::debug;
use render::{InfoLine, RenderContext, Section};
use std::process;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    let args = Cli::parse();
    debug!("CLI args: {:?}", args);

    if args.init_config {
        match config::write_default_config() {
            Ok(path) => {
                println!("✔ Wrote starter config to {}", path.display());
                println!("  Edit it to customize fetchy's defaults.");
            }
            Err(e) => {
                eprintln!("✘ Failed to write config: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    if args.list_logos {
        println!("Available logos:");
        println!();
        for name in logos::AVAILABLE_LOGOS {
            println!("  • {}", name);
        }
        println!();
        println!("Usage: systeminfo --logo <name>");
        return;
    }

    let file_config = config::load();
    debug!("Loaded config: {:?}", file_config);

    let show_logo = !(args.no_logo || file_config.no_logo);
    let use_colors = !(args.no_colors || file_config.no_colors);
    let compact = args.compact || file_config.compact;
    let separator = file_config.separator.clone();

    let logo_id = logos::resolve(
        &args
            .logo
            .or(if file_config.logo.is_empty() {
                None
            } else {
                Some(file_config.logo.clone())
            })
            .unwrap_or_else(|| os::distro_id()),
    );

    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let user = env::username();
    let host = os::hostname();
    debug!("Detected logo: {}", logo_id);

    let net = network::network_info();
    let sections = if compact {
        compact_sections(&sys, &net)
    } else {
        full_sections(&sys, &net)
    };

    if args.json {
        render::render_json(&sections);
        return;
    }

    let ctx = RenderContext {
        user: &user,
        host: &host,
        logo_id,
        sections: &sections,
        show_logo,
        use_colors,
        separator: &separator,
        version: VERSION,
    };
    render::render(&ctx);
}

fn full_sections(sys: &System, net: &network::NetworkInfo) -> Vec<Section> {
    let mut storage = vec![
        InfoLine::meter("Memory", hardware::memory_usage(sys)),
        InfoLine::meter("Swap", hardware::swap_usage(sys)),
        InfoLine::meter("Disk /", hardware::disk_usage_root()),
    ];
    if let Some((mount, usage)) = hardware::disk_usage_home() {
        storage.push(InfoLine::meter(&format!("Disk {mount}"), usage));
    }

    let mut hardware_lines = vec![
        InfoLine::field("CPU", hardware::cpu_name(sys)),
        InfoLine::meter("Load", hardware::cpu_load_usage(sys)),
        InfoLine::field("GPU", gpu::gpu_name()),
        InfoLine::field("Processes", hardware::process_count(sys).to_string()),
    ];
    if let Some(temp) = hardware::temperature() {
        hardware_lines.push(InfoLine::field("Temp", temp));
    }
    if let Some(bat) = battery::battery_status() {
        hardware_lines.push(InfoLine::field("Battery", bat));
    }

    vec![
        Section {
            title: "SYSTEM",
            lines: vec![
                InfoLine::field("OS", os::distro_name()),
                InfoLine::field("Host", os::hostname()),
                InfoLine::field("Machine", os::machine_model()),
                InfoLine::field("Arch", os::architecture()),
                InfoLine::field("Kernel", os::kernel_version()),
                InfoLine::field("Init", os::init_system()),
                InfoLine::field("Uptime", os::uptime()),
                InfoLine::field("Packages", packages::package_count()),
                InfoLine::field("Locale", env::locale()),
            ],
        },
        Section {
            title: "DESKTOP",
            lines: vec![
                InfoLine::field("DE/WM", env::desktop_environment()),
                InfoLine::field("Theme", env::theme_info()),
                InfoLine::field("Terminal", env::terminal()),
                InfoLine::field("Shell", env::shell()),
                InfoLine::field("Display", display::resolution()),
            ],
        },
        Section {
            title: "HARDWARE",
            lines: hardware_lines,
        },
        Section {
            title: "STORAGE",
            lines: storage,
        },
        Section {
            title: "NETWORK",
            lines: vec![InfoLine::field(
                "LAN",
                format!("{} ({})", net.local_ip, net.interface),
            )],
        },
    ]
}

fn compact_sections(sys: &System, net: &network::NetworkInfo) -> Vec<Section> {
    vec![Section {
        title: "SYSTEM",
        lines: vec![
            InfoLine::field("OS", os::distro_name()),
            InfoLine::field("Kernel", os::kernel_version()),
            InfoLine::field("Uptime", os::uptime()),
            InfoLine::field("DE/WM", env::desktop_environment()),
            InfoLine::field("Shell", env::shell()),
            InfoLine::field("CPU", hardware::cpu_name(sys)),
            InfoLine::meter("Load", hardware::cpu_load_usage(sys)),
            InfoLine::meter("Memory", hardware::memory_usage(sys)),
            InfoLine::field("GPU", gpu::gpu_name()),
            InfoLine::field("LAN", format!("{} ({})", net.local_ip, net.interface)),
        ],
    }]
}