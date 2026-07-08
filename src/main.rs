use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal;
use std::io::{self, Write as _};
use std::process;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut locked_stdout = stdout.lock();

    let mut previous_terminal_state = set_up_terminal()?;

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
                state: KeyEventState::NONE,
            }) => {
                // In the future, this should send a SIGINT to self and do
                // cleanup in the handler.  This means an external signal will
                // be also do cleanup.
                previous_terminal_state.restore()?;
                process::exit(130);
            }
            _ => {
                // \r Neede to move cursor to the beginning of the line.
                write!(locked_stdout, "{:?}\r\n", event)?;
                locked_stdout.flush()?;
            }
        }
    }
    Ok(())
}

struct PreviousTerminalState {
    raw_mode_was_enabled: bool,
}

impl PreviousTerminalState {
    fn restore(&mut self) -> io::Result<()> {
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
    Ok(PreviousTerminalState {
        raw_mode_was_enabled,
    })
}
