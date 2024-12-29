use serde::{Deserialize, Serialize};

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

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};

pub const AQUA: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
