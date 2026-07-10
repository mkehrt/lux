use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use std::io::{self, Write as _};
use std::process;

fn main() -> io::Result<()> {
    let mut previous_terminal_state = set_up_terminal()?;

    let mut terminal_size = get_terminal_size()?;

    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All))?;
    
    loop {
        let event = if let Ok(event) = read() {
            event
        } else {
            break;
        };

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                // In the future, this should send a SIGINT to self and do
                // cleanup in the handler.  This means an external signal will
                // be also do cleanup.
                previous_terminal_state.restore()?;
                process::exit(130);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                let mut stdout = io::stdout();
                write!(stdout, "{}", c)?;
                stdout.flush()?;
            }
            Event::Resize(cols, rows) => {
                terminal_size = TerminalSize { cols, rows };
            }
            other_event => {
                handle_unknown_event(other_event, terminal_size.cols)?;
            }
        }
    }
    Ok(())
}

fn handle_unknown_event(event: Event, terminal_columns: u16) -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, terminal_columns - 1),
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

fn get_terminal_size() -> io::Result<TerminalSize> {
    let (cols, rows) = terminal::size()?;
    Ok(TerminalSize { cols, rows })
}
struct PreviousTerminalState {
    raw_mode_was_enabled: bool,
}

impl PreviousTerminalState {
    fn restore(&mut self) -> io::Result<()> {
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

fn set_up_terminal() -> io::Result<PreviousTerminalState> {
    let raw_mode_was_enabled = terminal::is_raw_mode_enabled()?;
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    Ok(PreviousTerminalState {
        raw_mode_was_enabled,
    })
}
