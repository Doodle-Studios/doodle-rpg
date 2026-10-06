pub mod game;

use game::*;
use ratatui::{DefaultTerminal, Frame};
use std::io::Error;

pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {}
}
impl App {
    pub fn new() -> App {
        App { exit: false }
    }
}
pub struct Game {
    player: Player,
}
