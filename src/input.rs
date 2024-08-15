use crossterm::event::KeyEvent;

use crate::{app::Msg, bindings::bindings};

pub fn handle_select_input(e: KeyEvent) -> Msg {
    let bindings = bindings();
    
    match e {
        _ if &e == bindings.get("quit").unwrap() => {
            Msg::Quit
        }

        _ => Msg::None,
    }
}