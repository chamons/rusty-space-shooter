use crate::math::Rect;

use super::Position;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Circle {
    pub const fn new(x: f32, y: f32, r: f32) -> Self {
        Circle { x, y, r }
    }

    pub const fn point(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    pub const fn radius(&self) -> f32 {
        self.r
    }

    /// Moves the `Circle`'s origin to (x, y)
    pub fn move_to(&mut self, destination: Position) {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Circle` by a factor of sr
    pub fn scale(&mut self, sr: f32) {
        self.r *= sr;
    }

    /// Checks whether the `Circle` contains a `Point`
    pub fn contains(&self, pos: &Position) -> bool {
        pos.distance(Position {
            x: self.x,
            y: self.y,
        }) < self.r
    }

    /// Checks whether the `Circle` overlaps a `Circle`
    pub fn overlaps(&self, other: &Circle) -> bool {
        self.point().distance(other.point()) < self.r + other.r
    }

    /// Checks whether the `Circle` overlaps a `Rect`
    pub fn overlaps_rect(&self, rect: &Rect) -> bool {
        let dist_x = (self.x - rect.center().x).abs();
        let dist_y = (self.y - rect.center().y).abs();
        if dist_x > rect.w / 2.0 + self.r || dist_y > rect.h / 2.0 + self.r {
            return false;
        }
        if dist_x <= rect.w / 2.0 || dist_y <= rect.h / 2.0 {
            return true;
        }
        let lhs = dist_x - rect.w / 2.0;
        let rhs = dist_y - rect.h / 2.0;
        let dist_sq = (lhs * lhs) + (rhs * rhs);
        dist_sq <= self.r * self.r
    }

    /// Translate `Circle` origin by `offset` vector
    pub fn offset(self, offset: Position) -> Circle {
        Circle::new(self.x + offset.x, self.y + offset.y, self.r)
    }
}
