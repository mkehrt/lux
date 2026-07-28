use anyhow::Result;

pub trait Terminal {
    fn size(&self) -> Result<TerminalSize>;
    fn write_char(&mut self, c: char) -> Result<()>;
    fn move_cursor_to(&mut self, column: u16, row: u16) -> Result<()>;
    fn write_status_line(&mut self, text: &str, bottom_row: u16) -> Result<()>;
    fn write_screen(&mut self, text: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct TerminalSize {
    pub rows: u16,
    pub cols: u16,
}

impl TerminalSize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}
