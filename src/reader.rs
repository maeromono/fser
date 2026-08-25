use std::{collections::hash_map::Entry, io::Cursor};

use crossterm::event::{KeyCode, KeyEvent};
use html2text::from_read;
use ratatui::{Frame, layout::Rect, style::Stylize, text::Line, widgets::Paragraph};

use crate::{
    bookmark::{Bookmark, save_bookmarks},
    fser::{Fser, FserState},
};

pub fn render_reader(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let Some(content) = &fser.current_content else {
        let error = Paragraph::new("Load failed")
            .red()
            .bold();
        frame.render_widget(error, area);
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

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('n') => fser.next_chapter(),
        KeyCode::Char('p') => fser.prev_chapter(),
        KeyCode::Char('j') | KeyCode::Down => fser.scroll = fser.scroll.saturating_add(1),
        KeyCode::Char('d') => fser.scroll = fser.scroll.saturating_add(5),
        KeyCode::Char('k') | KeyCode::Up => fser.scroll = fser.scroll.saturating_sub(1),
        KeyCode::Char('u') => fser.scroll = fser.scroll.saturating_sub(5),
        KeyCode::Char('m') => fser.state = FserState::Meta,
        KeyCode::Char('b') => add_or_remove_bookmark(fser),
        _ => {}
    };
}

fn add_or_remove_bookmark(fser: &mut Fser) {
    let new_bookmark = Bookmark {
        page_index: fser.page_index,
        scroll: fser.scroll,
    };
    let path = fser.ebook_path.clone();
    match fser.bookmarks.bookmarks.entry(path) {
        Entry::Occupied(mut entry) => {
            let bookmarks = entry.get_mut();
            let before = bookmarks.len();
            bookmarks.retain(|bm| {
                !(bm.page_index == new_bookmark.page_index && bm.scroll == new_bookmark.scroll)
            });
            if bookmarks.len() == before {
                bookmarks.push(new_bookmark);
            }
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![new_bookmark]);
        }
    }
    let _ = save_bookmarks(&fser.bookmarks);
}
