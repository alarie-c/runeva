use std::io;
use app::App;

mod app;
mod bindings;
mod input;

fn main() -> Result<(), io::Error> {
    let mut app = App::new();
    while app.run()? {}

    Ok(())
}
