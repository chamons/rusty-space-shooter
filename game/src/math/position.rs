use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance(&self, other: Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

impl From<Position> for crate::caffeinated_gorilla::space::types::Position {
    fn from(value: Position) -> Self {
        crate::caffeinated_gorilla::space::types::Position {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Position;

    #[test]
    fn position_distance() {
        let first = Position { x: 1.0, y: 2.0 };
        let second = Position { x: 4.0, y: 3.0 };
        let distance = first.distance(second);
        assert_eq!("3.16228", format!("{:5}", distance));
    }
}
