use std::sync::{Arc, Mutex};

use crate::{
    caffeinated_gorilla::space::types::{Key, Size},
    colors::{AQUA, RED, YELLOW},
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
        state.enemies(screen);

        if !state.is_game_over() {
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
    let player = &mut state.player;

    if key.down.contains(&Key::Up) {
        player.shape.position.y -= MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Down) {
        player.shape.position.y += MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Left) {
        player.shape.position.x -= MOVEMENT_SPEED * frame_time;
    }
    if key.down.contains(&Key::Right) {
        player.shape.position.x += MOVEMENT_SPEED * frame_time;
    }

    player.shape.clamp_to_screen(screen);
}

fn run_physics(state: &mut GameState, screen: &Screen, frame_time: f32) {
    for enemy in &mut state.enemies {
        enemy.shape.position.y += enemy.shape.speed * frame_time;
    }

    state
        .enemies
        .retain(|enemy| enemy.shape.position.y < screen.height() + enemy.shape.size);

    state.check_player_hit();
}

fn draw(state: &mut GameState, screen: &Screen) {
    screen.draw_circle(
        (state.player.shape.position.x, state.player.shape.position.y).into(),
        state.player.shape.size / 2.0,
        AQUA.into(),
    );

    for enemy in &state.enemies {
        screen.draw_rectangle(
            enemy.shape.upper_left().into(),
            Size {
                width: enemy.shape.size,
                height: enemy.shape.size,
            },
            enemy.shape.color.clone().into(),
        );
    }

    if state.is_game_over() {
        const GAME_OVER_TEXT: &str = "Game Over";
        let dimensions = screen.measure_text(GAME_OVER_TEXT, 50);
        let text_position = Position {
            x: (screen.width() / 2.0) - (dimensions.width / 2.0),
            y: (screen.height() / 2.0),
        };
        screen.draw_text(GAME_OVER_TEXT, text_position.into(), 50, RED.into());
    }
}
