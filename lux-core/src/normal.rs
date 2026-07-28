use anyhow::Result;

use crate::data::Dirty;
use crate::sentence::InProgressSentence;
use crate::state::State;

#[derive(Debug)]
pub struct Normal {
    in_progress_sentence: InProgressSentence,
}

impl Normal {
    pub fn new() -> Self {
        let in_progress_sentence = InProgressSentence::new();
        Normal {
            in_progress_sentence,
        }
    }

    pub fn handle_char(&mut self, _state: &mut State, _c: char) -> Result<Dirty> {
        unimplemented!()
    }
}
