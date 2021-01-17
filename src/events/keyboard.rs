use super::{db, Note, AppState};
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
    state: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    if poll(Duration::from_millis(500))? {
        match read()? {
            Event::Key(event) => {
                if event.modifiers == KeyModifiers::CONTROL {
                    match event.code {
                        KeyCode::Char('j') => {
                            state.next_note();
                        },
                        KeyCode::Char('k') => {
                            state.previous_note();
                        },
                        KeyCode::Char('d') => {
                            if let Some(selected_note) = state.selected_note_id() {
                                let note = db::get_note(selected_note)?;
                                db::delete_note(note)?;
                            }
                        },
                        KeyCode::Char('n') => {
                            db::insert_note(Note::new())?;
                        },
                        KeyCode::Char('w') => {
                            /* if let Some(selected_note) = list_state.selected_note_id() {
                             *     let note = db::get_note(selected_note)?;
                             *     db::update_note(note)?;
                             * } */
                        },
                        KeyCode::Char('t') => {
                            //TODO: focus the input cursor on the title 
                        },
                        KeyCode::Char('c') => {
                            //TODO: focus the input cursor on the contents
                        },
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(stdout(), LeaveAlternateScreen)?;
                            std::process::exit(1)
                        },
                        _ => (),
                    }
                } else {
                    match event.code {
                        //TODO: Implement a simple pop function to remove the last character from
                        //the list of characters in the app state
                        KeyCode::Backspace => (),
                        KeyCode::Char(character) => {
                            state.current_note_chars.push(character);
                            if let Some(selected_note) = state.selected_note_id() {
                                let note = db::get_note(selected_note)?;
                                let mut updated_note_contents = String::new();
                                state.current_note_chars.iter().for_each(|character| {
                                    updated_note_contents.push(*character);
                                });
                                db::update_note(updated_note_contents, note)?;
                            }
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
