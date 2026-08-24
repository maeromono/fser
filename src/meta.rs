use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, widgets::Paragraph};

use crate::fser::{Fser, FserState};

pub fn render_meta(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let ebook_pubilished = if let Some(published) = fser.ebook.metadata().published() {
        published.date().to_string()
    } else {
        String::from("Unknown")
    };
    let ebook_authors = fser.ebook.metadata().creators().map(|creator| creator.value().to_string()).collect::<Vec<_>>().join(", ");
    let ebook_contribs = fser.ebook.metadata().contributors().map(|creator| creator.value().to_string()).collect::<Vec<_>>().join(", ");
    let meta_par = Paragraph::new(format!("Published: {}\nAuthors: {}\nContributors: {}", ebook_pubilished, ebook_authors, ebook_contribs));
    frame.render_widget(meta_par, area);
}

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('m') => fser.state = FserState::Reading,
        _ => {}
    }
}
