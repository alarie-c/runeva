use std::{collections::HashMap, fs};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

pub fn parse_bindings<'a>() -> HashMap<String, KeyEvent> {
    let mut map: HashMap<String, KeyEvent> = HashMap::new();
    let cfg = fs::read_to_string("src/utils/bindings.cfg").unwrap();

    for line in cfg.lines() {
        let mapping = parse_line(line);
        map.insert(mapping.0, mapping.1);
    }

    map
}

fn parse_line(line: &str) -> (String, KeyEvent) {
    let mut code = KeyCode::Char('~');
    let mut modifiers = KeyModifiers::NONE;
    let kind = KeyEventKind::Press;
    let state = KeyEventState::NONE;

    let split: Vec<&str> = line.split_whitespace().collect();
    let left = split.get(0).unwrap().to_string();
    let right = split.get(1).unwrap().to_string();
    
    for substr in left.split("+") {
        match substr {
            "ctrl" => modifiers = KeyModifiers::CONTROL,
            "shift" => modifiers = KeyModifiers::SHIFT,
            
            "up" => code = KeyCode::Up,
            "down" => code = KeyCode::Down,
            "right" => code = KeyCode::Right,
            "left" => code = KeyCode::Left,

            "esc" => code = KeyCode::Esc,
            "enter" => code = KeyCode::Enter,
            "backspace" => code = KeyCode::Backspace,

            // Characters
            // The only time this is matched is when anything is only 1 char
            // All valid multi-character keys are matched above instead
            _ => code = KeyCode::Char(substr.chars().nth(0).unwrap()),
        }
    }

    let e = KeyEvent {
        code,
        modifiers,
        kind,
        state
    };

    (right.to_string(), e)
}

