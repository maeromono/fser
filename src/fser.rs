use std::io::{Error, Result};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph},
};
use rbook::Epub;

use crate::{
    bookmark::{self, Bookmark, FserSave, render_bookmarks, save_bookmarks},
    fsermain::{render_footer, render_header},
    meta::{self, render_meta},
    reader::{self, render_reader},
};

pub struct Fser {
    pub ebook: Epub,
    pub ebook_path: String,
    pub bookmarks: FserSave,
    pub bookmarks_list: ListState,
    pub scroll: u16,
    pub current_content: Option<String>,
    pub cached_lines: Option<Vec<String>>,
    pub cached_width: u16,
    pub state: FserState,
    pub page_index: usize,
    pub bookmark_just_added: (bool, bool, Option<Error>), // for visual feedback (couldn't come up with a better solution)
    // 0: true for displaying, 1: true if added, false if removed, 2: error message
    pub ebook_title: String,
}

#[derive(PartialEq, Eq)]
pub enum FserState {
    Bookmark,
    Reading,
    Meta,
    Quit,
}

impl Fser {
    pub fn new(ebook: Epub, path: String) -> Self {
        let ebook_title = if let Some(title) = ebook.metadata().title() {
            title.value().to_string()
        } else {
            "Title-less book".to_string()
        };
        Self {
            ebook,
            ebook_path: path,
            bookmarks: FserSave::default(),
            bookmarks_list: ListState::default().with_selected(Some(0)),
            scroll: 0,
            current_content: None,
            cached_lines: None,
            cached_width: 0,
            state: FserState::Reading,
            page_index: 0,
            bookmark_just_added: (false, false, None),
            ebook_title,
        }
    }

    pub fn spine_len(&self) -> usize {
        self.ebook.spine().len()
    }

    pub fn load_chapter(&mut self) -> Option<String> {
        let mut reader = self.ebook.reader();
        let content = reader.read(self.page_index).ok()?;
        Some(content.content().to_owned())
    }

    pub fn jump_to_chapter(&mut self, index: usize) {
        if index >= self.spine_len() {
            return;
        }
        self.page_index = index;
        self.scroll = 0;
        self.refresh_content();
    }

    pub fn next_chapter(&mut self) {
        if self.page_index + 1 < self.spine_len() {
            self.page_index += 1;
            self.scroll = 0;
            self.refresh_content();
        }
    }

    pub fn prev_chapter(&mut self) {
        if self.page_index > 0 {
            self.page_index -= 1;
            self.scroll = 0;
            self.refresh_content();
        }
    }

    fn refresh_content(&mut self) {
        if let Some(content) = self.load_chapter() {
            self.current_content = Some(content);
        }
        self.cached_lines = None;
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state != FserState::Quit {
            term.draw(|frame| self.render(frame))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    self.handle_keys(&key);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        if area.width < 40 || area.height < 25 {
            // vv terminal too small text
            let ttst = Paragraph::new(format!(
                "Terminal too small\nRequired 40x25\nCurrent {}x{}",
                area.width, area.height
            ))
            .red()
            .centered();
            frame.render_widget(ttst, area);
        } else {
            let main_layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);
            render_header(self, frame, main_layout[0]);
            match self.state {
                FserState::Bookmark => render_bookmarks(self, frame, main_layout[1]),
                FserState::Reading => render_reader(self, frame, main_layout[1]),
                FserState::Meta => render_meta(self, frame, main_layout[1]),
                _ => {}
            }
            render_footer(self, frame, main_layout[2]);
        }
    }

    pub fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.save_and_quit(),
            _ => match self.state {
                FserState::Bookmark => bookmark::handle_keys(self, key),
                FserState::Reading => reader::handle_keys(self, key),
                FserState::Meta => meta::handle_keys(self, key),
                _ => {}
            },
        };
    }

    fn save_and_quit(&mut self) {
        self.bookmarks
            .last_state
            .entry(self.ebook_path.clone())
            .and_modify(|bm| {
                bm.page_index = self.page_index;
                bm.scroll = self.scroll;
            })
            .or_insert(Bookmark {
                page_index: self.page_index,
                scroll: self.scroll,
            });
        if let Err(err) = save_bookmarks(&self.bookmarks) {
            eprintln!("Failed to save bookmarks: {}", err);
        }
        self.state = FserState::Quit;
    }
}
