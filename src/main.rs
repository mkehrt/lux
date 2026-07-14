use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use std::io::{self, Write as _};

mod data;

fn main() -> Result<()> {
    let mut previous_terminal_state = set_up_terminal()?;
    let terminal_size = get_terminal_size()?;
    let data = data::Data::default();
    let mut state = State::new(previous_terminal_state, terminal_size, data);

    loop {
        let event = read()?;

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_ctrl_c();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_text(c)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_enter();
            }
            Event::Key(KeyEvent {
                code: code @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_arrow(code)?;
            }
            Event::Resize(cols, rows) => {
                state.handle_resize(cols, rows);
            }
            other_event => {
                state.handle_unknown_event(other_event)?;
            }
        }
    }
}

struct State {
    data: data::Data,
    previous_terminal_state: PreviousTerminalState,
    terminal_size: TerminalSize,
}

impl State {
    fn new(previous_terminal_state: PreviousTerminalState, terminal_size: TerminalSize, data: data::Data) -> Self {
        Self { previous_terminal_state, terminal_size, data }
    }

    fn update_cursor(&mut self) -> Result<()> {
        let row = self.data.get_row();
        let physical_column = self.data.get_physical_column();

        execute!(
            io::stdout(),
            cursor::MoveTo(physical_column as u16, row as u16),
        )?;
        
        Ok(())
    }

    fn handle_ctrl_c(&mut self) {
        if let Err(e) = self.previous_terminal_state.restore() {
            eprintln!("Error restoring terminal state: {}", e);
        }
        std::process::exit(130);
    }

    fn handle_text(&mut self, c: char) -> Result<()> {
        write!(io::stdout(), "{}", c)?;
        self.data.write(&c.to_string())?;
        self.update_cursor()?;
        Ok(())
    }

    fn handle_enter(&mut self) -> Result<()> {
        self.data.newline()?;
        self.update_cursor()?;
        Ok(())
    }

    fn handle_arrow(&mut self, code: KeyCode) -> Result<()> {
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

    fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.terminal_size = TerminalSize { cols, rows };
    }

    fn handle_unknown_event(&mut self, event: Event) -> Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, self.terminal_size.rows - 1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )?;
        write!(stdout, "{:?}", event)?;
        stdout.flush()?;

        execute!(stdout, cursor::RestorePosition    )?;

        Ok(())
    }
}

#[derive(Debug)]
struct TerminalSize {
    rows: u16,
    #[allow(unused)]
    cols: u16,
}

fn get_terminal_size() -> Result<TerminalSize> {
    let (cols, rows) = terminal::size()?;
    Ok(TerminalSize { cols, rows })
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
