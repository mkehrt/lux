use anyhow::{bail, Result};

#[derive(Default)]
pub struct Data {
    pub strings: Vec<String>,
    pub cursor: Cursor,
}

impl Data {
    pub fn write_to_data(&mut self, text: &str) -> Result<()> {
        while self.strings.len() <= self.cursor.row {
            self.strings.push(String::new());
        }

        let row = &mut self.strings[self.cursor.row];
        let mut chars: Vec<char> = row.chars().collect();

        if self.cursor.column > chars.len() {
            bail!(
                "cannot write at column {} in row {}, which only has {} columns",
                self.cursor.column,
                self.cursor.row,
                chars.len()
            );
        }

        let inserted_len = text.chars().count();
        for (offset, c) in text.chars().enumerate() {
            chars.insert(self.cursor.column + offset, c);
        }

        *row = chars.into_iter().collect();

        self.cursor.column += inserted_len;

        Ok(())
    }

    pub fn concatenate(&self) -> String {
        self.strings.concat()
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.row == 0 {
            return;
        }
        self.cursor.row -= 1;
        self.clamp_cursor_column();
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.row + 1 >= self.strings.len() {
            return;
        }
        self.cursor.row += 1;
        self.clamp_cursor_column();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.column == 0 {
            return;
        }
        self.cursor.column -= 1;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor.column >= self.row_len(self.cursor.row) {
            return;
        }
        self.cursor.column += 1;
    }

    fn row_len(&self, row: usize) -> usize {
        self.strings.get(row).map_or(0, |s| s.chars().count())
    }

    fn clamp_cursor_column(&mut self) {
        let len = self.row_len(self.cursor.row);
        if self.cursor.column > len {
            self.cursor.column = len;
        }
    }
}

#[derive(Default)]
pub struct Cursor {
    pub column: usize,
    pub row: usize,
}
