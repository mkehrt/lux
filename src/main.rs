use anyhow::Result;

mod data;
mod event_loop;
mod insert;
mod normal;
mod state;
mod terminal;

use state::State;

fn main() -> Result<()> {
    let previous_terminal_state = state::set_up_terminal()?;
    let (cols, rows) = terminal::size()?;
    let terminal_size = state::TerminalSize::new(cols, rows);
    let data = data::Data::default();
    let state = State::new(previous_terminal_state, terminal_size, data);
    state.write_status_line(&format!("{}x{}", cols, rows)).unwrap();

    event_loop::event_loop(state)
}
