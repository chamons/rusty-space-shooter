use crate::{
    colors::{AQUA, BLUE, RED, WHITE, YELLOW},
    math::{Circle, Position, Rect},
    Screen,
};

use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub const MOVEMENT_SPEED: f32 = 200.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for crate::caffeinated_gorilla::space::types::GameColor {
    fn from(value: Color) -> Self {
        crate::caffeinated_gorilla::space::types::GameColor {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

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
pub struct GameState {
    pub circle: Shape,
    pub squares: Vec<Shape>,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new(screen: &Screen) -> Self {
        Self {
            circle: Shape {
                position: Position {
                    x: screen.width() / 2.0,
                    y: screen.height() / 2.0,
                },
                speed: MOVEMENT_SPEED,
                size: 32.0,
                color: YELLOW,
                is_circle: true,
            },
            squares: vec![],
            is_game_over: false,
        }
    }

    pub fn add_squares(&mut self, screen: &Screen) {
        let mut rng = thread_rng();
        if rng.gen_range(0..99) > 95 {
            let size = rng.gen_range(16.0..64.0);
            let speed = rng.gen_range(50.0..150.0);
            let position = Position {
                x: rng.gen_range((size / 2.0)..(screen.width() - size / 2.0)),
                y: -size,
            };
            let color = [WHITE, RED, AQUA, BLUE, YELLOW].choose(&mut rng).unwrap();
            self.squares.push(Shape {
                position,
                speed,
                size,
                color: color.clone(),
                is_circle: false,
            });
        }
    }

    pub fn check_game_over(&mut self) {
        if self.squares.iter().any(|s| s.collides_with(&self.circle)) {
            self.is_game_over = true
        }
    }

    pub fn new_game(&mut self, screen: &Screen) {
        self.squares.clear();
        self.circle.position = Position {
            x: screen.width() / 2.0,
            y: screen.height() / 2.0,
        };
        self.is_game_over = false;
    }
}
