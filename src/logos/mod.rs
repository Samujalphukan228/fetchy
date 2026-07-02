use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Circle, Context, Line, Points, Rectangle};
use ratatui::widgets::{Block, BorderType, Widget};

pub const AVAILABLE_LOGOS: &[&str] = &[
    "arch",
    "ubuntu",
    "debian",
    "fedora",
    "manjaro",
    "opensuse",
    "void",
    "nixos",
    "gentoo",
    "mint",
    "pop",
    "endeavouros",
    "generic",
];

pub fn resolve(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "arch" | "archlinux" => "arch",
        "ubuntu" => "ubuntu",
        "debian" => "debian",
        "fedora" => "fedora",
        "manjaro" => "manjaro",
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => "opensuse",
        "void" => "void",
        "nixos" => "nixos",
        "gentoo" => "gentoo",
        "linuxmint" | "mint" => "mint",
        "pop" | "pop_os" => "pop",
        "endeavouros" => "endeavouros",
        "generic" => "generic",
        _ => "generic",
    }
}

pub fn accent(id: &str) -> Color {
    match id {
        "arch" => Color::Rgb(23, 147, 209),
        "ubuntu" => Color::Rgb(233, 84, 32),
        "debian" => Color::Rgb(215, 10, 83),
        "fedora" => Color::Rgb(60, 110, 180),
        "manjaro" => Color::Rgb(52, 181, 95),
        "opensuse" => Color::Rgb(115, 186, 56),
        "void" => Color::Rgb(72, 185, 140),
        "nixos" => Color::Rgb(95, 179, 228),
        "gentoo" => Color::Rgb(180, 90, 200),
        "mint" => Color::Rgb(135, 196, 82),
        "pop" => Color::Rgb(72, 176, 222),
        "endeavouros" => Color::Rgb(139, 92, 246),
        _ => Color::Rgb(250, 250, 250),
    }
}

pub fn display_name(id: &str) -> &'static str {
    match id {
        "arch" => "Arch Linux",
        "ubuntu" => "Ubuntu",
        "debian" => "Debian",
        "fedora" => "Fedora",
        "manjaro" => "Manjaro",
        "opensuse" => "openSUSE",
        "void" => "Void",
        "nixos" => "NixOS",
        "gentoo" => "Gentoo",
        "mint" => "Linux Mint",
        "pop" => "Pop!_OS",
        "endeavouros" => "EndeavourOS",
        _ => "Linux",
    }
}

pub struct LogoCanvas<'a> {
    pub id: &'a str,
    pub border_style: ratatui::style::Style,
    pub title_style: ratatui::style::Style,
}

impl Widget for LogoCanvas<'_> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let accent = accent(self.id);
        let block = Block::default()
            .title(display_name(self.id))
            .title_style(self.title_style)
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.border_style)
            .style(self.border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 4 || inner.height < 4 {
            return;
        }

        let canvas = Canvas::default()
            .block(Block::default())
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .marker(Marker::Braille)
            .paint(move |ctx| paint_logo(self.id, ctx, accent));

        canvas.render(inner, buf);
    }
}

fn paint_logo(id: &str, ctx: &mut Context<'_>, accent: Color) {
    match id {
        "arch" => paint_arch(ctx, accent),
        "ubuntu" => paint_ubuntu(ctx, accent),
        "debian" => paint_debian(ctx, accent),
        "fedora" => paint_fedora(ctx, accent),
        "manjaro" => paint_manjaro(ctx, accent),
        "opensuse" => paint_opensuse(ctx, accent),
        "void" => paint_void(ctx, accent),
        "nixos" => paint_nixos(ctx, accent),
        "gentoo" => paint_gentoo(ctx, accent),
        "mint" => paint_mint(ctx, accent),
        "pop" => paint_pop(ctx, accent),
        "endeavouros" => paint_endeavour(ctx, accent),
        _ => paint_generic(ctx, accent),
    }
}

fn paint_arch(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Line::new(38.0, 88.0, 50.0, 12.0, c));
    ctx.draw(&Line::new(50.0, 12.0, 62.0, 88.0, c));
    ctx.draw(&Line::new(42.0, 58.0, 58.0, 58.0, c));
    ctx.draw(&Line::new(50.0, 12.0, 50.0, 88.0, Color::Rgb(120, 120, 120)));
}

fn paint_ubuntu(ctx: &mut Context<'_>, c: Color) {
    let dots = Color::Rgb(250, 250, 250);
    ctx.draw(&Circle { x: 50.0, y: 50.0, radius: 8.0, color: c });
    ctx.draw(&Circle { x: 50.0, y: 28.0, radius: 5.5, color: dots });
    ctx.draw(&Circle { x: 35.0, y: 62.0, radius: 5.5, color: dots });
    ctx.draw(&Circle { x: 65.0, y: 62.0, radius: 5.5, color: dots });
    for i in 0..3 {
        let angle = (i as f64) * 2.1;
        let x = 50.0 + angle.cos() * 18.0;
        let y = 50.0 + angle.sin() * 18.0;
        ctx.draw(&Line::new(50.0, 50.0, x, y, Color::Rgb(90, 90, 90)));
    }
}

fn paint_debian(ctx: &mut Context<'_>, c: Color) {
    let mut coords = Vec::new();
    for i in 0..120 {
        let t = i as f64 * 0.12;
        let x = 50.0 + 28.0 * t.cos() * (1.0 + 0.08 * t.sin());
        let y = 50.0 + 28.0 * t.sin() * (1.0 + 0.08 * t.cos());
        coords.push((x, y));
    }
    ctx.draw(&Points { coords: &coords, color: c });
}

