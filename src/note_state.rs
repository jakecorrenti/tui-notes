#[derive(Default)]
pub struct NoteState {
    pub chars: Vec<char>,
    cursor: usize,
}

impl NoteState {
    pub fn add_char(&mut self, ch: char) {
        self.chars.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.chars.pop();
    }

    pub fn as_str(&mut self) -> String {
        let mut str = String::new();
        self.chars.iter().for_each(|character| str.push(*character));
        str
    }

    pub fn clear_chars(&mut self) {
        self.chars.clear();
    }

    pub fn cursor_loc(&self, frame_width: usize) -> (u16, u16) {
        let mut cursor_pos = (0, 0);

        // possible create an iter with n elements (n = frame_width)?
        for character in self.chars.iter() {
            // if the character width is None, then set the width to 0 and keep the current
            // location
            let character_width = unicode_width::UnicodeWidthChar::width(*character).unwrap_or(0);
        }
        // loop through each of the individual characters
        // determine their width
        // adjust the cursor x-coordinate accordingly
        // adjust cursor position based in terms of where the cursor is after each iteration in
        // terms of the entire width of the frame

        cursor_pos
    }

}

