use std::collections::HashMap;
use crossterm::event::KeyEvent;

use crate::utils;

#[derive(Debug)]
pub struct Bindings { pub bindings: HashMap<String, KeyEvent>}

impl Bindings {
    pub fn new() -> Self {
        Self {
            bindings: utils::cfg_parser::parse_bindings()
        }
    }
}