use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::state::State;
use crate::terminal;

pub fn event_loop(mut state: State) -> Result<()> {
    loop {
        let event = terminal::read_event()?;

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_ctrl_c();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_text(c)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                state.handle_enter();
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
            }
            other_event => {
                state.handle_unknown_event(other_event)?;
            }
        }
    }
}
