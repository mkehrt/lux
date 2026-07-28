use anyhow::Result;

use crate::data;
use crate::data::Dirty;
use crate::insert::Insert;
use crate::key::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::normal::Normal;
use crate::terminal::Terminal;

#[derive(Debug)]
pub enum Mode {
    Normal(Normal),
    Insert(Insert),
}

pub struct State {
    pub(crate) data: data::Data,
    mode: Mode,
    pub(crate) terminal: Box<dyn Terminal>,
    pub(crate) terminal_size: TerminalSize,
}

impl State {
    pub fn new(terminal: Box<dyn Terminal>, terminal_size: TerminalSize, data: data::Data) -> Self {
        Self {
            terminal,
            terminal_size,
            data,
            mode: Mode::Normal(Normal::new()),
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

    pub fn handle_event(&mut self, event: Event) -> Result<Dirty> {
        let mut dirty = Dirty::Clean;

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::None,
            }) => {
                self.set_mode(Mode::Normal(Normal::new()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::None | KeyModifiers::Shift,
            }) => {
                dirty = self.handle_char(c)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::None,
            }) => {
                dirty = self.handle_enter()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::None,
            }) => {
                dirty = self.handle_backspace()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::None,
            }) => {
                self.handle_up()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::None,
            }) => {
                self.handle_down()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::None,
            }) => {
                self.handle_left()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::None,
            }) => {
                self.handle_right()?;
            }
            Event::Resize(cols, rows) => {
                self.handle_resize(cols, rows);
                dirty = Dirty::Dirty;
            }
            other_event => {
                self.handle_unknown_event(other_event)?;
            }
        }

        Ok(dirty)
    }

    /// Dispatches a character to the handler for the current mode.
    fn handle_char(&self, ch: char) -> Result<Dirty> {
        match &mut self.mode {
            This doesn't work
            Mode::Normal(normal) => normal.handle_char(self, ch),
            Mode::Insert(insert) => insert.handle_char(self, ch),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
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

    pub fn handle_unknown_event(&mut self, event: Event) -> Result<()> {
        self.write_status_line(&format!("{:?}", event))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TerminalSize {
    rows: u16,
    pub(crate) cols: u16,
}

impl TerminalSize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}
