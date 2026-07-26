use anyhow::Result;

mod event_loop;
mod key_translation;
mod terminal;

use lux_core::data::Data;
use lux_core::state::{State, TerminalSize};

use terminal::CrosstermTerminal;

fn main() -> Result<()> {
    let previous_terminal_state = terminal::set_up_terminal()?;
    let (cols, rows) = terminal::size()?;
    let terminal_size = TerminalSize::new(cols, rows);
    let crossterm_terminal = Box::new(CrosstermTerminal);
    let data = Data::default();
    let state = State::new(crossterm_terminal, terminal_size, data);

    event_loop::event_loop(state, previous_terminal_state)
}
