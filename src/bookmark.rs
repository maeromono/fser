use std::{
    collections::HashMap,
    fs::{File, create_dir_all, read_to_string, rename},
    io::{Error, ErrorKind, Result, Write},
    path::PathBuf,
};

use crossterm::event::{KeyCode, KeyEvent};
use directories::ProjectDirs;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::List,
};
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

use crate::fser::{Fser, FserState};

#[derive(Serialize, Deserialize, Default)]
pub struct FserSave {
    pub last_state: HashMap<String, Bookmark>, // automatically created bookmarks
    pub bookmarks: HashMap<String, Vec<Bookmark>>, // manually created bookmarks
                                               // key = path to ebook btw
}

#[derive(Serialize, Deserialize, Default)]
pub struct Bookmark {
    pub page_index: usize,
    pub scroll: u16,
}

enum BookmarkAction {
    FirstPage,
    ContinueReading,
    Bookmark { page_index: usize, scroll: u16 },
}

fn build_bookmarks(fser: &Fser) -> (Vec<String>, Vec<BookmarkAction>) {
    let mut items = vec!["First page".to_string()];
    let mut actions = vec![BookmarkAction::FirstPage];
    if fser
        .bookmarks
        .last_state
        .get(&fser.ebook_path)
        .map(|s| s.page_index > 0 || s.scroll > 0)
        .unwrap_or(false)
    {
        items.push("Continue reading".to_string());
        actions.push(BookmarkAction::ContinueReading);
    }
    if let Some(bookmarks) = fser.bookmarks.bookmarks.get(&fser.ebook_path) {
        for b in bookmarks {
            items.push(format!("Bookmark {}/{}", b.page_index + 1, b.scroll));
            actions.push(BookmarkAction::Bookmark {
                page_index: b.page_index,
                scroll: b.scroll,
            });
        }
    }
    (items, actions)
}

pub fn render_bookmarks(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let (items, _) = build_bookmarks(fser);
    let bm_area = area.centered(Constraint::Max(50), Constraint::Percentage(30));
    let list = List::new(items).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(list, bm_area, &mut fser.bookmarks_list);
}

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('B') | KeyCode::Esc => fser.state = FserState::Reading,
        KeyCode::Char('j') | KeyCode::Down => fser.bookmarks_list.select_next(),
        KeyCode::Char('k') | KeyCode::Up => fser.bookmarks_list.select_previous(),
        KeyCode::Enter => {
            let (_, actions) = build_bookmarks(fser);
            if let Some(selected) = fser.bookmarks_list.selected()
                && let Some(action) = actions.get(selected)
            {
                match action {
                    BookmarkAction::FirstPage => {
                        if fser.page_index != 0 {
                            fser.jump_to_chapter(0);
                        }
                        fser.scroll = 0;
                    }
                    BookmarkAction::ContinueReading => {
                        if let Some(state) = fser.bookmarks.last_state.get(&fser.ebook_path) {
                            let (page_index, scroll) = (state.page_index, state.scroll);
                            fser.jump_to_chapter(page_index);
                            fser.scroll = scroll;
                        }
                    }
                    BookmarkAction::Bookmark { page_index, scroll } => {
                        fser.jump_to_chapter(*page_index);
                        fser.scroll = *scroll;
                    }
                }
            }
            fser.bookmarks_list.select(Some(0));
            fser.state = FserState::Reading;
        }
        _ => {}
    }
}

fn save_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("sh.surge", "maeromono", "fser")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "could not determine data directory"))?;
    Ok(dirs.data_dir().join("bookmarks.toml"))
}

pub fn save_bookmarks(save: &FserSave) -> Result<()> {
    let path = save_path()?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(save).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let tmp_path = path.with_extension("tmp");
    let mut tmp = File::create(&tmp_path)?;
    tmp.write_all(contents.as_bytes())?;
    tmp.sync_all()?;
    rename(&tmp_path, &path)?;
    Ok(())
}

pub fn load_bookmarks() -> Result<FserSave> {
    let path = save_path()?;
    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(FserSave::default()),
        Err(err) => return Err(err),
    };
    match from_str(&contents) {
        Ok(fsersave) => Ok(fsersave),
        Err(err) => {
            let _ = rename(&path, path.with_extension("corrupt"));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}
