use crate::{
    math::{Circle, Position, Rect},
    score::HighScore,
    ui::{Color, AQUA, BLUE, RED, WHITE, YELLOW},
    Screen,
};

use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub const MOVEMENT_SPEED: f32 = 200.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub position: Position,
    pub speed: f32,
    pub size: f32,
    pub color: Color,
    pub is_circle: bool,
}

impl Shape {
    pub fn clamp_to_screen(&mut self, screen: &Screen) {
        let half_width = self.half_width();
        self.position.x = self
            .position
            .x
            .clamp(half_width, screen.width() - half_width);
        self.position.y = self
            .position
            .y
            .clamp(half_width, screen.height() - half_width);
    }

    pub fn half_width(&self) -> f32 {
        self.size / 2.0
    }

    pub fn upper_left(&self) -> Position {
        let half_width = self.half_width();

        Position {
            x: self.position.x - half_width,
            y: self.position.y - half_width,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.position.x - self.half_width(),
            y: self.position.y - self.half_width(),
            w: self.size,
            h: self.size,
        }
    }

    pub fn circle(&self) -> Circle {
        Circle {
            x: self.position.x - self.half_width(),
            y: self.position.y - self.half_width(),
            r: self.size,
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        if self.is_circle {
            self.circle().overlaps_rect(&other.rect())
        } else {
            self.rect().overlaps(&other.rect())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub shape: Shape,
    pub last_fired: u64,
    pub is_dead: bool,
}

impl Ship {
    pub fn new_player(screen: &Screen) -> Self {
        Self {
            shape: Shape {
                position: Position {
                    x: screen.width() / 2.0,
                    y: screen.height() / 2.0,
                },
                speed: MOVEMENT_SPEED,
                size: 32.0,
                color: YELLOW,
                is_circle: true,
            },
            is_dead: false,
            last_fired: 0,
        }
    }

    pub fn new_enemy(screen: &Screen) -> Self {
        let mut rng = thread_rng();
        let size = rng.gen_range(16.0..64.0);
        let speed = rng.gen_range(50.0..150.0);
        let position = Position {
            x: rng.gen_range((size / 2.0)..(screen.width() - size / 2.0)),
            y: -size,
        };
        let color = [WHITE, RED, AQUA, BLUE, YELLOW].choose(&mut rng).unwrap();
        Ship {
            shape: Shape {
                position,
                speed,
                size,
                color: color.clone(),
                is_circle: false,
            },
            is_dead: false,
            last_fired: 0,
        }
    }

    const FIRE_RATE: u64 = 30;
    pub fn can_shoot(&self, update_frame: u64) -> bool {
        (update_frame - self.last_fired) > Self::FIRE_RATE
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bullet {
    pub shape: Shape,
    pub collided: bool,
}

impl Bullet {
    pub fn new(shooter: &Ship) -> Bullet {
        Bullet {
            shape: Shape {
                position: shooter.shape.position.clone(),
                speed: shooter.shape.speed * 2.0,
                size: 5.0,
                color: RED,
                is_circle: true,
            },
            collided: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub update_frame: u64,
    pub player: Ship,
    pub enemies: Vec<Ship>,
    pub bullets: Vec<Bullet>,
    pub score: HighScore,
}

impl GameState {
    pub fn new(screen: &Screen) -> Self {
        Self {
            update_frame: 0,
            player: Ship::new_player(screen),
            enemies: vec![],
            bullets: vec![],
            score: HighScore::load(),
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.player.is_dead
    }

    pub fn add_enemy(&mut self, screen: &Screen) {
        if thread_rng().gen_range(0..99) > 95 {
            self.enemies.push(Ship::new_enemy(screen));
        }
    }

    pub fn check_player_hit(&mut self) {
        if self
            .enemies
            .iter()
            .any(|s| s.shape.collides_with(&self.player.shape))
        {
            if !self.player.is_dead {
                self.player.is_dead = true;
                self.score.save();
            }
        }
    }
}
