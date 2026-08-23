use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout}, style::Stylize, widgets::Paragraph};

pub struct Fser {
    ebook: String,
    page_index: i32,
    state: FserState,
}

#[derive(PartialEq, Eq)]
enum FserState {
    Reading,
    Meta,
    Quit,
}

impl Fser {
    pub fn new() -> Self {
        Self {
            ebook: "hi".to_string(),
            page_index: 0,
            state: FserState::Reading,
        }
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

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        if area.width < 40 || area.height < 20 {
            // vv terminal too small text
            let ttst = Paragraph::new(format!("Terminal too small\nRequired 40x20\nCurrent {}x{}", area.width, area.height)).red().centered();
            frame.render_widget(ttst, area);
        } else {
            let main_layout = Layout::vertical([Constraint::Min(1), Constraint::Fill(1)]).split(area);
            let header = Layout::horizontal([Constraint::Min(15), Constraint::Fill(1)]).split(main_layout[0]);
            let name_and_ver = Paragraph::new("fser v0.1.0").bold();
            let ebook_title = Paragraph::new(self.ebook.clone()).right_aligned();
            frame.render_widget(name_and_ver, header[0]);
            frame.render_widget(ebook_title, header[1]);
        }
    }

    pub fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.state = FserState::Quit,
            _ => match self.state {
                FserState::Reading => {}
                _ => todo!()
            }
        };
    }
}
