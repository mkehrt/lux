use anyhow::Result;

use crate::data;
use crate::data::Dirty;
use crate::insert;
use crate::key;
use crate::normal;
use crate::terminal::Terminal;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Insert,
    Normal,
}

pub struct State<T: Terminal> {
    data: data::Data,
    mode: Mode,
    terminal: T,
    terminal_size: TerminalSize,
}

impl<T: Terminal> State<T> {
    pub fn new(terminal: T, terminal_size: TerminalSize, data: data::Data) -> Self {
        Self {
            terminal,
            terminal_size,
            data,
            mode: Mode::Insert,
        }
    }

    pub fn write_status_line(&mut self, text: &str) -> Result<()> {
        let bottom_row = self.terminal_size.rows - 1;
        self.terminal.write_status_line(text, bottom_row)
    }

    pub fn update_cursor(&mut self) -> Result<()> {
        let cols = self.terminal_size.cols;
        let (row, column) = self.data.rendered_cursor_position(cols);

        self.terminal.move_cursor_to(column, row)
    }

    pub fn render(&self) -> Result<String> {
        self.data
            .render(0, self.terminal_size.rows, self.terminal_size.cols)
    }

    /// Renders the current data and draws it to the terminal, then repositions
    /// the cursor.
    pub fn redraw(&mut self) -> Result<()> {
        let text = self.render()?;
        self.terminal.write_screen(&text)?;
        self.update_cursor()?;
        Ok(())
    }

    pub fn handle_event(&mut self, event: key::Event) -> Result<Dirty> {
        match self.mode {
            Mode::Insert => insert::handle_event(self, event),
            Mode::Normal => normal::handle_event(self, event),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn handle_text(&mut self, c: char) -> Result<Dirty> {
        self.terminal.write_char(c)?;
        let mut dirty = self.data.insert_char(c)?;

        let cols = self.terminal_size.cols as usize;
        let cursor_off_edge = self.data.get_physical_column() >= cols;
        if cursor_off_edge {
            dirty = Dirty::Dirty;
        }

        self.update_cursor()?;
        Ok(dirty)
    }

    pub fn handle_enter(&mut self) -> Result<Dirty> {
        let dirty = self.data.insert_newline()?;
        self.update_cursor()?;
        Ok(dirty)
    }

    pub fn handle_backspace(&mut self) -> Result<Dirty> {
        let dirty = self.data.delete_char()?;
        self.update_cursor()?;
        Ok(dirty)
    }

    pub fn handle_up(&mut self) -> Result<()> {
        self.data.move_cursor_up();
        self.update_cursor()
    }

    pub fn handle_down(&mut self) -> Result<()> {
        self.data.move_cursor_down();
        self.update_cursor()
    }

    pub fn handle_left(&mut self) -> Result<()> {
        self.data.move_cursor_left();
        self.update_cursor()
    }

    pub fn handle_right(&mut self) -> Result<()> {
        self.data.move_cursor_right();
        self.update_cursor()
    }

    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.terminal_size = TerminalSize { cols, rows };
    }

    pub fn handle_unknown_event(&mut self, event: key::Event) -> Result<()> {
        self.write_status_line(&format!("{:?}", event))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TerminalSize {
    rows: u16,
    cols: u16,
}

impl TerminalSize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}
