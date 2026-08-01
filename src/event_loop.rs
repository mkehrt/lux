use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use lux_core::data::Dirty;
use lux_core::state::State;

use crate::key_translation;
use crate::terminal::{self, PreviousTerminalState};

pub fn event_loop(mut state: State, previous: PreviousTerminalState) -> Result<()> {
    loop {
        let event = terminal::read_event()?;

        if is_ctrl_c(&event) {
            previous.restore()?;
            std::process::exit(130);
        }

        let translated = key_translation::translate_event(event);
        let Some(event) = translated else {
            continue;
        };

        let dirty = state.handle_event(event)?;
        if dirty == Dirty::Dirty {
            state.redraw()?;
        }
    }
}

fn is_ctrl_c(event: &Event) -> bool {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            ..
        }) => true,
        _ => false,
    }
}
