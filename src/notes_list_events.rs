use tui::widgets::ListState;

// Struct that handles the state of the list of user's saved notes
pub struct NoteListEvents {
    pub state: ListState,
    // vec of (String, String) that contains the info of the notes.
    //           id     name
    pub notes: Vec<(String, String)>,
}

impl NoteListEvents {
    pub fn new() -> Self {
        NoteListEvents {
            notes: vec![],
            state: ListState::default(),
        }
    }

    pub fn add_note(&mut self, note_id: String, note_name: String) {
        self.notes.push((note_id, note_name));
    }

    pub fn next(&mut self) {
        let index = match self.state.selected() {
            Some(idx) => {
                if idx >= self.notes.len() - 1 {
                    0
                } else {
                    idx + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }

    pub fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(idx) => {
                if idx <= 0 {
                    self.notes.len() - 1
                } else {
                    idx - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(index));
    }
}