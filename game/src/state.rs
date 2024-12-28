use crate::{
    colors::{AQUA, BLUE, RED, WHITE, YELLOW},
    Screen,
};

use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub const MOVEMENT_SPEED: f32 = 200.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<Position> for crate::caffeinated_gorilla::space::types::Position {
    fn from(value: Position) -> Self {
        crate::caffeinated_gorilla::space::types::Position {
            x: value.x,
            y: value.y,
        }
    }
}

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub circle: Shape,
    pub squares: Vec<Shape>,
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
            },
            squares: vec![],
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
            });
        }
    }
}
