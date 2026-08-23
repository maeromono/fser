mod fser;

use std::{env::args, path::PathBuf, process::exit};

use ratatui::run;
use rbook::Epub;

use crate::fser::Fser;

fn main() {
    println!("SAMSUNG SUCKS");
    let mut args = args().skip(1);
    let Some(da_sus_path) = args.next() else {
        eprintln!("No path was provided.\nUsage: fser (path to epub)");
        exit(1);
    };
    let da_path = PathBuf::from(da_sus_path);
    if args.next().is_some() {
        eprintln!("Provide ONLY a path to an EPUB file.");
        exit(1);
    }
    if !da_path.exists() {
        eprintln!("Not a valid path.");
        exit(1);
    }
    let Ok(ebook) = Epub::open(da_path) else {
        eprintln!("Not a valid EPUB file.");
        exit(1);
    };
    println!("{:?}", ebook);
    let mut fser = Fser::new(ebook);
    run(|term| fser.run(term));
}
