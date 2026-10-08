pub mod game;
pub mod saves;
pub mod ui;

use crate::assets::Assets;
use game::*;
use ratatui::DefaultTerminal;
use std::io::Error;

pub struct App {
    exit: bool,
    game: Game,
}

enum GameState {
    Title,
    Boot,
    FileManager,
}

impl App {
    pub fn run(&mut self) -> Result<(), Error> {
        let mut terminal = ratatui::init();

        boot_sequence(&mut terminal);

        ratatui::restore();
        Ok(())
    }
}
impl App {
    pub fn new() -> App {
        App {
            exit: false,
            game: Game {
                player: Player::new(),
                state: GameState::Boot,
            },
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

pub struct Game {
    player: Player,
    state: GameState,
}

fn boot_sequence(terminal: &mut DefaultTerminal) {
    let mut finished = false;
    let frame_count = 0;

    while !finished {
        
    }
}
