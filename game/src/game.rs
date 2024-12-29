use std::sync::{Arc, Mutex};

use crate::{
    caffeinated_gorilla::space::types::{Key, Size},
    colors::{RED, YELLOW},
    exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo},
    infrastructure::Screen,
    math::Position,
    state::{GameState, MOVEMENT_SPEED},
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
        serde_json::to_string(&*self.state.lock().unwrap())
            .expect("Unable to save state")
            .as_bytes()
            .to_vec()
    }

    pub fn restore(&self, data: Vec<u8>) {
        *self.state.lock().unwrap() =
            serde_json::from_slice(&data).expect("Unable to restore state");
    }

    pub fn update_frame(
        &self,
        _mouse: MouseInfo,
        key: KeyboardInfo,
        screen: &Screen,
        frame_time: f32,
    ) {
        let mut state = self.state.lock().unwrap();
        state.add_squares(screen);

        if !state.is_game_over {
            process_keyboard_movement(&mut state, &key, screen, frame_time);
        }
        process_new_game_input(&mut state, &key, screen);

        run_physics(&mut state, screen, frame_time);
    }

    pub fn render_frame(&self, screen: &Screen) {
        let mut state = self.state.lock().unwrap();
        draw(&mut state, screen);
    }
}

fn process_new_game_input(state: &mut GameState, key: &KeyboardInfo, screen: &Screen) {
    if key.pressed.contains(&Key::Space) {
        state.new_game(screen);
    }
}

fn process_keyboard_movement(
    state: &mut GameState,
    key: &KeyboardInfo,
    screen: &Screen,
    frame_time: f32,
) {
    let circle = &mut state.circle;

    if key.down.contains(&Key::Up) {
        circle.position.y -= MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Down) {
        circle.position.y += MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Left) {
        circle.position.x -= MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Right) {
        circle.position.x += MOVEMENT_SPEED * frame_time;
    }

    circle.clamp_to_screen(screen);
}

fn run_physics(state: &mut GameState, screen: &Screen, frame_time: f32) {
    for square in &mut state.squares {
        square.position.y += square.speed * frame_time;
    }

    state
        .squares
        .retain(|square| square.position.y < screen.height() + square.size);

    state.check_game_over();
}

fn draw(state: &mut GameState, screen: &Screen) {
    screen.draw_circle(
        (state.circle.position.x, state.circle.position.y).into(),
        state.circle.size / 2.0,
        YELLOW.into(),
    );

    for square in &state.squares {
        screen.draw_rectangle(
            square.upper_left().into(),
            Size {
                width: square.size,
                height: square.size,
            },
            square.color.clone().into(),
        );
    }

    if state.is_game_over {
        const GAME_OVER_TEXT: &str = "Game Over";
        let dimensions = screen.measure_text(GAME_OVER_TEXT, 50);
        let text_position = Position {
            x: (screen.width() / 2.0) - (dimensions.width / 2.0),
            y: (screen.height() / 2.0),
        };
        screen.draw_text(GAME_OVER_TEXT, text_position.into(), 50, RED.into());
    }
}
