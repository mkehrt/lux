use anyhow::Result;

const NEWLINE: &str = "\n";

#[derive(Default)]
pub struct Data {
    data: Vec<String>,
    cursor: Cursor,
}

impl Data {
    pub fn write(&mut self, text: &str) -> Result<()> {
        while self.data.len() <= self.cursor.row {
            self.data.push(String::new());
        }

        let row = &mut self.data.get_mut(self.cursor.row)
            .ok_or_else(|| anyhow::anyhow!("Failed to get row: {}", self.cursor.row))?;
        row.push_str(text);

        let inserted_len = text.chars().count();
        self.cursor.logical_column += inserted_len;
        self.clamp_physical_column();

        Ok(())
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
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.row + 1 >= self.data.len() {
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

    pub fn newline(&mut self) -> Result<()> {
        // Insert a newline at the current cursor position
        self.write(NEWLINE)?;

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

        Ok(())
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
}

#[derive(Default)]
struct Cursor {
    pub row: usize,
    pub logical_column: usize,
    pub physical_column: usize,
}
