use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::data::Dirty;
use crate::state::State;
use crate::terminal;

pub fn event_loop(mut state: State) -> Result<()> {
    loop {
        let mut dirty = Dirty::Clean;
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

        if dirty == Dirty::Dirty {
            let text = state.render()?;
            terminal::write_screen(&text)?;
        }
    }
}
