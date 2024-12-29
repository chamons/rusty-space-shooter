use async_trait::async_trait;

use crate::caffeinated_gorilla::space::types::{GameColor, Position, Size};

#[async_trait]
pub trait GameScreenInterface: Send + Sync {
    fn draw_text(&self, text: &str, position: Position, size: u16, color: GameColor);
    fn draw_line(&self, first: Position, second: Position, thickness: f32, color: GameColor);
    fn draw_image(&self, filename: &str, position: Position, size: Option<Size>);
    fn draw_circle(&self, position: Position, width: f32, color: GameColor);
    fn draw_rectangle(&self, position: Position, size: Size, color: GameColor);

    fn width(&self) -> f32;
    fn height(&self) -> f32;

    fn measure_text(&self, text: &str, size: u16) -> TextDimensions;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TextDimensions {
    pub width: f32,
    pub height: f32,
    pub offset_y: f32,
}

pub type GameScreen = dyn GameScreenInterface;
