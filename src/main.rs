use std::{env, io};
use app::App;
use crossterm::terminal;

mod app;
mod bindings;
mod input;
mod terminal_out;
mod utils;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "d" {
        let app = App::new();
        terminal::disable_raw_mode().unwrap();
        dbg!(app);
    } else {
        let mut app = App::new();
        while app.run()? {}
    }

    Ok(())
}
