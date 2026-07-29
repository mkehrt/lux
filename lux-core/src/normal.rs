use anyhow::Result;

use crate::data::Dirty;
use crate::sentence::CharResult;
use crate::state::{Mode, State};

/// Feeds a character to the sentence being parsed.  A sentence that becomes
/// complete is executed and parsing starts over.  A character the parser
/// rejects ends the sentence.
/// Callers must guarantee this is called in normal mode.
pub fn handle_char(state: &mut State, c: char) -> Result<Dirty> {
    let sentence = match &mut state.mode {
        Mode::Normal { sentence } => sentence,
        _ => unreachable!("normal::handle_char called outside normal mode")
    };

    let result = sentence.accept_character(c);

    match result {
        CharResult::Accepted => {
            let mut dirty = Dirty::Clean;
            if sentence.is_complete() {
                let old_sentence = std::mem::take(sentence);
                dirty = old_sentence.execute(state)?;
            }
            Ok(dirty)
        }
        CharResult::Rejected(c) => {
            sentence.reset();
            Ok(Dirty::Clean)
        }
    }
}
