use crate::Screen;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub position: Position,
}

impl GameState {
    pub fn new(screen: &Screen) -> Self {
        Self {
            position: Position {
                x: screen.width() / 2.0,
                y: screen.height() / 2.0,
            },
        }
    }
}
