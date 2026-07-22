/// Crossterm-free mirror of the key events the editor acts on. The binary
/// translates crossterm events into these types.
#[derive(Debug, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Debug, PartialEq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, PartialEq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Backspace,
    Esc,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum KeyModifiers {
    None,
    Shift,
    Control,
}
