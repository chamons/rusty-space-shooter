use crate::exports::caffeinated_gorilla::space::game_api::{Guest, GuestGameInstance};
use crate::exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo};

pub use crate::caffeinated_gorilla::space::host_api::GameScreen;
pub use crate::caffeinated_gorilla::space::host_api::Shader;

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

    fn update_frame(
        &self,
        mouse: MouseInfo,
        key: KeyboardInfo,
        screen: &GameScreen,
        frame_time: f32,
    ) {
        Game::update_frame(self, mouse, key, screen, frame_time);
    }

    fn render_frame(&self, screen: &GameScreen) {
        Game::render_frame(self, screen);
    }
}
