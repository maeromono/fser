use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph},
};
use rbook::Epub;

use crate::{bookmark::{self, Bookmark, FserSave, render_bookmarks, save_bookmarks}, meta::{self, render_meta}, reader::{self, render_reader}};

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
    ebook_title: String,
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
        if index + 1 > self.spine_len() {
            return;
        }
        self.page_index = index;
        self.scroll = 0;
        self.current_content = self.load_chapter();
        self.cached_lines = None;
    }

    pub fn next_chapter(&mut self) {
        if self.page_index + 1 < self.spine_len() {
            self.page_index += 1;
            self.scroll = 0;
            self.current_content = self.load_chapter();
            self.cached_lines = None;
        }
    }

    pub fn prev_chapter(&mut self) {
        self.page_index = self.page_index.saturating_sub(1);
        self.scroll = 0;
        self.current_content = self.load_chapter();
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
        if area.width < 40 || area.height < 20 {
            // vv terminal too small text
            let ttst = Paragraph::new(format!(
                "Terminal too small\nRequired 40x20\nCurrent {}x{}",
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
            let header = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)])
                .split(main_layout[0]);
            let name_and_ver = Paragraph::new("fser v0.1.0").bold();
            let ebook_title = Paragraph::new(self.ebook_title.clone()).right_aligned();
            let position_track =
                Paragraph::new(format!("{}/{}", self.page_index + 1, self.spine_len())).centered();
            frame.render_widget(name_and_ver, header[0]);
            frame.render_widget(ebook_title, header[1]);
            frame.render_widget(position_track, main_layout[2]);
            match self.state {
                FserState::Bookmark => render_bookmarks(self, frame, main_layout[1]),
                FserState::Reading => render_reader(self, frame, main_layout[1]),
                FserState::Meta => render_meta(self, frame, main_layout[1]),
                _ => {}
            }
        }
    }

    pub fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.save_and_quit(),
            _ => match self.state {
                FserState::Bookmark => bookmark::handle_keys(self, key),
                FserState::Reading => reader::handle_keys(self, &key),
                FserState::Meta => meta::handle_keys(self, &key),
                _ => {}
            },
        };
    }

    fn save_and_quit(&mut self) {
        self.bookmarks.last_state.entry(self.ebook_path.clone()).and_modify(|bm| {
            bm.page_index = self.page_index;
            bm.scroll = self.scroll;
        }).or_insert(Bookmark { page_index: self.page_index, scroll: self.scroll });
        if let Err(err) = save_bookmarks(&self.bookmarks) {
            eprintln!("Failed to save bookmarks: {}", err);
        }
        self.state = FserState::Quit;
    }
}
