use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use lux_core::key;

/// Translates a crossterm event into the library's key event, or `None` if the
/// library does not model it. Key presses and repeats are kept; releases and
/// other event kinds are dropped.
pub fn translate_event(event: Event) -> Option<key::Event> {
    match event {
        Event::Key(key_event) => translate_key_event(key_event),
        Event::Resize(cols, rows) => Some(key::Event::Resize(cols, rows)),
        _ => None,
    }
}

fn translate_key_event(key_event: KeyEvent) -> Option<key::Event> {
    let is_kept_kind = matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat);
    if !is_kept_kind {
        return None;
    }

    let code = translate_code(key_event.code)?;
    let modifiers = translate_modifiers(key_event.modifiers)?;
    let translated = key::KeyEvent { code, modifiers };
    Some(key::Event::Key(translated))
}

fn translate_code(code: KeyCode) -> Option<key::KeyCode> {
    let translated = match code {
        KeyCode::Char(c) => key::KeyCode::Char(c),
        KeyCode::Enter => key::KeyCode::Enter,
        KeyCode::Backspace => key::KeyCode::Backspace,
        KeyCode::Esc => key::KeyCode::Esc,
        KeyCode::Up => key::KeyCode::Up,
        KeyCode::Down => key::KeyCode::Down,
        KeyCode::Left => key::KeyCode::Left,
        KeyCode::Right => key::KeyCode::Right,
        _ => return None,
    };
    Some(translated)
}

fn translate_modifiers(modifiers: KeyModifiers) -> Option<key::KeyModifiers> {
    let translated = if modifiers == KeyModifiers::NONE {
        key::KeyModifiers::None
    } else if modifiers == KeyModifiers::SHIFT {
        key::KeyModifiers::Shift
    } else if modifiers == KeyModifiers::CONTROL {
        key::KeyModifiers::Control
    } else {
        return None;
    };
    Some(translated)
}
