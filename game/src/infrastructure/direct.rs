use async_trait::async_trait;

use crate::example::game::types::{GameColor, Position, Size};

#[async_trait]
pub trait GameScreenInterface: Send + Sync {
    fn draw_text(&self, text: &str, position: Position, size: f32, color: GameColor);
    fn draw_line(&self, first: Position, second: Position, thickness: f32, color: GameColor);
    fn draw_image(&self, filename: &str, position: Position, size: Option<Size>);
}

pub type GameScreen = dyn GameScreenInterface;
