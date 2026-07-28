use anyhow::Result;

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
    pub fn render(
        &self,
        data_start_row: usize,
        rendered_rows: u16,
        rendered_cols: u16,
    ) -> Result<String> {
        self.check_invariants();

        let rendered_rows = rendered_rows as usize;
        let rendered_cols = rendered_cols as usize;

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
                result.push_str("\r\n");

                rendered_row += 1;
                row_done = chunk_end >= row_data.len();

                chunk_start = chunk_end;
            }
            data_row += 1;
        }
        Ok(result)
    }

    /// Maps the cursor to terminal coordinates, accounting for data rows that
    /// wrap into multiple rendered rows. Returns (row, column).
    pub fn rendered_cursor_position(&self, cols: u16) -> (u16, u16) {
        let cols = cols as usize;

        let mut rendered_row = 0;

        let mut data_row = 0;
        while data_row < self.cursor.row {
            let len = self.row_len(data_row);
            rendered_row += Self::rendered_lines(len, cols);
            data_row += 1;
        }

        let wrapped_rows = self.cursor.physical_column / cols;
        let cursor_row = (rendered_row + wrapped_rows) as u16;
        let cursor_column = (self.cursor.physical_column % cols) as u16;

        (cursor_row, cursor_column)
    }

    /// The number of rendered rows a data row of length `len` occupies.
    fn rendered_lines(len: usize, cols: usize) -> usize {
        let chunks = len.div_ceil(cols);
        chunks.max(1)
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

    /// Deletes the character before the cursor. At the start of a row, joins
    /// the row with the row above instead.
    pub fn delete_char(&mut self) -> Result<Dirty> {
        let dirty;

        if self.cursor.physical_column > 0 {
            let row = self
                .data
                .get_mut(self.cursor.row)
                .ok_or_else(|| anyhow::anyhow!("Failed to get row: {}", self.cursor.row))?;
            row.remove(self.cursor.physical_column - 1);

            self.cursor.physical_column -= 1;
            self.cursor.logical_column = self.cursor.physical_column;
            dirty = Dirty::Dirty;
        } else if self.cursor.row > 0 {
            let row = self.data.remove(self.cursor.row);
            let previous_row = self.cursor.row - 1;
            let previous_len = self.row_len(previous_row);
            self.data[previous_row].push_str(&row);

            self.cursor.row = previous_row;
            self.cursor.physical_column = previous_len;
            self.cursor.logical_column = previous_len;
            dirty = Dirty::Dirty;
        } else {
            dirty = Dirty::Clean;
        }

        self.check_invariants();
        Ok(dirty)
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
                assert!(!is_newline, "Debug assertion: newline in row {}", row_index);
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
