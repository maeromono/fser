use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
};
use rbook::{Epub, epub::spine::{EpubSpine, EpubSpineEntry}};

use crate::reader::render_reader;

pub struct Fser {
    pub ebook: Epub,
    pub scroll: u16,
    ebook_title: String,
    page_index: usize,
    state: FserState,
}

#[derive(PartialEq, Eq)]
enum FserState {
    Reading,
    Meta,
    Quit,
}

impl Fser {
    pub fn new(ebook: Epub) -> Self {
        let ebook_title = if let Some(title) = ebook.metadata().title() {
            title.value().to_string()
        } else {
            "Title-less book".to_string()
        };
        Self {
            ebook,
            scroll: 0,
            ebook_title,
            page_index: 0,
            state: FserState::Reading,
        }
    }

    pub fn spine_len(&self) -> usize {
        self.ebook.spine().len()
    }

    pub fn current_content(&self) -> Option<String> {
        let mut reader = self.ebook.reader();
        let content = reader.read(self.page_index).ok()?;
        Some(content.content().to_owned())
    }

    fn next_chapter(&mut self) {
        if self.page_index + 1 < self.spine_len() {
            self.page_index += 1;
        }
    }

    fn prev_chapter(&mut self) {
        self.page_index = self.page_index.saturating_sub(1);
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
            let main_layout =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);
            let header = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)])
                .split(main_layout[0]);
            let name_and_ver = Paragraph::new("fser v0.1.0").bold();
            let ebook_title = Paragraph::new(self.ebook_title.clone()).right_aligned();
            frame.render_widget(name_and_ver, header[0]);
            frame.render_widget(ebook_title, header[1]);
            match self.state {
                FserState::Reading => render_reader(self, frame, main_layout[1]),
                _ => {}
            }
        }
    }

    pub fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.state = FserState::Quit,
            _ => match self.state {
                FserState::Reading => match key.code {
                    KeyCode::Char('n') => self.next_chapter(),
                    KeyCode::Char('p') => self.prev_chapter(),
                    _ => {}
                }
                _ => todo!(),
            },
        };
    }
}
