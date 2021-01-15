use super::note::Note;
use tui::widgets::ListState;

#[derive(Debug, Default)]
pub struct AppState {
    pub list_state: ListState,
    pub current_note_chars: Vec<char>,
    pub notes: Vec<Note>,
    pub cursor: usize,
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
    }

    pub fn selected_note_id(&self) -> Option<String> {
        if let Some(idx) = self.list_state.selected() {
            return Some(self.notes[idx].id.clone());
        }
        None
    }

    pub fn cursor_loc(&mut self, frame_width: usize) -> (u16, u16) {
        let mut pos = (0, 0);
        pos
    }
}
