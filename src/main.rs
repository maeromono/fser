mod fser;

use ratatui::run;

use crate::fser::Fser;

fn main() {
    println!("SAMSUNG SUCKS");
    let mut fser = Fser::new();
    run(|term| fser.run(term));
}
