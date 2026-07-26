use anyhow::Result;

use crate::state::State;

pub enum CharResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Default)]
pub enum InProgressSentence {
    #[default]
    Start,
    Count(Count),
    Complete(CountAndMotion),
}

#[derive(Debug, Default)]
pub enum Count {
    #[default]
    Empty,
    Number(u64),
}

impl Count {
    pub fn accept_character(&mut self, ch: char) -> CharResult {
        let digit = ch.to_digit(10).map(|d| d as u64);
        let this = std::mem::take(self);
        match (digit, this) {
            (Some(digit), Count::Empty) => {
                *self = Count::Number(digit);
                CharResult::Accepted
            }
            (Some(digit), Count::Number(old_number)) => {
                let new_number = old_number * 10 + digit;
                *self = Count::Number(new_number);
                CharResult::Accepted
            }
            (None, old) => {
                *self = old;
                CharResult::Rejected
            }
        }
    }

    /// The effective repeat count; an empty count means once.
    fn repeat(&self) -> u64 {
        match self {
            Count::Empty => 1,
            Count::Number(n) => *n,
        }
    }
}

impl InProgressSentence {
    pub fn new() -> Self {
        InProgressSentence::Start
    }

    pub fn accept_character(&mut self, ch: char) -> CharResult {
        if ch.is_ascii_digit() {
            return self.accept_count_digit(ch);
        }
        if let Some(motion) = Motion::from_char(ch) {
            return self.accept_motion(motion);
        }
        CharResult::Rejected
    }

    fn accept_count_digit(&mut self, ch: char) -> CharResult {
        match self {
            InProgressSentence::Start => {
                let mut count = Count::Empty;
                let result = count.accept_character(ch);
                *self = InProgressSentence::Count(count);
                result
            }
            InProgressSentence::Count(count) => count.accept_character(ch),
            InProgressSentence::Complete(_) => CharResult::Rejected,
        }
    }

    fn accept_motion(&mut self, motion: Motion) -> CharResult {
        let count = match self {
            InProgressSentence::Start => Count::Empty,
            InProgressSentence::Count(count) => std::mem::take(count),
            InProgressSentence::Complete(_) => return CharResult::Rejected,
        };
        let count_and_motion = CountAndMotion { count, motion };
        *self = InProgressSentence::Complete(count_and_motion);
        CharResult::Accepted
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, InProgressSentence::Complete(_))
    }

    /// Runs a completed sentence. The caller takes the sentence out of `State`
    /// before calling this, so there is no aliasing with `state`.
    pub fn execute(self, state: &mut State) -> Result<()> {
        let InProgressSentence::Complete(count_and_motion) = self else {
            return Ok(());
        };
        let CountAndMotion { count, motion } = count_and_motion;

        let repeat = count.repeat();
        let mut done = 0;
        while done < repeat {
            motion.execute(state)?;
            done += 1;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CountAndMotion {
    count: Count,
    motion: Motion,
}

#[derive(Debug)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
    // Deferred (see PLAN.md): the `_` line motion. Kept as its eventual home;
    // `from_char` does not map to it yet.
    #[allow(dead_code)]
    Line,
}

impl Motion {
    fn from_char(ch: char) -> Option<Motion> {
        let motion = match ch {
            'h' => Motion::Left,
            'j' => Motion::Down,
            'k' => Motion::Up,
            'l' => Motion::Right,
            _ => return None,
        };
        Some(motion)
    }

    fn execute(&self, state: &mut State) -> Result<()> {
        match self {
            Motion::Up => state.handle_up(),
            Motion::Down => state.handle_down(),
            Motion::Left => state.handle_left(),
            Motion::Right => state.handle_right(),
            Motion::Line => anyhow::bail!("Line motion (`_`) is not yet implemented"),
        }
    }
}

// Deferred to the next pass (see PLAN.md): operators (`d`, `y`, `c`) and the
// `[count] operator [count] motion` grammar, which need range delete/yank in
// `Data`.

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted(result: CharResult) -> bool {
        matches!(result, CharResult::Accepted)
    }

    /// Feeds each character in order, asserting every one is accepted.
    fn feed(sentence: &mut InProgressSentence, chars: &str) {
        for ch in chars.chars() {
            assert!(accepted(sentence.accept_character(ch)), "rejected {:?}", ch);
        }
    }

    fn completed(sentence: &InProgressSentence) -> (u64, &Motion) {
        match sentence {
            InProgressSentence::Complete(CountAndMotion { count, motion }) => {
                (count.repeat(), motion)
            }
            other => panic!("expected a completed sentence, got {:?}", other),
        }
    }

    #[test]
    fn bare_motion_has_count_one() {
        let mut sentence = InProgressSentence::new();
        feed(&mut sentence, "j");
        assert!(sentence.is_complete());
        let (repeat, motion) = completed(&sentence);
        assert_eq!(repeat, 1);
        assert!(matches!(motion, Motion::Down));
    }

    #[test]
    fn single_digit_count() {
        let mut sentence = InProgressSentence::new();
        feed(&mut sentence, "3j");
        let (repeat, motion) = completed(&sentence);
        assert_eq!(repeat, 3);
        assert!(matches!(motion, Motion::Down));
    }

    #[test]
    fn multi_digit_count() {
        let mut sentence = InProgressSentence::new();
        feed(&mut sentence, "12l");
        let (repeat, motion) = completed(&sentence);
        assert_eq!(repeat, 12);
        assert!(matches!(motion, Motion::Right));
    }

    #[test]
    fn count_is_incomplete_before_motion() {
        let mut sentence = InProgressSentence::new();
        feed(&mut sentence, "3");
        assert!(!sentence.is_complete());
    }

    #[test]
    fn non_sentence_char_is_rejected() {
        let mut sentence = InProgressSentence::new();
        let result = sentence.accept_character('x');
        assert!(!accepted(result));
        assert!(!sentence.is_complete());
    }

    #[test]
    fn all_directions_map() {
        let cases = [
            ('h', Motion::Left),
            ('j', Motion::Down),
            ('k', Motion::Up),
            ('l', Motion::Right),
        ];
        for (ch, expected) in cases {
            let mut sentence = InProgressSentence::new();
            feed(&mut sentence, &ch.to_string());
            let (_, motion) = completed(&sentence);
            assert!(
                std::mem::discriminant(motion) == std::mem::discriminant(&expected),
                "{:?} mapped to {:?}, expected {:?}",
                ch,
                motion,
                expected
            );
        }
    }
}
