mod app;
pub mod assets;

use app::*;
use std::io;

/// Integers - i32
/// Floats - f64

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}
