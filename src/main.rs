use std::io;
use app::App;

mod app;
mod bindings;
mod input;
mod terminal_out;
mod utils;

fn main() -> Result<(), io::Error> {
    let mut app = App::new();
    while app.run()? {}

    Ok(())
}
