use anyhow::Result;

use crate::data::Dirty;
use crate::state::State;

pub fn handle_char(state: &mut State, c: char) -> Result<Dirty> {
    state.terminal.write_char(c)?;
    let mut dirty = state.data.insert_char(c)?;

    let cols = state.terminal_size.cols as usize;
    let cursor_off_edge = state.data.get_physical_column() >= cols;
    if cursor_off_edge {
        dirty = Dirty::Dirty;
    }

    state.update_cursor()?;
    Ok(dirty)
}
