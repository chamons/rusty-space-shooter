use std::sync::{Arc, Mutex};

use crate::{
    caffeinated_gorilla::space::types::{Key, Size},
    exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo},
    infrastructure::Screen,
    state::{Bullet, GamePhase, GameState, Shape, MOVEMENT_SPEED},
    ui::{ScreenExt, TextSize, RED, WHITE, YELLOW},
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
        let save = if cfg!(debug_assertions) {
            serde_json::to_string_pretty(&*self.state.lock().unwrap())
        } else {
            serde_json::to_string(&*self.state.lock().unwrap())
        };

        save.expect("Unable to save state").as_bytes().to_vec()
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

        if !matches!(state.phase, GamePhase::Paused) {
            state.update_frame += 1;
            state.add_enemy(screen);
        }

        match state.phase {
            GamePhase::MainMenu => {
                if key.pressed.contains(&Key::Space) {
                    *state = GameState::new(screen);
                    state.phase = GamePhase::Playing;
                }
            }
            GamePhase::Playing => {
                process_movement(&mut state, &key, screen, frame_time);
                process_shoot(&mut state, &key);
                if key.pressed.contains(&Key::Escape) {
                    state.phase = GamePhase::Paused;
                }
            }
            GamePhase::Paused => {
                if key.pressed.contains(&Key::Space) {
                    state.phase = GamePhase::Playing;
                }
            }
            GamePhase::GameOver => {
                process_new_game_input(&mut state, &key, screen);
            }
        }

        if !matches!(state.phase, GamePhase::Paused) {
            run_physics(&mut state, screen, frame_time);
        }
    }

    pub fn render_frame(&self, screen: &Screen) {
        let mut state = self.state.lock().unwrap();
        draw(&mut state, screen);
    }
}

fn process_new_game_input(state: &mut GameState, key: &KeyboardInfo, screen: &Screen) {
    if key.pressed.contains(&Key::Space) {
        *state = GameState::new(screen);
    }
}

fn process_shoot(state: &mut GameState, key: &KeyboardInfo) {
    if key.pressed.contains(&Key::Space) {
        if state.player.can_shoot(state.update_frame) {
            state.player.last_fired = state.update_frame;
            state.bullets.push(Bullet::new(&state.player));
        }
    }
}

fn process_movement(state: &mut GameState, key: &KeyboardInfo, screen: &Screen, frame_time: f32) {
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
    for bullet in &mut state.bullets {
        bullet.shape.position.y += -bullet.shape.speed * frame_time;
    }

    for enemy in &mut state.enemies {
        for bullet in &mut state.bullets {
            if bullet.shape.collides_with(&enemy.shape) {
                bullet.collided = true;
                enemy.is_dead = true;
                state.score.add(enemy.shape.size.round() as u64);
            }
        }
    }

    state
        .enemies
        .retain(|enemy| is_on_screen(screen, &enemy.shape) && !enemy.is_dead);
    state
        .bullets
        .retain(|bullet| is_on_screen(screen, &bullet.shape) && !bullet.collided);

    state.check_player_hit();
}

fn is_on_screen(screen: &Screen, shape: &Shape) -> bool {
    shape.position.y < screen.height() + shape.size
}

fn draw(state: &mut GameState, screen: &Screen) {
    match state.phase {
        GamePhase::MainMenu => {
            draw_entities(state, screen);
            draw_high_score(state, screen);

            screen.centered_text(
                "Press Space to Start",
                ((screen.width() / 2.0), (screen.height() / 2.0)),
                TextSize::Title,
                WHITE.into(),
            );
        }
        GamePhase::Playing => {
            draw_entities(state, screen);
            draw_player(state, screen);
            draw_score(state, screen);
            draw_high_score(state, screen);
        }
        GamePhase::Paused => {
            draw_entities(state, screen);
            draw_player(state, screen);
            draw_score(state, screen);
            draw_high_score(state, screen);

            screen.centered_text(
                "Paused (Press Space)",
                ((screen.width() / 2.0), (screen.height() / 2.0)),
                TextSize::Title,
                WHITE.into(),
            );
        }
        GamePhase::GameOver => {
            draw_entities(state, screen);
            draw_score(state, screen);
            draw_high_score(state, screen);

            screen.centered_text(
                "Game Over (Press Space)",
                ((screen.width() / 2.0), (screen.height() / 2.0)),
                TextSize::Title,
                RED.into(),
            );
        }
    }
}

fn draw_entities(state: &mut GameState, screen: &Screen) {
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

    for bullet in &state.bullets {
        screen.draw_circle(
            bullet.shape.position.clone().into(),
            bullet.shape.size / 2.0,
            RED.into(),
        );
    }
}

fn draw_player(state: &mut GameState, screen: &Screen) {
    screen.draw_circle(
        (state.player.shape.position.x, state.player.shape.position.y).into(),
        state.player.shape.size / 2.0,
        YELLOW.into(),
    );
}

fn draw_score(state: &mut GameState, screen: &Screen) {
    screen.standard_text(
        &format!("Score: {}", state.score.current_score()),
        (10.0, 30.0),
    );
}

fn draw_high_score(state: &mut GameState, screen: &Screen) {
    screen.centered_text(
        &format!("High Score: {}", state.score.high_score()),
        (screen.width() - 77.0, 30.0),
        TextSize::Standard,
        WHITE.into(),
    );
}
