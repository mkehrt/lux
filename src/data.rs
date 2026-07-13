#[derive(Default)]
pub struct Data {
    pub strings: Vec<String>,
    pub cursor: Cursor,
}

impl Data {
    pub fn write(&mut self, text: &str) {
        while self.strings.len() <= self.cursor.row {
            self.strings.push(String::new());
        }

        let row = &mut self.strings[self.cursor.row];
        let mut chars: Vec<char> = row.chars().collect();

        assert!(
            self.cursor.logical_column <= chars.len(),
            "logical column {} is beyond row {}'s length of {}",
            self.cursor.logical_column,
            self.cursor.row,
            chars.len()
        );

        let inserted_len = text.chars().count();
        for (offset, c) in text.chars().enumerate() {
            chars.insert(self.cursor.logical_column + offset, c);
        }

        *row = chars.into_iter().collect();

        self.cursor.logical_column += inserted_len;
        self.clamp_physical_column();
    }

    pub fn concatenate(&self) -> String {
        self.strings.concat()
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.row == 0 {
            return;
        }
        self.cursor.row -= 1;
        self.clamp_physical_column();
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.row + 1 >= self.strings.len() {
            return;
        }
        self.cursor.row += 1;
        self.clamp_physical_column();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.physical_column == 0 {
            return;
        }
        self.cursor.physical_column -= 1;
        self.cursor.logical_column = self.cursor.physical_column;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor.physical_column >= self.row_len(self.cursor.row) {
            return;
        }
        self.cursor.physical_column += 1;
        self.cursor.logical_column = self.cursor.physical_column;
    }

    fn row_len(&self, row: usize) -> usize {
        self.strings.get(row).map_or(0, |s| s.chars().count())
    }

    fn clamp_physical_column(&mut self) {
        let len = self.row_len(self.cursor.row);
        self.cursor.physical_column = self.cursor.logical_column.min(len);
    }

    fn get_row(&self, row: usize) -> usize {
        self.cursor.row
    }

    fn get_physical_column(&self) -> usize {
        self.cursor.physical_column
    }
}

#[derive(Default)]
struct Cursor {
    row: usize,
    logical_column: usize,
    physical_column: usize,
}
