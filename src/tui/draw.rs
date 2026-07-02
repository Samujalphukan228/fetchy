use crate::info::hardware::Usage;
use crate::logos::LogoCanvas;
use crate::render::{InfoLine, RenderContext, Section};
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

const LOGO_WIDTH: u16 = 44;
const LOGO_HEIGHT: u16 = 22;

pub fn content_width(ctx: &RenderContext<'_>) -> u16 {
    let mut max_line = 0usize;
    for section in ctx.sections {
        for line in &section.lines {
            let len = line.label.len() + line.value.chars().count() + 6;
            max_line = max_line.max(len);
        }
    }
    let info = max_line.max(52) as u16;
    let base = if ctx.show_logo {
        LOGO_WIDTH + info + 6
    } else {
        info + 4
    };
    crossterm::terminal::size()
        .map(|(w, _)| base.min(w))
        .unwrap_or(base)
}

pub fn content_height(ctx: &RenderContext<'_>) -> u16 {
    let info_rows = info_line_count(ctx.sections);
    let panel_rows = info_rows.max(LOGO_HEIGHT as usize);
    // Header (2) + panel + info block bottom padding (2) + footer (2).
    (panel_rows + 6) as u16
}

pub fn draw_frame(frame: &mut Frame, ctx: &RenderContext<'_>) {
    let theme = Theme::void();
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(2),
    ])
    .split(area);

    draw_header(frame, layout[0], ctx, &theme);
    draw_body(frame, layout[1], ctx, &theme);
    draw_footer(frame, layout[2], ctx, &theme);
}

fn draw_header(frame: &mut Frame, area: Rect, ctx: &RenderContext<'_>, theme: &Theme) {
    let title = Line::from(vec![
        Span::styled(ctx.user, theme.header_user()),
        Span::styled("@", theme.header_at()),
        Span::styled(ctx.host, theme.header_host()),
    ]);
    frame.render_widget(Paragraph::new(title), area);

    let rule_area = Rect {
        y: area.y + 1,
        height: 1,
        ..area
    };
    let width = (ctx.user.len() + ctx.host.len() + 1).min(area.width as usize) as u16;
    let rule = "─".repeat(width as usize);
    frame.render_widget(
        Paragraph::new(Span::styled(rule, theme.rule())),
        rule_area,
    );
}

fn draw_body(frame: &mut Frame, area: Rect, ctx: &RenderContext<'_>, theme: &Theme) {
    if ctx.show_logo {
        let cols = Layout::horizontal([
            Constraint::Length(LOGO_WIDTH),
            Constraint::Fill(1),
        ])
        .split(area);
        frame.render_widget(
            LogoCanvas {
                id: ctx.logo_id,
                border_style: theme.logo_border(),
                title_style: theme.logo_title(),
            },
            cols[0],
        );
        draw_info_panel(frame, cols[1], ctx, &theme);
    } else {
        draw_info_panel(frame, area, ctx, &theme);
    }
}

fn draw_info_panel(frame: &mut Frame, area: Rect, ctx: &RenderContext<'_>, theme: &Theme) {
    let lines = build_info_lines(ctx.sections, ctx.separator, theme);
    let block = Block::default()
        .borders(ratatui::widgets::Borders::LEFT)
        .border_type(BorderType::Plain)
        .border_style(theme.rule())
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 2));

    frame.render_widget(
        Paragraph::new(lines).block(block),
        area,
    );
}

fn build_info_lines(sections: &[Section], separator: &str, theme: &Theme) -> Vec<Line<'static>> {
    let max_label = sections
        .iter()
        .flat_map(|s| s.lines.iter())
        .map(|l| l.label.len())
        .max()
        .unwrap_or(0);

    let mut out = Vec::new();
    let mut first = true;

    for section in sections {
        if section.lines.is_empty() {
            continue;
        }
        if !first {
            out.push(Line::from(""));
        }
        first = false;

        out.push(Line::from(Span::styled(
            section.title.to_string(),
            theme.section(),
        )));

        for line in &section.lines {
            out.push(field_line(line, separator, max_label, theme));
            if let Some(usage) = &line.usage {
                out.push(gauge_line(usage, max_label, theme));
            }
        }
    }

    out
}

fn field_line(line: &InfoLine, separator: &str, max_label: usize, theme: &Theme) -> Line<'static> {
    let label = format!("{:>width$}", line.label, width = max_label);
    let value = if let Some(usage) = &line.usage {
        truncate(&usage.compact(), 52)
    } else {
        truncate(&line.value, 56)
    };
    Line::from(vec![
        Span::styled(label, theme.label()),
        Span::styled(format!(" {separator} "), theme.separator()),
        Span::styled(value, theme.value()),
    ])
}

fn truncate(s: &str, max: usize) -> String {
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

fn gauge_line(usage: &Usage, max_label: usize, theme: &Theme) -> Line<'static> {
    let pad = " ".repeat(max_label + 3);
    let ratio = (usage.percent() / 100.0).clamp(0.0, 1.0);
    let label = format!("{:.0}%", usage.percent());
    let width = 20;
    let filled = (ratio * width as f64).round() as usize;
    let filled = filled.min(width);

    Line::from(vec![
        Span::raw(pad),
        Span::styled("█".repeat(filled), theme.gauge_fill()),
        Span::styled("░".repeat(width.saturating_sub(filled)), theme.gauge_empty()),
        Span::styled(format!(" {label}"), theme.footer()),
    ])
}

fn draw_footer(frame: &mut Frame, area: Rect, ctx: &RenderContext<'_>, theme: &Theme) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area);

    let rule = "─".repeat(area.width as usize);
    frame.render_widget(
        Paragraph::new(Span::styled(rule, theme.rule())),
        layout[0],
    );

    let swatches = [
        theme.ghost,
        theme.bar_track,
        theme.muted,
        theme.dim,
        theme.text,
        theme.bar_fill,
        theme.bright,
        theme.surface,
    ];
    let mut spans: Vec<Span> = vec![Span::styled(
        format!("fetchy {}  ", ctx.version),
        theme.footer(),
    )];
    for color in swatches {
        spans.push(Span::styled("  ", Style::default().bg(color)));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), layout[1]);
}

fn info_line_count(sections: &[Section]) -> usize {
    let fields: usize = sections.iter().map(|s| s.lines.len()).sum();
    let meters: usize = sections
        .iter()
        .flat_map(|s| &s.lines)
        .filter(|l| l.usage.is_some())
        .count();
    let headers = sections.iter().filter(|s| !s.lines.is_empty()).count();
    let spacers = headers.saturating_sub(1);
    fields + meters + headers + spacers
}