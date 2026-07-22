use anyhow::Result;

/// The drawing interface the editor needs from a terminal. The binary supplies
/// a concrete crossterm-backed implementation.
pub trait Terminal {
    fn write_char(&mut self, c: char) -> Result<()>;
    fn move_cursor_to(&mut self, column: u16, row: u16) -> Result<()>;
    fn write_status_line(&mut self, text: &str, bottom_row: u16) -> Result<()>;
    fn write_screen(&mut self, text: &str) -> Result<()>;
}
