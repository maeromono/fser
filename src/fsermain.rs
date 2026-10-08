use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::fser::Fser;

pub fn render_header(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let header = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)]).split(area);
    let name_and_ver = Paragraph::new("fser v0.5.3");
    let ebook_title = Paragraph::new(fser.ebook_title.clone()).right_aligned();
    frame.render_widget(name_and_ver, header[0]);
    frame.render_widget(ebook_title, header[1]);
}

pub fn render_footer(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let footer = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(9),
        Constraint::Fill(1),
    ])
    .split(area);
    render_bookmark_status(fser, frame, footer[0]);
    render_page_progress(fser, frame, footer[1]);
    render_read_progress(fser, frame, footer[2]);
}

fn render_bookmark_status(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        if let Some(message) = fser.footer_notif.clone() {
            fser.footer_notif = None;
            Paragraph::new(message).bold()
        } else {
            Paragraph::new(format!("{}", fser.state))
        },
        area,
    );
}

fn render_page_progress(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let position_track =
        Paragraph::new(format!("{}/{}", fser.page_index + 1, fser.spine_len())).centered();
    frame.render_widget(position_track, area);
}

fn render_read_progress(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let progress = Paragraph::new(format!("{}Ln", fser.scroll)).right_aligned();
    frame.render_widget(progress, area);
}
