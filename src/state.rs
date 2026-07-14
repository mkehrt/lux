use anyhow::Result;
use crossterm::event::{Event, KeyCode};

use crate::data;
use crate::data::Dirty;
use crate::terminal;

pub struct State {
    data: data::Data,
    previous_terminal_state: PreviousTerminalState,
    terminal_size: TerminalSize,
}

impl State {
    pub fn new(
        previous_terminal_state: PreviousTerminalState,
        terminal_size: TerminalSize,
        data: data::Data,
    ) -> Self {
        Self {
            previous_terminal_state,
            terminal_size,
            data,
        }
    }

    pub fn write_status_line(&self, text: &str) -> Result<()> {
        terminal::write_status_line(text, self.terminal_size.rows - 1)
    }

    fn update_cursor(&mut self) -> Result<()> {
        let row = self.data.get_row();
        let physical_column = self.data.get_physical_column();

        terminal::move_cursor_to(physical_column as u16, row as u16)
    }

    pub fn render(&self) -> Result<String> {
        self.data.render(
            0,
            self.terminal_size.rows as usize,
            self.terminal_size.cols as usize,
        )
    }

    pub fn handle_ctrl_c(&mut self) {
        if let Err(e) = self.previous_terminal_state.restore() {
            eprintln!("Error restoring terminal state: {}", e);
        }
        std::process::exit(130);
    }

    pub fn handle_text(&mut self, c: char) -> Result<Dirty> {
        terminal::write_char(c)?;
        let dirty = self.data.insert_char(c)?;
        self.update_cursor()?;
        Ok(dirty)
    }

    pub fn handle_enter(&mut self) -> Result<Dirty> {
        let dirty = self.data.insert_newline()?;
        self.update_cursor()?;
        Ok(dirty)
    }

    pub fn handle_arrow(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Up => self.data.move_cursor_up(),
            KeyCode::Down => self.data.move_cursor_down(),
            KeyCode::Left => self.data.move_cursor_left(),
            KeyCode::Right => self.data.move_cursor_right(),
            _ => return Err(anyhow::anyhow!("Invalid arrow key: {:?}", code)),
        }
        self.update_cursor()?;
        Ok(())
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
    cols: u16,
}

impl TerminalSize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

pub struct PreviousTerminalState {
    raw_mode_was_enabled: bool,
}

impl PreviousTerminalState {
    fn restore(&mut self) -> Result<()> {
        terminal::leave_alternate_screen()?;
        if self.raw_mode_was_enabled {
            terminal::enable_raw_mode()?;
        } else {
            terminal::disable_raw_mode()?;
        }
        println!("Restored terminal state");
        Ok(())
    }
}

pub fn set_up_terminal() -> Result<PreviousTerminalState> {
    let raw_mode_was_enabled = terminal::is_raw_mode_enabled()?;
    terminal::enable_raw_mode()?;
    terminal::enter_alternate_screen()?;
    Ok(PreviousTerminalState {
        raw_mode_was_enabled,
    })
}
