use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use std::io::{self, Write as _};
use std::process;

mod data;

fn main() -> Result<()> {
    let mut previous_terminal_state = set_up_terminal()?;

    let mut terminal_info = get_terminal_info()?;
    let mut data = data::Data::default();

    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All))?;
    
    loop {
        let event = read()?;

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                handle_ctrl_c(&mut previous_terminal_state)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                handle_char(c)?;
            }
            Event::Resize(cols, rows) => {
                handle_resize(&mut terminal_info, cols, rows);
            }
            other_event => {
                handle_unknown_event(other_event, &terminal_info)?;
            }
        }
    }
    Ok(())
}

fn handle_ctrl_c(previous_terminal_state: &mut PreviousTerminalState) -> Result<()> {
    // In the future, this should send a SIGINT to self and do
    // cleanup in the handler.  This means an external signal will
    // be also do cleanup.
    previous_terminal_state.restore()?;
    process::exit(130);
}

fn handle_char(c: char) -> Result<()> {
    let mut stdout = io::stdout();
    write!(stdout, "{}", c)?;
    stdout.flush()?;
    Ok(())
}

fn handle_resize(terminal_info: &mut TerminalInfo, cols: u16, rows: u16) {
    terminal_info.size = TerminalSize { cols, rows };
}

fn handle_unknown_event(event: Event, terminal_info: &TerminalInfo) -> Result<()> {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, terminal_info.size.rows - 1),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )?;
    write!(stdout, "{:?}", event)?;
    stdout.flush()?;

    execute!(stdout, cursor::RestorePosition    )?;

    Ok(())
}

#[derive(Debug)]
struct TerminalSize {
    rows: u16,
    cols: u16,
}

#[derive(Debug)]
struct TerminalInfo {
    size: TerminalSize,
}

fn get_terminal_info() -> Result<TerminalInfo> {
    let (cols, rows) = terminal::size()?;
    Ok(TerminalInfo {
        size: TerminalSize { cols, rows },
    })
}
struct PreviousTerminalState {
    raw_mode_was_enabled: bool,
}

impl PreviousTerminalState {
    fn restore(&mut self) -> Result<()> {
        execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
        if self.raw_mode_was_enabled {
            terminal::enable_raw_mode()?;
        } else {
            terminal::disable_raw_mode()?;
        }
        println!("Restored terminal state");
        Ok(())
    }
}

fn set_up_terminal() -> Result<PreviousTerminalState> {
    let raw_mode_was_enabled = terminal::is_raw_mode_enabled()?;
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    Ok(PreviousTerminalState {
        raw_mode_was_enabled,
    })
}
