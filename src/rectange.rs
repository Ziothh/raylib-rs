use std::ops::{Add, Neg, Sub};

pub use crate::ffi::Rectangle;
use crate::vectors::Vector2;

impl Rectangle {
    pub const ZERO: Rectangle = Rectangle {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };

    pub fn coords(&self) -> Vector2 {
        return Vector2 {
            x: self.x,
            y: self.y,
        };
    }

    pub fn dimensions(&self) -> Vector2 {
        return Vector2 {
            x: self.width,
            y: self.height,
        };
    }

    /// Returns a `Rectangle` with all of the values initialised to the given `value`
    pub fn from_f32(value: f32) -> Self {
        return Self {
            x: value,
            y: value,
            width: value,
            height: value,
        };
    }
}

impl Add for Rectangle {
    type Output = Rectangle;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}
impl Sub for Rectangle {
    type Output = Rectangle;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl Neg for Rectangle {
    type Output = Rectangle;

    fn neg(self) -> Self::Output {
        Rectangle {
            x: -self.x,
            y: -self.y,
            width: -self.width,
            height: -self.height,
        }
    }
}
