use doodle_tale::*;
use std::io;

/// Integers - i32
/// Floats - f64

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}
