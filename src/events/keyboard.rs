use super::{db, AppState, Note};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

use crate::app_state::FocusedBlock;

pub fn handle_notes_list_events(state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    if poll(Duration::from_millis(500))? {
        match read()? {
            Event::Key(event) => {
                if event.modifiers == KeyModifiers::CONTROL {
                    match event.code {
                        KeyCode::Char('j') => {
                            state.next_note();
                        }
                        KeyCode::Char('k') => {
                            state.previous_note();
                        }
                        KeyCode::Char('d') => {
                            if let Some(selected_note) = state.selected_note_id() {
                                let note = db::get_note(selected_note)?;
                                db::delete_note(note)?;
                            }
                        }
                        KeyCode::Char('n') => {
                            db::insert_note(Note::new())?;
                        }
                        KeyCode::Char('t') => {
                            state.focused = FocusedBlock::TITLE;
                        }
                        KeyCode::Char('c') => {
                            state.focused = FocusedBlock::CONTENTS;
                        }
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(stdout(), LeaveAlternateScreen)?;
                            std::process::exit(1)
                        }
                        _ => (),
                    }
                } else {
                    match event.code {
                        KeyCode::Enter => {
                            // TODO: Implement
                        },
                        KeyCode::Backspace => match state.focused {
                            FocusedBlock::TITLE => {
                                state.rmv_character_from_title()?;
                            }
                            FocusedBlock::CONTENTS => {
                                state.rmv_character_from_content()?;
                            }
                        }
                        KeyCode::Char(character) => match state.focused {
                            FocusedBlock::TITLE => {
                                state.add_character_to_title(character)?;
                            }
                            FocusedBlock::CONTENTS => {
                                state.add_character_to_content(character)?;
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
