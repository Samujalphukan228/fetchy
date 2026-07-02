use crate::info::hardware::Usage;
use crate::theme::Theme;
use std::io::IsTerminal;

#[derive(Clone)]
pub struct InfoLine {
    pub label: String,
    pub value: String,
    pub usage: Option<Usage>,
}

impl InfoLine {
    pub fn field(label: &str, value: String) -> Self {
        InfoLine {
            label: label.to_string(),
            value,
            usage: None,
        }
    }

    pub fn meter(label: &str, usage: Usage) -> Self {
        let display = usage.display();
        InfoLine {
            label: label.to_string(),
            value: display,
            usage: Some(usage),
        }
    }

    pub fn to_json_pair(&self) -> String {
        format!(
            "    \"{}\": \"{}\"",
            self.label.to_lowercase().replace([' ', '/'], "_"),
            self.value.replace('\\', "\\\\").replace('"', "\\\"")
        )
    }
}

pub struct Section {
    pub title: &'static str,
    pub lines: Vec<InfoLine>,
}

pub struct RenderContext<'a> {
    pub user: &'a str,
    pub host: &'a str,
    pub logo_id: &'a str,
    pub sections: &'a [Section],
    pub show_logo: bool,
    pub use_colors: bool,
    pub separator: &'a str,
    pub version: &'a str,
}

pub fn render(ctx: &RenderContext<'_>) {
    if ctx.use_colors && std::io::stdout().is_terminal() {
        if crate::tui::render_ratatui(ctx).is_ok() {
            return;
        }
    }
    render_plain(ctx);
}

fn render_plain(ctx: &RenderContext<'_>) {
    let theme = Theme::void();
    let title = format!("{}@{}", ctx.user, ctx.host);
    println!("{title}");
    println!("{}", "─".repeat(title.len()));

    let max_label = ctx
        .sections
        .iter()
        .flat_map(|s| s.lines.iter())
        .map(|l| l.label.len())
        .max()
        .unwrap_or(0);

    let mut first = true;
    for section in ctx.sections {
        if section.lines.is_empty() {
            continue;
        }
        if !first {
            println!();
        }
        first = false;
        println!("{}", section.title);

        for line in &section.lines {
            let label = format!("{:>width$}", line.label, width = max_label);
            let value = if let Some(usage) = &line.usage {
                usage.compact()
            } else {
                truncate_plain(&line.value, 56)
            };
            println!("  {label} {} {value}", ctx.separator);
            if let Some(usage) = &line.usage {
                let ratio = (usage.percent() / 100.0).clamp(0.0, 1.0);
                let filled = (ratio * 20.0).round() as usize;
                println!(
                    "  {} {}{} {:.0}%",
                    " ".repeat(max_label + 3),
                    "█".repeat(filled.min(20)),
                    "░".repeat(20usize.saturating_sub(filled)),
                    usage.percent()
                );
            }
        }
    }

    println!();
    println!("{}", "─".repeat(40));
    let _ = theme;
    println!("fetchy {}", ctx.version);
}

fn truncate_plain(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i >= max.saturating_sub(1) {
            out.push('…');
            break;
        }
        out.push(ch);
    }
    out
}

pub fn render_json(sections: &[Section]) {
    let lines: Vec<InfoLine> = sections
        .iter()
        .flat_map(|s| s.lines.clone())
        .collect();
    println!("{{");
    let pairs: Vec<String> = lines.iter().map(|l| l.to_json_pair()).collect();
    println!("{}", pairs.join(",\n"));
    println!("}}");
}