use std::io::Cursor;

use html2text::from_read;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Wrap},
};

use crate::fser::Fser;

pub fn render_reader(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let Some(content) = fser.current_content() else {
        return;
    };
    let Ok(parsed) = from_read(Cursor::new(content.as_bytes()), area.width as usize) else {
        return;
    };
    let content = Paragraph::new(parsed)
        .wrap(Wrap { trim: true })
        .scroll((fser.scroll, 0));
    frame.render_widget(content, area);
}
