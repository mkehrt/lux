use anyhow::Result;

use crate::data::Dirty;
use crate::key::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::state::{Mode, State};

pub fn handle_event(state: &mut State, event: Event) -> Result<Dirty> {
    let mut dirty = Dirty::Clean;

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::None,
        }) => {
            state.set_mode(Mode::Normal);
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::None | KeyModifiers::Shift,
        }) => {
            dirty = state.handle_text(c)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::None,
        }) => {
            dirty = state.handle_enter()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::None,
        }) => {
            dirty = state.handle_backspace()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_up()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_down()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_left()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_right()?;
        }
        Event::Resize(cols, rows) => {
            state.handle_resize(cols, rows);
            dirty = Dirty::Dirty;
        }
        other_event => {
            state.handle_unknown_event(other_event)?;
        }
    }

    Ok(dirty)
}
