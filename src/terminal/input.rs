use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// A cursor position within a multi-line buffer.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Cursor {
    /// Zero-based line index.
    pub row: usize,
    /// Zero-based column (character) index.
    pub col: usize,
}

/// Represents the editable input buffer supporting multi-line editing.
pub struct InputBuffer {
    /// Lines of text currently being edited.
    pub lines: Vec<String>,
    /// Current cursor position.
    pub cursor: Cursor,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            lines: vec![String::new()],
            cursor: Cursor::default(),
        }
    }

    /// Return the complete text of the buffer joined by newlines.
    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    /// Clear the buffer and reset the cursor.
    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.cursor = Cursor::default();
    }

    /// Insert a character at the current cursor position.
    pub fn insert_char(&mut self, ch: char) {
        let col = self.cursor.col;
        self.lines[self.cursor.row].insert(col, ch);
        self.cursor.col += 1;
    }

    /// Delete the character before the cursor (backspace).
    pub fn backspace(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
            self.lines[self.cursor.row].remove(self.cursor.col);
        } else if self.cursor.row > 0 {
            // Merge current line into the previous line
            let current = self.lines.remove(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
            self.lines[self.cursor.row].push_str(&current);
        }
    }

    /// Insert a newline at the cursor, splitting the current line.
    pub fn newline(&mut self) {
        let rest = self.lines[self.cursor.row].split_off(self.cursor.col);
        self.cursor.row += 1;
        self.cursor.col = 0;
        self.lines.insert(self.cursor.row, rest);
    }

    /// Move the cursor left by one character (wraps to previous line).
    pub fn move_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    /// Move the cursor right by one character (wraps to next line).
    pub fn move_right(&mut self) {
        let line_len = self.lines[self.cursor.row].len();
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    /// Move the cursor up one line, clamping the column.
    pub fn move_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    /// Move the cursor down one line, clamping the column.
    pub fn move_down(&mut self) {
        if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    /// Handle a crossterm key event and return `true` if the input should be submitted.
    pub fn handle_key(&mut self, event: KeyEvent) -> bool {
        match event.code {
            KeyCode::Enter if event.modifiers.contains(KeyModifiers::SHIFT) => {
                // Shift+Enter inserts a literal newline for multi-line editing
                self.newline();
                false
            }
            KeyCode::Enter => true, // submit
            KeyCode::Backspace => {
                self.backspace();
                false
            }
            KeyCode::Left => {
                self.move_left();
                false
            }
            KeyCode::Right => {
                self.move_right();
                false
            }
            KeyCode::Up => {
                self.move_up();
                false
            }
            KeyCode::Down => {
                self.move_down();
                false
            }
            KeyCode::Char(ch) => {
                self.insert_char(ch);
                false
            }
            _ => false,
        }
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_insert_and_text() {
        let mut buf = InputBuffer::new();
        buf.insert_char('h');
        buf.insert_char('i');
        assert_eq!(buf.text(), "hi");
        assert_eq!(buf.cursor.col, 2);
    }

    #[test]
    fn test_backspace() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        buf.insert_char('b');
        buf.backspace();
        assert_eq!(buf.text(), "a");
        assert_eq!(buf.cursor.col, 1);
    }

    #[test]
    fn test_newline_splits_line() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        buf.insert_char('b');
        buf.move_left();
        buf.newline();
        assert_eq!(buf.lines[0], "a");
        assert_eq!(buf.lines[1], "b");
        assert_eq!(buf.cursor.row, 1);
        assert_eq!(buf.cursor.col, 0);
    }

    #[test]
    fn test_backspace_merges_lines() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        buf.newline();
        buf.insert_char('b');
        // cursor is now at row=1, col=1
        // move to start of second line
        buf.cursor.col = 0;
        buf.backspace();
        assert_eq!(buf.lines.len(), 1);
        assert_eq!(buf.lines[0], "ab");
    }

    #[test]
    fn test_move_left_wraps() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        buf.newline();
        buf.cursor.col = 0;
        buf.move_left();
        assert_eq!(buf.cursor.row, 0);
        assert_eq!(buf.cursor.col, 1);
    }

    #[test]
    fn test_handle_key_submit_on_enter() {
        let mut buf = InputBuffer::new();
        buf.insert_char('x');
        let submitted = buf.handle_key(key(KeyCode::Enter));
        assert!(submitted);
    }

    #[test]
    fn test_handle_key_shift_enter_is_newline() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        let ev = KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let submitted = buf.handle_key(ev);
        assert!(!submitted);
        assert_eq!(buf.lines.len(), 2);
    }

    #[test]
    fn test_handle_key_unknown_does_not_modify_buffer() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        let ev = KeyEvent {
            code: KeyCode::F(1),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let submitted = buf.handle_key(ev);
        assert!(!submitted);
        assert_eq!(buf.text(), "a");
    }

    #[test]
    fn test_clear() {
        let mut buf = InputBuffer::new();
        buf.insert_char('a');
        buf.clear();
        assert_eq!(buf.text(), "");
        assert_eq!(buf.cursor, Cursor::default());
    }
}
