use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::Paragraph,
};

use crate::fser::{Fser, FserState};

const BINDINGS: &[(&str, &str)] = &[
    ("q", "Save progress & quit"),
    ("?/F1", "Bind help (you're here)"),
    ("n/p", "Next/previous page/chapter"),
    ("j/k, Up/Down", "Scroll down/up"),
    ("d/u", "Scroll down/up but faster"),
    ("g/G", "Jump to top/bottom"),
    ("m", "View ebook metadata"),
    ("b", "Add/remove bookmark"),
    ("B", "Open bookmark menu"),
    ("Esc", "Back to reading"),
];

pub fn render_bindhelp(_fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let lines: Vec<Line> = BINDINGS
        .iter()
        .map(|(keys, action)| Line::from(format!("{:<13} {}", keys, action)))
        .collect();
    frame.render_widget(Paragraph::new(lines), area);
}

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('?') | KeyCode::F(1) | KeyCode::Esc => fser.state = FserState::Reading,
        _ => {}
    }
}
