use anyhow::Result;

const NEWLINE: char = '\n';

pub struct Data {
    data: Vec<String>,
    cursor: Cursor,
}

impl Default for Data {
    fn default() -> Self {
        let one_empty_row = String::new();
        let data = vec![one_empty_row];
        let cursor = Cursor::default();
        Self { data, cursor }
    }
}

#[derive(Debug, PartialEq)]
pub enum Dirty {
    Dirty,
    Clean,
}

impl Data {
    pub fn render(&self, data_start_row: usize, rendered_rows: usize, rendered_cols: usize) -> Result<String> {
        self.check_invariants();

        let mut result = String::new();

        let mut data_row = data_start_row;
        let mut rendered_row = 0;

        while data_row < self.data.len() && rendered_row < rendered_rows {
            let row_data = &self.data[data_row];
            let mut chunk_start = 0;
            let mut row_done = false;

            while !row_done && rendered_row < rendered_rows {
                let chunk_end = (chunk_start + rendered_cols).min(row_data.len());
                result.push_str(&row_data[chunk_start..chunk_end]);
                result.push_str ("\r\n");

                rendered_row += 1;
                row_done = chunk_end >= row_data.len();

                chunk_start = chunk_end;
            }
            data_row += 1;
        }
        Ok(result)
    }

    fn write(&mut self, text: char) -> Result<Dirty> {
        assert!(self.data.len() > self.cursor.row, "Row index out of bounds");

        let row = &mut self
            .data
            .get_mut(self.cursor.row)
            .expect("Row index out of bounds");
        row.insert(self.cursor.physical_column, text);

        self.cursor.logical_column += 1;
        self.clamp_physical_column();

        let dirty;
        if self.cursor.physical_column < self.row_len(self.cursor.row) {
            dirty = Dirty::Dirty
        } else {
            dirty = Dirty::Clean;
        }

        self.check_invariants();
        Ok(dirty)
    }

    /// Char should not be a newline
    pub fn insert_char(&mut self, text: char) -> Result<Dirty> {
        assert!(text != '\n', "Char should not be a newline (\\n)");
        assert!(text != '\r', "Char should not be a newline (\\r)");

        let dirty = self.write(text)?;

        self.check_invariants();
        Ok(dirty)
    }

    /// Does not literally insert a newline, but rather splits the current row
    /// at the cursor position and inserts a new row after the current row.
    pub fn insert_newline(&mut self) -> Result<Dirty> {
        let row = self
            .data
            .get_mut(self.cursor.row)
            .ok_or_else(|| anyhow::anyhow!("Failed to get row: {}", self.cursor.row))?;
        let chars: Vec<char> = row.chars().collect();
        let split_at = self.cursor.logical_column.min(chars.len());

        // Move the rest of the current row to the new string
        let remainder: String = chars[split_at..].iter().collect();
        *row = chars[..split_at].iter().collect();

        // Insert a new string after the current row
        self.data.insert(self.cursor.row + 1, remainder);

        // Move the cursor to the beginning of the new string
        self.cursor.row += 1;
        self.cursor.logical_column = 0;
        self.clamp_physical_column();

        self.check_invariants();
        Ok(Dirty::Dirty)
    }

    pub fn concatenate(&self) -> String {
        self.data.concat()
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.row == 0 {
            return;
        }
        self.cursor.row -= 1;
        self.clamp_physical_column();

        self.check_invariants();
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.row + 1 >= self.data.len() {
            return;
        }
        self.cursor.row += 1;
        self.clamp_physical_column();

        self.check_invariants();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.physical_column == 0 {
            return;
        }
        self.cursor.physical_column -= 1;
        self.cursor.logical_column = self.cursor.physical_column;

        self.check_invariants();
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor.physical_column >= self.row_len(self.cursor.row) {
            return;
        }
        self.cursor.physical_column += 1;
        self.cursor.logical_column = self.cursor.physical_column;

        self.check_invariants();
    }

    fn row_len(&self, row: usize) -> usize {
        self.data.get(row).map_or(0, |s| s.chars().count())
    }

    fn clamp_physical_column(&mut self) {
        let len = self.row_len(self.cursor.row);
        self.cursor.physical_column = self.cursor.logical_column.min(len);
    }

    pub fn get_row(&self) -> usize {
        self.cursor.row
    }

    pub fn get_physical_column(&self) -> usize {
        self.cursor.physical_column
    }

    #[inline(always)]
    fn check_invariants(&self) {
        #[cfg(debug_assertions)]
        self.debug_check_invariants();
    }

    #[cfg(debug_assertions)]
    fn debug_check_invariants(&self) {
        assert!(
            self.cursor.row < self.data.len(),
            "Debug assertion: Cursor row out of bounds"
        );
        assert!(
            self.cursor.physical_column <= self.cursor.logical_column,
            "Debug assertion: Cursor physical column greater than logical column"
        );
        assert!(
            self.cursor.physical_column <= self.row_len(self.cursor.row),
            "Debug assertion: Cursor physical column greater than row length"
        );

        for (row_index, row) in self.data.iter().enumerate() {
            for ch in row.chars() {
                let is_newline = ch == '\n' || ch == '\r';
                assert!(
                    !is_newline,
                    "Debug assertion: newline in row {}",
                    row_index
                );
            }
        }
    }
}

#[derive(Default)]
struct Cursor {
    pub row: usize,
    pub logical_column: usize,
    pub physical_column: usize,
}
