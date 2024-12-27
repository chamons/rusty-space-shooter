use std::sync::{Arc, Mutex};

use crate::{
    caffeinated_gorilla::space::types::Key,
    colors::YELLOW,
    exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo},
    infrastructure::Screen,
    state::GameState,
};

pub struct Game {
    state: Arc<Mutex<GameState>>,
}

impl Game {
    pub fn new(screen: &Screen) -> Game {
        Self {
            state: Arc::new(Mutex::new(GameState::new(screen))),
        }
    }

    pub fn save(&self) -> Vec<u8> {
        bincode::serialize(&*self.state.lock().unwrap()).expect("Unable to save state")
    }

    pub fn restore(&self, data: Vec<u8>) {
        *self.state.lock().unwrap() = bincode::deserialize(&data).expect("Unable to restore state");
    }

    pub fn run_frame(
        &self,
        _mouse: MouseInfo,
        key: KeyboardInfo,
        screen: &Screen,
        frame_time: f32,
    ) {
        let mut state = self.state.lock().unwrap();
        let position = &mut state.position;

        const MOVEMENT_SPEED: f32 = 200.0;
        const CIRCLE_RADIUS: f32 = 16.0;

        if key.down.contains(&Key::Up) {
            position.y -= MOVEMENT_SPEED * frame_time;
        }
        if key.down.contains(&Key::Down) {
            position.y += MOVEMENT_SPEED * frame_time;
        }
        if key.down.contains(&Key::Left) {
            position.x -= MOVEMENT_SPEED * frame_time;
        }
        if key.down.contains(&Key::Right) {
            position.x += MOVEMENT_SPEED * frame_time;
        }

        position.x = position
            .x
            .clamp(CIRCLE_RADIUS, screen.width() - CIRCLE_RADIUS);
        position.y = position
            .y
            .clamp(CIRCLE_RADIUS, screen.height() - CIRCLE_RADIUS);

        screen.draw_circle((position.x, position.y).into(), CIRCLE_RADIUS, YELLOW);
    }
}
