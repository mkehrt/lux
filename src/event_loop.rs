use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use lux_core::data::Dirty;
use lux_core::state::State;
use lux_core::terminal::Terminal;

use crate::key_translation;
use crate::terminal::{self, PreviousTerminalState};

pub fn event_loop<T: Terminal>(
    mut state: State<T>,
    previous: PreviousTerminalState,
) -> Result<()> {
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
    let Event::Key(key_event) = event else {
        return false;
    };
    let is_char_c = key_event.code == KeyCode::Char('c');
    let is_control = key_event.modifiers == KeyModifiers::CONTROL;
    let is_press = key_event.kind == KeyEventKind::Press;
    is_char_c && is_control && is_press
}
