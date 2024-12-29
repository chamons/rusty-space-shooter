use crate::{
    caffeinated_gorilla::space::types::{GameColor, Position, Size},
    infrastructure::Screen,
};

#[allow(dead_code)]
mod colors;
pub use colors::*;

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(f32, f32)> for Size {
    fn from(value: (f32, f32)) -> Self {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

pub enum TextSize {
    Large,
    Title,
    Standard,
}

impl Into<u16> for TextSize {
    fn into(self) -> u16 {
        match self {
            TextSize::Large => 50,
            TextSize::Title => 40,
            TextSize::Standard => 20,
        }
    }
}

pub trait ScreenExt {
    fn text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor);

    fn standard_text(&self, text: &str, position: (f32, f32)) {
        self.text(text, position, TextSize::Standard, WHITE.into());
    }

    fn centered_text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor);
}

impl ScreenExt for Screen {
    fn text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor) {
        self.draw_text(text, (position.0, position.1).into(), size.into(), color);
    }

    fn centered_text(&self, text: &str, position: (f32, f32), size: TextSize, color: GameColor) {
        let size = size.into();
        let dimensions = self.measure_text(text, size);
        let text_position = Position {
            x: position.0 - (dimensions.width / 2.0),
            y: position.1,
        };
        self.draw_text(text, text_position.into(), size, color);
    }
}
