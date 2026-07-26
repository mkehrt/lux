use anyhow::Result;

use crate::data::Dirty;
use crate::key::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::state::{Mode, SentenceOutcome, State};

pub fn handle_event(state: &mut State, event: Event) -> Result<Dirty> {
    let mut dirty = Dirty::Clean;

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::None,
        }) => {
            let outcome = state.feed_sentence(c)?;
            if outcome == SentenceOutcome::NotConsumed {
                handle_command_char(state, c)?;
            }
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

/// Handles Normal-mode character commands that are not part of a sentence.
fn handle_command_char(state: &mut State, c: char) -> Result<()> {
    match c {
        'i' => state.set_mode(Mode::Insert),
        'a' => {
            state.handle_right()?;
            state.set_mode(Mode::Insert);
        }
        _ => {}
    }
    Ok(())
}
