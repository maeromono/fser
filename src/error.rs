use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

use crate::fser::{Fser, FserState};

pub fn render_error_page(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let error_layout = Layout::vertical([Constraint::Max(2), Constraint::Fill(1)]).split(area);
    let error_paragraph =
        Paragraph::new("An error has occurred. Review it, then press Esc to go back to reading.")
            .red()
            .bold()
            .wrap(Wrap { trim: true });
    let error_message_paragraph = Paragraph::new(if let Some(error) = fser.error.clone() {
        error
    } else {
        String::from("Unknown error")
    })
    .wrap(Wrap { trim: true });
    frame.render_widget(error_paragraph, error_layout[0]);
    frame.render_widget(error_message_paragraph, error_layout[1]);
}

pub fn handle_keys(fser: &mut Fser, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            fser.state = FserState::Reading;
        }
        _ => {}
    }
}
