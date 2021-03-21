use super::note::Note;
use std::default::Default;
use tui::widgets::ListState;

#[derive(Debug)]
pub enum FocusedBlock {
    TITLE,
    CONTENTS,
}

impl Default for FocusedBlock {
    fn default() -> Self {
        FocusedBlock::TITLE
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub list_state: ListState,
    pub current_content_chars: Vec<char>,
    pub current_title_chars: Vec<char>,
    pub notes: Vec<Note>,
    pub cursor: usize,
    pub focused: FocusedBlock,
}

impl AppState {
    pub fn next_note(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected >= self.notes.len() - 1 {
                self.list_state.select(Some(0));
            } else {
                self.list_state.select(Some(selected + 1));
            }
        } else {
            self.list_state.select(Some(0));
        }

        self.set_current_content_chars();
        self.set_current_title_chars();
    }

    pub fn previous_note(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected <= 0 {
                self.list_state.select(Some(self.notes.len() - 1));
            } else {
                self.list_state.select(Some(selected - 1));
            }
        } else {
            self.list_state.select(Some(0));
        }

        self.set_current_title_chars();
        self.set_current_content_chars();
    }

    pub fn selected_note_id(&self) -> Option<String> {
        if self.notes.len() <= 0 {
            return None;
        }
        if let Some(idx) = self.list_state.selected() {
            return Some(self.notes[idx].id.clone());
        }
        None
    }

    pub fn content_cursor_loc(&mut self, frame_width: usize) -> (u16, u16) {
        let mut pos = (0, 0);

        let x = pos.0 as u16;
        let y = pos.1 as u16;
        (x, y)
    }

    pub fn set_current_content_chars(&mut self) {
        self.current_content_chars.clear();
        if let Some(id) = self.selected_note_id() {
            let note = crate::db::get_note(id).expect(
                "Unable to access the current selected note"
            );
            note.contents.chars().for_each(|c| {
                self.current_content_chars.push(c);
            });
        }
    }

    pub fn set_current_title_chars(&mut self) {
        self.current_title_chars.clear();
        if let Some(id) = self.selected_note_id() {
            let note = crate::db::get_note(id).expect(
                "Unable to access the current selected note"
            );
            note.title.chars().for_each(|c| {
                self.current_title_chars.push(c);
            });
        }
    }

    pub fn add_character_to_content(
        &mut self,
        character: char,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.current_content_chars.push(character);
        self.sync_contents()
    }

    pub fn add_character_to_title(
        &mut self,
        character: char,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.current_title_chars.push(character);
        self.sync_title()
    }

    pub fn rmv_character_from_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_content_chars.pop();
        self.sync_contents()
    }

    pub fn rmv_character_from_title(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_title_chars.pop();
        self.sync_title()
    }

    fn sync_title(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::db;
        if let Some(selected_note) = self.selected_note_id() {
            let note = db::get_note(selected_note)?;
            let mut updated_title = String::new();
            self.current_title_chars.iter().for_each(|character| {
                updated_title.push(*character);
            });
            db::update_title(updated_title, note)?;
        }

        Ok(())
    }

    fn sync_contents(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::db;
        if let Some(selected_note) = self.selected_note_id() {
            let note = db::get_note(selected_note)?;
            let mut updated_contents = String::new();
            self.current_content_chars.iter().for_each(|character| {
                updated_contents.push(*character);
            });
            db::update_contents(updated_contents, note)?;
        }

        Ok(())
    }
}
