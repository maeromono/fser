use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::{Modifier, Stylize}, widgets::{List, Paragraph}};

use crate::fser::Fser;

pub fn render_error_page(fser: &Fser, frame: &mut Frame, area: Rect) {
    let error_layout = Layout::vertical([Constraint::Max(3), Constraint::Fill(1)]);
    let error_paragraph = Paragraph::new("Some errors have occurred at runtime. Review them, then press Esc to go back to reading.").red().bold();
    let mut error_list: Vec<String> = vec![];
    for error in &fser.errors {
        error_list.push(error.to_string());
    }
    let list = List::new(error_list).highlight_style(Modifier::REVERSED);
}
