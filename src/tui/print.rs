use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier, Style};

pub fn print_buffer(buf: &Buffer) {
    let area = buf.area;
    let mut prev = Style::default();

    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let cell = &buf[(x, y)];
            let style = cell.style();
            if style != prev {
                print!("{}", style_patch(style, prev));
                prev = style;
            }
            print!("{}", cell.symbol());
        }
        println!();
        if prev != Style::default() {
            print!("\x1b[0m");
            prev = Style::default();
        }
    }
}

fn style_patch(next: Style, prev: Style) -> String {
    let mut out = String::from("\x1b[0m");

    if next.add_modifier.contains(Modifier::BOLD) {
        out.push_str("\x1b[1m");
    }
    if next.add_modifier.contains(Modifier::DIM) {
        out.push_str("\x1b[2m");
    }
    if next.add_modifier.contains(Modifier::ITALIC) {
        out.push_str("\x1b[3m");
    }
    if next.add_modifier.contains(Modifier::UNDERLINED) {
        out.push_str("\x1b[4m");
    }

    if next.fg != prev.fg {
        match next.fg {
            Some(fg) => out.push_str(&color_fg(fg)),
            None => out.push_str("\x1b[39m"),
        }
    }
    if next.bg != prev.bg {
        match next.bg {
            Some(bg) => out.push_str(&color_bg(bg)),
            None => out.push_str("\x1b[49m"),
        }
    }

    out
}

fn color_fg(color: Color) -> String {
    match color {
        Color::Reset => "\x1b[39m".to_string(),
        Color::Black => "\x1b[30m".to_string(),
        Color::Red => "\x1b[31m".to_string(),
        Color::Green => "\x1b[32m".to_string(),
        Color::Yellow => "\x1b[33m".to_string(),
        Color::Blue => "\x1b[34m".to_string(),
        Color::Magenta => "\x1b[35m".to_string(),
        Color::Cyan => "\x1b[36m".to_string(),
        Color::Gray => "\x1b[90m".to_string(),
        Color::DarkGray => "\x1b[90m".to_string(),
        Color::LightRed => "\x1b[91m".to_string(),
        Color::LightGreen => "\x1b[92m".to_string(),
        Color::LightYellow => "\x1b[93m".to_string(),
        Color::LightBlue => "\x1b[94m".to_string(),
        Color::LightMagenta => "\x1b[95m".to_string(),
        Color::LightCyan => "\x1b[96m".to_string(),
        Color::White => "\x1b[97m".to_string(),
        Color::Rgb(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
        Color::Indexed(i) => format!("\x1b[38;5;{i}m"),
    }
}

fn color_bg(color: Color) -> String {
    match color {
        Color::Reset => "\x1b[49m".to_string(),
        Color::Black => "\x1b[40m".to_string(),
        Color::Red => "\x1b[41m".to_string(),
        Color::Green => "\x1b[42m".to_string(),
        Color::Yellow => "\x1b[43m".to_string(),
        Color::Blue => "\x1b[44m".to_string(),
        Color::Magenta => "\x1b[45m".to_string(),
        Color::Cyan => "\x1b[46m".to_string(),
        Color::Gray => "\x1b[100m".to_string(),
        Color::DarkGray => "\x1b[100m".to_string(),
        Color::LightRed => "\x1b[101m".to_string(),
        Color::LightGreen => "\x1b[102m".to_string(),
        Color::LightYellow => "\x1b[103m".to_string(),
        Color::LightBlue => "\x1b[104m".to_string(),
        Color::LightMagenta => "\x1b[105m".to_string(),
        Color::LightCyan => "\x1b[106m".to_string(),
        Color::White => "\x1b[107m".to_string(),
        Color::Rgb(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
        Color::Indexed(i) => format!("\x1b[48;5;{i}m"),
    }
}