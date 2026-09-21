use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::fser::Fser;

pub fn render_header(fser: &mut Fser, frame: &mut Frame, area: Rect) {
    let header = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)]).split(area);
    let name_and_ver = Paragraph::new("fser v0.4.2").bold();
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
    if fser.bookmark_just_added.0 || fser.bookmark_just_added.3 > 0 {
        let status = if let Some(err) = &fser.bookmark_just_added.2 {
            Paragraph::new(format!("Bookmark added, but won't save: {}", err))
                .red()
                .bold()
        } else {
            Paragraph::new(match fser.bookmark_just_added.1 {
                true => "Bookmark added",
                false => "Bookmark removed",
            })
            .bold()
        };
        frame.render_widget(status, area);
        fser.bookmark_just_added.3 = fser.bookmark_just_added.3.saturating_sub(1);
        if fser.bookmark_just_added.3 == 0 {
            fser.bookmark_just_added = (false, false, None, 0);
        }
    } else {
        frame.render_widget(Paragraph::new(format!("{}", fser.state)), area);
    }
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
