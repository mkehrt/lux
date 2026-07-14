use anyhow::Result;
use crossterm::cursor;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal;
use std::io::{self, Write as _};

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

pub fn move_cursor_to(column: u16, row: u16) -> Result<()> {
    execute!(io::stdout(), cursor::MoveTo(column, row))?;
    Ok(())
}

pub fn write_char(c: char) -> Result<()> {
    write!(io::stdout(), "{}", c)?;
    Ok(())
}

pub fn write_screen(text: &str) -> Result<()> {
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

pub fn write_status_line(text: &str, bottom_row: u16) -> Result<()> {
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
