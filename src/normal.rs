use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::data::Dirty;
use crate::state::{Mode, State};

pub fn handle_event(state: &mut State, event: Event) -> Result<Dirty> {
    let mut dirty = Dirty::Clean;

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.handle_arrow(KeyCode::Left)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.handle_arrow(KeyCode::Down)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.handle_arrow(KeyCode::Up)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.handle_arrow(KeyCode::Right)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.set_mode(Mode::Insert);
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => {
            state.handle_arrow(KeyCode::Right)?;
            state.set_mode(Mode::Insert);
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
