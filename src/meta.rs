use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Wrap},
};

use crate::fser::{Fser, FserState};

pub fn render_meta(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let meta = fser.ebook.metadata();
    let description = meta
        .description()
        .map(|desc| desc.value().to_string())
        .unwrap_or_else(|| "no description".to_string());
    let published = meta
        .published()
        .map(|publ| publ.date().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let modified = meta
        .modified()
        .map(|modi| modi.date().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let creators = meta
        .creators()
        .map(|creator| creator.value().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let creators = if creators.is_empty() {
        "unknown".to_string()
    } else {
        creators
    };
    let contribs = meta
        .contributors()
        .map(|contrib| contrib.value().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let contribs = if contribs.is_empty() {
        "none or unknown".to_string()
    } else {
        contribs
    };
    let publishers = meta
        .publishers()
        .map(|publer| publer.value().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let publishers = if publishers.is_empty() {
        "none or unknown".to_string()
    } else {
        publishers
    };
    let language = meta
        .language()
        .map(|lang| lang.value().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let meta_par = Paragraph::new(format!(
        "{}\n\nPublished: {}\nModified: {}\nAuthors: {}\nPublishers: {}\nContributors: {}\n\nLanguage: {}",
        description, published, modified, creators, publishers, contribs, language
    )).wrap(Wrap { trim: true });
    frame.render_widget(meta_par, area);
}

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('m') | KeyCode::Esc => fser.state = FserState::Reading,
        _ => {}
    }
}
