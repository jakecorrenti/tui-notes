use super::NoteListEvents;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

pub fn handle_notes_list_events(
    list_state: &mut NoteListEvents,
) -> Result<(), Box<dyn std::error::Error>> {
    if poll(Duration::from_millis(500))? {
        match read()? {
            Event::Key(event) => {
                if event.modifiers == KeyModifiers::CONTROL {
                    match event.code {
                        KeyCode::Char('j') => list_state.next(),
                        KeyCode::Char('k') => list_state.previous(),
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(stdout(), LeaveAlternateScreen)?;
                            std::process::exit(1)
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
