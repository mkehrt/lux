use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::data::Dirty;
use crate::state::{Mode, State};

pub fn handle_event(state: &mut State, event: Event) -> Result<Dirty> {
    let mut dirty = Dirty::Clean;

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.set_mode(Mode::Normal);
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            dirty = state.handle_text(c)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            dirty = state.handle_enter()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            dirty = state.handle_backspace()?;
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
            dirty = Dirty::Dirty;
        }
        other_event => {
            state.handle_unknown_event(other_event)?;
        }
    }

    Ok(dirty)
}