fn paint_fedora(ctx: &mut Context<'_>, c: Color) {
    let mut coords = Vec::new();
    for i in 0..100 {
        let t = i as f64 * 0.125;
        let x = 50.0 + 30.0 * (t.cos() / (1.0 + t.sin().powi(2)));
        let y = 50.0 + 22.0 * (t.sin() * t.cos()) / (1.0 + t.sin().powi(2));
        coords.push((x, y));
    }
    ctx.draw(&Points { coords: &coords, color: c });
}

fn paint_manjaro(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Rectangle { x: 22.0, y: 22.0, width: 18.0, height: 56.0, color: c });
    ctx.draw(&Rectangle { x: 41.0, y: 22.0, width: 18.0, height: 56.0, color: c });
    ctx.draw(&Rectangle { x: 60.0, y: 22.0, width: 18.0, height: 56.0, color: c });
    ctx.draw(&Line::new(22.0, 50.0, 78.0, 50.0, Color::Rgb(30, 30, 30)));
}

fn paint_opensuse(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Circle { x: 50.0, y: 48.0, radius: 26.0, color: c });
    ctx.draw(&Circle { x: 50.0, y: 48.0, radius: 14.0, color: Color::Rgb(18, 18, 18) });
    ctx.draw(&Line::new(50.0, 22.0, 50.0, 74.0, Color::Rgb(250, 250, 250)));
    ctx.draw(&Line::new(24.0, 48.0, 76.0, 48.0, Color::Rgb(250, 250, 250)));
}

fn paint_void(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Line::new(28.0, 20.0, 50.0, 80.0, c));
    ctx.draw(&Line::new(72.0, 20.0, 50.0, 80.0, c));
    ctx.draw(&Line::new(34.0, 52.0, 66.0, 52.0, Color::Rgb(250, 250, 250)));
}

fn paint_nixos(ctx: &mut Context<'_>, c: Color) {
    for arm in 0..6 {
        let base = arm as f64 * std::f64::consts::PI / 3.0;
        let x1 = 50.0 + 32.0 * base.cos();
        let y1 = 50.0 + 32.0 * base.sin();
        let x2 = 50.0 + 14.0 * (base + 0.5).cos();
        let y2 = 50.0 + 14.0 * (base + 0.5).sin();
        ctx.draw(&Line::new(x1, y1, x2, y2, c));
        ctx.draw(&Line::new(50.0, 50.0, x1, y1, Color::Rgb(100, 100, 100)));
    }
    ctx.draw(&Circle { x: 50.0, y: 50.0, radius: 4.0, color: Color::Rgb(250, 250, 250) });
}

fn paint_gentoo(ctx: &mut Context<'_>, c: Color) {
    let mut coords = Vec::new();
    for i in 0..80 {
        let t = i as f64 * 0.16;
        coords.push((50.0 + 24.0 * t.cos(), 50.0 + 16.0 * t.sin()));
    }
    ctx.draw(&Points { coords: &coords, color: c });
    ctx.draw(&Circle { x: 50.0, y: 50.0, radius: 6.0, color: Color::Rgb(250, 250, 250) });
}

fn paint_mint(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Rectangle { x: 28.0, y: 30.0, width: 44.0, height: 40.0, color: c });
    ctx.draw(&Line::new(28.0, 50.0, 72.0, 50.0, Color::Rgb(18, 18, 18)));
    ctx.draw(&Circle { x: 40.0, y: 40.0, radius: 4.0, color: Color::Rgb(250, 250, 250) });
    ctx.draw(&Circle { x: 60.0, y: 40.0, radius: 4.0, color: Color::Rgb(250, 250, 250) });
}

fn paint_pop(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Circle { x: 38.0, y: 50.0, radius: 18.0, color: c });
    ctx.draw(&Circle { x: 62.0, y: 50.0, radius: 18.0, color: Color::Rgb(250, 250, 250) });
    ctx.draw(&Rectangle { x: 44.0, y: 44.0, width: 12.0, height: 12.0, color: Color::Rgb(18, 18, 18) });
}

fn paint_endeavour(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Line::new(50.0, 15.0, 25.0, 85.0, c));
    ctx.draw(&Line::new(50.0, 15.0, 75.0, 85.0, c));
    ctx.draw(&Line::new(32.0, 60.0, 68.0, 60.0, c));
    ctx.draw(&Line::new(50.0, 15.0, 50.0, 85.0, Color::Rgb(120, 120, 120)));
}

fn paint_generic(ctx: &mut Context<'_>, c: Color) {
    ctx.draw(&Circle { x: 50.0, y: 38.0, radius: 16.0, color: c });
    ctx.draw(&Circle { x: 44.0, y: 36.0, radius: 2.5, color: Color::Rgb(18, 18, 18) });
    ctx.draw(&Circle { x: 56.0, y: 36.0, radius: 2.5, color: Color::Rgb(18, 18, 18) });
    ctx.draw(&Circle { x: 50.0, y: 44.0, radius: 2.0, color: Color::Rgb(18, 18, 18) });
    ctx.draw(&Rectangle { x: 34.0, y: 54.0, width: 32.0, height: 22.0, color: c });
    ctx.draw(&Line::new(28.0, 62.0, 18.0, 72.0, c));
    ctx.draw(&Line::new(72.0, 62.0, 82.0, 72.0, c));
}