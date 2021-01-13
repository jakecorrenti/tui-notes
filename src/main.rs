use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io::{self, Write};
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod db;
mod events;
mod note;
mod notes_list_events;
mod types;
mod ui;
use notes_list_events::NoteListEvents;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::create_notes_table()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = NoteListEvents::new();

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    list_state.state.select(Some(0));
    loop {
        terminal.draw(|f| {
            ui::draw(f, &mut list_state);
        })?;

        events::keyboard::handle_notes_list_events(&mut list_state)?;
        list_state.notes.clear();
    }
}
