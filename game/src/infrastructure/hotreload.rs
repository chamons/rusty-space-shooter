use crate::exports::example::game::game_api::{Guest, GuestGameInstance};
use crate::exports::example::game::game_api::{KeyboardInfo, MouseInfo};

pub use crate::example::game::host_api::GameScreen;

use crate::Game;

pub struct GameGuest;

impl Guest for GameGuest {
    type GameInstance = Game;
}

impl GuestGameInstance for Game {
    fn new() -> Game {
        Game::new()
    }

    fn save(&self) -> Vec<u8> {
        Game::save(self)
    }

    fn restore(&self, data: Vec<u8>) {
        Game::restore(self, data)
    }

    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: &GameScreen) {
        Game::run_frame(self, mouse, key, screen);
    }
}
