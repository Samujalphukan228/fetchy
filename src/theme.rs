use ratatui::style::{Color, Modifier, Style};

/// VOID-inspired monochrome palette (#121212 / #FAFAFA).
pub struct Theme {
    pub bright: Color,
    pub text: Color,
    pub dim: Color,
    pub muted: Color,
    pub ghost: Color,
    pub rule: Color,
    pub bar_fill: Color,
    pub bar_track: Color,
    pub surface: Color,
}

impl Theme {
    pub fn void() -> Self {
        Self {
            bright: Color::Rgb(255, 255, 255),
            text: Color::Rgb(250, 250, 250),
            dim: Color::Rgb(158, 158, 158),
            muted: Color::Rgb(97, 97, 97),
            ghost: Color::Rgb(66, 66, 66),
            rule: Color::Rgb(56, 56, 56),
            bar_fill: Color::Rgb(224, 224, 224),
            bar_track: Color::Rgb(46, 46, 46),
            surface: Color::Rgb(18, 18, 18),
        }
    }

    pub fn header_user(&self) -> Style {
        Style::default().fg(self.bright).add_modifier(Modifier::BOLD)
    }

    pub fn header_at(&self) -> Style {
        Style::default().fg(self.ghost)
    }

    pub fn header_host(&self) -> Style {
        Style::default().fg(self.text).add_modifier(Modifier::BOLD)
    }

    pub fn section(&self) -> Style {
        Style::default().fg(self.bright).add_modifier(Modifier::BOLD)
    }

    pub fn label(&self) -> Style {
        Style::default().fg(self.dim)
    }

    pub fn separator(&self) -> Style {
        Style::default().fg(self.ghost)
    }

    pub fn value(&self) -> Style {
        Style::default().fg(self.text)
    }

    pub fn rule(&self) -> Style {
        Style::default().fg(self.rule)
    }

    pub fn footer(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn logo_border(&self) -> Style {
        Style::default().fg(self.ghost).bg(self.surface)
    }

    pub fn logo_title(&self) -> Style {
        Style::default().fg(self.dim).add_modifier(Modifier::BOLD)
    }

    pub fn gauge_fill(&self) -> Style {
        Style::default().fg(self.bar_fill).bg(self.bar_track)
    }

    pub fn gauge_empty(&self) -> Style {
        Style::default().fg(self.bar_track)
    }
}