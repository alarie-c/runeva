use std::io::{self, stdout, Write};

use crossterm::{cursor, execute, queue, terminal};

#[derive(Debug)]
pub struct TerminalOutput {
    pub term_size: (u16, u16),
    pub cursor: (u16, u16),
    draw_buffer: String,
    rows_buffer: Vec<String>,
    cmd_buffer: String,
}

impl TerminalOutput {
    pub fn new() -> Self {
        Self {
            term_size: terminal::size().unwrap_or((0u16, 0u16)),
            cursor: (0u16, 0u16),
            draw_buffer: String::new(),
            rows_buffer: Vec::new(),
            cmd_buffer: String::new(),
        }
    }

    pub fn clear() -> Result<(), io::Error> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}

impl TerminalOutput {
    pub fn refresh(&mut self) -> Result<(), io::Error> {
        let cpos = self.cursor; 

        // Hide cursor
        // Clear terminal contents
        // Move cursor to 0,0
        queue!(
            self,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Draw to terminal
        self.draw_rows()?;
        
        // Move cursor to 0,0
        // Show cursor
        queue!(
            self,
            cursor::MoveTo(cpos.0, cpos.1),
            cursor::Show
        )?;

        // Flush buffer contents to terminal
        self.flush()
    }

    pub fn draw_rows(&mut self) -> Result<(), io::Error> {
        // For every row
        for row in 0..self.term_size.1 {
            self.draw_buffer.push('~');

            // If row is not last, return
            if row < (self.term_size.1 - 1) {
                self.draw_buffer.push_str("\r\n");
            }
            stdout().flush()?;
        }

        Ok(())
    }
}


// Handles writing and flushing the draw_buffer to the terminal
impl Write for TerminalOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.draw_buffer.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.draw_buffer);
        stdout().flush()?;
        self.draw_buffer.clear();
        out
    }
}