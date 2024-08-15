use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::utils;

pub struct Bindings<'a> { pub bindings: HashMap<&'a str, KeyEvent>}

impl<'a> Bindings<'a> {
    pub fn new() -> Self {
        Self {
            bindings: utils::cfg_parser::parse_bindings()
        }
    }
}

// pub fn bindings<'a>() -> &'a HashMap<&'a str, KeyEvent> {
//     //let mut map = HashMap::new();
//     utils::cfg_parser::parse_bindings()



//     // TOOD: Replace with a .cfg parser
//     // map.insert(
//     //     "quit",
//     //     KeyEvent {
//     //         code: KeyCode::Char('q'),
//     //         modifiers: KeyModifiers::CONTROL,
//     //         kind: KeyEventKind::Press,
//     //         state: KeyEventState::NONE,
//     //     }
//     // );

//     // map.insert(
//     //     "cursor_up",
//     //     KeyEvent {
//     //         code: KeyCode::Up,
//     //         modifiers: KeyModifiers::NONE,
//     //         kind: KeyEventKind::Press,
//     //         state: KeyEventState::NONE,
//     //     }
//     // );
//     // map.insert(
//     //     "cursor_down",
//     //     KeyEvent {
//     //         code: KeyCode::Down,
//     //         modifiers: KeyModifiers::NONE,
//     //         kind: KeyEventKind::Press,
//     //         state: KeyEventState::NONE,
//     //     }
//     // );
//     // map.insert(
//     //     "cursor_right",
//     //     KeyEvent {
//     //         code: KeyCode::Right,
//     //         modifiers: KeyModifiers::NONE,
//     //         kind: KeyEventKind::Press,
//     //         state: KeyEventState::NONE,
//     //     }
//     // );
//     // map.insert(
//     //     "cursor_left",
//     //     KeyEvent {
//     //         code: KeyCode::Left,
//     //         modifiers: KeyModifiers::NONE,
//     //         kind: KeyEventKind::Press,
//     //         state: KeyEventState::NONE,
//     //     }
//     // );

//     //map // Return map
// }