use super::{db, NoteListEvents, Note};
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
                        KeyCode::Char('d') => {
                            if let Some(selected_note) = list_state.selected_note_id() {
                                let note = db::get_note(selected_note)?;
                                db::delete_note(note)?;
                            }
                        },
                        KeyCode::Char('n') => {
                            db::insert_note(Note::new())?;
                        },
                        KeyCode::Char('w') => {
                            if let Some(selected_note) = list_state.selected_note_id() {
                                let note = db::get_note(selected_note)?;
                                db::update_note(note)?;
                            }
                        },
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(stdout(), LeaveAlternateScreen)?;
                            std::process::exit(1)
                        }
                        _ => (),
                    }
                } else {
                    // the keys that are then pressed are stored in the state of the application
                    // and will be stored in the intput view for editing a specific note
                }
            }
            _ => (),
        }
    }
    Ok(())
}
