mod helpers;
mod scanner;
mod ui;

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::stdout;
use ui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // terminal setup
    println!("setting up terminal...");
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal");

    // app setup
    println!("setting up app...");
    let mut app = App::new();
    match app.run(&mut terminal) {
        Ok(_) => {}
        Err(e) => println!("error: {e}"),
    };

    /*
    let path = Path::new(r"C:\Users");
    let res = scan_users(path);
     */

    Ok(())
}
