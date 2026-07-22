use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::data::Dirty;
use crate::state::State;
use crate::terminal;

pub fn event_loop(mut state: State) -> Result<()> {
    loop {
        let event = terminal::read_event()?;

        let dirty = match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_ctrl_c();
                Dirty::Clean
            }
            other_event => state.handle_event(other_event)?,
        };

        if dirty == Dirty::Dirty {
            let text = state.render()?;
            terminal::write_screen(&text)?;
            state.update_cursor()?;
        }
    }
}
