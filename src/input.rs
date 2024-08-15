use crossterm::event::KeyEvent;

use crate::{app::Msg, bindings::bindings};

pub fn handle_select_input(e: KeyEvent) -> Msg {
    let bindings = bindings();
    
    match e {
        
        // Cursor movement
        _ if &e == bindings.get("cursor_up").unwrap() => Msg::Up,
        _ if &e == bindings.get("cursor_down").unwrap() => Msg::Down,
        _ if &e == bindings.get("cursor_right").unwrap() => Msg::Right,
        _ if &e == bindings.get("cursor_left").unwrap() => Msg::Left,
        
        // Applications
        _ if &e == bindings.get("quit").unwrap() => Msg::Quit,
        _ => Msg::None,
    }
}