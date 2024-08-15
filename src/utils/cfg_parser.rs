use std::{collections::HashMap, fs};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

pub fn parse_bindings<'a>() -> HashMap<&'a str, KeyEvent> {
    let mut map: HashMap<&str, KeyEvent> = HashMap::new();
    let cfg = fs::read_to_string("bindings.cfg").unwrap();

    cfg.lines().into_iter().map(|x| {
        let mapping = parse_line(x);
        map.insert(mapping.0, mapping.1);
    });

    map
}

fn parse_line(line: &str) -> (&str, KeyEvent) {
    let mut code = KeyCode::Char('.');
    let mut modifiers = KeyModifiers::NONE;
    let kind = KeyEventKind::Press;
    let state = KeyEventState::NONE;

    let split: Vec<&str> = line.split_whitespace().to_owned().collect();
    let left = split.get(0).unwrap().to_owned();
    let right = split.get(1).unwrap().to_owned();

    drop(line);
    
    for substr in left.split("+") {
        match substr {
            "ctrl" => modifiers = KeyModifiers::CONTROL,
            "shift" => modifiers = KeyModifiers::SHIFT,
            
            "up" => code = KeyCode::Up,
            "down" => code = KeyCode::Down,
            "right" => code = KeyCode::Right,
            "left" => code = KeyCode::Left,

            // Characters
            _ => code = KeyCode::Char(substr.chars().nth(0).unwrap()),
        }
    }

    let e = KeyEvent {
        code,
        modifiers,
        kind,
        state
    };

    (right, e)
}

