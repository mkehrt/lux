use anyhow::Result;

use crate::data::Dirty;
use crate::key::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::state::{Mode, State};
use crate::terminal::Terminal;

pub fn handle_event<T: Terminal>(state: &mut State<T>, event: Event) -> Result<Dirty> {
    let mut dirty = Dirty::Clean;

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_arrow(KeyCode::Left)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_arrow(KeyCode::Down)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_arrow(KeyCode::Up)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::None,
        }) => {
            state.handle_arrow(KeyCode::Right)?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::None,
        }) => {
            state.set_mode(Mode::Insert);
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::None,
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
