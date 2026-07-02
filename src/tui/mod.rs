mod draw;
mod print;

use crate::render::RenderContext;
use draw::draw_frame;
use print::print_buffer;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::io;

pub fn render_ratatui(ctx: &RenderContext<'_>) -> io::Result<()> {
    let width = draw::content_width(ctx);
    let height = draw::content_height(ctx);
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|frame| draw_frame(frame, ctx))?;
    print_buffer(terminal.backend().buffer());
    println!();
    Ok(())
}