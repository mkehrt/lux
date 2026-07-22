use crate::terminal::Terminal;
use crate::state::State;

enum CharResult {
    Accepted,
    Rejected,
}

enum InProgressSentence {
    Start,
    Count(Count),
    CountAndOperator(Count, Box<dyn Operator>),
    CountAndOperatorCompleted(Count, Box<dyn Operator>), // dd, yy, etc.
    CountOperatorAndMotion(Count, Box<dyn Operator>, CountAndMotion),
    Motion(CountAndMotion),
}

enum Count {
    Empty,
    Number(u64),
}

impl Count {
    pub fn new() -> Self {
        Count::Empty
    }

    pub fn accept_character(&mut self, ch: char) -> CharResult {
        let digit = ch.to_digit(10).map(|d| d as u64);
        match (digit, *self) {
            (Some(digit), Count::Empty) => {
                *self = Count::Number(digit);
                CharResult::Accepted
            }
            (Some(digit), Count::Number(old_number)) => {
                let tens = old_number * 10;
                let ones = digit as u64;
                let new_number = tens + ones;
                *self = Count::Number(new_number);
                CharResult::Accepted
            }
            (None, _) => CharResult::Rejected,
        }
    }
}

impl InProgressSentence {
    pub fn new() -> Self {
        InProgressSentence::Start
    }

    pub fn accept_character(&mut self, ch: char) -> CharResult {
        unimplemented!()
    }

    pub fn is_complete(&self) -> bool {
        unimplemented!()
    }

    pub fn execute<T: Terminal>(state: &mut State<T>) -> Result<()> {
        unimplemented!()
    }
}

trait Operator {
    fn execute<T: Terminal>(state: &mut State<T>) -> Result<()>;
}

struct Delete;
impl Operator for Delete {
    fn execute<T: Terminal>(state: &mut State<T>) -> Result<()> {
        unimplemented!()
    }
}

struct Yank;
impl Operator for Yank {
    fn execute<T: Terminal>(state: &mut State<T>) -> Result<()> {
        unimplemented!()
    }
}

struct CountAndMotion {
    count: Count,
    motion: Motion,
}

enum Motion {
    Up,
    Down,
    Left,
    Right,
    Line,
}
