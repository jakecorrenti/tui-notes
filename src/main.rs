use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io::{self, Write};
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app_state;
mod db;
mod events;
mod note;
mod ui;

use app_state::AppState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::create_notes_table()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::default();

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    app_state.list_state.select(Some(0));
    loop {
        terminal.draw(|f| {
            ui::draw(f, &mut app_state);
        })?;

        events::keyboard::handle_notes_list_events(&mut app_state)?;
        app_state.notes.clear();
    }
}
