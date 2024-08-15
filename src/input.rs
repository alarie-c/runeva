use crossterm::event::KeyEvent;

use crate::{app::Msg, bindings::Bindings};

pub fn handle_select_input(b: &Bindings, e: KeyEvent) -> Msg {   
    match e {
        
        // Cursor movement
        _ if &e == b.bindings.get("cursor_up").unwrap() => Msg::Up,
        _ if &e == b.bindings.get("cursor_down").unwrap() => Msg::Down,
        _ if &e == b.bindings.get("cursor_right").unwrap() => Msg::Right,
        _ if &e == b.bindings.get("cursor_left").unwrap() => Msg::Left,

        // Modes
        _ if &e == b.bindings.get("open_cmd_prompt").unwrap() => Msg::ModeCmd,
        
        // Applications
        _ if &e == b.bindings.get("quit").unwrap() => Msg::Quit,
        _ => Msg::None,
    }
}