use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Wrap},
};

use crate::fser::Fser;

pub fn render_reader(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let ass = Paragraph::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque malesuada ante vel risus hendrerit ultrices.").wrap(Wrap {trim: true});
    frame.render_widget(ass, area);
}
