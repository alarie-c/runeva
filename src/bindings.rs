use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

pub fn bindings<'a>() -> HashMap<&'a str, KeyEvent> {
    let mut map = HashMap::new();

    // TOOD: Replace with a .cfg parser
    map.insert(
        "quit",
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    );

    map // Return map
}