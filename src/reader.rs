use std::io::Cursor;

use html2text::from_read;
use ratatui::{
    Frame, layout::Rect, text::Line, widgets::Paragraph
};

use crate::fser::Fser;

pub fn render_reader(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let Some(content) = &fser.current_content else {
        return;
    };
    if fser.cached_lines.is_none() || fser.cached_width != area.width {
        let Ok(parsed) = from_read(Cursor::new(content.as_bytes()), area.width as usize) else {
            return;
        };
        let lines: Vec<String> = parsed.lines().map(|l| l.to_string()).collect();
        fser.cached_lines = Some(lines);
        fser.cached_width = area.width;
    }
    let Some(cur_lines) = &fser.cached_lines else {
        return;
    };
    let height = area.height as usize;
    let max_scroll = cur_lines.len().saturating_sub(height);
    if fser.scroll as usize > max_scroll {
        fser.scroll = max_scroll as u16;
    }
    let start = fser.scroll as usize;
    let end = (start + height).min(cur_lines.len());
    let visible: Vec<Line> = cur_lines[start..end]
        .iter()
        .map(|l| Line::from(l.as_str()))
        .collect();

    let paragraph = Paragraph::new(visible);
    frame.render_widget(paragraph, area);
}
