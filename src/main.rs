use anyhow::Result;

mod data;
mod event_loop;
mod state;
mod terminal;

use state::State;

fn main() -> Result<()> {
    let previous_terminal_state = state::set_up_terminal()?;
    let terminal_size = state::get_terminal_size()?;
    let data = data::Data::default();
    let state = State::new(previous_terminal_state, terminal_size, data);

    event_loop::event_loop(state)
}
