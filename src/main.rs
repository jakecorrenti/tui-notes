use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod db;
mod note;
mod types;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::create_notes_table()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    // TODO way to exit the program using a keyboard shortcut
    loop {
        terminal.draw(|f| {
            ui::draw(f);
        })?;
    }
    /* disable_raw_mode()?;
     * execute!(std::io::stdout(), LeaveAlternateScreen)?; */
}
