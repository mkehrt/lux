use anyhow::Result;
use lux_core::state::State;
use terminal::CrosstermTerminal;

mod event_loop;
mod key_translation;
mod terminal;

fn main() -> Result<()> {
    let previous_terminal_state = terminal::set_up_terminal()?;
    let crossterm_terminal = Box::new(CrosstermTerminal);
    let state = State::new(crossterm_terminal)?;

    event_loop::event_loop(state, previous_terminal_state)
}
