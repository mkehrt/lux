use anyhow::Result;
use crossterm::cursor;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal;
use std::io::{self, Write as _};

use lux_core::terminal::Terminal;

/// Crossterm-backed implementation of the library's drawing interface.
pub struct CrosstermTerminal;

impl Terminal for CrosstermTerminal {
    fn write_char(&mut self, c: char) -> Result<()> {
        write!(io::stdout(), "{}", c)?;
        Ok(())
    }

    fn move_cursor_to(&mut self, column: u16, row: u16) -> Result<()> {
        execute!(io::stdout(), cursor::MoveTo(column, row))?;
        Ok(())
    }

    fn write_status_line(&mut self, text: &str, bottom_row: u16) -> Result<()> {
        execute!(
            io::stdout(),
            cursor::SavePosition,
            cursor::MoveTo(0, bottom_row)
        )?;
        write!(io::stdout(), "{}", text)?;
        io::stdout().flush()?;
        execute!(io::stdout(), cursor::RestorePosition)?;
        Ok(())
    }

    fn write_screen(&mut self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::SavePosition,
            cursor::MoveTo(0, 0),
        )?;
        let text = text.replace('\n', "\r\n");
        write!(io::stdout(), "{}", text)?;
        execute!(io::stdout(), cursor::RestorePosition)?;
        Ok(())
    }
}

pub fn read_event() -> Result<Event> {
    Ok(event::read()?)
}

pub fn size() -> Result<(u16, u16)> {
    Ok(terminal::size()?)
}

pub fn is_raw_mode_enabled() -> Result<bool> {
    Ok(terminal::is_raw_mode_enabled()?)
}

pub fn enable_raw_mode() -> Result<()> {
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn disable_raw_mode() -> Result<()> {
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn enter_alternate_screen() -> Result<()> {
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    Ok(())
}

pub fn leave_alternate_screen() -> Result<()> {
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

pub struct PreviousTerminalState {
    raw_mode_was_enabled: bool,
}

impl PreviousTerminalState {
    pub fn restore(&self) -> Result<()> {
        leave_alternate_screen()?;
        if self.raw_mode_was_enabled {
            enable_raw_mode()?;
        } else {
            disable_raw_mode()?;
        }
        println!("Restored terminal state");
        Ok(())
    }
}

pub fn set_up_terminal() -> Result<PreviousTerminalState> {
    let raw_mode_was_enabled = is_raw_mode_enabled()?;
    enable_raw_mode()?;
    enter_alternate_screen()?;
    Ok(PreviousTerminalState {
        raw_mode_was_enabled,
    })
}
