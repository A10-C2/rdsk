mod helpers;
mod scanner;
mod ui;

use crate::ui::App;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("setting up terminal...");
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal");
    println!("setting up app...");
    let mut app = App::new();
    match app.run(&mut terminal) {
        Ok(_) => {}
        Err(e) => println!("error: {e}"),
    };
    Ok(())
}
