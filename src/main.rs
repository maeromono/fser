mod bookmark;
mod fser;
mod fsermain;
mod meta;
mod reader;

use std::{env::args, path::PathBuf, process::exit};

use ratatui::run;
use rbook::Epub;

use crate::{
    bookmark::{FserSave, load_bookmarks},
    fser::{Fser, FserState},
};

fn main() {
    let mut args = args().skip(1);
    let Some(da_sus_path) = args.next() else {
        eprintln!("No path was provided.\nUsage: fser (path to epub)");
        exit(1);
    };
    if args.next().is_some() {
        eprintln!("Provide ONLY a path to an EPUB file.");
        exit(1);
    }
    let da_path = PathBuf::from(da_sus_path);
    if !da_path.exists() {
        eprintln!("Not a valid path.");
        exit(1);
    }
    let Ok(ebook) = Epub::open(&da_path) else {
        eprintln!("Not a valid EPUB file.");
        exit(1);
    };
    let path_string = da_path.to_string_lossy().into_owned();
    let mut fser = Fser::new(ebook, path_string.clone());
    fser.bookmarks = load_bookmarks().unwrap_or_else(|err| {
        eprintln!(
            "Failed to load bookmarks, falling back to defaults. Details: {}",
            err
        );
        FserSave::default()
    });
    let has_saved_state = fser
        .bookmarks
        .last_state
        .get(&path_string)
        .map(|s| s.page_index > 0 || s.scroll > 0)
        .unwrap_or(false)
        || fser
            .bookmarks
            .bookmarks
            .get(&path_string)
            .map(|b| !b.is_empty())
            .unwrap_or(false);

    if has_saved_state {
        fser.state = FserState::Bookmark;
    }
    fser.current_content = fser.load_chapter();
    if let Err(err) = run(|term| fser.run(term)) {
        eprintln!("Terminal error: {}", err);
        exit(1);
    }
}
