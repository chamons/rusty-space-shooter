use std::sync::{Arc, Mutex};

use crate::{
    exports::example::game::game_api::{KeyboardInfo, MouseInfo},
    infrastructure::Screen,
    ui::{ScreenExt, TextSize},
    AQUA, BLUE, RED,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct GameState {
    count: u32,
}

pub struct Game {
    state: Arc<Mutex<GameState>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(GameState { count: 0 })),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    pub fn save(&self) -> Vec<u8> {
        bincode::serialize(&*self.state.lock().unwrap()).expect("Unable to save state")
    }

    pub fn restore(&self, data: Vec<u8>) {
        *self.state.lock().unwrap() = bincode::deserialize(&data).expect("Unable to restore state");
    }

    pub fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: &Screen) {
        if mouse.left.pressed {
            let mut state = self.state.lock().unwrap();
            state.count += 1;
        }

        screen.text(
            "Hot Reloading with Rust!",
            (40.0, 80.0),
            TextSize::Title,
            AQUA,
        );
        screen.draw_image(
            "resources/rustacean-flat-happy.png",
            (500.0, 25.0).into(),
            Some((150.0, 90.0).into()),
        );

        screen.standard_text(
            &format!("Count: {}", self.state.lock().unwrap().count),
            (40.0, 120.0),
        );

        screen.standard_text(&format!("Key Down: ({:?})", key.down), (40.0, 160.0).into());

        screen.standard_text(
            &format!("Mouse: ({}, {})", mouse.position.x, mouse.position.y),
            (40.0, 185.0).into(),
        );

        screen.draw_line((625.0, 125.0).into(), (675.0, 200.0).into(), 4.0, RED);
        screen.draw_line((700.0, 125.0).into(), (700.0, 200.0).into(), 4.0, BLUE);
    }
}
