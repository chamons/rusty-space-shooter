use std::sync::{Arc, Mutex};

use crate::{
    caffeinated_gorilla::space::types::{Key, Size},
    colors::{AQUA, YELLOW},
    exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo},
    infrastructure::Screen,
    state::{GameState, MOVEMENT_SPEED},
    ui::ScreenExt,
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

        process_keyboard_input(&mut state, key, screen, frame_time);

        run_physics(&mut state, screen, frame_time);
    }

    pub fn render_frame(&self, screen: &Screen) {
        let mut state = self.state.lock().unwrap();
        draw(&mut state, screen);
    }
}

fn process_keyboard_input(
    state: &mut GameState,
    key: KeyboardInfo,
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
}

fn draw(state: &mut GameState, screen: &Screen) {
    screen.draw_circle(
        (state.circle.position.x, state.circle.position.y).into(),
        state.circle.size / 2.0,
        YELLOW,
    );

    for square in &state.squares {
        screen.draw_rectangle(
            square.upper_left().into(),
            Size {
                width: square.size,
                height: square.size,
            },
            AQUA,
        );
    }
}
