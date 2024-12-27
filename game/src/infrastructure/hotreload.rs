use crate::exports::caffeinated_gorilla::space::game_api::{Guest, GuestGameInstance};
use crate::exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo};

pub use crate::caffeinated_gorilla::space::host_api::GameScreen;

use crate::Game;

pub struct GameGuest;

impl Guest for GameGuest {
    type GameInstance = Game;
}

impl GuestGameInstance for Game {
    fn new(screen: &GameScreen) -> Game {
        Game::new(screen)
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
