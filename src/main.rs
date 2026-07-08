use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal;
use nix::unistd;
use nix::sys::signal;
use std::io::{self, Write as _};

fn main() -> io::Result<()>{
    let stdout = io::stdout();
    let mut locked_stdout = stdout.lock();

    let raw_mode_was_enabled = terminal::is_raw_mode_enabled()?;
    terminal::enable_raw_mode()?;
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
                interrupt_self()?;
            }
            event => {
                write!(locked_stdout, "{:?}", event)?;
                locked_stdout.flush()?;
            }
        }
    }

    if raw_mode_was_enabled {
        terminal::disable_raw_mode()?;
    }

    Ok(())
}

fn interrupt_self() -> io::Result<()> {
    let pid = unistd::getpid();
    signal::kill(pid, signal::Signal::SIGINT)?;
    Ok(())
}
