use anyhow::Result;

use crate::state::State;

pub enum CharResult {
    Accepted,
    Rejected(char),
}

#[derive(Copy, Clone, Debug)]
pub enum InProgressSentence {
    ParsingCountForOperator {
        operator_count: Count,
    },
    ParsingOperator {
        operator_count: Count,
        operator: Operator,
    },
    ParsingCountForMotion {
        operator_count: Count,
        operator: Operator,
        motion_count: Count,
    },
    ParsingMotion {
        operator_count: Count,
        operator: Operator,
        motion_count: Count,
        motion: Motion,
    },
}

impl InProgressSentence {
    pub fn new() -> Self {
        InProgressSentence::ParsingCountForOperator {
            operator_count: Count::Empty,
        }
    }

    /// A complete sentence is one which either ends in a
    ///   1. A complete operator, or
    ///   2. A complete motion.
    ///
    /// A complete operator is one which is ready to execute, such as `dd`,
    /// which does not require a motion.  An incomplete operator is one which
    /// could be completed or which could be followed by a motion.  For example
    /// `d` could be followed by a `d` for a complete operator, or by `j` (with
    /// an optional count preceding it), a motion.
    ///
    /// A complete motion is one which is ready to execute, for example, `j`.
    /// An incomplete motion is one which needs more characters, such as `i`,
    /// which could be completed by, for example, 'w'.
    pub fn is_complete(&self) -> bool {
        match self {
            InProgressSentence::ParsingCountForOperator { .. } => false,
            InProgressSentence::ParsingOperator { operator, .. } => operator.is_complete(),
            InProgressSentence::ParsingCountForMotion { .. } => false,
            InProgressSentence::ParsingMotion { motion, .. } => motion.is_complete(),
        }
    }

    pub fn accept_character(&mut self, ch: char) -> CharResult {
        use CharResult::*;
        use InProgressSentence::*;
        match *self {
            ParsingCountForOperator { mut operator_count } => {
                if let Rejected(ch) = operator_count.accept_character(ch) {
                    let operator = Operator::Empty;
                    *self = ParsingOperator {
                        operator_count,
                        operator,
                    };
                    self.accept_character(ch)
                } else {
                    Accepted
                }
            }
            ParsingOperator {
                operator_count,
                mut operator,
            } => {
                if let Rejected(ch) = operator.accept_character(ch) {
                    let motion_count = Count::Empty;
                    *self = ParsingCountForMotion {
                        operator_count,
                        operator,
                        motion_count,
                    };
                    self.accept_character(ch)
                } else {
                    Accepted
                }
            }
            ParsingCountForMotion {
                operator_count,
                operator,
                mut motion_count,
            } => {
                if let Rejected(ch) = motion_count.accept_character(ch) {
                    let motion = Motion::Empty;
                    *self = ParsingMotion {
                        operator_count,
                        operator,
                        motion_count,
                        motion,
                    };
                    self.accept_character(ch)
                } else {
                    Accepted
                }
            }
            ParsingMotion { mut motion, .. } => motion.accept_character(ch),
        }
    }

    /// Takes a completed sentence
    pub fn execute(self, _state: &mut State) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, Default)]
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
                CharResult::Rejected(ch)
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

#[derive(Clone, Copy, Debug)]
enum Operator {
    Empty,
}

impl Operator {
    pub fn accept_character(&mut self, ch: char) -> CharResult {
        CharResult::Rejected(ch)
    }

    pub fn is_complete(self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Motion {
    Empty,
    Left,
    Right,
    Up,
    Down,
}

impl Motion {
    fn accept_character(&mut self, ch: char) -> CharResult {
        let motion = match (*self, ch) {
            (Motion::Empty, 'h') => Motion::Left,
            (Motion::Empty, 'j') => Motion::Down,
            (Motion::Empty, 'k') => Motion::Up,
            (Motion::Empty, 'l') => Motion::Right,
            _ => return CharResult::Rejected(ch),
        };
        *self = motion;
        CharResult::Accepted
    }

    fn is_complete(&self) -> bool {
        match self {
            Motion::Empty => false,
            _ => true,
        }
    }
}
