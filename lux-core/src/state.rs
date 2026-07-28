use anyhow::Result;

use crate::data;
use crate::data::Dirty;
use crate::insert;
use crate::key::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::normal;
use crate::sentence;
use crate::terminal;

#[derive(Debug)]
pub enum Mode {
    Normal { sentence: sentence::Sentence },
    Insert,
}

impl Mode {
    pub fn default_normal() -> Self {
        let sentence = sentence::Sentence::default();
        Self::Normal { sentence }
    }
}
pub struct State {
    pub(crate) terminal: Box<dyn terminal::Terminal>,
    pub(crate) terminal_size: terminal::TerminalSize,
    pub(crate) mode: Mode,
    pub(crate) data: data::Data,
}

impl State {
    pub fn new(terminal: Box<dyn terminal::Terminal>) -> Result<Self> {
        let terminal_size = terminal.size()?;
        let data = data::Data::default();
        let mode = Mode::default_normal();
        let state = Self {
            terminal,
            terminal_size,
            data,
            mode,
        };
        Ok(state)
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
                self.set_mode(Mode::default_normal());
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
    fn handle_char(&mut self, ch: char) -> Result<Dirty> {
        match &mut self.mode {
            Mode::Normal { sentence: _ } => normal::handle_char(self, ch),
            Mode::Insert => insert::handle_char(self, ch),
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
        self.terminal_size = terminal::TerminalSize { cols, rows };
    }

    pub fn handle_unknown_event(&mut self, event: Event) -> Result<()> {
        self.write_status_line(&format!("{:?}", event))?;
        Ok(())
    }
}
