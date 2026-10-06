pub mod game;
pub mod saves;
pub mod ui;

use game::*;
use ratatui::{DefaultTerminal, Frame};
use saves::*;
use std::io::Error;

pub struct App {
    exit: bool,
}

enum GameState {
    Title,
    FileManager,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            terminal.draw(|frame| ui::draw(&self, frame))?;
        }
        Ok(())
    }
}
impl App {
    pub fn new() -> App {
        App { exit: false }
    }
}
pub struct Game {
    player: Player,
}
