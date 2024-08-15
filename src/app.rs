use std::{cell::RefCell, io, time::Duration};
use crossterm::{event::{self, Event}, terminal};
use crate::{bindings::Bindings, input, terminal_out::TerminalOutput};

#[derive(Debug)]
pub enum Mode {
    Select,
    Cmd,
    Edit,
    Files,
}

#[derive(Debug)]
pub enum Msg {
    // Application
    Quit,
    None,

    // Cursor
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct App {
    pub mode: Mode,
    pub termout: TerminalOutput,
    pub bindings: Bindings,
    pub msg_stack: RefCell<Vec<Msg>>,
}

impl App {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Error enabling raw mode");
        TerminalOutput::clear().unwrap();

        let mut a = App {
            mode: Mode::Select,
            termout: TerminalOutput::new(),
            bindings: Bindings::new(),
            msg_stack: RefCell::new(Vec::new()),
        };

        a.termout.refresh().unwrap();
        return a;
    }

    // Run method is a loop that polls key events every 500 ms
    // Everytime a key event is detected, it is matched and handled
    // The termout member handles all operations with the terminal
    // termout is based on immediate-mode rendering
    // Thus, it will react to any changes in the App struct
    pub fn run(&mut self) -> Result<bool, io::Error> {
        self.detect_input()?;
        let s = self.travese_msg_stack();
        self.termout.refresh()?;
        s
    }

    fn detect_input(&mut self) -> Result<bool, io::Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                // KeyEvents
                if let Event::Key(event) = event::read()? {
                    self.msg_stack.borrow_mut().push(input::handle_select_input(&self.bindings, event));
                    return Ok(true);
                } else if let Event::Resize(x, y) = event::read()? {
                    self.termout.term_size = (x, y);
                    return Ok(true);
                }
            }
        }
    }

    fn travese_msg_stack(&mut self) -> Result<bool, io::Error> {
        let mut stack = self.msg_stack.borrow_mut();
       
        for m in stack.drain(..) {
            match m {
                // Application
                Msg::Quit => return Ok(false),
                Msg::None => return Ok(true),

                // Cursor movements
                Msg::Up => {
                    self.termout.cursor.1 = 
                        (self.termout.cursor.1.saturating_sub(1)).clamp(0, self.termout.term_size.1);
                    return Ok(true);
                }
                Msg::Down => {
                    self.termout.cursor.1 = 
                        (self.termout.cursor.1 + 1).clamp(0, self.termout.term_size.1);
                    return Ok(true);
                }
                Msg::Left => {
                    self.termout.cursor.0 = 
                        (self.termout.cursor.0.saturating_sub(1)).clamp(0, self.termout.term_size.0);
                    return Ok(true);
                }
                Msg::Right => {
                    self.termout.cursor.0 = 
                        (self.termout.cursor.0 + 1).clamp(0, self.termout.term_size.0);
                    return Ok(true);
                }
            };
        }

        Ok(true)
    }
    
}

impl Drop for App {
    fn drop(&mut self) {
        TerminalOutput::clear().expect("Error clearing terminal");
        terminal::disable_raw_mode().expect("Error disabling raw mode");
    }
}