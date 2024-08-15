use std::{cell::RefCell, io, time::Duration};
use crossterm::{event::{self, Event}, terminal};
use crate::input;

pub enum Mode {
    Select,
    Cmd,
    Edit,
    Files,
}

pub enum Msg {
    // Application
    Quit,
    None,
}

pub struct App {
    pub mode: Mode,
    pub msg_stack: RefCell<Vec<Msg>>,
}

impl App {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Error enabling raw mode");
        
        App {
            mode: Mode::Select,
            msg_stack: RefCell::new(Vec::new()),
        }
    }

    // Run method is a loop that polls key events every 500 ms
    // Everytime a key event is detected, it is matched and handled
    // The Writer member handles all operations with the terminal
    // The Writer is based on immediate-mode rendering
    // Thus, it will react to any changes in the App struct
    pub fn run(&mut self) -> Result<bool, io::Error> {
        self.detect_input()?;
        self.travese_msg_stack()
    }

    fn detect_input(&mut self) -> Result<bool, io::Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                // KeyEvents
                if let Event::Key(event) = event::read()? {
                    self.msg_stack.borrow_mut().push(input::handle_select_input(event));
                    return Ok(true);
                } 
            }
        }
    }

    fn travese_msg_stack(&mut self) -> Result<bool, io::Error> {
        let mut stack = self.msg_stack.borrow_mut();
       
        for m in stack.drain(..) {
            match m {
                Msg::Quit => return Ok(false),
                Msg::None => return Ok(true),
            };
        }

        Ok(true)
    }
    
}

impl Drop for App {
    fn drop(&mut self) {
        terminal::Clear(terminal::ClearType::All);
    }
}